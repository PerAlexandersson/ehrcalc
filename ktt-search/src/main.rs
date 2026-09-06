use clap::Parser;
use ehrcalc_kostka_engine::ehrhart::{compute_hstar, try_compute_ehrhart, EhrhartPoly};
use ehrcalc_kostka_engine::gt_dim::gt_polytope_dim_full;
use ehrcalc_kostka_engine::Partition;
use mysql::prelude::*;
use mysql::*;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::panic;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug, Parser)]
#[command(
    name = "ktt-scan",
    about = "Bounded exact KTT scan that populates the existing gt_ehrhart DB",
    version
)]
struct Args {
    /// Minimum |lambda|.
    #[arg(long, default_value_t = 2)]
    min_size: u32,

    /// Maximum |lambda|.
    #[arg(long, default_value_t = 10)]
    max_size: u32,

    /// Database URL (MariaDB/MySQL), also read from KOSTKA_DB_URL.
    #[arg(long, env = "KOSTKA_DB_URL")]
    db_url: String,

    /// Skip weights with exactly one part.
    #[arg(long, default_value_t = false)]
    skip_single_part: bool,

    /// Skip all-ones weights.
    #[arg(long, default_value_t = false)]
    skip_all_ones: bool,

    /// Include connected skew shapes λ/μ.
    #[arg(long)]
    skew: bool,

    /// Search only connected skew shapes, excluding straight shapes.
    #[arg(long)]
    skew_only: bool,

    /// Minimum weight part count.
    #[arg(long)]
    min_weight_parts: Option<usize>,

    /// Maximum weight part count.
    #[arg(long)]
    max_weight_parts: Option<usize>,

    /// Skip weights whose largest part exceeds this value.
    #[arg(long)]
    max_weight_part: Option<u32>,

    /// Skip λ partitions whose largest part exceeds this value.
    #[arg(long)]
    max_lambda_part: Option<u32>,

    /// Skip weights with more than this many non-one parts.
    #[arg(long)]
    max_nonones: Option<usize>,

    /// Minimum GT dimension to compute.
    #[arg(long, default_value_t = 0)]
    min_dimension: usize,

    /// Maximum GT dimension to compute.
    #[arg(long)]
    max_dimension: Option<usize>,

    /// Abort DP if a state level exceeds this many states.
    #[arg(long)]
    max_states: Option<usize>,

    /// Optional upper flag list for flagged scans (comma-separated row caps).
    #[arg(long)]
    upper_flags: Option<String>,

    /// Optional lower flag list for flagged scans (comma-separated row minima).
    #[arg(long)]
    lower_flags: Option<String>,

    /// Stop after this many successful computations.
    #[arg(long)]
    max_cases: Option<u64>,

    /// Continue scanning after first negative/inequality witness.
    #[arg(long, default_value_t = false)]
    continue_after_first: bool,

    /// Print candidates without inserting to DB.
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Write a JSON reproducibility report to this path.
    #[arg(long, default_value = "ktt-search/scan-report.json")]
    report_path: PathBuf,
}

#[derive(Debug, Serialize, Eq, PartialEq, Ord, PartialOrd)]
enum SkipReason {
    NotDominated,
    AllOnes,
    SinglePart,
    MinWeightParts,
    MaxWeightParts,
    MaxWeightPart,
    MaxLambdaPart,
    MaxNonOnes,
    FlagsLengthMismatch,
    EmptyPolytope,
    DimensionBelowMin,
    DimensionAboveMax,
    AlreadyInDb,
    ComputeError,
    DbError,
    CachedUnverified,
    NegativeCoefficient,
    HibiStanleyFailure,
}

#[derive(Debug, Serialize)]
struct ConfigSummary {
    min_size: u32,
    max_size: u32,
    skip_single_part: bool,
    skip_all_ones: bool,
    skew: bool,
    skew_only: bool,
    min_weight_parts: Option<usize>,
    max_weight_parts: Option<usize>,
    max_weight_part: Option<u32>,
    max_lambda_part: Option<u32>,
    max_nonones: Option<usize>,
    min_dimension: usize,
    max_dimension: Option<usize>,
    max_states: Option<usize>,
    upper_flags: Option<Vec<u32>>,
    lower_flags: Option<Vec<u32>>,
    max_cases: Option<u64>,
    continue_after_first: bool,
    dry_run: bool,
    report_path: PathBuf,
}

#[derive(Debug, Default, Serialize)]
struct ScanTotals {
    total_pairs: u64,
    candidates: u64,
    computed: u64,
    db_skips: u64,
    cached_verified: u64,
    cached_unverified: u64,
    compute_errors: u64,
    db_errors: u64,
    elapsed_ms: u128,
    min_case_ms: u128,
    max_case_ms: u128,
    total_compute_ms: u128,
}

#[derive(Debug, Serialize)]
struct CounterexampleRecord {
    size: u32,
    lambda: Vec<u32>,
    mu: Vec<u32>,
    weight: Vec<u32>,
    upper_flags: String,
    lower_flags: String,
    gt_dimension: usize,
    polynomial_degree: usize,
    failure_kind: String,
    first_negative_degree: Option<usize>,
    first_negative_coefficient: Option<String>,
    failure_family: Option<String>,
    failure_name: Option<String>,
    failure_index: Option<usize>,
    failure_lhs: Option<String>,
    failure_rhs: Option<String>,
    failure_formula: Option<String>,
    polynomial: String,
    hstar: String,
}

#[derive(Debug, Serialize)]
struct ScanReport {
    started_utc_unix_ms: u128,
    completed_utc_unix_ms: u128,
    config: ConfigSummary,
    totals: ScanTotals,
    skipped: BTreeMap<SkipReason, u64>,
    first_counterexample: Option<CounterexampleRecord>,
    first_unverified: Option<UnverifiedRecord>,
    capped: bool,
    status: String,
}

#[derive(Debug, Serialize)]
struct UnverifiedRecord {
    size: u32,
    lambda: Vec<u32>,
    mu: Vec<u32>,
    weight: Vec<u32>,
    upper_flags: String,
    lower_flags: String,
    reason: String,
}

#[derive(Debug)]
struct CachedRow {
    kostka: String,
    dimension: Option<i64>,
    polynomial: String,
    hstar: Option<String>,
}

#[derive(Clone, Copy)]
struct CaseContext<'a> {
    size: u32,
    lambda: &'a Partition,
    mu: &'a Partition,
    weight: &'a Partition,
    gt_dimension: usize,
    upper_flags: &'a str,
    lower_flags: &'a str,
}

#[derive(Clone, Copy)]
struct DbCase<'a> {
    n: u32,
    lambda: &'a str,
    mu: &'a str,
    weight: &'a str,
    upper_flags: &'a str,
    lower_flags: &'a str,
}

const CREATE_TABLE: &str = r"
CREATE TABLE IF NOT EXISTS gt_ehrhart (
    id              BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    n               INT UNSIGNED NOT NULL,
    lambda          VARCHAR(255) NOT NULL,
    mu              VARCHAR(255) NOT NULL DEFAULT '',
    weight          VARCHAR(255) NOT NULL,
    upper_flags     VARCHAR(255) NOT NULL DEFAULT '',
    lower_flags     VARCHAR(255) NOT NULL DEFAULT '',
    kostka          TEXT NOT NULL,
    dimension       INT,
    polynomial      TEXT NOT NULL,
    hstar           TEXT,
    elapsed_ms      BIGINT UNSIGNED,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE KEY uq_shape (lambda, mu, weight, upper_flags, lower_flags)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
";

fn main() {
    let args = Args::parse();

    if args.min_size > args.max_size {
        eprintln!(
            "invalid bounds: --min-size ({}) must be <= --max-size ({})",
            args.min_size, args.max_size
        );
        std::process::exit(1);
    }

    let started = SystemTime::now();
    let started_ms = started
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_millis();

    let opts = mysql::Opts::from_url(&args.db_url).unwrap_or_else(|err| {
        eprintln!("invalid DB URL: {err}");
        std::process::exit(1);
    });
    let pool = Pool::new(opts).unwrap_or_else(|err| {
        eprintln!("failed to connect DB: {err}");
        std::process::exit(1);
    });

    if !args.dry_run {
        let mut conn = pool.get_conn().expect("failed to get DB connection");
        conn.query_drop(CREATE_TABLE)
            .unwrap_or_else(|err| panic!("failed to create gt_ehrhart table: {err}"));
    }

    let mut totals = ScanTotals {
        total_pairs: 0,
        candidates: 0,
        computed: 0,
        db_skips: 0,
        cached_verified: 0,
        cached_unverified: 0,
        compute_errors: 0,
        db_errors: 0,
        elapsed_ms: 0,
        min_case_ms: u128::MAX,
        max_case_ms: 0,
        total_compute_ms: 0,
    };
    let mut skipped: BTreeMap<SkipReason, u64> = BTreeMap::new();
    let mut first_counterexample: Option<CounterexampleRecord> = None;
    let mut first_unverified: Option<UnverifiedRecord> = None;
    let mut capped = false;

    let parsed_upper_flags = parse_flag_arg(args.upper_flags.as_deref(), "upper_flags")
        .unwrap_or_else(|err| {
            eprintln!("invalid --upper-flags: {err}");
            std::process::exit(1);
        });
    let parsed_lower_flags = parse_flag_arg(args.lower_flags.as_deref(), "lower_flags")
        .unwrap_or_else(|err| {
            eprintln!("invalid --lower-flags: {err}");
            std::process::exit(1);
        });

    let run_start = Instant::now();

    'outer: for size in args.min_size..=args.max_size {
        let mut lambdas = Partition::all_of_size(size);
        lambdas.sort_by(|a, b| {
            a.part(0)
                .cmp(&b.part(0))
                .then_with(|| a.num_parts().cmp(&b.num_parts()))
                .then_with(|| a.parts().cmp(b.parts()))
        });

        for lambda in &lambdas {
            if let Some(max_lambda_part) = args.max_lambda_part {
                if lambda.part(0) > max_lambda_part {
                    *skipped.entry(SkipReason::MaxLambdaPart).or_insert(0) += 1;
                    continue;
                }
            }
            let lambda_str = partition_string(lambda);
            let inners = if args.skew || args.skew_only {
                inner_shapes(lambda)
            } else {
                vec![Partition::empty()]
            };

            for mu in &inners {
                if args.skew_only && mu.is_empty() {
                    continue;
                }
                if !args.skew && !args.skew_only && !mu.is_empty() {
                    continue;
                }

                let skew_size = size.saturating_sub(mu.size());
                if skew_size == 0 {
                    continue;
                }

                let mu_str = partition_string(mu);
                let weights = Partition::all_of_size(skew_size);

                for weight in &weights {
                    totals.total_pairs += 1;

                    if let Some(reason) = skip_by_weight_filters(weight, &args) {
                        *skipped.entry(reason).or_insert(0) += 1;
                        continue;
                    }

                    if !dominance_holds(lambda, weight) {
                        *skipped.entry(SkipReason::NotDominated).or_insert(0) += 1;
                        continue;
                    }

                    let effective_upper_flags =
                        normalized_flags(parsed_upper_flags.as_ref(), weight.num_parts(), u32::MAX);
                    let effective_lower_flags =
                        normalized_flags(parsed_lower_flags.as_ref(), weight.num_parts(), 1);
                    let upper_flags = as_slice_opt(&effective_upper_flags);
                    let lower_flags = as_slice_opt(&effective_lower_flags);

                    if requested_flags_missing(
                        parsed_upper_flags.as_ref(),
                        upper_flags,
                        !weight.is_empty(),
                    ) || requested_flags_missing(
                        parsed_lower_flags.as_ref(),
                        lower_flags,
                        !weight.is_empty(),
                    ) {
                        *skipped.entry(SkipReason::FlagsLengthMismatch).or_insert(0) += 1;
                        continue;
                    }

                    let upper_flags_str = flags_to_string(&effective_upper_flags);
                    let lower_flags_str = flags_to_string(&effective_lower_flags);
                    let Some(dimension) = gt_polytope_dim_full(
                        lambda.parts(),
                        mu.parts(),
                        weight.parts(),
                        upper_flags,
                        lower_flags,
                    ) else {
                        *skipped.entry(SkipReason::EmptyPolytope).or_insert(0) += 1;
                        continue;
                    };

                    if dimension < args.min_dimension {
                        *skipped.entry(SkipReason::DimensionBelowMin).or_insert(0) += 1;
                        continue;
                    }
                    if let Some(max_dimension) = args.max_dimension {
                        if dimension > max_dimension {
                            *skipped.entry(SkipReason::DimensionAboveMax).or_insert(0) += 1;
                            continue;
                        }
                    }

                    let weight_str = partition_string(weight);
                    let context = CaseContext {
                        size,
                        lambda,
                        mu,
                        weight,
                        gt_dimension: dimension,
                        upper_flags: &upper_flags_str,
                        lower_flags: &lower_flags_str,
                    };
                    let db_case = DbCase {
                        n: size,
                        lambda: &lambda_str,
                        mu: &mu_str,
                        weight: &weight_str,
                        upper_flags: &upper_flags_str,
                        lower_flags: &lower_flags_str,
                    };
                    match load_cached_row(
                        &pool,
                        &lambda_str,
                        &mu_str,
                        &weight_str,
                        &upper_flags_str,
                        &lower_flags_str,
                    ) {
                        Ok(Some(cached)) => {
                            totals.db_skips += 1;
                            match validate_cached_row(context, &cached) {
                                Ok(cached_failure) => {
                                    totals.cached_verified += 1;
                                    *skipped.entry(SkipReason::AlreadyInDb).or_insert(0) += 1;
                                    if let Some(counterexample) = cached_failure {
                                        let skip_reason = match counterexample.failure_kind.as_str()
                                        {
                                            "negative_coefficient" => {
                                                SkipReason::NegativeCoefficient
                                            }
                                            _ => SkipReason::HibiStanleyFailure,
                                        };
                                        *skipped.entry(skip_reason).or_insert(0) += 1;
                                        if first_counterexample.is_none() {
                                            first_counterexample = Some(counterexample);
                                        }
                                        if !args.continue_after_first {
                                            break 'outer;
                                        }
                                    }
                                }
                                Err(reason) => {
                                    totals.cached_unverified += 1;
                                    *skipped.entry(SkipReason::CachedUnverified).or_insert(0) += 1;
                                    if first_unverified.is_none() {
                                        first_unverified = Some(unverified_record(context, reason));
                                    }
                                }
                            }
                            continue;
                        }
                        Ok(None) => {}
                        Err(error) => {
                            totals.db_errors += 1;
                            eprintln!(
                                "DB lookup failed: n={} lambda={} mu={} weight={} :: {error}",
                                size, lambda_str, mu_str, weight_str
                            );
                        }
                    }

                    if args.max_cases.is_some_and(|limit| totals.computed >= limit) {
                        capped = true;
                        break 'outer;
                    }
                    totals.candidates += 1;

                    if args.dry_run {
                        totals.computed += 1;
                        println!(
                            "[dry-run] n={} lambda={} mu={} weight={} upper_flags={} lower_flags={}",
                            size, lambda_str, mu_str, weight_str, upper_flags_str, lower_flags_str
                        );
                        continue;
                    }

                    let candidate_start = Instant::now();
                    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                        try_compute_ehrhart(
                            lambda,
                            mu,
                            weight.parts(),
                            upper_flags,
                            lower_flags,
                            false,
                            args.max_states,
                            true,
                        )
                    }));
                    let case_ms = candidate_start.elapsed().as_millis();
                    totals.total_compute_ms += case_ms;
                    totals.min_case_ms = totals.min_case_ms.min(case_ms);
                    totals.max_case_ms = totals.max_case_ms.max(case_ms);

                    match result {
                        Ok(Ok(poly)) => {
                            totals.computed += 1;
                            if let Some(counterexample) = first_violation_record(context, &poly) {
                                let skip_reason = match counterexample.failure_kind.as_str() {
                                    "negative_coefficient" => SkipReason::NegativeCoefficient,
                                    _ => SkipReason::HibiStanleyFailure,
                                };
                                *skipped.entry(skip_reason).or_insert(0) += 1;

                                if first_counterexample.is_none() {
                                    first_counterexample = Some(counterexample);
                                }
                                if let Err(err) = insert_row(
                                    &pool,
                                    db_case,
                                    &poly,
                                    Some(u64::try_from(case_ms).unwrap_or(u64::MAX)),
                                ) {
                                    totals.db_errors += 1;
                                    *skipped.entry(SkipReason::DbError).or_insert(0) += 1;
                                    eprintln!(
                                        "DB insert failed: n={} lambda={} mu={} weight={} :: {err}",
                                        size, lambda_str, mu_str, weight_str
                                    );
                                }

                                if !args.continue_after_first {
                                    break 'outer;
                                }
                            } else if let Err(err) = insert_row(
                                &pool,
                                db_case,
                                &poly,
                                Some(u64::try_from(case_ms).unwrap_or(u64::MAX)),
                            ) {
                                totals.db_errors += 1;
                                *skipped.entry(SkipReason::DbError).or_insert(0) += 1;
                                eprintln!(
                                    "DB insert failed: n={} lambda={} mu={} weight={} :: {err}",
                                    size, lambda_str, mu_str, weight_str
                                );
                            }
                        }
                        Ok(Err(err)) => {
                            totals.compute_errors += 1;
                            *skipped.entry(SkipReason::ComputeError).or_insert(0) += 1;
                            println!(
                                "compute error: n={} lambda={} mu={} weight={}\n  {err}",
                                size, lambda_str, mu_str, weight_str
                            );
                        }
                        Err(_) => {
                            totals.compute_errors += 1;
                            *skipped.entry(SkipReason::ComputeError).or_insert(0) += 1;
                            println!(
                                "panic/error: n={} lambda={} mu={} weight={}",
                                size, lambda_str, mu_str, weight_str
                            );
                        }
                    }

                    totals.elapsed_ms = run_start.elapsed().as_millis();
                }
            }
        }
    }

    totals.elapsed_ms = run_start.elapsed().as_millis();
    if totals.min_case_ms == u128::MAX {
        totals.min_case_ms = 0;
    }

    let completed_ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_millis();

    let status = report_status(
        first_counterexample.is_some(),
        capped,
        args.dry_run,
        &totals,
    );

    let report = ScanReport {
        started_utc_unix_ms: started_ms,
        completed_utc_unix_ms: completed_ms,
        config: ConfigSummary {
            min_size: args.min_size,
            max_size: args.max_size,
            skip_single_part: args.skip_single_part,
            skip_all_ones: args.skip_all_ones,
            skew: args.skew,
            skew_only: args.skew_only,
            min_weight_parts: args.min_weight_parts,
            max_weight_parts: args.max_weight_parts,
            max_weight_part: args.max_weight_part,
            max_lambda_part: args.max_lambda_part,
            max_nonones: args.max_nonones,
            min_dimension: args.min_dimension,
            max_dimension: args.max_dimension,
            max_states: args.max_states,
            upper_flags: parsed_upper_flags,
            lower_flags: parsed_lower_flags,
            max_cases: args.max_cases,
            continue_after_first: args.continue_after_first,
            dry_run: args.dry_run,
            report_path: args.report_path.clone(),
        },
        totals,
        skipped,
        first_counterexample,
        first_unverified,
        capped,
        status: status.to_string(),
    };

    match serde_json::to_string_pretty(&report) {
        Ok(json) => {
            if let Some(parent) = args.report_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Err(err) = fs::write(&args.report_path, json.as_bytes()) {
                eprintln!(
                    "unable to write report to {}: {err}",
                    args.report_path.display()
                );
                println!("{}", json);
            }
        }
        Err(err) => {
            eprintln!("unable to serialize report: {err}");
        }
    }

    println!(
        "scan complete: status={} pairs={} candidates={} computed={} db_skips={} cached_verified={} cached_unverified={} errors={} elapsed_ms={}",
        status,
        report.totals.total_pairs,
        report.totals.candidates,
        report.totals.computed,
        report.totals.db_skips,
        report.totals.cached_verified,
        report.totals.cached_unverified,
        report.totals.compute_errors + report.totals.db_errors,
        report.totals.elapsed_ms
    );
    println!("skipped cases:");
    for (reason, count) in &report.skipped {
        println!("  {:?}: {}", reason, count);
    }

    if let Some(counterexample) = &report.first_counterexample {
        println!(
            "counterexample: n={} lambda={} mu={} weight={} flags={}/{} kind={}",
            counterexample.size,
            partition_string_from_parts(&counterexample.lambda),
            partition_string_from_parts(&counterexample.mu),
            partition_string_from_parts(&counterexample.weight),
            counterexample.upper_flags,
            counterexample.lower_flags,
            counterexample.failure_kind
        );
        if let Some(term) = counterexample.first_negative_degree {
            println!(
                "  first_negative_term={}:{}",
                term,
                counterexample
                    .first_negative_coefficient
                    .as_ref()
                    .unwrap_or(&"<missing>".to_string())
            );
        }
        if let Some(family) = &counterexample.failure_family {
            println!(
                "  inequality failure: family={} name={} index={:?} lhs={} rhs={}",
                family,
                counterexample
                    .failure_name
                    .as_ref()
                    .unwrap_or(&"<missing>".to_string()),
                counterexample.failure_index,
                counterexample
                    .failure_lhs
                    .as_ref()
                    .unwrap_or(&"<missing>".to_string()),
                counterexample
                    .failure_rhs
                    .as_ref()
                    .unwrap_or(&"<missing>".to_string())
            );
            if let Some(formula) = &counterexample.failure_formula {
                println!("  formula: {formula}");
            }
        }
        println!("  polynomial = {}", counterexample.polynomial);
        println!("  h* = {}", counterexample.hstar);
        std::process::exit(2);
    }
}

fn parse_flag_arg(raw: Option<&str>, label: &str) -> Result<Option<Vec<u32>>, String> {
    let Some(raw) = raw else {
        return Ok(None);
    };
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(Some(Vec::new()));
    }

    let mut parsed = Vec::new();
    for (index, token) in raw.split(',').enumerate() {
        let token = token.trim();
        if token.is_empty() {
            return Err(format!(
                "{label}: empty entry at position {}",
                index.saturating_add(1)
            ));
        }
        let value = token
            .parse::<u32>()
            .map_err(|_| format!("{label}: invalid value at position {}: {token}", index + 1))?;
        parsed.push(value);
    }
    Ok(Some(parsed))
}

fn normalized_flags(base: Option<&Vec<u32>>, parts: usize, pad_with: u32) -> Vec<u32> {
    let Some(base) = base else {
        return Vec::new();
    };

    let mut flags = base.clone();
    if flags.len() > parts {
        flags.truncate(parts);
    } else if flags.len() < parts {
        flags.resize(parts, pad_with);
    }
    flags
}

fn flags_to_string(flags: &[u32]) -> String {
    if flags.is_empty() {
        String::new()
    } else {
        flags
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn as_slice_opt(flags: &[u32]) -> Option<&[u32]> {
    if flags.is_empty() {
        None
    } else {
        Some(flags)
    }
}

fn requested_flags_missing(
    requested: Option<&Vec<u32>>,
    normalized: Option<&[u32]>,
    flags_required: bool,
) -> bool {
    requested.is_some() && flags_required && normalized.is_none()
}

fn report_status(
    has_counterexample: bool,
    capped: bool,
    dry_run: bool,
    totals: &ScanTotals,
) -> &'static str {
    if has_counterexample {
        "counterexample_found"
    } else if capped {
        "capped"
    } else if totals.cached_unverified > 0 || totals.compute_errors > 0 || totals.db_errors > 0 {
        "incomplete"
    } else if dry_run {
        "dry_run"
    } else {
        "completed"
    }
}

fn skip_by_weight_filters(weight: &Partition, args: &Args) -> Option<SkipReason> {
    if args.skip_single_part && weight.num_parts() == 1 {
        return Some(SkipReason::SinglePart);
    }
    if args.skip_all_ones && weight.parts().iter().all(|&part| part == 1) {
        return Some(SkipReason::AllOnes);
    }
    if let Some(min_weight_parts) = args.min_weight_parts {
        if weight.num_parts() < min_weight_parts {
            return Some(SkipReason::MinWeightParts);
        }
    }
    if let Some(max_weight_parts) = args.max_weight_parts {
        if weight.num_parts() > max_weight_parts {
            return Some(SkipReason::MaxWeightParts);
        }
    }
    if let Some(max_weight_part) = args.max_weight_part {
        if weight.part(0) > max_weight_part {
            return Some(SkipReason::MaxWeightPart);
        }
    }
    if let Some(max_nonones) = args.max_nonones {
        let nonones = weight.parts().iter().filter(|&&part| part > 1).count();
        if nonones > max_nonones {
            return Some(SkipReason::MaxNonOnes);
        }
    }
    None
}

fn dominance_holds(lambda: &Partition, weight: &Partition) -> bool {
    let mut lambda_sum = 0u32;
    let mut weight_sum = 0u32;
    let max_parts = lambda.num_parts().max(weight.num_parts());
    for index in 0..max_parts {
        lambda_sum = lambda_sum.saturating_add(lambda.part(index));
        weight_sum = weight_sum.saturating_add(weight.part(index));
        if weight_sum > lambda_sum {
            return false;
        }
    }
    true
}

fn inner_shapes(lambda: &Partition) -> Vec<Partition> {
    let mut result = vec![Partition::empty()];
    for size in 1..lambda.size() {
        for mu in Partition::all_of_size_bounded(size, lambda.num_parts(), lambda.parts()[0]) {
            if mu.partition_less_equal(lambda) && valid_skew(lambda, &mu) {
                result.push(mu);
            }
        }
    }
    result
}

fn valid_skew(lambda: &Partition, mu: &Partition) -> bool {
    for row in 0..lambda.num_parts() {
        if lambda.part(row) <= mu.part(row) {
            return false;
        }
    }
    for row in 0..lambda.num_parts().saturating_sub(1) {
        if mu.part(row) >= lambda.part(row + 1) {
            return false;
        }
    }
    true
}

fn load_cached_row(
    pool: &Pool,
    lambda: &str,
    mu: &str,
    weight: &str,
    upper_flags: &str,
    lower_flags: &str,
) -> Result<Option<CachedRow>, String> {
    let mut conn = pool
        .get_conn()
        .map_err(|err| format!("db connection failed: {err}"))?;
    let row: Option<(String, Option<i64>, String, Option<String>)> = conn
        .exec_first(
            r"SELECT kostka, dimension, polynomial, hstar
              FROM gt_ehrhart
              WHERE lambda = ? AND mu = ? AND weight = ?
                AND upper_flags = ? AND lower_flags = ?",
            (lambda, mu, weight, upper_flags, lower_flags),
        )
        .map_err(|err| format!("cached-row query failed: {err}"))?;
    Ok(row.map(|(kostka, dimension, polynomial, hstar)| CachedRow {
        kostka,
        dimension,
        polynomial,
        hstar,
    }))
}

fn parse_stored_rational(raw: &str) -> Result<BigRational, String> {
    let raw = raw.trim();
    let raw = raw
        .strip_prefix('(')
        .and_then(|inner| inner.strip_suffix(')'))
        .unwrap_or(raw);
    if let Some((numerator, denominator)) = raw.split_once('/') {
        let numerator = numerator
            .parse::<BigInt>()
            .map_err(|_| format!("invalid rational numerator: {raw}"))?;
        let denominator = denominator
            .parse::<BigInt>()
            .map_err(|_| format!("invalid rational denominator: {raw}"))?;
        if denominator.is_zero() {
            return Err("zero rational denominator".to_string());
        }
        Ok(BigRational::new(numerator, denominator))
    } else {
        raw.parse::<BigInt>()
            .map(BigRational::from)
            .map_err(|_| format!("invalid integer coefficient: {raw}"))
    }
}

fn parse_stored_polynomial(raw: &str) -> Result<EhrhartPoly, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err("empty polynomial".to_string());
    }

    let mut by_degree = BTreeMap::new();
    for term in raw.split(" + ") {
        let term = term.trim();
        let (degree, coefficient) = if let Some((coefficient, exponent)) = term.split_once("n^") {
            let degree = exponent
                .parse::<usize>()
                .map_err(|_| format!("invalid polynomial exponent: {term}"))?;
            let coefficient = if coefficient.is_empty() {
                BigRational::one()
            } else {
                parse_stored_rational(coefficient)?
            };
            (degree, coefficient)
        } else if let Some(coefficient) = term.strip_suffix('n') {
            let coefficient = if coefficient.is_empty() {
                BigRational::one()
            } else {
                parse_stored_rational(coefficient)?
            };
            (1, coefficient)
        } else {
            (0, parse_stored_rational(term)?)
        };
        if by_degree.insert(degree, coefficient).is_some() {
            return Err(format!("duplicate polynomial degree {degree}"));
        }
    }

    let degree = by_degree
        .iter()
        .rev()
        .find_map(|(degree, coefficient)| (!coefficient.is_zero()).then_some(*degree))
        .unwrap_or(0);
    let mut coeffs = vec![BigRational::zero(); degree + 1];
    for (index, coefficient) in by_degree {
        if index > degree {
            if !coefficient.is_zero() {
                return Err("inconsistent polynomial degree".to_string());
            }
        } else {
            coeffs[index] = coefficient;
        }
    }
    Ok(EhrhartPoly { coeffs, degree })
}

fn parse_stored_hstar(raw: &str) -> Result<Vec<BigInt>, String> {
    let values: Vec<serde_json::Value> =
        serde_json::from_str(raw).map_err(|err| format!("invalid h* JSON: {err}"))?;
    values
        .into_iter()
        .map(|value| match value {
            serde_json::Value::String(value) => value
                .parse::<BigInt>()
                .map_err(|_| format!("invalid h* coefficient: {value}")),
            serde_json::Value::Number(value) => value
                .to_string()
                .parse::<BigInt>()
                .map_err(|_| format!("invalid h* coefficient: {value}")),
            _ => Err("h* coefficients must be integers or integer strings".to_string()),
        })
        .collect()
}

fn validate_cached_row(
    context: CaseContext<'_>,
    cached: &CachedRow,
) -> Result<Option<CounterexampleRecord>, String> {
    let stored_dimension = cached
        .dimension
        .ok_or_else(|| "cached dimension is missing".to_string())?;
    let stored_dimension = usize::try_from(stored_dimension)
        .map_err(|_| "cached dimension is negative or too large".to_string())?;
    if stored_dimension != context.gt_dimension {
        return Err(format!(
            "cached dimension {stored_dimension} differs from derived dimension {}",
            context.gt_dimension
        ));
    }

    let poly = parse_stored_polynomial(&cached.polynomial)?;
    if poly.degree != context.gt_dimension {
        return Err(format!(
            "cached polynomial degree {} differs from derived dimension {}",
            poly.degree, context.gt_dimension
        ));
    }
    if poly.eval(0) != BigRational::one() {
        return Err("cached polynomial does not have constant term 1".to_string());
    }
    for dilation in 0..=poly.degree {
        if !poly.eval(dilation as u64).is_integer() {
            return Err(format!(
                "cached polynomial is nonintegral at dilation {dilation}"
            ));
        }
    }

    let stored_kostka = cached
        .kostka
        .parse::<BigInt>()
        .map_err(|_| "cached Kostka value is not an integer".to_string())?;
    if poly.eval(1).to_integer() != stored_kostka {
        return Err("cached Kostka value differs from polynomial L(1)".to_string());
    }

    let stored_hstar = parse_stored_hstar(
        cached
            .hstar
            .as_deref()
            .ok_or_else(|| "cached h* vector is missing".to_string())?,
    )?;
    let recomputed_hstar = compute_hstar(&poly);
    if stored_hstar != recomputed_hstar {
        return Err("cached h* vector differs from polynomial-derived h*".to_string());
    }

    Ok(first_violation_record(context, &poly))
}

fn unverified_record(context: CaseContext<'_>, reason: String) -> UnverifiedRecord {
    UnverifiedRecord {
        size: context.size,
        lambda: context.lambda.parts().to_vec(),
        mu: context.mu.parts().to_vec(),
        weight: context.weight.parts().to_vec(),
        upper_flags: context.upper_flags.to_string(),
        lower_flags: context.lower_flags.to_string(),
        reason,
    }
}

fn first_negative_record(
    context: CaseContext<'_>,
    poly: &EhrhartPoly,
) -> Option<CounterexampleRecord> {
    let first_negative = poly
        .coeffs
        .iter()
        .enumerate()
        .find(|(_, coeff)| *coeff < &BigRational::zero())
        .map(|(index, coeff)| (index, coeff.to_string()))?;

    let hstar = compute_hstar(poly);
    Some(CounterexampleRecord {
        size: context.size,
        lambda: context.lambda.parts().to_vec(),
        mu: context.mu.parts().to_vec(),
        weight: context.weight.parts().to_vec(),
        upper_flags: context.upper_flags.to_string(),
        lower_flags: context.lower_flags.to_string(),
        gt_dimension: context.gt_dimension,
        polynomial_degree: poly.degree,
        failure_kind: "negative_coefficient".to_string(),
        first_negative_degree: Some(first_negative.0),
        first_negative_coefficient: Some(first_negative.1),
        failure_family: None,
        failure_name: None,
        failure_index: None,
        failure_lhs: None,
        failure_rhs: None,
        failure_formula: None,
        polynomial: poly.display(),
        hstar: hstar_json(&hstar),
    })
}

#[derive(Debug)]
struct HibiStanleyFailure {
    family: String,
    name: String,
    index: usize,
    lhs: BigInt,
    rhs: BigInt,
    formula: String,
}

fn first_hibi_stanley_failure(hstar: &[BigInt], dimension: usize) -> Option<HibiStanleyFailure> {
    let mut padded = if hstar.len() <= dimension {
        let mut v = hstar.to_vec();
        v.resize(dimension + 1, BigInt::zero());
        v
    } else {
        hstar.to_vec()
    };
    while padded.last().is_some_and(|c| c.is_zero()) {
        padded.pop();
    }
    if padded.is_empty() {
        return None;
    }

    let coeff = |vec: &Vec<BigInt>, index: usize| -> BigInt {
        vec.get(index).cloned().unwrap_or_else(BigInt::zero)
    };
    let sum_range = |vec: &Vec<BigInt>, start: usize, end: usize| -> BigInt {
        if start > end {
            return BigInt::zero();
        }
        (start..=end).fold(BigInt::zero(), |acc, i| acc + coeff(vec, i))
    };
    let degree = padded
        .iter()
        .rposition(|value| !value.is_zero())
        .unwrap_or(0);

    if dimension >= 3 {
        for i in 1..=(dimension - 1) / 2 {
            let lhs = sum_range(&padded, dimension - i, dimension - 1);
            let rhs = sum_range(&padded, 2, i + 1);
            if lhs > rhs {
                return Some(HibiStanleyFailure {
                    family: "Hibi".to_string(),
                    name: "tail-sum".to_string(),
                    index: i,
                    lhs,
                    rhs,
                    formula: "h*_{d-1}+...+h*_{d-i} <= h*_2+...+h*_{i+1}".to_string(),
                });
            }
        }
    }

    for i in 0..=degree / 2 {
        let lhs = sum_range(&padded, 0, i);
        let rhs = sum_range(&padded, degree.saturating_sub(i), degree);
        if lhs > rhs {
            return Some(HibiStanleyFailure {
                family: "Stanley".to_string(),
                name: "partial-sum".to_string(),
                index: i,
                lhs,
                rhs,
                formula: "h*_0+...+h*_i <= h*_{s-i}+...+h*_s with s=deg(h*)".to_string(),
            });
        }
    }

    if coeff(&padded, dimension) > BigInt::zero() && dimension >= 3 {
        for i in 2..dimension {
            let lhs = coeff(&padded, 1);
            let rhs = coeff(&padded, i);
            if lhs > rhs {
                return Some(HibiStanleyFailure {
                    family: "Hibi".to_string(),
                    name: "interior-point".to_string(),
                    index: i,
                    lhs,
                    rhs,
                    formula: "if h*_d>0 then h*_1 <= h*_i".to_string(),
                });
            }
        }
    }

    None
}

fn first_violation_record(
    context: CaseContext<'_>,
    poly: &EhrhartPoly,
) -> Option<CounterexampleRecord> {
    if let Some(counterexample) = first_negative_record(context, poly) {
        return Some(counterexample);
    }

    let hstar = compute_hstar(poly);
    if let Some(failure) = first_hibi_stanley_failure(&hstar, context.gt_dimension) {
        return Some(CounterexampleRecord {
            size: context.size,
            lambda: context.lambda.parts().to_vec(),
            mu: context.mu.parts().to_vec(),
            weight: context.weight.parts().to_vec(),
            upper_flags: context.upper_flags.to_string(),
            lower_flags: context.lower_flags.to_string(),
            gt_dimension: context.gt_dimension,
            polynomial_degree: poly.degree,
            failure_kind: "hibi_stanley".to_string(),
            first_negative_degree: None,
            first_negative_coefficient: None,
            failure_family: Some(failure.family),
            failure_name: Some(failure.name),
            failure_index: Some(failure.index),
            failure_lhs: Some(failure.lhs.to_string()),
            failure_rhs: Some(failure.rhs.to_string()),
            failure_formula: Some(failure.formula),
            polynomial: poly.display(),
            hstar: hstar_json(&hstar),
        });
    }

    None
}

fn hstar_json(hstar: &[BigInt]) -> String {
    let entries = hstar
        .iter()
        .map(|entry| format!("\"{entry}\""))
        .collect::<Vec<_>>();
    format!("[{}]", entries.join(","))
}

fn insert_row(
    pool: &Pool,
    db_case: DbCase<'_>,
    poly: &EhrhartPoly,
    elapsed_ms: Option<u64>,
) -> Result<(), String> {
    let hstar = hstar_json(&compute_hstar(poly));
    let polynomial = poly.display();
    let kostka_rational = poly.eval(1);
    let kostka = if kostka_rational.denom() == &BigInt::from(1u8) {
        kostka_rational.to_integer().to_string()
    } else {
        return Err(format!(
            "L(1) denominator is not 1: {}/{}",
            kostka_rational.numer(),
            kostka_rational.denom()
        ));
    };

    let mut conn = pool
        .get_conn()
        .map_err(|err| format!("db conn failed: {err}"))?;
    conn.exec_drop(
        r"INSERT IGNORE INTO gt_ehrhart
          (n, lambda, mu, weight, upper_flags, lower_flags,
           kostka, dimension, polynomial, hstar, elapsed_ms)
          VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        (
            db_case.n,
            db_case.lambda,
            db_case.mu,
            db_case.weight,
            db_case.upper_flags,
            db_case.lower_flags,
            kostka,
            poly.degree as i32,
            polynomial,
            hstar,
            elapsed_ms,
        ),
    )
    .map_err(|err| format!("insert failed: {err}"))?;
    Ok(())
}

fn partition_string(partition: &Partition) -> String {
    partition_string_from_parts(partition.parts())
}

fn partition_string_from_parts(parts: &[u32]) -> String {
    if parts.is_empty() {
        String::new()
    } else {
        parts
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square_context<'a>(
        lambda: &'a Partition,
        mu: &'a Partition,
        weight: &'a Partition,
    ) -> CaseContext<'a> {
        CaseContext {
            size: 2,
            lambda,
            mu,
            weight,
            gt_dimension: 2,
            upper_flags: "",
            lower_flags: "",
        }
    }

    #[test]
    fn upper_and_lower_flag_requests_are_validated_independently() {
        let empty = Vec::new();
        let present = vec![2];

        assert!(requested_flags_missing(Some(&empty), None, true));
        assert!(!requested_flags_missing(
            Some(&present),
            Some(&present),
            true
        ));
        assert!(!requested_flags_missing(None, None, true));
        assert!(!requested_flags_missing(Some(&empty), None, false));

        let upper_missing = requested_flags_missing(Some(&empty), None, true);
        let lower_present = requested_flags_missing(Some(&present), Some(&present), true);
        assert!(upper_missing || lower_present);

        let upper_present = requested_flags_missing(Some(&present), Some(&present), true);
        let lower_missing = requested_flags_missing(Some(&empty), None, true);
        assert!(upper_present || lower_missing);
    }

    #[test]
    fn capped_and_unverified_scans_are_not_completed() {
        let totals = ScanTotals::default();
        assert_eq!(report_status(false, true, false, &totals), "capped");

        let totals = ScanTotals {
            cached_unverified: 1,
            ..ScanTotals::default()
        };
        assert_eq!(report_status(false, false, false, &totals), "incomplete");
        assert_eq!(
            report_status(false, false, false, &ScanTotals::default()),
            "completed"
        );
    }

    #[test]
    fn hibi_interior_range_excludes_endpoints_for_the_square() {
        let hstar = vec![BigInt::from(1), BigInt::from(6), BigInt::from(1)];
        assert!(first_hibi_stanley_failure(&hstar, 2).is_none());
    }

    #[test]
    fn stored_polynomial_parser_round_trips_engine_display() {
        let poly = EhrhartPoly {
            coeffs: vec![
                BigRational::one(),
                BigRational::new(BigInt::from(-1), BigInt::from(2)),
                BigRational::new(BigInt::from(3), BigInt::from(2)),
            ],
            degree: 2,
        };
        let parsed = parse_stored_polynomial(&poly.display()).expect("parse displayed polynomial");
        assert_eq!(parsed.coeffs, poly.coeffs);
        assert_eq!(parsed.degree, poly.degree);
    }

    #[test]
    fn cached_rows_are_revalidated_before_being_counted_as_verified() {
        let lambda = Partition::new(vec![2]);
        let mu = Partition::empty();
        let weight = Partition::new(vec![1, 1]);
        let context = square_context(&lambda, &mu, &weight);
        let valid = CachedRow {
            kostka: "9".to_string(),
            dimension: Some(2),
            polynomial: "4n^2 + 4n + 1".to_string(),
            hstar: Some("[\"1\",\"6\",\"1\"]".to_string()),
        };
        assert!(validate_cached_row(context, &valid)
            .expect("valid cached row")
            .is_none());

        let wrong_polynomial = CachedRow {
            polynomial: "4n^2 + 3n + 1".to_string(),
            ..valid
        };
        assert!(validate_cached_row(context, &wrong_polynomial)
            .expect_err("mismatched polynomial must be unverified")
            .contains("Kostka"));

        let wrong_hstar = CachedRow {
            kostka: "9".to_string(),
            dimension: Some(2),
            polynomial: "4n^2 + 4n + 1".to_string(),
            hstar: Some("[\"1\",\"5\",\"1\"]".to_string()),
        };
        assert!(validate_cached_row(context, &wrong_hstar)
            .expect_err("mismatched h* must be unverified")
            .contains("polynomial-derived"));
    }
}

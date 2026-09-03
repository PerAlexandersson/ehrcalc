//! Key h*-polynomial scans for the Key-HStar-Bruhat project.
//!
//! The ordinary `key` command uses the general Kogan-face engine.  This module
//! uses a faster principal-specialization Demazure evaluator, because the scan
//! only needs `K_{k lambda, sigma}(1^n)` for every permutation in one rank.

use crate::exact::{format_rational, EhrhartData, EhrhartPolynomial, ExactResult};
use crate::render::OutputFormat;
use hashbrown::HashMap;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use polytool::real_rootedness::{
    check_weak_interlacing_bigint_coeffs, is_real_rooted_bigint_coeffs,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Packet/check output requested by the key scan.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum KeyPacketMode {
    /// Use D-route checks for staircase scans and h*-cover checks otherwise.
    Auto,
    /// Only compute row data: Ehrhart, h*, B, optional D, and lower covers.
    None,
    /// Compute h*-coefficient, real-rootedness, and cover-interlacing checks.
    HChecks,
    /// Also compute the D/G/W/barK/L route and exact polytool checks.
    DRoute,
}

/// Input for a key scan.
#[derive(Clone, Debug)]
pub struct KeyScanInput {
    pub n: Option<usize>,
    pub max_n: Option<usize>,
    pub staircase: bool,
    pub lambda: Option<Vec<u32>>,
    pub gaps: Option<Vec<u32>>,
    pub sigma: Option<Vec<usize>>,
    pub packets: KeyPacketMode,
    pub include_rows: bool,
    pub include_block_summary: bool,
    pub block_representatives: bool,
    pub start_index: usize,
    pub limit: Option<usize>,
    pub checkpoint: Option<PathBuf>,
    pub resume: Option<PathBuf>,
    pub sample_checkpoint: Option<PathBuf>,
    pub format: OutputFormat,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScanSpec {
    lambda: Vec<u32>,
    specialization: &'static str,
}

#[derive(Clone, Debug)]
struct Cover {
    tau: Vec<usize>,
    i: usize,
    j: usize,
    weight: i64,
}

#[derive(Clone, Debug)]
struct ScanCover {
    tau: Vec<usize>,
    i: usize,
    j: usize,
    weight: i64,
}

#[derive(Clone, Debug)]
struct ScanRow {
    index: usize,
    sigma: Vec<usize>,
    length: usize,
    dimension: usize,
    ehrhart_power: Vec<String>,
    hstar: Vec<BigInt>,
    b: Vec<BigInt>,
    d: Option<Vec<BigInt>>,
    lower_covers: Vec<ScanCover>,
}

#[derive(Clone, Debug)]
struct LambdaBlock {
    start: usize,
    end: usize,
    value: u32,
}

#[derive(Clone, Debug)]
struct KeyEvalPlan {
    sorted_lambda: Vec<u32>,
    initial_live: Vec<usize>,
    steps: Vec<KeyEvalStep>,
    max_live_len: usize,
}

#[derive(Clone, Debug)]
struct KeyEvalStep {
    op: usize,
    pos_i: usize,
    pos_ip1: usize,
    live_next: Vec<usize>,
    projection: Vec<(usize, usize)>,
}

#[derive(Clone, Debug)]
struct BlockMatrixVariant {
    row_count: usize,
    example_sigma: Vec<usize>,
    dimension: usize,
    hstar: Vec<BigInt>,
}

#[derive(Clone, Debug)]
struct BlockMatrixClass {
    matrix: Vec<Vec<usize>>,
    row_count: usize,
    example_sigma: Vec<usize>,
    variants: Vec<BlockMatrixVariant>,
}

#[derive(Clone, Debug)]
struct BlockSummary {
    representatives_only: bool,
    block_values: Vec<u32>,
    block_sizes: Vec<usize>,
    matrix_classes: Vec<BlockMatrixClass>,
    hstar_classes: usize,
    matrix_failures: Option<usize>,
}

#[derive(Clone, Debug)]
struct RankScan {
    n: usize,
    spec: ScanSpec,
    complete: bool,
    computed_this_run: usize,
    skipped_from_resume: usize,
    rows_total: usize,
    elapsed_ms: u128,
    block_representatives: bool,
    selected_sigma: Option<Vec<usize>>,
    rows: Vec<ScanRow>,
    h_summary: Option<HCheckSummary>,
    route_summary: Option<RouteSummary>,
}

#[derive(Clone, Debug)]
struct HCheckSummary {
    h_coefficientwise_nonnegative: Counter,
    h_sigma_real_rooted: Counter,
    h_cover_interlacing: Counter,
}

#[derive(Clone, Debug)]
struct RouteSummary {
    h_coefficientwise_nonnegative: Counter,
    d_sigma_real_rooted: Counter,
    d_cover_interlacing: Counter,
    w_interlaces_g: Counter,
    deleted_w_interlaces_g: Counter,
    l_interlaces_d_all_covers: Counter,
    each_upper_has_good_l: Counter,
    max_weight_l_interlaces_d: Counter,
    max_weight_l_interlaces_g: Counter,
    max_weight_l_interlaces_w: Counter,
    bark_coefficientwise_nonnegative: Counter,
    bark_interlaces_d_tau: Counter,
    bark_interlaces_d_alpha: Counter,
    bark_interlaces_g: Counter,
}

#[derive(Clone, Debug)]
struct Counter {
    label: &'static str,
    pass: usize,
    total: usize,
    first_failure: Option<String>,
}

impl Counter {
    fn new(label: &'static str) -> Self {
        Self {
            label,
            pass: 0,
            total: 0,
            first_failure: None,
        }
    }

    fn record(&mut self, ok: bool, context: impl FnOnce() -> String) {
        self.total += 1;
        if ok {
            self.pass += 1;
        } else if self.first_failure.is_none() {
            self.first_failure = Some(context());
        }
    }

    fn text_line(&self) -> String {
        let mut line = format!("{}: {}/{}", self.label, self.pass, self.total);
        if let Some(first_failure) = &self.first_failure {
            line.push_str(&format!("\n  first failure: {first_failure}"));
        }
        line
    }

    fn json(&self) -> Value {
        json!({
            "label": self.label,
            "pass": self.pass,
            "total": self.total,
            "first_failure": self.first_failure,
        })
    }
}

/// Run a key scan and render it.
pub fn run_key_scan(input: &KeyScanInput) -> ExactResult<String> {
    validate_scan_input(input)?;
    let ranks = scan_ranks(input)?;
    render_scan(
        &ranks,
        input.format,
        input.include_rows,
        input.include_block_summary,
    )
}

fn validate_scan_input(input: &KeyScanInput) -> ExactResult<()> {
    match (input.n, input.max_n) {
        (Some(_), Some(_)) => Err("use either --n or --max-n, not both".to_string()),
        (None, None) => Err("key-scan requires --n or --max-n".to_string()),
        _ => Ok(()),
    }?;
    if input.lambda.is_some() && input.gaps.is_some() {
        return Err("use at most one of --lambda and --gaps".to_string());
    }
    if input.staircase && (input.lambda.is_some() || input.gaps.is_some()) {
        return Err("use --staircase, --lambda, or --gaps, not more than one".to_string());
    }
    if !input.staircase && input.lambda.is_none() && input.gaps.is_none() {
        return Err("key-scan requires one of --staircase, --lambda, or --gaps".to_string());
    }
    if input.max_n.is_some() && (input.lambda.is_some() || input.gaps.is_some()) {
        return Err("--lambda and --gaps require --n, not --max-n".to_string());
    }
    if let Some(n) = input.n {
        validate_scan_rank(n)?;
    }
    if let Some(max_n) = input.max_n {
        validate_scan_rank(max_n)?;
    }
    if input.start_index > 0 && input.n.is_none() {
        return Err("--start-index is only supported with --n".to_string());
    }
    if input.limit.is_some() && input.n.is_none() {
        return Err("--limit is only supported with --n".to_string());
    }
    if let Some(sigma) = &input.sigma {
        let Some(n) = input.n else {
            return Err("--sigma is only supported with --n".to_string());
        };
        validate_scan_sigma(sigma, n)?;
    }
    if input.resume.is_some() && input.n.is_none() {
        return Err("--resume is only supported with --n".to_string());
    }
    if input.checkpoint.is_some() && input.n.is_none() {
        return Err("--checkpoint is only supported with --n".to_string());
    }
    if input.sample_checkpoint.is_some() {
        if input.n.is_none() || input.sigma.is_none() {
            return Err("--sample-checkpoint is only supported with --n and --sigma".to_string());
        }
        if input.resume.is_some() {
            return Err("--sample-checkpoint cannot be combined with --resume".to_string());
        }
        if input.checkpoint.is_some() {
            return Err("--sample-checkpoint cannot be combined with --checkpoint".to_string());
        }
    }
    if input.block_representatives {
        if input.n.is_none() {
            return Err("--block-representatives is only supported with --n".to_string());
        }
        if input.sigma.is_some() {
            return Err("--block-representatives cannot be combined with --sigma".to_string());
        }
        if input.start_index != 0 || input.limit.is_some() {
            return Err(
                "--block-representatives cannot be combined with --start-index or --limit"
                    .to_string(),
            );
        }
        if input.checkpoint.is_some() || input.resume.is_some() {
            return Err(
                "--block-representatives cannot be combined with --checkpoint or --resume"
                    .to_string(),
            );
        }
    }
    if input.sigma.is_some() && (input.start_index != 0 || input.limit.is_some()) {
        return Err("--sigma cannot be combined with --start-index or --limit".to_string());
    }
    Ok(())
}

fn validate_scan_rank(n: usize) -> ExactResult<()> {
    if n == 0 {
        return Err("key-scan ranks start at n=1".to_string());
    }
    if n > 9 {
        return Err(
            "key-scan currently supports n <= 9 because row identifiers use compact digit strings"
                .to_string(),
        );
    }
    Ok(())
}

fn validate_scan_sigma(sigma: &[usize], n: usize) -> ExactResult<()> {
    if sigma.len() != n {
        return Err(format!(
            "--sigma has length {}, but S_{n} needs length {n}",
            sigma.len()
        ));
    }
    let mut sorted = sigma.to_vec();
    sorted.sort_unstable();
    let expected = (1..=n).collect::<Vec<_>>();
    if sorted != expected {
        return Err(format!(
            "--sigma must be a one-based permutation of 1,...,{n}"
        ));
    }
    Ok(())
}

fn scan_ranks(input: &KeyScanInput) -> ExactResult<Vec<RankScan>> {
    if let Some(n) = input.n {
        Ok(vec![scan_rank(n, input)?])
    } else {
        let max_n = input.max_n.expect("validated max_n");
        let mut scans = Vec::new();
        for n in 1..=max_n {
            scans.push(scan_rank(n, input)?);
        }
        Ok(scans)
    }
}

fn scan_rank(n: usize, input: &KeyScanInput) -> ExactResult<RankScan> {
    let started = Instant::now();
    let spec = resolve_scan_spec(n, input)?;
    let packet_mode = effective_packet_mode(input.packets, &spec)?;
    let permutations = all_permutations(n);
    let rows_total = permutations.len();
    let mut rows_by_sigma = if let Some(path) = &input.resume {
        read_checkpoint(path, n, &spec)?
    } else {
        BTreeMap::new()
    };
    let resumed = rows_by_sigma.len();

    let selected = if input.block_representatives {
        block_representative_indices(&permutations, &spec.lambda)
    } else if let Some(sigma) = &input.sigma {
        selected_sigma_index(&permutations, sigma)?
    } else {
        selected_indices(rows_total, input.start_index, input.limit)
    };
    let selected_set = selected.iter().copied().collect::<BTreeSet<_>>();
    let mut checkpoint = if let Some(path) = &input.checkpoint {
        Some(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|error| {
                    format!("could not open checkpoint {}: {error}", path.display())
                })?,
        )
    } else {
        None
    };

    let mut computed_this_run = 0;
    for (index, sigma) in permutations.iter().enumerate() {
        if !selected_set.contains(&index) || rows_by_sigma.contains_key(sigma) {
            continue;
        }
        let sample_checkpoint = input.sample_checkpoint.as_ref().filter(|_| {
            input
                .sigma
                .as_ref()
                .is_some_and(|selected| selected == sigma)
        });
        let row = compute_key_scan_row(index, sigma, &spec, sample_checkpoint)?;
        if let Some(file) = checkpoint.as_mut() {
            writeln!(file, "{}", row_json(&row, &spec))
                .map_err(|error| format!("could not write checkpoint row: {error}"))?;
            file.flush()
                .map_err(|error| format!("could not flush checkpoint row: {error}"))?;
        }
        rows_by_sigma.insert(sigma.clone(), row);
        computed_this_run += 1;
        if computed_this_run % 20 == 0 {
            eprintln!(
                "  computed {} new rows for S_{} ({} total available, {:.1?})",
                computed_this_run,
                n,
                rows_by_sigma.len(),
                started.elapsed()
            );
        }
    }

    let complete = !input.block_representatives && rows_by_sigma.len() == rows_total;
    let h_summary =
        if complete && matches!(packet_mode, KeyPacketMode::HChecks | KeyPacketMode::DRoute) {
            Some(compute_h_summary(&permutations, &rows_by_sigma)?)
        } else {
            None
        };
    let route_summary = if complete && packet_mode == KeyPacketMode::DRoute {
        Some(compute_route_summary(n, &permutations, &rows_by_sigma)?)
    } else {
        None
    };
    let rows = rows_by_sigma.into_values().collect::<Vec<_>>();
    Ok(RankScan {
        n,
        spec,
        complete,
        computed_this_run,
        skipped_from_resume: resumed,
        rows_total,
        elapsed_ms: started.elapsed().as_millis(),
        block_representatives: input.block_representatives,
        selected_sigma: input.sigma.clone(),
        rows,
        h_summary,
        route_summary,
    })
}

fn selected_indices(rows_total: usize, start_index: usize, limit: Option<usize>) -> Vec<usize> {
    let end = limit
        .map(|limit| start_index.saturating_add(limit))
        .unwrap_or(rows_total)
        .min(rows_total);
    (start_index.min(rows_total)..end).collect()
}

fn selected_sigma_index(permutations: &[Vec<usize>], sigma: &[usize]) -> ExactResult<Vec<usize>> {
    let index = permutations
        .iter()
        .position(|candidate| candidate == sigma)
        .ok_or_else(|| format!("--sigma {} is not in this rank", perm_string(sigma)))?;
    Ok(vec![index])
}

fn block_representative_indices(permutations: &[Vec<usize>], lambda: &[u32]) -> Vec<usize> {
    let blocks = lambda_blocks(lambda);
    let mut value_to_block = vec![0usize; lambda.len() + 1];
    let mut position_to_block = vec![0usize; lambda.len()];
    for (block_index, block) in blocks.iter().enumerate() {
        value_to_block[block.start + 1..=block.end].fill(block_index);
        position_to_block[block.start..block.end].fill(block_index);
    }

    let mut seen = BTreeSet::new();
    let mut selected = Vec::new();
    for (index, sigma) in permutations.iter().enumerate() {
        let matrix = block_matrix(sigma, &value_to_block, &position_to_block, blocks.len());
        if seen.insert(matrix) {
            selected.push(index);
        }
    }
    selected
}

fn resolve_scan_spec(n: usize, input: &KeyScanInput) -> ExactResult<ScanSpec> {
    if input.staircase {
        return Ok(ScanSpec {
            lambda: staircase_lambda(n),
            specialization: "staircase",
        });
    }
    if let Some(lambda) = &input.lambda {
        return Ok(ScanSpec {
            lambda: normalize_lambda(lambda, n)?,
            specialization: "lambda",
        });
    }
    if let Some(gaps) = &input.gaps {
        return Ok(ScanSpec {
            lambda: lambda_from_gaps(gaps, n)?,
            specialization: "gaps",
        });
    }
    Err("key-scan requires one of --staircase, --lambda, or --gaps".to_string())
}

fn effective_packet_mode(mode: KeyPacketMode, spec: &ScanSpec) -> ExactResult<KeyPacketMode> {
    match mode {
        KeyPacketMode::Auto if spec.is_staircase() => Ok(KeyPacketMode::DRoute),
        KeyPacketMode::Auto => Ok(KeyPacketMode::HChecks),
        KeyPacketMode::DRoute if !spec.is_staircase() => Err(
            "--packets d-route is currently staircase-only; use --packets h-checks or none"
                .to_string(),
        ),
        other => Ok(other),
    }
}

fn staircase_lambda(n: usize) -> Vec<u32> {
    (0..n).map(|j| (n - 1 - j) as u32).collect()
}

fn normalize_lambda(lambda: &[u32], n: usize) -> ExactResult<Vec<u32>> {
    if lambda.is_empty() {
        return Err("--lambda must not be empty".to_string());
    }
    if lambda.len() > n {
        return Err(format!(
            "--lambda has length {}, but S_{n} needs at most {n} parts",
            lambda.len()
        ));
    }
    let mut padded = lambda.to_vec();
    padded.resize(n, 0);
    validate_dominant_lambda(&padded)?;
    Ok(padded)
}

fn lambda_from_gaps(gaps: &[u32], n: usize) -> ExactResult<Vec<u32>> {
    if gaps.len() != n.saturating_sub(1) {
        return Err(format!(
            "--gaps has length {}, but S_{n} needs {} gaps",
            gaps.len(),
            n.saturating_sub(1)
        ));
    }
    let mut lambda = vec![0u32; n];
    for index in (0..gaps.len()).rev() {
        lambda[index] = lambda[index + 1]
            .checked_add(gaps[index])
            .ok_or_else(|| format!("gap sum overflows at position {}", index + 1))?;
    }
    Ok(lambda)
}

fn validate_dominant_lambda(lambda: &[u32]) -> ExactResult<()> {
    for index in 0..lambda.len().saturating_sub(1) {
        if lambda[index] < lambda[index + 1] {
            return Err(format!(
                "--lambda must be weakly decreasing, but part {} is {} < {}",
                index + 1,
                lambda[index],
                lambda[index + 1]
            ));
        }
    }
    Ok(())
}

fn lambda_gaps(lambda: &[u32]) -> Vec<u32> {
    lambda
        .windows(2)
        .map(|window| window[0] - window[1])
        .collect()
}

fn lambda_blocks(lambda: &[u32]) -> Vec<LambdaBlock> {
    if lambda.is_empty() {
        return Vec::new();
    }
    let mut blocks = Vec::new();
    let mut start = 0;
    let mut value = lambda[0];
    for (index, &part) in lambda.iter().enumerate().skip(1) {
        if part != value {
            blocks.push(LambdaBlock {
                start,
                end: index,
                value,
            });
            start = index;
            value = part;
        }
    }
    blocks.push(LambdaBlock {
        start,
        end: lambda.len(),
        value,
    });
    blocks
}

impl ScanSpec {
    fn is_staircase(&self) -> bool {
        self.lambda == staircase_lambda(self.lambda.len())
    }

    fn common_part_scale(&self) -> Option<u32> {
        let scale = self
            .lambda
            .iter()
            .copied()
            .filter(|&part| part != 0)
            .fold(0, gcd_u32);
        (scale > 1).then_some(scale)
    }

    fn display(&self) -> String {
        format!(
            "{} lambda=[{}] gaps=[{}]",
            self.specialization,
            join_u32s(&self.lambda),
            join_u32s(&lambda_gaps(&self.lambda))
        )
    }
}

fn compute_key_scan_row(
    index: usize,
    sigma: &[usize],
    spec: &ScanSpec,
    sample_checkpoint: Option<&PathBuf>,
) -> ExactResult<ScanRow> {
    let n = sigma.len();
    let length = inv_count(sigma);
    let ehrhart = if let Some(scale) = spec.common_part_scale() {
        if sample_checkpoint.is_some() {
            return Err(
                "--sample-checkpoint is not supported for common-factor lambda shortcuts"
                    .to_string(),
            );
        }
        scaled_lambda_ehrhart(n, sigma, &spec.lambda, length, scale)?
    } else {
        direct_key_ehrhart(n, sigma, &spec.lambda, length, sample_checkpoint)?
    };
    let dimension = ehrhart.dimension();
    let data = EhrhartData::new(ehrhart)?;
    let b = data.ehrhart.to_binomial_basis()?.coeffs().to_vec();
    let d = if spec.is_staircase() {
        Some(if is_identity(sigma) {
            vec![BigInt::one()]
        } else {
            div_by_one_plus_u(&b)?
        })
    } else {
        None
    };
    Ok(ScanRow {
        index,
        sigma: sigma.to_vec(),
        length,
        dimension,
        ehrhart_power: data
            .ehrhart
            .power_coeffs()
            .iter()
            .map(format_rational)
            .collect(),
        hstar: data.hstar.coeffs().to_vec(),
        b,
        d,
        lower_covers: bruhat_covers(sigma, &spec.lambda)
            .into_iter()
            .map(|cover| ScanCover {
                tau: cover.tau,
                i: cover.i,
                j: cover.j,
                weight: cover.weight,
            })
            .collect(),
    })
}

fn direct_key_ehrhart(
    n: usize,
    sigma: &[usize],
    lambda: &[u32],
    degree_bound: usize,
    sample_checkpoint: Option<&PathBuf>,
) -> ExactResult<EhrhartPolynomial> {
    let values = if is_longest(sigma) && lambda == staircase_lambda(n) {
        let degree = n * (n - 1) / 2;
        (0..=degree)
            .map(|k| BigInt::from(k + 1).pow(degree as u32))
            .collect::<Vec<_>>()
    } else {
        principal_values(n, sigma, lambda, degree_bound, sample_checkpoint)?
    };
    ehrhart_from_values_with_actual_dimension(degree_bound, &values)
}

fn scaled_lambda_ehrhart(
    n: usize,
    sigma: &[usize],
    lambda: &[u32],
    degree_bound: usize,
    scale: u32,
) -> ExactResult<EhrhartPolynomial> {
    let primitive = lambda.iter().map(|part| part / scale).collect::<Vec<_>>();
    let base = direct_key_ehrhart(n, sigma, &primitive, degree_bound, None)?;
    base.dilate_and_shift(i64::from(scale), 0)
}

fn gcd_u32(a: u32, b: u32) -> u32 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn ehrhart_from_values_with_actual_dimension(
    degree_bound: usize,
    values: &[BigInt],
) -> ExactResult<EhrhartPolynomial> {
    let preliminary = EhrhartPolynomial::from_values(degree_bound, values)?;
    let dimension = preliminary.degree();
    EhrhartPolynomial::new(dimension, preliminary.power_coeffs().to_vec())
}

fn principal_values(
    n: usize,
    sigma: &[usize],
    lambda: &[u32],
    degree_bound: usize,
    sample_checkpoint: Option<&PathBuf>,
) -> ExactResult<Vec<BigInt>> {
    let base_alpha = (0..n).map(|j| lambda[sigma[j] - 1]).collect::<Vec<_>>();
    let plan = KeyEvalPlan::new(n, &base_alpha);
    let mut checkpoint = sample_checkpoint
        .map(|path| SampleCheckpoint::open(path, n, sigma, lambda, degree_bound))
        .transpose()?;
    let mut values = Vec::with_capacity(degree_bound + 1);

    for k in 0..=degree_bound {
        if let Some(value) = checkpoint.as_ref().and_then(|checkpoint| checkpoint.get(k)) {
            values.push(value.clone());
            continue;
        }

        let scale = u32::try_from(k).map_err(|_| format!("dilation {k} does not fit in u32"))?;
        let scaled_lambda = plan
            .sorted_lambda
            .iter()
            .enumerate()
            .map(|(index, part)| {
                part.checked_mul(scale).ok_or_else(|| {
                    format!(
                        "dilation overflow for sorted lambda part {}={} and k={k}",
                        index + 1,
                        part
                    )
                })
            })
            .collect::<ExactResult<Vec<_>>>()?;
        let value = key_eval_ones_with_plan(&plan, &scaled_lambda)?;
        if let Some(checkpoint) = checkpoint.as_mut() {
            checkpoint.append(k, &value)?;
        }
        values.push(value);
    }

    Ok(values)
}

struct SampleCheckpoint {
    path: PathBuf,
    n: usize,
    sigma: Vec<usize>,
    lambda: Vec<u32>,
    degree_bound: usize,
    values: BTreeMap<usize, BigInt>,
    file: File,
}

impl SampleCheckpoint {
    fn open(
        path: &PathBuf,
        n: usize,
        sigma: &[usize],
        lambda: &[u32],
        degree_bound: usize,
    ) -> ExactResult<Self> {
        let values = read_sample_checkpoint(path, n, sigma, lambda, degree_bound)?;
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|error| {
                format!(
                    "could not open sample checkpoint {}: {error}",
                    path.display()
                )
            })?;
        Ok(Self {
            path: path.clone(),
            n,
            sigma: sigma.to_vec(),
            lambda: lambda.to_vec(),
            degree_bound,
            values,
            file,
        })
    }

    fn get(&self, k: usize) -> Option<&BigInt> {
        self.values.get(&k)
    }

    fn append(&mut self, k: usize, value: &BigInt) -> ExactResult<()> {
        if let Some(existing) = self.values.get(&k) {
            if existing == value {
                return Ok(());
            }
            return Err(format!(
                "sample checkpoint {} already has k={k} with a different value",
                self.path.display()
            ));
        }
        let rendered =
            serde_json::to_string(&sample_checkpoint_json(self, k, value)).map_err(|error| {
                format!(
                    "could not render sample checkpoint record for {}: {error}",
                    self.path.display()
                )
            })?;
        writeln!(self.file, "{rendered}").map_err(|error| {
            format!(
                "could not write sample checkpoint {}: {error}",
                self.path.display()
            )
        })?;
        self.file.flush().map_err(|error| {
            format!(
                "could not flush sample checkpoint {}: {error}",
                self.path.display()
            )
        })?;
        self.values.insert(k, value.clone());
        Ok(())
    }
}

fn read_sample_checkpoint(
    path: &PathBuf,
    n: usize,
    sigma: &[usize],
    lambda: &[u32],
    degree_bound: usize,
) -> ExactResult<BTreeMap<usize, BigInt>> {
    if !path.exists() {
        return Ok(BTreeMap::new());
    }
    let file = File::open(path).map_err(|error| {
        format!(
            "could not open sample checkpoint {}: {error}",
            path.display()
        )
    })?;
    let mut values = BTreeMap::new();
    for (line_index, line) in BufReader::new(file).lines().enumerate() {
        let line = line.map_err(|error| {
            format!(
                "could not read line {} from {}: {error}",
                line_index + 1,
                path.display()
            )
        })?;
        if line.trim().is_empty() {
            continue;
        }
        let value = serde_json::from_str::<Value>(&line).map_err(|error| {
            format!(
                "invalid sample JSON on line {} of {}: {error}",
                line_index + 1,
                path.display()
            )
        })?;
        validate_sample_checkpoint_metadata(
            &value,
            path,
            line_index + 1,
            n,
            sigma,
            lambda,
            degree_bound,
        )?;
        let k = usize_json_field(&value, "k", "sample checkpoint")?;
        if k > degree_bound {
            return Err(format!(
                "sample checkpoint {} line {} has k={k} beyond degree bound {degree_bound}",
                path.display(),
                line_index + 1
            ));
        }
        let sample_value = value["value"]
            .as_str()
            .ok_or_else(|| {
                format!(
                    "sample checkpoint {} line {} is missing string value",
                    path.display(),
                    line_index + 1
                )
            })?
            .parse::<BigInt>()
            .map_err(|_| {
                format!(
                    "sample checkpoint {} line {} has non-integer value",
                    path.display(),
                    line_index + 1
                )
            })?;
        match values.get(&k) {
            Some(existing) if existing != &sample_value => {
                return Err(format!(
                    "sample checkpoint {} has conflicting values for k={k}",
                    path.display()
                ));
            }
            Some(_) => {}
            None => {
                values.insert(k, sample_value);
            }
        }
    }
    Ok(values)
}

fn validate_sample_checkpoint_metadata(
    value: &Value,
    path: &Path,
    line: usize,
    n: usize,
    sigma: &[usize],
    lambda: &[u32],
    degree_bound: usize,
) -> ExactResult<()> {
    if value["kind"].as_str() != Some("key_scan_sample") {
        return Err(format!(
            "sample checkpoint {} line {line} has wrong kind",
            path.display()
        ));
    }
    if value["family"].as_str() != Some("key") {
        return Err(format!(
            "sample checkpoint {} line {line} has wrong family",
            path.display()
        ));
    }
    let sample_n = usize_json_field(value, "n", "sample checkpoint")?;
    if sample_n != n {
        return Err(format!(
            "sample checkpoint {} line {line} has n={sample_n}, expected {n}",
            path.display()
        ));
    }
    let sample_sigma_text = value["sigma"].as_str().ok_or_else(|| {
        format!(
            "sample checkpoint {} line {line} is missing sigma string",
            path.display()
        )
    })?;
    let sample_sigma = parse_perm_string(sample_sigma_text)?;
    if sample_sigma != sigma {
        return Err(format!(
            "sample checkpoint {} line {line} has sigma {}, expected {}",
            path.display(),
            perm_string(&sample_sigma),
            perm_string(sigma)
        ));
    }
    let sample_lambda = u32_json_array(&value["lambda"], "lambda")?;
    if sample_lambda != lambda {
        return Err(format!(
            "sample checkpoint {} line {line} has lambda [{}], expected [{}]",
            path.display(),
            join_u32s(&sample_lambda),
            join_u32s(lambda)
        ));
    }
    let sample_degree_bound = usize_json_field(value, "degree_bound", "sample checkpoint")?;
    if sample_degree_bound != degree_bound {
        return Err(format!(
            "sample checkpoint {} line {line} has degree_bound={sample_degree_bound}, \
             expected {degree_bound}",
            path.display()
        ));
    }
    Ok(())
}

fn sample_checkpoint_json(checkpoint: &SampleCheckpoint, k: usize, value: &BigInt) -> Value {
    json!({
        "kind": "key_scan_sample",
        "family": "key",
        "n": checkpoint.n,
        "lambda": checkpoint.lambda,
        "gaps": lambda_gaps(&checkpoint.lambda),
        "sigma": perm_string(&checkpoint.sigma),
        "degree_bound": checkpoint.degree_bound,
        "k": k,
        "value": value.to_string(),
    })
}

fn compute_h_summary(
    permutations: &[Vec<usize>],
    rows: &BTreeMap<Vec<usize>, ScanRow>,
) -> ExactResult<HCheckSummary> {
    let mut summary = HCheckSummary {
        h_coefficientwise_nonnegative: Counter::new("H coefficientwise nonnegative"),
        h_sigma_real_rooted: Counter::new("H_sigma real-rooted"),
        h_cover_interlacing: Counter::new("H_tau << H_sigma for Bruhat covers"),
    };
    for sigma in permutations {
        let row = &rows[sigma];
        summary
            .h_coefficientwise_nonnegative
            .record(nonnegative(&row.hstar), || {
                format!(
                    "sigma={}, H={}",
                    perm_string(sigma),
                    fmt_poly(&row.hstar, "t")
                )
            });
        summary
            .h_sigma_real_rooted
            .record(is_real_rooted(&row.hstar), || {
                format!(
                    "sigma={}, H={}",
                    perm_string(sigma),
                    fmt_poly(&row.hstar, "t")
                )
            });
        for cover in &row.lower_covers {
            let tau_row = &rows[&cover.tau];
            summary
                .h_cover_interlacing
                .record(interlaces(&tau_row.hstar, &row.hstar), || {
                    format!(
                        "{}: H_tau={}, H_sigma={}",
                        context_scan_cover(sigma, cover),
                        fmt_poly(&tau_row.hstar, "t"),
                        fmt_poly(&row.hstar, "t")
                    )
                });
        }
    }
    Ok(summary)
}

fn compute_route_summary(
    n: usize,
    permutations: &[Vec<usize>],
    rows: &BTreeMap<Vec<usize>, ScanRow>,
) -> ExactResult<RouteSummary> {
    let mut g_map: BTreeMap<Vec<usize>, Vec<BigInt>> = BTreeMap::new();
    let mut w_map: BTreeMap<Vec<usize>, Vec<BigInt>> = BTreeMap::new();
    for sigma in permutations {
        let row = &rows[sigma];
        if row.length < 2 {
            continue;
        }
        let mut w = vec![BigInt::zero()];
        for cover in &row.lower_covers {
            w = add_scaled(&w, row_d(&rows[&cover.tau])?, cover.weight);
        }
        let g = sub(row_d(row)?, &shift_u(&w));
        w_map.insert(sigma.clone(), w);
        g_map.insert(sigma.clone(), g);
    }

    let mut summary = RouteSummary {
        h_coefficientwise_nonnegative: Counter::new("H coefficientwise nonnegative"),
        d_sigma_real_rooted: Counter::new("D_sigma real-rooted"),
        d_cover_interlacing: Counter::new("D_tau << D_sigma for Bruhat covers"),
        w_interlaces_g: Counter::new("W_sigma << G_sigma"),
        deleted_w_interlaces_g: Counter::new("W_sigma-mD_tau << G_sigma"),
        l_interlaces_d_all_covers: Counter::new("L_{tau,sigma} << D_tau, all covers"),
        each_upper_has_good_l: Counter::new("each upper has a good L lower cover"),
        max_weight_l_interlaces_d: Counter::new("max-weight L_{tau,sigma} << D_tau"),
        max_weight_l_interlaces_g: Counter::new("max-weight L_{tau,sigma} << G_sigma"),
        max_weight_l_interlaces_w: Counter::new("max-weight L_{tau,sigma} << W_sigma"),
        bark_coefficientwise_nonnegative: Counter::new("bar K coefficientwise nonnegative"),
        bark_interlaces_d_tau: Counter::new("bar K_{tau,sigma} << D_tau"),
        bark_interlaces_d_alpha: Counter::new("bar K_{tau,sigma} << D_alpha"),
        bark_interlaces_g: Counter::new("bar K_{tau,sigma} << G_sigma"),
    };

    for sigma in permutations {
        let row = &rows[sigma];
        summary
            .h_coefficientwise_nonnegative
            .record(nonnegative(&row.hstar), || {
                format!(
                    "sigma={}, H={}",
                    perm_string(sigma),
                    fmt_poly(&row.hstar, "t")
                )
            });
        summary
            .d_sigma_real_rooted
            .record(is_real_rooted(row_d(row)?), || {
                format!(
                    "sigma={}, D={}",
                    perm_string(sigma),
                    fmt_poly(row_d(row).expect("D is present"), "u")
                )
            });

        let covers = &row.lower_covers;
        for cover in covers {
            let tau_row = &rows[&cover.tau];
            summary
                .d_cover_interlacing
                .record(interlaces(row_d(tau_row)?, row_d(row)?), || {
                    format!(
                        "{}: D_tau={}, D_sigma={}",
                        context_scan_cover(sigma, cover),
                        fmt_poly(row_d(tau_row).expect("D is present"), "u"),
                        fmt_poly(row_d(row).expect("D is present"), "u")
                    )
                });
        }

        if row.length < 2 {
            continue;
        }
        let g = &g_map[sigma];
        let w = &w_map[sigma];
        summary.w_interlaces_g.record(interlaces(w, g), || {
            format!(
                "sigma={}, W={}, G={}",
                perm_string(sigma),
                fmt_poly(w, "u"),
                fmt_poly(g, "u")
            )
        });

        let max_weight = covers.iter().map(|cover| cover.weight).max().unwrap_or(0);
        let mut has_good_l = false;
        for cover in covers {
            let d_tau = row_d(&rows[&cover.tau])?;
            let deleted_w = sub(w, &scale(d_tau, cover.weight));
            summary
                .deleted_w_interlaces_g
                .record(interlaces(&deleted_w, g), || {
                    format!(
                        "{}: W-mD_tau={}, G={}",
                        context_scan_cover(sigma, cover),
                        fmt_poly(&deleted_w, "u"),
                        fmt_poly(g, "u")
                    )
                });

            let l = div_by_u(&sub(g, d_tau))?;
            let l_good = interlaces(&l, d_tau);
            has_good_l |= l_good;
            summary.l_interlaces_d_all_covers.record(l_good, || {
                format!(
                    "{}: L={}, D_tau={}, G={}",
                    context_scan_cover(sigma, cover),
                    fmt_poly(&l, "u"),
                    fmt_poly(d_tau, "u"),
                    fmt_poly(g, "u")
                )
            });
            if cover.weight == max_weight {
                summary.max_weight_l_interlaces_d.record(l_good, || {
                    format!(
                        "{}: L={}, D_tau={}",
                        context_scan_cover(sigma, cover),
                        fmt_poly(&l, "u"),
                        fmt_poly(d_tau, "u")
                    )
                });
                summary
                    .max_weight_l_interlaces_g
                    .record(interlaces(&l, g), || {
                        format!(
                            "{}: L={}, G={}",
                            context_scan_cover(sigma, cover),
                            fmt_poly(&l, "u"),
                            fmt_poly(g, "u")
                        )
                    });
                summary
                    .max_weight_l_interlaces_w
                    .record(interlaces(&l, w), || {
                        format!(
                            "{}: L={}, W={}",
                            context_scan_cover(sigma, cover),
                            fmt_poly(&l, "u"),
                            fmt_poly(w, "u")
                        )
                    });
            }

            if rows[&cover.tau].length == 0 {
                continue;
            }
            let bark_num = sub(row_d(row)?, &one_plus_mu_times(d_tau, cover.weight));
            let bark = div_by_u(&bark_num)?;
            if is_zero_poly(&bark) {
                continue;
            }

            summary
                .bark_coefficientwise_nonnegative
                .record(nonnegative(&bark), || {
                    format!(
                        "{}: barK={}",
                        context_scan_cover(sigma, cover),
                        fmt_poly(&bark, "u")
                    )
                });
            summary
                .bark_interlaces_d_tau
                .record(interlaces(&bark, d_tau), || {
                    format!(
                        "{}: barK={}, D_tau={}",
                        context_scan_cover(sigma, cover),
                        fmt_poly(&bark, "u"),
                        fmt_poly(d_tau, "u")
                    )
                });
            summary.bark_interlaces_g.record(interlaces(&bark, g), || {
                format!(
                    "{}: barK={}, G={}",
                    context_scan_cover(sigma, cover),
                    fmt_poly(&bark, "u"),
                    fmt_poly(g, "u")
                )
            });
            for alpha in covers {
                let d_alpha = row_d(&rows[&alpha.tau])?;
                summary
                    .bark_interlaces_d_alpha
                    .record(interlaces(&bark, d_alpha), || {
                        format!(
                            "tau={}, alpha={}, sigma={}, barK={}, D_alpha={}",
                            perm_string(&cover.tau),
                            perm_string(&alpha.tau),
                            perm_string(sigma),
                            fmt_poly(&bark, "u"),
                            fmt_poly(d_alpha, "u")
                        )
                    });
            }
        }
        summary.each_upper_has_good_l.record(has_good_l, || {
            format!(
                "sigma={} has no lower cover with L << D_tau in S_{}",
                perm_string(sigma),
                n
            )
        });
    }
    Ok(summary)
}

fn compute_block_summary(scan: &RankScan) -> BlockSummary {
    let blocks = lambda_blocks(&scan.spec.lambda);
    let mut value_to_block = vec![0usize; scan.n + 1];
    let mut position_to_block = vec![0usize; scan.n];
    for (block_index, block) in blocks.iter().enumerate() {
        value_to_block[block.start + 1..=block.end].fill(block_index);
        position_to_block[block.start..block.end].fill(block_index);
    }

    let mut actual_matrix_counts: BTreeMap<Vec<Vec<usize>>, usize> = BTreeMap::new();
    for sigma in all_permutations(scan.n) {
        let matrix = block_matrix(&sigma, &value_to_block, &position_to_block, blocks.len());
        *actual_matrix_counts.entry(matrix).or_insert(0) += 1;
    }

    let mut matrix_rows: BTreeMap<Vec<Vec<usize>>, Vec<&ScanRow>> = BTreeMap::new();
    for row in &scan.rows {
        let matrix = block_matrix(
            &row.sigma,
            &value_to_block,
            &position_to_block,
            blocks.len(),
        );
        matrix_rows.entry(matrix).or_default().push(row);
    }

    let mut hstar_classes = BTreeSet::new();
    let mut matrix_failures = 0;
    let mut matrix_classes = Vec::new();
    for (matrix, rows) in matrix_rows {
        let mut variants_by_row: BTreeMap<(usize, Vec<BigInt>), BlockMatrixVariant> =
            BTreeMap::new();
        for row in &rows {
            hstar_classes.insert(trim(row.hstar.clone()));
            let key = (row.dimension, row.hstar.clone());
            let entry = variants_by_row
                .entry(key)
                .or_insert_with(|| BlockMatrixVariant {
                    row_count: 0,
                    example_sigma: row.sigma.clone(),
                    dimension: row.dimension,
                    hstar: row.hstar.clone(),
                });
            entry.row_count += 1;
        }
        let variants = variants_by_row.into_values().collect::<Vec<_>>();
        if variants.len() > 1 {
            matrix_failures += 1;
        }
        let row_count = actual_matrix_counts
            .get(&matrix)
            .copied()
            .unwrap_or(rows.len());
        matrix_classes.push(BlockMatrixClass {
            matrix,
            row_count,
            example_sigma: rows[0].sigma.clone(),
            variants,
        });
    }

    BlockSummary {
        representatives_only: scan.block_representatives,
        block_values: blocks.iter().map(|block| block.value).collect(),
        block_sizes: blocks.iter().map(|block| block.end - block.start).collect(),
        matrix_classes,
        hstar_classes: hstar_classes.len(),
        matrix_failures: (!scan.block_representatives).then_some(matrix_failures),
    }
}

fn block_matrix(
    sigma: &[usize],
    value_to_block: &[usize],
    position_to_block: &[usize],
    block_count: usize,
) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0usize; block_count]; block_count];
    for (position, &value) in sigma.iter().enumerate() {
        let value_block = value_to_block[value];
        let position_block = position_to_block[position];
        matrix[value_block][position_block] += 1;
    }
    matrix
}

fn row_d(row: &ScanRow) -> ExactResult<&[BigInt]> {
    row.d.as_deref().ok_or_else(|| {
        format!(
            "D polynomial is only available for staircase rows, missing at sigma={}",
            perm_string(&row.sigma)
        )
    })
}

impl KeyEvalPlan {
    fn new(n: usize, alpha: &[u32]) -> Self {
        let word = sorting_word(alpha);
        let mut sorted_lambda = alpha.to_vec();
        sorted_lambda.sort_unstable_by(|a, b| b.cmp(a));

        if word.is_empty() {
            return Self {
                sorted_lambda,
                initial_live: Vec::new(),
                steps: Vec::new(),
                max_live_len: 0,
            };
        }

        let word_len = word.len();
        let mut live_at: Vec<Vec<usize>> = vec![vec![]; word_len];
        let mut needed = vec![false; n];
        for t in (0..word_len).rev() {
            needed[word[t]] = true;
            needed[word[t] + 1] = true;
            live_at[t] = (0..n).filter(|&value| needed[value]).collect();
        }

        let mut steps = Vec::with_capacity(word_len);
        let mut max_live_len = 0usize;
        for t in 0..word_len {
            let op = word[t];
            let live_now = &live_at[t];
            max_live_len = max_live_len.max(live_now.len());
            let pos_i = live_now
                .iter()
                .position(|&value| value == op)
                .expect("operator index is live");
            let pos_ip1 = live_now
                .iter()
                .position(|&value| value == op + 1)
                .expect("operator index is live");
            let live_next = if t + 1 < word_len {
                live_at[t + 1].clone()
            } else {
                Vec::new()
            };
            max_live_len = max_live_len.max(live_next.len());
            let projection = live_next
                .iter()
                .enumerate()
                .map(|(next_pos, &value)| {
                    (
                        next_pos,
                        live_now
                            .iter()
                            .position(|&current| current == value)
                            .expect("live projection"),
                    )
                })
                .collect();
            steps.push(KeyEvalStep {
                op,
                pos_i,
                pos_ip1,
                live_next,
                projection,
            });
        }

        Self {
            sorted_lambda,
            initial_live: live_at[0].clone(),
            steps,
            max_live_len,
        }
    }
}

fn key_eval_ones_with_plan(plan: &KeyEvalPlan, sorted_lambda: &[u32]) -> ExactResult<BigInt> {
    if let Some(value) = key_eval_ones_i64_with_plan(plan, sorted_lambda)? {
        return Ok(BigInt::from(value));
    }
    key_eval_ones_bigint_with_plan(plan, sorted_lambda)
}

fn key_eval_ones_i64_with_plan(
    plan: &KeyEvalPlan,
    sorted_lambda: &[u32],
) -> ExactResult<Option<i64>> {
    if plan.steps.is_empty() {
        return Ok(Some(1));
    }
    if let Some(bits) = packed_exponent_bits(sorted_lambda, plan.max_live_len) {
        key_eval_ones_i64_packed(plan, sorted_lambda, bits)
    } else {
        key_eval_ones_i64_vec_with_plan(plan, sorted_lambda)
    }
}

fn key_eval_ones_i64_packed(
    plan: &KeyEvalPlan,
    sorted_lambda: &[u32],
    bits: usize,
) -> ExactResult<Option<i64>> {
    let init_key = pack_live_key(&plan.initial_live, sorted_lambda, bits);
    let mut state: HashMap<u64, i64> = HashMap::new();
    state.insert(init_key, 1);

    for step in &plan.steps {
        let next_len = step.live_next.len();
        let mut new_state: HashMap<u64, i64> =
            HashMap::with_capacity(state.len().saturating_mul(4).max(1));

        for (&key, &count) in &state {
            if count == 0 {
                continue;
            }
            let ai = packed_exp(key, bits, step.pos_i) as i64;
            let aip1 = packed_exp(key, bits, step.pos_ip1) as i64;
            let p = ai + 1;
            let q = aip1;
            if p == q {
                continue;
            }

            let (negative, hi, lo, spread) = if p > q {
                (false, ai, aip1, (p - q - 1) as u32)
            } else {
                (true, aip1 - 1, ai + 1, (q - p - 1) as u32)
            };

            if next_len == 0 {
                let multiplicity = i64::from(spread) + 1;
                let signed_multiplicity = if negative {
                    -multiplicity
                } else {
                    multiplicity
                };
                let delta = match count.checked_mul(signed_multiplicity) {
                    Some(value) => value,
                    None => return Ok(None),
                };
                let entry = new_state.entry(0).or_insert(0);
                *entry = match entry.checked_add(delta) {
                    Some(value) => value,
                    None => return Ok(None),
                };
                continue;
            }

            for offset in 0..=(spread as i64) {
                let new_ai = (hi - offset) as u32;
                let new_aip1 = (lo + offset) as u32;

                let mut new_key = 0u64;
                for &(next_pos, old_pos) in &step.projection {
                    let value = step.live_next[next_pos];
                    let exp = if value == step.op {
                        new_ai
                    } else if value == step.op + 1 {
                        new_aip1
                    } else {
                        packed_exp(key, bits, old_pos)
                    };
                    new_key |= u64::from(exp) << (bits * next_pos);
                }
                let delta = if negative {
                    match count.checked_neg() {
                        Some(value) => value,
                        None => return Ok(None),
                    }
                } else {
                    count
                };
                let entry = new_state.entry(new_key).or_insert(0);
                *entry = match entry.checked_add(delta) {
                    Some(value) => value,
                    None => return Ok(None),
                };
            }
        }

        new_state.retain(|_, value| *value != 0);
        state = new_state;
    }

    let total = match state
        .values()
        .try_fold(0i64, |acc, value| acc.checked_add(*value))
    {
        Some(total) => total,
        None => return Ok(None),
    };
    if total < 0 {
        return Err(format!(
            "key evaluation was negative for sorted lambda {sorted_lambda:?}: {total}"
        ));
    }
    Ok(Some(total))
}

fn key_eval_ones_i64_vec_with_plan(
    plan: &KeyEvalPlan,
    sorted_lambda: &[u32],
) -> ExactResult<Option<i64>> {
    let mut state: HashMap<Vec<u32>, i64> = HashMap::new();
    let init_key = plan
        .initial_live
        .iter()
        .map(|&value| sorted_lambda[value])
        .collect::<Vec<_>>();
    state.insert(init_key, 1);

    for step in &plan.steps {
        let next_len = step.live_next.len();
        let mut new_state: HashMap<Vec<u32>, i64> =
            HashMap::with_capacity(state.len().saturating_mul(4).max(1));

        for (exps, &count) in &state {
            if count == 0 {
                continue;
            }
            let ai = exps[step.pos_i] as i64;
            let aip1 = exps[step.pos_ip1] as i64;
            let p = ai + 1;
            let q = aip1;
            if p == q {
                continue;
            }

            let (negative, hi, lo, spread) = if p > q {
                (false, ai, aip1, (p - q - 1) as u32)
            } else {
                (true, aip1 - 1, ai + 1, (q - p - 1) as u32)
            };

            if next_len == 0 {
                let multiplicity = i64::from(spread) + 1;
                let signed_multiplicity = if negative {
                    -multiplicity
                } else {
                    multiplicity
                };
                let delta = match count.checked_mul(signed_multiplicity) {
                    Some(value) => value,
                    None => return Ok(None),
                };
                let entry = new_state.entry(Vec::new()).or_insert(0);
                *entry = match entry.checked_add(delta) {
                    Some(value) => value,
                    None => return Ok(None),
                };
                continue;
            }

            for offset in 0..=(spread as i64) {
                let new_ai = (hi - offset) as u32;
                let new_aip1 = (lo + offset) as u32;

                let mut new_key = vec![0u32; next_len];
                for &(next_pos, old_pos) in &step.projection {
                    let value = step.live_next[next_pos];
                    new_key[next_pos] = if value == step.op {
                        new_ai
                    } else if value == step.op + 1 {
                        new_aip1
                    } else {
                        exps[old_pos]
                    };
                }
                let delta = if negative {
                    match count.checked_neg() {
                        Some(value) => value,
                        None => return Ok(None),
                    }
                } else {
                    count
                };
                let entry = new_state.entry(new_key).or_insert(0);
                *entry = match entry.checked_add(delta) {
                    Some(value) => value,
                    None => return Ok(None),
                };
            }
        }

        new_state.retain(|_, value| *value != 0);
        state = new_state;
    }

    let total = match state
        .values()
        .try_fold(0i64, |acc, value| acc.checked_add(*value))
    {
        Some(total) => total,
        None => return Ok(None),
    };
    if total < 0 {
        return Err(format!(
            "key evaluation was negative for sorted lambda {sorted_lambda:?}: {total}"
        ));
    }
    Ok(Some(total))
}

fn key_eval_ones_bigint_with_plan(
    plan: &KeyEvalPlan,
    sorted_lambda: &[u32],
) -> ExactResult<BigInt> {
    if plan.steps.is_empty() {
        return Ok(BigInt::one());
    }
    if let Some(bits) = packed_exponent_bits(sorted_lambda, plan.max_live_len) {
        return key_eval_ones_bigint_packed(plan, sorted_lambda, bits);
    }
    key_eval_ones_bigint_vec_with_plan(plan, sorted_lambda)
}

fn key_eval_ones_bigint_packed(
    plan: &KeyEvalPlan,
    sorted_lambda: &[u32],
    bits: usize,
) -> ExactResult<BigInt> {
    let mut state: HashMap<u64, BigInt> = HashMap::new();
    let init_key = pack_live_key(&plan.initial_live, sorted_lambda, bits);
    state.insert(init_key, BigInt::one());

    for step in &plan.steps {
        let next_len = step.live_next.len();
        let mut new_state: HashMap<u64, BigInt> =
            HashMap::with_capacity(state.len().saturating_mul(4).max(1));

        for (key, count) in &state {
            if count.is_zero() {
                continue;
            }
            let ai = packed_exp(*key, bits, step.pos_i) as i64;
            let aip1 = packed_exp(*key, bits, step.pos_ip1) as i64;
            let p = ai + 1;
            let q = aip1;
            if p == q {
                continue;
            }

            let (negative, hi, lo, spread) = if p > q {
                (false, ai, aip1, (p - q - 1) as u32)
            } else {
                (true, aip1 - 1, ai + 1, (q - p - 1) as u32)
            };

            if next_len == 0 {
                let multiplicity = BigInt::from(u64::from(spread) + 1);
                let delta = count * multiplicity;
                let entry = new_state.entry(0).or_insert_with(BigInt::zero);
                if negative {
                    *entry -= delta;
                } else {
                    *entry += delta;
                }
                continue;
            }

            for offset in 0..=(spread as i64) {
                let new_ai = (hi - offset) as u32;
                let new_aip1 = (lo + offset) as u32;

                let mut new_key = 0u64;
                for &(next_pos, old_pos) in &step.projection {
                    let value = step.live_next[next_pos];
                    let exp = if value == step.op {
                        new_ai
                    } else if value == step.op + 1 {
                        new_aip1
                    } else {
                        packed_exp(*key, bits, old_pos)
                    };
                    new_key |= u64::from(exp) << (bits * next_pos);
                }
                let entry = new_state.entry(new_key).or_insert_with(BigInt::zero);
                if negative {
                    *entry -= count;
                } else {
                    *entry += count;
                }
            }
        }

        new_state.retain(|_, value| !value.is_zero());
        state = new_state;
    }

    let total = state
        .values()
        .fold(BigInt::zero(), |acc, value| acc + value);
    if total.is_negative() {
        return Err(format!(
            "key evaluation was negative for sorted lambda {sorted_lambda:?}: {total}"
        ));
    }
    Ok(total)
}

fn key_eval_ones_bigint_vec_with_plan(
    plan: &KeyEvalPlan,
    sorted_lambda: &[u32],
) -> ExactResult<BigInt> {
    let mut state: HashMap<Vec<u32>, BigInt> = HashMap::new();
    let init_key = plan
        .initial_live
        .iter()
        .map(|&value| sorted_lambda[value])
        .collect::<Vec<_>>();
    state.insert(init_key, BigInt::one());

    for step in &plan.steps {
        let next_len = step.live_next.len();
        let mut new_state: HashMap<Vec<u32>, BigInt> =
            HashMap::with_capacity(state.len().saturating_mul(4).max(1));

        for (exps, count) in &state {
            if count.is_zero() {
                continue;
            }
            let ai = exps[step.pos_i] as i64;
            let aip1 = exps[step.pos_ip1] as i64;
            let p = ai + 1;
            let q = aip1;
            if p == q {
                continue;
            }

            let (negative, hi, lo, spread) = if p > q {
                (false, ai, aip1, (p - q - 1) as u32)
            } else {
                (true, aip1 - 1, ai + 1, (q - p - 1) as u32)
            };

            if next_len == 0 {
                let multiplicity = BigInt::from(u64::from(spread) + 1);
                let delta = count * multiplicity;
                let entry = new_state.entry(Vec::new()).or_insert_with(BigInt::zero);
                if negative {
                    *entry -= delta;
                } else {
                    *entry += delta;
                }
                continue;
            }

            for offset in 0..=(spread as i64) {
                let new_ai = (hi - offset) as u32;
                let new_aip1 = (lo + offset) as u32;

                let mut new_key = vec![0u32; next_len];
                for &(next_pos, old_pos) in &step.projection {
                    let value = step.live_next[next_pos];
                    new_key[next_pos] = if value == step.op {
                        new_ai
                    } else if value == step.op + 1 {
                        new_aip1
                    } else {
                        exps[old_pos]
                    };
                }
                let entry = new_state.entry(new_key).or_insert_with(BigInt::zero);
                if negative {
                    *entry -= count;
                } else {
                    *entry += count;
                }
            }
        }

        new_state.retain(|_, value| !value.is_zero());
        state = new_state;
    }

    let total = state
        .values()
        .fold(BigInt::zero(), |acc, value| acc + value);
    if total.is_negative() {
        return Err(format!(
            "key evaluation was negative for sorted lambda {sorted_lambda:?}: {total}"
        ));
    }
    Ok(total)
}

fn packed_exponent_bits(sorted_lambda: &[u32], max_live_len: usize) -> Option<usize> {
    if max_live_len == 0 {
        return Some(1);
    }
    let max_exp = sorted_lambda.iter().copied().max().unwrap_or(0);
    let bits = (u32::BITS - max_exp.leading_zeros()).max(1) as usize;
    bits.checked_mul(max_live_len)
        .filter(|&total_bits| total_bits <= u64::BITS as usize)
        .map(|_| bits)
}

fn pack_live_key(live: &[usize], sorted_lambda: &[u32], bits: usize) -> u64 {
    let mut key = 0u64;
    for (pos, &value) in live.iter().enumerate() {
        key |= u64::from(sorted_lambda[value]) << (bits * pos);
    }
    key
}

fn packed_exp(key: u64, bits: usize, pos: usize) -> u32 {
    let mask = if bits == u64::BITS as usize {
        u64::MAX
    } else {
        (1u64 << bits) - 1
    };
    ((key >> (bits * pos)) & mask) as u32
}

fn sorting_word(alpha: &[u32]) -> Vec<usize> {
    let mut perm = alpha.to_vec();
    let n = perm.len();
    let mut word = Vec::new();
    loop {
        let mut swapped = false;
        for i in 0..n.saturating_sub(1) {
            if perm[i] < perm[i + 1] {
                perm.swap(i, i + 1);
                word.push(i);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    word
}

fn bruhat_covers(sigma: &[usize], lambda: &[u32]) -> Vec<Cover> {
    let n = sigma.len();
    let mut covers = Vec::new();
    for a in 0..n {
        for b in a + 1..n {
            if sigma[a] > sigma[b] {
                let lo = sigma[b];
                let hi = sigma[a];
                if !(a + 1..b).any(|c| sigma[c] > lo && sigma[c] < hi) {
                    let mut tau = sigma.to_vec();
                    tau.swap(a, b);
                    covers.push(Cover {
                        tau,
                        i: a + 1,
                        j: b + 1,
                        weight: i64::from(lambda[a] - lambda[b]),
                    });
                }
            }
        }
    }
    covers
}

fn all_permutations(n: usize) -> Vec<Vec<usize>> {
    fn rec(out: &mut Vec<Vec<usize>>, current: &mut Vec<usize>, unused: &mut Vec<usize>) {
        if unused.is_empty() {
            out.push(current.clone());
            return;
        }
        for index in 0..unused.len() {
            let value = unused.remove(index);
            current.push(value);
            rec(out, current, unused);
            current.pop();
            unused.insert(index, value);
        }
    }
    let mut out = Vec::new();
    let mut current = Vec::new();
    let mut unused = (1..=n).collect::<Vec<_>>();
    rec(&mut out, &mut current, &mut unused);
    out
}

fn inv_count(perm: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..perm.len() {
        for j in i + 1..perm.len() {
            if perm[i] > perm[j] {
                count += 1;
            }
        }
    }
    count
}

fn is_identity(perm: &[usize]) -> bool {
    perm.iter()
        .enumerate()
        .all(|(index, value)| *value == index + 1)
}

fn is_longest(perm: &[usize]) -> bool {
    let n = perm.len();
    perm.iter()
        .enumerate()
        .all(|(index, value)| *value == n - index)
}

fn nonnegative(p: &[BigInt]) -> bool {
    p.iter().all(|coeff| !coeff.is_negative())
}

fn is_real_rooted(p: &[BigInt]) -> bool {
    is_zero_poly(p) || is_real_rooted_bigint_coeffs(p)
}

fn interlaces(p: &[BigInt], q: &[BigInt]) -> bool {
    is_zero_poly(p) || check_weak_interlacing_bigint_coeffs(p, q) == Some(true)
}

fn is_zero_poly(p: &[BigInt]) -> bool {
    p.iter().all(Zero::is_zero)
}

fn trim(mut p: Vec<BigInt>) -> Vec<BigInt> {
    while p.len() > 1 && p.last().is_some_and(Zero::is_zero) {
        p.pop();
    }
    if p.is_empty() {
        p.push(BigInt::zero());
    }
    p
}

fn scale(p: &[BigInt], scalar: i64) -> Vec<BigInt> {
    let scalar = BigInt::from(scalar);
    trim(p.iter().map(|coeff| coeff * &scalar).collect())
}

fn add_scaled(a: &[BigInt], b: &[BigInt], scale: i64) -> Vec<BigInt> {
    let scale = BigInt::from(scale);
    let len = a.len().max(b.len());
    let mut out = vec![BigInt::zero(); len];
    for index in 0..len {
        if index < a.len() {
            out[index] += &a[index];
        }
        if index < b.len() {
            out[index] += &b[index] * &scale;
        }
    }
    trim(out)
}

fn sub(a: &[BigInt], b: &[BigInt]) -> Vec<BigInt> {
    add_scaled(a, b, -1)
}

fn shift_u(p: &[BigInt]) -> Vec<BigInt> {
    if is_zero_poly(p) {
        return vec![BigInt::zero()];
    }
    let mut out = Vec::with_capacity(p.len() + 1);
    out.push(BigInt::zero());
    out.extend_from_slice(p);
    trim(out)
}

fn one_plus_mu_times(p: &[BigInt], m: i64) -> Vec<BigInt> {
    add_scaled(p, &shift_u(p), m)
}

fn div_by_u(p: &[BigInt]) -> ExactResult<Vec<BigInt>> {
    if p.first().is_some_and(|coeff| !coeff.is_zero()) {
        return Err(format!(
            "polynomial is not divisible by u: {}",
            fmt_poly(p, "u")
        ));
    }
    Ok(trim(p.get(1..).unwrap_or(&[]).to_vec()))
}

fn div_by_one_plus_u(p: &[BigInt]) -> ExactResult<Vec<BigInt>> {
    if p.len() < 2 {
        return Err("cannot divide a constant polynomial by 1+u".to_string());
    }
    let mut q = vec![BigInt::zero(); p.len() - 1];
    q[0] = p[0].clone();
    for index in 1..q.len() {
        q[index] = &p[index] - &q[index - 1];
    }
    if p[p.len() - 1] != q[q.len() - 1] {
        return Err(format!(
            "polynomial is not divisible by 1+u: {}",
            fmt_poly(p, "u")
        ));
    }
    Ok(trim(q))
}

fn read_checkpoint(
    path: &PathBuf,
    n: usize,
    spec: &ScanSpec,
) -> ExactResult<BTreeMap<Vec<usize>, ScanRow>> {
    let file = File::open(path)
        .map_err(|error| format!("could not open resume file {}: {error}", path.display()))?;
    let mut rows = BTreeMap::new();
    for (line_index, line) in BufReader::new(file).lines().enumerate() {
        let line = line.map_err(|error| {
            format!(
                "could not read line {} from {}: {error}",
                line_index + 1,
                path.display()
            )
        })?;
        if line.trim().is_empty() {
            continue;
        }
        let value = serde_json::from_str::<Value>(&line).map_err(|error| {
            format!(
                "invalid JSON on line {} of {}: {error}",
                line_index + 1,
                path.display()
            )
        })?;
        validate_checkpoint_metadata(&value, spec)?;
        let row = row_from_json(&value)?;
        if row.sigma.len() != n {
            return Err(format!(
                "resume row {} has rank {}, expected {n}",
                perm_string(&row.sigma),
                row.sigma.len()
            ));
        }
        rows.insert(row.sigma.clone(), row);
    }
    Ok(rows)
}

fn validate_checkpoint_metadata(value: &Value, spec: &ScanSpec) -> ExactResult<()> {
    if let Some(lambda_value) = value.get("lambda") {
        let lambda = u32_json_array(lambda_value, "lambda")?;
        if lambda != spec.lambda {
            return Err(format!(
                "resume row lambda [{}] does not match current lambda [{}]",
                join_u32s(&lambda),
                join_u32s(&spec.lambda)
            ));
        }
    } else if value["specialization"].as_str() == Some("staircase") && !spec.is_staircase() {
        return Err(
            "cannot resume a legacy staircase checkpoint for a non-staircase scan".to_string(),
        );
    }
    Ok(())
}

fn row_from_json(value: &Value) -> ExactResult<ScanRow> {
    let index = value["index"]
        .as_u64()
        .ok_or_else(|| "resume row is missing integer index".to_string())? as usize;
    let sigma_text = value["sigma"]
        .as_str()
        .ok_or_else(|| "resume row is missing sigma string".to_string())?;
    let sigma = parse_perm_string(sigma_text)?;
    let length = value["length"]
        .as_u64()
        .ok_or_else(|| "resume row is missing integer length".to_string())?
        as usize;
    let dimension = value
        .get("dimension")
        .and_then(Value::as_u64)
        .map(|value| value as usize)
        .unwrap_or_else(|| {
            value["hstar"]
                .as_array()
                .map_or(1, Vec::len)
                .saturating_sub(1)
        });
    let ehrhart_power = string_array(&value["ehrhart_power"], "ehrhart_power")?;
    let hstar = bigint_array(&value["hstar"], "hstar")?;
    let b = bigint_array(&value["B"], "B")?;
    let d = match value.get("D") {
        Some(Value::Null) | None => None,
        Some(value) => Some(bigint_array(value, "D")?),
    };
    let lower_covers = value["lower_covers"]
        .as_array()
        .ok_or_else(|| "resume row is missing lower_covers array".to_string())?
        .iter()
        .map(|cover| {
            let tau = parse_perm_string(
                cover["tau"]
                    .as_str()
                    .ok_or_else(|| "resume cover is missing tau".to_string())?,
            )?;
            let label = cover["label"]
                .as_array()
                .ok_or_else(|| "resume cover is missing label array".to_string())?;
            if label.len() != 2 {
                return Err("resume cover label must have length two".to_string());
            }
            Ok(ScanCover {
                tau,
                i: label[0]
                    .as_u64()
                    .ok_or_else(|| "resume cover label has non-integer i".to_string())?
                    as usize,
                j: label[1]
                    .as_u64()
                    .ok_or_else(|| "resume cover label has non-integer j".to_string())?
                    as usize,
                weight: cover["weight"]
                    .as_i64()
                    .ok_or_else(|| "resume cover is missing integer weight".to_string())?,
            })
        })
        .collect::<ExactResult<Vec<_>>>()?;
    Ok(ScanRow {
        index,
        sigma,
        length,
        dimension,
        ehrhart_power,
        hstar,
        b,
        d,
        lower_covers,
    })
}

fn string_array(value: &Value, name: &str) -> ExactResult<Vec<String>> {
    value
        .as_array()
        .ok_or_else(|| format!("resume row is missing {name} array"))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(ToString::to_string)
                .ok_or_else(|| format!("resume {name} entry is not a string"))
        })
        .collect()
}

fn bigint_array(value: &Value, name: &str) -> ExactResult<Vec<BigInt>> {
    value
        .as_array()
        .ok_or_else(|| format!("resume row is missing {name} array"))?
        .iter()
        .map(|item| {
            item.as_str()
                .ok_or_else(|| format!("resume {name} entry is not a string"))?
                .parse::<BigInt>()
                .map_err(|_| format!("resume {name} entry is not an integer"))
        })
        .collect()
}

fn u32_json_array(value: &Value, name: &str) -> ExactResult<Vec<u32>> {
    value
        .as_array()
        .ok_or_else(|| format!("resume row is missing {name} array"))?
        .iter()
        .map(|item| {
            let value = item
                .as_u64()
                .ok_or_else(|| format!("resume {name} entry is not an integer"))?;
            u32::try_from(value).map_err(|_| format!("resume {name} entry {value} is too large"))
        })
        .collect()
}

fn usize_json_field(value: &Value, name: &str, context: &str) -> ExactResult<usize> {
    let raw = value[name]
        .as_u64()
        .ok_or_else(|| format!("{context} is missing integer {name}"))?;
    usize::try_from(raw).map_err(|_| format!("{context} integer {name}={raw} is too large"))
}

fn parse_perm_string(text: &str) -> ExactResult<Vec<usize>> {
    text.chars()
        .map(|ch| {
            ch.to_digit(10)
                .map(|value| value as usize)
                .ok_or_else(|| format!("invalid permutation character `{ch}`"))
        })
        .collect()
}

fn render_scan(
    scans: &[RankScan],
    format: OutputFormat,
    include_rows: bool,
    include_block_summary: bool,
) -> ExactResult<String> {
    match format {
        OutputFormat::Text | OutputFormat::Latex => {
            Ok(render_scan_text(scans, include_rows, include_block_summary))
        }
        OutputFormat::Json => serde_json::to_string_pretty(&json!({
            "family": "key",
            "specialization": scans.first().map(|scan| scan.spec.specialization),
            "ranks": scans.iter().map(|scan| {
                rank_json(scan, include_rows, include_block_summary)
            }).collect::<Vec<_>>(),
        }))
        .map_err(|error| format!("could not render scan JSON: {error}")),
    }
}

fn render_scan_text(scans: &[RankScan], include_rows: bool, include_block_summary: bool) -> String {
    let mut output = String::new();
    for scan in scans {
        output.push_str(&format!(
            "key {} S_{}: rows {}/{} complete={} new={} resumed={} elapsed_ms={}\n",
            scan.spec.display(),
            scan.n,
            scan.rows.len(),
            scan.rows_total,
            scan.complete,
            scan.computed_this_run,
            scan.skipped_from_resume,
            scan.elapsed_ms
        ));
        if let Some(sigma) = &scan.selected_sigma {
            output.push_str(&format!("selected sigma: {}\n", perm_string(sigma)));
        }
        if let Some(summary) = &scan.h_summary {
            for counter in summary.counters() {
                output.push_str(&counter.text_line());
                output.push('\n');
            }
        } else if !scan.complete {
            output.push_str("h* summary: unavailable until the rank is complete\n");
        }
        if let Some(summary) = &scan.route_summary {
            for counter in summary.counters() {
                output.push_str(&counter.text_line());
                output.push('\n');
            }
        }
        if include_block_summary {
            let summary = compute_block_summary(scan);
            let failure_text = summary
                .matrix_failures
                .map(|count| count.to_string())
                .unwrap_or_else(|| "not_checked".to_string());
            output.push_str(&format!(
                "block summary: values=[{}] sizes=[{}] matrix_classes={} hstar_classes={} representatives_only={} matrix_failures={}\n",
                join_u32s(&summary.block_values),
                join_usizes(&summary.block_sizes),
                summary.matrix_classes.len(),
                summary.hstar_classes,
                summary.representatives_only,
                failure_text
            ));
            for class in summary.matrix_classes.iter().take(20) {
                output.push_str(&format!(
                    "  M={} rows={} example={} variants={}",
                    fmt_matrix(&class.matrix),
                    class.row_count,
                    perm_string(&class.example_sigma),
                    class.variants.len()
                ));
                if class.variants.len() == 1 {
                    let variant = &class.variants[0];
                    output.push_str(&format!(
                        " dim={} h*=[{}]",
                        variant.dimension,
                        join_bigints(&trim(variant.hstar.clone()))
                    ));
                }
                output.push('\n');
            }
            if summary.matrix_classes.len() > 20 {
                output.push_str(&format!(
                    "  ... {} additional matrix classes omitted from text output\n",
                    summary.matrix_classes.len() - 20
                ));
            }
        }
        if include_rows {
            for row in &scan.rows {
                output.push_str(&format!(
                    "  {} ell={} dim={} h*=[{}] B=[{}]",
                    perm_string(&row.sigma),
                    row.length,
                    row.dimension,
                    join_bigints(&row.hstar),
                    join_bigints(&row.b)
                ));
                if let Some(d) = &row.d {
                    output.push_str(&format!(" D=[{}]", join_bigints(d)));
                }
                output.push('\n');
            }
        }
    }
    output
}

fn rank_json(scan: &RankScan, include_rows: bool, include_block_summary: bool) -> Value {
    let mut value = json!({
        "n": scan.n,
        "specialization": scan.spec.specialization,
        "lambda": scan.spec.lambda,
        "gaps": lambda_gaps(&scan.spec.lambda),
        "complete": scan.complete,
        "rows_available": scan.rows.len(),
        "rows_total": scan.rows_total,
        "computed_this_run": scan.computed_this_run,
        "skipped_from_resume": scan.skipped_from_resume,
        "elapsed_ms": scan.elapsed_ms,
        "block_representatives": scan.block_representatives,
        "selected_sigma": scan.selected_sigma.as_ref().map(|sigma| perm_string(sigma)),
        "h_summary": scan.h_summary.as_ref().map(h_summary_json),
        "route_summary": scan.route_summary.as_ref().map(route_summary_json),
        "rows": include_rows.then(|| {
            scan.rows.iter().map(|row| row_json(row, &scan.spec)).collect::<Vec<_>>()
        }),
    });
    if include_block_summary {
        value["block_summary"] = block_summary_json(&compute_block_summary(scan));
    }
    value
}

fn block_summary_json(summary: &BlockSummary) -> Value {
    json!({
        "block_values": summary.block_values,
        "block_sizes": summary.block_sizes,
        "representatives_only": summary.representatives_only,
        "matrix_class_count": summary.matrix_classes.len(),
        "hstar_class_count": summary.hstar_classes,
        "matrix_failures_checked": summary.matrix_failures.is_some(),
        "matrix_failure_count": summary.matrix_failures,
        "classes": summary.matrix_classes.iter().map(block_matrix_class_json).collect::<Vec<_>>(),
    })
}

fn block_matrix_class_json(class: &BlockMatrixClass) -> Value {
    json!({
        "matrix": class.matrix,
        "row_count": class.row_count,
        "example_sigma": perm_string(&class.example_sigma),
        "variants": class.variants.iter().map(block_matrix_variant_json).collect::<Vec<_>>(),
    })
}

fn block_matrix_variant_json(variant: &BlockMatrixVariant) -> Value {
    json!({
        "row_count": variant.row_count,
        "example_sigma": perm_string(&variant.example_sigma),
        "dimension": variant.dimension,
        "hstar": bigint_strings(&trim(variant.hstar.clone())),
    })
}

fn h_summary_json(summary: &HCheckSummary) -> Value {
    json!({
        "h_coefficientwise_nonnegative": summary.h_coefficientwise_nonnegative.json(),
        "h_sigma_real_rooted": summary.h_sigma_real_rooted.json(),
        "h_cover_interlacing": summary.h_cover_interlacing.json(),
    })
}

impl HCheckSummary {
    fn counters(&self) -> [&Counter; 3] {
        [
            &self.h_coefficientwise_nonnegative,
            &self.h_sigma_real_rooted,
            &self.h_cover_interlacing,
        ]
    }
}

fn route_summary_json(summary: &RouteSummary) -> Value {
    json!({
        "h_coefficientwise_nonnegative": summary.h_coefficientwise_nonnegative.json(),
        "d_sigma_real_rooted": summary.d_sigma_real_rooted.json(),
        "d_cover_interlacing": summary.d_cover_interlacing.json(),
        "w_interlaces_g": summary.w_interlaces_g.json(),
        "deleted_w_interlaces_g": summary.deleted_w_interlaces_g.json(),
        "l_interlaces_d_all_covers": summary.l_interlaces_d_all_covers.json(),
        "each_upper_has_good_l": summary.each_upper_has_good_l.json(),
        "max_weight_l_interlaces_d": summary.max_weight_l_interlaces_d.json(),
        "max_weight_l_interlaces_g": summary.max_weight_l_interlaces_g.json(),
        "max_weight_l_interlaces_w": summary.max_weight_l_interlaces_w.json(),
        "bark_coefficientwise_nonnegative": summary.bark_coefficientwise_nonnegative.json(),
        "bark_interlaces_d_tau": summary.bark_interlaces_d_tau.json(),
        "bark_interlaces_d_alpha": summary.bark_interlaces_d_alpha.json(),
        "bark_interlaces_g": summary.bark_interlaces_g.json(),
    })
}

impl RouteSummary {
    fn counters(&self) -> [&Counter; 14] {
        [
            &self.h_coefficientwise_nonnegative,
            &self.d_sigma_real_rooted,
            &self.d_cover_interlacing,
            &self.w_interlaces_g,
            &self.deleted_w_interlaces_g,
            &self.l_interlaces_d_all_covers,
            &self.each_upper_has_good_l,
            &self.max_weight_l_interlaces_d,
            &self.max_weight_l_interlaces_g,
            &self.max_weight_l_interlaces_w,
            &self.bark_coefficientwise_nonnegative,
            &self.bark_interlaces_d_tau,
            &self.bark_interlaces_d_alpha,
            &self.bark_interlaces_g,
        ]
    }
}

fn row_json(row: &ScanRow, spec: &ScanSpec) -> Value {
    json!({
        "family": "key",
        "specialization": spec.specialization,
        "lambda": spec.lambda,
        "gaps": lambda_gaps(&spec.lambda),
        "n": row.sigma.len(),
        "index": row.index,
        "sigma": perm_string(&row.sigma),
        "length": row.length,
        "dimension": row.dimension,
        "ehrhart_power": row.ehrhart_power,
        "hstar": bigint_strings(&row.hstar),
        "B": bigint_strings(&row.b),
        "D": row.d.as_ref().map(|d| bigint_strings(d)),
        "lower_covers": row.lower_covers.iter().map(|cover| {
            json!({
                "tau": perm_string(&cover.tau),
                "label": [cover.i, cover.j],
                "weight": cover.weight,
            })
        }).collect::<Vec<_>>(),
    })
}

fn bigint_strings(values: &[BigInt]) -> Vec<String> {
    values.iter().map(ToString::to_string).collect()
}

fn join_bigints(values: &[BigInt]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn join_u32s(values: &[u32]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn join_usizes(values: &[usize]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn fmt_matrix(matrix: &[Vec<usize>]) -> String {
    let rows = matrix
        .iter()
        .map(|row| format!("[{}]", join_usizes(row)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{rows}]")
}

fn fmt_poly(coeffs: &[BigInt], var: &str) -> String {
    let mut terms = Vec::new();
    for (degree, coeff) in coeffs.iter().enumerate() {
        if coeff.is_zero() {
            continue;
        }
        let negative = coeff.is_negative();
        let magnitude = if negative { -coeff } else { coeff.clone() };
        let body = match degree {
            0 => magnitude.to_string(),
            1 if magnitude.is_one() => var.to_string(),
            1 => format!("{magnitude}{var}"),
            _ if magnitude.is_one() => format!("{var}^{degree}"),
            _ => format!("{magnitude}{var}^{degree}"),
        };
        if terms.is_empty() {
            terms.push(if negative { format!("-{body}") } else { body });
        } else if negative {
            terms.push(format!(" - {body}"));
        } else {
            terms.push(format!(" + {body}"));
        }
    }
    if terms.is_empty() {
        "0".to_string()
    } else {
        terms.concat()
    }
}

fn context_scan_cover(sigma: &[usize], cover: &ScanCover) -> String {
    format!(
        "{} <dot {}, label ({},{}), m={}",
        perm_string(&cover.tau),
        perm_string(sigma),
        cover.i,
        cover.j,
        cover.weight
    )
}

fn perm_string(perm: &[usize]) -> String {
    perm.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staircase_s3_scan_reproduces_route_counts() {
        let output = run_key_scan(&KeyScanInput {
            n: Some(3),
            max_n: None,
            staircase: true,
            lambda: None,
            gaps: None,
            sigma: None,
            packets: KeyPacketMode::DRoute,
            include_rows: true,
            include_block_summary: false,
            block_representatives: false,
            start_index: 0,
            limit: None,
            checkpoint: None,
            resume: None,
            sample_checkpoint: None,
            format: OutputFormat::Json,
        })
        .expect("S_3 scan");
        let value = serde_json::from_str::<Value>(&output).expect("scan JSON");
        let rank = &value["ranks"][0];
        assert_eq!(rank["complete"], true);
        assert_eq!(rank["rows_total"], 6);
        assert_eq!(rank["route_summary"]["d_cover_interlacing"]["pass"], 8);
        assert_eq!(rank["route_summary"]["bark_interlaces_d_tau"]["pass"], 6);
        let top = rank["rows"]
            .as_array()
            .expect("rows")
            .iter()
            .find(|row| row["sigma"] == "321")
            .expect("top row");
        assert_eq!(top["hstar"], json!(["1", "4", "1", "0"]));
        assert_eq!(top["B"], json!(["1", "7", "12", "6"]));
    }

    #[test]
    fn general_lambda_s3_scan_uses_fast_principal_evaluator() {
        let output = run_key_scan(&KeyScanInput {
            n: Some(3),
            max_n: None,
            staircase: false,
            lambda: Some(vec![3, 1]),
            gaps: None,
            sigma: None,
            packets: KeyPacketMode::HChecks,
            include_rows: true,
            include_block_summary: false,
            block_representatives: false,
            start_index: 0,
            limit: None,
            checkpoint: None,
            resume: None,
            sample_checkpoint: None,
            format: OutputFormat::Json,
        })
        .expect("S_3 lambda scan");
        let value = serde_json::from_str::<Value>(&output).expect("scan JSON");
        let rank = &value["ranks"][0];
        assert_eq!(rank["lambda"], json!([3, 1, 0]));
        assert_eq!(rank["gaps"], json!([2, 1]));
        assert_eq!(rank["h_summary"]["h_sigma_real_rooted"]["pass"], 6);
        assert_eq!(rank["h_summary"]["h_cover_interlacing"]["pass"], 7);
        let row = rank["rows"]
            .as_array()
            .expect("rows")
            .iter()
            .find(|row| row["sigma"] == "312")
            .expect("312 row");
        assert_eq!(row["hstar"], json!(["1", "6", "1"]));
        assert_eq!(row["D"], Value::Null);
    }

    #[test]
    fn gap_scan_builds_partition() {
        let output = run_key_scan(&KeyScanInput {
            n: Some(3),
            max_n: None,
            staircase: false,
            lambda: None,
            gaps: Some(vec![2, 1]),
            sigma: None,
            packets: KeyPacketMode::None,
            include_rows: false,
            include_block_summary: false,
            block_representatives: false,
            start_index: 0,
            limit: None,
            checkpoint: None,
            resume: None,
            sample_checkpoint: None,
            format: OutputFormat::Json,
        })
        .expect("S_3 gap scan");
        let value = serde_json::from_str::<Value>(&output).expect("scan JSON");
        assert_eq!(value["ranks"][0]["lambda"], json!([3, 1, 0]));
    }

    #[test]
    fn sigma_filter_computes_one_fast_scan_row() {
        let output = run_key_scan(&KeyScanInput {
            n: Some(5),
            max_n: None,
            staircase: false,
            lambda: Some(vec![2, 1, 1]),
            gaps: None,
            sigma: Some(vec![4, 2, 5, 1, 3]),
            packets: KeyPacketMode::HChecks,
            include_rows: true,
            include_block_summary: true,
            block_representatives: false,
            start_index: 0,
            limit: None,
            checkpoint: None,
            resume: None,
            sample_checkpoint: None,
            format: OutputFormat::Json,
        })
        .expect("single sigma scan");
        let value = serde_json::from_str::<Value>(&output).expect("scan JSON");
        let rank = &value["ranks"][0];
        assert_eq!(rank["complete"], false);
        assert_eq!(rank["selected_sigma"], "42513");
        assert_eq!(rank["rows_available"], 1);
        assert_eq!(rank["rows_total"], 120);
        assert_eq!(rank["h_summary"], Value::Null);
        assert_eq!(rank["rows"][0]["sigma"], "42513");
        assert_eq!(
            rank["rows"][0]["hstar"],
            json!(["1", "24", "73", "43", "4", "0", "0"])
        );
        assert_eq!(rank["block_summary"]["matrix_class_count"], 1);
    }

    #[test]
    fn scaled_staircase_s3_uses_veronese_dilation() {
        let output = run_key_scan(&KeyScanInput {
            n: Some(3),
            max_n: None,
            staircase: false,
            lambda: None,
            gaps: Some(vec![2, 2]),
            sigma: None,
            packets: KeyPacketMode::HChecks,
            include_rows: true,
            include_block_summary: false,
            block_representatives: false,
            start_index: 0,
            limit: None,
            checkpoint: None,
            resume: None,
            sample_checkpoint: None,
            format: OutputFormat::Json,
        })
        .expect("S_3 scaled staircase scan");
        let value = serde_json::from_str::<Value>(&output).expect("scan JSON");
        let rank = &value["ranks"][0];
        assert_eq!(rank["lambda"], json!([4, 2, 0]));
        assert_eq!(rank["h_summary"]["h_cover_interlacing"]["pass"], 8);
        let top = rank["rows"]
            .as_array()
            .expect("rows")
            .iter()
            .find(|row| row["sigma"] == "321")
            .expect("top row");
        assert_eq!(top["hstar"], json!(["1", "23", "23", "1"]));
        assert_eq!(top["D"], Value::Null);
    }

    #[test]
    fn scaled_rectangle_s3_uses_common_lambda_dilation() {
        let output = run_key_scan(&KeyScanInput {
            n: Some(3),
            max_n: None,
            staircase: false,
            lambda: Some(vec![2, 2]),
            gaps: None,
            sigma: None,
            packets: KeyPacketMode::HChecks,
            include_rows: true,
            include_block_summary: false,
            block_representatives: false,
            start_index: 0,
            limit: None,
            checkpoint: None,
            resume: None,
            sample_checkpoint: None,
            format: OutputFormat::Json,
        })
        .expect("S_3 scaled rectangle scan");
        let value = serde_json::from_str::<Value>(&output).expect("scan JSON");
        let rank = &value["ranks"][0];
        assert_eq!(rank["lambda"], json!([2, 2, 0]));
        assert_eq!(rank["h_summary"]["h_cover_interlacing"]["pass"], 8);
        let top = rank["rows"]
            .as_array()
            .expect("rows")
            .iter()
            .find(|row| row["sigma"] == "321")
            .expect("top row");
        assert_eq!(top["hstar"], json!(["1", "1"]));
        assert_eq!(top["B"], json!(["1", "2"]));
        assert_eq!(top["D"], Value::Null);
    }

    #[test]
    fn block_summary_groups_block_constant_lambda_rows() {
        let output = run_key_scan(&KeyScanInput {
            n: Some(4),
            max_n: None,
            staircase: false,
            lambda: Some(vec![2, 1]),
            gaps: None,
            sigma: None,
            packets: KeyPacketMode::HChecks,
            include_rows: false,
            include_block_summary: true,
            block_representatives: false,
            start_index: 0,
            limit: None,
            checkpoint: None,
            resume: None,
            sample_checkpoint: None,
            format: OutputFormat::Json,
        })
        .expect("S_4 block-constant scan");
        let value = serde_json::from_str::<Value>(&output).expect("scan JSON");
        let summary = &value["ranks"][0]["block_summary"];
        assert_eq!(summary["block_values"], json!([2, 1, 0]));
        assert_eq!(summary["block_sizes"], json!([1, 1, 2]));
        assert_eq!(summary["matrix_class_count"], 7);
        assert_eq!(summary["hstar_class_count"], 4);
        assert_eq!(summary["matrix_failure_count"], 0);
        let top_class = summary["classes"]
            .as_array()
            .expect("classes")
            .iter()
            .find(|class| class["example_sigma"] == "3412")
            .expect("top matrix class");
        assert_eq!(top_class["row_count"], 4);
        assert_eq!(top_class["variants"][0]["hstar"], json!(["1", "9", "6"]));
    }

    #[test]
    fn block_representatives_compute_one_row_per_matrix_class() {
        let output = run_key_scan(&KeyScanInput {
            n: Some(4),
            max_n: None,
            staircase: false,
            lambda: Some(vec![2, 1]),
            gaps: None,
            sigma: None,
            packets: KeyPacketMode::None,
            include_rows: true,
            include_block_summary: true,
            block_representatives: true,
            start_index: 0,
            limit: None,
            checkpoint: None,
            resume: None,
            sample_checkpoint: None,
            format: OutputFormat::Json,
        })
        .expect("S_4 block representatives scan");
        let value = serde_json::from_str::<Value>(&output).expect("scan JSON");
        let rank = &value["ranks"][0];
        assert_eq!(rank["complete"], false);
        assert_eq!(rank["block_representatives"], true);
        assert_eq!(rank["rows_available"], 7);
        assert_eq!(rank["rows_total"], 24);
        let summary = &rank["block_summary"];
        assert_eq!(summary["representatives_only"], true);
        assert_eq!(summary["matrix_class_count"], 7);
        assert_eq!(summary["matrix_failures_checked"], false);
        assert_eq!(summary["matrix_failure_count"], Value::Null);
        let top_class = summary["classes"]
            .as_array()
            .expect("classes")
            .iter()
            .find(|class| class["example_sigma"] == "3412")
            .expect("top matrix class");
        assert_eq!(top_class["row_count"], 4);
        assert_eq!(top_class["variants"][0]["row_count"], 1);
    }

    #[test]
    fn checkpoint_rows_can_be_resumed() {
        let spec = ScanSpec {
            lambda: staircase_lambda(3),
            specialization: "staircase",
        };
        let row = compute_key_scan_row(0, &[1, 2, 3], &spec, None).expect("identity row");
        let parsed = row_from_json(&row_json(&row, &spec)).expect("parse row");
        assert_eq!(parsed.sigma, vec![1, 2, 3]);
        assert_eq!(parsed.hstar, vec![BigInt::one()]);
        assert_eq!(parsed.dimension, 0);
    }

    #[test]
    fn sample_checkpoint_values_can_be_resumed() {
        let path = std::env::temp_dir().join(format!(
            "ehrcalc-key-sample-checkpoint-{}.jsonl",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);

        let first = principal_values(3, &[3, 2, 1], &[2, 1, 0], 3, Some(&path)).expect("first run");
        let text = std::fs::read_to_string(&path).expect("read sample checkpoint");
        assert_eq!(text.lines().count(), 4);

        let second =
            principal_values(3, &[3, 2, 1], &[2, 1, 0], 3, Some(&path)).expect("resume run");
        assert_eq!(second, first);
        let resumed_text = std::fs::read_to_string(&path).expect("read resumed checkpoint");
        assert_eq!(resumed_text.lines().count(), 4);
        let first_line = serde_json::from_str::<Value>(
            resumed_text.lines().next().expect("first checkpoint line"),
        )
        .expect("sample checkpoint JSON");
        assert_eq!(first_line["kind"], "key_scan_sample");
        assert_eq!(first_line["sigma"], "321");
        assert_eq!(first_line["lambda"], json!([2, 1, 0]));

        let _ = std::fs::remove_file(&path);
    }
}

//! Plan and reconstruct exact LR coefficients through independent Kostka jobs.

use ehrcalc_kostka_engine::lr::lr_kostka_plan;
use ehrcalc_kostka_engine::Partition;
use num_bigint::BigUint;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn csv(partition: &Partition) -> String {
    if partition.is_empty() {
        "-".to_string()
    } else {
        partition
            .parts()
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn parse_counts(path: &Path) -> Result<HashMap<String, BigUint>, String> {
    let input = fs::read_to_string(path)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let mut counts = HashMap::new();
    for (line_number, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let (id, raw_count) = line.split_once('\t').ok_or_else(|| {
            format!(
                "{}:{} must contain ID<TAB>COUNT",
                path.display(),
                line_number + 1
            )
        })?;
        let count = BigUint::parse_bytes(raw_count.as_bytes(), 10).ok_or_else(|| {
            format!(
                "{}:{} has an invalid count",
                path.display(),
                line_number + 1
            )
        })?;
        if counts.insert(id.to_string(), count).is_some() {
            return Err(format!("duplicate count for job {id}"));
        }
    }
    Ok(counts)
}

fn main() -> Result<(), String> {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.len() < 5 || arguments.len() > 6 {
        return Err(format!(
            "usage: {} plan|reconstruct LAMBDA MU NU [COUNTS_FILE]",
            arguments
                .first()
                .map(String::as_str)
                .unwrap_or("lr_kostka_plan")
        ));
    }
    let lambda = Partition::parse(&arguments[2])?;
    let mu = Partition::parse(&arguments[3])?;
    let nu = Partition::parse(&arguments[4])?;
    let plan = lr_kostka_plan(&lambda, &mu, &nu)?;
    match arguments[1].as_str() {
        "plan" => {
            if arguments.len() != 5 {
                return Err("plan does not accept a counts file".to_string());
            }
            if let Some(result) = plan.trivial_result() {
                println!("result\t{result}");
                return Ok(());
            }
            for job in &plan.jobs {
                println!(
                    "job\t{}\t{}\t{}\t{}\t{}",
                    job.id,
                    job.upper_bound,
                    csv(&job.outer),
                    csv(&job.inner),
                    job.weight
                        .iter()
                        .map(u32::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                );
            }
        }
        "reconstruct" => {
            if arguments.len() != 6 {
                return Err("reconstruct requires a counts file".to_string());
            }
            let counts = parse_counts(Path::new(&arguments[5]))?;
            println!("{}", plan.reconstruct(&counts)?);
        }
        mode => return Err(format!("unknown mode {mode:?}; use plan or reconstruct")),
    }
    Ok(())
}

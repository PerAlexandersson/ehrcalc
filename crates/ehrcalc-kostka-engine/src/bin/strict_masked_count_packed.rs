//! Exact packed-key CPU counter for relative interiors of masked tableau faces.

use ehrcalc_kostka_engine::packed_modular::{
    try_strict_masked_flagged_skew_kostka_packed_exact_parallel_stats,
    try_strict_masked_flagged_skew_kostka_packed_exact_stats,
    try_strict_masked_flagged_skew_kostka_packed_u192_parallel_stats,
};
use ehrcalc_kostka_engine::Partition;
use serde_json::json;
use std::env;

fn parse_list(raw: &str, name: &str) -> Result<Vec<u32>, String> {
    if raw == "-" || raw.is_empty() {
        return Ok(Vec::new());
    }
    raw.split(',')
        .map(|part| {
            part.parse::<u32>()
                .map_err(|error| format!("invalid {name} entry {part:?}: {error}"))
        })
        .collect()
}

fn scale(values: &[u32], dilation: u32, name: &str) -> Result<Vec<u32>, String> {
    values
        .iter()
        .map(|&value| {
            value
                .checked_mul(dilation)
                .ok_or_else(|| format!("{name} dilation overflow"))
        })
        .collect()
}

fn main() -> Result<(), String> {
    let arguments = env::args().collect::<Vec<_>>();
    if !(9..=10).contains(&arguments.len()) {
        return Err(format!(
            "usage: {} DILATION OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS FORBIDDEN_MASKS MAX_STATES [THREADS]",
            arguments
                .first()
                .map(String::as_str)
                .unwrap_or("strict_masked_count_packed")
        ));
    }
    let dilation = arguments[1]
        .parse::<u32>()
        .map_err(|error| format!("invalid dilation: {error}"))?;
    let lambda = Partition::from_sorted(scale(
        &parse_list(&arguments[2], "outer")?,
        dilation,
        "outer",
    )?);
    let mu = Partition::from_sorted(scale(
        &parse_list(&arguments[3], "inner")?,
        dilation,
        "inner",
    )?);
    let weight = scale(&parse_list(&arguments[4], "weight")?, dilation, "weight")?;
    let upper = parse_list(&arguments[5], "upper flag")?;
    let lower = parse_list(&arguments[6], "lower flag")?;
    let forbidden = parse_list(&arguments[7], "forbidden-row mask")?;
    let max_states = arguments[8]
        .parse::<usize>()
        .map_err(|error| format!("invalid max states: {error}"))?;
    let threads = arguments
        .get(9)
        .map(|value| {
            value
                .parse::<usize>()
                .map_err(|error| format!("invalid thread count: {error}"))
        })
        .transpose()?
        .unwrap_or(1);

    let upper = (!upper.is_empty()).then_some(upper.as_slice());
    let lower = (!lower.is_empty()).then_some(lower.as_slice());
    let forbidden = (!forbidden.is_empty()).then_some(forbidden.as_slice());
    let u192_result = try_strict_masked_flagged_skew_kostka_packed_u192_parallel_stats(
        &lambda,
        &mu,
        &weight,
        upper,
        lower,
        forbidden,
        Some(max_states),
        threads,
    );
    let (result, counter) = match u192_result {
        Ok(result) => (result, "u192"),
        Err(error) if error == "packed exact count exceeds 192 bits" => {
            let result = if threads == 1 {
                try_strict_masked_flagged_skew_kostka_packed_exact_stats(
                    &lambda,
                    &mu,
                    &weight,
                    upper,
                    lower,
                    forbidden,
                    Some(max_states),
                )
            } else {
                try_strict_masked_flagged_skew_kostka_packed_exact_parallel_stats(
                    &lambda,
                    &mu,
                    &weight,
                    upper,
                    lower,
                    forbidden,
                    Some(max_states),
                    threads,
                )
            }?;
            (result, "biguint_fallback")
        }
        Err(error) => return Err(error),
    };
    let Some((dimension, stats)) = result else {
        println!(
            "{}",
            json!({
                "dilation": dilation,
                "empty": true,
                "strict": "0",
            })
        );
        return Ok(());
    };
    println!(
        "{}",
        json!({
            "dilation": dilation,
            "dimension": dimension,
            "counter": counter,
            "threads": threads,
            "strict": stats.value.to_string(),
            "peak_states": stats.peak_states,
            "level_states": stats.level_states,
            "level_transitions": stats.level_transitions,
        })
    );
    Ok(())
}

//! Exact CPU companion for the packed modular masked-face backend.

use ehrcalc_kostka_engine::kostka_dp::try_strict_masked_flagged_skew_kostka;
use ehrcalc_kostka_engine::Partition;
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
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 9 {
        return Err(format!(
            "usage: {} DILATION OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS FORBIDDEN_MASKS MAX_STATES",
            arguments
                .first()
                .map(String::as_str)
                .unwrap_or("strict_masked_count")
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
    let weight = scale(
        &parse_list(&arguments[4], "weight")?,
        dilation,
        "weight",
    )?;
    let upper = parse_list(&arguments[5], "upper flag")?;
    let lower = parse_list(&arguments[6], "lower flag")?;
    let forbidden = parse_list(&arguments[7], "forbidden-row mask")?;
    let max_states = arguments[8]
        .parse::<usize>()
        .map_err(|error| format!("invalid max states: {error}"))?;
    println!(
        "{}",
        try_strict_masked_flagged_skew_kostka(
            &lambda,
            &mu,
            &weight,
            (!upper.is_empty()).then_some(upper.as_slice()),
            (!lower.is_empty()).then_some(lower.as_slice()),
            (!forbidden.is_empty()).then_some(forbidden.as_slice()),
            Some(max_states),
        )?
    );
    Ok(())
}

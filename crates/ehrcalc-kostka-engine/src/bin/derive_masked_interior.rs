use ehrcalc_kostka_engine::gt_dim::gt_polytope_bounds_masked;
use ehrcalc_kostka_engine::kostka_dp::masked_relative_interior_masks;
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

fn main() -> Result<(), String> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 7 {
        return Err(format!(
            "usage: {} OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS FORBIDDEN_MASKS",
            arguments
                .first()
                .map(String::as_str)
                .unwrap_or("derive_masked_interior")
        ));
    }

    let lambda = Partition::from_sorted(parse_list(&arguments[1], "outer")?);
    let mu = Partition::from_sorted(parse_list(&arguments[2], "inner")?);
    let weight = parse_list(&arguments[3], "weight")?;
    let upper = parse_list(&arguments[4], "upper flag")?;
    let lower = parse_list(&arguments[5], "lower flag")?;
    let forbidden = parse_list(&arguments[6], "forbidden-row mask")?;
    let upper_flags = (!upper.is_empty()).then_some(upper.as_slice());
    let lower_flags = (!lower.is_empty()).then_some(lower.as_slice());
    let forbidden_masks = (!forbidden.is_empty()).then_some(forbidden.as_slice());

    let result = masked_relative_interior_masks(
        &lambda,
        &mu,
        &weight,
        upper_flags,
        lower_flags,
        forbidden_masks,
    )?;
    let output = match result {
        None => json!({ "empty": true }),
        Some(masks) => {
            let (_, lower_bounds, upper_bounds) = gt_polytope_bounds_masked(
                lambda.parts(),
                mu.parts(),
                &weight,
                upper_flags,
                lower_flags,
                forbidden_masks,
            )
            .ok_or_else(|| "interior masks and affine bounds disagree".to_string())?;
            json!({
                "empty": false,
                "dimension": masks.dimension,
                "lower_bounds": lower_bounds,
                "upper_bounds": upper_bounds,
                "strict_lower_masks": masks.strict_lower_masks,
                "strict_diagonal_masks": masks.strict_diagonal_masks,
            })
        }
    };
    println!("{output}");
    Ok(())
}

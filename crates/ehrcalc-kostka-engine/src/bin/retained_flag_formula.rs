use ehrcalc_kostka_engine::flagged_formula::count_retained_flag_five_singletons;
use serde_json::json;

fn main() -> Result<(), String> {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() != 3 {
        return Err("usage: retained_flag_formula START END".to_string());
    }
    let start: u32 = arguments[1]
        .parse()
        .map_err(|_| "START must be a nonnegative integer".to_string())?;
    let end: u32 = arguments[2]
        .parse()
        .map_err(|_| "END must be a nonnegative integer".to_string())?;
    if end < start {
        return Err("END must be at least START".to_string());
    }
    for dilation in start..=end {
        let stats = count_retained_flag_five_singletons(dilation)?;
        println!(
            "{}",
            json!({
                "dilation": dilation,
                "state_counts": stats.state_counts,
                "ordinary": stats.value.to_string(),
                "certified_upper_bound": stats.certified_upper_bound.to_string(),
                "moduli": stats.moduli,
                "residues": stats.residues,
            })
        );
    }
    Ok(())
}

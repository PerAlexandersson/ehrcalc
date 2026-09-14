use ehrcalc_kostka_engine::packed_modular::crt_reconstruct_bounded;
use num_bigint::BigUint;
use std::env;

fn parse_u32_list(raw: &str, name: &str) -> Result<Vec<u32>, String> {
    raw.split(',')
        .map(|part| {
            part.parse::<u32>()
                .map_err(|error| format!("invalid {name} entry {part:?}: {error}"))
        })
        .collect()
}

fn main() -> Result<(), String> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 4 {
        return Err(format!(
            "usage: {} UPPER_BOUND RESIDUES MODULI",
            arguments
                .first()
                .map(String::as_str)
                .unwrap_or("reconstruct_modular")
        ));
    }
    let upper_bound = BigUint::parse_bytes(arguments[1].as_bytes(), 10)
        .ok_or_else(|| "invalid nonnegative decimal upper bound".to_string())?;
    let residues = parse_u32_list(&arguments[2], "residue")?;
    let moduli = parse_u32_list(&arguments[3], "modulus")?;
    println!(
        "{}",
        crt_reconstruct_bounded(&residues, &moduli, &upper_bound)?
    );
    Ok(())
}

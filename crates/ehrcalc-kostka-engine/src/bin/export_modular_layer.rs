use ehrcalc_kostka_engine::packed_modular::{
    try_masked_flagged_skew_kostka_modular_layer_trace,
    try_masked_flagged_skew_kostka_modular_stats, DEFAULT_MODULI,
};
use ehrcalc_kostka_engine::Partition;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const MAGIC: &[u8; 8] = b"EHRGPU1\0";

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

fn write_u32(output: &mut impl Write, value: u32) -> std::io::Result<()> {
    output.write_all(&value.to_le_bytes())
}

fn write_u64(output: &mut impl Write, value: u64) -> std::io::Result<()> {
    output.write_all(&value.to_le_bytes())
}

fn main() -> Result<(), String> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 9 && arguments.len() != 10 && arguments.len() != 13 {
        return Err(format!(
            "usage: {} OUTPUT LAYER DILATION OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS \
             [MODULUS [FORBIDDEN_MASKS STRICT_LOWER_MASKS STRICT_DIAGONAL_MASKS]]",
            arguments
                .first()
                .map(String::as_str)
                .unwrap_or("export_modular_layer")
        ));
    }

    let output_path = Path::new(&arguments[1]);
    let layer: usize = arguments[2]
        .parse()
        .map_err(|error| format!("invalid zero-indexed layer: {error}"))?;
    let dilation: u32 = arguments[3]
        .parse()
        .map_err(|error| format!("invalid dilation: {error}"))?;
    let outer = scale(&parse_list(&arguments[4], "outer")?, dilation, "outer")?;
    let inner = scale(&parse_list(&arguments[5], "inner")?, dilation, "inner")?;
    let weight = scale(&parse_list(&arguments[6], "weight")?, dilation, "weight")?;
    let upper = parse_list(&arguments[7], "upper flag")?;
    let lower = parse_list(&arguments[8], "lower flag")?;
    let modulus = arguments
        .get(9)
        .map(|raw| {
            raw.parse::<u32>()
                .map_err(|error| format!("invalid modulus: {error}"))
        })
        .transpose()?
        .unwrap_or(DEFAULT_MODULI[0]);
    let forbidden = arguments
        .get(10)
        .map(|raw| parse_list(raw, "forbidden-row mask"))
        .transpose()?
        .unwrap_or_default();
    let strict_lower = arguments
        .get(11)
        .map(|raw| parse_list(raw, "strict-lower mask"))
        .transpose()?
        .unwrap_or_default();
    let strict_diagonal = arguments
        .get(12)
        .map(|raw| parse_list(raw, "strict-diagonal mask"))
        .transpose()?
        .unwrap_or_default();
    let lambda = Partition::from_sorted(outer);
    let mu = Partition::from_sorted(inner);
    let upper_flags = (!upper.is_empty()).then_some(upper.as_slice());
    let lower_flags = (!lower.is_empty()).then_some(lower.as_slice());
    let forbidden_masks = (!forbidden.is_empty()).then_some(forbidden.as_slice());
    let strict_lower_masks = (!strict_lower.is_empty()).then_some(strict_lower.as_slice());
    let strict_diagonal_masks = (!strict_diagonal.is_empty()).then_some(strict_diagonal.as_slice());

    if output_path == Path::new("-") {
        let stats = try_masked_flagged_skew_kostka_modular_stats(
            &lambda,
            &mu,
            &weight,
            upper_flags,
            lower_flags,
            forbidden_masks,
            strict_lower_masks,
            strict_diagonal_masks,
            &[modulus],
            None,
            false,
        )?;
        for (level, states) in stats.level_states.iter().enumerate() {
            if level == 0 {
                eprintln!("level=0 states={states}");
            } else {
                eprintln!(
                    "level={level} states={states} transitions={}",
                    stats.level_transitions[level - 1]
                );
            }
        }
        eprintln!(
            "peak_states={} residues={:?} moduli={:?}",
            stats.peak_states, stats.residues, stats.moduli
        );
        return Ok(());
    }

    let trace = try_masked_flagged_skew_kostka_modular_layer_trace(
        &lambda,
        &mu,
        &weight,
        upper_flags,
        lower_flags,
        forbidden_masks,
        strict_lower_masks,
        strict_diagonal_masks,
        modulus,
        layer,
        None,
    )?;

    let file = File::create(output_path)
        .map_err(|error| format!("failed to create {}: {error}", output_path.display()))?;
    let mut output = BufWriter::new(file);
    output
        .write_all(MAGIC)
        .and_then(|_| write_u32(&mut output, trace.modulus))
        .and_then(|_| write_u32(&mut output, trace.rows as u32))
        .and_then(|_| write_u32(&mut output, trace.bits_per_row))
        .and_then(|_| write_u32(&mut output, 0))
        .and_then(|_| write_u64(&mut output, trace.source_states as u64))
        .and_then(|_| write_u64(&mut output, trace.records.len() as u64))
        .and_then(|_| write_u64(&mut output, trace.reduced.len() as u64))
        .map_err(|error| format!("failed to write trace header: {error}"))?;

    for record in trace.records.iter().chain(&trace.reduced) {
        write_u64(&mut output, record.key as u64)
            .and_then(|_| write_u64(&mut output, (record.key >> 64) as u64))
            .and_then(|_| write_u32(&mut output, record.value))
            .map_err(|error| format!("failed to write transition record: {error}"))?;
    }
    output
        .flush()
        .map_err(|error| format!("failed to flush trace: {error}"))?;

    eprintln!(
        "layer={layer} source_states={} transitions={} unique_states={} output={}",
        trace.source_states,
        trace.records.len(),
        trace.reduced.len(),
        output_path.display()
    );
    Ok(())
}

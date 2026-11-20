//! Bounded JSONL benchmark adapter. Resource and wall-clock limits belong to
//! the accompanying Python runner; each input is one isolated process.
use ehrcalc::families::{
    ConstraintOptions, ConstraintPolytopeInput, CountingPlan, OrderHeuristic,
    PreparedConstraintPolytope,
};
use ehrcalc::render::counting_plan_json;
use ehrcalc_kostka_engine::constraint_polytope::ConstraintError;
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::time::Instant;

fn emit(value: Value) {
    println!("{value}");
    std::io::stdout().flush().expect("flush JSONL");
}

fn legacy_plans(
    prepared: &PreparedConstraintPolytope,
    dilation: u64,
    strict: bool,
    options: &ConstraintOptions,
) -> Result<Vec<CountingPlan>, String> {
    let mut best = Vec::new();
    for (index, component) in prepared.components().iter().enumerate() {
        let factor = prepared.component(index).map_err(|e| e.to_string())?;
        let mut choices = Vec::new();
        for heuristic in [
            OrderHeuristic::Greedy,
            OrderHeuristic::SumContiguous,
            OrderHeuristic::ReverseCuthillMcKee,
            OrderHeuristic::ExactSubset,
        ] {
            if heuristic == OrderHeuristic::ExactSubset && component.len() > 22 {
                continue;
            }
            let mut plans = factor
                .plan(
                    dilation,
                    strict,
                    &ConstraintOptions {
                        order_heuristic: heuristic,
                        plan_only: true,
                        ..options.clone()
                    },
                )
                .map_err(|e| e.to_string())?;
            choices.push(plans.remove(0));
        }
        let mut chosen = choices
            .iter()
            .min_by(|a, b| {
                (&a.estimated_peak, &a.estimated_work).cmp(&(&b.estimated_peak, &b.estimated_work))
            })
            .expect("legacy candidates")
            .clone();
        chosen.candidates = choices.into_iter().flat_map(|p| p.candidates).collect();
        chosen.selection = "legacy-estimate".into();
        best.push(chosen);
    }
    Ok(best)
}

fn run(request: &Value, start: Instant) -> Result<(), String> {
    let input: ConstraintPolytopeInput =
        serde_json::from_value(request["system"].clone()).map_err(|e| e.to_string())?;
    let cap = request["max_states"].as_u64().unwrap_or(2_000_000) as usize;
    let prepared = PreparedConstraintPolytope::new(&input).map_err(|e| e.to_string())?;
    let policy = request["policy"].as_str().unwrap_or("auto");
    let options = ConstraintOptions {
        max_states: Some(cap),
        extra_samples: 0,
        order_heuristic: if policy == "gt-level" {
            OrderHeuristic::GtLevel
        } else {
            OrderHeuristic::Auto
        },
        ..ConstraintOptions::default()
    };
    let mut schedule_options = options.clone();
    // Same estimate-based interpolation split as the production service.
    // Legacy computes the original four-candidate choices below.
    if policy == "legacy-auto" {
        schedule_options.order_heuristic = OrderHeuristic::Greedy;
    }
    let single = request["mode"].as_str() == Some("count");
    let (samples, ordinary, interior) = if single {
        let dilation = request["dilation"].as_u64().unwrap_or(1);
        let strict = request["strict"].as_bool().unwrap_or(false);
        let plans = if policy == "legacy-auto" {
            legacy_plans(&prepared, dilation, strict, &options)?
        } else {
            prepared
                .plan(dilation, strict, &options)
                .map_err(|e| e.to_string())?
        };
        (
            vec![(dilation, strict)],
            if strict { Vec::new() } else { plans.clone() },
            if strict { plans } else { Vec::new() },
        )
    } else {
        let plan = prepared
            .sample_plan(&schedule_options)
            .map_err(|e| e.to_string())?;
        let largest = plan.ordinary_plans.first().map_or(1, |p| p.dilation);
        let ordinary = if policy == "legacy-auto" {
            legacy_plans(&prepared, largest, false, &options)?
        } else {
            plan.ordinary_plans
        };
        let interior = if policy == "legacy-auto" && plan.reciprocal > 0 {
            legacy_plans(&prepared, largest, true, &options)?
        } else {
            plan.strict_plans
        };
        let samples = (1..=plan.positive)
            .map(|n| (n as u64, false))
            .chain((1..=plan.reciprocal).map(|n| (n as u64, true)))
            .collect();
        (samples, ordinary, interior)
    };
    emit(
        json!({"event":"plan", "dimension":prepared.dimension(), "policy":policy,
        "ordinary":ordinary.iter().map(counting_plan_json).collect::<Vec<_>>(),
        "strict":interior.iter().map(counting_plan_json).collect::<Vec<_>>(),
        "samples":samples, "elapsed_micros":start.elapsed().as_micros()}),
    );
    let mut peak = 0;
    for (dilation, strict) in samples {
        let plans = if strict { &interior } else { &ordinary };
        let mut order = plans
            .iter()
            .flat_map(|p| p.order.clone())
            .collect::<Vec<_>>();
        for v in 0..input.variables {
            if !order.contains(&v) {
                order.push(v);
            }
        }
        let sample_start = Instant::now();
        match prepared.count(
            dilation,
            strict,
            &ConstraintOptions {
                order: Some(order),
                ..options.clone()
            },
        ) {
            Ok(count) => {
                peak = peak.max(count.peak_states);
                emit(
                    json!({"event":"sample", "dilation":dilation,"strict":strict,
                    "count":count.value.to_string(),"peak_states":count.peak_states,
                    "elapsed_micros":sample_start.elapsed().as_micros()}),
                );
            }
            Err(error) => {
                let capped = matches!(error, ConstraintError::StateLimitExceeded { .. });
                emit(
                    json!({"event":"result","status":if capped {"state-limit"} else {"error"},
                    "error":error.to_string(),"dilation":dilation,"strict":strict,
                    "peak_states":if capped {peak.max(cap)} else {peak},"peak_is_lower_bound":capped,
                    "elapsed_micros":start.elapsed().as_micros()}),
                );
                return Ok(());
            }
        }
    }
    emit(
        json!({"event":"result","status":"complete","peak_states":peak,
        "peak_is_lower_bound":false,"elapsed_micros":start.elapsed().as_micros()}),
    );
    Ok(())
}

fn main() {
    let start = Instant::now();
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read request");
    let result = serde_json::from_str::<Value>(&input)
        .map_err(|e| e.to_string())
        .and_then(|request| run(&request, start));
    if let Err(error) = result {
        emit(
            json!({"event":"result","status":"error","error":error,"elapsed_micros":start.elapsed().as_micros()}),
        );
        std::process::exit(1);
    }
}

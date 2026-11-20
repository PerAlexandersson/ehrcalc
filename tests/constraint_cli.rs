//! Binary-level input, converter, plan, schema, warning and budget contracts.
use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run(args: &[&str], stdin: Option<&str>) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ehrcalc"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(input) = stdin {
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
    }
    // Close the input even when empty, so stdin parsing cannot block.
    drop(child.stdin.take());
    child.wait_with_output().unwrap()
}
fn success(args: &[&str], stdin: Option<&str>) -> Value {
    let output = run(args, stdin);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut value = serde_json::from_slice(&output.stdout).unwrap();
    remove_timing(&mut value);
    value
}
// Wall-clock observations are deliberately part of the JSON contract, but
// cannot be compared byte-for-byte across equivalent invocations.
fn remove_timing(value: &mut Value) {
    match value {
        Value::Object(fields) => {
            fields.remove("elapsed_micros");
            fields.remove("predicted_runtime_micros");
            for field in fields.values_mut() {
                remove_timing(field);
            }
        }
        Value::Array(values) => {
            for value in values {
                remove_timing(value);
            }
        }
        _ => {}
    }
}
const INTERVAL: &str = r#"{"variables":1,"bounds":[[0,2]]}"#;

#[test]
fn pilot_evidence_and_gt_heuristics_are_exposed_by_cli() {
    let output = run(&["polytope", "-", "--format", "json"], Some(INTERVAL));
    assert!(output.status.success());
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    let plan = &result["plan"]["ordinary"][0];
    assert_eq!(plan["selection"], "pilot");
    for candidate in plan["candidates"].as_array().unwrap() {
        assert!(candidate["predicted_peak_states"].is_string());
        assert!(candidate["predicted_runtime_micros"].is_string());
        for pilot in candidate["pilots"].as_array().unwrap() {
            assert!(pilot["elapsed_micros"].is_number());
            assert!(pilot["actual_peak_states"].is_number());
            assert!(pilot["status"].is_string());
        }
    }
    let system = success(
        &[
            "kostka",
            "--lambda",
            "3,2,1",
            "--weight",
            "2,2,2",
            "--emit-constraints",
            "--reduced",
        ],
        None,
    );
    for heuristic in ["gt-level", "gt-row"] {
        let value = success(
            &[
                "polytope",
                "-",
                "--order-heuristic",
                heuristic,
                "--format",
                "json",
            ],
            Some(&system.to_string()),
        );
        assert_eq!(value["dimension"], 1);
        assert!(value["plan"]["ordinary"][0]["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .all(|c| c["pilots"] == json!([])));
    }
}

#[test]
fn json_text_stdin_and_file_inputs_use_one_result_contract() {
    let expected = success(&["polytope", "-", "--format", "json"], Some(INTERVAL));
    assert_eq!(expected["schema"], "ehrcalc.constraint-polytope-result.v1");
    assert_eq!(expected["dimension"], 1);
    assert_eq!(expected["ehrhart_power"], json!(["1", "2"]));
    assert_eq!(expected["hstar"], json!(["1", "1"]));
    assert_eq!(expected["binomial_basis"], json!(["1", "2"]));
    assert_eq!(expected["assumes_polynomial"], true);
    assert_eq!(expected["warnings"], json!([]));
    assert!(expected["samples"]
        .as_array()
        .unwrap()
        .iter()
        .any(|s| s["interpolation"] == false));
    assert_eq!(
        expected,
        success(
            &["polytope", "--input", "-", "--format", "json"],
            Some(INTERVAL)
        )
    );
    assert_eq!(
        expected,
        success(
            &["polytope", "--text", "-", "--format", "json"],
            Some("vars 1\nbound 0 0 2\n")
        )
    );
    let path = std::env::temp_dir().join(format!(
        "ehrcalc-constraint-cli-{}.json",
        std::process::id()
    ));
    std::fs::write(&path, INTERVAL).unwrap();
    let file = success(
        &[
            "polytope",
            "--input",
            path.to_str().unwrap(),
            "--format",
            "json",
        ],
        None,
    );
    std::fs::remove_file(path).unwrap();
    assert_eq!(file, expected);
    for sample in expected["samples"].as_array().unwrap() {
        assert!(sample["count"].is_string());
        assert!(sample["actual_peak_states"].is_number());
        for crt in sample["crt"].as_array().unwrap() {
            assert!(crt["bound_bits"].is_number());
            assert_eq!(
                crt["moduli_count"].as_u64().unwrap() as usize,
                crt["moduli"].as_array().unwrap().len()
            );
            assert!(crt["upper_bound"].is_string());
        }
    }
    for format in ["text", "latex"] {
        let output = run(&["polytope", "-", "--format", format], Some(INTERVAL));
        assert!(output.status.success());
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(text.contains("estimated peak"));
        assert!(text.contains("actual peak"));
        if format == "latex" {
            assert!(text.contains("L_P(n)"));
        }
    }
}

#[test]
fn converters_feed_the_same_polytope_command() {
    let mut gt_results = Vec::new();
    for reduced in [false, true] {
        let mut args = vec![
            "kostka",
            "--lambda",
            "3,2,1",
            "--weight",
            "2,2,2",
            "--lower-flags",
            "1,1,2",
            "--emit-constraints",
        ];
        if reduced {
            args.push("--reduced");
        }
        let system = success(&args, None);
        assert_eq!(system["schema"], "ehrcalc.constraint-polytope.v1");
        let result = success(
            &["polytope", "-", "--format", "json"],
            Some(&system.to_string()),
        );
        gt_results.push((system, result));
    }
    assert!(gt_results[0].0["variables"].as_u64() > gt_results[1].0["variables"].as_u64());
    let reference = success(
        &[
            "gt",
            "--lambda",
            "3,2,1",
            "--weight",
            "2,2,2",
            "--lower-flags",
            "1,1,2",
            "--format",
            "json",
        ],
        None,
    );
    assert_eq!(gt_results[0].1["ehrhart"], reference);
    assert_eq!(gt_results[1].1["ehrhart"], reference);
    let system = success(&["order", "--chain", "3", "--emit-constraints"], None);
    let result = success(
        &["polytope", "-", "--format", "json"],
        Some(&system.to_string()),
    );
    assert_eq!(result["dimension"], 3);
    assert_eq!(result["hstar"], json!(["1", "0", "0", "0"]));
    assert!(!run(
        &["kostka", "--lambda", "1", "--weight", "1", "--reduced"],
        None
    )
    .status
    .success());
}

#[test]
fn ordering_plan_only_exact_and_positive_options() {
    let input = r#"{"variables":3,"bounds":[[0,1],[0,1],[0,1]],"sums":[{"vars":[0,1,2],"lower":1,"upper":1}]}"#;
    let expected = success(&["polytope", "-", "--format", "json"], Some(input));
    for heuristic in [
        "auto",
        "greedy",
        "sum-contiguous",
        "rcm",
        "exact-subset",
        "min-degree",
        "min-fill",
        "cm",
        "local",
    ] {
        let result = success(
            &[
                "polytope",
                "-",
                "--format",
                "json",
                "--order-heuristic",
                heuristic,
                "--positive-only",
                "--exact",
                "--extra-samples",
                "2",
            ],
            Some(input),
        );
        assert_eq!(result["ehrhart"], expected["ehrhart"]);
        assert_eq!(result["plan"]["reciprocal"], 0);
        assert_eq!(
            result["plan"]["extra_dilations"].as_array().unwrap().len(),
            2
        );
        assert!(result["samples"]
            .as_array()
            .unwrap()
            .iter()
            .all(|s| s["crt"] == json!([])));
    }
    let plan = success(
        &[
            "polytope",
            "-",
            "--format",
            "json",
            "--plan-only",
            "--order",
            "2,0,1",
        ],
        Some(input),
    );
    assert_eq!(plan["samples"], json!([]));
    assert_eq!(plan["ehrhart"], Value::Null);
    assert_eq!(plan["dimension"], 2);
    assert_eq!(plan["plan"]["actual_peak_states"], Value::Null);
    assert_eq!(plan["plan"]["ordinary"][0]["order"], json!([2, 0, 1]));
    for extra in [
        &["--order", "0,0,1"][..],
        &["--max-states", "0"][..],
        &["--order-heuristic", "bogus"][..],
    ] {
        let mut args = vec!["polytope", "-"];
        args.extend(extra);
        assert!(!run(&args, Some(input)).status.success());
    }
}

#[test]
fn failed_polynomiality_is_visible_in_json_text_latex_and_stderr() {
    let input = r#"{"variables":1,"bounds":[[0,"1/2"]]}"#;
    for format in ["json", "text", "latex"] {
        let result = run(&["polytope", "-", "--format", format], Some(input));
        assert!(result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("extra-sample mismatch"));
        if format == "json" {
            let value: Value = serde_json::from_slice(&result.stdout).unwrap();
            assert!(!value["warnings"].as_array().unwrap().is_empty());
            assert!(!value["mismatches"].as_array().unwrap().is_empty());
        } else {
            assert!(String::from_utf8_lossy(&result.stdout).contains("WARNING:"));
        }
    }
    let unchecked = success(
        &["polytope", "-", "--extra-samples", "0", "--format", "json"],
        Some(input),
    );
    assert_eq!(unchecked["mismatches"], json!([]));
}

#[test]
fn invalid_sources_and_constraints_fail_deterministically() {
    for args in [
        &["polytope"][..],
        &["polytope", "-", "--input", "-"][..],
        &["polytope", "--input", "/definitely/missing.json"][..],
    ] {
        assert!(!run(args, Some(INTERVAL)).status.success());
    }
    for input in [
        "{}",
        r#"{"variables":1,"bounds":[[2,1]]}"#,
        r#"{"variables":1,"differences":[[0,2,0]]}"#,
        r#"{"variables":1}"#,
    ] {
        let first = run(&["polytope", "-"], Some(input));
        let second = run(&["polytope", "-"], Some(input));
        assert!(!first.status.success());
        assert_eq!(first.stderr, second.stderr);
    }
}

#[test]
fn checked_in_examples_reproduce_the_documented_data() {
    for reduced in [false, true] {
        let mut args = vec![
            "kostka",
            "--lambda",
            "4,3,1",
            "--weight",
            "2,2,2,2",
            "--upper-flags",
            "2,3,3,3",
            "--lower-flags",
            "1,1,1,2",
            "--emit-constraints",
        ];
        if reduced {
            args.push("--reduced");
        }
        let generated = success(&args, None);
        let file = if reduced {
            "flagged-reduced.json"
        } else {
            "flagged-full.json"
        };
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/constraints")
            .join(file);
        let checked: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        assert_eq!(checked, generated);
    }
    let examples = [
        (
            "hypersimplex.json",
            3,
            json!(["1", "7/3", "2", "2/3"]),
            json!(["1", "2", "1", "0"]),
        ),
        (
            "birkhoff3.json",
            4,
            json!(["1", "9/4", "15/8", "3/4", "1/8"]),
            json!(["1", "1", "1", "0", "0"]),
        ),
        (
            "flagged-full.json",
            2,
            json!(["1", "2", "1"]),
            json!(["1", "1", "0"]),
        ),
        (
            "flagged-reduced.json",
            2,
            json!(["1", "2", "1"]),
            json!(["1", "1", "0"]),
        ),
    ];
    for (file, dimension, coefficients, hstar) in examples {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/constraints")
            .join(file);
        let result = success(
            &[
                "polytope",
                "--input",
                path.to_str().unwrap(),
                "--format",
                "json",
            ],
            None,
        );
        assert_eq!(result["dimension"], dimension);
        assert_eq!(result["ehrhart_power"], coefficients);
        assert_eq!(result["hstar"], hstar);
        assert_eq!(result["warnings"], json!([]));
    }
}

#[test]
fn audited_polynomiality_limits_and_order_conflicts() {
    let fixed = r#"{"variables":2,"bounds":[["1/2","1/2"],[0,1]]}"#;
    let value = success(
        &["polytope", "-", "--extra-samples", "0", "--format", "json"],
        Some(fixed),
    );
    assert_eq!(value["polynomiality_checks_failed"], true);
    assert!(value["warnings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|w| w.as_str().unwrap().contains("fixed value")));
    assert!(!run(
        &[
            "polytope",
            "-",
            "--extra-samples",
            "0",
            "--fail-on-mismatch"
        ],
        Some(fixed)
    )
    .status
    .success());
    let implicit = r#"{"variables":2,"bounds":[[0,1],[0,1]],"sums":[{"vars":[0,1],"lower":"1/2","upper":"1/2"}]}"#;
    let value = success(
        &["polytope", "-", "--extra-samples", "0", "--format", "json"],
        Some(implicit),
    );
    assert!(value["warnings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|w| w.as_str().unwrap().contains("implicit-equality")));
    for (input, expected) in [
        (
            r#"{"variables":1,"bounds":[["1/3","2/3"]]}"#,
            "nonpositive leading coefficient",
        ),
        (
            r#"{"variables":2,"bounds":[[0,"1/10"],[0,"1/10"]]}"#,
            "differs from affine dimension",
        ),
    ] {
        let value = success(
            &[
                "polytope",
                "-",
                "--positive-only",
                "--extra-samples",
                "0",
                "--format",
                "json",
            ],
            Some(input),
        );
        assert!(
            value["warnings"]
                .as_array()
                .unwrap()
                .iter()
                .any(|w| w.as_str().unwrap().contains(expected)),
            "{value}"
        );
    }
    let conflict = success(
        &["polytope", "--text", "-", "--format", "json"],
        Some("vars 1\nbound 0 0 1\nbound 0 2 3"),
    );
    assert_eq!(conflict["empty"], true);
    for (args, input, message) in [
        (
            vec!["polytope", "-"],
            r#"{"variables":1000000000000}"#,
            "at most 256",
        ),
        (
            vec!["polytope", "--text", "-"],
            "vars 1000000000000",
            "at most 256",
        ),
        (
            vec!["polytope", "-", "--extra-samples", "65"],
            INTERVAL,
            "at most 64",
        ),
        (
            vec!["polytope", "-", "--order", "0", "--order-heuristic", "auto"],
            INTERVAL,
            "cannot be used with",
        ),
    ] {
        let output = run(&args, Some(input));
        assert!(!output.status.success());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(message),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let point = r#"{"variables":0}"#;
    let boundary = success(
        &["polytope", "-", "--extra-samples", "64", "--format", "json"],
        Some(point),
    );
    assert_eq!(boundary["samples"].as_array().unwrap().len(), 65);
}

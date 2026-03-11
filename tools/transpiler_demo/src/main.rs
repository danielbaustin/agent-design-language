use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

const FIXTURE_REL: &str = "examples/workflows/rust_transpiler_demo.yaml";
const RUST_OUTPUT_REL: &str = "demos/rust_output/workflow_runtime.rs";
const VERIFICATION_REL: &str = "demos/rust_output/transpiler_verification.v0.8.json";

#[derive(Debug, Clone, Serialize)]
struct MappingPair {
    index: usize,
    workflow_step_id: String,
    rust_function: String,
    status: String,
}

#[derive(Debug, Clone, Serialize)]
struct MappingReport {
    pairs: Vec<MappingPair>,
    order_check: String,
}

#[derive(Debug, Clone, Serialize)]
struct AdaptiveExecutionReport {
    mode: String,
    attempts_executed: usize,
    policy_actions: Vec<String>,
    notes: String,
}

#[derive(Debug, Clone, Serialize)]
struct ValidationReport {
    command: String,
    status: String,
}

#[derive(Debug, Clone, Serialize)]
struct VerificationArtifact {
    schema_version: String,
    workflow_fixture: String,
    runtime_skeleton: String,
    mapping: MappingReport,
    adaptive_execution: AdaptiveExecutionReport,
    validation: ValidationReport,
}

fn extract_fixture_step_ids(yaml_text: &str) -> Vec<String> {
    yaml_text
        .lines()
        .filter_map(|line| {
            let t = line.trim();
            if !t.starts_with("- id:") {
                return None;
            }
            let first = t.find('"')?;
            let second = t[first + 1..].find('"')? + first + 1;
            Some(t[first + 1..second].to_string())
        })
        .collect()
}

fn extract_rust_step_functions(rust_text: &str) -> Vec<String> {
    rust_text
        .lines()
        .filter_map(|line| {
            let t = line.trim_start();
            if !t.starts_with("fn step_") {
                return None;
            }
            let open = t.find('(')?;
            Some(t[3..open].trim().to_string())
        })
        .collect()
}

fn path_from_repo_root(rel: &str) -> PathBuf {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    repo_root.join(rel)
}

fn main() -> Result<(), String> {
    let fixture_path = path_from_repo_root(FIXTURE_REL);
    let rust_output_path = path_from_repo_root(RUST_OUTPUT_REL);
    let verification_path = path_from_repo_root(VERIFICATION_REL);

    if !fixture_path.exists() {
        return Err(format!("missing fixture: {}", fixture_path.display()));
    }
    if !rust_output_path.exists() {
        return Err(format!(
            "missing rust output skeleton: {}",
            rust_output_path.display()
        ));
    }

    let fixture_text =
        fs::read_to_string(&fixture_path).map_err(|e| format!("failed to read fixture: {e}"))?;
    let rust_text = fs::read_to_string(&rust_output_path)
        .map_err(|e| format!("failed to read rust output skeleton: {e}"))?;

    let fixture_steps = extract_fixture_step_ids(&fixture_text);
    let rust_steps = extract_rust_step_functions(&rust_text);

    if fixture_steps.is_empty() {
        return Err("no workflow steps found in fixture".to_string());
    }
    if rust_steps.is_empty() {
        return Err("no step functions found in rust output skeleton".to_string());
    }

    println!("v0.8 rust transpiler demo scaffold");
    println!("fixture: {FIXTURE_REL}");
    println!("rust output skeleton: {RUST_OUTPUT_REL}");
    println!("deterministic step mapping (fixture -> rust):");

    let mut pairs = Vec::new();
    for (index, step_id) in fixture_steps.iter().enumerate() {
        let rust_fn = rust_steps
            .get(index)
            .map(String::as_str)
            .unwrap_or("<missing>");
        let status = if step_id == rust_fn { "PASS" } else { "FAIL" };
        pairs.push(MappingPair {
            index: index + 1,
            workflow_step_id: step_id.clone(),
            rust_function: rust_fn.to_string(),
            status: status.to_string(),
        });
        println!(
            "  {}. {} -> fn {}(...) [{}]",
            index + 1,
            step_id,
            rust_fn,
            status
        );
    }

    let same_order = fixture_steps == rust_steps;
    println!(
        "mapping order check: {}",
        if same_order { "PASS" } else { "FAIL" }
    );
    println!("artifact layout check:");
    println!("  - fixture exists: PASS");
    println!("  - rust skeleton exists: PASS");
    println!("  - verification artifact emitted: {}", VERIFICATION_REL);
    println!("note: this scaffold demonstrates deterministic mapping only;");
    println!("it does not generate Rust code dynamically.");

    let artifact = VerificationArtifact {
        schema_version: "rust_transpiler_verification.v0.8".to_string(),
        workflow_fixture: FIXTURE_REL.to_string(),
        runtime_skeleton: RUST_OUTPUT_REL.to_string(),
        mapping: MappingReport {
            pairs,
            order_check: if same_order {
                "PASS".to_string()
            } else {
                "FAIL".to_string()
            },
        },
        adaptive_execution: AdaptiveExecutionReport {
            mode: "bounded_reporting_only".to_string(),
            attempts_executed: 0,
            policy_actions: Vec::new(),
            notes: "WP-12 verification captures adaptive execution evidence fields only; no autonomous retry loop executed.".to_string(),
        },
        validation: ValidationReport {
            command:
                "cargo run --manifest-path tools/transpiler_demo/Cargo.toml --quiet".to_string(),
            status: if same_order {
                "PASS".to_string()
            } else {
                "FAIL".to_string()
            },
        },
    };
    let bytes = serde_json::to_vec_pretty(&artifact)
        .map_err(|e| format!("failed to serialize verification artifact: {e}"))?;
    fs::write(&verification_path, bytes)
        .map_err(|e| format!("failed to write verification artifact: {e}"))?;

    if same_order {
        Ok(())
    } else {
        Err("fixture and rust step ordering do not match".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_step_ids_from_fixture_shape() {
        let yaml = r#"
steps:
  - id: "step_prepare_input"
  - id: "step_normalize_payload"
  - id: "step_finalize_output"
"#;
        let got = extract_fixture_step_ids(yaml);
        assert_eq!(
            got,
            vec![
                "step_prepare_input".to_string(),
                "step_normalize_payload".to_string(),
                "step_finalize_output".to_string()
            ]
        );
    }

    #[test]
    fn extracts_rust_step_functions_in_source_order() {
        let rs = r#"
fn step_prepare_input(source_token: &str) -> String { source_token.to_string() }
fn step_normalize_payload(payload: &str) -> String { payload.trim().to_string() }
fn step_finalize_output(normalized_payload: &str) -> String { normalized_payload.to_string() }
"#;
        let got = extract_rust_step_functions(rs);
        assert_eq!(
            got,
            vec![
                "step_prepare_input".to_string(),
                "step_normalize_payload".to_string(),
                "step_finalize_output".to_string()
            ]
        );
    }

    #[test]
    fn repo_root_relative_paths_are_stable() {
        let fixture = path_from_repo_root(FIXTURE_REL);
        let output = path_from_repo_root(RUST_OUTPUT_REL);
        assert!(fixture.ends_with(Path::new(FIXTURE_REL)));
        assert!(output.ends_with(Path::new(RUST_OUTPUT_REL)));
    }
}

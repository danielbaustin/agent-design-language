use std::fs;
use std::path::Path;

const FIXTURE_PATH: &str = "examples/workflows/rust_transpiler_demo.yaml";
const RUST_OUTPUT_PATH: &str = "demos/rust_output/workflow_runtime.rs";

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

fn main() -> Result<(), String> {
    if !Path::new(FIXTURE_PATH).exists() {
        return Err(format!("missing fixture: {FIXTURE_PATH}"));
    }
    if !Path::new(RUST_OUTPUT_PATH).exists() {
        return Err(format!("missing rust output skeleton: {RUST_OUTPUT_PATH}"));
    }

    let fixture_text = fs::read_to_string(FIXTURE_PATH)
        .map_err(|e| format!("failed to read fixture: {e}"))?;
    let rust_text = fs::read_to_string(RUST_OUTPUT_PATH)
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
    println!("fixture: {FIXTURE_PATH}");
    println!("rust output skeleton: {RUST_OUTPUT_PATH}");
    println!("deterministic step mapping (fixture -> rust):");

    for (index, step_id) in fixture_steps.iter().enumerate() {
        let rust_fn = rust_steps
            .get(index)
            .map(String::as_str)
            .unwrap_or("<missing>");
        let status = if step_id == rust_fn { "PASS" } else { "FAIL" };
        println!("  {}. {} -> fn {}(...) [{}]", index + 1, step_id, rust_fn, status);
    }

    let same_order = fixture_steps == rust_steps;
    println!(
        "mapping order check: {}",
        if same_order { "PASS" } else { "FAIL" }
    );
    println!("artifact layout check:");
    println!("  - fixture exists: PASS");
    println!("  - rust skeleton exists: PASS");
    println!("note: this scaffold demonstrates deterministic mapping only;");
    println!("it does not generate Rust code dynamically.");

    if same_order {
        Ok(())
    } else {
        Err("fixture and rust step ordering do not match".to_string())
    }
}

// v0.8 bounded Rust transpiler demo output skeleton.
// This file represents deterministic compiled structure from:
// demos/rust-transpiler/workflow/rust_transpiler_demo.yaml

#[derive(Debug, Clone)]
struct WorkflowState {
    source_token: String,
    payload: String,
    normalized_payload: String,
    result_token: String,
}

fn step_prepare_input(source_token: &str) -> String {
    source_token.to_string()
}

fn step_normalize_payload(payload: &str) -> String {
    payload.trim().to_lowercase()
}

fn step_finalize_output(normalized_payload: &str) -> String {
    format!("workflow_complete:{normalized_payload}")
}

fn run_workflow() -> WorkflowState {
    let source_token = "Hello_ADL  ".to_string();
    let payload = step_prepare_input(&source_token);
    let normalized_payload = step_normalize_payload(&payload);
    let result_token = step_finalize_output(&normalized_payload);

    WorkflowState {
        source_token,
        payload,
        normalized_payload,
        result_token,
    }
}

fn main() {
    let state = run_workflow();
    println!("{:?}", state);
}

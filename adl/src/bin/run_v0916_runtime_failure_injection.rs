const MESSAGE: &str = "\
run_v0916_runtime_failure_injection is retired.

The retained v0.91.6/#4547 packet remains historical evidence only.
Use the #5665 Runtime v3 API WSS proof instead:
- adl-runtime/tests/runtime_api_wss.rs
- docs/milestones/v0.91.8/review/runtime/5665_feature_adapter_matrix.json
";

fn main() {
    eprintln!("{MESSAGE}");
    std::process::exit(2);
}

const MESSAGE: &str = "\
run_wp12_acip_websocket_transport_proof is retired.

The retained v0.91.7 packet for #4659 remains historical evidence only.
Use the #5665 Runtime API WSS proof instead:
- adl-runtime/tests/runtime_api_wss.rs
- docs/milestones/v0.91.8/review/runtime/5665_feature_adapter_matrix.json
";

fn main() {
    eprintln!("{MESSAGE}");
    std::process::exit(2);
}

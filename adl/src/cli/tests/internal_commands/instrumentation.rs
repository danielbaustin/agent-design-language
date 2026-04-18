use super::super::*;

#[test]
fn cli_internal_instrument_variants_succeed() {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/v0-5-pattern-fork-join.adl.yaml");
    real_instrument(&[
        "graph".to_string(),
        fixture.to_string_lossy().to_string(),
        "--format".to_string(),
        "json".to_string(),
    ])
    .expect("graph json");
    real_instrument(&[
        "graph".to_string(),
        fixture.to_string_lossy().to_string(),
        "--format".to_string(),
        "dot".to_string(),
    ])
    .expect("graph dot");
    real_instrument(&[
        "diff-plan".to_string(),
        fixture.to_string_lossy().to_string(),
        fixture.to_string_lossy().to_string(),
    ])
    .expect("diff-plan");
    let provider_fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/v0-7-provider-portability-http-profile.adl.yaml");
    real_instrument(&[
        "provider-substrate".to_string(),
        provider_fixture.to_string_lossy().to_string(),
    ])
    .expect("provider-substrate");
    real_instrument(&["provider-substrate-schema".to_string()]).expect("provider-substrate-schema");

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base = std::env::temp_dir().join(format!("adl-main-instrument-{now}"));
    std::fs::create_dir_all(&base).expect("create base dir");
    let left = base.join("left.trace.json");
    let right = base.join("right.trace.json");
    std::fs::write(&left, "[]").expect("write left trace");
    std::fs::write(&right, "[]").expect("write right trace");
    real_instrument(&["replay".to_string(), left.to_string_lossy().to_string()]).expect("replay");
    real_instrument(&[
        "diff-trace".to_string(),
        left.to_string_lossy().to_string(),
        right.to_string_lossy().to_string(),
    ])
    .expect("diff-trace");
    real_instrument(&["trace-schema".to_string()]).expect("trace-schema");
    let trace_v1 = base.join("trace-v1.json");
    std::fs::write(
        &trace_v1,
        serde_json::to_string_pretty(&serde_json::json!({
            "schema_version": "trace.v1",
            "events": [
                {
                    "event_id": "run-start-1",
                    "timestamp": "2026-04-03T12:00:00Z",
                    "event_type": "RUN_START",
                    "trace_id": "trace-1",
                    "run_id": "run-1",
                    "span_id": "span-root",
                    "parent_span_id": null,
                    "actor": {"type": "agent", "id": "agent.main"},
                    "scope": {"level": "run", "name": "run"},
                    "inputs_ref": "artifacts/run-1/inputs.json",
                    "outputs_ref": null,
                    "artifact_ref": null,
                    "decision_context": null,
                    "provider": null,
                    "error": null,
                    "contract_validation": null
                },
                {
                    "event_id": "run-end-1",
                    "timestamp": "2026-04-03T12:00:01Z",
                    "event_type": "RUN_END",
                    "trace_id": "trace-1",
                    "run_id": "run-1",
                    "span_id": "span-root",
                    "parent_span_id": null,
                    "actor": {"type": "agent", "id": "agent.main"},
                    "scope": {"level": "run", "name": "run"},
                    "inputs_ref": null,
                    "outputs_ref": "artifacts/run-1/outputs.json",
                    "artifact_ref": null,
                    "decision_context": null,
                    "provider": null,
                    "error": null,
                    "contract_validation": null
                }
            ]
        }))
        .expect("serialize trace v1 fixture"),
    )
    .expect("write trace v1 fixture");
    real_instrument(&[
        "validate-trace-v1".to_string(),
        trace_v1.to_string_lossy().to_string(),
    ])
    .expect("validate-trace-v1");
    let _ = std::fs::remove_dir_all(base);
}

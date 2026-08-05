use std::fs;
use std::path::{Path, PathBuf};

use ::adl::long_lived_agent::{self, RunOptions};
use ::adl::memory_palace::{
    build_context_packet, context_packet_bytes, MemoryPalaceAgentConfig, MemoryPalaceInput,
    MEMORY_PALACE_CONTEXT_REF, MEMORY_PALACE_CONTEXT_SCHEMA,
};

mod helpers;
use helpers::unique_test_temp_dir;

fn fixture_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn memory_palace_fixture() -> MemoryPalaceInput {
    let fixture = fixture_path("tests/fixtures/memory_palace/long_running_context.json");
    serde_json::from_slice(&fs::read(fixture).expect("read Memory Palace fixture"))
        .expect("parse Memory Palace fixture")
}

fn memory_palace_config(max_working_set_items: usize) -> MemoryPalaceAgentConfig {
    MemoryPalaceAgentConfig {
        input_ref: "memory_palace_input.json".to_string(),
        max_working_set_items,
        stale_after_ms: 1000,
        required_continuity_id: Some("continuity-v092-handoff".to_string()),
        observed_epoch_ms: Some(4102444800500),
    }
}

#[test]
fn memory_palace_fixture_builds_deterministic_obs_mem_handoff() {
    let input = memory_palace_fixture();
    let config = memory_palace_config(1);

    let first = build_context_packet("cycle-000001", &config, &input, 4102444800500)
        .expect("build first Memory Palace packet");
    let second = build_context_packet("cycle-000001", &config, &input, 4102444800500)
        .expect("build second Memory Palace packet");

    assert_eq!(
        context_packet_bytes(&first).unwrap(),
        context_packet_bytes(&second).unwrap()
    );
    assert_eq!(first.schema, MEMORY_PALACE_CONTEXT_SCHEMA);
    assert_eq!(first.topology.rooms.len(), 1);
    assert_eq!(first.topology.anchors.len(), 2);
    assert_eq!(first.working_set.selected.len(), 1);
    assert_eq!(first.working_set.excluded.len(), 1);
    assert_eq!(first.working_set.selected[0].record_id, "context-a");
    assert_eq!(
        first.working_set.selected[0]
            .temporal_anchor
            .continuity_id
            .as_deref(),
        Some("continuity-v092-handoff")
    );
    assert_eq!(
        first.working_set.selected[0].provenance[0].path,
        "runs/run-context-a/trace.json"
    );
    assert_eq!(first.working_set.excluded[0].record_id, "context-b");
    assert!(first
        .stale_context_report
        .dispositions
        .iter()
        .all(|disposition| disposition.status == "current"));
}

#[test]
fn long_lived_agent_cycle_consumes_memory_palace_context_ref() {
    let root = unique_test_temp_dir("memory-palace-agent");
    let fixture = fixture_path("tests/fixtures/memory_palace/long_running_context.json");
    fs::copy(&fixture, root.join("memory_palace_input.json")).expect("copy Memory Palace fixture");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: memory-palace-agent
display_name: Memory Palace Agent
state_root: state
workflow:
  kind: demo_adapter
  name: memory_palace_context_probe
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/memory-palace
  write_policy: append_only
  memory_palace:
    input_ref: memory_palace_input.json
    max_working_set_items: 1
    stale_after_ms: 1000
    required_continuity_id: continuity-v092-handoff
    observed_epoch_ms: 4102444800500
"#,
    )
    .expect("write agent spec");

    let status = long_lived_agent::run(
        &spec,
        RunOptions {
            max_cycles: 1,
            interval_secs: None,
            no_sleep: true,
            recover_stale_lease: false,
        },
    )
    .expect("run one long-lived-agent cycle");

    assert_eq!(status.last_cycle_id.as_deref(), Some("cycle-000001"));
    assert_eq!(status.last_cycle_status.as_deref(), Some("success"));
    let cycle_dir = root.join("state/cycles/cycle-000001");
    let decision_request: serde_json::Value =
        serde_json::from_slice(&fs::read(cycle_dir.join("decision_request.json")).unwrap())
            .expect("parse decision request");
    assert_eq!(
        decision_request["memory_refs"],
        serde_json::json!([MEMORY_PALACE_CONTEXT_REF])
    );

    let packet: serde_json::Value =
        serde_json::from_slice(&fs::read(cycle_dir.join(MEMORY_PALACE_CONTEXT_REF)).unwrap())
            .expect("parse Memory Palace packet");
    assert_eq!(packet["schema"], MEMORY_PALACE_CONTEXT_SCHEMA);
    assert_eq!(
        packet["working_set"]["selected"][0]["record_id"],
        "context-a"
    );
    assert_eq!(
        packet["working_set"]["selected"][0]["temporal_anchor"]["continuity_id"],
        "continuity-v092-handoff"
    );

    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(cycle_dir.join("cycle_manifest.json")).unwrap())
            .expect("parse cycle manifest");
    assert_eq!(
        manifest["artifacts"]["memory_palace_context"],
        MEMORY_PALACE_CONTEXT_REF
    );
}

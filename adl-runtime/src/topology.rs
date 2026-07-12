//! CSM runtime component topology.

use serde::Serialize;
use serde_json::{json, Value};

use crate::acip::{runtime_capability as acip_runtime_capability, CSM_ACIP_COMPONENT};
use crate::supervision::{default_component_supervision, SUPERVISION_SCHEMA};

pub const CSM_RUNTIME_STACK_SCHEMA: &str = "adl.csm.runtime_stack.v1";
pub const CSM_COMPONENT_TOPOLOGY_SCHEMA: &str = "adl.csm.component_topology.v1";
pub const CSM_RUNTIME_CHANNEL_SCHEMA: &str = "adl.csm.typed_runtime_channels.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct RuntimeComponent {
    pub id: &'static str,
    pub plane: &'static str,
    pub role: &'static str,
}

pub fn runtime_components() -> Vec<RuntimeComponent> {
    vec![
        RuntimeComponent {
            id: "runtime_api",
            plane: "operations",
            role: "embedded HTTP and WebSocket control/status API",
        },
        RuntimeComponent {
            id: "chronosense",
            plane: "operations",
            role: "time source and ordering authority",
        },
        RuntimeComponent {
            id: "scheduler",
            plane: "operations",
            role: "cadence, admission, and scheduling control",
        },
        RuntimeComponent {
            id: "weather",
            plane: "operations",
            role: "CPU, memory, disk, and GPU resource weather for graceful runtime stop decisions",
        },
        RuntimeComponent {
            id: "reasoning_runtime",
            plane: "cognition",
            role: "reasoning graphs, loops, and adaptive DAG execution",
        },
        RuntimeComponent {
            id: "curiosity_engine",
            plane: "cognition",
            role: "governed discovery and bounded hypothesis proposal routing",
        },
        RuntimeComponent {
            id: "resident_agents",
            plane: "cognition",
            role: "provider-backed resident agents admitted through CSM lifecycle",
        },
        RuntimeComponent {
            id: "freedom_gate",
            plane: "security",
            role: "lawful execution and commitment mediation gate",
        },
        RuntimeComponent {
            id: "aee",
            plane: "execution",
            role: "governed execution with actuation boundaries",
        },
        RuntimeComponent {
            id: "checkpoint",
            plane: "continuity",
            role: "partials, state snapshots, and restoration",
        },
        RuntimeComponent {
            id: "observability",
            plane: "operations",
            role: "telemetry, metrics, traces, logs, and OTel pipeline",
        },
        RuntimeComponent {
            id: "lifelog",
            plane: "continuity",
            role: "database-backed lifecycle journal",
        },
        RuntimeComponent {
            id: CSM_ACIP_COMPONENT,
            plane: "communications",
            role: "governed ACIP/A2A JSON, protobuf, and WebSocket carrier",
        },
        RuntimeComponent {
            id: "cloud_bridge",
            plane: "communications",
            role: "API Gateway, EventBridge, and Cloud SDK bridge",
        },
    ]
}

pub fn runtime_stack_json() -> Value {
    json!({
        "schema": CSM_RUNTIME_STACK_SCHEMA,
        "runtime_owner": "csm",
        "async_runtime": "tokio",
        "orchestration": {
            "model": "main_task_join_set",
            "component_set": "supervised_component_set",
            "channel_schema": CSM_RUNTIME_CHANNEL_SCHEMA
        },
        "component_topology": {
            "schema": CSM_COMPONENT_TOPOLOGY_SCHEMA,
            "components": runtime_components(),
            "supervision_schema": SUPERVISION_SCHEMA,
            "supervision": default_component_supervision()
        },
        "api_server": {
            "http_framework": "axum",
            "service_substrate": "tower",
            "http_engine": "hyper",
            "status": "integrated"
        },
        "resource_pooling": {
            "pool_crate": "deadpool",
            "status": "integrated",
            "source": "csm_connection_pool_status"
        },
        "determinism_boundary": {
            "schema": crate::determinism::CSM_DETERMINISM_BOUNDARY_SCHEMA,
            "model": "typed_deterministic_core_and_nondeterministic_shell",
            "capture_policy": "retain_before_governed_influence",
            "failure_policy": "quarantine_missing_reclassified_or_mutated_shell_evidence"
        },
        "time_sync": {
            "primary_crate": "rsntp",
            "status": "integrated",
            "source": "/chronosense"
        },
        "resource_weather": {
            "schema": crate::weather::WEATHER_SCHEMA,
            "component": "weather",
            "primary_crate": "sysinfo",
            "status": "integrated",
            "source": "/weather",
            "stop_policy": "serialize_state_then_stop_on_configured_cpu_memory_or_disk_pressure",
            "gpu_policy": "observed_on_gpu_host_or_explicitly_deferred"
        },
        "persistence_domains": {
            "schema": crate::continuity_history::PERSISTENCE_DOMAINS_SCHEMA,
            "checkpoint_continuity": {
                "schema": crate::continuity_history::CHECKPOINT_SCHEMA_V1,
                "store": "checkpoint.redb",
                "lifecycle": "write_validate_restore_or_fail_closed",
                "restore_authority": true
            },
            "autobiographical_lifelog": {
                "schema": crate::continuity_history::LIFELOG_SCHEMA_V1,
                "store": "lifelog.redb",
                "lifecycle": "append_query_retain_independently",
                "restore_authority": false
            },
            "cross_reference": "typed_identifier_only_no_shared_payload_or_transaction"
        },
        "observability_pipeline": {
            "pipeline": "vector",
            "status": "planned_csm_managed_runtime_component",
            "role": "collect_transform_redact_route_logs_metrics_and_otel",
            "runtime_topology": "csm_managed_observability_component",
            "csm_role": "emit_canonical_runtime_events_and_otel_shaped_summaries"
        },
        "resident_agent_entrypoint": {
            "schema": crate::resident_agent::CSM_RESIDENT_AGENT_SET_SCHEMA,
            "provider_entrypoint": "provider_substrate",
            "lifecycle": "same_csm_supervision_checkpoint_lifelog_observability_path_for_privileged_and_ordinary_agents",
            "shepherd_model": "privileged_resident_agent_not_bespoke_model_path"
        },
        "acip_carrier": acip_runtime_capability(),
        "curiosity_engine": {
            "schema": crate::curiosity::CSM_CURIOSITY_STATUS_SCHEMA,
            "component": crate::curiosity::CSM_CURIOSITY_COMPONENT,
            "process_model": "embedded_csm_runtime_component",
            "retained_status_ref": crate::curiosity::CSM_CURIOSITY_STATUS_REF,
            "governance": "freedom_gate_cav_constructability_fail_closed"
        },
        "freedom_gate": {
            "schema": crate::freedom_gate::CSM_FREEDOM_GATE_STATUS_SCHEMA,
            "status": "integrated",
            "component": crate::freedom_gate::CSM_FREEDOM_GATE_COMPONENT,
            "retained_status_ref": crate::freedom_gate::CSM_FREEDOM_GATE_STATUS_REF,
            "executor_requires_gate_decision": true,
            "unmediated_execution_allowed": false
        },
        "reasoning_runtime": {
            "schema": crate::reasoning_runtime::REASONING_RUNTIME_SCHEMA,
            "component": crate::reasoning_runtime::REASONING_RUNTIME_COMPONENT,
            "process_model": "csm_supervised_bounded_typed_channel_component",
            "objects": ["reasoning_graph", "bounded_loop", "adaptive_dag"],
            "determinism": "captured_provider_shell_then_deterministic_core_replay",
            "continuity": "checkpoint_lineage_and_replay_cursor_distinct_from_lifelog_history",
            "governance": "freedom_gate_before_aee"
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topology_contains_core_runtime_components() {
        let ids = runtime_components()
            .into_iter()
            .map(|component| component.id)
            .collect::<Vec<_>>();
        assert!(ids.contains(&"runtime_api"));
        assert!(ids.contains(&"chronosense"));
        assert!(ids.contains(&"scheduler"));
        assert!(ids.contains(&"weather"));
        assert!(ids.contains(&"reasoning_runtime"));
        assert!(ids.contains(&"curiosity_engine"));
        assert!(ids.contains(&"resident_agents"));
        assert!(ids.contains(&"freedom_gate"));
        assert!(ids.contains(&"acip_carrier"));
        assert!(ids.contains(&"observability"));
        assert!(ids.contains(&"cloud_bridge"));
    }

    #[test]
    fn runtime_stack_projects_freedom_gate_contract() {
        let stack = runtime_stack_json();
        assert_eq!(stack["freedom_gate"]["status"], "integrated");
        assert_eq!(
            stack["freedom_gate"]["executor_requires_gate_decision"],
            true
        );
        assert_eq!(stack["freedom_gate"]["unmediated_execution_allowed"], false);
    }
}

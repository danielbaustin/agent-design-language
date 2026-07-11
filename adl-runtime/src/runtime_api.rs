//! Embedded CSM runtime API contracts.

pub const CSM_RUNTIME_API_SCHEMA: &str = "adl.csm.runtime_api.v1";
pub const CSM_RUNTIME_API_STATUS_SCHEMA: &str = "adl.csm.runtime_api.status.v1";
pub const CSM_RUNTIME_API_HEALTH_SCHEMA: &str = "adl.csm.runtime_api.health.v1";
pub const CSM_RUNTIME_API_READY_SCHEMA: &str = "adl.csm.runtime_api.ready.v1";
pub const CSM_RUNTIME_API_METRICS_SCHEMA: &str = "adl.csm.runtime_api.metrics.v1";
pub const CSM_RUNTIME_API_EVENTS_SCHEMA: &str = "adl.csm.runtime_api.events.v1";
pub const CSM_RUNTIME_API_CHRONOSENSE_SCHEMA: &str = "adl.csm.runtime_api.chronosense.v1";
pub const CSM_RUNTIME_API_SHEPHERD_SCHEMA: &str = "adl.csm.runtime_api.shepherd.v1";
pub const CSM_RUNTIME_API_CAV_SCHEMA: &str = "adl.csm.runtime_api.cav.v1";
pub const CSM_RUNTIME_API_CURIOSITY_SCHEMA: &str = "adl.csm.runtime_api.curiosity.v1";
pub const CSM_RUNTIME_API_ACIP_SCHEMA: &str = "adl.csm.runtime_api.acip.v1";
pub const CSM_RUNTIME_API_FREEDOM_GATE_SCHEMA: &str = "adl.csm.runtime_api.freedom_gate.v1";
pub const CSM_RUNTIME_API_REASONING_SCHEMA: &str = "adl.csm.runtime_api.reasoning.v1";
pub const CSM_RUNTIME_API_WEATHER_SCHEMA: &str = "adl.csm.runtime_api.weather.v1";
pub const CSM_RUNTIME_API_API_GATEWAY_BRIDGE_SCHEMA: &str =
    "adl.csm.runtime_api.api_gateway_bridge.v1";
pub const CSM_RUNTIME_API_CONSTRUCTABILITY_SCHEMA: &str = "adl.csm.runtime_api.constructability.v1";
pub const CSM_RUNTIME_API_PERSISTENCE_SCHEMA: &str = "adl.csm.runtime_api.persistence.v1";

pub const CSM_RUNTIME_API_ENDPOINTS: [&str; 17] = [
    "/status",
    "/health",
    "/ready",
    "/metrics",
    "/events",
    "/chronosense",
    "/weather",
    "/shepherd",
    "/cav",
    "/curiosity",
    "/acip",
    "/acip/ws",
    "/freedom-gate",
    "/reasoning",
    "/api-gateway-bridge",
    "/constructability",
    "/persistence",
];

pub fn persistence_health(
    checkpoint: crate::continuity_history::DomainHealth,
    lifelog: crate::continuity_history::DomainHealth,
) -> serde_json::Value {
    serde_json::json!({
        "schema": CSM_RUNTIME_API_PERSISTENCE_SCHEMA,
        "checkpoint_continuity": checkpoint,
        "autobiographical_lifelog": lifelog,
        "restore_authority": "checkpoint_continuity_only",
        "failure_isolation": "independent_stores_and_lifecycle"
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_api_contract_keeps_canonical_routes() {
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/status"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/chronosense"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/weather"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/shepherd"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/cav"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/curiosity"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/acip"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/acip/ws"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/freedom-gate"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/reasoning"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/api-gateway-bridge"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/constructability"));
        assert_eq!(
            CSM_RUNTIME_API_STATUS_SCHEMA,
            "adl.csm.runtime_api.status.v1"
        );
    }
}

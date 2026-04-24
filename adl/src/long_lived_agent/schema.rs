//! Schema version constants and defaults for the long-lived agent subsystem.
pub(crate) const SPEC_SCHEMA: &str = "adl.long_lived_agent_spec.v1";
pub(crate) const LEASE_SCHEMA: &str = "adl.long_lived_agent_lease.v1";
pub(crate) const STATUS_SCHEMA: &str = "adl.long_lived_agent_status.v1";
pub(crate) const STOP_SCHEMA: &str = "adl.long_lived_agent_stop.v1";
pub(crate) const CYCLE_MANIFEST_SCHEMA: &str = "adl.long_lived_agent_cycle_manifest.v1";
pub(crate) const OBSERVATIONS_SCHEMA: &str = "adl.long_lived_agent_observations.v1";
pub(crate) const DECISION_REQUEST_SCHEMA: &str = "adl.long_lived_agent_decision_request.v1";
pub(crate) const DECISION_RESULT_SCHEMA: &str = "adl.long_lived_agent_decision_result.v1";
pub(crate) const RUN_REF_SCHEMA: &str = "adl.long_lived_agent_run_ref.v1";
pub(crate) const MEMORY_WRITE_SCHEMA: &str = "adl.long_lived_agent_memory_write.v1";
pub(crate) const GUARDRAIL_REPORT_SCHEMA: &str = "adl.long_lived_agent_guardrail_report.v1";
pub(crate) const CONTINUITY_SCHEMA: &str = "adl.long_lived_agent_continuity.v1";
pub(crate) const CYCLE_LEDGER_ENTRY_SCHEMA: &str = "adl.long_lived_agent_cycle_ledger_entry.v1";
pub(crate) const PROVIDER_BINDING_SCHEMA: &str = "adl.long_lived_agent_provider_binding.v1";
pub(crate) const MEMORY_INDEX_SCHEMA: &str = "adl.long_lived_agent_memory_index.v1";
pub(crate) const OPERATOR_EVENT_SCHEMA: &str = "adl.long_lived_agent_operator_event.v1";
pub(crate) const INSPECTION_PACKET_SCHEMA: &str = "adl.long_lived_agent_inspection_packet.v1";

pub(crate) const DEFAULT_MAX_CYCLE_RUNTIME_SECS: u64 = 120;
pub(crate) const DEFAULT_MAX_CONSECUTIVE_FAILURES: u64 = 2;
pub(crate) const STOP_MODE_BEFORE_NEXT_CYCLE: &str = "stop_before_next_cycle";

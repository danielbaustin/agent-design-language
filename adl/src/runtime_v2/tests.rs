use super::*;

#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
mod a2a_adapter_boundary;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-security"))]
mod access_control;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-security"))]
mod acip_hardening;
mod aee_obsmem_pvf_trace_handoff;
mod affect_reasoning_control;
mod agent_lifecycle_state;
mod anti_harm_trajectory_constraints;
mod bid_schema;
mod boot_admission;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
mod challenge;
mod citizen_lifecycle;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
mod citizen_state_substrate;
mod codefriend_adapter_obligations;
mod cognitive_being_flagship_demo;
mod common;
mod constructability_anchor_validator;
mod contract_lifecycle_state;
mod contract_market_demo;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
mod contract_registry_accessors;
mod contract_schema;
mod csm_run_packet;
mod cultivating_intelligence;
mod curiosity_engine;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
mod delegation_subcontract;
mod economics_civilization_boundary;
mod evaluation_selection;
mod external_counterparty;
mod feature_proof_coverage;
mod freedom_gate_mediation;
mod godel_agent_runtime;
mod godel_constructability_boundary;
mod governed_episode;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
mod governed_learning_substrate;
mod governed_tools_flagship_demo;
mod guild_foundation_boundary;
mod hardening;
mod humor_and_absurdity;
mod integrated_csm_run;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
mod intelligence_metric_architecture;
mod invalid_action_rejection;
mod invariant_contract;
mod invariant_violation;
mod kernel_loop;
mod kindness_model;
mod loop_runtime;
mod manifold;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
mod memory_identity_architecture;
mod minimal_integrated_runtime_path;
mod moral_event_validation;
mod moral_metrics;
mod moral_resources;
mod moral_trace_schema;
mod moral_trajectory_review;
mod observatory;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-observatory"))]
mod observatory_flagship;
mod operator_control;
mod outcome_linkage_attribution;
mod private_state;
mod private_state_envelope;
mod private_state_equivocation;
mod private_state_lineage;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-private-state"))]
mod private_state_observatory;
mod private_state_sanctuary;
mod private_state_sealing;
mod private_state_witness;
mod quarantine;
mod reasoning_graph;
mod recovery_eligibility;
mod resource_stewardship_bridge;
mod runtime_inhabitant_integration;
mod security_boundary;
mod snapshot_rehydration;
mod standing;
mod theory_of_mind_foundation;
mod transition_authority;
mod wake_continuity;
mod wellbeing_metrics;

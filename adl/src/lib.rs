//! ADL runtime library crate (`adl`).
//!
//! This crate provides the language model (`adl`), resolution/planning (`resolve`,
//! `execution_plan`), deterministic execution (`execute`), and trust/verification
//! boundaries (`signing`, `remote_exec`) used by the canonical `adl` CLI and
//! legacy compatibility shim.
//!
//! v0.6 invariants:
//! - deterministic execution order for ready steps
//! - bounded concurrency for concurrent execution plans
//! - optional signature verification with strict enforcement on `--run`
//! - remote execution MVP where scheduling remains local

pub mod acc;
pub mod adl;
pub mod adl_gws_context_mirror;
pub mod adl_gws_drive_sync;
pub mod adl_gws_native;
pub mod adl_skill_v1;
pub mod adversarial_execution_runner;
pub mod adversarial_runtime;
pub mod agent_comms;
pub mod artifacts;
pub mod bounded_executor;
pub mod capability_aptitude_testing;
pub mod chronosense;
pub mod cognitive_transition_schema;
pub mod continuous_verification_self_attack;
pub mod control_plane;
pub mod csm_backpressure;
pub mod csm_cav;
pub mod csm_cav_red_blue;
pub mod csm_cloud_control;
pub mod csm_constructability_gate;
pub mod csm_continuity_capsule;
pub mod csm_credential_policy;
pub mod csm_curiosity_engine;
pub mod csm_freedom_gate;
pub mod csm_godel_snapshot;
pub mod csm_networking;
pub mod csm_observatory;
pub mod csm_polis_storage;
pub mod csm_resident_agents;
pub mod csm_runtime_api;
pub mod csm_shepherd_agent;
pub mod dangerous_negative_suite;
pub mod delegation_policy;
pub mod delegation_refusal_coordination;
pub mod demo;
pub mod demo_proof_entry_points;
pub mod dspark_speculative_decoding_evaluation;
pub mod execute;
pub mod execution_plan;
pub mod exploit_artifact_replay;
pub mod failure_taxonomy;
pub mod freedom_gate;
pub mod godel;
pub mod governed_executor;
pub mod gws_live_capability_execution_surface;
pub mod gws_live_content_card_roundtrip;
pub mod gws_live_safety_package;
#[cfg(test)]
pub mod gws_live_test_support;
pub mod instrumentation;
pub mod learning_export;
pub mod learning_guardrails;
pub mod local_gemma_model_evaluation;
pub mod long_lived_agent;
pub mod memory_palace;
pub mod model_identity;
pub mod model_proposal_benchmark;
pub mod observability;
pub mod obsmem_adapter;
pub mod obsmem_contract;
pub mod obsmem_demo;
pub mod obsmem_indexing;
pub mod obsmem_retrieval_policy;
pub mod obsmem_store;
pub mod obsmem_transition_memory;
pub mod operational_skills_substrate;
pub mod overlay;
pub mod plan;
pub mod policy_authority;
pub mod prompt;
pub mod provider;
pub mod provider_adapter;
pub mod provider_adapter_cli;
pub mod provider_communication;
pub mod provider_extension_packaging;
pub mod provider_native_tool_call_comparison;
pub mod provider_substrate;
pub mod red_blue_agent_architecture;
pub mod remote_exec;
pub mod resilience;
pub mod resolve;
pub mod runtime_aws_signal;
pub mod runtime_environment;
pub mod runtime_v2;
pub mod rust_native_gws_adapter_boundary;
pub mod sandbox;
pub mod scheduler;
pub mod schema;
// Session ownership remains a shared lifecycle invariant while the v1 command
// wrappers are retired; C-SDLC v2 owns the control-plane binaries.
pub mod session_ledger;
pub mod signing;
pub mod skill_composition_model;
pub mod speculative_decoding_prototype;
pub mod tool_registry;
pub mod tool_result;
pub mod trace;
pub mod trace_schema_v1;
pub mod uts;
pub mod uts_acc_compiler;
pub mod uts_acc_multi_model_benchmark;
pub mod uts_conformance;
pub mod wp08_acip_sns_proof;

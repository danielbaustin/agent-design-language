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
pub mod adversarial_execution_runner;
pub mod adversarial_runtime;
pub mod agent_comms;
pub mod artifacts;
pub mod bounded_executor;
pub mod chronosense;
pub mod continuous_verification_self_attack;
pub mod control_plane;
pub mod csm_observatory;
pub mod dangerous_negative_suite;
pub mod delegation_policy;
pub mod delegation_refusal_coordination;
pub mod demo;
pub mod demo_proof_entry_points;
pub mod execute;
pub mod execution_plan;
pub mod exploit_artifact_replay;
pub mod failure_taxonomy;
pub mod freedom_gate;
pub mod godel;
pub mod governed_executor;
pub mod instrumentation;
pub mod learning_export;
pub mod learning_guardrails;
pub mod long_lived_agent;
pub mod obsmem_adapter;
pub mod obsmem_contract;
pub mod obsmem_demo;
pub mod obsmem_indexing;
pub mod obsmem_retrieval_policy;
pub mod obsmem_store;
pub mod operational_skills_substrate;
pub mod overlay;
pub mod plan;
pub mod policy_authority;
pub mod prompt;
pub mod provider;
pub mod provider_extension_packaging;
pub mod provider_substrate;
pub mod red_blue_agent_architecture;
pub mod remote_exec;
pub mod resolve;
pub mod runtime_environment;
pub mod runtime_v2;
pub mod sandbox;
pub mod schema;
pub mod signing;
pub mod skill_composition_model;
pub mod tool_registry;
pub mod tool_result;
pub mod trace;
pub mod trace_schema_v1;
pub mod uts;
pub mod uts_acc_compiler;
pub mod uts_conformance;

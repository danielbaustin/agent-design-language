use serde::{Deserialize, Serialize};

use crate::adversarial_execution_runner::ADVERSARIAL_EXECUTION_RUNNER_SCHEMA;
use crate::continuous_verification_self_attack::CONTINUOUS_VERIFICATION_SELF_ATTACK_SCHEMA;

pub const OPERATIONAL_SKILLS_SUBSTRATE_SCHEMA: &str = "operational_skills_substrate.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillExecutionPhaseContract {
    pub phase_id: String,
    pub purpose: String,
    pub required_inputs: Vec<String>,
    pub required_outputs: Vec<String>,
    pub determinism_rules: Vec<String>,
    pub trace_events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillInvocationBoundaryContract {
    pub required_fields: Vec<String>,
    pub review_invariants: Vec<String>,
    pub forbidden_shortcuts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillExecutionContextContract {
    pub required_context_fields: Vec<String>,
    pub immutability_rules: Vec<String>,
    pub data_passing_rules: Vec<String>,
    pub artifact_reference_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompositionRuntimeMappingContract {
    pub primitive: String,
    pub runtime_obligation: String,
    pub trace_obligation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillTraceContract {
    pub composition_events: Vec<String>,
    pub invocation_events: Vec<String>,
    pub decision_events: Vec<String>,
    pub trace_fidelity_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillDeterminismContract {
    pub deterministic_surfaces: Vec<String>,
    pub stochastic_surfaces: Vec<String>,
    pub replay_modes: Vec<String>,
    pub replay_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillSecurityBoundaryContract {
    pub required_capability_fields: Vec<String>,
    pub enforcement_rules: Vec<String>,
    pub prohibited_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BoundedArxivPaperWriterSkillContract {
    pub skill_id: String,
    pub version: String,
    pub purpose: String,
    pub required_inputs: Vec<String>,
    pub lifecycle_stages: Vec<String>,
    pub output_artifacts: Vec<String>,
    pub capability_bounds: Vec<String>,
    pub source_discipline_rules: Vec<String>,
    pub prohibited_actions: Vec<String>,
    pub human_review_gates: Vec<String>,
    pub downstream_integration: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationalSkillReviewSurfaceContract {
    pub required_questions: Vec<String>,
    pub required_visibility: Vec<String>,
    pub downstream_boundaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationalSkillsSubstrateContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub runtime_condition: String,
    pub upstream_contracts: Vec<String>,
    pub execution_phases: Vec<SkillExecutionPhaseContract>,
    pub invocation_boundary: SkillInvocationBoundaryContract,
    pub execution_context: SkillExecutionContextContract,
    pub composition_runtime_mapping: Vec<CompositionRuntimeMappingContract>,
    pub trace_contract: SkillTraceContract,
    pub determinism: SkillDeterminismContract,
    pub security: SkillSecurityBoundaryContract,
    pub bounded_arxiv_paper_writer: BoundedArxivPaperWriterSkillContract,
    pub review_surface: OperationalSkillReviewSurfaceContract,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl OperationalSkillsSubstrateContract {
    pub fn v1() -> Self {
        Self {
            schema_version: OPERATIONAL_SKILLS_SUBSTRATE_SCHEMA.to_string(),
            owned_runtime_surfaces: strings(&[
                "adl::operational_skills_substrate::OperationalSkillsSubstrateContract",
                "adl::operational_skills_substrate::SkillExecutionPhaseContract",
                "adl::operational_skills_substrate::SkillInvocationBoundaryContract",
                "adl::operational_skills_substrate::SkillExecutionContextContract",
                "adl::operational_skills_substrate::BoundedArxivPaperWriterSkillContract",
                "adl identity operational-skills",
            ]),
            runtime_condition:
                "ADL operational skills must execute as explicit, bounded, traceable invocations over a deterministic composition substrate rather than as implicit prompt nesting."
                    .to_string(),
            upstream_contracts: strings(&[
                ADVERSARIAL_EXECUTION_RUNNER_SCHEMA,
                CONTINUOUS_VERIFICATION_SELF_ATTACK_SCHEMA,
            ]),
            execution_phases: vec![
                SkillExecutionPhaseContract {
                    phase_id: "plan".to_string(),
                    purpose: "load the composition graph, resolve skill definitions, and validate structure, inputs, and capabilities before execution".to_string(),
                    required_inputs: strings(&["composition_graph", "skill_definitions", "declared_capabilities"]),
                    required_outputs: strings(&["validated_graph", "resolved_skill_invocation_plan"]),
                    determinism_rules: strings(&[
                        "graph validation order is deterministic",
                        "capability denial happens before any node execution",
                    ]),
                    trace_events: strings(&["composition_plan_start", "composition_plan_end"]),
                },
                SkillExecutionPhaseContract {
                    phase_id: "bind".to_string(),
                    purpose: "bind inputs, artifact references, and immutable execution contexts to each skill invocation".to_string(),
                    required_inputs: strings(&["validated_graph", "input_payloads", "artifact_references"]),
                    required_outputs: strings(&["bound_invocation_contexts"]),
                    determinism_rules: strings(&[
                        "artifact references resolve before scheduling",
                        "context construction cannot read hidden global state",
                    ]),
                    trace_events: strings(&["invocation_bind_start", "invocation_bind_end"]),
                },
                SkillExecutionPhaseContract {
                    phase_id: "schedule".to_string(),
                    purpose: "derive the deterministic execution order and parallelizable node sets from explicit graph edges".to_string(),
                    required_inputs: strings(&["bound_invocation_contexts", "composition_edges"]),
                    required_outputs: strings(&["execution_schedule"]),
                    determinism_rules: strings(&[
                        "same graph and inputs produce the same schedule",
                        "parallel groups require explicit merge strategies",
                    ]),
                    trace_events: strings(&["schedule_decision"]),
                },
                SkillExecutionPhaseContract {
                    phase_id: "execute".to_string(),
                    purpose: "invoke skills under their declared capabilities while recording node status and controlled non-determinism".to_string(),
                    required_inputs: strings(&["execution_schedule", "bound_invocation_contexts"]),
                    required_outputs: strings(&["invocation_outputs", "invocation_statuses"]),
                    determinism_rules: strings(&[
                        "provider and tool outputs are the only stochastic surface",
                        "retry and fallback policies are declared before execution",
                    ]),
                    trace_events: strings(&["skill_invocation_start", "skill_invocation_end"]),
                },
                SkillExecutionPhaseContract {
                    phase_id: "commit".to_string(),
                    purpose: "persist outputs, trace events, and review packets after invocation boundaries are closed".to_string(),
                    required_inputs: strings(&["invocation_outputs", "invocation_statuses", "trace_buffer"]),
                    required_outputs: strings(&["composition_outputs", "review_packet", "trace_artifact"]),
                    determinism_rules: strings(&[
                        "committed artifact names and trace ordering derive from the schedule",
                        "failed or deferred nodes must commit explicit status records",
                    ]),
                    trace_events: strings(&["composition_commit_start", "composition_commit_end"]),
                },
            ],
            invocation_boundary: SkillInvocationBoundaryContract {
                required_fields: strings(&[
                    "invocation_id",
                    "skill_id",
                    "skill_version",
                    "input_payload",
                    "execution_context",
                    "allowed_capabilities",
                    "trace_correlation_id",
                    "termination_condition",
                ]),
                review_invariants: strings(&[
                    "one invocation maps to one reviewable trace span",
                    "capabilities are visible before execution",
                    "outputs are attributed to the invocation that produced them",
                    "termination reason is explicit for success, failure, skip, or defer",
                ]),
                forbidden_shortcuts: strings(&[
                    "implicit prompt nesting without invocation_id",
                    "tool use outside declared capabilities",
                    "output mutation after invocation end without a new invocation",
                    "hidden retry loops",
                ]),
            },
            execution_context: SkillExecutionContextContract {
                required_context_fields: strings(&[
                    "run_id",
                    "composition_id",
                    "invocation_id",
                    "parent_invocation_refs",
                    "artifact_root",
                    "trace_correlation_id",
                    "allowed_capabilities",
                    "replay_mode",
                ]),
                immutability_rules: strings(&[
                    "context is immutable during a single invocation",
                    "state transfer requires an explicit output edge",
                    "shared mutable process state is not a valid context source",
                ]),
                data_passing_rules: strings(&[
                    "edges carry structured payloads or artifact refs",
                    "all data transitions are trace-visible",
                    "parallel branches cannot communicate through hidden mutable memory",
                ]),
                artifact_reference_rules: strings(&[
                    "large payloads should move by immutable artifact reference",
                    "artifact refs include producer invocation and schema version",
                    "artifact reads are part of bind-phase trace",
                ]),
            },
            composition_runtime_mapping: vec![
                runtime_mapping(
                    "sequence",
                    "execute source invocation before dependent invocation",
                    "record parent-child edge and output binding",
                ),
                runtime_mapping(
                    "parallel",
                    "schedule independent invocations together and require explicit merge behavior",
                    "record parallel group, per-node status, and merge result",
                ),
                runtime_mapping(
                    "validation_gate",
                    "run validation as a separate invocation against a prior output",
                    "record pass/fail, critique, and whether downstream execution is allowed",
                ),
                runtime_mapping(
                    "conditional_branch",
                    "evaluate declared condition and select one branch",
                    "record condition input, selected branch, and skipped branches",
                ),
                runtime_mapping(
                    "fallback",
                    "invoke fallback only after declared primary failure",
                    "record primary failure definition and fallback activation",
                ),
                runtime_mapping(
                    "bounded_retry",
                    "retry within a declared maximum count and policy",
                    "record retry count, reason, and final termination",
                ),
                runtime_mapping(
                    "adjudication",
                    "compare multiple invocation outputs using declared criteria",
                    "record criteria, selected or synthesized output, and rationale",
                ),
            ],
            trace_contract: SkillTraceContract {
                composition_events: strings(&[
                    "composition_start",
                    "composition_plan",
                    "composition_schedule",
                    "composition_commit",
                    "composition_end",
                ]),
                invocation_events: strings(&[
                    "skill_invocation_bound",
                    "skill_invocation_start",
                    "skill_invocation_end",
                    "skill_invocation_deferred",
                    "skill_invocation_failed",
                ]),
                decision_events: strings(&[
                    "branch_decision",
                    "retry_decision",
                    "fallback_decision",
                    "merge_decision",
                    "adjudication_decision",
                ]),
                trace_fidelity_rules: strings(&[
                    "trace is the ground truth of execution, not a summary",
                    "every invocation and control decision creates a trace boundary",
                    "trace order must match the deterministic schedule",
                ]),
            },
            determinism: SkillDeterminismContract {
                deterministic_surfaces: strings(&[
                    "composition graph",
                    "phase order",
                    "schedule derivation",
                    "branch predicates",
                    "retry and fallback policy",
                    "artifact commit path derivation",
                ]),
                stochastic_surfaces: strings(&[
                    "model output inside a skill invocation",
                    "external tool response inside declared capabilities",
                    "provider latency and transient provider failures",
                ]),
                replay_modes: strings(&["strict_replay", "bounded_reexecution"]),
                replay_rules: strings(&[
                    "strict replay reuses recorded invocation outputs",
                    "bounded re-execution preserves graph, schedule, context, and declared capabilities",
                    "mode is selected before execution and recorded in trace",
                ]),
            },
            security: SkillSecurityBoundaryContract {
                required_capability_fields: strings(&[
                    "capability_id",
                    "tool_family",
                    "scope",
                    "mutation_authority",
                    "network_authority",
                    "artifact_write_authority",
                ]),
                enforcement_rules: strings(&[
                    "unauthorized capability use fails the invocation immediately",
                    "mutation-capable tools require explicit capability grants",
                    "network-capable tools require explicit capability grants",
                    "capability denial is recorded as a first-class runtime event",
                ]),
                prohibited_actions: strings(&[
                    "ambient tool access",
                    "hidden network access",
                    "unbounded loops",
                    "silent artifact mutation",
                    "implicit privilege escalation through nested prompts",
                ]),
            },
            bounded_arxiv_paper_writer: BoundedArxivPaperWriterSkillContract {
                skill_id: "skill.arxiv_paper_writer".to_string(),
                version: "arxiv_paper_writer.v1".to_string(),
                purpose:
                    "support bounded technical manuscript drafting from source packets while preserving claim discipline, review gates, and human publication authority"
                        .to_string(),
                required_inputs: strings(&[
                    "paper_brief",
                    "source_packet",
                    "claim_boundary",
                    "target_paper_id",
                    "citation_inventory",
                    "human_owner",
                ]),
                lifecycle_stages: strings(&[
                    "scope_packet",
                    "outline",
                    "section_draft",
                    "citation_gap_review",
                    "claim_discipline_review",
                    "review_packet",
                ]),
                output_artifacts: strings(&[
                    "manuscript_outline.md",
                    "draft_sections.md",
                    "citation_gap_report.json",
                    "claim_boundary_report.json",
                    "review_packet.json",
                ]),
                capability_bounds: strings(&[
                    "read declared source packets",
                    "write draft and review artifacts",
                    "emit citation gaps instead of inventing references",
                    "defer publication decisions to the human owner",
                ]),
                source_discipline_rules: strings(&[
                    "repo claims must cite the source packet or be marked future direction",
                    "unsupported claims move to the claim gap report",
                    "citation placeholders require explicit gap status",
                    "paper identity and authorship remain human-approved",
                ]),
                prohibited_actions: strings(&[
                    "submit_to_arxiv",
                    "invent_citations",
                    "claim_repo_facts_without_source_packet",
                    "hide_human_approval_requirement",
                    "convert planning notes into release claims without review",
                ]),
                human_review_gates: strings(&[
                    "scope approval before drafting",
                    "claim-boundary approval before manuscript packet finalization",
                    "human publication approval before any external submission",
                ]),
                downstream_integration: strings(&[
                    "WP-13 owns the three-paper manuscript status packet",
                    "D9 owns the reviewer-facing manuscript workflow proof package",
                    "this contract only defines the bounded writer skill surface",
                ]),
            },
            review_surface: OperationalSkillReviewSurfaceContract {
                required_questions: strings(&[
                    "Which skill invocation produced each artifact?",
                    "Which capabilities were granted before execution?",
                    "Which graph primitive governed each control decision?",
                    "Where is stochastic model/tool behavior isolated?",
                    "What replay mode was selected before execution?",
                ]),
                required_visibility: strings(&[
                    "phase order",
                    "invocation boundaries",
                    "capability grants",
                    "artifact references",
                    "trace correlation identifiers",
                    "explicit defers",
                ]),
                downstream_boundaries: strings(&[
                    "WP-09 owns delegation, refusal, and coordination governance follow-through",
                    "WP-13 owns the reviewer-facing manuscript/demo packet",
                    "this contract does not implement a general-purpose dynamic workflow engine",
                ]),
            },
            proof_fixture_hooks: strings(&[
                "cargo test --manifest-path adl/Cargo.toml operational_skills_substrate",
                "adl identity operational-skills --out .adl/state/operational_skills_substrate_v1.json",
            ]),
            proof_hook_command:
                "adl identity operational-skills --out .adl/state/operational_skills_substrate_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/operational_skills_substrate_v1.json".to_string(),
            scope_boundary:
                "WP-08 defines the operational substrate, composition handoff, and bounded arxiv-paper-writer skill contract; it does not ship autonomous publication, hidden delegation governance, or unbounded dynamic graph mutation."
                    .to_string(),
        }
    }
}

fn runtime_mapping(
    primitive: &str,
    runtime_obligation: &str,
    trace_obligation: &str,
) -> CompositionRuntimeMappingContract {
    CompositionRuntimeMappingContract {
        primitive: primitive.to_string(),
        runtime_obligation: runtime_obligation.to_string(),
        trace_obligation: trace_obligation.to_string(),
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operational_skill_phases_are_explicit_and_ordered() {
        let contract = OperationalSkillsSubstrateContract::v1();
        let phase_ids: Vec<&str> = contract
            .execution_phases
            .iter()
            .map(|phase| phase.phase_id.as_str())
            .collect();

        assert_eq!(
            phase_ids,
            vec!["plan", "bind", "schedule", "execute", "commit"]
        );
        assert!(contract
            .execution_phases
            .iter()
            .all(|phase| !phase.trace_events.is_empty()));
    }

    #[test]
    fn invocation_context_forbids_hidden_global_state() {
        let contract = OperationalSkillsSubstrateContract::v1();

        assert!(contract
            .invocation_boundary
            .required_fields
            .contains(&"trace_correlation_id".to_string()));
        assert!(contract
            .invocation_boundary
            .forbidden_shortcuts
            .contains(&"hidden retry loops".to_string()));
        assert!(contract
            .execution_context
            .immutability_rules
            .iter()
            .any(|rule| rule.contains("shared mutable process state")));
        assert!(contract
            .security
            .enforcement_rules
            .iter()
            .any(|rule| rule.contains("fails the invocation immediately")));
    }

    #[test]
    fn arxiv_writer_skill_is_bounded_and_claim_disciplined() {
        let contract = OperationalSkillsSubstrateContract::v1();
        let writer = &contract.bounded_arxiv_paper_writer;

        assert_eq!(writer.skill_id, "skill.arxiv_paper_writer");
        assert!(writer
            .prohibited_actions
            .contains(&"submit_to_arxiv".to_string()));
        assert!(writer
            .prohibited_actions
            .contains(&"invent_citations".to_string()));
        assert!(writer
            .source_discipline_rules
            .iter()
            .any(|rule| rule.contains("source packet")));
        assert!(writer
            .human_review_gates
            .iter()
            .any(|gate| gate.contains("publication approval")));
    }

    #[test]
    fn operational_skills_contract_has_identity_proof_hook() {
        let contract = OperationalSkillsSubstrateContract::v1();

        assert_eq!(contract.schema_version, OPERATIONAL_SKILLS_SUBSTRATE_SCHEMA);
        assert_eq!(
            contract.proof_hook_command,
            "adl identity operational-skills --out .adl/state/operational_skills_substrate_v1.json"
        );
        assert_eq!(
            contract.proof_hook_output_path,
            ".adl/state/operational_skills_substrate_v1.json"
        );
        assert!(contract
            .owned_runtime_surfaces
            .contains(&"adl identity operational-skills".to_string()));
    }
}

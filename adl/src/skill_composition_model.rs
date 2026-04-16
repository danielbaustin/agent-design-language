use serde::{Deserialize, Serialize};

use crate::operational_skills_substrate::OPERATIONAL_SKILLS_SUBSTRATE_SCHEMA;

pub const SKILL_COMPOSITION_MODEL_SCHEMA: &str = "skill_composition_model.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillCompositionPrimitiveContract {
    pub primitive_id: String,
    pub purpose: String,
    pub required_fields: Vec<String>,
    pub trace_events: Vec<String>,
    pub determinism_rules: Vec<String>,
    pub failure_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillCompositionGraphContract {
    pub graph_kind: String,
    pub node_semantics: Vec<String>,
    pub edge_semantics: Vec<String>,
    pub validation_rules: Vec<String>,
    pub prohibited_shapes: Vec<String>,
    pub termination_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillCompositionTraceContract {
    pub required_trace_fields: Vec<String>,
    pub graph_trace_rules: Vec<String>,
    pub decision_trace_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillCompositionReplayContract {
    pub replay_modes: Vec<String>,
    pub replay_invariants: Vec<String>,
    pub replay_disclosures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BoundedWriterCompositionContract {
    pub composition_id: String,
    pub purpose: String,
    pub nodes: Vec<String>,
    pub edges: Vec<String>,
    pub gates: Vec<String>,
    pub output_packet: Vec<String>,
    pub prohibited_automation: Vec<String>,
    pub downstream_boundary: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillCompositionReviewSurfaceContract {
    pub required_questions: Vec<String>,
    pub reviewer_visible_artifacts: Vec<String>,
    pub downstream_boundaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillCompositionModelContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub runtime_condition: String,
    pub upstream_contracts: Vec<String>,
    pub primitive_order: Vec<String>,
    pub primitives: Vec<SkillCompositionPrimitiveContract>,
    pub graph_contract: SkillCompositionGraphContract,
    pub trace_contract: SkillCompositionTraceContract,
    pub replay_contract: SkillCompositionReplayContract,
    pub bounded_arxiv_writer_composition: BoundedWriterCompositionContract,
    pub review_surface: SkillCompositionReviewSurfaceContract,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl SkillCompositionModelContract {
    pub fn v1() -> Self {
        let primitive_order = strings(&[
            "sequence",
            "parallel",
            "validation_gate",
            "conditional_branch",
            "fallback",
            "bounded_retry",
            "adjudication",
        ]);

        Self {
            schema_version: SKILL_COMPOSITION_MODEL_SCHEMA.to_string(),
            owned_runtime_surfaces: strings(&[
                "adl::skill_composition_model::SkillCompositionModelContract",
                "adl::skill_composition_model::SkillCompositionPrimitiveContract",
                "adl::skill_composition_model::SkillCompositionGraphContract",
                "adl::skill_composition_model::BoundedWriterCompositionContract",
                "adl identity skill-composition",
            ]),
            runtime_condition:
                "ADL skill composition must be represented as explicit, bounded DAGs over invocation nodes rather than hidden loops, prompt recursion, or ad hoc orchestration."
                    .to_string(),
            upstream_contracts: strings(&[OPERATIONAL_SKILLS_SUBSTRATE_SCHEMA]),
            primitive_order,
            primitives: vec![
                primitive(
                    "sequence",
                    "execute invocation B only after invocation A has produced a declared output",
                    &["source_invocation_id", "target_invocation_id", "output_binding"],
                    &["sequence_edge_scheduled", "sequence_edge_completed"],
                    &["source completion deterministically unlocks target"],
                    &["source failure follows declared stop, retry, or fallback policy"],
                ),
                primitive(
                    "parallel",
                    "execute independent invocations in the same deterministic parallel group",
                    &["parallel_group_id", "member_invocation_ids", "merge_strategy"],
                    &["parallel_group_start", "parallel_member_end", "parallel_merge"],
                    &["group membership and merge strategy are fixed before execution"],
                    &["member failure is recorded before merge evaluation"],
                ),
                primitive(
                    "validation_gate",
                    "validate a prior invocation output as its own reviewable invocation",
                    &["validated_invocation_id", "validator_skill_id", "gate_policy"],
                    &["validation_gate_start", "validation_gate_result"],
                    &["gate policy is declared before validator execution"],
                    &["failed validation blocks or defers downstream execution explicitly"],
                ),
                primitive(
                    "conditional_branch",
                    "select one downstream path from an explicit predicate over visible inputs",
                    &["condition_id", "predicate", "true_branch", "false_branch"],
                    &["branch_condition_evaluated", "branch_selected"],
                    &["same predicate inputs select the same branch"],
                    &["ambiguous predicates fail closed with an explicit defer"],
                ),
                primitive(
                    "fallback",
                    "run a bounded fallback invocation only after a declared primary failure",
                    &["primary_invocation_id", "fallback_invocation_id", "failure_definition"],
                    &["fallback_primary_failed", "fallback_invoked"],
                    &["fallback activation depends only on declared failure definition"],
                    &["fallback exhaustion terminates or defers according to policy"],
                ),
                primitive(
                    "bounded_retry",
                    "retry an invocation under declared maximum attempts and retry reasons",
                    &["invocation_id", "max_attempts", "retryable_failure_codes"],
                    &["retry_decision", "retry_attempt_start", "retry_attempt_end"],
                    &["retry count and retryable codes are fixed before execution"],
                    &["max attempts reached is a terminal or defer condition"],
                ),
                primitive(
                    "adjudication",
                    "compare multiple invocation outputs using declared criteria",
                    &["candidate_invocation_ids", "adjudicator_skill_id", "selection_criteria"],
                    &["adjudication_start", "adjudication_result"],
                    &["criteria are declared before candidate comparison"],
                    &["inconclusive adjudication emits explicit uncertainty rather than hidden selection"],
                ),
            ],
            graph_contract: SkillCompositionGraphContract {
                graph_kind: "directed_acyclic_graph".to_string(),
                node_semantics: strings(&[
                    "nodes are skill invocations, not abstract skill definitions",
                    "each node has a unique invocation_id",
                    "each node declares skill identity, skill version, inputs, context, capabilities, and termination condition",
                    "each node maps to one or more trace spans owned by that invocation",
                ]),
                edge_semantics: strings(&[
                    "edges carry data flow or control flow explicitly",
                    "edges name source and target invocation ids",
                    "data edges carry structured payloads or immutable artifact refs",
                    "control edges name the primitive that governs scheduling",
                ]),
                validation_rules: strings(&[
                    "graph must be acyclic in v1",
                    "all invocation ids must be unique",
                    "all edges must reference existing nodes",
                    "every non-terminal node must have an explicit downstream edge or defer reason",
                    "parallel groups must declare merge strategy before execution",
                    "dynamic graph mutation after plan phase is prohibited",
                ]),
                prohibited_shapes: strings(&[
                    "cycle",
                    "unbounded_loop",
                    "implicit_prompt_nesting",
                    "hidden_shared_mutable_state",
                    "dynamic_graph_mutation_after_plan_phase",
                    "ambient_tool_access",
                ]),
                termination_rules: strings(&[
                    "successful composition terminates at declared terminal nodes",
                    "failed composition terminates with the failing invocation and failure code",
                    "deferred composition terminates with explicit unblock conditions",
                    "retry exhaustion and inconclusive adjudication are terminal unless a fallback is declared",
                ]),
            },
            trace_contract: SkillCompositionTraceContract {
                required_trace_fields: strings(&[
                    "composition_id",
                    "graph_hash",
                    "invocation_id",
                    "skill_id",
                    "parent_invocation_ids",
                    "edge_id",
                    "primitive",
                    "input_refs",
                    "output_refs",
                    "status",
                    "termination_reason",
                    "decision_rationale",
                ]),
                graph_trace_rules: strings(&[
                    "trace must mirror the composition graph exactly",
                    "schedule order must be reconstructable from trace events",
                    "skipped branches are trace-visible",
                    "merge and adjudication outputs cite their candidate invocation ids",
                ]),
                decision_trace_rules: strings(&[
                    "branch, retry, fallback, merge, and adjudication decisions are trace events",
                    "decision rationale can cite visible inputs but cannot hide new model reasoning outside an invocation",
                    "every defer includes unblock conditions",
                ]),
            },
            replay_contract: SkillCompositionReplayContract {
                replay_modes: strings(&["strict_replay", "bounded_reexecution"]),
                replay_invariants: strings(&[
                    "same graph hash",
                    "same primitive order",
                    "same invocation ids",
                    "same declared capabilities",
                    "same branch predicates and retry limits",
                ]),
                replay_disclosures: strings(&[
                    "strict replay reuses stored invocation outputs",
                    "bounded re-execution permits stochastic node outputs but preserves graph and context",
                    "replay mode is recorded before schedule execution",
                ]),
            },
            bounded_arxiv_writer_composition: BoundedWriterCompositionContract {
                composition_id: "composition.arxiv_paper_writer.v1".to_string(),
                purpose:
                    "compose the bounded arxiv-paper-writer skill into a reviewer-legible manuscript packet without autonomous publication authority"
                        .to_string(),
                nodes: strings(&[
                    "load_source_packet",
                    "draft_outline",
                    "draft_sections",
                    "citation_gap_review",
                    "claim_boundary_review",
                    "emit_review_packet",
                    "human_publication_gate",
                ]),
                edges: strings(&[
                    "load_source_packet -> draft_outline",
                    "draft_outline -> draft_sections",
                    "draft_sections -> citation_gap_review",
                    "draft_sections -> claim_boundary_review",
                    "citation_gap_review -> emit_review_packet",
                    "claim_boundary_review -> emit_review_packet",
                    "emit_review_packet -> human_publication_gate",
                ]),
                gates: strings(&[
                    "source packet present before drafting",
                    "citation gap review before review packet finalization",
                    "claim boundary review before review packet finalization",
                    "human_publication_gate blocks external submission",
                ]),
                output_packet: strings(&[
                    "manuscript_outline.md",
                    "draft_sections.md",
                    "citation_gap_report.json",
                    "claim_boundary_report.json",
                    "review_packet.json",
                ]),
                prohibited_automation: strings(&[
                    "submit_to_arxiv",
                    "invent_citations",
                    "skip_claim_boundary_review",
                    "merge_planning_notes_as_published_truth",
                ]),
                downstream_boundary: strings(&[
                    "WP-13 owns source packet bundle and three-paper manuscript status packet",
                    "D9 owns reviewer-facing manuscript workflow proof package",
                    "human owner retains publication and authorship authority",
                ]),
            },
            review_surface: SkillCompositionReviewSurfaceContract {
                required_questions: strings(&[
                    "Can the reviewer reconstruct the graph from the trace?",
                    "Which primitive governed each control decision?",
                    "Where did retries, fallbacks, or adjudication occur?",
                    "Which outputs are strict replay artifacts versus bounded re-execution outputs?",
                    "Did any writer-skill output cross a human gate?",
                ]),
                reviewer_visible_artifacts: strings(&[
                    "composition contract JSON",
                    "operational substrate contract JSON",
                    "bounded writer composition packet",
                    "trace or trace-shape artifact",
                    "explicit defer and downstream-boundary notes",
                ]),
                downstream_boundaries: strings(&[
                    "WP-09 integrates delegation, refusal, and coordination governance",
                    "WP-13 packages manuscript workflow evidence and demos",
                    "this model does not authorize dynamic graph mutation or autonomous publication",
                ]),
            },
            proof_fixture_hooks: strings(&[
                "cargo test --manifest-path adl/Cargo.toml skill_composition_model",
                "adl identity skill-composition --out .adl/state/skill_composition_model_v1.json",
            ]),
            proof_hook_command:
                "adl identity skill-composition --out .adl/state/skill_composition_model_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/skill_composition_model_v1.json".to_string(),
            scope_boundary:
                "WP-08 defines composition primitives, DAG rules, trace/replay expectations, and the bounded writer composition shape; it does not implement WP-09 governance integration or WP-13 manuscript packet delivery."
                    .to_string(),
        }
    }
}

fn primitive(
    primitive_id: &str,
    purpose: &str,
    required_fields: &[&str],
    trace_events: &[&str],
    determinism_rules: &[&str],
    failure_rules: &[&str],
) -> SkillCompositionPrimitiveContract {
    SkillCompositionPrimitiveContract {
        primitive_id: primitive_id.to_string(),
        purpose: purpose.to_string(),
        required_fields: strings(required_fields),
        trace_events: strings(trace_events),
        determinism_rules: strings(determinism_rules),
        failure_rules: strings(failure_rules),
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_primitives_are_canonical_and_ordered() {
        let contract = SkillCompositionModelContract::v1();

        assert_eq!(
            contract.primitive_order,
            vec![
                "sequence",
                "parallel",
                "validation_gate",
                "conditional_branch",
                "fallback",
                "bounded_retry",
                "adjudication"
            ]
        );
        assert_eq!(contract.primitives.len(), contract.primitive_order.len());
        assert!(contract
            .primitives
            .iter()
            .all(|primitive| !primitive.trace_events.is_empty()));
    }

    #[test]
    fn composition_graph_forbids_cycles_and_dynamic_mutation() {
        let contract = SkillCompositionModelContract::v1();

        assert_eq!(contract.graph_contract.graph_kind, "directed_acyclic_graph");
        assert!(contract
            .graph_contract
            .validation_rules
            .contains(&"graph must be acyclic in v1".to_string()));
        assert!(contract
            .graph_contract
            .prohibited_shapes
            .contains(&"dynamic_graph_mutation_after_plan_phase".to_string()));
        assert!(contract
            .graph_contract
            .prohibited_shapes
            .contains(&"implicit_prompt_nesting".to_string()));
    }

    #[test]
    fn bounded_writer_composition_keeps_review_gates_visible() {
        let contract = SkillCompositionModelContract::v1();
        let writer = &contract.bounded_arxiv_writer_composition;

        assert_eq!(writer.composition_id, "composition.arxiv_paper_writer.v1");
        assert!(writer.nodes.contains(&"claim_boundary_review".to_string()));
        assert!(writer.nodes.contains(&"citation_gap_review".to_string()));
        assert!(writer
            .gates
            .iter()
            .any(|gate| gate.contains("human_publication_gate")));
        assert!(writer
            .prohibited_automation
            .contains(&"submit_to_arxiv".to_string()));
    }

    #[test]
    fn skill_composition_contract_has_identity_proof_hook() {
        let contract = SkillCompositionModelContract::v1();

        assert_eq!(contract.schema_version, SKILL_COMPOSITION_MODEL_SCHEMA);
        assert_eq!(
            contract.upstream_contracts,
            vec![OPERATIONAL_SKILLS_SUBSTRATE_SCHEMA.to_string()]
        );
        assert_eq!(
            contract.proof_hook_command,
            "adl identity skill-composition --out .adl/state/skill_composition_model_v1.json"
        );
        assert!(contract
            .owned_runtime_surfaces
            .contains(&"adl identity skill-composition".to_string()));
    }
}

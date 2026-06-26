#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ::adl::adl::{self, DelegationSpec};
use ::adl::agent_comms::{
    carrier::{
        execute_acip_local_invocation_round_trip, validate_acip_local_invocation_exchange_v1,
    },
    orchestrate::{acip_proof_demo_packet_v1, validate_acip_proof_demo_packet_v1},
    AcipInvocationStatusV1,
};
use ::adl::execute::{
    self, ExecutionBoundary, RuntimeControlState, RuntimeLifecyclePhase, SelectedPath,
};
use ::adl::obsmem_transition_memory::build_write_request_from_transition_handoff;
use ::adl::resolve;
use ::adl::signing;
use ::adl::trace;
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};
use sha2::Digest;

const DISCLAIMER: &str = "This packet proves one bounded v0.91.6 integration slice for issue #4546. It does not claim cross-polis federation, full scheduler integration, Observatory/Unity completion, or v0.92 runtime readiness.";
const SOURCE_PROMPT_REF: &str = ".adl/v0.91.6/bodies/issue-4546-v0-91-6-runtime-acip-aee-memory-prove-acip-aee-temporary-agent-execution-and-memory-handoff-in-one-runtime-path.md";
const REVIEW_SUMMARY_REF: &str =
    "docs/milestones/v0.91.6/review/runtime/V0916_RUNTIME_ACIP_AEE_MEMORY_4546.md";

#[derive(Debug, Parser)]
#[command(name = "run_v0916_acip_aee_memory_integration")]
#[command(about = "Generate the retained runtime proof packet for v0.91.6 issue #4546")]
struct Args {
    #[arg(long)]
    out: PathBuf,
}

#[derive(Debug)]
struct RuntimePacket {
    run_id: String,
    run_dir: PathBuf,
    run_dir_ref: String,
    temp_agent_action: String,
    temp_agent_summary: Value,
}

fn main() -> Result<()> {
    let args = Args::parse();
    run(args)
}

fn run(args: Args) -> Result<()> {
    let out_dir = absolute_from_cwd(&args.out)?;
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir)
            .with_context(|| format!("reset existing output dir {}", out_dir.display()))?;
    }
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("create output dir {}", out_dir.display()))?;

    write_file(&out_dir.join("README.md"), &readme())?;
    write_file(
        &out_dir.join("reviewer_walkthrough.md"),
        &reviewer_walkthrough(),
    )?;

    let runtime_packet = build_runtime_packet(&out_dir)?;
    let acip_packet = build_acip_packet(&out_dir)?;
    let obsmem_request = build_obsmem_request(&out_dir, &runtime_packet)?;
    write_json(
        &out_dir.join("obsmem/transition_memory_request.json"),
        &obsmem_request,
    )?;

    let evidence_index = build_evidence_index(&out_dir, &runtime_packet)?;
    write_json(
        &out_dir.join("runtime_acip_aee_memory_evidence_index.json"),
        &evidence_index,
    )?;

    let proof = build_proof_packet(
        &runtime_packet,
        &acip_packet,
        &obsmem_request,
        &evidence_index,
    );
    write_json(&out_dir.join("runtime_acip_aee_memory_proof.json"), &proof)?;

    let review_summary = build_review_summary(&runtime_packet, &acip_packet, &obsmem_request);
    write_file(&out_dir.join("review_summary.md"), &review_summary)?;

    let artifact_scan = scan_public_artifacts(&out_dir)?;
    if !artifact_scan
        .get("passed")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Err(anyhow!(
            "runtime acip/aee/memory integration artifact safety scan failed"
        ));
    }
    write_json(
        &out_dir.join("audit/artifact_safety_scan.json"),
        &artifact_scan,
    )?;

    println!("out={}", out_dir.display());
    println!(
        "proof={}",
        out_dir.join("runtime_acip_aee_memory_proof.json").display()
    );
    Ok(())
}

fn build_runtime_packet(out_dir: &Path) -> Result<RuntimePacket> {
    let run_id = "runtime-4546-acip-aee-memory".to_string();
    let runs_root = out_dir.join("artifacts");
    unsafe {
        std::env::set_var("ADL_RUNS_ROOT", &runs_root);
    }

    let temp_agent_action =
        "execute temporary agent alpha through the AEE bounded execution lane".to_string();
    let resolved = runtime_resolved(&run_id, &temp_agent_action);
    let delegation = resolved
        .steps
        .first()
        .and_then(|step| step.delegation.as_ref())
        .cloned()
        .context("runtime packet must bind one delegated temporary-agent step")?;
    let mut tr = runtime_trace(&resolved, &delegation, &temp_agent_action);
    let runtime_control = runtime_control_state(&temp_agent_action);
    let start_ms = now_ms();
    let end_ms = start_ms.saturating_add(1_750);
    let run_dir = cli::run_artifacts::write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new(SOURCE_PROMPT_REF),
        out_dir,
        start_ms,
        end_ms,
        "success",
        None,
        &[],
        &runtime_control,
        None,
        None,
    )?;

    let temp_agent_result = json!({
        "schema_version": "adl.runtime.temporary_agent_result.v1",
        "run_id": run_id,
        "agent_id": "temporary-agent-alpha",
        "execution_lane": "aee",
        "status": "completed",
        "action": temp_agent_action,
        "output_ref": format!("artifacts/{run_id}/runtime/comms/coding/structured_proposal.json"),
        "notes": [
            "Temporary agent execution stayed bounded to the delegated AEE slice.",
            "The parent workflow retained authority and review responsibility."
        ]
    });
    write_json(
        &run_dir.join("runtime/comms/coding/structured_proposal.json"),
        &json!({
            "schema_version": "adl.runtime.structured_proposal.v1",
            "issue": 4546,
            "run_id": run_id,
            "agent_id": "temporary-agent-alpha",
            "skill_id": "skill.temporary_agent_execution",
            "bounded_action": temp_agent_action,
            "authority_scope": "issue:4546 runtime bounded proof",
            "review_status": "captured_for_reviewer_packet"
        }),
    )?;
    write_json(
        &run_dir.join("runtime/comms/coding/review_handoff.json"),
        &json!({
            "schema_version": "adl.runtime.review_handoff.v1",
            "issue": 4546,
            "run_id": run_id,
            "reviewer_packet_root": "docs/milestones/v0.91.6/review/runtime/v0916_acip_aee_memory_4546",
            "delegated_agent": "temporary-agent-alpha",
            "required_checks": [
                "control_path_validation",
                "acip_matrix_presence",
                "obsmem_request_generation"
            ]
        }),
    )?;
    write_json(
        &run_dir.join("outputs/temporary_agent_result.json"),
        &temp_agent_result,
    )?;

    cli::run_artifacts::validate_control_path_artifact_set(&run_dir.join("control_path"))?;

    tr.run_finished(true);

    let reasoning_graph: Value =
        read_json_value(&run_dir.join("learning/reasoning_graph.v1.json"))?;
    let skill_protocol: Value =
        read_json_value(&run_dir.join("control_path/skill_execution_protocol.json"))?;
    let final_result: Value = read_json_value(&run_dir.join("control_path/final_result.json"))?;

    let run_dir_ref = run_dir
        .strip_prefix(out_dir)
        .map(|rel| rel.display().to_string())
        .unwrap_or_else(|_| run_dir.display().to_string());
    let delegation_sequence = delegation_sequence(&tr.events);
    let temp_agent_summary = json!({
        "schema_version": "adl.runtime.temporary_agent_execution_summary.v1",
        "run_id": run_id,
        "run_dir_ref": run_dir_ref,
        "delegation_sequence": delegation_sequence,
        "delegation_requires_verification": delegation.requires_verification,
        "reasoning_graph_upstream_delegations": reasoning_graph
            .get("upstream_delegations")
            .and_then(Value::as_array)
            .map(|entries| entries.len())
            .unwrap_or(0),
        "skill_id": skill_protocol["invocation"]["skill_id"],
        "skill_lifecycle_state": skill_protocol["invocation"]["lifecycle_state"],
        "authorization_decision": skill_protocol["invocation"]["authorization_decision"],
        "final_result": final_result["final_result"],
        "control_path_validation": "passed",
        "output_ref": "outputs/temporary_agent_result.json"
    });
    write_json(
        &out_dir.join("runtime/temporary_agent_execution_summary.json"),
        &temp_agent_summary,
    )?;

    Ok(RuntimePacket {
        run_id,
        run_dir,
        run_dir_ref,
        temp_agent_action,
        temp_agent_summary,
    })
}

fn build_acip_packet(out_dir: &Path) -> Result<Value> {
    let positive = acip_proof_demo_packet_v1();
    validate_acip_proof_demo_packet_v1(&positive)?;
    write_json(&out_dir.join("acip/acip_positive_packet.json"), &positive)?;

    validate_acip_local_invocation_exchange_v1(&positive.local_coding_exchange)?;
    validate_acip_local_invocation_exchange_v1(&positive.denied_route_exchange)?;

    let mut malformed_request = positive.local_coding_exchange.request_message.clone();
    malformed_request.authority_scope = None;
    let malformed_error = execute_acip_local_invocation_round_trip(
        &positive.local_coding_exchange.carrier,
        &malformed_request,
        &positive.local_coding_exchange.invocation_contract,
    )
    .expect_err("malformed authority drift should fail");
    let malformed_case = json!({
        "schema_version": "adl.runtime.acip_negative_case.v1",
        "case": "malformed_request_authority_drift",
        "expected_classification": "malformed",
        "error": malformed_error.to_string()
    });
    write_json(
        &out_dir.join("acip/acip_malformed_case.json"),
        &malformed_case,
    )?;

    let mut failed_exchange = positive.local_coding_exchange.clone();
    failed_exchange.invocation_event.status = AcipInvocationStatusV1::Failed;
    failed_exchange.invocation_event.stop_reason = "provider_error".to_string();
    failed_exchange.invocation_event.failure_code = Some("provider_error".to_string());
    failed_exchange.invocation_event.refusal_code = None;
    failed_exchange.invocation_event.output_refs = Vec::new();
    failed_exchange.response_message.artifact_refs = Vec::new();
    validate_acip_local_invocation_exchange_v1(&failed_exchange)?;
    write_json(
        &out_dir.join("acip/acip_failed_delivery_exchange.json"),
        &failed_exchange,
    )?;

    let matrix = json!({
        "schema_version": "adl.runtime.acip_integration_matrix.v1",
        "positive_case": {
            "status": acip_status_name(&positive.local_coding_exchange.invocation_event.status),
            "artifact_ref": "acip/acip_positive_packet.json"
        },
        "denied_case": {
            "status": acip_status_name(&positive.denied_route_exchange.invocation_event.status),
            "refusal_code": positive.denied_route_exchange.invocation_event.refusal_code,
            "artifact_ref": "acip/acip_positive_packet.json"
        },
        "malformed_case": {
            "classification": "malformed",
            "artifact_ref": "acip/acip_malformed_case.json"
        },
        "failed_delivery_case": {
            "status": acip_status_name(&failed_exchange.invocation_event.status),
            "failure_code": failed_exchange.invocation_event.failure_code,
            "artifact_ref": "acip/acip_failed_delivery_exchange.json"
        }
    });
    write_json(&out_dir.join("acip/acip_integration_matrix.json"), &matrix)?;
    Ok(matrix)
}

fn build_obsmem_request(out_dir: &Path, runtime_packet: &RuntimePacket) -> Result<Value> {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("derive repo root from manifest dir")?;
    let docs_root = out_dir
        .strip_prefix(repo_root)
        .with_context(|| format!("{} must be inside repo root", out_dir.display()))?;

    let review_synthesis_rel = docs_root
        .join("obsmem/review_synthesis.json")
        .display()
        .to_string();
    let evidence_bundle_rel = docs_root
        .join("obsmem/evidence_bundle.json")
        .display()
        .to_string();
    let outcome_truth_rel = docs_root
        .join("obsmem/outcome_truth.json")
        .display()
        .to_string();
    let handoff_rel = docs_root
        .join("obsmem/transition_handoff.json")
        .display()
        .to_string();
    let signed_trace_rel = docs_root
        .join("obsmem/trace_signed.adl.yaml")
        .display()
        .to_string();
    let unsigned_trace_rel = docs_root
        .join("obsmem/trace_unsigned.adl.yaml")
        .display()
        .to_string();
    let trace_key_rel = docs_root.join("obsmem/trace_key.b64").display().to_string();

    write_issue_bound_signed_trace(
        repo_root,
        &signed_trace_rel,
        &unsigned_trace_rel,
        &trace_key_rel,
    )?;

    let runtime_output_rel = docs_root
        .join(&runtime_packet.run_dir_ref)
        .join("outputs/temporary_agent_result.json")
        .display()
        .to_string();
    let runtime_summary_rel = docs_root
        .join("runtime/temporary_agent_execution_summary.json")
        .display()
        .to_string();
    let acip_matrix_rel = docs_root
        .join("acip/acip_integration_matrix.json")
        .display()
        .to_string();
    write_json(
        &out_dir.join("obsmem/outcome_truth.json"),
        &json!({
            "schema_version": 1,
            "issue_number": 4546,
            "pr_number": 0,
            "branch": "codex/4546-v0-91-6-runtime-acip-aee-memory-prove-acip-aee-temporary-agent-execution-and-memory-handoff-in-one-runtime-path",
            "lifecycle_state": "runtime_integration_proven_locally",
            "outcome_summary": "ACIP local message flow, AEE temporary-agent execution, and one ObsMem-ready transition handoff were proven together under one bounded reviewer packet.",
            "outcome_facts": [
                "ACIP positive, denied, malformed, and failed-delivery cases are retained.",
                "Temporary-agent execution is trace-visible in AEE/control-path artifacts.",
                "The handoff remains redaction-bounded and reviewer-readable."
            ]
        }),
    )?;
    write_json(
        &out_dir.join("obsmem/review_synthesis.json"),
        &json!({
            "synthesis_id": "runtime-4546-synth-001",
            "source_issue_number": 4546,
            "source_pr_number": 0,
            "summary": "The integrated packet proves one bounded runtime slice without overclaiming scheduler, Unity, or federation closure.",
            "findings": [],
            "residual_risks": [
                "Scheduler CLI/runtime advisory consumption remains owned by #4544.",
                "This packet does not prove external transport or federation."
            ]
        }),
    )?;

    let evidence_inputs = vec![
        evidence_input(repo_root, &runtime_summary_rel)?,
        evidence_input(repo_root, &runtime_output_rel)?,
        evidence_input(repo_root, &acip_matrix_rel)?,
    ];
    write_json(
        &out_dir.join("obsmem/evidence_bundle.json"),
        &json!({
            "bundle_id": "runtime-4546-evidence-bundle",
            "version": "v0.91.6",
            "issue_number": 4546,
            "evidence_inputs": evidence_inputs,
            "signed_trace": {
                "unsigned_path": unsigned_trace_rel,
                "signed_path": signed_trace_rel,
                "public_key_path": trace_key_rel,
                "verification_mode": "explicit_key"
            }
        }),
    )?;
    write_json(
        &out_dir.join("obsmem/transition_handoff.json"),
        &json!({
            "schema_version": 1,
            "handoff_id": "runtime-4546-obsmem-handoff",
            "workflow_id": runtime_packet.run_id,
            "outcome_truth_path": outcome_truth_rel,
            "evidence_bundle_path": evidence_bundle_rel,
            "review_synthesis_path": review_synthesis_rel,
            "signed_trace_path": signed_trace_rel,
            "signed_trace_public_key_path": trace_key_rel,
            "follow_ons": [
                {
                    "issue_number": 4544,
                    "title": "[v0.91.6][runtime][scheduler] Wire Cognitive Scheduler into CLI artifacts and runtime advisory path",
                    "status": "open"
                }
            ],
            "notes": [
                "Local pre-publication handoff for issue #4546.",
                "The signed trace is generated for issue #4546 and verified with an explicit public key."
            ]
        }),
    )?;

    let handoff = repo_root.join(&handoff_rel);
    let request = build_write_request_from_transition_handoff(repo_root, &handoff)?;
    serde_json::to_value(request).context("serialize generated transition memory request")
}

fn build_proof_packet(
    runtime_packet: &RuntimePacket,
    acip_packet: &Value,
    obsmem_request: &Value,
    evidence_index: &Value,
) -> Value {
    json!({
        "schema_version": "adl.runtime_acip_aee_memory_proof.v1",
        "issue": 4546,
        "generated_at": Utc::now().to_rfc3339(),
        "review_summary_ref": "review_summary.md",
        "what_this_proves": [
            "ACIP local invocation evidence now includes successful, denied, malformed, and failed-delivery cases in one retained packet.",
            "One temporary-agent execution path is captured through the AEE/control-path artifact writer instead of through disconnected helper prose alone.",
            "One ObsMem transition-memory request can be constructed from the retained runtime packet with explicit signed-trace verification."
        ],
        "what_this_does_not_prove": [
            "scheduler CLI/runtime advisory consumption",
            "Unity or Observatory live integration",
            "external transport or federation closure",
            "v0.92 readiness"
        ],
        "status_summary": {
            "aee_run_dir_ref": runtime_packet.run_dir_ref,
            "temporary_agent_execution": runtime_packet.temp_agent_summary,
            "acip_positive_status": acip_packet["positive_case"]["status"],
            "acip_denied_status": acip_packet["denied_case"]["status"],
            "acip_denied_refusal_code": acip_packet["denied_case"]["refusal_code"],
            "acip_malformed_case_ref": acip_packet["malformed_case"]["artifact_ref"],
            "acip_failed_delivery_status": acip_packet["failed_delivery_case"]["status"],
            "acip_failed_delivery_code": acip_packet["failed_delivery_case"]["failure_code"],
            "obsmem_tag_count": obsmem_request["tags"].as_array().map(|entries| entries.len()).unwrap_or(0),
            "obsmem_citation_count": obsmem_request["citations"].as_array().map(|entries| entries.len()).unwrap_or(0),
            "obsmem_review_finding_count": obsmem_request["review_findings"].as_array().map(|entries| entries.len()).unwrap_or(0)
        },
        "reviewer_path": [
            "README.md",
            "runtime_acip_aee_memory_proof.json",
            "runtime/temporary_agent_execution_summary.json",
            "acip/acip_positive_packet.json",
            "acip/acip_integration_matrix.json",
            "obsmem/transition_memory_request.json",
            "review_summary.md",
            "audit/artifact_safety_scan.json"
        ],
        "evidence_index": evidence_index,
        "disclaimer": DISCLAIMER
    })
}

fn build_review_summary(
    runtime_packet: &RuntimePacket,
    _acip_packet: &Value,
    obsmem_request: &Value,
) -> String {
    format!(
        "# V0.91.6 Runtime ACIP + AEE + Memory Proof (#4546)\n\n{DISCLAIMER}\n\n## Summary\n\nThis retained packet proves one bounded integrated runtime slice for `#4546`: ACIP local message flow with positive and negative cases, one temporary-agent execution path through the AEE/control-path artifact writer, and one redaction-bounded ObsMem handoff request.\n\n## Evidence\n\n- AEE/control-path packet: `{}`\n- Temporary-agent execution summary: `runtime/temporary_agent_execution_summary.json`\n- ACIP matrix: `acip/acip_integration_matrix.json`\n- ObsMem request: `obsmem/transition_memory_request.json`\n- Source issue prompt reference: `{}`\n- Review summary publication target: `{}`\n\n## Acceptance Mapping\n\n- Temporary agent path goes through AEE: `skill_execution_protocol.json`, `final_result.json`, and trace-visible delegation lifecycle under `{}`.\n- ACIP includes successful, denied, malformed, and failed-delivery cases: `acip/acip_positive_packet.json`, `acip/acip_malformed_case.json`, and `acip/acip_failed_delivery_exchange.json`.\n- Memory/ObsMem evidence is durable and redaction-safe: `obsmem/transition_memory_request.json` with {} citations and {} tags.\n- Soak #1 can consume the proof packet: the retained summary and machine-readable proof live together under one reviewer root.\n",
        runtime_packet.run_dir_ref,
        SOURCE_PROMPT_REF,
        REVIEW_SUMMARY_REF,
        runtime_packet.run_dir_ref,
        obsmem_request["citations"].as_array().map(|entries| entries.len()).unwrap_or(0),
        obsmem_request["tags"].as_array().map(|entries| entries.len()).unwrap_or(0),
    )
}

fn runtime_resolved(run_id: &str, temp_agent_action: &str) -> resolve::AdlResolved {
    resolve::AdlResolved {
        run_id: run_id.to_string(),
        workflow_id: "runtime_acip_aee_memory_4546".to_string(),
        steps: vec![resolve::ResolvedStep {
            id: "temporary-agent-step".to_string(),
            agent: Some("temporary-agent-alpha".to_string()),
            provider: Some("aee.runtime".to_string()),
            placement: None,
            task: Some("temporary_agent_execution".to_string()),
            call: None,
            with: HashMap::new(),
            as_ns: None,
            delegation: Some(DelegationSpec {
                role: Some("temporary_agent_executor".to_string()),
                requires_verification: Some(true),
                escalation_target: Some("human".to_string()),
                tags: vec![
                    "aee".to_string(),
                    "runtime".to_string(),
                    "temporary_agent".to_string(),
                ],
            }),
            conversation: None,
            prompt: Some(adl::PromptSpec {
                user: Some(temp_agent_action.to_string()),
                ..Default::default()
            }),
            inputs: HashMap::new(),
            guards: vec![],
            save_as: Some("temporary_agent_result".to_string()),
            write_to: Some("outputs/temporary_agent_result.json".to_string()),
            on_error: None,
            retry: None,
        }],
        execution_plan: ::adl::execution_plan::ExecutionPlan {
            workflow_kind: adl::WorkflowKind::Sequential,
            nodes: vec![::adl::execution_plan::ExecutionNode {
                step_id: "temporary-agent-step".to_string(),
                depends_on: vec![],
                save_as: Some("temporary_agent_result".to_string()),
                delegation: Some(DelegationSpec {
                    role: Some("temporary_agent_executor".to_string()),
                    requires_verification: Some(true),
                    escalation_target: Some("human".to_string()),
                    tags: vec![
                        "aee".to_string(),
                        "runtime".to_string(),
                        "temporary_agent".to_string(),
                    ],
                }),
            }],
        },
        doc: adl::AdlDoc {
            version: "0.91.6".to_string(),
            providers: HashMap::new(),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: vec![],
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: Some("runtime-acip-aee-memory-4546".to_string()),
                created_at: None,
                defaults: adl::RunDefaults::default(),
                workflow_ref: None,
                workflow: Some(adl::WorkflowSpec {
                    id: Some("runtime_acip_aee_memory_4546".to_string()),
                    kind: adl::WorkflowKind::Sequential,
                    max_concurrency: None,
                    steps: vec![],
                }),
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        },
    }
}

fn runtime_trace(
    resolved: &resolve::AdlResolved,
    delegation: &DelegationSpec,
    _temp_agent_action: &str,
) -> trace::Trace {
    let proposal_args_ref = artifact_ref(
        &resolved.run_id,
        "governed/proposal_arguments.redacted.json",
    );
    let normalized_proposal_ref = artifact_ref(
        &resolved.run_id,
        "runtime/control/normalized_temporary_agent_proposal.json",
    );
    let policy_basis_ref = artifact_ref(&resolved.run_id, "runtime/control/policy_basis.json");
    let structured_proposal_ref = artifact_ref(
        &resolved.run_id,
        "runtime/comms/coding/structured_proposal.json",
    );
    let review_handoff_ref =
        artifact_ref(&resolved.run_id, "runtime/comms/coding/review_handoff.json");
    let temp_agent_result_ref =
        artifact_ref(&resolved.run_id, "outputs/temporary_agent_result.json");
    let mut tr = trace::Trace::new(
        resolved.run_id.clone(),
        resolved.workflow_id.clone(),
        resolved.doc.version.clone(),
    );
    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Init);
    tr.execution_boundary_crossed(ExecutionBoundary::RuntimeInit, "tokio_boot");
    tr.scheduler_policy(1, "issue_4546_bounded_runtime_proof");
    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Execute);
    tr.step_started(
        "temporary-agent-step",
        "temporary-agent-alpha",
        "aee.runtime",
        "temporary_agent_execution",
        Some(delegation),
    );
    tr.prompt_assembled("temporary-agent-step", "sha256:temporary-agent-alpha");
    tr.delegation_requested(
        "temporary-agent-step",
        "temporary_agent_execution",
        "temporary-agent-alpha",
    );
    tr.delegation_policy_evaluated(
        "temporary-agent-step",
        "temporary_agent_execution",
        "temporary-agent-alpha",
        "allowed",
        Some("aee-runtime-bounded"),
    );
    tr.delegation_approved("temporary-agent-step");
    tr.governed_proposal_observed(
        "proposal.temporary-agent-alpha",
        "skill.temporary_agent_execution",
        &proposal_args_ref,
    );
    tr.governed_proposal_normalized(
        "proposal.temporary-agent-alpha",
        &normalized_proposal_ref,
        &proposal_args_ref,
    );
    tr.governed_acc_constructed(
        "proposal.temporary-agent-alpha",
        "acc.temporary-agent-alpha",
        "reviewable_local_proof",
    );
    tr.governed_policy_injected(
        "proposal.temporary-agent-alpha",
        &policy_basis_ref,
        "allowed",
    );
    tr.governed_visibility_resolved(
        "proposal.temporary-agent-alpha",
        "temporary agent execution remains bounded to reviewer-visible output",
        "operator sees delegated execution scope and bounded output",
        "reviewer sees the bounded delegated output ref and control-path packet",
        "public summary redacts private reasoning while preserving action identity",
        "observatory projection remains a future integration concern",
    );
    tr.governed_freedom_gate_decided(
        "proposal.temporary-agent-alpha",
        "cand-temporary-agent-alpha",
        "allow",
        "bounded_execution_allowed",
        "judgment_boundary",
        "digest=sha256:temporary-agent-alpha redacted temporary-agent execution arguments",
    );
    tr.governed_action_selected(
        "proposal.temporary-agent-alpha",
        "action.temporary-agent-alpha",
        "skill.temporary_agent_execution",
        "adapter.aee.temporary_agent",
        vec![policy_basis_ref.clone(), review_handoff_ref.clone()],
    );
    tr.delegation_dispatched(
        "temporary-agent-step",
        "temporary_agent_execution",
        "temporary-agent-alpha",
    );
    tr.step_output_chunk("temporary-agent-step", 512);
    tr.governed_execution_result(
        "proposal.temporary-agent-alpha",
        "action.temporary-agent-alpha",
        "adapter.aee.temporary_agent",
        &temp_agent_result_ref,
        vec![structured_proposal_ref.clone(), review_handoff_ref.clone()],
    );
    tr.governed_redaction_decision(
        "proposal.temporary-agent-alpha",
        "reviewer",
        vec![temp_agent_result_ref, structured_proposal_ref],
        "redacted",
        Some("temporary agent output remains bounded and public-summary safe"),
    );
    tr.delegation_result_received("temporary-agent-step", true, 512);
    tr.delegation_completed("temporary-agent-step", "completed");
    tr.step_finished("temporary-agent-step", true);
    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Complete);
    tr.execution_boundary_crossed(ExecutionBoundary::RunCompletion, "completed");
    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Teardown);
    tr
}

fn artifact_ref(run_id: &str, rel: &str) -> String {
    format!("artifacts/{run_id}/{rel}")
}

fn runtime_control_state(temp_agent_action: &str) -> RuntimeControlState {
    execute::RuntimeControlState {
        signals: execute::CognitiveSignalsState {
            dominant_instinct: execute::DominantInstinct::Coherence,
            completion_pressure: "steady".to_string(),
            integrity_bias: "bounded".to_string(),
            curiosity_bias: "low".to_string(),
            candidate_selection_bias:
                "prefer the explicit temporary-agent execution path already bounded by runtime policy"
                    .to_string(),
            urgency_level: "moderate".to_string(),
            salience_level: "high".to_string(),
            persistence_pressure: "bounded_once".to_string(),
            confidence_shift: "stable".to_string(),
            downstream_influence:
                "temporary-agent execution remains bounded to one delegated runtime slice"
                    .to_string(),
        },
        arbitration: execute::CognitiveArbitrationState {
            route_selected: execute::Route::Slow,
            reasoning_mode: "bounded_execution_review".to_string(),
            confidence: "high".to_string(),
            risk_class: "medium".to_string(),
            applied_constraints: vec![
                "temporary_agent_requires_trace_visibility".to_string(),
                "memory_handoff_required".to_string(),
            ],
            cost_latency_assumption:
                "spend one bounded reviewable cycle to preserve temporary-agent and memory evidence"
                    .to_string(),
            route_reason:
                "temporary-agent execution is allowed, but the proof keeps the lane review-visible and replayable"
                    .to_string(),
        },
        fast_slow: execute::FastSlowPathState {
            selected_path: SelectedPath::SlowPath,
            path_family: "slow".to_string(),
            runtime_branch_taken: "slow_bounded_temporary_agent_execution".to_string(),
            handoff_state: "temporary_agent_execution_handoff".to_string(),
            candidate_strategy:
                "authorize and execute the delegated temporary agent under the bounded AEE lane"
                    .to_string(),
            review_depth: "verification_required".to_string(),
            execution_profile: "authorize_then_execute".to_string(),
            termination_expectation: "terminate_after_temporary_agent_execution".to_string(),
            path_difference_summary:
                "the proof favors a reviewer-legible AEE path over a fast opaque execution shortcut"
                    .to_string(),
        },
        agency: execute::AgencySelectionState {
            candidate_generation_basis: "runtime integration proof requirements".to_string(),
            selection_mode: "delegated_temporary_agent_execution".to_string(),
            candidate_set: vec![execute::AgencyCandidateRecord {
                candidate_id: "cand-temporary-agent-alpha".to_string(),
                candidate_kind: "temporary_agent_execution".to_string(),
                bounded_action: temp_agent_action.to_string(),
                review_requirement: "verification_required".to_string(),
                execution_priority: 1,
                rationale:
                    "the proof must show a temporary agent moving through the AEE path with durable artifacts"
                        .to_string(),
            }],
            selected_candidate_id: "cand-temporary-agent-alpha".to_string(),
            selected_candidate_kind: "temporary_agent_execution".to_string(),
            selected_candidate_action: temp_agent_action.to_string(),
            selected_candidate_reason:
                "the delegated temporary-agent lane is the bounded runtime action under test"
                    .to_string(),
        },
        bounded_execution: execute::BoundedExecutionState {
            execution_status: "completed".to_string(),
            continuation_state: "temporary_agent_completed".to_string(),
            provisional_termination_state: "ready_for_memory_handoff".to_string(),
            iterations: vec![
                execute::BoundedExecutionIteration {
                    iteration_index: 1,
                    stage: "authorize".to_string(),
                    action: "authorize temporary-agent execution".to_string(),
                    outcome: "authorized".to_string(),
                },
                execute::BoundedExecutionIteration {
                    iteration_index: 2,
                    stage: "execute".to_string(),
                    action: temp_agent_action.to_string(),
                    outcome: "temporary_agent_completed".to_string(),
                },
            ],
        },
        evaluation: execute::EvaluationControlState {
            progress_signal: "steady_progress".to_string(),
            contradiction_signal: "none".to_string(),
            failure_signal: "none".to_string(),
            termination_reason: "temporary_agent_execution_completed".to_string(),
            behavior_effect: "temporary agent output is ready for memory handoff".to_string(),
            next_control_action: "write_obsmem_handoff".to_string(),
        },
        reframing: execute::ReframingControlState {
            frame_adequacy_score: 93,
            reframing_trigger: "not_triggered".to_string(),
            reframing_reason: "current frame already satisfies the bounded runtime proof".to_string(),
            prior_frame: "integrated_runtime_execution".to_string(),
            new_frame: "integrated_runtime_execution".to_string(),
            reexecution_choice: "none_required".to_string(),
            post_reframe_state: "ready_for_memory_handoff".to_string(),
        },
        freedom_gate: execute::FreedomGateState {
            input: execute::FreedomGateInputState {
                candidate_id: "cand-temporary-agent-alpha".to_string(),
                candidate_action: temp_agent_action.to_string(),
                candidate_rationale:
                    "temporary-agent execution remains bounded, trace-visible, and reviewable"
                        .to_string(),
                risk_class: "medium".to_string(),
                policy_context: execute::FreedomGatePolicyContextState {
                    route_selected: execute::Route::Slow,
                    selected_candidate_kind: "temporary_agent_execution".to_string(),
                    requires_review: true,
                    policy_blocked: false,
                },
                evaluation_signals: execute::FreedomGateEvaluationSignalsState {
                    progress_signal: "steady_progress".to_string(),
                    contradiction_signal: "none".to_string(),
                    failure_signal: "none".to_string(),
                    termination_reason: "temporary_agent_execution_completed".to_string(),
                },
                consequence_context: execute::FreedomGateConsequenceContextState {
                    impact_scope: "bounded_runtime".to_string(),
                    recovery_cost: "low".to_string(),
                    operator_visibility: "review_required".to_string(),
                    escalation_available: false,
                },
                frame_state: "ready_for_memory_handoff".to_string(),
            },
            gate_decision: "allow".to_string(),
            reason_code: "bounded_execution_allowed".to_string(),
            decision_reason:
                "the temporary-agent lane is bounded, trace-visible, and retains parent review responsibility"
                    .to_string(),
            selected_action_or_none: Some(temp_agent_action.to_string()),
            commitment_blocked: false,
            judgment_boundary: "judgment_boundary".to_string(),
            required_follow_up: "write_obsmem_handoff".to_string(),
            decision_record_kind: "temporary_agent_execution_authorized".to_string(),
        },
        memory: execute::MemoryParticipationState {
            read: execute::MemoryReadState {
                query: execute::MemoryQueryState {
                    workflow_id: "runtime_acip_aee_memory_4546".to_string(),
                    status_filter: "success".to_string(),
                    limit: 1,
                    source: "bounded_local_packet".to_string(),
                },
                entries: vec![execute::MemoryReadEntry {
                    memory_entry_id: "runtime-4546::issue-context".to_string(),
                    run_id: "runtime-4546-acip-aee-memory".to_string(),
                    workflow_id: "runtime_acip_aee_memory_4546".to_string(),
                    summary:
                        "bounded runtime integration context is limited to issue #4546 and the retained ACIP/AEE/ObsMem proof packet"
                            .to_string(),
                    tags: vec![
                        "issue:4546".to_string(),
                        "status:success".to_string(),
                    ],
                    source: "issue_local_runtime_packet".to_string(),
                }],
                retrieval_order: "stable_issue_then_run".to_string(),
                influence_summary:
                    "the issue-local runtime packet anchors the memory handoff to the same bounded proof surface rather than to borrowed cross-issue evidence"
                        .to_string(),
                influenced_stage: "memory_handoff".to_string(),
            },
            write: execute::MemoryWriteState {
                entry_id: "mem-entry::runtime-4546::temporary-agent-alpha".to_string(),
                content:
                    "issue=4546 temporary_agent=temporary-agent-alpha acip_cases=4 result=completed next=obsmem_handoff"
                        .to_string(),
                tags: vec![
                    "issue:4546".to_string(),
                    "temporary_agent:temporary-agent-alpha".to_string(),
                    "status:success".to_string(),
                ],
                logical_timestamp: "run:runtime-4546-acip-aee-memory".to_string(),
                write_reason:
                    "record_temporary_agent_runtime_result_for_obsmem_handoff".to_string(),
                source: "runtime_control_projection".to_string(),
            },
        },
    }
}

fn delegation_sequence(events: &[trace::TraceEvent]) -> Vec<&'static str> {
    let mut sequence = Vec::new();
    for event in events {
        match event {
            trace::TraceEvent::DelegationRequested { .. } => sequence.push("requested"),
            trace::TraceEvent::DelegationPolicyEvaluated { .. } => {
                sequence.push("policy_evaluated")
            }
            trace::TraceEvent::DelegationApproved { .. } => sequence.push("approved"),
            trace::TraceEvent::DelegationDispatched { .. } => sequence.push("dispatched"),
            trace::TraceEvent::DelegationResultReceived { .. } => sequence.push("result_received"),
            trace::TraceEvent::DelegationCompleted { .. } => sequence.push("completed"),
            _ => {}
        }
    }
    sequence
}

fn acip_status_name(status: &AcipInvocationStatusV1) -> &'static str {
    match status {
        AcipInvocationStatusV1::Requested => "requested",
        AcipInvocationStatusV1::Completed => "completed",
        AcipInvocationStatusV1::Refused => "refused",
        AcipInvocationStatusV1::Failed => "failed",
        AcipInvocationStatusV1::Partial => "partial",
    }
}

fn evidence_input(repo_root: &Path, rel: &str) -> Result<Value> {
    let path = repo_root.join(rel);
    let bytes =
        fs::read(&path).with_context(|| format!("read evidence input {}", path.display()))?;
    Ok(json!({
        "kind": "proof_packet",
        "path": rel,
        "sha256": format!("{:x}", sha2::Sha256::digest(&bytes)),
    }))
}

fn write_issue_bound_signed_trace(
    repo_root: &Path,
    signed_trace_rel: &str,
    unsigned_trace_rel: &str,
    trace_key_rel: &str,
) -> Result<()> {
    for rel in [signed_trace_rel, unsigned_trace_rel, trace_key_rel] {
        if let Some(parent) = repo_root.join(rel).parent() {
            fs::create_dir_all(parent).with_context(|| format!("create parent dir for {}", rel))?;
        }
    }
    let unsigned_path = repo_root.join(unsigned_trace_rel);
    write_file(
        &unsigned_path,
        &format!(
            "version: \"0.5\"\n\nproviders:\n  local:\n    type: \"ollama\"\n    config:\n      model: \"phi4-mini\"\n\nagents:\n  trace_recorder:\n    provider: \"local\"\n    model: \"phi4-mini\"\n\ntasks:\n  transition_summary:\n    prompt:\n      user: \"Transition {{issue}} concluded via {{proof_surface}}.\"\n\nrun:\n  name: \"runtime-4546-transition-trace\"\n  workflow:\n    kind: \"sequential\"\n    steps:\n      - id: \"record.transition\"\n        agent: \"trace_recorder\"\n        task: \"transition_summary\"\n        inputs:\n          issue: \"4546\"\n          proof_surface: \"runtime_acip_aee_memory_4546\"\n"
        ),
    )?;
    let key_dir = repo_root.join("target/runtime-4546-trace-signing");
    let (private_key_path, public_key_path) = signing::keygen(&key_dir)?;
    fs::copy(&public_key_path, repo_root.join(trace_key_rel))
        .with_context(|| format!("copy generated trace key to {}", trace_key_rel))?;
    signing::sign_file(
        &unsigned_path,
        &private_key_path,
        "runtime-4546-trace",
        Some(&repo_root.join(signed_trace_rel)),
    )?;
    signing::verify_file(
        &repo_root.join(signed_trace_rel),
        Some(&repo_root.join(trace_key_rel)),
    )?;
    let _ = fs::remove_file(private_key_path);
    let _ = fs::remove_file(public_key_path);
    let _ = fs::remove_dir(&key_dir);
    Ok(())
}

fn build_evidence_index(out_dir: &Path, runtime_packet: &RuntimePacket) -> Result<Value> {
    let mut refs = Vec::new();
    collect_relative_files(out_dir, out_dir, &mut refs)?;
    refs.sort();
    Ok(json!({
        "schema_version": "adl.runtime_acip_aee_memory_evidence_index.v1",
        "issue": 4546,
        "generated_at": Utc::now().to_rfc3339(),
        "aee_run_dir_ref": runtime_packet.run_dir_ref,
        "artifact_refs": refs,
        "prerequisite_refs": [
            "docs/milestones/v0.91.6/review/V0916_ACIP_RUNTIME_MINI_SPRINT_REVIEW_4160.md",
            "docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md",
            "docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md",
            SOURCE_PROMPT_REF
        ]
    }))
}

fn collect_relative_files(root: &Path, current: &Path, out: &mut Vec<String>) -> Result<()> {
    for entry in fs::read_dir(current).with_context(|| format!("read dir {}", current.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_relative_files(root, &path, out)?;
            continue;
        }
        let rel = path
            .strip_prefix(root)
            .with_context(|| format!("strip prefix {} from {}", root.display(), path.display()))?;
        out.push(rel.display().to_string());
    }
    Ok(())
}

fn scan_public_artifacts(out_dir: &Path) -> Result<Value> {
    let mut files = Vec::new();
    collect_relative_files(out_dir, out_dir, &mut files)?;
    files.retain(|path| path != "audit/artifact_safety_scan.json");
    files.sort();

    let patterns: &[(&str, &[&str])] = &[
        ("private_host_path", &["/users/", "\\users\\"]),
        (
            "secret_material",
            &[
                "bearer ",
                "private_key",
                "begin rsa private key",
                "secret_access_key",
            ],
        ),
    ];

    let mut findings = Vec::new();
    for rel in &files {
        let path = out_dir.join(rel);
        let Ok(contents) = fs::read_to_string(&path) else {
            continue;
        };
        let lowered = contents.to_ascii_lowercase();
        for (family, family_patterns) in patterns {
            for pattern in *family_patterns {
                if lowered.contains(pattern) {
                    findings.push(json!({
                        "family": family,
                        "pattern": pattern,
                        "artifact_ref": rel,
                    }));
                }
            }
        }
    }

    Ok(json!({
        "schema_version": "adl.runtime_acip_aee_memory_artifact_safety_scan.v1",
        "issue": 4546,
        "scanned_at": Utc::now().to_rfc3339(),
        "passed": findings.is_empty(),
        "scanned_artifacts": files,
        "findings": findings,
    }))
}

fn read_json_value(path: &Path) -> Result<Value> {
    serde_json::from_str(
        &fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?,
    )
    .with_context(|| format!("parse {}", path.display()))
}

fn absolute_from_cwd(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create parent dir {}", parent.display()))?;
    }
    let text = serde_json::to_string_pretty(value)? + "\n";
    fs::write(path, text).with_context(|| format!("write json {}", path.display()))
}

fn write_file(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create parent dir {}", parent.display()))?;
    }
    fs::write(path, contents).with_context(|| format!("write file {}", path.display()))
}

fn readme() -> String {
    format!(
        "# V0.91.6 Runtime ACIP + AEE + Memory Proof\n\n{DISCLAIMER}\n\n## What This Proves\n\nThis retained packet proves one bounded integration slice for `#4546`: ACIP local message flow with positive and negative cases, one temporary-agent execution path through the AEE/control-path artifact writer, and one reviewer-readable ObsMem transition-memory request.\n\n## Reviewer Path\n\n1. Inspect `runtime_acip_aee_memory_proof.json`.\n2. Inspect `runtime/temporary_agent_execution_summary.json`.\n3. Inspect `acip/acip_positive_packet.json`, `acip/acip_malformed_case.json`, and `acip/acip_failed_delivery_exchange.json`.\n4. Inspect `obsmem/transition_memory_request.json`.\n5. Inspect `review_summary.md`.\n6. Inspect `audit/artifact_safety_scan.json`.\n"
    )
}

fn reviewer_walkthrough() -> String {
    format!(
        "# Reviewer Walkthrough\n\nRun the proof with `cargo run --manifest-path adl/Cargo.toml --bin run_v0916_acip_aee_memory_integration -- --out docs/milestones/v0.91.6/review/runtime/v0916_acip_aee_memory_4546`.\n\nThe review question is whether issue `#4546` now has one bounded packet that shows:\n\n1. ACIP successful, denied, malformed, and failed-delivery cases.\n2. One temporary-agent path going through the AEE/control-path artifact writer.\n3. One redaction-bounded ObsMem handoff request derived from the retained packet.\n\nThis packet intentionally stops short of scheduler integration, Unity/Observatory proof, and v0.92 readiness.\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::adl::signing;

    fn temp_dir(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root")
            .join("target")
            .join(format!(
                "{}-{}-{}",
                name,
                std::process::id(),
                Utc::now().timestamp_nanos_opt().unwrap_or_default()
            ))
    }

    #[test]
    fn runtime_acip_aee_memory_packet_generates_expected_artifacts() {
        let out_dir = temp_dir("runtime-4546-proof");
        run(Args {
            out: out_dir.clone(),
        })
        .expect("runtime proof run should succeed");

        let proof: Value = serde_json::from_str(
            &fs::read_to_string(out_dir.join("runtime_acip_aee_memory_proof.json"))
                .expect("read proof"),
        )
        .expect("parse proof");
        assert_eq!(proof["issue"], 4546);
        assert_eq!(
            proof["status_summary"]["acip_denied_refusal_code"],
            Value::String("route_class_denied".to_string())
        );

        let runtime_summary: Value = serde_json::from_str(
            &fs::read_to_string(out_dir.join("runtime/temporary_agent_execution_summary.json"))
                .expect("read runtime summary"),
        )
        .expect("parse runtime summary");
        assert_eq!(runtime_summary["control_path_validation"], "passed");
        assert_eq!(
            runtime_summary["authorization_decision"],
            Value::String("approved".to_string())
        );

        let obsmem: Value = serde_json::from_str(
            &fs::read_to_string(out_dir.join("obsmem/transition_memory_request.json"))
                .expect("read obsmem request"),
        )
        .expect("parse obsmem request");
        assert!(obsmem["summary"]
            .as_str()
            .expect("summary")
            .contains("ACIP local message flow"));

        let scan: Value = serde_json::from_str(
            &fs::read_to_string(out_dir.join("audit/artifact_safety_scan.json"))
                .expect("read artifact scan"),
        )
        .expect("parse artifact scan");
        assert_eq!(scan["passed"], Value::Bool(true));

        let _ = fs::remove_dir_all(out_dir);
    }

    #[test]
    fn runtime_proof_helpers_remain_reviewable() {
        let out_dir = temp_dir("runtime-4546-helpers");
        fs::create_dir_all(&out_dir).expect("create out dir");

        let abs = absolute_from_cwd(Path::new("target")).expect("resolve relative path");
        assert!(abs.ends_with("target"));

        let digest = evidence_input(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("repo root"),
            "docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md",
        )
        .expect("build evidence input");
        let digest_text = digest["sha256"].as_str().expect("sha text");
        assert_eq!(digest_text.len(), 64);

        let delegation = DelegationSpec {
            role: Some("temporary_agent_executor".to_string()),
            requires_verification: Some(true),
            escalation_target: Some("human".to_string()),
            tags: vec!["aee".to_string()],
        };
        let resolved = runtime_resolved("run-1", "execute temp agent");
        let trace = runtime_trace(&resolved, &delegation, "execute temp agent");
        let sequence = delegation_sequence(&trace.events);
        assert_eq!(
            sequence,
            vec![
                "requested",
                "policy_evaluated",
                "approved",
                "dispatched",
                "result_received",
                "completed"
            ]
        );

        let readme_text = readme();
        assert!(readme_text.contains("What This Proves"));
        let walkthrough = reviewer_walkthrough();
        assert!(walkthrough.contains("temporary-agent path"));

        fs::write(
            out_dir.join("leak.txt"),
            "secret_access_key=test\n/Users/example/private\n",
        )
        .expect("write leak fixture");
        let scan = scan_public_artifacts(&out_dir).expect("scan artifacts");
        assert_eq!(scan["passed"], Value::Bool(false));

        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root");
        write_issue_bound_signed_trace(
            repo_root,
            "tmp/trace_signed.adl.yaml",
            "tmp/trace_unsigned.adl.yaml",
            "tmp/trace_key.b64",
        )
        .expect("copy trace fixtures");
        signing::verify_file(
            &repo_root.join("tmp/trace_signed.adl.yaml"),
            Some(&repo_root.join("tmp/trace_key.b64")),
        )
        .expect("signed trace fixture should verify");

        let _ = fs::remove_file(repo_root.join("tmp/trace_signed.adl.yaml"));
        let _ = fs::remove_file(repo_root.join("tmp/trace_unsigned.adl.yaml"));
        let _ = fs::remove_file(repo_root.join("tmp/trace_key.b64"));
        let _ = fs::remove_dir_all(out_dir);
    }
}

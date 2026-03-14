use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::canonical_evidence::load_canonical_evidence;
use super::experiment_record::load_canonical_record;
use super::mutation::load_canonical_mutation;
use super::workflow_template::{parse_workflow_template, GodelWorkflowTemplate};

pub const GODEL_RUNTIME_STATUS_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GodelRuntimeSurfaceStatus {
    pub status_version: u32,
    pub stage_order: Vec<String>,
    pub loaded_artifacts: Vec<String>,
    pub checks: Vec<String>,
}

pub fn load_v08_surface_status(repo_root: &Path) -> Result<GodelRuntimeSurfaceStatus> {
    let spec_examples_root = repo_root.join("adl-spec").join("examples").join("v0.8");

    let workflow_template =
        read_json(&spec_examples_root.join("godel_experiment_workflow.template.v1.json"))
            .context("load workflow template")?;
    let evidence_view = serde_json::to_value(
        load_canonical_evidence(
            &spec_examples_root.join("canonical_evidence_view.v1.example.json"),
        )
        .context("load canonical evidence example")?,
    )
    .context("serialize canonical evidence example")?;
    let mutation = serde_json::to_value(
        load_canonical_mutation(&spec_examples_root.join("mutation.v1.example.json"))
            .context("load mutation example")?,
    )
    .context("serialize mutation example")?;
    let evaluation_plan = read_json(&spec_examples_root.join("evaluation_plan.v1.example.json"))
        .context("load evaluation plan example")?;
    let canonical_record_path = spec_examples_root.join("experiment_record.v1.example.json");
    let experiment_record = serde_json::to_value(
        load_canonical_record(&canonical_record_path)
            .context("load canonical experiment record example")?,
    )
    .context("serialize canonical experiment record example")?;
    let run_summary = read_json(&spec_examples_root.join("run_summary.v1.example.json"))
        .context("load run summary example")?;
    let experiment_index =
        read_json(&spec_examples_root.join("experiment_index_entry.v1.example.json"))
            .context("load experiment index example")?;

    let template = parse_workflow_template_value(&workflow_template)?;
    let stage_order = template.stage_order.clone();
    validate_stage_order(&stage_order)?;
    validate_cross_links(
        &evidence_view,
        &mutation,
        &evaluation_plan,
        &experiment_record,
    )?;
    validate_index_and_summary(&run_summary, &experiment_index)?;

    Ok(GodelRuntimeSurfaceStatus {
        status_version: GODEL_RUNTIME_STATUS_VERSION,
        stage_order,
        loaded_artifacts: vec![
            "adl-spec/examples/v0.8/godel_experiment_workflow.template.v1.json".to_string(),
            "adl-spec/examples/v0.8/canonical_evidence_view.v1.example.json".to_string(),
            "adl-spec/examples/v0.8/mutation.v1.example.json".to_string(),
            "adl-spec/examples/v0.8/evaluation_plan.v1.example.json".to_string(),
            "adl-spec/examples/v0.8/experiment_record.v1.example.json".to_string(),
            "adl-spec/examples/v0.8/run_summary.v1.example.json".to_string(),
            "adl-spec/examples/v0.8/experiment_index_entry.v1.example.json".to_string(),
        ],
        checks: vec![
            "workflow stage order matches deterministic scientific loop".to_string(),
            "mutation/evaluation_plan/experiment_record references are consistent".to_string(),
            "run_summary and experiment_index include deterministic recovery fields".to_string(),
        ],
    })
}

pub fn repo_root_from_manifest() -> Result<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let Some(repo_root) = manifest.parent() else {
        bail!("unable to derive repository root from CARGO_MANIFEST_DIR");
    };
    Ok(repo_root.to_path_buf())
}

fn read_json(path: &Path) -> Result<Value> {
    let raw =
        fs::read_to_string(path).with_context(|| format!("failed to read '{}'", path.display()))?;
    let parsed: Value = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse '{}' as JSON", path.display()))?;
    Ok(parsed)
}

fn parse_workflow_template_value(template: &Value) -> Result<GodelWorkflowTemplate> {
    parse_workflow_template(
        &serde_json::to_string(template).context("serialize workflow template")?,
    )
    .context("parse workflow template")
}

fn validate_stage_order(stage_order: &[String]) -> Result<()> {
    let expected = [
        "failure",
        "hypothesis",
        "mutation",
        "experiment",
        "evaluation",
        "record",
    ];
    if stage_order != expected {
        bail!(
            "unexpected stage order: expected {:?}, got {:?}",
            expected,
            stage_order
        );
    }
    Ok(())
}

fn validate_cross_links(
    evidence_view: &Value,
    mutation: &Value,
    evaluation_plan: &Value,
    experiment_record: &Value,
) -> Result<()> {
    if evidence_view
        .get("schema_name")
        .and_then(Value::as_str)
        .unwrap_or_default()
        != "canonical_evidence_view"
    {
        bail!("canonical evidence example has unexpected schema_name");
    }

    let mutation_id = mutation
        .get("mutation_id")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("mutation example missing mutation_id"))?;
    if mutation_id.trim().is_empty() {
        bail!("mutation example mutation_id is empty");
    }
    let plan_id = evaluation_plan
        .get("plan_id")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("evaluation plan example missing plan_id"))?;
    if plan_id.trim().is_empty() {
        bail!("evaluation plan example plan_id is empty");
    }

    let exp_mutation_id = experiment_record
        .get("mutation")
        .and_then(|v| v.get("mutation_id"))
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("experiment record missing mutation.mutation_id"))?;
    if exp_mutation_id.trim().is_empty() {
        bail!("experiment record mutation.mutation_id is empty");
    }
    let exp_plan_id = experiment_record
        .get("evaluation_plan")
        .and_then(|v| v.get("evaluation_plan_id"))
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("experiment record missing evaluation_plan_id"))?;
    if exp_plan_id.trim().is_empty() {
        bail!("experiment record evaluation_plan_id is empty");
    }

    let mutation_plan_schema_name = mutation
        .get("evaluation_plan_ref")
        .and_then(|v| v.get("schema_name"))
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("mutation missing evaluation_plan_ref.schema_name"))?;
    if mutation_plan_schema_name != "evaluation_plan" {
        bail!("mutation evaluation_plan_ref.schema_name must be evaluation_plan");
    }

    let eval_schema_name = evaluation_plan
        .get("schema_name")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("evaluation plan example missing schema_name"))?;
    if eval_schema_name != "evaluation_plan" {
        bail!("evaluation plan example schema_name must be evaluation_plan");
    }

    Ok(())
}

fn validate_index_and_summary(run_summary: &Value, experiment_index: &Value) -> Result<()> {
    if run_summary
        .get("schema_version")
        .and_then(Value::as_str)
        .unwrap_or_default()
        != "run_summary.v1"
    {
        bail!("run summary example missing schema_version=run_summary.v1");
    }
    if experiment_index
        .get("schema_version")
        .and_then(Value::as_str)
        .unwrap_or_default()
        != "experiment_index_entry.v1"
    {
        bail!("experiment index example missing schema_version=experiment_index_entry.v1");
    }
    if experiment_index.get("improvement_delta").is_none() {
        bail!("experiment index example missing improvement_delta");
    }
    if experiment_index.get("experiment_seed").is_none() {
        bail!("experiment index example missing experiment_seed");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::path::PathBuf;

    fn unique_temp_dir(label: &str) -> PathBuf {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "adl-godel-runtime-{label}-pid{}-{n}",
            std::process::id()
        ));
        std::fs::create_dir_all(root.join("adl-spec/examples/v0.8")).expect("mkdir examples root");
        root
    }

    fn write_v08_fixtures(root: &Path) {
        let docs = root.join("adl-spec/examples/v0.8");
        std::fs::write(
            docs.join("godel_experiment_workflow.template.v1.json"),
            serde_json::to_vec_pretty(&json!({
                "template_name": "godel_experiment_workflow",
                "template_version": 1,
                "stage_order": ["failure", "hypothesis", "mutation", "experiment", "evaluation", "record"]
                ,
                "stages": [
                    {"stage_id": "failure", "inputs": [], "outputs": ["canonical_evidence_view_ref"], "artifact_contracts": [{"schema_name": "canonical_evidence_view", "schema_version": 1}]},
                    {"stage_id": "hypothesis", "inputs": ["canonical_evidence_view_ref"], "outputs": ["hypothesis_list_ref"], "artifact_contracts": []},
                    {"stage_id": "mutation", "inputs": ["hypothesis_list_ref"], "outputs": ["mutation_refs"], "artifact_contracts": [{"schema_name": "mutation", "schema_version": 1}]},
                    {"stage_id": "experiment", "inputs": ["baseline_run_ref", "mutation_refs"], "outputs": ["candidate_run_refs"], "artifact_contracts": []},
                    {"stage_id": "evaluation", "inputs": ["candidate_evidence_refs", "mutation_refs", "evaluation_plan_ref"], "outputs": ["evaluation_decision_ref"], "artifact_contracts": [{"schema_name": "evaluation_plan", "schema_version": 1}]},
                    {"stage_id": "record", "inputs": ["evaluation_decision_ref", "mutation_refs"], "outputs": ["experiment_record_ref"], "artifact_contracts": [{"schema_name": "experiment_record", "schema_version": 1}]}
                ],
                "determinism": {
                    "stage_order_fixed": true,
                    "input_order_required": true,
                    "hidden_state_allowed": false,
                    "tie_break_policy": "lexicographic_ids"
                },
                "security_privacy": {
                    "allow_secrets": false,
                    "allow_raw_prompts": false,
                    "allow_tool_args": false,
                    "allow_absolute_host_paths": false
                },
                "replay_audit": {
                    "replay_compatible": true,
                    "artifact_references_required": true,
                    "traceability_required": true
                },
                "downstream": {
                    "obsmem_indexing_ready": true,
                    "demo_path_ready": true,
                    "related_issues": [609, 610, 611, 612, 614, 615]
                }
            }))
            .expect("serialize workflow template"),
        )
        .expect("write workflow template");
        std::fs::write(
            docs.join("canonical_evidence_view.v1.example.json"),
            serde_json::to_vec_pretty(&json!({
                "schema_name": "canonical_evidence_view",
                "schema_version": 1,
                "evidence_view_id": "cev-test-fixture",
                "run_context": {
                    "run_id": "run-test-001",
                    "workflow_id": "wf-godel-loop",
                    "subject": "workflow:wf-godel-loop",
                    "variant_label": "fixture"
                },
                "canonicalization_profile": {
                    "profile_name": "godel-evidence-default",
                    "profile_version": 1,
                    "volatile_fields_excluded": ["elapsed_ms", "host_paths", "timestamps"]
                },
                "failure_codes": ["none"],
                "verification_results": [],
                "artifact_hashes": [],
                "trace_bundle_ref": "runs/run-test-001/trace_bundle_v2.json",
                "activation_log_ref": "runs/run-test-001/logs/activation_log.json",
                "comparison_axes": {
                    "primary_metric": "failure_occurrence",
                    "direction": "decrease_is_better",
                    "secondary_metrics": [
                        {
                            "metric": "evidence_ref_count",
                            "direction": "target_match"
                        }
                    ]
                },
                "privacy": {
                    "secrets_present": false,
                    "raw_prompt_or_tool_args_present": false,
                    "absolute_host_paths_present": false,
                    "redaction_notes": [
                        "prompt bodies omitted",
                        "tool argument payloads omitted"
                    ]
                },
                "derived_metrics": [
                    {
                        "metric": "evidence_ref_count",
                        "value": 1.0,
                        "unit": "count"
                    }
                ],
                "notes": ["fixture canonical evidence example"]
            }))
            .expect("serialize evidence"),
        )
        .expect("write evidence");
        std::fs::write(
            docs.join("mutation.v1.example.json"),
            serde_json::to_vec_pretty(&json!({
                "schema_name": "mutation",
                "schema_version": 1,
                "mutation_id": "mut_fixture_retry_policy_001",
                "experiment_id": "exp_fixture_retry_policy_001",
                "hypothesis_id": "hyp_fixture_retry_policy_001",
                "mutation_type": "overlay_update",
                "bounded_scope": ["overlay:run.retry_policy"],
                "operations": [
                    {
                        "op_id": "op_set_retry_max_attempts",
                        "action": "set",
                        "target_surface": "workflow_overlay",
                        "target_pointer": "/run/retry_policy/max_attempts",
                        "value": 2,
                        "expected_old_value": 1
                    }
                ],
                "constraints": {
                    "max_operations": 1,
                    "policy_gate_required": true,
                    "sandbox_required": true,
                    "allow_create_new_paths": false
                },
                "comparison": {
                    "canonical_fingerprint": "sha256:1111111111111111111111111111111111111111111111111111111111111111",
                    "ordering_key": "workflow_overlay.retry_policy.max_attempts"
                },
                "safety": {
                    "allowed_surfaces": ["evaluation_plan", "workflow_overlay"],
                    "prohibited_surfaces": [
                        "artifact_validation_strictness",
                        "security_envelope",
                        "signing_trust_policy"
                    ]
                },
                "evidence_ref": {
                    "evidence_id": "ev_fixture_retry_policy_001",
                    "schema_name": "canonical_evidence_view",
                    "schema_version": 1
                },
                "evaluation_plan_ref": {
                    "plan_id": "plan_fixture_retry_policy_001",
                    "schema_name": "evaluation_plan",
                    "schema_version": 1
                },
                "metadata": {
                    "tags": ["deterministic", "mutation", "retry-policy", "v0.8"],
                    "created_by": "godel.candidate.generator"
                }
            }))
            .expect("serialize mutation"),
        )
        .expect("write mutation");
        std::fs::write(
            docs.join("evaluation_plan.v1.example.json"),
            serde_json::to_vec_pretty(&json!({
                "schema_name": "evaluation_plan",
                "plan_id": "plan:1"
            }))
            .expect("serialize plan"),
        )
        .expect("write plan");
        std::fs::write(
            docs.join("experiment_record.v1.example.json"),
            serde_json::to_vec_pretty(&json!({
                "mutation": {"mutation_id": "mut:1"},
                "evaluation_plan": {"evaluation_plan_id": "plan:1"}
            }))
            .expect("serialize record"),
        )
        .expect("write record");
        std::fs::write(
            docs.join("run_summary.v1.example.json"),
            serde_json::to_vec_pretty(&json!({
                "schema_version": "run_summary.v1"
            }))
            .expect("serialize run summary"),
        )
        .expect("write run summary");
        std::fs::write(
            docs.join("experiment_index_entry.v1.example.json"),
            serde_json::to_vec_pretty(&json!({
                "schema_version": "experiment_index_entry.v1",
                "improvement_delta": {"metric": "success_rate"},
                "experiment_seed": "seed-1"
            }))
            .expect("serialize index"),
        )
        .expect("write index");
    }

    #[test]
    fn loads_and_validates_v08_surfaces() {
        let repo_root = repo_root_from_manifest().expect("repo root");
        let status = load_v08_surface_status(&repo_root).expect("status should load");
        assert_eq!(status.status_version, GODEL_RUNTIME_STATUS_VERSION);
        assert_eq!(
            status.stage_order,
            vec![
                "failure".to_string(),
                "hypothesis".to_string(),
                "mutation".to_string(),
                "experiment".to_string(),
                "evaluation".to_string(),
                "record".to_string(),
            ]
        );
        assert!(status
            .loaded_artifacts
            .iter()
            .all(|p| p.starts_with("adl-spec/examples/v0.8/")));
    }

    #[test]
    fn parse_workflow_template_rejects_missing_and_non_string_values() {
        let err =
            parse_workflow_template_value(&json!({})).expect_err("missing stage_order must fail");
        assert!(err.to_string().contains("parse workflow template"));

        let err = parse_workflow_template_value(&json!({
            "template_name": "godel_experiment_workflow",
            "template_version": 1,
            "stage_order": ["failure", 7],
            "stages": [],
            "determinism": {
                "stage_order_fixed": true,
                "input_order_required": true,
                "hidden_state_allowed": false,
                "tie_break_policy": "lexicographic_ids"
            },
            "security_privacy": {
                "allow_secrets": false,
                "allow_raw_prompts": false,
                "allow_tool_args": false,
                "allow_absolute_host_paths": false
            },
            "replay_audit": {
                "replay_compatible": true,
                "artifact_references_required": true,
                "traceability_required": true
            },
            "downstream": {
                "obsmem_indexing_ready": true,
                "demo_path_ready": true,
                "related_issues": [609]
            }
        }))
        .expect_err("non-string stage value must fail");
        assert!(err.to_string().contains("parse workflow template"));
    }

    #[test]
    fn validate_stage_order_rejects_non_canonical_sequence() {
        let err = validate_stage_order(&[
            "failure".to_string(),
            "mutation".to_string(),
            "hypothesis".to_string(),
            "experiment".to_string(),
            "evaluation".to_string(),
            "record".to_string(),
        ])
        .expect_err("out-of-order stages must fail");
        assert!(err.to_string().contains("unexpected stage order"));
    }

    #[test]
    fn validate_cross_links_and_summary_reject_malformed_inputs() {
        let err = validate_cross_links(
            &json!({"schema_name": "wrong"}),
            &json!({"mutation_id": "mut:1", "evaluation_plan_ref": {"schema_name": "evaluation_plan"}}),
            &json!({"schema_name": "evaluation_plan", "plan_id": "plan:1"}),
            &json!({"mutation": {"mutation_id": "mut:1"}, "evaluation_plan": {"evaluation_plan_id": "plan:1"}}),
        )
        .expect_err("unexpected evidence schema must fail");
        assert!(err.to_string().contains("unexpected schema_name"));

        let err = validate_index_and_summary(
            &json!({"schema_version": "run_summary.v1"}),
            &json!({"schema_version": "experiment_index_entry.v1", "improvement_delta": {}}),
        )
        .expect_err("missing experiment_seed must fail");
        assert!(err.to_string().contains("missing experiment_seed"));
    }

    #[test]
    fn validate_cross_links_rejects_empty_ids_and_schema_mismatches() {
        let evidence = json!({"schema_name": "canonical_evidence_view"});
        let base_mutation = json!({"mutation_id": "mut:1", "evaluation_plan_ref": {"schema_name": "evaluation_plan"}});
        let base_plan = json!({"schema_name": "evaluation_plan", "plan_id": "plan:1"});
        let base_record = json!({"mutation": {"mutation_id": "mut:1"}, "evaluation_plan": {"evaluation_plan_id": "plan:1"}});

        let err = validate_cross_links(
            &evidence,
            &json!({"mutation_id": " ", "evaluation_plan_ref": {"schema_name": "evaluation_plan"}}),
            &base_plan,
            &base_record,
        )
        .expect_err("empty mutation_id must fail");
        assert!(err.to_string().contains("mutation_id is empty"));

        let err = validate_cross_links(
            &evidence,
            &base_mutation,
            &json!({"schema_name": "evaluation_plan", "plan_id": " "}),
            &base_record,
        )
        .expect_err("empty plan_id must fail");
        assert!(err.to_string().contains("plan_id is empty"));

        let err = validate_cross_links(
            &evidence,
            &base_mutation,
            &base_plan,
            &json!({"mutation": {"mutation_id": " "}, "evaluation_plan": {"evaluation_plan_id": "plan:1"}}),
        )
        .expect_err("empty experiment record mutation id must fail");
        assert!(err.to_string().contains("mutation.mutation_id is empty"));

        let err = validate_cross_links(
            &evidence,
            &base_mutation,
            &base_plan,
            &json!({"mutation": {"mutation_id": "mut:1"}, "evaluation_plan": {"evaluation_plan_id": " "}}),
        )
        .expect_err("empty experiment record plan id must fail");
        assert!(err.to_string().contains("evaluation_plan_id is empty"));

        let err = validate_cross_links(
            &evidence,
            &json!({"mutation_id": "mut:1", "evaluation_plan_ref": {"schema_name": "wrong"}}),
            &base_plan,
            &base_record,
        )
        .expect_err("mutation schema mismatch must fail");
        assert!(err.to_string().contains("must be evaluation_plan"));

        let err = validate_cross_links(
            &evidence,
            &base_mutation,
            &json!({"schema_name": "wrong", "plan_id": "plan:1"}),
            &base_record,
        )
        .expect_err("evaluation schema mismatch must fail");
        assert!(err
            .to_string()
            .contains("schema_name must be evaluation_plan"));
    }

    #[test]
    fn validate_index_and_summary_rejects_schema_mismatches_and_missing_delta() {
        let err = validate_index_and_summary(
            &json!({"schema_version": "wrong"}),
            &json!({"schema_version": "experiment_index_entry.v1", "improvement_delta": {}, "experiment_seed": "seed-1"}),
        )
        .expect_err("run summary schema mismatch must fail");
        assert!(err.to_string().contains("schema_version=run_summary.v1"));

        let err = validate_index_and_summary(
            &json!({"schema_version": "run_summary.v1"}),
            &json!({"schema_version": "wrong", "improvement_delta": {}, "experiment_seed": "seed-1"}),
        )
        .expect_err("index schema mismatch must fail");
        assert!(err
            .to_string()
            .contains("schema_version=experiment_index_entry.v1"));

        let err = validate_index_and_summary(
            &json!({"schema_version": "run_summary.v1"}),
            &json!({"schema_version": "experiment_index_entry.v1", "experiment_seed": "seed-1"}),
        )
        .expect_err("missing improvement_delta must fail");
        assert!(err.to_string().contains("missing improvement_delta"));
    }

    #[test]
    fn load_v08_surface_status_errors_when_fixture_is_missing() {
        let root = unique_temp_dir("missing-fixtures");
        write_v08_fixtures(&root);
        std::fs::remove_file(
            root.join("adl-spec/examples/v0.8")
                .join("evaluation_plan.v1.example.json"),
        )
        .expect("remove plan fixture");

        let err = load_v08_surface_status(&root).expect_err("missing fixture should fail");
        assert!(err.to_string().contains("load evaluation plan example"));
        let _ = std::fs::remove_dir_all(&root);
    }
}

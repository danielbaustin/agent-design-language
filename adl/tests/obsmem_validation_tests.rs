use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ::adl::obsmem_contract::{MemoryCitation, MemoryQueryResult, MemoryRecord};
use ::adl::obsmem_indexing::index_run_from_artifacts;
use ::adl::obsmem_retrieval_policy::{
    apply_policy_to_results, RetrievalOrder, RetrievalPolicyV1, RetrievalRequest,
};

mod helpers;
use helpers::unique_test_temp_dir;

fn fixture_path(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn runs_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .join(".adl")
        .join("runs")
}

fn write_fixture_run(root: &Path, run_id: &str, failure_kind: Option<&str>, tags_hint: &str) {
    let run = root.join(run_id);
    fs::create_dir_all(run.join("logs")).expect("mkdir logs");
    fs::write(
        run.join("run_summary.json"),
        format!(
            r#"{{"run_summary_version":1,"run_id":"{run_id}","workflow_id":"wf-obsmem-{tags_hint}"}}"#
        ),
    )
    .expect("write run_summary");

    let failure_json = failure_kind
        .map(|v| format!("\"{v}\""))
        .unwrap_or_else(|| "null".to_string());
    fs::write(
        run.join("run_status.json"),
        format!(
            r#"{{"run_status_version":1,"run_id":"{run_id}","overall_status":"success","failure_kind":{failure_json}}}"#
        ),
    )
    .expect("write run_status");

    let activation = serde_json::json!({
        "activation_log_version": 1,
        "ordering": "append_only_emission_order",
        "stable_ids": {
            "step_id": "stable within resolved execution plan",
            "delegation_id": "deterministic per run: del-<counter>",
            "run_id": "run-scoped identifier; not replay-stable across independent runs",
        },
        "events": [
            {
                "kind": "StepStarted",
                "step_id": "s1",
                "agent_id": "a",
                "provider_id": "local",
                "task_id": "t",
                "delegation_json": null
            },
            {
                "kind": "PromptAssembled",
                "step_id": "s1",
                "prompt_hash": "abc123"
            },
            {
                "kind": "StepFinished",
                "step_id": "s1",
                "success": true
            }
        ]
    });
    fs::write(
        run.join("logs").join("activation_log.json"),
        serde_json::to_vec_pretty(&activation).expect("serialize activation"),
    )
    .expect("write activation");
}

fn make_record(id: &str, score: &str, tags: &[&str], workflow_id: &str) -> MemoryRecord {
    let mut t: Vec<String> = tags.iter().map(|s| (*s).to_string()).collect();
    t.sort();
    t.dedup();
    MemoryRecord {
        id: id.to_string(),
        run_id: format!("run-{id}"),
        workflow_id: workflow_id.to_string(),
        tags: t,
        payload: format!("payload-{id}"),
        score: score.to_string(),
        citations: vec![MemoryCitation {
            path: format!("runs/{id}/run_summary.json"),
            hash: "det64:0000000000000001".to_string(),
        }],
    }
}

#[test]
fn indexing_pipeline_produces_deterministic_entries_with_stable_sequence() {
    let tmp = unique_test_temp_dir("obsmem-index");
    write_fixture_run(&tmp, "run-a", Some("tool_failure"), "a");

    let left = index_run_from_artifacts(&tmp, "run-a").expect("left index");
    let right = index_run_from_artifacts(&tmp, "run-a").expect("right index");

    assert_eq!(left, right);
    assert!(!left.workflow_id.trim().is_empty());
    assert!(!left.summary.trim().is_empty());
    assert!(left.tags.binary_search(&"run:run-a".to_string()).is_ok());
    assert!(left
        .steps
        .iter()
        .enumerate()
        .all(|(idx, step)| step.sequence == idx));
}

#[test]
fn retrieval_determinism_returns_identical_result_set_and_order() {
    let policy = RetrievalPolicyV1 {
        default_limit: 10,
        required_tags: vec!["workflow:wf-a".to_string()],
        required_failure_code: Some("tool_failure".to_string()),
        order: RetrievalOrder::ScoreDescIdAsc,
    };
    let request = RetrievalRequest {
        workflow_id: Some("wf-a".to_string()),
        failure_code: None,
        tags: vec![],
        limit_override: Some(10),
    };

    let input = MemoryQueryResult {
        hits: vec![
            make_record(
                "b",
                "1.20",
                &["workflow:wf-a", "failure:tool_failure"],
                "wf-a",
            ),
            make_record(
                "a",
                "1.20",
                &["workflow:wf-a", "failure:tool_failure"],
                "wf-a",
            ),
            make_record(
                "z",
                "0.01",
                &["workflow:wf-b", "failure:tool_failure"],
                "wf-b",
            ),
        ],
    };

    let first = apply_policy_to_results(&policy, &request, input.clone()).expect("first");
    let second = apply_policy_to_results(&policy, &request, input).expect("second");

    assert_eq!(first, second);
    let ids: Vec<String> = first.hits.iter().map(|h| h.id.clone()).collect();
    assert_eq!(ids, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn policy_filtering_keeps_only_matching_tags_and_failure_codes() {
    let policy = RetrievalPolicyV1 {
        default_limit: 10,
        required_tags: vec!["workflow:wf-a".to_string(), "status:success".to_string()],
        required_failure_code: Some("tool_failure".to_string()),
        order: RetrievalOrder::IdAsc,
    };
    let request = RetrievalRequest {
        workflow_id: Some("wf-a".to_string()),
        failure_code: None,
        tags: vec![],
        limit_override: None,
    };

    let input = MemoryQueryResult {
        hits: vec![
            make_record(
                "keep",
                "1.00",
                &["workflow:wf-a", "status:success", "failure:tool_failure"],
                "wf-a",
            ),
            make_record(
                "drop-tag",
                "2.00",
                &["workflow:wf-a", "failure:tool_failure"],
                "wf-a",
            ),
            make_record(
                "drop-failure",
                "3.00",
                &["workflow:wf-a", "status:success", "failure:runtime_failure"],
                "wf-a",
            ),
        ],
    };

    let filtered = apply_policy_to_results(&policy, &request, input).expect("filtered");
    assert_eq!(filtered.hits.len(), 1);
    assert_eq!(filtered.hits[0].id, "keep");
}

#[test]
fn end_to_end_hierarchical_demo_emits_obsmem_artifacts() {
    let run_id = format!("v075-obsmem-e2e-pid{}", std::process::id());
    let tmp = unique_test_temp_dir("obsmem-e2e");
    let yaml_src = fixture_path("examples/v0-7-hierarchical-planner.adl.yaml");
    let yaml = fs::read_to_string(&yaml_src).expect("read example");
    let patched = yaml.replace(
        "name: \"v0-7-hierarchical-planner\"",
        &format!("name: \"{run_id}\""),
    );
    let yaml_path = tmp.join("hierarchical-obsmem-e2e.adl.yaml");
    fs::write(&yaml_path, patched).expect("write patched yaml");

    let out_dir = tmp.join("out");
    let mock = fixture_path("tools/mock_ollama_v0_4.sh");
    let exe = env!("CARGO_BIN_EXE_adl");
    let output = Command::new(exe)
        .arg(yaml_path.to_str().expect("yaml path utf8"))
        .arg("--run")
        .arg("--trace")
        .arg("--allow-unsigned")
        .arg("--out")
        .arg(out_dir.to_str().expect("out path utf8"))
        .env("ADL_OLLAMA_BIN", mock)
        .env("ADL_OBSMEM_DEMO", "1")
        .output()
        .expect("run adl");

    assert!(
        output.status.success(),
        "stdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let learning_dir = runs_root().join(&run_id).join("learning");
    let index_path = learning_dir.join("obs_mem_index_summary.json");
    let query_path = learning_dir.join("obs_mem_query_result.json");
    assert!(
        index_path.is_file(),
        "missing index artifact: {}",
        index_path.display()
    );
    assert!(
        query_path.is_file(),
        "missing query artifact: {}",
        query_path.display()
    );

    let query_text = fs::read_to_string(&query_path).expect("read query artifact");
    let query_json: serde_json::Value =
        serde_json::from_str(&query_text).expect("parse query artifact json");

    assert_eq!(
        query_json
            .get("ordering")
            .and_then(|o| o.get("policy_order"))
            .and_then(serde_json::Value::as_str),
        Some("evidence_adjusted_desc_id_asc")
    );
    assert!(
        query_json
            .get("query")
            .and_then(|q| q.get("tags"))
            .is_some(),
        "query tags missing"
    );
    assert!(
        query_json
            .get("entries")
            .and_then(serde_json::Value::as_array)
            .is_some(),
        "entries missing"
    );

    for forbidden in ["/Users/", "/home/", "gho_", "sk-"] {
        assert!(
            !query_text.contains(forbidden),
            "query artifact leaked forbidden token '{forbidden}':\n{query_text}"
        );
    }
}

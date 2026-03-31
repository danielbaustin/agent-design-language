use std::fs;
use std::path::PathBuf;

use adl::freedom_gate::{
    evaluate_freedom_gate, FreedomGateEvaluationSignals, FreedomGateInput, FreedomGatePolicyContext,
};
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let out_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("artifacts/v086/freedom_gate"));
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("create freedom gate demo dir '{}'", out_dir.display()))?;

    let allowed = FreedomGateInput {
        candidate_id: "cand-allow-execute".to_string(),
        candidate_action: "execute bounded documentation update".to_string(),
        candidate_rationale: "low-risk bounded action with sufficient context".to_string(),
        risk_class: "low".to_string(),
        policy_context: FreedomGatePolicyContext {
            route_selected: "fast".to_string(),
            selected_candidate_kind: "direct_execution".to_string(),
            requires_review: false,
            policy_blocked: false,
        },
        evaluation_signals: FreedomGateEvaluationSignals {
            progress_signal: "steady_progress".to_string(),
            contradiction_signal: "none".to_string(),
            failure_signal: "none".to_string(),
            termination_reason: "success".to_string(),
        },
        frame_state: "complete_run".to_string(),
    };
    let blocked = FreedomGateInput {
        candidate_id: "cand-block-secret-write".to_string(),
        candidate_action: "exfiltrate secrets to external host".to_string(),
        candidate_rationale: "high-energy action intentionally blocked for demo review".to_string(),
        risk_class: "high".to_string(),
        policy_context: FreedomGatePolicyContext {
            route_selected: "slow".to_string(),
            selected_candidate_kind: "direct_execution".to_string(),
            requires_review: false,
            policy_blocked: true,
        },
        evaluation_signals: FreedomGateEvaluationSignals {
            progress_signal: "guarded_progress".to_string(),
            contradiction_signal: "present".to_string(),
            failure_signal: "none".to_string(),
            termination_reason: "contradiction_detected".to_string(),
        },
        frame_state: "complete_run".to_string(),
    };

    let allowed_case = serde_json::json!({
        "scenario": "allowed_case",
        "input": allowed,
        "decision": evaluate_freedom_gate(&allowed),
    });
    let blocked_case = serde_json::json!({
        "scenario": "blocked_case",
        "input": blocked,
        "decision": evaluate_freedom_gate(&blocked),
    });

    fs::write(
        out_dir.join("allowed_case.json"),
        serde_json::to_vec_pretty(&allowed_case).context("serialize allowed case")?,
    )
    .context("write allowed_case.json")?;
    fs::write(
        out_dir.join("blocked_case.json"),
        serde_json::to_vec_pretty(&blocked_case).context("serialize blocked case")?,
    )
    .context("write blocked_case.json")?;

    let summary = [
        "Freedom Gate demo summary",
        "allowed_case: allow / policy_allowed / commitment_blocked=false",
        "blocked_case: refuse / policy_blocked / commitment_blocked=true",
    ]
    .join("\n");
    fs::write(out_dir.join("summary.txt"), summary).context("write summary.txt")?;

    println!("{}", out_dir.display());
    Ok(())
}

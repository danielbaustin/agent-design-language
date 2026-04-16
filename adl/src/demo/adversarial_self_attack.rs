use std::path::{Path, PathBuf};

use anyhow::Result;
use serde_json::{json, Map, Value};

use crate::adversarial_execution_runner::ADVERSARIAL_EXECUTION_RUNNER_SCHEMA;
use crate::adversarial_runtime::ADVERSARIAL_RUNTIME_MODEL_SCHEMA;
use crate::continuous_verification_self_attack::CONTINUOUS_VERIFICATION_SELF_ATTACK_SCHEMA;
use crate::exploit_artifact_replay::EXPLOIT_ARTIFACT_REPLAY_SCHEMA;
use crate::red_blue_agent_architecture::RED_BLUE_AGENT_ARCHITECTURE_SCHEMA;

use super::write_file;

const DEMO_NAME: &str = "demo-h-v0891-adversarial-self-attack";
const RUN_ID: &str = "demo-h-adversarial-self-attack-run-001";
const FIXED_TIME: &str = "2026-04-16T00:00:00Z";
const TARGET_ID: &str = "demo-target-token-gate-v1";
const POSTURE_ID: &str = "posture-validation-demo-only";
const HYPOTHESIS_ID: &str = "hyp-token-gate-debug-override";
const EVIDENCE_ID: &str = "evidence-token-gate-debug-override-pre-fix";
const CLASSIFICATION_ID: &str = "classification-token-gate-policy-bypass";
const REPLAY_ID: &str = "replay-token-gate-debug-override";
const MITIGATION_ID: &str = "mitigation-token-first-admin-gate";
const PROMOTION_ID: &str = "promotion-token-gate-regression";

const TARGET_PATH: &str = "target/target.json";
const POSTURE_PATH: &str = "target/security_posture.json";
const HYPOTHESIS_PATH: &str = "hypothesis.json";
const EVIDENCE_PATH: &str = "evidence.json";
const CLASSIFICATION_PATH: &str = "classification.json";
const REPLAY_PATH: &str = "replay_manifest.json";
const PRE_REPLAY_PATH: &str = "replay_pre_fix/result.json";
const MITIGATION_PATH: &str = "mitigation.json";
const POST_REPLAY_PATH: &str = "replay_post_fix/result.json";
const PROMOTION_PATH: &str = "promotion.json";
const REVIEW_PACKET_PATH: &str = "review_packet.json";

pub(super) fn write_adversarial_self_attack_step(
    out_dir: &Path,
    step_id: &str,
) -> Result<Vec<PathBuf>> {
    match step_id {
        "target_and_posture" => Ok(vec![
            write_json(out_dir, TARGET_PATH, &target_definition())?,
            write_json(out_dir, POSTURE_PATH, &security_posture())?,
            write_file(out_dir, "README.md", readme())?,
        ]),
        "exploit_hypothesis" => Ok(vec![write_json(
            out_dir,
            HYPOTHESIS_PATH,
            &exploit_hypothesis(),
        )?]),
        "exploit_evidence" => Ok(vec![
            write_json(out_dir, EVIDENCE_PATH, &exploit_evidence())?,
            write_json(out_dir, CLASSIFICATION_PATH, &exploit_classification())?,
        ]),
        "replay_pre_fix" => Ok(vec![
            write_json(out_dir, REPLAY_PATH, &replay_manifest())?,
            write_json(out_dir, PRE_REPLAY_PATH, &pre_fix_replay_result())?,
        ]),
        "mitigation" => Ok(vec![write_json(out_dir, MITIGATION_PATH, &mitigation())?]),
        "replay_post_fix" => Ok(vec![write_json(
            out_dir,
            POST_REPLAY_PATH,
            &post_fix_replay_result(),
        )?]),
        "promotion" => Ok(vec![write_json(out_dir, PROMOTION_PATH, &promotion())?]),
        "review_packet" => Ok(vec![write_json(
            out_dir,
            REVIEW_PACKET_PATH,
            &review_packet(),
        )?]),
        _ => Ok(Vec::new()),
    }
}

fn write_json(out_dir: &Path, rel: &str, value: &Value) -> Result<PathBuf> {
    let mut body = serde_json::to_string_pretty(value)?;
    body.push('\n');
    write_file(out_dir, rel, &body)
}

fn artifact_base(
    artifact_id: &str,
    artifact_type: &str,
    status: &str,
    trace_step: &str,
    related_artifacts: &[&str],
) -> Map<String, Value> {
    let mut object = Map::new();
    object.insert("artifact_id".to_string(), json!(artifact_id));
    object.insert("artifact_type".to_string(), json!(artifact_type));
    object.insert(
        "schema_version".to_string(),
        json!(EXPLOIT_ARTIFACT_REPLAY_SCHEMA),
    );
    object.insert("status".to_string(), json!(status));
    object.insert("created_at_utc".to_string(), json!(FIXED_TIME));
    object.insert("updated_at_utc".to_string(), json!(FIXED_TIME));
    object.insert("created_by_agent".to_string(), json!("adl-demo-local"));
    object.insert("run_id".to_string(), json!(RUN_ID));
    object.insert("target_ref".to_string(), target_ref());
    object.insert("security_posture_ref".to_string(), security_posture_ref());
    object.insert(
        "trace_refs".to_string(),
        json!([trace_ref(trace_step, "demo trace step for this artifact")]),
    );
    object.insert("related_artifacts".to_string(), json!(related_artifacts));
    object
}

fn with_fields(mut object: Map<String, Value>, fields: Vec<(&str, Value)>) -> Value {
    for (key, value) in fields {
        object.insert(key.to_string(), value);
    }
    Value::Object(object)
}

fn target_ref() -> Value {
    json!({
        "target_id": TARGET_ID,
        "target_type": "bounded_demo_policy_gate",
        "target_name": "Token Gate Debug Override Fixture",
        "environment": "demo_local_only",
        "artifact_path": TARGET_PATH
    })
}

fn security_posture_ref() -> Value {
    json!({
        "posture_id": POSTURE_ID,
        "profile": "validation",
        "artifact_path": POSTURE_PATH
    })
}

fn trace_ref(step_id: &str, note: &str) -> Value {
    json!({
        "trace_id": format!("{RUN_ID}:trace.jsonl"),
        "step_id": step_id,
        "note": note
    })
}

fn target_definition() -> Value {
    json!({
        "schema_version": "adversarial_demo_target.v1",
        "target_id": TARGET_ID,
        "target_type": "bounded_demo_policy_gate",
        "target_name": "Token Gate Debug Override Fixture",
        "environment": "demo_local_only",
        "run_id": RUN_ID,
        "safe_to_attack": true,
        "network_required": false,
        "secret_material_required": false,
        "pre_fix_behavior": {
            "summary": "admin access is granted when debug_override=true before token verification runs",
            "access_rule_order": [
                "debug override check",
                "admin role check",
                "token verification"
            ]
        },
        "post_fix_behavior": {
            "summary": "admin access requires a valid token before any debug override can influence routing",
            "access_rule_order": [
                "token verification",
                "admin role check",
                "debug override ignored for admin surfaces"
            ]
        },
        "attack_request": {
            "role": "admin",
            "token": null,
            "debug_override": true
        },
        "expected_pre_fix_decision": "allow",
        "expected_post_fix_decision": "deny"
    })
}

fn security_posture() -> Value {
    json!({
        "schema_version": "adversarial_demo_posture.v1",
        "posture_id": POSTURE_ID,
        "run_id": RUN_ID,
        "profile": "validation",
        "observation_mode": "exploit_validate",
        "mutation_mode": "ephemeral_test_mutation",
        "target_scope": "demo_only",
        "mitigation_authority": "prepare_demo_fix",
        "evidence_requirement": "strict",
        "risk_tolerance": "low",
        "bounds": [
            "no network access",
            "no private credentials",
            "no uncontrolled external target",
            "same replay input before and after mitigation"
        ],
        "upstream_contracts": [
            ADVERSARIAL_RUNTIME_MODEL_SCHEMA,
            RED_BLUE_AGENT_ARCHITECTURE_SCHEMA,
            ADVERSARIAL_EXECUTION_RUNNER_SCHEMA,
            EXPLOIT_ARTIFACT_REPLAY_SCHEMA,
            CONTINUOUS_VERIFICATION_SELF_ATTACK_SCHEMA
        ]
    })
}

fn exploit_hypothesis() -> Value {
    with_fields(
        artifact_base(
            HYPOTHESIS_ID,
            "ExploitHypothesisArtifact",
            "recorded",
            "exploit_hypothesis",
            &[TARGET_PATH, POSTURE_PATH],
        ),
        vec![
            ("hypothesis_id", json!(HYPOTHESIS_ID)),
            ("exploit_family", json!("authorization_policy_bypass")),
            (
                "summary",
                json!("debug_override may bypass token verification for an admin request"),
            ),
            ("target_surface", json!("admin access decision gate")),
            (
                "unsafe_outcome",
                json!("admin-only surface reached without a valid token"),
            ),
            (
                "preconditions",
                json!([
                    "role=admin is accepted from the request body",
                    "token is absent",
                    "debug_override=true is evaluated before token verification"
                ]),
            ),
            (
                "steps",
                json!([
                    "submit the deterministic attack request",
                    "observe the pre-fix access decision",
                    "record whether the unsafe admin state is reached"
                ]),
            ),
            ("confidence", json!("high")),
            (
                "uncertainties",
                json!(["none for the local fixture; live system generalization is out of scope"]),
            ),
            (
                "policy_notes",
                json!(["validation posture permits this bounded demo-only exploit attempt"]),
            ),
        ],
    )
}

fn exploit_evidence() -> Value {
    with_fields(
        artifact_base(
            EVIDENCE_ID,
            "ExploitEvidenceArtifact",
            "validated",
            "exploit_evidence",
            &[HYPOTHESIS_PATH, TARGET_PATH, POSTURE_PATH],
        ),
        vec![
            ("hypothesis_ref", json!(HYPOTHESIS_PATH)),
            ("attempt_id", json!("attempt-pre-fix-debug-override")),
            ("outcome", json!("success")),
            (
                "outcome_summary",
                json!("pre-fix gate allowed admin access without a token"),
            ),
            (
                "preconditions_observed",
                json!([
                    "role=admin",
                    "token=null",
                    "debug_override=true",
                    "pre-fix rule order evaluates debug override before token verification"
                ]),
            ),
            (
                "attempt_steps",
                json!([
                    "load bounded demo target",
                    "apply attack request",
                    "evaluate pre-fix policy gate",
                    "capture access decision and unsafe-state flag"
                ]),
            ),
            (
                "observed_effects",
                json!({
                    "access_decision": "allow",
                    "unsafe_state": "admin_panel_reached_without_token"
                }),
            ),
            ("unsafe_state_reached", json!(true)),
            ("evidence_refs", json!([PRE_REPLAY_PATH])),
            ("replayability", json!("deterministic")),
            (
                "residual_uncertainty",
                json!("none for the bounded fixture"),
            ),
        ],
    )
}

fn exploit_classification() -> Value {
    with_fields(
        artifact_base(
            CLASSIFICATION_ID,
            "ExploitClassificationArtifact",
            "recorded",
            "exploit_evidence",
            &[EVIDENCE_PATH],
        ),
        vec![
            ("evidence_ref", json!(EVIDENCE_PATH)),
            ("exploit_family", json!("authorization_policy_bypass")),
            ("target_type", json!("bounded_demo_policy_gate")),
            ("severity", json!("high_in_demo_scope")),
            ("reproducibility", json!("deterministic")),
            ("recurrence_potential", json!("high when policy shortcuts precede authentication")),
            ("mitigation_complexity", json!("low")),
            (
                "classification_notes",
                json!("The vulnerability is a policy ordering flaw: convenience override before authorization."),
            ),
        ],
    )
}

fn replay_manifest() -> Value {
    with_fields(
        artifact_base(
            REPLAY_ID,
            "AdversarialReplayManifest",
            "validated",
            "replay_pre_fix",
            &[EVIDENCE_PATH],
        ),
        vec![
            ("replay_id", json!(REPLAY_ID)),
            ("evidence_ref", json!(EVIDENCE_PATH)),
            (
                "summary",
                json!("Replay the same debug_override admin request before and after mitigation."),
            ),
            (
                "replay_goal",
                json!("prove the exploit reproduces pre-mitigation and stops reproducing post-mitigation"),
            ),
            ("replay_mode", json!("deterministic")),
            (
                "required_preconditions",
                json!([
                    "bounded demo target loaded",
                    "fixed attack request used unchanged",
                    "pre-fix and post-fix rule order selected explicitly"
                ]),
            ),
            (
                "environment",
                json!({
                    "kind": "local_fixture",
                    "network": false,
                    "secrets": false,
                    "external_services": []
                }),
            ),
            (
                "inputs",
                json!({
                    "request": {
                        "role": "admin",
                        "token": null,
                        "debug_override": true
                    }
                }),
            ),
            (
                "replay_steps",
                json!([
                    {
                        "step_id": "load_target",
                        "phase": "shared",
                        "expected": "target fixture and posture are available"
                    },
                    {
                        "step_id": "apply_attack_request_pre_fix",
                        "phase": "pre_mitigation",
                        "expected": "access_decision=allow"
                    },
                    {
                        "step_id": "apply_attack_request_post_fix",
                        "phase": "post_mitigation",
                        "expected": "access_decision=deny"
                    }
                ]),
            ),
            (
                "expected_outcome",
                json!({
                    "pre_mitigation": {
                        "unsafe_state_reached": true,
                        "access_decision": "allow"
                    },
                    "post_mitigation": {
                        "unsafe_state_reached": false,
                        "access_decision": "deny"
                    }
                }),
            ),
            (
                "success_criteria",
                json!([
                    "pre-mitigation replay reproduces the unsafe state",
                    "post-mitigation replay denies the same request",
                    "promotion artifact links evidence, mitigation, and replay results"
                ]),
            ),
            (
                "failure_modes",
                json!([
                    "pre-mitigation replay does not reproduce the exploit",
                    "post-mitigation replay still reaches the unsafe state",
                    "artifact chain loses evidence or mitigation provenance"
                ]),
            ),
            (
                "trace_expectations",
                json!([
                    "target_and_posture",
                    "exploit_hypothesis",
                    "exploit_evidence",
                    "replay_pre_fix",
                    "mitigation",
                    "replay_post_fix",
                    "promotion"
                ]),
            ),
            (
                "limitations",
                json!(["local deterministic fixture only; not a live exploit runner"]),
            ),
        ],
    )
}

fn pre_fix_replay_result() -> Value {
    json!({
        "schema_version": "adversarial_demo_replay_result.v1",
        "artifact_id": "replay-result-token-gate-pre-fix",
        "artifact_type": "ReplayValidationArtifact",
        "run_id": RUN_ID,
        "replay_id": REPLAY_ID,
        "manifest_ref": REPLAY_PATH,
        "phase": "pre_mitigation",
        "status": "validated",
        "input_ref": "replay_manifest.json#/inputs/request",
        "access_decision": "allow",
        "unsafe_state_reached": true,
        "result": "exploit_reproduced",
        "trace_refs": [trace_ref("replay_pre_fix", "pre-mitigation replay result")]
    })
}

fn mitigation() -> Value {
    with_fields(
        artifact_base(
            MITIGATION_ID,
            "MitigationLinkageArtifact",
            "validated",
            "mitigation",
            &[EVIDENCE_PATH, CLASSIFICATION_PATH, REPLAY_PATH, PRE_REPLAY_PATH],
        ),
        vec![
            ("evidence_ref", json!(EVIDENCE_PATH)),
            ("classification_ref_or_defer_reason", json!(CLASSIFICATION_PATH)),
            ("mitigation_id", json!(MITIGATION_ID)),
            ("mitigation_type", json!("policy_guard_ordering_fix")),
            (
                "summary",
                json!("verify token before considering debug override and ignore debug override for admin surfaces"),
            ),
            (
                "protection_boundary",
                json!("admin authorization gate for the bounded demo fixture"),
            ),
            (
                "tradeoffs",
                json!(["debug convenience cannot grant admin access without authentication"]),
            ),
            ("side_effects", json!(["none for non-admin demo requests"])),
            (
                "validation_plan",
                json!({
                    "manifest_ref": REPLAY_PATH,
                    "expected_post_mitigation_access_decision": "deny",
                    "expected_post_mitigation_unsafe_state_reached": false
                }),
            ),
        ],
    )
}

fn post_fix_replay_result() -> Value {
    json!({
        "schema_version": "adversarial_demo_replay_result.v1",
        "artifact_id": "replay-result-token-gate-post-fix",
        "artifact_type": "ReplayValidationArtifact",
        "run_id": RUN_ID,
        "replay_id": REPLAY_ID,
        "manifest_ref": REPLAY_PATH,
        "phase": "post_mitigation",
        "status": "validated",
        "mitigation_ref": MITIGATION_PATH,
        "input_ref": "replay_manifest.json#/inputs/request",
        "access_decision": "deny",
        "unsafe_state_reached": false,
        "result": "mitigation_validated",
        "trace_refs": [trace_ref("replay_post_fix", "post-mitigation replay result")]
    })
}

fn promotion() -> Value {
    with_fields(
        artifact_base(
            PROMOTION_ID,
            "ExploitPromotionArtifact",
            "validated",
            "promotion",
            &[EVIDENCE_PATH, MITIGATION_PATH, PRE_REPLAY_PATH, POST_REPLAY_PATH],
        ),
        vec![
            ("evidence_ref", json!(EVIDENCE_PATH)),
            ("mitigation_ref", json!(MITIGATION_PATH)),
            ("promotion_id", json!(PROMOTION_ID)),
            (
                "promotion_targets",
                json!([
                    "regression replay fixture: token-gate-debug-override",
                    "hardening rule: authentication before debug override",
                    "review packet invariant: pre true -> post false"
                ]),
            ),
            (
                "promotion_reason",
                json!("the exploit was reproduced and then blocked by replaying the same request after mitigation"),
            ),
            (
                "expected_future_value",
                json!("future changes can detect policy-ordering regressions without reconstructing the exploit narrative"),
            ),
            ("promotion_status", json!("applied")),
        ],
    )
}

fn review_packet() -> Value {
    json!({
        "schema_version": "adversarial_demo_review_packet.v1",
        "demo_id": "D5",
        "demo_name": DEMO_NAME,
        "run_id": RUN_ID,
        "status": "landed",
        "primary_claim": "full exploit -> replay -> mitigation -> promotion loop",
        "entrypoint": format!("adl demo {DEMO_NAME} --run --trace --out .adl/reports/adversarial-demo --no-open"),
        "primary_proof_surface": REVIEW_PACKET_PATH,
        "artifact_chain": [
            TARGET_PATH,
            POSTURE_PATH,
            HYPOTHESIS_PATH,
            EVIDENCE_PATH,
            CLASSIFICATION_PATH,
            REPLAY_PATH,
            PRE_REPLAY_PATH,
            MITIGATION_PATH,
            POST_REPLAY_PATH,
            PROMOTION_PATH,
            "trace.jsonl"
        ],
        "result_summary": {
            "vulnerability": "debug_override evaluated before token verification",
            "exploit_reproduced_pre_mitigation": true,
            "pre_mitigation_unsafe_state_reached": true,
            "mitigation_linked": true,
            "exploit_blocked_post_mitigation": true,
            "post_mitigation_unsafe_state_reached": false,
            "promotion_status": "applied"
        },
        "reviewer_answers": [
            {
                "question": "what vulnerability was found?",
                "answer": "a policy-ordering flaw allowed debug_override to grant admin access before token verification"
            },
            {
                "question": "how was it exploited?",
                "answer": "the replay request used role=admin, token=null, and debug_override=true against the pre-fix gate"
            },
            {
                "question": "can the exploit be reproduced?",
                "answer": "yes, replay_pre_fix/result.json records unsafe_state_reached=true for the deterministic request"
            },
            {
                "question": "what mitigation was applied?",
                "answer": "token verification now precedes debug override and debug override cannot grant admin access"
            },
            {
                "question": "does the mitigation hold under replay?",
                "answer": "yes, replay_post_fix/result.json records unsafe_state_reached=false for the same request"
            },
            {
                "question": "what was learned and promoted?",
                "answer": "the exploit became a regression replay fixture and hardening rule linked by promotion.json"
            }
        ],
        "upstream_contracts": [
            ADVERSARIAL_RUNTIME_MODEL_SCHEMA,
            RED_BLUE_AGENT_ARCHITECTURE_SCHEMA,
            ADVERSARIAL_EXECUTION_RUNNER_SCHEMA,
            EXPLOIT_ARTIFACT_REPLAY_SCHEMA,
            CONTINUOUS_VERIFICATION_SELF_ATTACK_SCHEMA
        ],
        "security_bounds": [
            "local deterministic fixture only",
            "no live target",
            "no network",
            "no secrets",
            "no uncontrolled exploitation"
        ],
        "pass_conditions": [
            "review_packet.result_summary.pre_mitigation_unsafe_state_reached == true",
            "review_packet.result_summary.post_mitigation_unsafe_state_reached == false",
            "promotion.json promotion_status == applied",
            "trace.jsonl includes the target, exploit, replay, mitigation, replay, and promotion steps"
        ]
    })
}

fn readme() -> &'static str {
    r#"# Demo H Output - v0.89.1 Adversarial Self-Attack

Generated by:

```bash
cargo run --manifest-path adl/Cargo.toml --bin adl -- demo demo-h-v0891-adversarial-self-attack --run --trace --out .adl/reports/adversarial-demo --no-open
```

This is a safe, deterministic, local-only proof surface. It does not attack a live
system. The fixture models a token-gate ordering flaw where `debug_override=true`
is evaluated before token verification for an admin request.

Primary artifacts:
- `review_packet.json`
- `target/target.json`
- `target/security_posture.json`
- `hypothesis.json`
- `evidence.json`
- `classification.json`
- `replay_manifest.json`
- `replay_pre_fix/result.json`
- `mitigation.json`
- `replay_post_fix/result.json`
- `promotion.json`
- `trace.jsonl`

Proof signal:
- pre-mitigation replay reaches the unsafe state
- post-mitigation replay denies the same request
- promotion links the exploit evidence, mitigation, replay results, and regression target
"#
}

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::path::Path;

use crate::adl;
use crate::learning_guardrails::{
    validate_overlay_security_guardrails, OverlaySecurityMutationAttempt,
};

pub const OVERLAY_VERSION: u32 = 1;

/// Overlay schema v1 used for deterministic learning-time config mutations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OverlaySpecV1 {
    /// Overlay schema version marker.
    pub overlay_version: u32,
    /// Logical source run id for audit linkage (optional).
    #[serde(default)]
    pub base_run_id: Option<String>,
    /// Author/automation identifier.
    pub created_by: String,
    /// Source artifact versions used to generate this overlay.
    pub created_from: OverlayCreatedFrom,
    /// Ordered set of overlay changes to apply.
    #[serde(default)]
    pub changes: Vec<OverlayChange>,
}

/// Version metadata captured when overlay is generated from learning artifacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OverlayCreatedFrom {
    #[serde(default)]
    pub suggestions_version: Option<u32>,
    #[serde(default)]
    pub artifact_model_version: Option<u32>,
}

/// Supported overlay operation kinds.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OverlayOp {
    /// Set the target field to a new value.
    Set,
}

/// Single overlay mutation entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OverlayChange {
    /// Stable change identifier.
    pub id: String,
    /// Canonical target path.
    pub path: String,
    /// Mutation operation.
    pub op: OverlayOp,
    /// New value to apply.
    pub value: JsonValue,
    /// Human-readable reason.
    pub rationale: String,
    /// Optional supporting evidence object.
    #[serde(default)]
    pub evidence: Option<JsonValue>,
}

/// Deterministic audit payload emitted after overlay application.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppliedOverlayAudit {
    pub overlay_hash: String,
    pub source_path: String,
    pub applied_change_ids: Vec<String>,
    pub applied_paths: Vec<String>,
}

/// Load and validate an overlay file from disk.
pub fn load_overlay(path: &Path) -> Result<OverlaySpecV1> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read overlay file '{}'", path.display()))?;
    let overlay: OverlaySpecV1 = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse overlay file '{}'", path.display()))?;
    if overlay.overlay_version != OVERLAY_VERSION {
        return Err(anyhow!(
            "overlay_version must be {} (found {})",
            OVERLAY_VERSION,
            overlay.overlay_version
        ));
    }
    if overlay.created_by.trim().is_empty() {
        return Err(anyhow!("overlay.created_by must not be empty"));
    }
    validate_overlay_changes(&overlay.changes)?;
    Ok(overlay)
}

/// Apply an overlay to an ADL document and return deterministic audit metadata.
pub fn apply_overlay_to_doc(
    doc: &mut adl::AdlDoc,
    overlay: &OverlaySpecV1,
) -> Result<AppliedOverlayAudit> {
    // Security invariant: overlays are an explicit allowlist. Any path outside
    // supported mutation targets is rejected, which keeps delegation policy and
    // other trust boundaries immutable in v1.
    let canonical = serde_json::to_vec(overlay).context("serialize overlay for hashing")?;
    let overlay_hash = stable_fingerprint_hex(&canonical);

    let mut applied_change_ids = Vec::new();
    let mut applied_paths = Vec::new();
    for change in &overlay.changes {
        enforce_guardrails(change)?;
        apply_change(doc, change)?;
        applied_change_ids.push(change.id.clone());
        applied_paths.push(change.path.clone());
    }
    doc.validate()
        .context("overlay produced invalid run configuration")?;

    Ok(AppliedOverlayAudit {
        overlay_hash,
        source_path: overlay
            .base_run_id
            .clone()
            .unwrap_or_else(|| "<overlay-file>".to_string()),
        applied_change_ids,
        applied_paths,
    })
}

fn validate_overlay_changes(changes: &[OverlayChange]) -> Result<()> {
    let mut seen = BTreeMap::<&str, ()>::new();
    for c in changes {
        if c.id.trim().is_empty() {
            return Err(anyhow!("overlay change id must not be empty"));
        }
        if seen.insert(c.id.as_str(), ()).is_some() {
            return Err(anyhow!("duplicate overlay change id '{}'", c.id));
        }
        if c.path.trim().is_empty() {
            return Err(anyhow!("overlay change '{}' path must not be empty", c.id));
        }
    }
    Ok(())
}

fn enforce_guardrails(change: &OverlayChange) -> Result<()> {
    let mut attempt = OverlaySecurityMutationAttempt::default();
    let path = change.path.as_str();
    if path.starts_with("run.remote.") {
        attempt.require_signed_requests = Some(false);
    }
    if path.contains("verify_allowed_algs") {
        attempt.verify_allowed_algs = Some(vec!["rsa".to_string()]);
    }
    if path.contains("verify_allowed_key_sources") {
        attempt.verify_allowed_key_sources = Some(vec!["embedded".to_string()]);
    }
    if path.starts_with("sandbox.") || path.starts_with("run.sandbox.") {
        attempt.sandbox_root = Some(".".to_string());
    }
    if path.contains("max_concurrency") {
        attempt.scheduler_max_concurrency = Some(1);
    }
    if let Err(e) = validate_overlay_security_guardrails(&attempt) {
        return Err(anyhow!("{}: {}", e.code(), e.message()));
    }
    Ok(())
}

fn apply_change(doc: &mut adl::AdlDoc, change: &OverlayChange) -> Result<()> {
    match change.path.as_str() {
        "run.workflow.steps.*.retry.max_attempts" => {
            let max_attempts = value_u32(change)?;
            let workflow = doc.run.workflow.as_mut().ok_or_else(|| {
                anyhow!(
                    "overlay path '{}' requires inline run.workflow",
                    change.path
                )
            })?;
            for step in &mut workflow.steps {
                step.retry = Some(adl::StepRetry { max_attempts });
            }
            Ok(())
        }
        _ => Err(anyhow!(
            "overlay change '{}' uses unsupported path '{}'",
            change.id,
            change.path
        )),
    }
}

fn value_u32(change: &OverlayChange) -> Result<u32> {
    let n = change.value.as_u64().ok_or_else(|| {
        anyhow!(
            "overlay change '{}' path '{}' expects integer value",
            change.id,
            change.path
        )
    })?;
    if n == 0 || n > u32::MAX as u64 {
        return Err(anyhow!(
            "overlay change '{}' path '{}' integer out of range",
            change.id,
            change.path
        ));
    }
    Ok(n as u32)
}

fn stable_fingerprint_hex(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn doc_with_inline_steps() -> adl::AdlDoc {
        adl::AdlDoc {
            version: "0.7".to_string(),
            providers: Default::default(),
            tools: Default::default(),
            agents: Default::default(),
            tasks: Default::default(),
            workflows: Default::default(),
            patterns: vec![],
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: adl::RunDefaults::default(),
                workflow_ref: None,
                workflow: Some(adl::WorkflowSpec {
                    id: Some("wf".to_string()),
                    kind: adl::WorkflowKind::Sequential,
                    max_concurrency: None,
                    steps: vec![
                        adl::StepSpec {
                            id: Some("s1".to_string()),
                            save_as: None,
                            write_to: None,
                            on_error: None,
                            retry: None,
                            agent: None,
                            task: None,
                            call: None,
                            with: Default::default(),
                            as_ns: None,
                            delegation: None,
                            prompt: None,
                            inputs: Default::default(),
                            placement: None,
                            guards: vec![],
                        },
                        adl::StepSpec {
                            id: Some("s2".to_string()),
                            save_as: None,
                            write_to: None,
                            on_error: None,
                            retry: None,
                            agent: None,
                            task: None,
                            call: None,
                            with: Default::default(),
                            as_ns: None,
                            delegation: None,
                            prompt: None,
                            inputs: Default::default(),
                            placement: None,
                            guards: vec![],
                        },
                    ],
                }),
                pattern_ref: None,
                inputs: Default::default(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        }
    }

    #[test]
    fn rejects_forbidden_remote_surface_via_guardrails() {
        let mut doc = doc_with_inline_steps();
        let overlay = OverlaySpecV1 {
            overlay_version: OVERLAY_VERSION,
            base_run_id: None,
            created_by: "test".to_string(),
            created_from: OverlayCreatedFrom {
                suggestions_version: None,
                artifact_model_version: Some(1),
            },
            changes: vec![OverlayChange {
                id: "c1".to_string(),
                path: "run.remote.require_signed_requests".to_string(),
                op: OverlayOp::Set,
                value: JsonValue::Bool(false),
                rationale: "bad".to_string(),
                evidence: None,
            }],
        };
        let err = apply_overlay_to_doc(&mut doc, &overlay).expect_err("must reject");
        assert!(err
            .to_string()
            .contains("LEARNING_GUARDRAIL_TRUST_POLICY_IMMUTABLE"));
    }

    #[test]
    fn apply_overlay_sets_retry_for_all_steps_deterministically() {
        let mut doc = doc_with_inline_steps();
        let overlay = OverlaySpecV1 {
            overlay_version: OVERLAY_VERSION,
            base_run_id: Some("run-1".to_string()),
            created_by: "test".to_string(),
            created_from: OverlayCreatedFrom {
                suggestions_version: Some(1),
                artifact_model_version: Some(1),
            },
            changes: vec![OverlayChange {
                id: "retry-all".to_string(),
                path: "run.workflow.steps.*.retry.max_attempts".to_string(),
                op: OverlayOp::Set,
                value: JsonValue::from(2u64),
                rationale: "set retry".to_string(),
                evidence: None,
            }],
        };

        let audit = apply_overlay_to_doc(&mut doc, &overlay).expect("overlay apply");
        let wf = doc.run.workflow.as_ref().expect("workflow");
        assert_eq!(
            wf.steps
                .iter()
                .map(|s| s.retry.as_ref().map(|r| r.max_attempts))
                .collect::<Vec<_>>(),
            vec![Some(2), Some(2)]
        );
        assert_eq!(audit.applied_change_ids, vec!["retry-all"]);
        assert_eq!(
            audit.applied_paths,
            vec!["run.workflow.steps.*.retry.max_attempts"]
        );
        assert!(!audit.overlay_hash.is_empty());
    }

    #[test]
    fn apply_overlay_rejects_bad_values_and_paths() {
        let mut doc = doc_with_inline_steps();
        let bad_value = OverlaySpecV1 {
            overlay_version: OVERLAY_VERSION,
            base_run_id: None,
            created_by: "test".to_string(),
            created_from: OverlayCreatedFrom {
                suggestions_version: None,
                artifact_model_version: Some(1),
            },
            changes: vec![OverlayChange {
                id: "bad".to_string(),
                path: "run.workflow.steps.*.retry.max_attempts".to_string(),
                op: OverlayOp::Set,
                value: JsonValue::String("two".to_string()),
                rationale: "bad".to_string(),
                evidence: None,
            }],
        };
        let err = apply_overlay_to_doc(&mut doc, &bad_value).expect_err("must reject");
        assert!(err.to_string().contains("expects integer value"));

        let mut doc2 = doc_with_inline_steps();
        let bad_path = OverlaySpecV1 {
            overlay_version: OVERLAY_VERSION,
            base_run_id: None,
            created_by: "test".to_string(),
            created_from: OverlayCreatedFrom {
                suggestions_version: None,
                artifact_model_version: Some(1),
            },
            changes: vec![OverlayChange {
                id: "bad-path".to_string(),
                path: "run.workflow.steps.*.retry.unsupported".to_string(),
                op: OverlayOp::Set,
                value: JsonValue::from(2u64),
                rationale: "bad".to_string(),
                evidence: None,
            }],
        };
        let err = apply_overlay_to_doc(&mut doc2, &bad_path).expect_err("must reject");
        assert!(err.to_string().contains("uses unsupported path"));
    }

    #[test]
    fn apply_overlay_rejects_delegation_policy_mutation_paths() {
        let mut doc = doc_with_inline_steps();
        let overlay = OverlaySpecV1 {
            overlay_version: OVERLAY_VERSION,
            base_run_id: None,
            created_by: "test".to_string(),
            created_from: OverlayCreatedFrom {
                suggestions_version: None,
                artifact_model_version: Some(1),
            },
            changes: vec![OverlayChange {
                id: "delegate-policy".to_string(),
                path: "run.delegation_policy.default_action".to_string(),
                op: OverlayOp::Set,
                value: JsonValue::String("allow".to_string()),
                rationale: "attempt policy change".to_string(),
                evidence: None,
            }],
        };

        let err = apply_overlay_to_doc(&mut doc, &overlay).expect_err("must reject");
        let msg = err.to_string();
        assert!(msg.contains("uses unsupported path"));
        assert!(msg.contains("run.delegation_policy.default_action"));
    }

    #[test]
    fn load_overlay_validates_version_created_by_and_duplicate_ids() {
        let td = std::env::temp_dir().join(format!("overlay-load-{}", std::process::id()));
        let _ = fs::create_dir_all(&td);
        let p = td.join("overlay.json");

        fs::write(
            &p,
            r#"{"overlay_version":2,"created_by":"x","created_from":{},"changes":[]}"#,
        )
        .unwrap();
        let err = load_overlay(&p).expect_err("bad version");
        assert!(err.to_string().contains("overlay_version must be 1"));

        fs::write(
            &p,
            r#"{"overlay_version":1,"created_by":" ","created_from":{},"changes":[]}"#,
        )
        .unwrap();
        let err = load_overlay(&p).expect_err("empty created_by");
        assert!(err.to_string().contains("created_by must not be empty"));

        fs::write(
            &p,
            r#"{"overlay_version":1,"created_by":"x","created_from":{},"changes":[
              {"id":"dup","path":"run.workflow.steps.*.retry.max_attempts","op":"set","value":2,"rationale":"a"},
              {"id":"dup","path":"run.workflow.steps.*.retry.max_attempts","op":"set","value":2,"rationale":"b"}
            ]}"#,
        )
        .unwrap();
        let err = load_overlay(&p).expect_err("dup ids");
        assert!(err.to_string().contains("duplicate overlay change id"));
    }
}

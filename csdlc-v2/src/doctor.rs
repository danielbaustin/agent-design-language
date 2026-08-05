use std::fs;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString};

use crate::cards::digest;
use crate::error::{ErrorCode, Result, V2Error};
use crate::model::{DesignReview, LifecyclePhase};
use crate::preparation::{
    load_binding_intent, load_manifest, BindingIntentState, PreparationState,
};
use crate::review::evaluate_publication_review_in_repo;
use crate::store::{now_seconds, verify_cards, verify_record, Store};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DoctorStatus {
    Pass,
    Block,
    Corrupt,
    Interrupted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Finding {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DoctorReport {
    pub schema: String,
    pub issue: u64,
    pub status: DoctorStatus,
    pub phase: Option<LifecyclePhase>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preparation_state: Option<PreparationState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_intent_digest: Option<String>,
    pub generation: Option<u64>,
    pub ready: bool,
    pub findings: Vec<Finding>,
    pub next_operation: Option<String>,
}

pub fn diagnose(store: &Store, issue: u64) -> DoctorReport {
    diagnose_internal(store, issue, true)
}

pub(crate) fn diagnose_canonical(store: &Store, issue: u64) -> DoctorReport {
    diagnose_internal(store, issue, false)
}

fn diagnose_internal(store: &Store, issue: u64, include_preparation: bool) -> DoctorReport {
    let mut report = DoctorReport {
        schema: "csdlc.doctor.report.v1".into(),
        issue,
        status: DoctorStatus::Pass,
        phase: None,
        preparation_state: None,
        binding_intent_digest: None,
        generation: None,
        ready: false,
        findings: Vec::new(),
        next_operation: None,
    };
    if !store.issue_dir(issue).exists() && store.interrupted_backup(issue).exists() {
        report.status = DoctorStatus::Interrupted;
        report.findings.push(Finding {
            code: "interrupted_transaction".into(),
            message: "complete prior generation is recoverable by the next writer".into(),
        });
        report.next_operation = Some("recover_then_retry".into());
        return report;
    }
    if include_preparation {
        let preparation_manifest = store
            .root()
            .join(".csdlc/preparation/issues")
            .join(issue.to_string())
            .join("manifest.json");
        match load_manifest(store, issue) {
            Ok(manifest)
                if store.issue_dir(issue).exists() && manifest.state != PreparationState::Bound =>
            {
                report.status = DoctorStatus::Corrupt;
                report.preparation_state = Some(manifest.state);
                report.findings.push(Finding {
                    code: "dual_lifecycle_authority".into(),
                    message:
                        "canonical issue authority coexists with non-bound preparation authority"
                            .into(),
                });
                report.next_operation = Some("csdlc-migrate repair".into());
                return report;
            }
            Ok(manifest)
                if !store.issue_dir(issue).exists()
                    || manifest.state != PreparationState::Bound =>
            {
                report.preparation_state = Some(manifest.state);
                report.ready = manifest.state == PreparationState::ExecutionReady;
                let next = match manifest.state {
                    PreparationState::Draft => "csdlc-prepare sync",
                    PreparationState::Prepared => "csdlc-prepare seal",
                    PreparationState::ExecutionReady => "csdlc-bind run",
                    PreparationState::Binding => match load_binding_intent(store, issue) {
                        Ok(Some(intent)) if intent.state == BindingIntentState::Releasing => {
                            report.binding_intent_digest = Some(intent.digest);
                            "csdlc-bind release"
                        }
                        Ok(Some(intent)) => {
                            report.binding_intent_digest = Some(intent.digest);
                            "csdlc-bind run"
                        }
                        Ok(None) => "csdlc-migrate repair",
                        Err(error) => {
                            report.status = DoctorStatus::Corrupt;
                            report.findings.push(finding(error));
                            report.next_operation = Some("csdlc-migrate repair".into());
                            return report;
                        }
                    },
                    PreparationState::Bound => "inspect_bound_worktree",
                };
                report.next_operation = Some(next.into());
                return report;
            }
            Ok(_) => {}
            Err(_) if !preparation_manifest.exists() => {}
            Err(error) => {
                report.status = DoctorStatus::Corrupt;
                report.findings.push(finding(error));
                report.next_operation = Some("csdlc-migrate repair".into());
                return report;
            }
        }
    }
    let record = match store.load_record(issue) {
        Ok(record) => record,
        Err(error) => {
            report.status = DoctorStatus::Corrupt;
            report.findings.push(finding(error));
            return report;
        }
    };
    report.phase = Some(record.phase);
    report.generation = Some(record.generation);
    if let Err(error) = verify_record(&record) {
        report.status = DoctorStatus::Corrupt;
        report.findings.push(finding(error));
        return report;
    }
    for (code, path) in [
        ("design_missing", &record.design_path),
        ("diagram_missing", &record.diagram_path),
    ] {
        if !store.root().join(path).is_file() {
            report.findings.push(Finding {
                code: code.into(),
                message: format!("required path is missing: {path}"),
            });
        }
    }
    if let Some(claim) = &record.claim {
        let now = now_seconds().unwrap_or(u64::MAX);
        if let Err(error) = claim.validate(&claim.id, now) {
            report.findings.push(Finding {
                code: "claim_not_live".into(),
                message: error.message,
            });
        }
    } else if !matches!(
        record.phase,
        LifecyclePhase::Merged | LifecyclePhase::ClosedOut
    ) && !record.audit.last().is_some_and(|event| {
        serde_json::from_str::<serde_json::Value>(&event.operation)
            .ok()
            .and_then(|value| {
                value
                    .get("operation")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned)
            })
            .as_deref()
            == Some("release_closed_claim")
    }) {
        report.findings.push(Finding {
            code: "claim_dormant".into(),
            message: "nonterminal issue has no active writer claim".into(),
        });
    }
    if !report.findings.is_empty() {
        report.status = DoctorStatus::Block;
        report.next_operation =
            Some(
                if report.findings.iter().any(|finding| {
                    finding.code == "claim_dormant" || finding.code == "claim_not_live"
                }) {
                    "csdlc-migrate repair"
                } else {
                    "repair_design_readiness"
                }
                .into(),
            );
        return report;
    }
    let cards = match store.load_cards(issue) {
        Ok(cards) => cards,
        Err(error) => {
            report.status = DoctorStatus::Corrupt;
            report.findings.push(finding(error));
            return report;
        }
    };
    if let Err(error) = verify_cards(store, &record, &cards) {
        report.status = DoctorStatus::Corrupt;
        report.findings.push(finding(error));
        return report;
    }
    let diagram = fs::read_to_string(store.root().join(&record.diagram_path)).unwrap_or_default();
    let first = diagram
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or_default()
        .trim();
    if !(first.starts_with("flowchart ")
        || first == "stateDiagram-v2"
        || first.starts_with("sequenceDiagram"))
        || diagram.lines().count() < 2
    {
        report.findings.push(Finding {
            code: "diagram_invalid".into(),
            message: "diagram is not recognized Mermaid source".into(),
        });
    }
    let design_bytes = fs::read(store.root().join(&record.design_path)).unwrap_or_default();
    match &record.design_review {
        DesignReview::Approved { reviewer, revision }
            if !reviewer.trim().is_empty() && revision == &digest(&design_bytes) => {}
        _ => report.findings.push(Finding {
            code: "design_review_missing_or_stale".into(),
            message: "design review does not cover the current design digest".into(),
        }),
    }
    if matches!(
        record.phase,
        LifecyclePhase::Reviewed | LifecyclePhase::Published | LifecyclePhase::MergeReady
    ) {
        if let Some(review) = record.review.as_ref() {
            let current = crate::git::substantive_revision(store.root(), &review.scope);
            let stale = current.as_ref().is_ok_and(|current| {
                evaluate_publication_review_in_repo(store.root(), Some(review), current)
                    .blocker_codes
                    .iter()
                    .any(|code| code == "review_stale")
            });
            if stale {
                report.findings.push(Finding {
                    code: "review_publication_dead_end".into(),
                    message: "reviewed evidence does not match a clean current substantive commit; recover_review is required before publication".into(),
                });
            }
        }
    }
    if report.findings.is_empty() {
        report.ready = record.phase == LifecyclePhase::Initialized;
        report.next_operation = Some(
            if report.ready {
                "advance_ready"
            } else {
                "inspect_phase"
            }
            .into(),
        );
    } else {
        report.status = DoctorStatus::Block;
        report.next_operation = Some(
            if report
                .findings
                .iter()
                .any(|finding| finding.code == "review_publication_dead_end")
            {
                "recover_review"
            } else {
                "repair_design_readiness"
            }
            .into(),
        );
    }
    report
}

fn finding(error: V2Error) -> Finding {
    Finding {
        code: match error.code {
            ErrorCode::CorruptRecord => "corrupt_record",
            ErrorCode::InterruptedTransaction => "interrupted_transaction",
            _ => "doctor_error",
        }
        .into(),
        message: error.message,
    }
}

pub fn diagnose_result(store: &Store, issue: u64) -> Result<DoctorReport> {
    if issue == 0 {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "issue must be non-zero",
        ));
    }
    Ok(diagnose(store, issue))
}

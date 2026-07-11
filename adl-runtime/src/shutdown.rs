//! Ordered, retained shutdown coordination for the CSM runtime.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

pub const CSM_SHUTDOWN_STATE_SCHEMA: &str = "adl.csm.shutdown_state.v1";
pub const CSM_SHUTDOWN_DISPOSITION_SCHEMA: &str = "adl.csm.shutdown_disposition.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShutdownPhase {
    QuiesceAdmission,
    DrainWork,
    FlushContinuity,
    CloseLifelog,
    DrainObservability,
    FinalCloudNotices,
    JoinComponents,
    RetainDisposition,
}

impl ShutdownPhase {
    pub const ORDERED: [Self; 8] = [
        Self::QuiesceAdmission,
        Self::DrainWork,
        Self::FlushContinuity,
        Self::CloseLifelog,
        Self::DrainObservability,
        Self::FinalCloudNotices,
        Self::JoinComponents,
        Self::RetainDisposition,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuiesceAdmission => "quiesce_admission",
            Self::DrainWork => "drain_work",
            Self::FlushContinuity => "flush_continuity",
            Self::CloseLifelog => "close_lifelog",
            Self::DrainObservability => "drain_observability",
            Self::FinalCloudNotices => "final_cloud_notices",
            Self::JoinComponents => "join_components",
            Self::RetainDisposition => "retain_disposition",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShutdownStepOutcome {
    Completed,
    RecoverablePartial,
    Degraded,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShutdownStepRecord {
    pub sequence: u8,
    pub phase: ShutdownPhase,
    pub outcome: ShutdownStepOutcome,
    pub component: String,
    pub evidence_ref: String,
    pub detail: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernedShutdownState {
    pub schema: String,
    pub shutdown_id: String,
    pub runtime_owner: String,
    pub status: String,
    pub admission_quiesced: bool,
    pub active_phase: Option<ShutdownPhase>,
    pub completed_phase_count: usize,
    pub steps: Vec<ShutdownStepRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernedShutdownDisposition {
    pub schema: String,
    pub shutdown_id: String,
    pub status: String,
    pub final_state: String,
    pub publishable: bool,
    pub recoverable_partial_count: usize,
    pub degraded_count: usize,
    pub blocked_count: usize,
    pub steps: Vec<ShutdownStepRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShutdownError {
    EmptyShutdownId,
    OutOfOrder {
        expected: ShutdownPhase,
        observed: ShutdownPhase,
    },
    InvalidEvidence(ShutdownPhase),
    AlreadyComplete,
    Incomplete {
        completed: usize,
        required: usize,
    },
}

impl fmt::Display for ShutdownError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyShutdownId => write!(formatter, "shutdown_id must be non-empty"),
            Self::OutOfOrder { expected, observed } => write!(
                formatter,
                "shutdown phase out of order: expected={} observed={}",
                expected.as_str(),
                observed.as_str()
            ),
            Self::InvalidEvidence(phase) => write!(
                formatter,
                "shutdown phase {} requires a non-empty evidence_ref",
                phase.as_str()
            ),
            Self::AlreadyComplete => write!(formatter, "shutdown DAG is already complete"),
            Self::Incomplete {
                completed,
                required,
            } => write!(
                formatter,
                "shutdown DAG is incomplete: completed={completed} required={required}"
            ),
        }
    }
}

impl std::error::Error for ShutdownError {}

impl GovernedShutdownState {
    pub fn new(shutdown_id: impl Into<String>) -> Result<Self, ShutdownError> {
        let shutdown_id = shutdown_id.into();
        if shutdown_id.trim().is_empty() {
            return Err(ShutdownError::EmptyShutdownId);
        }
        Ok(Self {
            schema: CSM_SHUTDOWN_STATE_SCHEMA.to_string(),
            shutdown_id,
            runtime_owner: "csm".to_string(),
            status: "quiescing".to_string(),
            admission_quiesced: false,
            active_phase: Some(ShutdownPhase::QuiesceAdmission),
            completed_phase_count: 0,
            steps: Vec::new(),
        })
    }

    pub fn expected_phase(&self) -> Option<ShutdownPhase> {
        ShutdownPhase::ORDERED.get(self.steps.len()).copied()
    }

    pub fn record(
        &mut self,
        phase: ShutdownPhase,
        outcome: ShutdownStepOutcome,
        component: impl Into<String>,
        evidence_ref: impl Into<String>,
        detail: Value,
    ) -> Result<(), ShutdownError> {
        let Some(expected) = self.expected_phase() else {
            return Err(ShutdownError::AlreadyComplete);
        };
        if phase != expected {
            return Err(ShutdownError::OutOfOrder {
                expected,
                observed: phase,
            });
        }
        let evidence_ref = evidence_ref.into();
        if evidence_ref.trim().is_empty() {
            return Err(ShutdownError::InvalidEvidence(phase));
        }
        self.steps.push(ShutdownStepRecord {
            sequence: self.steps.len() as u8 + 1,
            phase,
            outcome,
            component: component.into(),
            evidence_ref,
            detail,
        });
        self.completed_phase_count = self.steps.len();
        self.admission_quiesced = true;
        self.active_phase = self.expected_phase();
        self.status = if self.active_phase.is_some() {
            "shutting_down".to_string()
        } else {
            "shutdown_complete".to_string()
        };
        Ok(())
    }

    pub fn disposition(
        &self,
        final_state: impl Into<String>,
    ) -> Result<GovernedShutdownDisposition, ShutdownError> {
        if self.steps.len() != ShutdownPhase::ORDERED.len() {
            return Err(ShutdownError::Incomplete {
                completed: self.steps.len(),
                required: ShutdownPhase::ORDERED.len(),
            });
        }
        let recoverable_partial_count = self
            .steps
            .iter()
            .filter(|step| step.outcome == ShutdownStepOutcome::RecoverablePartial)
            .count();
        let degraded_count = self
            .steps
            .iter()
            .filter(|step| step.outcome == ShutdownStepOutcome::Degraded)
            .count();
        let blocked_count = self
            .steps
            .iter()
            .filter(|step| step.outcome == ShutdownStepOutcome::Blocked)
            .count();
        Ok(GovernedShutdownDisposition {
            schema: CSM_SHUTDOWN_DISPOSITION_SCHEMA.to_string(),
            shutdown_id: self.shutdown_id.clone(),
            status: "retained".to_string(),
            final_state: final_state.into(),
            publishable: blocked_count == 0
                && degraded_count == 0
                && recoverable_partial_count == 0,
            recoverable_partial_count,
            degraded_count,
            blocked_count,
            steps: self.steps.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn complete_state() -> GovernedShutdownState {
        let mut state = GovernedShutdownState::new("shutdown-1").expect("state");
        for phase in ShutdownPhase::ORDERED {
            state
                .record(
                    phase,
                    ShutdownStepOutcome::Completed,
                    phase.as_str(),
                    format!("shutdown/{}.json", phase.as_str()),
                    json!({"observed": true}),
                )
                .expect("ordered phase");
        }
        state
    }

    #[test]
    fn shutdown_state_enforces_order_and_evidence() {
        let mut state = GovernedShutdownState::new("shutdown-1").expect("state");
        let error = state
            .record(
                ShutdownPhase::DrainWork,
                ShutdownStepOutcome::Completed,
                "scheduler",
                "shutdown/drain.json",
                json!({}),
            )
            .expect_err("out of order");
        assert!(matches!(error, ShutdownError::OutOfOrder { .. }));
        assert!(state
            .record(
                ShutdownPhase::QuiesceAdmission,
                ShutdownStepOutcome::Completed,
                "runtime_api_and_scheduler",
                "",
                json!({}),
            )
            .is_err());
    }

    #[test]
    fn shutdown_disposition_counts_degraded_and_partial_outcomes() {
        let mut state = complete_state();
        state.steps[1].outcome = ShutdownStepOutcome::RecoverablePartial;
        state.steps[5].outcome = ShutdownStepOutcome::Blocked;
        let disposition = state
            .disposition("recoverable_sleeping")
            .expect("disposition");
        assert_eq!(disposition.recoverable_partial_count, 1);
        assert_eq!(disposition.blocked_count, 1);
        assert!(!disposition.publishable);
    }

    #[test]
    fn partial_or_degraded_shutdown_is_never_publishable() {
        for outcome in [
            ShutdownStepOutcome::RecoverablePartial,
            ShutdownStepOutcome::Degraded,
        ] {
            let mut state = complete_state();
            state.steps[1].outcome = outcome;
            let disposition = state
                .disposition("recoverable_sleeping")
                .expect("disposition");
            assert!(!disposition.publishable);
        }
    }

    #[test]
    fn incomplete_shutdown_cannot_claim_final_disposition() {
        let state = GovernedShutdownState::new("shutdown-1").expect("state");
        assert!(matches!(
            state.disposition("stopped"),
            Err(ShutdownError::Incomplete { .. })
        ));
    }
}

//! Execution policy errors and stable classification codes.

use super::super::DELEGATION_POLICY_DENY_CODE;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Stable policy rejection kinds emitted by execution-time delegation checks.
pub enum ExecutionPolicyErrorKind {
    /// Action was denied by policy and execution must fail/stop that step.
    Denied,
    /// Action requires approval and cannot proceed automatically.
    ApprovalRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionPolicyError {
    pub kind: ExecutionPolicyErrorKind,
    pub step_id: String,
    pub action_kind: String,
    pub target_id: String,
    pub rule_id: Option<String>,
}

impl ExecutionPolicyError {
    /// Convert the error variant to a short stable error code.
    pub fn code(&self) -> &'static str {
        match self.kind {
            ExecutionPolicyErrorKind::Denied => DELEGATION_POLICY_DENY_CODE,
            ExecutionPolicyErrorKind::ApprovalRequired => "DELEGATION_POLICY_APPROVAL_REQUIRED",
        }
    }
}

impl std::fmt::Display for ExecutionPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = self
            .rule_id
            .as_ref()
            .map(|id| format!(" (rule_id={id})"))
            .unwrap_or_default();
        match self.kind {
            ExecutionPolicyErrorKind::Denied => write!(
                f,
                "{}: step '{}' action '{}' target '{}' denied{}",
                self.code(),
                self.step_id,
                self.action_kind,
                self.target_id,
                suffix
            ),
            ExecutionPolicyErrorKind::ApprovalRequired => write!(
                f,
                "{}: step '{}' action '{}' target '{}' requires approval{}",
                self.code(),
                self.step_id,
                self.action_kind,
                self.target_id,
                suffix
            ),
        }
    }
}

impl std::error::Error for ExecutionPolicyError {}

/// Map execution policy errors to a stable failure classification.
pub fn stable_failure_kind(err: &anyhow::Error) -> Option<&'static str> {
    for cause in err.chain() {
        if cause.downcast_ref::<ExecutionPolicyError>().is_some() {
            return Some("policy_denied");
        }
    }
    None
}

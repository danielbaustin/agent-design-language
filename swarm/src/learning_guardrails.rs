#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OverlaySecurityMutationAttempt {
    pub require_signed_requests: Option<bool>,
    pub allow_unsigned: Option<bool>,
    pub require_key_id: Option<bool>,
    pub verify_allowed_algs: Option<Vec<String>>,
    pub verify_allowed_key_sources: Option<Vec<String>>,
    pub sandbox_root: Option<String>,
    pub requested_paths: Option<Vec<String>>,
    pub scheduler_max_concurrency: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LearningGuardrailError {
    TrustPolicyImmutable,
    SandboxPolicyImmutable,
    SchedulerPolicyImmutable,
}

impl LearningGuardrailError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::TrustPolicyImmutable => "LEARNING_GUARDRAIL_TRUST_POLICY_IMMUTABLE",
            Self::SandboxPolicyImmutable => "LEARNING_GUARDRAIL_SANDBOX_POLICY_IMMUTABLE",
            Self::SchedulerPolicyImmutable => "LEARNING_GUARDRAIL_SCHEDULER_POLICY_IMMUTABLE",
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::TrustPolicyImmutable => {
                "learning overlay cannot modify signing/trust verification requirements"
            }
            Self::SandboxPolicyImmutable => {
                "learning overlay cannot modify sandbox roots or requested path access"
            }
            Self::SchedulerPolicyImmutable => {
                "learning overlay cannot modify scheduler policy surfaces"
            }
        }
    }
}

pub fn validate_overlay_security_guardrails(
    attempt: &OverlaySecurityMutationAttempt,
) -> Result<(), LearningGuardrailError> {
    if attempt.require_signed_requests.is_some()
        || attempt.allow_unsigned.is_some()
        || attempt.require_key_id.is_some()
        || attempt.verify_allowed_algs.is_some()
        || attempt.verify_allowed_key_sources.is_some()
    {
        return Err(LearningGuardrailError::TrustPolicyImmutable);
    }

    if attempt.sandbox_root.is_some() || attempt.requested_paths.is_some() {
        return Err(LearningGuardrailError::SandboxPolicyImmutable);
    }

    if attempt.scheduler_max_concurrency.is_some() {
        return Err(LearningGuardrailError::SchedulerPolicyImmutable);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_overlay_attempt_to_relax_signing_requirements() {
        let attempt = OverlaySecurityMutationAttempt {
            require_signed_requests: Some(false),
            allow_unsigned: Some(true),
            ..Default::default()
        };

        let err = validate_overlay_security_guardrails(&attempt)
            .expect_err("trust policy mutation must be rejected");
        assert_eq!(err.code(), "LEARNING_GUARDRAIL_TRUST_POLICY_IMMUTABLE");
        assert_eq!(
            err.message(),
            "learning overlay cannot modify signing/trust verification requirements"
        );
    }

    #[test]
    fn rejects_overlay_attempt_to_expand_sandbox_paths() {
        let attempt = OverlaySecurityMutationAttempt {
            sandbox_root: Some("../outside".to_string()),
            requested_paths: Some(vec!["../escape.txt".to_string()]),
            ..Default::default()
        };

        let err = validate_overlay_security_guardrails(&attempt)
            .expect_err("sandbox policy mutation must be rejected");
        assert_eq!(err.code(), "LEARNING_GUARDRAIL_SANDBOX_POLICY_IMMUTABLE");
        assert_eq!(
            err.message(),
            "learning overlay cannot modify sandbox roots or requested path access"
        );
    }

    #[test]
    fn rejects_overlay_attempt_to_change_key_id_or_allowed_algs() {
        let attempt = OverlaySecurityMutationAttempt {
            require_key_id: Some(false),
            verify_allowed_algs: Some(vec!["rsa".to_string()]),
            verify_allowed_key_sources: Some(vec!["mystery".to_string()]),
            ..Default::default()
        };

        let err = validate_overlay_security_guardrails(&attempt)
            .expect_err("verification policy mutation must be rejected");
        assert_eq!(err.code(), "LEARNING_GUARDRAIL_TRUST_POLICY_IMMUTABLE");
        assert_eq!(
            err.message(),
            "learning overlay cannot modify signing/trust verification requirements"
        );
    }

    #[test]
    fn rejects_overlay_attempt_to_change_scheduler_policy() {
        let attempt = OverlaySecurityMutationAttempt {
            scheduler_max_concurrency: Some(1),
            ..Default::default()
        };

        let err = validate_overlay_security_guardrails(&attempt)
            .expect_err("scheduler mutation must be rejected");
        assert_eq!(err.code(), "LEARNING_GUARDRAIL_SCHEDULER_POLICY_IMMUTABLE");
        assert_eq!(
            err.message(),
            "learning overlay cannot modify scheduler policy surfaces"
        );
    }

    #[test]
    fn allows_empty_overlay_security_mutation_attempt() {
        let attempt = OverlaySecurityMutationAttempt::default();
        assert!(validate_overlay_security_guardrails(&attempt).is_ok());
    }
}

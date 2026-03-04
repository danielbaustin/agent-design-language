use crate::adl::{
    DelegationActionKind, DelegationPolicySpec, DelegationRuleEffect, DelegationSpec,
};

/// Result of evaluating delegation policy for an attempted action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelegationDecision {
    /// Delegated action is permitted immediately.
    Allowed,
    /// Delegated action is rejected by policy.
    Denied,
    /// Delegated action is permitted only after explicit approval.
    NeedsApproval,
}

impl DelegationDecision {
    pub fn as_str(&self) -> &'static str {
        match self {
            DelegationDecision::Allowed => "allowed",
            DelegationDecision::Denied => "denied",
            DelegationDecision::NeedsApproval => "needs_approval",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationPolicyOutcome {
    /// Final policy decision for this attempted action.
    pub decision: DelegationDecision,
    /// Matched rule id when a concrete policy rule decided the outcome.
    pub rule_id: Option<String>,
}

/// Evaluate delegation policy for a single action and target.
///
/// Evaluation order is deterministic:
/// 1) if delegation metadata is absent, allow
/// 2) if no policy exists, allow
/// 3) first matching rule in declared order decides
/// 4) otherwise fall back to `default_allow`
pub fn evaluate(
    policy: Option<&DelegationPolicySpec>,
    delegation: Option<&DelegationSpec>,
    action: DelegationActionKind,
    target_id: &str,
) -> DelegationPolicyOutcome {
    if delegation.is_none() {
        return DelegationPolicyOutcome {
            decision: DelegationDecision::Allowed,
            rule_id: None,
        };
    }

    let Some(policy) = policy else {
        return DelegationPolicyOutcome {
            decision: DelegationDecision::Allowed,
            rule_id: None,
        };
    };

    for rule in &policy.rules {
        if rule.action != action {
            continue;
        }
        if let Some(expected) = rule.target_id.as_ref() {
            if expected != target_id {
                continue;
            }
        }
        let decision = match rule.effect {
            DelegationRuleEffect::Deny => DelegationDecision::Denied,
            DelegationRuleEffect::Allow if rule.require_approval => {
                DelegationDecision::NeedsApproval
            }
            DelegationRuleEffect::Allow => DelegationDecision::Allowed,
        };
        return DelegationPolicyOutcome {
            decision,
            rule_id: Some(rule.id.clone()),
        };
    }

    DelegationPolicyOutcome {
        decision: if policy.default_allow {
            DelegationDecision::Allowed
        } else {
            DelegationDecision::Denied
        },
        rule_id: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adl::{DelegationPolicyRuleSpec, DelegationPolicySpec};

    fn delegation() -> DelegationSpec {
        DelegationSpec {
            role: Some("reviewer".to_string()),
            requires_verification: Some(true),
            escalation_target: None,
            tags: vec!["safety".to_string()],
        }
    }

    #[test]
    fn default_allow_and_deny_are_deterministic() {
        let allow_policy = DelegationPolicySpec {
            default_allow: true,
            rules: vec![],
        };
        let deny_policy = DelegationPolicySpec {
            default_allow: false,
            rules: vec![],
        };

        assert_eq!(
            evaluate(
                Some(&allow_policy),
                Some(&delegation()),
                DelegationActionKind::ProviderCall,
                "local",
            )
            .decision,
            DelegationDecision::Allowed
        );
        assert_eq!(
            evaluate(
                Some(&deny_policy),
                Some(&delegation()),
                DelegationActionKind::ProviderCall,
                "local",
            )
            .decision,
            DelegationDecision::Denied
        );
    }

    #[test]
    fn allow_and_deny_rules_and_precedence() {
        let policy = DelegationPolicySpec {
            default_allow: false,
            rules: vec![
                DelegationPolicyRuleSpec {
                    id: "deny-local".to_string(),
                    action: DelegationActionKind::ProviderCall,
                    target_id: Some("local".to_string()),
                    effect: DelegationRuleEffect::Deny,
                    require_approval: false,
                },
                DelegationPolicyRuleSpec {
                    id: "allow-local".to_string(),
                    action: DelegationActionKind::ProviderCall,
                    target_id: Some("local".to_string()),
                    effect: DelegationRuleEffect::Allow,
                    require_approval: false,
                },
            ],
        };
        let out = evaluate(
            Some(&policy),
            Some(&delegation()),
            DelegationActionKind::ProviderCall,
            "local",
        );
        assert_eq!(out.decision, DelegationDecision::Denied);
        assert_eq!(out.rule_id.as_deref(), Some("deny-local"));
    }

    #[test]
    fn allow_rule_overrides_default_deny() {
        let policy = DelegationPolicySpec {
            default_allow: false,
            rules: vec![DelegationPolicyRuleSpec {
                id: "allow-local".to_string(),
                action: DelegationActionKind::ProviderCall,
                target_id: Some("local".to_string()),
                effect: DelegationRuleEffect::Allow,
                require_approval: false,
            }],
        };
        let out = evaluate(
            Some(&policy),
            Some(&delegation()),
            DelegationActionKind::ProviderCall,
            "local",
        );
        assert_eq!(out.decision, DelegationDecision::Allowed);
        assert_eq!(out.rule_id.as_deref(), Some("allow-local"));
    }

    #[test]
    fn allow_with_approval_emits_needs_approval() {
        let policy = DelegationPolicySpec {
            default_allow: false,
            rules: vec![DelegationPolicyRuleSpec {
                id: "approve-remote".to_string(),
                action: DelegationActionKind::RemoteExec,
                target_id: None,
                effect: DelegationRuleEffect::Allow,
                require_approval: true,
            }],
        };
        let out = evaluate(
            Some(&policy),
            Some(&delegation()),
            DelegationActionKind::RemoteExec,
            "endpoint-a",
        );
        assert_eq!(out.decision, DelegationDecision::NeedsApproval);
        assert_eq!(out.rule_id.as_deref(), Some("approve-remote"));
    }

    #[test]
    fn policy_is_skipped_when_step_has_no_delegation() {
        let policy = DelegationPolicySpec {
            default_allow: false,
            rules: vec![],
        };
        let out = evaluate(
            Some(&policy),
            None,
            DelegationActionKind::ProviderCall,
            "local",
        );
        assert_eq!(out.decision, DelegationDecision::Allowed);
        assert!(out.rule_id.is_none());
    }
}

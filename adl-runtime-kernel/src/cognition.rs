use std::collections::BTreeSet;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    Capability, CapabilityRequirement, Component, ComponentContext, ComponentError,
    ComponentFactory, ComponentId, ComponentSpec, DeterminismClass, FailurePolicy,
    LifecycleGuarantees, PortSpec, ServiceContract, SERVICE_CONTRACT_SCHEMA,
};

pub const COGNITION_CONTEXT_SCHEMA: &str = "adl.runtime.cognition.context.v1";
pub const COGNITION_DECISION_SCHEMA: &str = "adl.runtime.cognition.decision.v1";
pub const COGNITION_REVIEW_SCHEMA: &str = "adl.runtime.cognition.review.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CognitionContext {
    pub schema: String,
    pub subject_id: String,
    pub policy_hash: String,
    pub evidence_hash: String,
    pub review_hash: Option<String>,
    pub affect_balance: i16,
    pub wellbeing_score: u8,
    pub curiosity_score: u8,
    pub intelligence_confidence: u8,
    pub theory_of_mind_confidence: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CognitionReviewRecord {
    pub schema: String,
    pub review_id: String,
    pub subject_id: String,
    pub policy_hash: String,
    pub reviewer: String,
    pub accepted_risk: bool,
    pub evidence_hash: String,
}

impl CognitionReviewRecord {
    pub fn hash(&self) -> Result<String, CognitionError> {
        validate_review(self)?;
        canonical_hash(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MoralAffectWellbeingPolicy {
    pub policy_hash: String,
    pub min_wellbeing_score: u8,
    pub max_affect_abs: u8,
    pub require_review_below_wellbeing: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CuriosityIntelligenceTheoryPolicy {
    pub policy_hash: String,
    pub min_curiosity_score: u8,
    pub min_intelligence_confidence: u8,
    pub min_theory_of_mind_confidence: u8,
    pub require_review_below_confidence: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CognitionDisposition {
    Allow,
    ReviewRequired,
    Refuse,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CognitionDecision {
    pub schema: String,
    pub surface: CognitionSurface,
    pub subject_id: String,
    pub disposition: CognitionDisposition,
    pub reasons: BTreeSet<String>,
    pub policy_hash: String,
    pub evidence_hash: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CognitionSurface {
    MoralAffectWellbeing,
    CuriosityIntelligenceTheoryOfMind,
}

pub struct GovernedCognitionAdapter;

impl GovernedCognitionAdapter {
    pub fn evaluate_moral_affect_wellbeing(
        context: &CognitionContext,
        policy: &MoralAffectWellbeingPolicy,
        review: Option<&CognitionReviewRecord>,
    ) -> Result<CognitionDecision, CognitionError> {
        validate_context(context)?;
        validate_moral_policy(policy)?;
        validate_policy_binding(context, &policy.policy_hash)?;
        let mut reasons = BTreeSet::new();
        if context.wellbeing_score < policy.min_wellbeing_score {
            reasons.insert("wellbeing_below_minimum".to_owned());
        }
        if context.affect_balance.unsigned_abs() > u16::from(policy.max_affect_abs) {
            reasons.insert("affect_outside_bounds".to_owned());
        }
        Ok(decision(
            context,
            CognitionSurface::MoralAffectWellbeing,
            disposition_with_review(
                context,
                review,
                &policy.policy_hash,
                !reasons.is_empty(),
                context.wellbeing_score < policy.require_review_below_wellbeing,
            )?,
            reasons,
        ))
    }

    pub fn evaluate_curiosity_intelligence_theory_of_mind(
        context: &CognitionContext,
        policy: &CuriosityIntelligenceTheoryPolicy,
        review: Option<&CognitionReviewRecord>,
    ) -> Result<CognitionDecision, CognitionError> {
        validate_context(context)?;
        validate_curiosity_policy(policy)?;
        validate_policy_binding(context, &policy.policy_hash)?;
        let mut reasons = BTreeSet::new();
        if context.curiosity_score < policy.min_curiosity_score {
            reasons.insert("curiosity_below_minimum".to_owned());
        }
        if context.intelligence_confidence < policy.min_intelligence_confidence {
            reasons.insert("intelligence_confidence_below_minimum".to_owned());
        }
        if context.theory_of_mind_confidence < policy.min_theory_of_mind_confidence {
            reasons.insert("theory_of_mind_confidence_below_minimum".to_owned());
        }
        let confidence_floor = context
            .intelligence_confidence
            .min(context.theory_of_mind_confidence);
        Ok(decision(
            context,
            CognitionSurface::CuriosityIntelligenceTheoryOfMind,
            disposition_with_review(
                context,
                review,
                &policy.policy_hash,
                !reasons.is_empty(),
                confidence_floor < policy.require_review_below_confidence,
            )?,
            reasons,
        ))
    }
}

fn disposition_with_review(
    context: &CognitionContext,
    review: Option<&CognitionReviewRecord>,
    policy_hash: &str,
    failing: bool,
    review_required: bool,
) -> Result<CognitionDisposition, CognitionError> {
    if !failing {
        return Ok(CognitionDisposition::Allow);
    }
    if review_required {
        let review = review.ok_or(CognitionError::ReviewRequired)?;
        validate_review_binding(context, review, policy_hash)?;
        if review.accepted_risk {
            return Ok(CognitionDisposition::ReviewRequired);
        }
    }
    Ok(CognitionDisposition::Refuse)
}

fn decision(
    context: &CognitionContext,
    surface: CognitionSurface,
    disposition: CognitionDisposition,
    reasons: BTreeSet<String>,
) -> CognitionDecision {
    CognitionDecision {
        schema: COGNITION_DECISION_SCHEMA.to_owned(),
        surface,
        subject_id: context.subject_id.clone(),
        disposition,
        reasons,
        policy_hash: context.policy_hash.clone(),
        evidence_hash: context.evidence_hash.clone(),
    }
}

#[derive(Clone)]
pub struct CognitionComponentFactory {
    spec: ComponentSpec,
}

struct CognitionComponent;

#[async_trait::async_trait]
impl Component for CognitionComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        context.cancellation.cancelled().await;
        Ok(())
    }
}

impl ComponentFactory for CognitionComponentFactory {
    fn spec(&self) -> ComponentSpec {
        self.spec.clone()
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(CognitionComponent)
    }
}

pub fn cognition_component_specs() -> Vec<ComponentSpec> {
    [
        (
            "moral_affect_wellbeing_adapter",
            vec![],
            vec![],
            vec![
                PortSpec::typed::<CognitionContext>("context"),
                PortSpec::typed::<CognitionDecision>("decision"),
            ],
        ),
        (
            "curiosity_intelligence_theory_of_mind_adapter",
            vec![ComponentId::new("moral_affect_wellbeing_adapter")],
            vec![PortSpec::typed::<CognitionContext>("context")],
            vec![PortSpec::typed::<CognitionDecision>("decision")],
        ),
        (
            "cognition_review_record",
            vec![ComponentId::new(
                "curiosity_intelligence_theory_of_mind_adapter",
            )],
            vec![PortSpec::typed::<CognitionDecision>("decision")],
            vec![PortSpec::typed::<CognitionReviewRecord>("review")],
        ),
    ]
    .into_iter()
    .map(|(id, dependencies, inputs, outputs)| ComponentSpec {
        id: ComponentId::new(id),
        dependencies,
        inputs,
        outputs,
        failure_policy: FailurePolicy::Fatal,
    })
    .collect()
}

pub fn cognition_component_factories() -> Vec<CognitionComponentFactory> {
    cognition_component_specs()
        .into_iter()
        .map(|spec| CognitionComponentFactory { spec })
        .collect()
}

pub fn cognition_service_contracts() -> Vec<ServiceContract> {
    cognition_component_specs()
        .into_iter()
        .map(|spec| {
            let name = spec.id.as_str().to_owned();
            ServiceContract {
                schema: SERVICE_CONTRACT_SCHEMA.to_owned(),
                component: spec.id,
                service: name.clone(),
                version: Version::new(1, 0, 0),
                config_schema: format!("adl.runtime.{name}.config.v1"),
                determinism: DeterminismClass::DeterministicCore,
                lifecycle: LifecycleGuarantees {
                    readiness_required: true,
                    bounded_shutdown_millis: 1_000,
                    restart_safe: true,
                    idempotent_start: true,
                },
                provides: vec![Capability {
                    name: format!("cognition.{name}"),
                    version: Version::new(1, 0, 0),
                }],
                requires: match name.as_str() {
                    "moral_affect_wellbeing_adapter" => {
                        vec![requirement("governance.freedom_gate")]
                    }
                    "curiosity_intelligence_theory_of_mind_adapter" => vec![
                        requirement("cognition.moral_affect_wellbeing_adapter"),
                        optional_requirement("reasoning.evaluation_feedback"),
                    ],
                    "cognition_review_record" => vec![requirement(
                        "cognition.curiosity_intelligence_theory_of_mind_adapter",
                    )],
                    _ => vec![],
                },
                inputs: spec.inputs,
                outputs: spec.outputs,
                failure_policy: spec.failure_policy,
            }
        })
        .collect()
}

fn requirement(name: &str) -> CapabilityRequirement {
    CapabilityRequirement {
        name: name.to_owned(),
        version: VersionReq::parse("^1").expect("static semver requirement"),
        optional: false,
    }
}

fn optional_requirement(name: &str) -> CapabilityRequirement {
    CapabilityRequirement {
        optional: true,
        ..requirement(name)
    }
}

fn validate_context(context: &CognitionContext) -> Result<(), CognitionError> {
    if context.schema != COGNITION_CONTEXT_SCHEMA
        || !safe_id(&context.subject_id)
        || !is_hash(&context.policy_hash)
        || !is_hash(&context.evidence_hash)
        || context.affect_balance < -100
        || context.affect_balance > 100
        || context
            .review_hash
            .as_deref()
            .is_some_and(|hash| !is_hash(hash))
        || !score(context.wellbeing_score)
        || !score(context.curiosity_score)
        || !score(context.intelligence_confidence)
        || !score(context.theory_of_mind_confidence)
    {
        return Err(CognitionError::InvalidContext);
    }
    Ok(())
}

fn validate_review(review: &CognitionReviewRecord) -> Result<(), CognitionError> {
    if review.schema != COGNITION_REVIEW_SCHEMA
        || !safe_id(&review.review_id)
        || !safe_id(&review.subject_id)
        || !safe_id(&review.reviewer)
        || !is_hash(&review.policy_hash)
        || !is_hash(&review.evidence_hash)
    {
        return Err(CognitionError::InvalidReview);
    }
    Ok(())
}

fn validate_review_binding(
    context: &CognitionContext,
    review: &CognitionReviewRecord,
    policy_hash: &str,
) -> Result<(), CognitionError> {
    validate_review(review)?;
    if review.subject_id != context.subject_id
        || review.policy_hash != policy_hash
        || review.evidence_hash != context.evidence_hash
        || context.review_hash.as_deref() != Some(review.hash()?.as_str())
    {
        return Err(CognitionError::InvalidReview);
    }
    Ok(())
}

fn validate_moral_policy(policy: &MoralAffectWellbeingPolicy) -> Result<(), CognitionError> {
    if !is_hash(&policy.policy_hash)
        || policy.max_affect_abs > 100
        || !score(policy.min_wellbeing_score)
        || !score(policy.require_review_below_wellbeing)
        || policy.require_review_below_wellbeing > policy.min_wellbeing_score
    {
        return Err(CognitionError::InvalidPolicy);
    }
    Ok(())
}

fn validate_curiosity_policy(
    policy: &CuriosityIntelligenceTheoryPolicy,
) -> Result<(), CognitionError> {
    if !is_hash(&policy.policy_hash)
        || !score(policy.min_curiosity_score)
        || !score(policy.min_intelligence_confidence)
        || !score(policy.min_theory_of_mind_confidence)
        || !score(policy.require_review_below_confidence)
        || policy.require_review_below_confidence > policy.min_intelligence_confidence
        || policy.require_review_below_confidence > policy.min_theory_of_mind_confidence
    {
        return Err(CognitionError::InvalidPolicy);
    }
    Ok(())
}

fn validate_policy_binding(
    context: &CognitionContext,
    policy_hash: &str,
) -> Result<(), CognitionError> {
    if context.policy_hash == policy_hash {
        Ok(())
    } else {
        Err(CognitionError::PolicyMismatch)
    }
}

fn canonical_hash<T: Serialize + ?Sized>(value: &T) -> Result<String, CognitionError> {
    let bytes =
        serde_json::to_vec(value).map_err(|error| CognitionError::Encoding(error.to_string()))?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

fn safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 96
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
}

fn is_hash(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn score(value: u8) -> bool {
    value <= 100
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CognitionError {
    #[error("cognition context is invalid")]
    InvalidContext,
    #[error("cognition policy is invalid")]
    InvalidPolicy,
    #[error("cognition policy does not match context")]
    PolicyMismatch,
    #[error("cognition review is required")]
    ReviewRequired,
    #[error("cognition review is invalid")]
    InvalidReview,
    #[error("cognition encoding failed: {0}")]
    Encoding(String),
}

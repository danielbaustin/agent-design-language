use std::collections::{BTreeMap, BTreeSet};

use markdown::mdast::Node;
use markdown::{to_mdast, ParseOptions};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString};

use crate::error::{ErrorCode, Result, V2Error};
use crate::model::LifecyclePhase;

macro_rules! closed_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Display,
            EnumString, AsRefStr, EnumIter,
        )]
        #[serde(rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        pub enum $name { $($variant),+ }
    };
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CardKind {
    Sip,
    Stp,
    Spp,
    Vpp,
    Srp,
    Sor,
}

impl CardKind {
    pub fn title(self) -> &'static str {
        match self {
            Self::Sip => "Structured Intent Prompt",
            Self::Stp => "Structured Task Prompt",
            Self::Spp => "Structured Planning Prompt",
            Self::Vpp => "Validation Planning Prompt",
            Self::Srp => "Structured Review Prompt",
            Self::Sor => "Structured Output Record",
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
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
pub enum CardStatus {
    PrePhase,
    Draft,
    Ready,
    Approved,
    Blocked,
    Superseded,
    Complete,
}

impl CardStatus {
    fn allows(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::PrePhase, Self::Draft | Self::Ready)
                | (Self::Draft, Self::Ready | Self::Blocked | Self::Superseded)
                | (
                    Self::Ready,
                    Self::Approved | Self::Blocked | Self::Superseded | Self::Complete
                )
                | (
                    Self::Approved,
                    Self::Blocked | Self::Superseded | Self::Complete
                )
                | (Self::Blocked, Self::Draft | Self::Ready | Self::Superseded)
        )
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
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
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(
    Debug,
    Clone,
    Copy,
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
pub enum PlanningProfile {
    Small,
    Medium,
    Large,
    Migration,
}

impl PlanningProfile {
    pub fn estimates(self) -> (ExecutionEstimates, u64) {
        match self {
            Self::Small => (
                ExecutionEstimates {
                    elapsed_seconds: 7_200,
                    total_tokens: 40_000,
                    validation_seconds: 1_200,
                },
                10_000,
            ),
            Self::Medium => (
                ExecutionEstimates {
                    elapsed_seconds: 21_600,
                    total_tokens: 80_000,
                    validation_seconds: 3_600,
                },
                25_000,
            ),
            Self::Large => (
                ExecutionEstimates {
                    elapsed_seconds: 43_200,
                    total_tokens: 140_000,
                    validation_seconds: 7_200,
                },
                50_000,
            ),
            Self::Migration => (
                ExecutionEstimates {
                    elapsed_seconds: 86_400,
                    total_tokens: 240_000,
                    validation_seconds: 21_600,
                },
                100_000,
            ),
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
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
pub enum FindingDisposition {
    Open,
    Fixed,
    AcceptedRisk,
    OutOfScope,
}

closed_enum!(ResourceProfile {
    Small,
    Medium,
    Large
});
closed_enum!(FindingSeverity { P0, P1, P2, P3 });
closed_enum!(ReviewResult {
    PreReview,
    Pass,
    ChangesRequired,
    Blocked
});
closed_enum!(EvidenceOutcome {
    Passed,
    Failed,
    Blocked,
    Waiting,
    Deferred,
    SkippedNonGoal
});
closed_enum!(IntegrationState {
    NotStarted,
    WorktreeOnly,
    PrOpen,
    Merged,
    ClosedNoPr
});
closed_enum!(PublicationState {
    NotPublished,
    Draft,
    Ready,
    Closed
});
closed_enum!(MergeState {
    NotMerged,
    Pending,
    Merged,
    ClosedUnmerged
});
closed_enum!(CloseoutState {
    NotStarted,
    InProgress,
    Complete,
    Blocked
});

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CardIdentity {
    pub schema_version: String,
    pub template_version: String,
    pub issue: u64,
    pub repository: String,
    pub title: String,
    pub slug: String,
    pub version: String,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SipValues {
    pub goal: String,
    pub required_outcome: String,
    pub declared_scope: Vec<String>,
    pub authority_boundary: Vec<String>,
    pub initial_assumptions: Vec<String>,
    pub operator_constraints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StpValues {
    pub task_boundary: String,
    pub deliverables: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub dependencies: Vec<String>,
    pub repo_inputs: Vec<String>,
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PlanStep {
    pub id: String,
    pub action: String,
    pub acceptance_ids: Vec<String>,
    pub status: StepStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionEstimates {
    pub elapsed_seconds: u64,
    pub total_tokens: u64,
    pub validation_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SppValues {
    pub plan_revision: u64,
    pub summary: String,
    pub steps: Vec<PlanStep>,
    pub affected_areas: Vec<String>,
    pub invariants: Vec<String>,
    pub risks: Vec<String>,
    pub execution_estimates: ExecutionEstimates,
    pub stop_conditions: Vec<String>,
    pub replan_triggers: Vec<String>,
    pub design_ref: String,
    pub design_digest: String,
    pub diagram_ref: String,
    pub diagram_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ValidationLane {
    pub lane: String,
    pub proof_role: String,
    pub acceptance_ids: Vec<String>,
    pub deterministic: bool,
    pub resource_profile: ResourceProfile,
    pub budget_seconds: u64,
    pub budget_tokens: u64,
    pub argv: Vec<String>,
    pub parallel_group: String,
    pub defer_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct VppValues {
    pub summary: String,
    pub lanes: Vec<ValidationLane>,
    pub planned_validation_seconds: u64,
    pub planned_validation_tokens: u64,
    pub failure_policy: String,
    pub design_ref: String,
    pub design_digest: String,
    pub diagram_ref: String,
    pub diagram_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewFinding {
    pub id: String,
    pub severity: FindingSeverity,
    pub summary: String,
    pub actionable: bool,
    #[serde(default = "default_true")]
    pub in_scope: bool,
    pub disposition: FindingDisposition,
    #[serde(default)]
    pub fix_revision: Option<String>,
    #[serde(default)]
    pub route: Option<String>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SrpValues {
    pub review_scope: String,
    pub review_revision: Option<String>,
    pub reviewer: Option<String>,
    pub review_prompts: Vec<String>,
    pub findings: Vec<ReviewFinding>,
    pub residual_risk: Vec<String>,
    pub review_result: ReviewResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ValidationResult {
    pub command: Vec<String>,
    pub purpose: String,
    pub outcome: EvidenceOutcome,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SorValues {
    pub summary: String,
    pub actual_changes: Vec<String>,
    pub artifacts: Vec<String>,
    pub actual_validation: Vec<ValidationResult>,
    pub integration_state: IntegrationState,
    pub publication_state: PublicationState,
    pub merge_state: MergeState,
    pub closeout_state: CloseoutState,
    pub follow_ups: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "card_kind", content = "values", rename_all = "lowercase")]
pub enum CardContent {
    Sip(SipValues),
    Stp(StpValues),
    Spp(SppValues),
    Vpp(VppValues),
    Srp(SrpValues),
    Sor(SorValues),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CardValues {
    pub identity: CardIdentity,
    pub status: CardStatus,
    pub content: CardContent,
}

impl CardValues {
    pub fn kind(&self) -> CardKind {
        match self.content {
            CardContent::Sip(_) => CardKind::Sip,
            CardContent::Stp(_) => CardKind::Stp,
            CardContent::Spp(_) => CardKind::Spp,
            CardContent::Vpp(_) => CardKind::Vpp,
            CardContent::Srp(_) => CardKind::Srp,
            CardContent::Sor(_) => CardKind::Sor,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct InitialCardInput {
    pub title: String,
    pub slug: String,
    pub version: String,
    pub goal: String,
    pub required_outcome: String,
    pub declared_scope: Vec<String>,
    pub authority_boundary: Vec<String>,
    pub task_boundary: String,
    pub deliverables: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub dependencies: Vec<String>,
    pub repo_inputs: Vec<String>,
    pub non_goals: Vec<String>,
    pub plan_summary: String,
    pub steps: Vec<PlanStep>,
    pub invariants: Vec<String>,
    pub risks: Vec<String>,
    pub planning_profile: PlanningProfile,
    pub stop_conditions: Vec<String>,
    pub validation_lanes: Vec<ValidationLane>,
    pub failure_policy: String,
    pub review_prompts: Vec<String>,
}

#[derive(
    Debug,
    Clone,
    Copy,
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
pub enum TextField {
    Goal,
    RequiredOutcome,
    TaskBoundary,
    PlanSummary,
    FailurePolicy,
    ReviewScope,
    SorSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum SemanticOperation {
    Replan {
        field: TextField,
        value: String,
    },
    SetField {
        field: TextField,
        value: String,
    },
    AppendReference {
        value: String,
    },
    RecordValidation {
        result: ValidationResult,
    },
    RecordFinding {
        finding: ReviewFinding,
    },
    RecordReview {
        reviewer: String,
        revision: String,
        result: ReviewResult,
        residual_risk: Vec<String>,
    },
    DisposeFinding {
        finding_id: String,
        disposition: FindingDisposition,
    },
    AdvanceStatus {
        status: CardStatus,
    },
    RecordExecution {
        summary: String,
        changes: Vec<String>,
        artifacts: Vec<String>,
    },
    RecordCloseout {
        integration_state: IntegrationState,
        publication_state: PublicationState,
        merge_state: MergeState,
        closeout_state: CloseoutState,
    },
    RecordPublication {
        state: PublicationState,
    },
    RecordMerge {
        state: MergeState,
    },
    AdvancePhase {
        phase: LifecyclePhase,
    },
}

#[derive(Debug, Clone)]
pub struct RenderedCard {
    pub markdown: String,
    pub values_digest: String,
    pub rendered_digest: String,
    pub ast_digest: String,
}

struct CardTemplate {
    version: &'static str,
    headings: &'static [&'static str],
}

fn template_for(kind: CardKind) -> CardTemplate {
    let headings = match kind {
        CardKind::Sip => &[
            "Goal",
            "Required Outcome",
            "Scope",
            "Authority",
            "Assumptions",
            "Operator Constraints",
        ][..],
        CardKind::Stp => &[
            "Task",
            "Deliverables",
            "Acceptance",
            "Dependencies",
            "Inputs",
            "Non Goals",
        ][..],
        CardKind::Spp => &[
            "Summary",
            "Plan",
            "Steps",
            "Invariants",
            "Risks",
            "Estimates",
            "Design",
            "Diagram",
            "Stop Conditions",
            "Handoff",
        ][..],
        CardKind::Vpp => &[
            "Summary",
            "Lane Inputs",
            "Selected Lanes",
            "Parallelization",
            "Budgets",
            "Commands",
            "Failure Semantics",
            "Handoff",
        ][..],
        CardKind::Srp => &[
            "Scope",
            "Prompts",
            "Findings",
            "Dispositions",
            "Residual Risk",
            "Review Result",
        ][..],
        CardKind::Sor => &[
            "Summary",
            "Artifacts",
            "Execution",
            "Validation",
            "Integration",
            "Publication",
            "Closeout",
            "Follow Ups",
        ][..],
    };
    CardTemplate {
        version: "1.0.0",
        headings,
    }
}

pub fn initial_cards(
    issue: u64,
    repository: &str,
    design_ref: &str,
    design_digest: &str,
    diagram_ref: &str,
    diagram_digest: &str,
    input: InitialCardInput,
) -> Result<BTreeMap<CardKind, CardValues>> {
    require_input(&input)?;
    let identity = CardIdentity {
        schema_version: "1.0.0".into(),
        template_version: "1.0.0".into(),
        issue,
        repository: repository.into(),
        title: input.title.clone(),
        slug: input.slug.clone(),
        version: input.version.clone(),
        generation: 0,
    };
    let (estimates, planned_validation_tokens) = input.planning_profile.estimates();
    let planned_seconds = estimates.validation_seconds;
    let cards = [
        (
            CardKind::Sip,
            CardStatus::Ready,
            CardContent::Sip(SipValues {
                goal: input.goal,
                required_outcome: input.required_outcome,
                declared_scope: input.declared_scope,
                authority_boundary: input.authority_boundary,
                initial_assumptions: Vec::new(),
                operator_constraints: Vec::new(),
            }),
        ),
        (
            CardKind::Stp,
            CardStatus::Ready,
            CardContent::Stp(StpValues {
                task_boundary: input.task_boundary,
                deliverables: input.deliverables,
                acceptance_criteria: input.acceptance_criteria,
                dependencies: input.dependencies,
                repo_inputs: input.repo_inputs,
                non_goals: input.non_goals,
            }),
        ),
        (
            CardKind::Spp,
            CardStatus::Ready,
            CardContent::Spp(SppValues {
                plan_revision: 1,
                summary: input.plan_summary,
                steps: input.steps,
                affected_areas: Vec::new(),
                invariants: input.invariants,
                risks: input.risks,
                execution_estimates: estimates,
                stop_conditions: input.stop_conditions,
                replan_triggers: Vec::new(),
                design_ref: design_ref.into(),
                design_digest: design_digest.into(),
                diagram_ref: diagram_ref.into(),
                diagram_digest: diagram_digest.into(),
            }),
        ),
        (
            CardKind::Vpp,
            CardStatus::Ready,
            CardContent::Vpp(VppValues {
                summary: "Execute the smallest proving validation DAG.".into(),
                lanes: input.validation_lanes,
                planned_validation_seconds: planned_seconds,
                planned_validation_tokens,
                failure_policy: input.failure_policy,
                design_ref: design_ref.into(),
                design_digest: design_digest.into(),
                diagram_ref: diagram_ref.into(),
                diagram_digest: diagram_digest.into(),
            }),
        ),
        (
            CardKind::Srp,
            CardStatus::PrePhase,
            CardContent::Srp(SrpValues {
                review_scope: "Exact implementation revision before publication.".into(),
                review_revision: None,
                reviewer: None,
                review_prompts: input.review_prompts,
                findings: Vec::new(),
                residual_risk: Vec::new(),
                review_result: ReviewResult::PreReview,
            }),
        ),
        (
            CardKind::Sor,
            CardStatus::PrePhase,
            CardContent::Sor(SorValues {
                summary: "Pre-execution output record.".into(),
                actual_changes: Vec::new(),
                artifacts: Vec::new(),
                actual_validation: Vec::new(),
                integration_state: IntegrationState::NotStarted,
                publication_state: PublicationState::NotPublished,
                merge_state: MergeState::NotMerged,
                closeout_state: CloseoutState::NotStarted,
                follow_ups: Vec::new(),
            }),
        ),
    ]
    .into_iter()
    .map(|(kind, status, content)| {
        (
            kind,
            CardValues {
                identity: identity.clone(),
                status,
                content,
            },
        )
    })
    .collect::<BTreeMap<_, _>>();
    validate_cross_card(
        &cards,
        design_ref,
        design_digest,
        diagram_ref,
        diagram_digest,
    )?;
    Ok(cards)
}

pub fn apply(
    values: &mut CardValues,
    operation: &SemanticOperation,
) -> Result<Option<LifecyclePhase>> {
    match operation {
        SemanticOperation::Replan { field, value } => {
            set_text(values, *field, value.clone())?;
            Ok(None)
        }
        SemanticOperation::SetField { field, value } => {
            set_text(values, *field, value.clone())?;
            Ok(None)
        }
        SemanticOperation::AppendReference { value } => {
            append_reference(values, value.clone())?;
            Ok(None)
        }
        SemanticOperation::RecordValidation { result } => match &mut values.content {
            CardContent::Sor(v) => {
                validate_result(result)?;
                v.actual_validation.push(result.clone());
                Ok(None)
            }
            _ => ownership(values.kind(), "record_validation"),
        },
        SemanticOperation::RecordFinding { finding } => match &mut values.content {
            CardContent::Srp(v) => {
                v.findings.push(finding.clone());
                Ok(None)
            }
            _ => ownership(values.kind(), "record_finding"),
        },
        SemanticOperation::RecordReview {
            reviewer,
            revision,
            result,
            residual_risk,
        } => match &mut values.content {
            CardContent::Srp(v) => {
                if reviewer.trim().is_empty() || revision.trim().is_empty() {
                    return Err(V2Error::new(
                        ErrorCode::InvalidInput,
                        "reviewer and exact revision are required",
                    ));
                }
                v.reviewer = Some(reviewer.clone());
                v.review_revision = Some(revision.clone());
                v.review_result = *result;
                v.residual_risk = residual_risk.clone();
                Ok(None)
            }
            _ => ownership(values.kind(), "record_review"),
        },
        SemanticOperation::DisposeFinding {
            finding_id,
            disposition,
        } => match &mut values.content {
            CardContent::Srp(v) => {
                let finding = v
                    .findings
                    .iter_mut()
                    .find(|f| &f.id == finding_id)
                    .ok_or_else(|| {
                        V2Error::new(ErrorCode::InvalidInput, "finding does not exist")
                    })?;
                finding.disposition = *disposition;
                Ok(None)
            }
            _ => ownership(values.kind(), "dispose_finding"),
        },
        SemanticOperation::AdvanceStatus { status } => {
            if !values.status.allows(*status) {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    format!("card status {} -> {} is not allowed", values.status, status),
                ));
            }
            validate_status_guard(values, *status)?;
            values.status = *status;
            Ok(None)
        }
        SemanticOperation::RecordExecution {
            summary,
            changes,
            artifacts,
        } => match &mut values.content {
            CardContent::Sor(v) => {
                v.summary = summary.clone();
                v.actual_changes.extend(changes.clone());
                v.artifacts.extend(artifacts.clone());
                Ok(None)
            }
            _ => ownership(values.kind(), "record_execution"),
        },
        SemanticOperation::RecordCloseout {
            integration_state,
            publication_state,
            merge_state,
            closeout_state,
        } => match &mut values.content {
            CardContent::Sor(v) => {
                v.integration_state = *integration_state;
                v.publication_state = *publication_state;
                v.merge_state = *merge_state;
                v.closeout_state = *closeout_state;
                Ok(None)
            }
            _ => ownership(values.kind(), "record_closeout"),
        },
        SemanticOperation::RecordPublication { state } => match &mut values.content {
            CardContent::Sor(v) => {
                v.publication_state = *state;
                Ok(None)
            }
            _ => ownership(values.kind(), "record_publication"),
        },
        SemanticOperation::RecordMerge { state } => match &mut values.content {
            CardContent::Sor(v) => {
                v.merge_state = *state;
                Ok(None)
            }
            _ => ownership(values.kind(), "record_merge"),
        },
        SemanticOperation::AdvancePhase { phase } => Ok(Some(*phase)),
    }
}

fn validate_status_guard(values: &CardValues, next: CardStatus) -> Result<()> {
    if matches!(next, CardStatus::Approved | CardStatus::Complete) {
        if let CardContent::Srp(srp) = &values.content {
            if srp.review_result != ReviewResult::Pass
                || srp
                    .review_revision
                    .as_deref()
                    .unwrap_or_default()
                    .is_empty()
                || srp.reviewer.as_deref().unwrap_or_default().is_empty()
                || srp.findings.iter().any(|finding| {
                    finding.actionable && finding.disposition == FindingDisposition::Open
                })
            {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "SRP cannot be approved/complete without current resolved review evidence",
                ));
            }
        }
    }
    if next == CardStatus::Complete {
        if let CardContent::Sor(sor) = &values.content {
            let terminal_integration = matches!(
                sor.integration_state,
                IntegrationState::Merged | IntegrationState::ClosedNoPr
            );
            let terminal_merge = matches!(
                sor.merge_state,
                MergeState::Merged | MergeState::ClosedUnmerged
            );
            let terminal_validation = !sor.actual_validation.is_empty()
                && sor.actual_validation.iter().all(|result| {
                    validate_result(result).is_ok()
                        && matches!(
                            result.outcome,
                            EvidenceOutcome::Passed | EvidenceOutcome::SkippedNonGoal
                        )
                });
            if !terminal_integration
                || !terminal_merge
                || sor.closeout_state != CloseoutState::Complete
                || !terminal_validation
            {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "SOR cannot be complete before terminal validation/integration/closeout truth",
                ));
            }
        }
    }
    Ok(())
}

fn validate_result(result: &ValidationResult) -> Result<()> {
    if result.command.is_empty()
        || result.command.iter().any(|part| part.trim().is_empty())
        || result.purpose.trim().is_empty()
        || result.evidence_ref.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "validation result requires argv, purpose, and evidence reference",
        ));
    }
    Ok(())
}

pub fn render(values: &CardValues) -> Result<RenderedCard> {
    validate_values(values)?;
    let kind = values.kind();
    let sections = sections(values);
    let template = template_for(kind);
    if values.identity.template_version != template.version
        || sections
            .iter()
            .map(|(heading, _)| *heading)
            .collect::<Vec<_>>()
            != template.headings
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "card template version/shape mismatch",
        ));
    }
    let mut markdown = format!(
        "# {}\n\nTemplate: {}\n\nIssue: {}\n\nRepository: {}\n\nCard: {}\n\nStatus: {}\n",
        kind.title(),
        template.version,
        values.identity.issue,
        values.identity.repository,
        kind,
        values.status
    );
    for (heading, body) in &sections {
        markdown.push_str(&format!("\n## {heading}\n\n{}\n", body.trim()));
    }
    let ast = to_mdast(&markdown, &ParseOptions::gfm()).map_err(|message| {
        V2Error::new(
            ErrorCode::CardInvalid,
            format!("markdown parse failed: {message}"),
        )
    })?;
    validate_mdast(&ast, &sections)?;
    let values_bytes = serde_json::to_vec(values)?;
    Ok(RenderedCard {
        values_digest: digest(&values_bytes),
        rendered_digest: digest(markdown.as_bytes()),
        ast_digest: digest(format!("{ast:?}").as_bytes()),
        markdown,
    })
}

pub fn validate_cross_card(
    cards: &BTreeMap<CardKind, CardValues>,
    design_ref: &str,
    design_digest: &str,
    diagram_ref: &str,
    diagram_digest: &str,
) -> Result<()> {
    if cards.len() != 6 {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "all six cards are required",
        ));
    }
    let first = cards.values().next().expect("six cards");
    if cards.iter().any(|(kind, card)| {
        card.kind() != *kind
            || card.identity.issue != first.identity.issue
            || card.identity.repository != first.identity.repository
            || card.identity.slug != first.identity.slug
            || card.identity.version != first.identity.version
    }) {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "cross-card identity mismatch",
        ));
    }
    let (stp, spp, vpp) = match (
        &cards[&CardKind::Stp].content,
        &cards[&CardKind::Spp].content,
        &cards[&CardKind::Vpp].content,
    ) {
        (CardContent::Stp(stp), CardContent::Spp(spp), CardContent::Vpp(vpp)) => (stp, spp, vpp),
        _ => unreachable!(),
    };
    if spp.design_ref != design_ref
        || spp.design_digest != design_digest
        || spp.diagram_ref != diagram_ref
        || spp.diagram_digest != diagram_digest
        || vpp.design_ref != design_ref
        || vpp.design_digest != design_digest
        || vpp.diagram_ref != diagram_ref
        || vpp.diagram_digest != diagram_digest
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "design/diagram references are stale",
        ));
    }
    let acceptance_ids: Vec<String> = (1..=stp.acceptance_criteria.len())
        .map(|n| format!("AC-{n}"))
        .collect();
    let mapped: BTreeSet<_> = spp
        .steps
        .iter()
        .flat_map(|step| step.acceptance_ids.iter())
        .cloned()
        .collect();
    if acceptance_ids.iter().any(|id| !mapped.contains(id)) {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "acceptance criterion lacks a plan step",
        ));
    }
    let proven: BTreeSet<_> = vpp
        .lanes
        .iter()
        .flat_map(|lane| lane.acceptance_ids.iter())
        .cloned()
        .collect();
    if acceptance_ids.iter().any(|id| !proven.contains(id)) {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "acceptance criterion lacks a VPP proof role",
        ));
    }
    let lane_seconds: u64 = vpp.lanes.iter().map(|lane| lane.budget_seconds).sum();
    let lane_tokens: u64 = vpp.lanes.iter().map(|lane| lane.budget_tokens).sum();
    if vpp.lanes.is_empty()
        || vpp
            .lanes
            .iter()
            .any(|lane| lane.argv.is_empty() || lane.budget_seconds == 0 || lane.budget_tokens == 0)
        || lane_seconds > vpp.planned_validation_seconds
        || lane_tokens > vpp.planned_validation_tokens
        || vpp.planned_validation_tokens == 0
        || spp.execution_estimates.elapsed_seconds == 0
        || spp.execution_estimates.total_tokens == 0
        || spp.execution_estimates.validation_seconds == 0
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "SPP/VPP automatic budgets or lanes are incomplete",
        ));
    }
    Ok(())
}

fn require_input(input: &InitialCardInput) -> Result<()> {
    if [
        &input.title,
        &input.slug,
        &input.version,
        &input.goal,
        &input.required_outcome,
        &input.task_boundary,
        &input.plan_summary,
        &input.failure_policy,
    ]
    .iter()
    .any(|v| v.trim().is_empty())
        || input.declared_scope.is_empty()
        || input.authority_boundary.is_empty()
        || input.deliverables.is_empty()
        || input.acceptance_criteria.is_empty()
        || input.steps.is_empty()
        || input.stop_conditions.is_empty()
        || input.review_prompts.is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "initial typed card input is incomplete",
        ));
    }
    Ok(())
}

fn set_text(values: &mut CardValues, field: TextField, value: String) -> Result<()> {
    if value.trim().is_empty() {
        return Err(V2Error::new(ErrorCode::CardInvalid, "empty field value"));
    }
    match (&mut values.content, field) {
        (CardContent::Sip(v), TextField::Goal) => v.goal = value,
        (CardContent::Sip(v), TextField::RequiredOutcome) => v.required_outcome = value,
        (CardContent::Stp(v), TextField::TaskBoundary) => v.task_boundary = value,
        (CardContent::Spp(v), TextField::PlanSummary) => {
            v.summary = value;
            v.plan_revision += 1;
        }
        (CardContent::Vpp(v), TextField::FailurePolicy) => v.failure_policy = value,
        (CardContent::Srp(v), TextField::ReviewScope) => v.review_scope = value,
        (CardContent::Sor(v), TextField::SorSummary) => v.summary = value,
        _ => return ownership(values.kind(), field.as_ref()),
    }
    Ok(())
}

fn append_reference(values: &mut CardValues, value: String) -> Result<()> {
    match &mut values.content {
        CardContent::Stp(v) => v.repo_inputs.push(value),
        CardContent::Srp(v) => v.residual_risk.push(value),
        CardContent::Sor(v) => v.follow_ups.push(value),
        _ => return ownership(values.kind(), "append_reference"),
    }
    Ok(())
}

fn ownership<T>(kind: CardKind, operation: &str) -> Result<T> {
    Err(V2Error::new(
        ErrorCode::FieldOwnership,
        format!("{kind} does not own {operation}"),
    ))
}

fn validate_values(values: &CardValues) -> Result<()> {
    if values.identity.schema_version != "1.0.0"
        || values.identity.template_version != "1.0.0"
        || values.identity.issue == 0
        || values.identity.repository.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "invalid card identity/schema",
        ));
    }
    if let CardContent::Sor(sor) = &values.content {
        for result in &sor.actual_validation {
            validate_result(result)?;
        }
    }
    Ok(())
}

fn sections(values: &CardValues) -> Vec<(&'static str, String)> {
    match &values.content {
        CardContent::Sip(v) => vec![
            ("Goal", v.goal.clone()),
            ("Required Outcome", v.required_outcome.clone()),
            ("Scope", bullets(&v.declared_scope)),
            ("Authority", bullets(&v.authority_boundary)),
            ("Assumptions", bullets(&v.initial_assumptions)),
            ("Operator Constraints", bullets(&v.operator_constraints)),
        ],
        CardContent::Stp(v) => vec![
            ("Task", v.task_boundary.clone()),
            ("Deliverables", bullets(&v.deliverables)),
            ("Acceptance", numbered(&v.acceptance_criteria)),
            ("Dependencies", bullets(&v.dependencies)),
            ("Inputs", bullets(&v.repo_inputs)),
            ("Non Goals", bullets(&v.non_goals)),
        ],
        CardContent::Spp(v) => vec![
            ("Summary", v.summary.clone()),
            ("Plan", format!("Revision {}", v.plan_revision)),
            (
                "Steps",
                serde_json::to_string_pretty(&v.steps).expect("steps"),
            ),
            ("Invariants", bullets(&v.invariants)),
            ("Risks", bullets(&v.risks)),
            (
                "Estimates",
                serde_json::to_string_pretty(&v.execution_estimates).expect("estimates"),
            ),
            (
                "Design",
                format!("{}\n\nDigest: {}", v.design_ref, v.design_digest),
            ),
            (
                "Diagram",
                format!("{}\n\nDigest: {}", v.diagram_ref, v.diagram_digest),
            ),
            ("Stop Conditions", bullets(&v.stop_conditions)),
            ("Handoff", "Proceed only after doctor readiness.".into()),
        ],
        CardContent::Vpp(v) => vec![
            ("Summary", v.summary.clone()),
            (
                "Lane Inputs",
                format!("Design: {}\n\nDiagram: {}", v.design_ref, v.diagram_ref),
            ),
            (
                "Selected Lanes",
                serde_json::to_string_pretty(&v.lanes).expect("lanes"),
            ),
            (
                "Parallelization",
                "Only declared parallel groups may overlap.".into(),
            ),
            (
                "Budgets",
                format!(
                    "Seconds: {}\n\nTokens: {}",
                    v.planned_validation_seconds, v.planned_validation_tokens
                ),
            ),
            (
                "Commands",
                v.lanes
                    .iter()
                    .map(|lane| format!("- `{}`", lane.argv.join(" ")))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            ("Failure Semantics", v.failure_policy.clone()),
            (
                "Handoff",
                "Retain typed evidence before convergence.".into(),
            ),
        ],
        CardContent::Srp(v) => vec![
            ("Scope", v.review_scope.clone()),
            ("Prompts", bullets(&v.review_prompts)),
            (
                "Findings",
                serde_json::to_string_pretty(&v.findings).expect("findings"),
            ),
            (
                "Dispositions",
                "Every actionable finding requires a terminal disposition.".into(),
            ),
            ("Residual Risk", bullets(&v.residual_risk)),
            (
                "Review Result",
                format!(
                    "Revision: {:?}\n\nReviewer: {:?}\n\nResult: {}",
                    v.review_revision, v.reviewer, v.review_result
                ),
            ),
        ],
        CardContent::Sor(v) => vec![
            ("Summary", v.summary.clone()),
            ("Artifacts", bullets(&v.artifacts)),
            ("Execution", bullets(&v.actual_changes)),
            (
                "Validation",
                serde_json::to_string_pretty(&v.actual_validation).expect("validation"),
            ),
            ("Integration", v.integration_state.to_string()),
            (
                "Publication",
                format!(
                    "Publication: {}\n\nMerge: {}",
                    v.publication_state, v.merge_state
                ),
            ),
            ("Closeout", v.closeout_state.to_string()),
            ("Follow Ups", bullets(&v.follow_ups)),
        ],
    }
}

fn validate_mdast(ast: &Node, expected: &[(&str, String)]) -> Result<()> {
    let children = match ast {
        Node::Root(root) => &root.children,
        _ => return Err(V2Error::new(ErrorCode::CardInvalid, "mdast root missing")),
    };
    let mut headings = Vec::new();
    for child in children {
        if let Node::Heading(heading) = child {
            if heading.depth == 2 {
                let text = heading
                    .children
                    .iter()
                    .filter_map(|node| match node {
                        Node::Text(text) => Some(text.value.as_str()),
                        _ => None,
                    })
                    .collect::<String>();
                headings.push(text);
            }
        }
    }
    let expected_headings: Vec<_> = expected
        .iter()
        .map(|(heading, _)| (*heading).to_string())
        .collect();
    if headings != expected_headings {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "mdast semantic anchors mismatch",
        ));
    }
    Ok(())
}

fn bullets(values: &[String]) -> String {
    if values.is_empty() {
        "- none".into()
    } else {
        values
            .iter()
            .map(|value| format!("- {value}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
fn numbered(values: &[String]) -> String {
    values
        .iter()
        .enumerate()
        .map(|(index, value)| format!("{}. {value}", index + 1))
        .collect::<Vec<_>>()
        .join("\n")
}
pub fn digest(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, IntoEnumIterator};

use crate::cards::{
    initial_cards, render, PlanStep, PlanningProfile, ResourceProfile, StepStatus, ValidationLane,
};
use crate::error::{ErrorCode, Result, V2Error};

macro_rules! closed_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
            JsonSchema, Display, EnumString, AsRefStr, EnumIter)]
        #[serde(rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        pub enum $name { $($variant),+ }
    };
}

closed_enum!(Generation { V1, V2 });

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct GenerationSelector {
    pub schema: String,
    pub default_generation: Generation,
    pub opted_in_issues: BTreeSet<u64>,
}

pub fn select_generation(
    selector: &GenerationSelector,
    issue: u64,
    requested: Option<Generation>,
) -> Result<Generation> {
    if selector.schema != "csdlc.generation_selector.v1" {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "generation selector schema must be csdlc.generation_selector.v1",
        ));
    }
    let selected = requested.unwrap_or(selector.default_generation);
    if selector.default_generation == Generation::V1
        && selected == Generation::V2
        && !selector.opted_in_issues.contains(&issue)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            format!("issue {issue} is not explicitly opted in to C-SDLC v2"),
        ));
    }
    Ok(selected)
}

closed_enum!(SoakScenario {
    DocsOnly,
    SmallRust,
    ValidationFailureRetry,
    ReviewFindingRepair,
    PrCheckFailureRecovery,
    MergeCloseout,
    InterruptInitializedReady,
    InterruptReadyBound,
    InterruptBoundImplemented,
    InterruptImplementedReviewed,
    InterruptReviewedPublished,
    InterruptPublishedMergeReady,
    InterruptMergeCloseoutTransaction,
    DirtyWorktreeRefusal,
    GithubOutageRetry,
});

closed_enum!(ScenarioOutcome {
    Passed,
    Failed,
    Waiting,
    NotRun
});

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ScenarioEvidence {
    pub scenario: SoakScenario,
    pub outcome: ScenarioOutcome,
    pub evidence_refs: Vec<String>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BudgetEvidence {
    pub name: BudgetKind,
    pub measured: f64,
    pub target: Option<f64>,
    pub hard_ceiling: f64,
    pub unit: String,
    pub hard_pass: bool,
    pub review_approved: bool,
    pub qualification: Option<String>,
    pub evidence_ref: String,
}

closed_enum!(BudgetKind {
    ImplementationLoc,
    RustTests,
    LargestInstalledBinary,
    SevenInstalledBinariesTotal,
    CleanReleaseConstruction,
    WarmReleaseConstruction,
    InitPlusDoctorP95,
    BindP95,
    FocusedValidation,
    FullDeterministicValidation,
});

impl BudgetKind {
    pub fn contract(self) -> (&'static str, f64, Option<f64>) {
        match self {
            Self::ImplementationLoc => ("rust_lines", 8_000.0, Some(8_000.0)),
            Self::RustTests => ("tests", 150.0, Some(100.0)),
            Self::LargestInstalledBinary => ("bytes", 15_728_640.0, None),
            Self::SevenInstalledBinariesTotal => ("bytes", 73_400_320.0, None),
            Self::CleanReleaseConstruction => ("seconds", 209.275, Some(209.275)),
            Self::WarmReleaseConstruction => ("seconds", 0.8125, Some(0.8125)),
            Self::InitPlusDoctorP95 => ("seconds", 1.0, Some(1.0)),
            Self::BindP95 => ("seconds", 2.0, Some(2.0)),
            Self::FocusedValidation => ("seconds", 120.0, Some(120.0)),
            Self::FullDeterministicValidation => ("seconds", 600.0, Some(600.0)),
        }
    }

    fn reviewable(self) -> bool {
        self == Self::ImplementationLoc
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ParityEvidence {
    pub compared_cases: u64,
    pub critical_differences: u64,
    pub explained_noncritical_differences: Vec<String>,
    pub evidence_ref: String,
}

closed_enum!(CutoverDecision {
    Proceed,
    Incubate,
    Stop
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SoakDecisionPacket {
    pub schema: String,
    pub default_generation: Generation,
    pub decision: CutoverDecision,
    pub scenarios: Vec<ScenarioEvidence>,
    pub budgets: Vec<BudgetEvidence>,
    pub parity: ParityEvidence,
    pub blockers: Vec<String>,
    pub residual_risks: Vec<String>,
    pub rollback_window_started: bool,
    pub importer_expiry_started: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SoakEvidenceInput {
    pub schema: String,
    pub default_generation: Generation,
    pub scenarios: Vec<ScenarioEvidence>,
    pub budgets: Vec<BudgetEvidence>,
    pub parity: ParityEvidence,
    pub residual_risks: Vec<String>,
}

pub fn decide_from_evidence(input: SoakEvidenceInput) -> Result<SoakDecisionPacket> {
    if input.schema != "csdlc.soak_evidence.v1" || input.default_generation != Generation::V1 {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "soak evidence must use schema v1 with v1 default",
        ));
    }
    Ok(decide_cutover(
        input.scenarios,
        input.budgets,
        input.parity,
        input.residual_risks,
    ))
}

pub fn decide_cutover(
    scenarios: Vec<ScenarioEvidence>,
    budgets: Vec<BudgetEvidence>,
    parity: ParityEvidence,
    residual_risks: Vec<String>,
) -> SoakDecisionPacket {
    let observed = scenarios
        .iter()
        .map(|item| item.scenario)
        .collect::<BTreeSet<_>>();
    let required = SoakScenario::iter().collect::<BTreeSet<_>>();
    let budget_names = budgets.iter().map(|item| item.name).collect::<Vec<_>>();
    let budget_set = budget_names.iter().copied().collect::<BTreeSet<_>>();
    let mut blockers = Vec::new();
    if budget_names.len() != budget_set.len() {
        blockers.push("budget evidence contains duplicate categories".into());
    }
    for missing in BudgetKind::iter()
        .collect::<BTreeSet<_>>()
        .difference(&budget_set)
    {
        blockers.push(format!("required budget {missing} has no evidence"));
    }
    for missing in required.difference(&observed) {
        blockers.push(format!("required scenario {missing} has no evidence"));
    }
    for item in &scenarios {
        if item.outcome != ScenarioOutcome::Passed {
            blockers.push(format!("scenario {} is {}", item.scenario, item.outcome));
        }
        if item.evidence_refs.is_empty() {
            blockers.push(format!(
                "scenario {} has no evidence reference",
                item.scenario
            ));
        }
    }
    for item in &budgets {
        let (unit, ceiling, target) = item.name.contract();
        if !item.measured.is_finite()
            || !item.hard_ceiling.is_finite()
            || item.measured < 0.0
            || item.hard_ceiling < 0.0
            || item.unit != unit
            || item.hard_ceiling != ceiling
            || item.target != target
        {
            blockers.push(format!("budget {} is malformed", item.name));
        } else if (!item.hard_pass || item.measured > item.hard_ceiling)
            && !(item.name.reviewable()
                && item.review_approved
                && item
                    .qualification
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty()))
        {
            blockers.push(format!("budget {} is not satisfied", item.name));
        }
        if item.evidence_ref.trim().is_empty() {
            blockers.push(format!("budget {} has no evidence reference", item.name));
        }
    }
    if parity.critical_differences != 0 {
        blockers.push(format!(
            "normalized parity has {} critical differences",
            parity.critical_differences
        ));
    }
    if parity.compared_cases == 0 || parity.evidence_ref.trim().is_empty() {
        blockers.push("normalized parity evidence is missing".into());
    }

    let hard_failure = scenarios
        .iter()
        .any(|item| item.outcome == ScenarioOutcome::Failed)
        || budgets.iter().any(|item| {
            !item.measured.is_finite()
                || !item.hard_ceiling.is_finite()
                || item.measured < 0.0
                || (!item.name.reviewable()
                    && (!item.hard_pass || item.measured > item.hard_ceiling))
        })
        || parity.critical_differences != 0;
    let decision = if blockers.is_empty() {
        CutoverDecision::Proceed
    } else if hard_failure {
        CutoverDecision::Stop
    } else {
        CutoverDecision::Incubate
    };
    SoakDecisionPacket {
        schema: "csdlc.soak_decision.v1".into(),
        default_generation: Generation::V1,
        decision,
        scenarios,
        budgets,
        parity,
        blockers,
        residual_risks,
        rollback_window_started: false,
        importer_expiry_started: false,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SamplePacket {
    pub issue: u64,
    pub slug: String,
    pub generation: Generation,
    pub root: PathBuf,
    pub card_paths: BTreeMap<String, PathBuf>,
    pub design_path: PathBuf,
    pub diagram_path: PathBuf,
    pub execution_evidence: String,
}

struct SampleDefinition {
    issue: u64,
    slug: &'static str,
    title: &'static str,
    goal: &'static str,
    scenario: &'static str,
}

const SAMPLES: [SampleDefinition; 3] = [
    SampleDefinition {
        issue: 9_001,
        slug: "docs-only",
        title: "Gate 9 sample: docs-only lifecycle",
        goal: "Prove automated structured cards and review for a documentation-only change.",
        scenario: "docs-only",
    },
    SampleDefinition {
        issue: 9_002,
        slug: "small-rust",
        title: "Gate 9 sample: small Rust lifecycle",
        goal: "Prove focused PVF selection and closeout for a small standalone Rust change.",
        scenario: "small-rust",
    },
    SampleDefinition {
        issue: 9_003,
        slug: "hostile-recovery",
        title: "Gate 9 sample: hostile failures and recovery",
        goal: "Prove fail-closed retry, interruption, dirty-state, review, and remote recovery.",
        scenario: "hostile-recovery",
    },
];

pub fn generate_sample_packets(repo: &Path, root: &Path) -> Result<Vec<SamplePacket>> {
    crate::registry::validate_native_registry(repo)?;
    fs::create_dir_all(root)?;
    let selector = GenerationSelector {
        schema: "csdlc.generation_selector.v1".into(),
        default_generation: Generation::V1,
        opted_in_issues: SAMPLES.iter().map(|sample| sample.issue).collect(),
    };
    write_json(&root.join("generation-selector.json"), &selector)?;
    let mut packets = Vec::new();
    for sample in &SAMPLES {
        let generation = select_generation(&selector, sample.issue, Some(Generation::V2))?;
        let sample_root = root.join(sample.slug);
        fs::create_dir_all(&sample_root)?;
        let design_path = sample_root.join("design.md");
        let diagram_path = sample_root.join("diagram.mmd");
        let design = format!(
            "# {}\n\n## Boundary\n\n{}\n\n## State-machine proof\n\nThe sample exercises `{}` through typed C-SDLC v2 operations. Cards are generated projections; Rust binaries alone own lifecycle mutation.\n\n## Review questions\n\n- Is the sample bounded and independently reproducible?\n- Do failures remain fail-closed and resumable?\n- Does evidence support only the observed outcome?\n",
            sample.title, sample.goal, sample.scenario
        );
        let diagram = format!(
            "flowchart LR\n  A[\"Explicit v2 opt-in\"] --> B[\"Automated six-card packet\"]\n  B --> C[\"{} proof\"]\n  C --> D[\"Exact-revision review\"]\n  D --> E[\"Typed outcome evidence\"]\n  C -. failure .-> R[\"Fail closed and retry\"]\n  R --> C\n",
            sample.scenario
        );
        write_text(&design_path, &design)?;
        write_text(&diagram_path, &diagram)?;
        let design_digest = blake3::hash(design.as_bytes()).to_hex().to_string();
        let diagram_digest = blake3::hash(diagram.as_bytes()).to_hex().to_string();
        let cards = initial_cards(
            sample.issue,
            "danielbaustin/agent-design-language",
            &relative(&design_path, root),
            &design_digest,
            &relative(&diagram_path, root),
            &diagram_digest,
            crate::cards::InitialCardInput {
                title: sample.title.into(),
                slug: sample.slug.into(),
                version: "v0.91.7".into(),
                goal: sample.goal.into(),
                required_outcome: format!(
                    "A complete, reviewed {} sample packet.",
                    sample.scenario
                ),
                declared_scope: vec![sample.scenario.into(), "C-SDLC v2 qualification".into()],
                authority_boundary: vec!["No default-generation change".into()],
                operator_constraints: vec!["none".into()],
                task_boundary: format!("Exercise only the {} qualification path.", sample.scenario),
                deliverables: vec![
                    "six generated cards".into(),
                    "design".into(),
                    "diagram".into(),
                ],
                acceptance_criteria: vec![
                    "typed evidence is reproducible".into(),
                    "failure is fail-closed".into(),
                ],
                dependencies: vec!["C-SDLC v2 Gates 2-8".into()],
                repo_inputs: vec!["csdlc-v2".into()],
                non_goals: vec!["default cutover".into(), "legacy deletion".into()],
                plan_summary: format!(
                    "Run the bounded {} sample and retain exact evidence.",
                    sample.scenario
                ),
                steps: vec![PlanStep {
                    id: "sample-proof".into(),
                    action: format!("Execute {} proof", sample.scenario),
                    acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                    status: StepStatus::Pending,
                }],
                affected_areas: vec!["csdlc-v2".into()],
                invariants: vec![
                    "v1 remains the default".into(),
                    "review precedes publication".into(),
                ],
                risks: vec!["sample evidence could overclaim production behavior".into()],
                planning_profile: PlanningProfile::Small,
                stop_conditions: vec!["unexplained critical parity difference".into()],
                validation_lanes: vec![ValidationLane {
                    lane: "sample-focused".into(),
                    proof_role: sample.scenario.into(),
                    acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                    deterministic: true,
                    resource_profile: ResourceProfile::Small,
                    budget_seconds: 120,
                    budget_tokens: 10_000,
                    argv: vec![
                        "cargo".into(),
                        "test".into(),
                        "--test".into(),
                        "gate9".into(),
                    ],
                    parallel_group: "gate9-local".into(),
                    defer_reason: None,
                }],
                failure_policy: "Fail closed, retain evidence, repair, and retry idempotently."
                    .into(),
                review_prompts: vec![
                    "Check scope, evidence, retry truth, and non-overclaiming.".into()
                ],
                review_scope: "generated sample packet".into(),
            },
        )?;
        let mut card_paths = BTreeMap::new();
        for (kind, values) in cards {
            let rendered = render(&values)?;
            let path = sample_root.join(format!("{kind}.md"));
            write_text(&path, &rendered.markdown)?;
            card_paths.insert(kind.to_string(), path);
        }
        let packet = SamplePacket {
            issue: sample.issue,
            slug: sample.slug.into(),
            generation,
            root: PathBuf::from(sample.slug),
            card_paths: card_paths
                .into_iter()
                .map(|(kind, path)| {
                    (
                        kind,
                        path.strip_prefix(&sample_root)
                            .unwrap_or(&path)
                            .to_path_buf(),
                    )
                })
                .collect(),
            design_path: PathBuf::from("design.md"),
            diagram_path: PathBuf::from("diagram.mmd"),
            execution_evidence: "csdlc-v2/tests/gate9.rs::representative_samples_reopen_persisted_store_between_lifecycle_phases".into(),
        };
        write_json(&sample_root.join("packet.json"), &packet)?;
        packets.push(packet);
    }
    Ok(packets)
}

fn relative(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

fn write_text(path: &Path, value: &str) -> Result<()> {
    if fs::read_to_string(path).ok().as_deref() != Some(value) {
        fs::write(path, value)?;
    }
    Ok(())
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    if fs::read(path).ok().as_deref() != Some(bytes.as_slice()) {
        fs::write(path, bytes)?;
    }
    Ok(())
}

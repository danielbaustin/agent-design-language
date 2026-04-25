use super::*;
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_EVALUATION_SELECTION_SCHEMA: &str =
    "runtime_v2.evaluation_selection_artifact.v1";
pub const RUNTIME_V2_SELECTION_NEGATIVE_CASES_SCHEMA: &str =
    "runtime_v2.selection_negative_cases.v1";
pub const RUNTIME_V2_EVALUATION_SELECTION_PATH: &str =
    "runtime_v2/contract_market/evaluation_selection.json";
pub const RUNTIME_V2_SELECTION_NEGATIVE_CASES_PATH: &str =
    "runtime_v2/contract_market/selection_negative_cases.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2SelectionCriterionScore {
    pub criterion_id: String,
    pub label: String,
    pub max_basis_points: u64,
    pub awarded_basis_points: u64,
    pub mandatory: bool,
    pub passed: bool,
    pub rationale: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2BidEvaluation {
    pub bid_id: String,
    pub bid_ref: String,
    pub bidder_actor_id: String,
    pub criterion_scores: Vec<RuntimeV2SelectionCriterionScore>,
    pub aggregate_score_basis_points: u64,
    pub mandatory_failures: Vec<String>,
    pub minimum_assurance_satisfied: bool,
    pub sponsor_present: bool,
    pub gateway_present: bool,
    pub tool_readiness_status: String,
    pub tool_readiness_notes: Vec<String>,
    pub evaluation_summary: String,
    pub rank: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2SelectionRecommendation {
    pub decision: String,
    pub selected_bid_id: String,
    pub selected_bid_ref: String,
    pub explanation: String,
    pub winning_score_basis_points: u64,
    pub runner_up_bid_id: String,
    pub runner_up_score_basis_points: u64,
    pub tie_break_rationale: Option<String>,
    pub override_applied: bool,
    pub override_rationale: Option<String>,
    pub authority_guardrails: Vec<String>,
    pub execution_authority_granted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2EvaluationSelectionArtifact {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub evaluation_id: String,
    pub artifact_path: String,
    pub contract_ref: String,
    pub contract_id: String,
    pub evaluated_bid_refs: Vec<String>,
    pub evaluation_method: String,
    pub bid_evaluations: Vec<RuntimeV2BidEvaluation>,
    pub recommendation: RuntimeV2SelectionRecommendation,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2SelectionNegativeCase {
    pub case_id: String,
    pub mutation: String,
    pub expected_error_fragment: String,
    pub invalid_selection: RuntimeV2EvaluationSelectionArtifact,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2SelectionNegativeCases {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub contract_ref: String,
    pub valid_selection_ref: String,
    pub required_negative_cases: Vec<RuntimeV2SelectionNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2EvaluationSelectionArtifacts {
    pub contract: RuntimeV2ContractArtifact,
    pub valid_bids: Vec<RuntimeV2BidArtifact>,
    pub selection: RuntimeV2EvaluationSelectionArtifact,
    pub negative_cases: RuntimeV2SelectionNegativeCases,
}

impl RuntimeV2EvaluationSelectionArtifacts {
    pub fn prototype() -> Result<Self> {
        let bid_schema = runtime_v2_bid_schema_contract()?;
        let selection = RuntimeV2EvaluationSelectionArtifact::prototype(
            &bid_schema.contract,
            &bid_schema.valid_bids,
        )?;
        let negative_cases = RuntimeV2SelectionNegativeCases::prototype(
            &bid_schema.contract,
            &bid_schema.valid_bids,
            &selection,
        )?;
        let artifacts = Self {
            contract: bid_schema.contract,
            valid_bids: bid_schema.valid_bids,
            selection,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.contract.validate()?;
        if self.valid_bids.len() != 2 {
            return Err(anyhow!(
                "evaluation_selection.valid_bids must contain exactly two valid bids"
            ));
        }
        for bid in &self.valid_bids {
            bid.validate_against(&self.contract)?;
        }
        self.selection
            .validate_against(&self.contract, &self.valid_bids)?;
        self.negative_cases
            .validate_against(&self.contract, &self.valid_bids, &self.selection)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_EVALUATION_SELECTION_PATH,
            self.selection.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_SELECTION_NEGATIVE_CASES_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2EvaluationSelectionArtifact {
    pub fn prototype(
        contract: &RuntimeV2ContractArtifact,
        valid_bids: &[RuntimeV2BidArtifact],
    ) -> Result<Self> {
        contract.validate()?;
        if valid_bids.len() != 2 {
            return Err(anyhow!(
                "evaluation selection prototype expects exactly two valid bids"
            ));
        }

        let alpha = &valid_bids[0];
        let bravo = &valid_bids[1];

        let alpha_eval = RuntimeV2BidEvaluation {
            bid_id: alpha.bid_id.clone(),
            bid_ref: alpha.artifact_path.clone(),
            bidder_actor_id: alpha.bidder_actor_id.clone(),
            criterion_scores: vec![
                criterion_score(&contract.evaluation_criteria[0], 5000, true, &[
                    "runtime_v2/contract_market/trace_requirements.json#trace-integrity",
                    "runtime_v2/contract_market/bid_alpha.json#trace_requirements",
                ]),
                criterion_score(&contract.evaluation_criteria[1], 2700, true, &[
                    "runtime_v2/access_control/access_events.json#projection",
                    "runtime_v2/contract_market/bid_alpha.json#commitments",
                ]),
                criterion_score(&contract.evaluation_criteria[2], 1600, true, &[
                    "runtime_v2/contract_market/bid_alpha.json#resource_claims",
                ]),
            ],
            aggregate_score_basis_points: 9300,
            mandatory_failures: vec![],
            minimum_assurance_satisfied: true,
            sponsor_present: true,
            gateway_present: true,
            tool_readiness_status: "deferred_governed_tool_authority".to_string(),
            tool_readiness_notes: strings(&[
                "selected bid records tool dependency as a deferred execution constraint",
                "adapter availability is not treated as authority to execute a governed tool",
            ]),
            evaluation_summary:
                "Alpha preserves full trace integrity, keeps reviewer-visible evidence quality high, and remains within the lower bounded operator budget."
                    .to_string(),
            rank: 1,
        };

        let bravo_eval = RuntimeV2BidEvaluation {
            bid_id: bravo.bid_id.clone(),
            bid_ref: bravo.artifact_path.clone(),
            bidder_actor_id: bravo.bidder_actor_id.clone(),
            criterion_scores: vec![
                criterion_score(&contract.evaluation_criteria[0], 4800, true, &[
                    "runtime_v2/contract_market/trace_requirements.json#trace-integrity",
                    "runtime_v2/contract_market/bid_bravo.json#trace_requirements",
                ]),
                criterion_score(&contract.evaluation_criteria[1], 2800, true, &[
                    "runtime_v2/access_control/access_events.json#projection",
                    "runtime_v2/contract_market/bid_bravo.json#commitments",
                ]),
                criterion_score(&contract.evaluation_criteria[2], 1500, true, &[
                    "runtime_v2/contract_market/bid_bravo.json#resource_claims",
                ]),
            ],
            aggregate_score_basis_points: 9100,
            mandatory_failures: vec![],
            minimum_assurance_satisfied: true,
            sponsor_present: true,
            gateway_present: true,
            tool_readiness_status: "deferred_governed_tool_authority".to_string(),
            tool_readiness_notes: strings(&[
                "bravo remains tool-dependent but keeps that dependency in evidence-only mode",
                "selection does not treat gateway review, valid JSON, or model confidence as tool authority",
            ]),
            evaluation_summary:
                "Bravo remains admissible and reviewer-visible, but it carries the larger bounded operator burden and a slightly lower aggregate score."
                    .to_string(),
            rank: 2,
        };

        let artifact = Self {
            schema_version: RUNTIME_V2_EVALUATION_SELECTION_SCHEMA.to_string(),
            demo_id: "D4".to_string(),
            wp_id: "WP-05".to_string(),
            evaluation_id: "selection-observatory-readiness-alpha".to_string(),
            artifact_path: RUNTIME_V2_EVALUATION_SELECTION_PATH.to_string(),
            contract_ref: contract.artifact_path.clone(),
            contract_id: contract.contract_id.clone(),
            evaluated_bid_refs: vec![alpha.artifact_path.clone(), bravo.artifact_path.clone()],
            evaluation_method:
                "Evaluate both admissible bids against the parent contract criteria, fail closed on mandatory authority checks, and emit a reviewer-visible award recommendation."
                    .to_string(),
            bid_evaluations: vec![alpha_eval, bravo_eval],
            recommendation: RuntimeV2SelectionRecommendation {
                decision: "recommend_award".to_string(),
                selected_bid_id: alpha.bid_id.clone(),
                selected_bid_ref: alpha.artifact_path.clone(),
                explanation:
                    "Recommend alpha because it satisfies all mandatory criteria, preserves stronger trace continuity, and fits the bounded operator budget while explicitly deferring governed-tool execution."
                        .to_string(),
                winning_score_basis_points: 9300,
                runner_up_bid_id: bravo.bid_id.clone(),
                runner_up_score_basis_points: 9100,
                tie_break_rationale: None,
                override_applied: false,
                override_rationale: None,
                authority_guardrails: strings(&[
                    "selection re-checks minimum assurance, sponsor presence, and gateway presence before award recommendation",
                    "tool-readiness warnings are evidence only and do not grant tool execution authority",
                    "valid JSON, model confidence, and adapter availability are never treated as authority to execute a tool",
                ]),
                execution_authority_granted: false,
            },
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_evaluation_selection -- --nocapture"
                    .to_string(),
            claim_boundary:
                "This artifact explains bounded bid evaluation and award recommendation, but it does not settle payment, grant citizen standing, or authorize governed tool execution."
                    .to_string(),
        };
        artifact.validate_against(contract, valid_bids)?;
        Ok(artifact)
    }

    pub fn validate_against(
        &self,
        contract: &RuntimeV2ContractArtifact,
        valid_bids: &[RuntimeV2BidArtifact],
    ) -> Result<()> {
        if self.schema_version != RUNTIME_V2_EVALUATION_SELECTION_SCHEMA {
            return Err(anyhow!(
                "unsupported evaluation_selection.schema_version '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "evaluation_selection.demo_id")?;
        validate_nonempty_text(&self.wp_id, "evaluation_selection.wp_id")?;
        normalize_id(
            self.evaluation_id.clone(),
            "evaluation_selection.evaluation_id",
        )?;
        validate_relative_path(&self.artifact_path, "evaluation_selection.artifact_path")?;
        if self.contract_ref != contract.artifact_path {
            return Err(anyhow!(
                "evaluation_selection.contract_ref must bind to the valid parent contract"
            ));
        }
        if self.contract_id != contract.contract_id {
            return Err(anyhow!(
                "evaluation_selection.contract_id must match the parent contract"
            ));
        }
        validate_nonempty_text(
            &self.evaluation_method,
            "evaluation_selection.evaluation_method",
        )?;
        validate_nonempty_text(
            &self.validation_command,
            "evaluation_selection.validation_command",
        )?;
        validate_nonempty_text(&self.claim_boundary, "evaluation_selection.claim_boundary")?;
        if !self.claim_boundary.contains("does not settle payment") {
            return Err(anyhow!(
                "evaluation_selection.claim_boundary must state that selection does not settle payment"
            ));
        }
        if !self
            .claim_boundary
            .contains("authorize governed tool execution")
        {
            return Err(anyhow!(
                "evaluation_selection.claim_boundary must state that selection does not authorize governed tool execution"
            ));
        }
        if self.evaluated_bid_refs.len() != valid_bids.len() {
            return Err(anyhow!(
                "evaluation_selection.evaluated_bid_refs must match the valid bid count"
            ));
        }
        if self.bid_evaluations.len() != valid_bids.len() {
            return Err(anyhow!(
                "evaluation_selection.bid_evaluations must match the valid bid count"
            ));
        }

        let expected_refs = valid_bids
            .iter()
            .map(|bid| bid.artifact_path.clone())
            .collect::<Vec<_>>();
        if self.evaluated_bid_refs != expected_refs {
            return Err(anyhow!(
                "evaluation_selection.evaluated_bid_refs must preserve deterministic valid-bid order"
            ));
        }

        let mut seen_bids = BTreeSet::new();
        let mut seen_ranks = BTreeSet::new();
        for (evaluation, bid) in self.bid_evaluations.iter().zip(valid_bids.iter()) {
            evaluation.validate_against(contract, bid)?;
            if !seen_bids.insert(evaluation.bid_id.clone()) {
                return Err(anyhow!(
                    "evaluation_selection.bid_evaluations contains duplicate bid_id '{}'",
                    evaluation.bid_id
                ));
            }
            if !seen_ranks.insert(evaluation.rank) {
                return Err(anyhow!(
                    "evaluation_selection.bid_evaluations must have unique deterministic ranks"
                ));
            }
        }

        self.recommendation
            .validate_against(&self.bid_evaluations, valid_bids)?;
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 evaluation selection")
    }
}

impl RuntimeV2BidEvaluation {
    fn validate_against(
        &self,
        contract: &RuntimeV2ContractArtifact,
        bid: &RuntimeV2BidArtifact,
    ) -> Result<()> {
        normalize_id(self.bid_id.clone(), "bid_evaluation.bid_id")?;
        if self.bid_id != bid.bid_id {
            return Err(anyhow!(
                "bid_evaluation.bid_id must match the referenced valid bid"
            ));
        }
        if self.bid_ref != bid.artifact_path {
            return Err(anyhow!(
                "bid_evaluation.bid_ref must bind to the referenced valid bid"
            ));
        }
        if self.bidder_actor_id != bid.bidder_actor_id {
            return Err(anyhow!(
                "bid_evaluation.bidder_actor_id must match the valid bid"
            ));
        }
        if self.criterion_scores.len() != contract.evaluation_criteria.len() {
            return Err(anyhow!(
                "bid_evaluation.criterion_scores must cover every contract evaluation criterion"
            ));
        }

        let mut total = 0_u64;
        let mut derived_mandatory_failures = Vec::new();
        for (score, criterion) in self
            .criterion_scores
            .iter()
            .zip(contract.evaluation_criteria.iter())
        {
            score.validate_against(criterion)?;
            total += score.awarded_basis_points;
            if score.mandatory && !score.passed {
                derived_mandatory_failures.push(score.criterion_id.clone());
            }
        }
        if total != self.aggregate_score_basis_points {
            return Err(anyhow!(
                "bid_evaluation.aggregate_score_basis_points must equal the awarded criterion total"
            ));
        }
        if self.mandatory_failures != derived_mandatory_failures {
            return Err(anyhow!(
                "bid_evaluation.mandatory_failures must match failed mandatory criteria"
            ));
        }
        if !self.minimum_assurance_satisfied {
            return Err(anyhow!(
                "bid_evaluation must preserve the minimum counterparty assurance check"
            ));
        }
        if !self.sponsor_present {
            return Err(anyhow!(
                "bid_evaluation must preserve sponsor presence for award eligibility"
            ));
        }
        if !self.gateway_present {
            return Err(anyhow!(
                "bid_evaluation must preserve gateway presence for award eligibility"
            ));
        }
        match self.tool_readiness_status.as_str() {
            "deferred_governed_tool_authority" | "no_tool_dependency" => {}
            other => {
                return Err(anyhow!(
                    "unsupported bid_evaluation.tool_readiness_status '{other}'"
                ))
            }
        }
        if !bid.expected_tool_usage.is_empty()
            && self.tool_readiness_status != "deferred_governed_tool_authority"
        {
            return Err(anyhow!(
                "tool-dependent bids must remain in deferred_governed_tool_authority status"
            ));
        }
        if self.tool_readiness_notes.is_empty() {
            return Err(anyhow!(
                "bid_evaluation.tool_readiness_notes must explain tool readiness status"
            ));
        }
        validate_nonempty_text(
            &self.evaluation_summary,
            "bid_evaluation.evaluation_summary",
        )?;
        if self.rank == 0 {
            return Err(anyhow!("bid_evaluation.rank must be positive"));
        }
        Ok(())
    }
}

impl RuntimeV2SelectionCriterionScore {
    fn validate_against(&self, criterion: &RuntimeV2ContractEvaluationCriterion) -> Result<()> {
        normalize_id(
            self.criterion_id.clone(),
            "selection_criterion_score.criterion_id",
        )?;
        if self.criterion_id != criterion.criterion_id {
            return Err(anyhow!(
                "selection_criterion_score.criterion_id must preserve contract criterion order"
            ));
        }
        validate_nonempty_text(&self.label, "selection_criterion_score.label")?;
        if self.label != criterion.label {
            return Err(anyhow!(
                "selection_criterion_score.label must match the contract criterion"
            ));
        }
        if self.max_basis_points != criterion.weight_basis_points {
            return Err(anyhow!(
                "selection_criterion_score.max_basis_points must match the contract criterion weight"
            ));
        }
        if self.awarded_basis_points > self.max_basis_points {
            return Err(anyhow!(
                "selection_criterion_score.awarded_basis_points must not exceed the criterion weight"
            ));
        }
        if self.mandatory != criterion.mandatory {
            return Err(anyhow!(
                "selection_criterion_score.mandatory must match the contract criterion"
            ));
        }
        if self.mandatory && !self.passed && self.awarded_basis_points != 0 {
            return Err(anyhow!(
                "failed mandatory criteria must not receive awarded basis points"
            ));
        }
        validate_nonempty_text(&self.rationale, "selection_criterion_score.rationale")?;
        if self.evidence_refs.is_empty() {
            return Err(anyhow!(
                "selection_criterion_score.evidence_refs must not be empty"
            ));
        }
        for reference in &self.evidence_refs {
            validate_nonempty_text(reference, "selection_criterion_score.evidence_refs")?;
        }
        Ok(())
    }
}

impl RuntimeV2SelectionRecommendation {
    fn validate_against(
        &self,
        bid_evaluations: &[RuntimeV2BidEvaluation],
        valid_bids: &[RuntimeV2BidArtifact],
    ) -> Result<()> {
        validate_nonempty_text(&self.decision, "selection_recommendation.decision")?;
        if self.decision != "recommend_award" {
            return Err(anyhow!(
                "selection_recommendation.decision must remain recommend_award"
            ));
        }
        normalize_id(
            self.selected_bid_id.clone(),
            "selection_recommendation.selected_bid_id",
        )?;
        normalize_id(
            self.runner_up_bid_id.clone(),
            "selection_recommendation.runner_up_bid_id",
        )?;
        validate_relative_path(
            &self.selected_bid_ref,
            "selection_recommendation.selected_bid_ref",
        )?;
        validate_nonempty_text(&self.explanation, "selection_recommendation.explanation")?;
        if self.authority_guardrails.is_empty() {
            return Err(anyhow!(
                "selection_recommendation.authority_guardrails must not be empty"
            ));
        }
        if self.execution_authority_granted {
            return Err(anyhow!(
                "evaluation selection must not grant governed tool execution authority"
            ));
        }
        let mut selected_eval = None;
        let mut runner_up_eval = None;
        for evaluation in bid_evaluations {
            if evaluation.bid_id == self.selected_bid_id {
                selected_eval = Some(evaluation);
            }
            if evaluation.bid_id == self.runner_up_bid_id {
                runner_up_eval = Some(evaluation);
            }
        }
        let selected_eval = selected_eval.ok_or_else(|| {
            anyhow!("selection_recommendation.selected_bid_id must reference an evaluated bid")
        })?;
        let runner_up_eval = runner_up_eval.ok_or_else(|| {
            anyhow!("selection_recommendation.runner_up_bid_id must reference an evaluated bid")
        })?;

        let selected_bid = valid_bids
            .iter()
            .find(|bid| bid.bid_id == self.selected_bid_id)
            .ok_or_else(|| {
                anyhow!("selection_recommendation.selected_bid_id must bind to a valid bid")
            })?;
        if self.selected_bid_ref != selected_bid.artifact_path {
            return Err(anyhow!(
                "selection_recommendation.selected_bid_ref must bind to the selected valid bid"
            ));
        }
        if self.winning_score_basis_points != selected_eval.aggregate_score_basis_points {
            return Err(anyhow!(
                "selection_recommendation.winning_score_basis_points must match the selected bid evaluation"
            ));
        }
        if self.runner_up_score_basis_points != runner_up_eval.aggregate_score_basis_points {
            return Err(anyhow!(
                "selection_recommendation.runner_up_score_basis_points must match the runner-up bid evaluation"
            ));
        }
        if !selected_eval.mandatory_failures.is_empty() {
            return Err(anyhow!(
                "selected bid must satisfy all mandatory criteria before award recommendation"
            ));
        }
        if !selected_eval.minimum_assurance_satisfied
            || !selected_eval.sponsor_present
            || !selected_eval.gateway_present
        {
            return Err(anyhow!(
                "selection recommendation must not bypass authority checks"
            ));
        }
        let top_score = bid_evaluations
            .iter()
            .map(|evaluation| evaluation.aggregate_score_basis_points)
            .max()
            .unwrap_or(0);
        let tied_top = bid_evaluations
            .iter()
            .filter(|evaluation| evaluation.aggregate_score_basis_points == top_score)
            .count()
            > 1;

        if tied_top
            && self
                .tie_break_rationale
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
        {
            return Err(anyhow!(
                "selection_recommendation.tie_break_rationale is required when top bids tie"
            ));
        }
        if !tied_top && self.tie_break_rationale.is_some() {
            return Err(anyhow!(
                "selection_recommendation.tie_break_rationale is only valid for top-score ties"
            ));
        }
        if self.override_applied {
            let rationale = self
                .override_rationale
                .as_deref()
                .ok_or_else(|| anyhow!("selection_recommendation.override_rationale is required when override_applied is true"))?;
            validate_nonempty_text(rationale, "selection_recommendation.override_rationale")?;
            let lowered = rationale.to_ascii_lowercase();
            if lowered.contains("adapter availability")
                || lowered.contains("model confidence")
                || lowered.contains("valid json")
            {
                return Err(anyhow!(
                    "selection override rationale must not treat adapter availability, model confidence, or valid JSON as tool execution authority"
                ));
            }
        } else if self.override_rationale.is_some() {
            return Err(anyhow!(
                "selection_recommendation.override_rationale is only valid when override_applied is true"
            ));
        } else if !tied_top && self.selected_bid_id != highest_ranked_bid_id(bid_evaluations)? {
            return Err(anyhow!(
                "selection recommendation must follow the highest-ranked bid unless a traceable override is recorded"
            ));
        }

        Ok(())
    }
}

impl RuntimeV2SelectionNegativeCases {
    pub fn prototype(
        contract: &RuntimeV2ContractArtifact,
        valid_bids: &[RuntimeV2BidArtifact],
        selection: &RuntimeV2EvaluationSelectionArtifact,
    ) -> Result<Self> {
        let mut critical_failure = selection.clone();
        let selected = critical_failure
            .bid_evaluations
            .iter_mut()
            .find(|evaluation| evaluation.bid_id == critical_failure.recommendation.selected_bid_id)
            .expect("selected evaluation");
        let evidence_quality = selected
            .criterion_scores
            .iter_mut()
            .find(|score| score.criterion_id == "evidence-quality")
            .expect("evidence-quality criterion");
        evidence_quality.passed = false;
        evidence_quality.awarded_basis_points = 0;
        evidence_quality.rationale =
            "Mutation: selected bid loses the mandatory evidence-quality criterion.".to_string();
        selected.aggregate_score_basis_points = selected
            .criterion_scores
            .iter()
            .map(|score| score.awarded_basis_points)
            .sum();
        selected.mandatory_failures = vec!["evidence-quality".to_string()];
        critical_failure.recommendation.winning_score_basis_points =
            selected.aggregate_score_basis_points;

        let mut tie_without_rationale = selection.clone();
        let runner_up = tie_without_rationale
            .bid_evaluations
            .iter_mut()
            .find(|evaluation| {
                evaluation.bid_id == tie_without_rationale.recommendation.runner_up_bid_id
            })
            .expect("runner up evaluation");
        runner_up.aggregate_score_basis_points = tie_without_rationale
            .recommendation
            .winning_score_basis_points;
        if let Some(resource_fit) = runner_up
            .criterion_scores
            .iter_mut()
            .find(|score| score.criterion_id == "resource-fit")
        {
            resource_fit.awarded_basis_points = 1700;
            resource_fit.rationale =
                "Mutation: equalize resource-fit to create a top-score tie.".to_string();
        }
        runner_up.aggregate_score_basis_points = runner_up
            .criterion_scores
            .iter()
            .map(|score| score.awarded_basis_points)
            .sum();
        tie_without_rationale
            .recommendation
            .runner_up_score_basis_points = runner_up.aggregate_score_basis_points;
        tie_without_rationale.recommendation.tie_break_rationale = None;

        let mut unsupported_override = selection.clone();
        unsupported_override.recommendation.override_applied = true;
        unsupported_override.recommendation.override_rationale = Some(
            "Adapter availability and model confidence are enough authority to execute the tool immediately."
                .to_string(),
        );
        unsupported_override.recommendation.selected_bid_id =
            unsupported_override.recommendation.runner_up_bid_id.clone();
        unsupported_override.recommendation.selected_bid_ref = valid_bids[1].artifact_path.clone();
        unsupported_override
            .recommendation
            .winning_score_basis_points = unsupported_override
            .recommendation
            .runner_up_score_basis_points;
        unsupported_override.recommendation.runner_up_bid_id = valid_bids[0].bid_id.clone();
        unsupported_override
            .recommendation
            .runner_up_score_basis_points = selection.recommendation.winning_score_basis_points;

        let proof = Self {
            schema_version: RUNTIME_V2_SELECTION_NEGATIVE_CASES_SCHEMA.to_string(),
            proof_id: "evaluation-selection-negative-cases".to_string(),
            demo_id: "D4".to_string(),
            artifact_path: RUNTIME_V2_SELECTION_NEGATIVE_CASES_PATH.to_string(),
            contract_ref: contract.artifact_path.clone(),
            valid_selection_ref: selection.artifact_path.clone(),
            required_negative_cases: vec![
                RuntimeV2SelectionNegativeCase {
                    case_id: "selected-bid-loses-mandatory-criterion".to_string(),
                    mutation:
                        "remove evidence-quality from the selected bid while keeping the recommendation"
                            .to_string(),
                    expected_error_fragment:
                        "selected bid must satisfy all mandatory criteria".to_string(),
                    invalid_selection: critical_failure,
                },
                RuntimeV2SelectionNegativeCase {
                    case_id: "top-score-tie-without-rationale".to_string(),
                    mutation: "force a top-score tie and remove tie-break rationale".to_string(),
                    expected_error_fragment:
                        "selection_recommendation.tie_break_rationale is required when top bids tie"
                            .to_string(),
                    invalid_selection: tie_without_rationale,
                },
                RuntimeV2SelectionNegativeCase {
                    case_id: "unsupported-override-authority-shortcut".to_string(),
                    mutation:
                        "override the winner using adapter availability and model confidence as tool authority"
                            .to_string(),
                    expected_error_fragment:
                        "selection override rationale must not treat adapter availability, model confidence, or valid JSON as tool execution authority"
                            .to_string(),
                    invalid_selection: unsupported_override,
                },
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_evaluation_selection -- --nocapture"
                    .to_string(),
            claim_boundary:
                "Negative cases prove that selection fails closed on mandatory criterion loss, unresolved ties, and override rationales that try to smuggle tool authority through confidence or adapter claims."
                    .to_string(),
        };
        proof.validate_against(contract, valid_bids, selection)?;
        Ok(proof)
    }

    pub fn validate_against(
        &self,
        contract: &RuntimeV2ContractArtifact,
        valid_bids: &[RuntimeV2BidArtifact],
        selection: &RuntimeV2EvaluationSelectionArtifact,
    ) -> Result<()> {
        if self.schema_version != RUNTIME_V2_SELECTION_NEGATIVE_CASES_SCHEMA {
            return Err(anyhow!(
                "unsupported selection_negative_cases.schema_version '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "selection_negative_cases.proof_id")?;
        validate_demo_id(&self.demo_id, "selection_negative_cases.demo_id")?;
        validate_relative_path(
            &self.artifact_path,
            "selection_negative_cases.artifact_path",
        )?;
        if self.contract_ref != contract.artifact_path {
            return Err(anyhow!(
                "selection_negative_cases.contract_ref must bind to the valid parent contract"
            ));
        }
        if self.valid_selection_ref != selection.artifact_path {
            return Err(anyhow!(
                "selection_negative_cases.valid_selection_ref must bind to the valid selection artifact"
            ));
        }
        if self.required_negative_cases.len() != 3 {
            return Err(anyhow!(
                "selection_negative_cases.required_negative_cases must contain three required mutations"
            ));
        }
        let actual_case_ids = self
            .required_negative_cases
            .iter()
            .map(|case| case.case_id.as_str())
            .collect::<BTreeSet<_>>();
        let expected_case_ids = BTreeSet::from([
            "selected-bid-loses-mandatory-criterion",
            "top-score-tie-without-rationale",
            "unsupported-override-authority-shortcut",
        ]);
        if actual_case_ids != expected_case_ids {
            return Err(anyhow!(
                "selection_negative_cases.required_negative_cases must contain the required case-id set"
            ));
        }
        validate_nonempty_text(
            &self.validation_command,
            "selection_negative_cases.validation_command",
        )?;
        validate_nonempty_text(
            &self.claim_boundary,
            "selection_negative_cases.claim_boundary",
        )?;
        for case in &self.required_negative_cases {
            case.validate()?;
            let err = case
                .invalid_selection
                .validate_against(contract, valid_bids)
                .expect_err("negative selection should fail");
            if !err.to_string().contains(&case.expected_error_fragment) {
                return Err(anyhow!(
                    "selection negative case '{}' failed with unexpected error '{}'",
                    case.case_id,
                    err
                ));
            }
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 selection negative cases")
    }
}

impl RuntimeV2SelectionNegativeCase {
    fn validate(&self) -> Result<()> {
        normalize_id(self.case_id.clone(), "selection_negative_case.case_id")?;
        validate_nonempty_text(&self.mutation, "selection_negative_case.mutation")?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "selection_negative_case.expected_error_fragment",
        )
    }
}

fn criterion_score(
    criterion: &RuntimeV2ContractEvaluationCriterion,
    awarded_basis_points: u64,
    passed: bool,
    evidence_refs: &[&str],
) -> RuntimeV2SelectionCriterionScore {
    RuntimeV2SelectionCriterionScore {
        criterion_id: criterion.criterion_id.clone(),
        label: criterion.label.clone(),
        max_basis_points: criterion.weight_basis_points,
        awarded_basis_points,
        mandatory: criterion.mandatory,
        passed,
        rationale: format!(
            "Selection score for '{}' records reviewer-visible evidence and stays within the contract weight.",
            criterion.label
        ),
        evidence_refs: strings(evidence_refs),
    }
}

fn highest_ranked_bid_id(bid_evaluations: &[RuntimeV2BidEvaluation]) -> Result<String> {
    bid_evaluations
        .iter()
        .min_by_key(|evaluation| evaluation.rank)
        .map(|evaluation| evaluation.bid_id.clone())
        .ok_or_else(|| anyhow!("selection recommendation requires at least one evaluated bid"))
}

fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)?;
    if !value.starts_with('D') {
        return Err(anyhow!("{field} must start with 'D'"));
    }
    Ok(())
}

fn validate_nonempty_text(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

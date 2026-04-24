use super::*;
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_RESOURCE_STEWARDSHIP_BRIDGE_SCHEMA: &str =
    "runtime_v2.resource_stewardship_bridge_artifact.v1";
pub const RUNTIME_V2_RESOURCE_STEWARDSHIP_BRIDGE_PATH: &str =
    "runtime_v2/contract_market/resource_stewardship_bridge.json";

pub fn runtime_v2_resource_stewardship_bridge() -> Result<RuntimeV2ResourceStewardshipBridgeArtifact>
{
    RuntimeV2ResourceStewardshipBridgeArtifact::prototype()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ResourcePolicyBinding {
    pub policy_domain: String,
    pub source_surface: String,
    pub enforcement_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ResourceClaim {
    pub claim_id: String,
    pub resource_kind: String,
    pub estimate_kind: String,
    pub estimated_units: u64,
    pub unit_basis: String,
    pub bounded_by_policy: bool,
    pub tool_execution_authorized: bool,
    pub rationale: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ToolResourceConstraint {
    pub requirement_id: String,
    pub capability: String,
    pub adapter_family: String,
    pub usage_mode: String,
    pub budget_claim_id: String,
    pub governed_authority_required: bool,
    pub execution_authority_granted: bool,
    pub required_evidence: Vec<String>,
    pub boundary_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2BidResourceEstimate {
    pub bid_id: String,
    pub bid_ref: String,
    pub bidder_actor_id: String,
    pub estimate_summary: String,
    pub claims: Vec<RuntimeV2ResourceClaim>,
    pub tool_resource_constraints: Vec<RuntimeV2ToolResourceConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ResourceStewardshipBridgeArtifact {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub artifact_path: String,
    pub contract_ref: String,
    pub selection_ref: String,
    pub selected_bid_ref: String,
    pub subcontract_ref: String,
    pub bridge_decision: String,
    pub policy_bindings: Vec<RuntimeV2ResourcePolicyBinding>,
    pub contract_resource_claims: Vec<RuntimeV2ResourceClaim>,
    pub bid_resource_estimates: Vec<RuntimeV2BidResourceEstimate>,
    pub selected_bid_resource_summary: String,
    pub boundary_notes: Vec<String>,
    pub validation_command: String,
    pub claim_boundary: String,
}

impl RuntimeV2ResourceStewardshipBridgeArtifact {
    pub fn prototype() -> Result<Self> {
        let contract = runtime_v2_contract_schema_contract()?.contract;
        let selection_artifacts = RuntimeV2EvaluationSelectionArtifacts::prototype()?;
        let delegation = runtime_v2_delegation_subcontract_model()?;
        let selected_bid = selected_bid_for_selection(
            &selection_artifacts.selection,
            &selection_artifacts.valid_bids,
        )?;
        let runner_up_bid = selection_artifacts
            .valid_bids
            .iter()
            .find(|bid| bid.bid_id != selected_bid.bid_id)
            .ok_or_else(|| anyhow!("resource stewardship bridge requires a runner-up bid"))?;

        let artifact = Self {
            schema_version: RUNTIME_V2_RESOURCE_STEWARDSHIP_BRIDGE_SCHEMA.to_string(),
            demo_id: "D8".to_string(),
            wp_id: "WP-10".to_string(),
            artifact_path: RUNTIME_V2_RESOURCE_STEWARDSHIP_BRIDGE_PATH.to_string(),
            contract_ref: contract.artifact_path.clone(),
            selection_ref: selection_artifacts.selection.artifact_path.clone(),
            selected_bid_ref: selection_artifacts.recommendation_ref(),
            subcontract_ref: delegation.subcontract.artifact_path.clone(),
            bridge_decision:
                "Keep resource stewardship as a policy-bound review surface: the parent contract declares bounded resource envelopes, bids estimate how they fit that envelope, the selected bid remains within scope, and delegated packaging stays reviewable without transferring authority or pricing semantics."
                    .to_string(),
            policy_bindings: vec![
                RuntimeV2ResourcePolicyBinding {
                    policy_domain: "standing".to_string(),
                    source_surface:
                        "v0.90.3 standing inheritance audit for contract-market participation"
                            .to_string(),
                    enforcement_rule:
                        "resource claims may shape feasibility review but never create or override standing"
                            .to_string(),
                },
                RuntimeV2ResourcePolicyBinding {
                    policy_domain: "access_control".to_string(),
                    source_surface: contract.authority_basis_ref.clone(),
                    enforcement_rule:
                        "resource envelopes remain subordinate to explicit access-control review and gateway checks"
                            .to_string(),
                },
                RuntimeV2ResourcePolicyBinding {
                    policy_domain: "quarantine".to_string(),
                    source_surface:
                        "v0.90.3 quarantine and evidence-preservation policy surface"
                            .to_string(),
                    enforcement_rule:
                        "resource pressure cannot bypass quarantine or evidence-preservation requirements"
                            .to_string(),
                },
                RuntimeV2ResourcePolicyBinding {
                    policy_domain: "sanctuary".to_string(),
                    source_surface:
                        "v0.90.3 sanctuary and private-state protection policy surface"
                            .to_string(),
                    enforcement_rule:
                        "resource scarcity cannot justify sanctuary bypass or private-state inspection"
                            .to_string(),
                },
                RuntimeV2ResourcePolicyBinding {
                    policy_domain: "challenge".to_string(),
                    source_surface:
                        "v0.90.3 challenge and appeal rights for contested operational decisions"
                            .to_string(),
                    enforcement_rule:
                        "resource stewardship decisions remain reviewable and cannot extinguish challenge or appeal rights"
                            .to_string(),
                },
            ],
            contract_resource_claims: contract_resource_claims(&contract),
            bid_resource_estimates: vec![
                RuntimeV2BidResourceEstimate::from_bid(&contract, selected_bid)?,
                RuntimeV2BidResourceEstimate::from_bid(&contract, runner_up_bid)?,
            ],
            selected_bid_resource_summary:
                "Selected bid alpha preserves the lowest bounded compute and operator-review profile while keeping the tool-mediated projection step in constraint mode. The later subcontracted trace-manifest packaging step remains parent-reviewed and does not transfer tool or standing authority."
                    .to_string(),
            boundary_notes: strings(&[
                "Payment and pricing remain explicitly out of scope for v0.90.4 resource stewardship; this bridge records bounded feasibility rather than settlement or rate formation.",
                "Tool-resource requirements remain non-executable constraints until v0.90.5 governed-tool authority exists.",
                "Reputation markets, inter-polis economics, and production budget enforcement remain deferred until a later milestone.",
                "Resource claims may inform evaluation and review load, but they do not override standing, access control, quarantine, sanctuary, or challenge rights.",
            ]),
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_resource_stewardship_bridge -- --nocapture"
                    .to_string(),
            claim_boundary:
                "D8 proves that resource stewardship in v0.90.4 is a reviewable estimate layer only: it records compute, memory, attention, bandwidth, artifact storage, operator time, and tool-adapter budget without pricing, payment settlement, or governed-tool execution authority."
                    .to_string(),
        };
        artifact.validate_against(&contract, &selection_artifacts, &delegation)?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        let contract = runtime_v2_contract_schema_contract()?.contract;
        let selection_artifacts = RuntimeV2EvaluationSelectionArtifacts::prototype()?;
        let delegation = runtime_v2_delegation_subcontract_model()?;
        self.validate_against(&contract, &selection_artifacts, &delegation)
    }

    pub(crate) fn validate_against(
        &self,
        contract: &RuntimeV2ContractArtifact,
        selection_artifacts: &RuntimeV2EvaluationSelectionArtifacts,
        delegation: &RuntimeV2DelegationArtifacts,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.contract_ref != contract.artifact_path {
            return Err(anyhow!(
                "resource bridge contract_ref must bind the parent contract"
            ));
        }
        if self.selection_ref != selection_artifacts.selection.artifact_path {
            return Err(anyhow!(
                "resource bridge selection_ref must bind the evaluation selection artifact"
            ));
        }
        if self.selected_bid_ref
            != selection_artifacts
                .selection
                .recommendation
                .selected_bid_ref
        {
            return Err(anyhow!(
                "resource bridge selected_bid_ref must bind the selected bid"
            ));
        }
        if self.subcontract_ref != delegation.subcontract.artifact_path {
            return Err(anyhow!(
                "resource bridge subcontract_ref must bind the delegation subcontract artifact"
            ));
        }

        let required_policy_domains = BTreeSet::from_iter(strings(&[
            "standing",
            "access_control",
            "quarantine",
            "sanctuary",
            "challenge",
        ]));
        let actual_policy_domains = self
            .policy_bindings
            .iter()
            .map(|binding| binding.policy_domain.clone())
            .collect::<BTreeSet<_>>();
        if actual_policy_domains != required_policy_domains {
            return Err(anyhow!(
                "resource bridge policy_bindings must include standing, access_control, quarantine, sanctuary, and challenge"
            ));
        }

        validate_resource_claim_set(
            &self.contract_resource_claims,
            "contract_resource_claims",
            "contract_constraint",
        )?;
        for estimate in &self.bid_resource_estimates {
            let bid = selection_artifacts
                .valid_bids
                .iter()
                .find(|candidate| candidate.bid_id == estimate.bid_id)
                .ok_or_else(|| {
                    anyhow!(
                        "resource bridge contains bid estimate for unknown bid '{}'",
                        estimate.bid_id
                    )
                })?;
            estimate.validate_against(contract, bid)?;
        }

        let bid_refs = self
            .bid_resource_estimates
            .iter()
            .map(|estimate| estimate.bid_ref.clone())
            .collect::<BTreeSet<_>>();
        let expected_bid_refs = selection_artifacts
            .valid_bids
            .iter()
            .map(|bid| bid.artifact_path.clone())
            .collect::<BTreeSet<_>>();
        if bid_refs != expected_bid_refs {
            return Err(anyhow!(
                "resource bridge bid_resource_estimates must cover each valid bid exactly once"
            ));
        }

        for claim in &self.contract_resource_claims {
            if claim.tool_execution_authorized {
                return Err(anyhow!(
                    "resource bridge contract claims must not authorize tool execution"
                ));
            }
        }
        if delegation
            .subcontract
            .delegated_tool_constraints
            .iter()
            .any(|constraint| constraint.execution_authority_granted)
        {
            return Err(anyhow!(
                "resource bridge must not rely on delegated tool execution authority"
            ));
        }
        if self
            .boundary_notes
            .iter()
            .all(|note| !note.contains("Payment and pricing remain explicitly out of scope"))
        {
            return Err(anyhow!(
                "resource bridge boundary_notes must keep payment and pricing out of scope"
            ));
        }
        if self
            .boundary_notes
            .iter()
            .all(|note| !note.contains("v0.90.5 governed-tool authority"))
        {
            return Err(anyhow!(
                "resource bridge boundary_notes must defer tool execution to v0.90.5 governed-tool authority"
            ));
        }
        if !self
            .selected_bid_resource_summary
            .contains("parent-reviewed")
        {
            return Err(anyhow!(
                "resource bridge selected_bid_resource_summary must preserve parent-reviewed delegation accountability"
            ));
        }
        if !self
            .claim_boundary
            .contains("without pricing, payment settlement, or governed-tool execution authority")
        {
            return Err(anyhow!(
                "resource bridge claim_boundary must preserve the payment and governed-tool boundary"
            ));
        }

        Ok(())
    }

    fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_RESOURCE_STEWARDSHIP_BRIDGE_SCHEMA {
            return Err(anyhow!(
                "unsupported resource stewardship bridge schema '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "resource_bridge.demo_id")?;
        validate_nonempty_text(&self.wp_id, "resource_bridge.wp_id")?;
        validate_relative_path(&self.artifact_path, "resource_bridge.artifact_path")?;
        validate_relative_path(&self.contract_ref, "resource_bridge.contract_ref")?;
        validate_relative_path(&self.selection_ref, "resource_bridge.selection_ref")?;
        validate_relative_path(&self.selected_bid_ref, "resource_bridge.selected_bid_ref")?;
        validate_relative_path(&self.subcontract_ref, "resource_bridge.subcontract_ref")?;
        validate_nonempty_text(&self.bridge_decision, "resource_bridge.bridge_decision")?;
        if self.policy_bindings.is_empty() {
            return Err(anyhow!("resource bridge policy_bindings must not be empty"));
        }
        for binding in &self.policy_bindings {
            binding.validate()?;
        }
        validate_nonempty_vec(
            &self.contract_resource_claims,
            "resource_bridge.contract_resource_claims",
        )?;
        validate_nonempty_vec(
            &self.bid_resource_estimates,
            "resource_bridge.bid_resource_estimates",
        )?;
        validate_nonempty_text(
            &self.selected_bid_resource_summary,
            "resource_bridge.selected_bid_resource_summary",
        )?;
        validate_nonempty_vec(&self.boundary_notes, "resource_bridge.boundary_notes")?;
        if !self
            .validation_command
            .contains("runtime_v2_resource_stewardship_bridge")
        {
            return Err(anyhow!(
                "resource bridge validation_command must target focused tests"
            ));
        }
        validate_nonempty_text(&self.claim_boundary, "resource_bridge.claim_boundary")
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        write_relative(
            root.as_ref(),
            RUNTIME_V2_RESOURCE_STEWARDSHIP_BRIDGE_PATH,
            self.pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2ResourcePolicyBinding {
    fn validate(&self) -> Result<()> {
        validate_nonempty_text(
            &self.policy_domain,
            "resource_bridge.policy_bindings.policy_domain",
        )?;
        validate_nonempty_text(
            &self.source_surface,
            "resource_bridge.policy_bindings.source_surface",
        )?;
        validate_nonempty_text(
            &self.enforcement_rule,
            "resource_bridge.policy_bindings.enforcement_rule",
        )
    }
}

impl RuntimeV2ResourceClaim {
    fn validate(&self, field: &str, expected_kind: &str) -> Result<()> {
        normalize_id(self.claim_id.clone(), &format!("{field}.claim_id"))?;
        validate_resource_kind(&self.resource_kind, &format!("{field}.resource_kind"))?;
        if self.estimate_kind != expected_kind {
            return Err(anyhow!("{field}.estimate_kind must be '{expected_kind}'"));
        }
        if self.estimated_units == 0 {
            return Err(anyhow!("{field}.estimated_units must be positive"));
        }
        validate_nonempty_text(&self.unit_basis, &format!("{field}.unit_basis"))?;
        if !self.bounded_by_policy {
            return Err(anyhow!("{field}.bounded_by_policy must be true"));
        }
        if self.tool_execution_authorized {
            return Err(anyhow!(
                "{field}.tool_execution_authorized must remain false in v0.90.4"
            ));
        }
        validate_nonempty_text(&self.rationale, &format!("{field}.rationale"))?;
        validate_nonempty_vec(&self.evidence_refs, &format!("{field}.evidence_refs"))
    }
}

impl RuntimeV2ToolResourceConstraint {
    fn validate_against(
        &self,
        contract: &RuntimeV2ContractArtifact,
        claims: &[RuntimeV2ResourceClaim],
        field: &str,
    ) -> Result<()> {
        normalize_id(
            self.requirement_id.clone(),
            &format!("{field}.requirement_id"),
        )?;
        validate_nonempty_text(&self.capability, &format!("{field}.capability"))?;
        validate_nonempty_text(&self.adapter_family, &format!("{field}.adapter_family"))?;
        validate_nonempty_text(&self.usage_mode, &format!("{field}.usage_mode"))?;
        normalize_id(
            self.budget_claim_id.clone(),
            &format!("{field}.budget_claim_id"),
        )?;
        if !self.governed_authority_required {
            return Err(anyhow!(
                "{field}.governed_authority_required must stay true"
            ));
        }
        if self.execution_authority_granted {
            return Err(anyhow!(
                "{field}.execution_authority_granted must stay false"
            ));
        }
        validate_nonempty_vec(
            &self.required_evidence,
            &format!("{field}.required_evidence"),
        )?;
        validate_nonempty_text(&self.boundary_note, &format!("{field}.boundary_note"))?;

        let contract_requirement = contract
            .tool_requirements
            .iter()
            .find(|requirement| requirement.requirement_id == self.requirement_id)
            .ok_or_else(|| {
                anyhow!("{field}.requirement_id must bind a parent contract tool requirement")
            })?;
        if contract_requirement.capability != self.capability
            || contract_requirement.adapter_family != self.adapter_family
        {
            return Err(anyhow!(
                "{field} must preserve the parent contract tool requirement capability and adapter"
            ));
        }
        let budget_claim = claims
            .iter()
            .find(|claim| claim.claim_id == self.budget_claim_id)
            .ok_or_else(|| anyhow!("{field}.budget_claim_id must bind a resource claim"))?;
        if budget_claim.resource_kind != "tool_adapter_budget_units" {
            return Err(anyhow!(
                "{field}.budget_claim_id must bind the tool_adapter_budget_units claim"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2BidResourceEstimate {
    fn from_bid(contract: &RuntimeV2ContractArtifact, bid: &RuntimeV2BidArtifact) -> Result<Self> {
        let bid_slug = bid
            .bid_id
            .strip_prefix("bid-")
            .unwrap_or(&bid.bid_id)
            .replace('-', "_");
        let mut claims = vec![
            bridge_claim(
                &format!("{bid_slug}_operator_review_minutes"),
                "operator_review_minutes",
                "bid_estimate",
                overlapping_bid_claim_units(bid, "operator_review_minutes").unwrap_or(30),
                "minutes",
                "bounded reviewer time estimate for award, integration, and residual-risk review",
                &[&format!("{}#resource_claims", bid.artifact_path)],
            ),
            bridge_claim(
                &format!("{bid_slug}_compute_hours"),
                "compute_hours",
                "bid_estimate",
                overlapping_bid_claim_units(bid, "compute_hours").unwrap_or(5),
                "hours",
                "bounded compute estimate for projection handling, trace packaging, and packet assembly",
                &[&format!("{}#resource_claims", bid.artifact_path)],
            ),
            bridge_claim(
                &format!("{bid_slug}_memory_gib"),
                "memory_gib",
                "bid_estimate",
                if bid.bid_id.contains("alpha") { 6 } else { 8 },
                "gib",
                "working-memory estimate for bounded replayable packet assembly",
                &[&bid.artifact_path],
            ),
            bridge_claim(
                &format!("{bid_slug}_attention_items"),
                "attention_items",
                "bid_estimate",
                if bid.bid_id.contains("alpha") { 8 } else { 10 },
                "review_items",
                "review attention estimate for exceptions, commitments, and artifact verification",
                &[&bid.artifact_path],
            ),
            bridge_claim(
                &format!("{bid_slug}_bandwidth_mb"),
                "bandwidth_mb",
                "bid_estimate",
                if bid.bid_id.contains("alpha") { 128 } else { 160 },
                "megabytes",
                "network transfer estimate for bounded packet fetch, upload, and trace exchange",
                &[&bid.artifact_path],
            ),
            bridge_claim(
                &format!("{bid_slug}_artifact_storage_mb"),
                "artifact_storage_mb",
                "bid_estimate",
                overlapping_bid_claim_units(bid, "artifact_storage_mb").unwrap_or(20),
                "megabytes",
                "artifact storage estimate for review packet, manifest, and supporting notes",
                &[&format!("{}#resource_claims", bid.artifact_path)],
            ),
            bridge_claim(
                &format!("{bid_slug}_tool_adapter_budget_units"),
                "tool_adapter_budget_units",
                "bid_estimate",
                1,
                "adapter_session",
                "one bounded adapter-mediated step is budgeted as a deferred constraint rather than an execution grant",
                &[&format!("{}#expected_tool_usage", bid.artifact_path)],
            ),
        ];
        claims.sort_by(|left, right| left.claim_id.cmp(&right.claim_id));

        let tool = bid
            .expected_tool_usage
            .first()
            .ok_or_else(|| anyhow!("resource bridge requires one tool usage entry per bid"))?;
        let budget_claim_id = claims
            .iter()
            .find(|claim| claim.resource_kind == "tool_adapter_budget_units")
            .map(|claim| claim.claim_id.clone())
            .ok_or_else(|| anyhow!("resource bridge missing tool adapter budget claim"))?;
        let estimate = Self {
            bid_id: bid.bid_id.clone(),
            bid_ref: bid.artifact_path.clone(),
            bidder_actor_id: bid.bidder_actor_id.clone(),
            estimate_summary: if bid.bid_id.contains("alpha") {
                "Alpha keeps the lowest bounded operator and compute profile while treating projection rendering as a deferred adapter constraint."
                    .to_string()
            } else {
                "Bravo remains feasible but carries the larger operator and storage profile while still keeping tool needs in non-executable evidence mode."
                    .to_string()
            },
            claims,
            tool_resource_constraints: vec![RuntimeV2ToolResourceConstraint {
                requirement_id: tool.requirement_id.clone(),
                capability: tool.capability.clone(),
                adapter_family: tool.adapter_family.clone(),
                usage_mode: tool.usage_mode.clone(),
                budget_claim_id,
                governed_authority_required: true,
                execution_authority_granted: false,
                required_evidence: tool.required_evidence.clone(),
                boundary_note:
                    "Tool-resource budgeting remains a feasibility constraint only; governed-tool authority is still required before any execution may occur."
                        .to_string(),
            }],
        };
        estimate.validate_against(contract, bid)?;
        Ok(estimate)
    }

    fn validate_against(
        &self,
        contract: &RuntimeV2ContractArtifact,
        bid: &RuntimeV2BidArtifact,
    ) -> Result<()> {
        normalize_id(self.bid_id.clone(), "resource_bridge.bid_id")?;
        validate_relative_path(&self.bid_ref, "resource_bridge.bid_ref")?;
        if self.bid_ref != bid.artifact_path {
            return Err(anyhow!(
                "resource bridge bid estimate must bind the original bid artifact"
            ));
        }
        if self.bidder_actor_id != bid.bidder_actor_id {
            return Err(anyhow!(
                "resource bridge bid estimate must preserve the original bidder_actor_id"
            ));
        }
        validate_nonempty_text(
            &self.estimate_summary,
            "resource_bridge.bid_estimate_summary",
        )?;
        validate_resource_claim_set(&self.claims, "bid_resource_estimate.claims", "bid_estimate")?;

        for original_claim in &bid.resource_claims {
            let bridge_claim = self
                .claims
                .iter()
                .find(|claim| claim.resource_kind == original_claim.resource_kind)
                .ok_or_else(|| {
                    anyhow!(
                        "resource bridge must preserve overlapping bid resource claim '{}'",
                        original_claim.resource_kind
                    )
                })?;
            if bridge_claim.estimated_units != original_claim.estimated_units
                || bridge_claim.unit_basis != original_claim.unit_basis
            {
                return Err(anyhow!(
                    "resource bridge must preserve overlapping bid resource claim '{}'",
                    original_claim.resource_kind
                ));
            }
        }

        if self.tool_resource_constraints.is_empty() {
            return Err(anyhow!(
                "resource bridge bid estimates must include at least one tool resource constraint"
            ));
        }
        for (index, constraint) in self.tool_resource_constraints.iter().enumerate() {
            constraint.validate_against(
                contract,
                &self.claims,
                &format!("bid_resource_estimate.tool_resource_constraints[{index}]"),
            )?;
        }
        Ok(())
    }
}

impl RuntimeV2EvaluationSelectionArtifacts {
    fn recommendation_ref(&self) -> String {
        self.selection.recommendation.selected_bid_ref.clone()
    }
}

fn contract_resource_claims(contract: &RuntimeV2ContractArtifact) -> Vec<RuntimeV2ResourceClaim> {
    vec![
        bridge_claim(
            "contract_compute_hours_limit",
            "compute_hours",
            "contract_constraint",
            6,
            "hours",
            "bounded compute envelope for replayable projection, review, and trace packaging",
            &[&contract.artifact_path],
        ),
        bridge_claim(
            "contract_memory_gib_limit",
            "memory_gib",
            "contract_constraint",
            8,
            "gib",
            "bounded memory envelope for redacted packet assembly",
            &[&contract.artifact_path],
        ),
        bridge_claim(
            "contract_attention_items_limit",
            "attention_items",
            "contract_constraint",
            12,
            "review_items",
            "bounded reviewer attention envelope for trace, evidence, and exception checks",
            &[&format!("{}#evaluation_criteria", contract.artifact_path)],
        ),
        bridge_claim(
            "contract_bandwidth_mb_limit",
            "bandwidth_mb",
            "contract_constraint",
            256,
            "megabytes",
            "bounded transfer envelope for packet exchange and manifest delivery",
            &[&contract.artifact_path],
        ),
        bridge_claim(
            "contract_artifact_storage_mb_limit",
            "artifact_storage_mb",
            "contract_constraint",
            32,
            "megabytes",
            "bounded artifact storage envelope for packet, manifest, and review summary",
            &[&format!("{}#deliverables", contract.artifact_path)],
        ),
        bridge_claim(
            "contract_operator_review_minutes_limit",
            "operator_review_minutes",
            "contract_constraint",
            90,
            "minutes",
            "bounded reviewer and operator time envelope for award, integration, and dispute-ready review",
            &[&format!("{}#process_rules", contract.artifact_path)],
        ),
        bridge_claim(
            "contract_tool_adapter_budget_units_limit",
            "tool_adapter_budget_units",
            "contract_constraint",
            1,
            "adapter_session",
            "one bounded tool-mediated step may be budgeted as a deferred requirement without authorizing execution",
            &[&format!("{}#tool_requirements", contract.artifact_path)],
        ),
    ]
}

fn bridge_claim(
    claim_id: &str,
    resource_kind: &str,
    estimate_kind: &str,
    estimated_units: u64,
    unit_basis: &str,
    rationale: &str,
    evidence_refs: &[&str],
) -> RuntimeV2ResourceClaim {
    RuntimeV2ResourceClaim {
        claim_id: claim_id.to_string(),
        resource_kind: resource_kind.to_string(),
        estimate_kind: estimate_kind.to_string(),
        estimated_units,
        unit_basis: unit_basis.to_string(),
        bounded_by_policy: true,
        tool_execution_authorized: false,
        rationale: rationale.to_string(),
        evidence_refs: strings(evidence_refs),
    }
}

fn overlapping_bid_claim_units(bid: &RuntimeV2BidArtifact, resource_kind: &str) -> Option<u64> {
    bid.resource_claims
        .iter()
        .find(|claim| claim.resource_kind == resource_kind)
        .map(|claim| claim.estimated_units)
}

fn selected_bid_for_selection<'a>(
    selection: &RuntimeV2EvaluationSelectionArtifact,
    valid_bids: &'a [RuntimeV2BidArtifact],
) -> Result<&'a RuntimeV2BidArtifact> {
    valid_bids
        .iter()
        .find(|bid| bid.artifact_path == selection.recommendation.selected_bid_ref)
        .ok_or_else(|| anyhow!("resource bridge selected bid ref must bind a valid bid"))
}

fn validate_resource_claim_set(
    claims: &[RuntimeV2ResourceClaim],
    field: &str,
    expected_kind: &str,
) -> Result<()> {
    let mut seen_claim_ids = BTreeSet::new();
    let mut seen_resource_kinds = BTreeSet::new();
    for (index, claim) in claims.iter().enumerate() {
        claim.validate(&format!("{field}[{index}]"), expected_kind)?;
        if !seen_claim_ids.insert(claim.claim_id.clone()) {
            return Err(anyhow!("{field} contains duplicate claim_id"));
        }
        if !seen_resource_kinds.insert(claim.resource_kind.clone()) {
            return Err(anyhow!("{field} contains duplicate resource_kind"));
        }
    }
    let required_resource_kinds = required_resource_kind_set();
    if seen_resource_kinds != required_resource_kinds {
        return Err(anyhow!(
            "{field} must cover compute_hours, memory_gib, attention_items, bandwidth_mb, artifact_storage_mb, operator_review_minutes, and tool_adapter_budget_units"
        ));
    }
    Ok(())
}

fn validate_resource_kind(value: &str, field: &str) -> Result<()> {
    if !required_resource_kind_set().contains(value) {
        return Err(anyhow!(
            "{field} '{}' is not a supported WP-10 resource kind",
            value
        ));
    }
    Ok(())
}

fn required_resource_kind_set() -> BTreeSet<String> {
    BTreeSet::from_iter(strings(&[
        "compute_hours",
        "memory_gib",
        "attention_items",
        "bandwidth_mb",
        "artifact_storage_mb",
        "operator_review_minutes",
        "tool_adapter_budget_units",
    ]))
}

fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    if !value.starts_with('D') {
        return Err(anyhow!("{field} must start with 'D'"));
    }
    let suffix = value
        .strip_prefix('D')
        .ok_or_else(|| anyhow!("{field} must start with 'D'"))?;
    if suffix.is_empty() || !suffix.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(anyhow!("{field} must contain a numeric demo suffix"));
    }
    Ok(())
}

fn validate_nonempty_vec<T>(values: &[T], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

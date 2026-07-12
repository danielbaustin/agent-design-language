//! Runtime-v2 guild foundation boundary contract.
//!
//! WP-13 proves the smallest useful guild substrate that v0.92 may consume:
//! evidence routing, membership/role records, moderation hooks, witness inputs,
//! and explicit non-claims for v0.93 constitutional governance.

use super::*;
use std::collections::BTreeSet;

pub const RUNTIME_V2_GUILD_FOUNDATION_BOUNDARY_SCHEMA: &str =
    "runtime_v2.guild_foundation_boundary.v1";
pub const RUNTIME_V2_GUILD_FOUNDATION_BOUNDARY_PATH: &str =
    "runtime_v2/guild_foundation_boundary/boundary_packet.json";
pub const RUNTIME_V2_GUILD_FOUNDATION_BOUNDARY_TEST_MARKER: &str =
    "runtime_v2_guild_foundation_boundary";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GuildFoundationBoundaryPacket {
    pub schema_version: String,
    pub boundary_id: String,
    pub milestone: String,
    pub wp: String,
    pub issue: String,
    pub artifact_path: String,
    pub activation_posture: String,
    pub minimum_foundation_surfaces: Vec<String>,
    pub v092_consumption_allowlist: Vec<String>,
    pub governance_handoff: Vec<RuntimeV2GuildGovernanceHandoff>,
    pub required_promotion_gates: Vec<String>,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GuildGovernanceHandoff {
    pub surface_id: String,
    pub status: String,
    pub target_milestone: String,
    pub v092_consequence: String,
}

impl RuntimeV2GuildFoundationBoundaryPacket {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_GUILD_FOUNDATION_BOUNDARY_SCHEMA.to_string(),
            boundary_id: "runtime-v2-guild-foundation-boundary-4755".to_string(),
            milestone: "v0.91.7".to_string(),
            wp: "WP-13".to_string(),
            issue: "#4755".to_string(),
            artifact_path: RUNTIME_V2_GUILD_FOUNDATION_BOUNDARY_PATH.to_string(),
            activation_posture: "foundation_proof_for_v0_92_governance_handoff".to_string(),
            minimum_foundation_surfaces: vec![
                "guild_identity_record".to_string(),
                "member_role_registry".to_string(),
                "governed_membership_event_log".to_string(),
                "moderation_escalation_hook".to_string(),
                "witness_evidence_reference".to_string(),
                "v093_governance_handoff_anchor".to_string(),
            ],
            v092_consumption_allowlist: vec![
                "birthday_governance_context".to_string(),
                "identity_witness_evidence_routing".to_string(),
                "community_memory_boundary_language".to_string(),
                "future_governance_issue_inputs".to_string(),
            ],
            governance_handoff: vec![
                governance_handoff(
                    "constitutional-citizenship",
                    "v0.93",
                    "No v0.92 citizenship, rights, duties, or constitutional membership claim.",
                ),
                governance_handoff(
                    "polis-governance",
                    "v0.93",
                    "No v0.92 polis decision authority, voting system, or social-contract claim.",
                ),
                governance_handoff(
                    "delegated-authority",
                    "v0.93_or_later",
                    "No v0.92 delegated governance authority or binding representative action claim.",
                ),
                governance_handoff(
                    "public-guild-product",
                    "post_mvp",
                    "No v0.92 product, launch, marketplace, or community-platform readiness claim.",
                ),
            ],
            required_promotion_gates: vec![
                "operator_approval".to_string(),
                "tracked_issue".to_string(),
                "bounded_test_plan".to_string(),
                "security_governance_review".to_string(),
                "retained_proof_artifact".to_string(),
                "public_claim_review".to_string(),
            ],
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_guild_foundation_boundary"
                    .to_string(),
            ],
            claim_boundary: "Runtime v2 guild foundation boundary for v0.92: MVP guild evidence routing is proven for birthday/governance handoff context only; constitutional governance, polis authority, delegated authority, and public guild product readiness are not claimed.".to_string(),
            non_claims: vec![
                "constitutional_citizenship".to_string(),
                "polis_governance_runtime".to_string(),
                "delegated_governance_authority".to_string(),
                "binding_collective_decision_making".to_string(),
                "public_guild_product_readiness".to_string(),
                "v0_92_governance_completion".to_string(),
            ],
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_exact(
            &self.schema_version,
            RUNTIME_V2_GUILD_FOUNDATION_BOUNDARY_SCHEMA,
            "guild_foundation_boundary.schema_version",
        )?;
        normalize_id(
            self.boundary_id.clone(),
            "guild_foundation_boundary.boundary_id",
        )?;
        require_exact(
            &self.milestone,
            "v0.91.7",
            "guild_foundation_boundary.milestone",
        )?;
        require_exact(&self.wp, "WP-13", "guild_foundation_boundary.wp")?;
        require_exact(&self.issue, "#4755", "guild_foundation_boundary.issue")?;
        require_exact(
            &self.artifact_path,
            RUNTIME_V2_GUILD_FOUNDATION_BOUNDARY_PATH,
            "guild_foundation_boundary.artifact_path",
        )?;
        validate_relative_path(
            &self.artifact_path,
            "guild_foundation_boundary.artifact_path",
        )?;
        require_exact(
            &self.activation_posture,
            "foundation_proof_for_v0_92_governance_handoff",
            "guild_foundation_boundary.activation_posture",
        )?;
        validate_minimum_surfaces(&self.minimum_foundation_surfaces)?;
        validate_consumption_allowlist(&self.v092_consumption_allowlist)?;
        validate_governance_handoff(&self.governance_handoff)?;
        validate_required_gates(&self.required_promotion_gates)?;
        validate_command_list(&self.validation_commands)?;
        ensure_contains(
            &self.claim_boundary,
            "handoff context only",
            "guild boundary claim must preserve v0.92 handoff-only posture",
        )?;
        ensure_contains(
            &self.claim_boundary,
            "not claimed",
            "guild boundary claim must deny governance/product overclaims",
        )?;
        for required in [
            "constitutional_citizenship",
            "polis_governance_runtime",
            "delegated_governance_authority",
            "binding_collective_decision_making",
            "public_guild_product_readiness",
            "v0_92_governance_completion",
        ] {
            ensure_contains_in_list(
                &self.non_claims,
                required,
                "guild boundary non-claims must include every unsafe v0.92 claim",
            )?;
        }
        Ok(())
    }

    pub fn canonicalized(&self) -> Result<Self> {
        self.validate()?;
        let mut canonical = self.clone();
        canonical.minimum_foundation_surfaces.sort();
        canonical.v092_consumption_allowlist.sort();
        canonical
            .governance_handoff
            .sort_by(|a, b| a.surface_id.cmp(&b.surface_id));
        canonical.required_promotion_gates.sort();
        canonical.validation_commands.sort();
        canonical.non_claims.sort();
        canonical.validate()?;
        Ok(canonical)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(&self.canonicalized()?)
            .context("serialize Runtime v2 guild foundation boundary packet")
    }
}

pub fn runtime_v2_guild_foundation_boundary_contract(
) -> Result<RuntimeV2GuildFoundationBoundaryPacket> {
    let packet = RuntimeV2GuildFoundationBoundaryPacket::prototype();
    packet.validate()?;
    Ok(packet)
}

fn governance_handoff(
    surface_id: &str,
    target_milestone: &str,
    v092_consequence: &str,
) -> RuntimeV2GuildGovernanceHandoff {
    RuntimeV2GuildGovernanceHandoff {
        surface_id: surface_id.to_string(),
        status: "deferred_to_governance_handoff".to_string(),
        target_milestone: target_milestone.to_string(),
        v092_consequence: v092_consequence.to_string(),
    }
}

fn validate_minimum_surfaces(surfaces: &[String]) -> Result<()> {
    ensure_no_duplicates(
        surfaces.iter().map(String::as_str),
        "guild_foundation_boundary.minimum_foundation_surfaces",
    )?;
    let actual = surfaces.iter().map(String::as_str).collect::<BTreeSet<_>>();
    let required = BTreeSet::from([
        "governed_membership_event_log",
        "guild_identity_record",
        "member_role_registry",
        "moderation_escalation_hook",
        "v093_governance_handoff_anchor",
        "witness_evidence_reference",
    ]);
    if actual != required {
        return Err(anyhow!(
            "guild_foundation_boundary.minimum_foundation_surfaces must remain the exact MVP foundation set"
        ));
    }
    Ok(())
}

fn validate_consumption_allowlist(consumption: &[String]) -> Result<()> {
    ensure_no_duplicates(
        consumption.iter().map(String::as_str),
        "guild_foundation_boundary.v092_consumption_allowlist",
    )?;
    let actual = consumption
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let required = BTreeSet::from([
        "birthday_governance_context",
        "community_memory_boundary_language",
        "future_governance_issue_inputs",
        "identity_witness_evidence_routing",
    ]);
    if actual != required {
        return Err(anyhow!(
            "guild_foundation_boundary.v092_consumption_allowlist must remain exactly the handoff-context allowlist"
        ));
    }
    Ok(())
}

fn validate_governance_handoff(surfaces: &[RuntimeV2GuildGovernanceHandoff]) -> Result<()> {
    ensure_no_duplicates(
        surfaces.iter().map(|surface| surface.surface_id.as_str()),
        "guild_foundation_boundary.governance_handoff",
    )?;
    let ids = surfaces
        .iter()
        .map(|surface| surface.surface_id.as_str())
        .collect::<BTreeSet<_>>();
    let required = BTreeSet::from([
        "constitutional-citizenship",
        "delegated-authority",
        "polis-governance",
        "public-guild-product",
    ]);
    if ids != required {
        return Err(anyhow!(
            "guild_foundation_boundary.governance_handoff must name constitutional citizenship, polis governance, delegated authority, and public guild product"
        ));
    }
    for surface in surfaces {
        require_exact(
            &surface.status,
            "deferred_to_governance_handoff",
            "guild_foundation_boundary.governance_handoff.status",
        )?;
        match surface.surface_id.as_str() {
            "constitutional-citizenship" => {
                require_exact(
                    &surface.target_milestone,
                    "v0.93",
                    "guild_foundation_boundary.constitutional_citizenship.target_milestone",
                )?;
                ensure_contains(
                    &surface.v092_consequence,
                    "No v0.92 citizenship",
                    "constitutional citizenship must deny v0.92 citizenship claims",
                )?;
            }
            "polis-governance" => {
                require_exact(
                    &surface.target_milestone,
                    "v0.93",
                    "guild_foundation_boundary.polis_governance.target_milestone",
                )?;
                ensure_contains(
                    &surface.v092_consequence,
                    "No v0.92 polis decision authority",
                    "polis governance must deny v0.92 decision-authority claims",
                )?;
            }
            "delegated-authority" => {
                require_exact(
                    &surface.target_milestone,
                    "v0.93_or_later",
                    "guild_foundation_boundary.delegated_authority.target_milestone",
                )?;
                ensure_contains(
                    &surface.v092_consequence,
                    "No v0.92 delegated governance authority",
                    "delegated authority must deny v0.92 authority claims",
                )?;
            }
            "public-guild-product" => {
                require_exact(
                    &surface.target_milestone,
                    "post_mvp",
                    "guild_foundation_boundary.public_guild_product.target_milestone",
                )?;
                ensure_contains(
                    &surface.v092_consequence,
                    "No v0.92 product",
                    "public guild product must deny v0.92 product-readiness claims",
                )?;
            }
            _ => unreachable!("governance handoff surface ids were checked above"),
        }
    }
    Ok(())
}

fn validate_required_gates(gates: &[String]) -> Result<()> {
    ensure_no_duplicates(
        gates.iter().map(String::as_str),
        "guild_foundation_boundary.required_promotion_gates",
    )?;
    let actual = gates.iter().map(String::as_str).collect::<BTreeSet<_>>();
    let required = BTreeSet::from([
        "bounded_test_plan",
        "operator_approval",
        "public_claim_review",
        "retained_proof_artifact",
        "security_governance_review",
        "tracked_issue",
    ]);
    if actual != required {
        return Err(anyhow!(
            "guild_foundation_boundary.required_promotion_gates must retain the full promotion gate set"
        ));
    }
    Ok(())
}

fn validate_command_list(commands: &[String]) -> Result<()> {
    if commands.is_empty() {
        return Err(anyhow!(
            "guild_foundation_boundary.validation_commands must not be empty"
        ));
    }
    if !commands
        .iter()
        .any(|command| command.contains(RUNTIME_V2_GUILD_FOUNDATION_BOUNDARY_TEST_MARKER))
    {
        return Err(anyhow!(
            "guild_foundation_boundary.validation_commands must include the focused runtime test marker"
        ));
    }
    for command in commands {
        if command.trim().is_empty() || command.contains("/Users/") {
            return Err(anyhow!(
                "guild_foundation_boundary.validation_commands must be non-empty and path-safe"
            ));
        }
    }
    Ok(())
}

fn ensure_contains(value: &str, needle: &str, message: &str) -> Result<()> {
    if value.contains(needle) {
        Ok(())
    } else {
        Err(anyhow!("{message}: missing '{needle}'"))
    }
}

fn ensure_contains_in_list(values: &[String], needle: &str, message: &str) -> Result<()> {
    if values.iter().any(|value| value.contains(needle)) {
        Ok(())
    } else {
        Err(anyhow!("{message}: missing '{needle}'"))
    }
}

fn ensure_no_duplicates<'a>(values: impl IntoIterator<Item = &'a str>, field: &str) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(anyhow!(
                "{field} must not contain duplicate entry '{value}'"
            ));
        }
    }
    Ok(())
}

fn require_exact(actual: &str, expected: &str, field: &str) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(anyhow!("{field} must be '{expected}', got '{actual}'"))
    }
}

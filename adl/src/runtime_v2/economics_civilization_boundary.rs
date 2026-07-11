//! Runtime-v2 economics and civilization boundary contract.
//!
//! WP-13 keeps economics as v0.92 context unless an operator-approved
//! activation test is explicitly promoted. This executable packet prevents
//! payment, market, civilization, or autonomous-economy claims from entering the
//! birthday readiness path as ambient prose.

use super::*;
use std::collections::BTreeSet;

pub const RUNTIME_V2_ECONOMICS_CIVILIZATION_BOUNDARY_SCHEMA: &str =
    "runtime_v2.economics_civilization_boundary.v1";
pub const RUNTIME_V2_ECONOMICS_CIVILIZATION_BOUNDARY_PATH: &str =
    "runtime_v2/economics_civilization_boundary/boundary_packet.json";
pub const RUNTIME_V2_ECONOMICS_CIVILIZATION_BOUNDARY_TEST_MARKER: &str =
    "runtime_v2_economics_civilization_boundary";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2EconomicsCivilizationBoundaryPacket {
    pub schema_version: String,
    pub boundary_id: String,
    pub milestone: String,
    pub wp: String,
    pub issue: String,
    pub artifact_path: String,
    pub activation_posture: String,
    pub allowed_v092_consumption: Vec<String>,
    pub promoted_activation_tests: Vec<String>,
    pub postponed_surfaces: Vec<RuntimeV2EconomicsPostponedSurface>,
    pub required_promotion_gates: Vec<String>,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2EconomicsPostponedSurface {
    pub surface_id: String,
    pub title: String,
    pub status: String,
    pub target_milestone: String,
    pub v092_consequence: String,
}

impl RuntimeV2EconomicsCivilizationBoundaryPacket {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_ECONOMICS_CIVILIZATION_BOUNDARY_SCHEMA.to_string(),
            boundary_id: "runtime-v2-economics-civilization-boundary-4754".to_string(),
            milestone: "v0.91.7".to_string(),
            wp: "WP-13".to_string(),
            issue: "#4754".to_string(),
            artifact_path: RUNTIME_V2_ECONOMICS_CIVILIZATION_BOUNDARY_PATH.to_string(),
            activation_posture: "context_only_for_v0_92".to_string(),
            allowed_v092_consumption: vec![
                "scheduler_and_resource_stewardship_context".to_string(),
                "public_claim_non_claims".to_string(),
                "future_issue_routing_inputs".to_string(),
            ],
            promoted_activation_tests: Vec::new(),
            postponed_surfaces: vec![
                postponed_surface(
                    "payments-settlement",
                    "Payments and settlement",
                    "v0.94.1_or_later",
                    "No v0.92 activation dependency unless separately promoted.",
                ),
                postponed_surface(
                    "market-mechanisms",
                    "Market mechanisms",
                    "post_mvp",
                    "No market proof or marketplace readiness claim in v0.92.",
                ),
                postponed_surface(
                    "civilization-economics",
                    "Civilization-scale economics",
                    "post_mvp",
                    "No civilization or autonomous economy claim in v0.92.",
                ),
                postponed_surface(
                    "runtime-economic-optimization",
                    "Runtime economic optimization",
                    "post_mvp",
                    "No optimizer-governed agent behavior claim in v0.92.",
                ),
            ],
            required_promotion_gates: vec![
                "operator_approval".to_string(),
                "tracked_issue".to_string(),
                "bounded_test_plan".to_string(),
                "security_governance_review".to_string(),
                "retained_proof_artifact".to_string(),
            ],
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_economics_civilization_boundary".to_string(),
            ],
            claim_boundary: "Runtime v2 economics/civilization boundary for v0.92: context-only consumption with executable non-claims; no payment, market, civilization, autonomous-economy, or product-readiness implementation is claimed.".to_string(),
            non_claims: vec![
                "payments_implementation".to_string(),
                "settlement_implementation".to_string(),
                "market_mechanism_proof".to_string(),
                "civilization_runtime".to_string(),
                "autonomous_economy".to_string(),
                "runtime_economic_optimizer".to_string(),
                "v0_92_product_readiness".to_string(),
            ],
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_exact(
            &self.schema_version,
            RUNTIME_V2_ECONOMICS_CIVILIZATION_BOUNDARY_SCHEMA,
            "economics_civilization_boundary.schema_version",
        )?;
        normalize_id(
            self.boundary_id.clone(),
            "economics_civilization_boundary.boundary_id",
        )?;
        require_exact(
            &self.milestone,
            "v0.91.7",
            "economics_civilization_boundary.milestone",
        )?;
        require_exact(&self.wp, "WP-13", "economics_civilization_boundary.wp")?;
        require_exact(
            &self.issue,
            "#4754",
            "economics_civilization_boundary.issue",
        )?;
        require_exact(
            &self.artifact_path,
            RUNTIME_V2_ECONOMICS_CIVILIZATION_BOUNDARY_PATH,
            "economics_civilization_boundary.artifact_path",
        )?;
        validate_relative_path(
            &self.artifact_path,
            "economics_civilization_boundary.artifact_path",
        )?;
        require_exact(
            &self.activation_posture,
            "context_only_for_v0_92",
            "economics_civilization_boundary.activation_posture",
        )?;
        if !self.promoted_activation_tests.is_empty() {
            return Err(anyhow!(
                "economics_civilization_boundary promoted activation tests require a separate operator-approved issue"
            ));
        }
        validate_allowed_consumption(&self.allowed_v092_consumption)?;
        validate_postponed_surfaces(&self.postponed_surfaces)?;
        validate_required_gates(&self.required_promotion_gates)?;
        validate_command_list(&self.validation_commands)?;
        ensure_contains(
            &self.claim_boundary,
            "context-only",
            "economics boundary claim must preserve context-only v0.92 posture",
        )?;
        ensure_contains(
            &self.claim_boundary,
            "no payment, market, civilization, autonomous-economy, or product-readiness",
            "economics boundary claim must deny unsafe activation claims",
        )?;
        for required in [
            "payments_implementation",
            "settlement_implementation",
            "market_mechanism_proof",
            "civilization_runtime",
            "autonomous_economy",
            "runtime_economic_optimizer",
            "v0_92_product_readiness",
        ] {
            ensure_contains_in_list(
                &self.non_claims,
                required,
                "economics boundary non-claims must include every unsafe v0.92 claim",
            )?;
        }
        Ok(())
    }

    pub fn canonicalized(&self) -> Result<Self> {
        self.validate()?;
        let mut canonical = self.clone();
        canonical.allowed_v092_consumption.sort();
        canonical
            .postponed_surfaces
            .sort_by(|a, b| a.surface_id.cmp(&b.surface_id));
        canonical.required_promotion_gates.sort();
        canonical.validation_commands.sort();
        canonical.non_claims.sort();
        canonical.validate()?;
        Ok(canonical)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(&self.canonicalized()?)
            .context("serialize Runtime v2 economics/civilization boundary packet")
    }
}

pub fn runtime_v2_economics_civilization_boundary_contract(
) -> Result<RuntimeV2EconomicsCivilizationBoundaryPacket> {
    let packet = RuntimeV2EconomicsCivilizationBoundaryPacket::prototype();
    packet.validate()?;
    Ok(packet)
}

fn postponed_surface(
    surface_id: &str,
    title: &str,
    target_milestone: &str,
    v092_consequence: &str,
) -> RuntimeV2EconomicsPostponedSurface {
    RuntimeV2EconomicsPostponedSurface {
        surface_id: surface_id.to_string(),
        title: title.to_string(),
        status: "postponed".to_string(),
        target_milestone: target_milestone.to_string(),
        v092_consequence: v092_consequence.to_string(),
    }
}

fn validate_allowed_consumption(consumption: &[String]) -> Result<()> {
    let allowed = BTreeSet::from([
        "future_issue_routing_inputs",
        "public_claim_non_claims",
        "scheduler_and_resource_stewardship_context",
    ]);
    let actual = consumption
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if actual != allowed {
        return Err(anyhow!(
            "economics_civilization_boundary.allowed_v092_consumption must remain exactly the context-only allowlist"
        ));
    }
    Ok(())
}

fn validate_postponed_surfaces(surfaces: &[RuntimeV2EconomicsPostponedSurface]) -> Result<()> {
    let ids = surfaces
        .iter()
        .map(|surface| surface.surface_id.as_str())
        .collect::<BTreeSet<_>>();
    let required = BTreeSet::from([
        "civilization-economics",
        "market-mechanisms",
        "payments-settlement",
        "runtime-economic-optimization",
    ]);
    if ids != required {
        return Err(anyhow!(
            "economics_civilization_boundary.postponed_surfaces must name payments, market, civilization, and runtime optimization"
        ));
    }
    for surface in surfaces {
        require_exact(
            &surface.status,
            "postponed",
            "economics_civilization_boundary.postponed_surface.status",
        )?;
        match surface.surface_id.as_str() {
            "payments-settlement" => {
                require_exact(
                    &surface.target_milestone,
                    "v0.94.1_or_later",
                    "economics_civilization_boundary.payments_settlement.target_milestone",
                )?;
                ensure_contains(
                    &surface.v092_consequence,
                    "No v0.92 activation dependency",
                    "payments and settlement must deny v0.92 activation dependency",
                )?;
            }
            "market-mechanisms" => {
                require_exact(
                    &surface.target_milestone,
                    "post_mvp",
                    "economics_civilization_boundary.market_mechanisms.target_milestone",
                )?;
                ensure_contains(
                    &surface.v092_consequence,
                    "No market proof",
                    "market mechanisms must deny v0.92 market proof",
                )?;
            }
            "civilization-economics" => {
                require_exact(
                    &surface.target_milestone,
                    "post_mvp",
                    "economics_civilization_boundary.civilization_economics.target_milestone",
                )?;
                ensure_contains(
                    &surface.v092_consequence,
                    "No civilization or autonomous economy claim",
                    "civilization economics must deny v0.92 civilization claims",
                )?;
            }
            "runtime-economic-optimization" => {
                require_exact(
                    &surface.target_milestone,
                    "post_mvp",
                    "economics_civilization_boundary.runtime_economic_optimization.target_milestone",
                )?;
                ensure_contains(
                    &surface.v092_consequence,
                    "No optimizer-governed agent behavior claim",
                    "runtime economic optimization must deny v0.92 optimizer claims",
                )?;
            }
            _ => unreachable!("postponed surface ids were checked above"),
        }
    }
    Ok(())
}

fn validate_required_gates(gates: &[String]) -> Result<()> {
    let actual = gates.iter().map(String::as_str).collect::<BTreeSet<_>>();
    let required = BTreeSet::from([
        "bounded_test_plan",
        "operator_approval",
        "retained_proof_artifact",
        "security_governance_review",
        "tracked_issue",
    ]);
    if actual != required {
        return Err(anyhow!(
            "economics_civilization_boundary.required_promotion_gates must retain the full promotion gate set"
        ));
    }
    Ok(())
}

fn validate_command_list(commands: &[String]) -> Result<()> {
    if commands.is_empty() {
        return Err(anyhow!(
            "economics_civilization_boundary.validation_commands must not be empty"
        ));
    }
    for command in commands {
        if command.trim().is_empty() || command.contains("/Users/") {
            return Err(anyhow!(
                "economics_civilization_boundary.validation_commands must be non-empty and path-safe"
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

fn require_exact(actual: &str, expected: &str, field: &str) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(anyhow!("{field} must be '{expected}', got '{actual}'"))
    }
}

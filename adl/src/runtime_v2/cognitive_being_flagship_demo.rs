//! Runtime-v2 cognitive-being flagship demo proof bundle.
//!
//! This module composes the v0.91 moral-governance, cognitive-being,
//! structured-planning/review, and secure-local-comms artifacts into one
//! reviewer-facing D13 proof surface.

use super::*;
use crate::agent_comms::{
    acip_a2a_fixture_set_v1, acip_invocation_fixture_set_v1, acip_proof_demo_packet_v1,
    validate_acip_a2a_fixture_set_v1, validate_acip_invocation_fixture_set_v1,
    validate_acip_proof_demo_packet_v1,
};
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_PROOF_SCHEMA: &str =
    "runtime_v2.cognitive_being_flagship_proof_packet.v1";
pub const RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_SECTION_SCHEMA: &str =
    "runtime_v2.cognitive_being_flagship_section.v1";

pub const RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_PROOF_PATH: &str =
    "runtime_v2/cognitive_being/flagship_proof_packet.json";
pub const RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_SECTION_PATH: &str =
    "runtime_v2/cognitive_being/flagship_sections.json";
pub const RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_REPORT_PATH: &str =
    "runtime_v2/cognitive_being/flagship_reviewer_report.md";

pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_MORAL_TRACE_PATH: &str =
    "runtime_v2/cognitive_being/support/moral_trace_examples.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_OUTCOME_LINKAGE_PATH: &str =
    "runtime_v2/cognitive_being/support/outcome_linkage_examples.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_TRAJECTORY_PATH: &str =
    "runtime_v2/cognitive_being/support/moral_trajectory_review_packet.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ANTI_HARM_PATH: &str =
    "runtime_v2/cognitive_being/support/anti_harm_trajectory_constraint_packet.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_WELLBEING_PATH: &str =
    "runtime_v2/cognitive_being/support/wellbeing_diagnostic_packet.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_KINDNESS_PATH: &str =
    "runtime_v2/cognitive_being/support/kindness_review_packet.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_HUMOR_PATH: &str =
    "runtime_v2/cognitive_being/support/humor_and_absurdity_review_packet.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_AFFECT_PATH: &str =
    "runtime_v2/cognitive_being/support/affect_reasoning_control_packet.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_MORAL_RESOURCES_PATH: &str =
    "runtime_v2/cognitive_being/support/moral_resource_review_packet.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_CULTIVATION_PATH: &str =
    "runtime_v2/cognitive_being/support/cultivating_intelligence_review_packet.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_PROOF_PATH: &str =
    "runtime_v2/cognitive_being/support/acip_proof_demo_packet_v1.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_INVOCATION_PATH: &str =
    "runtime_v2/cognitive_being/support/acip_invocation_fixture_set_v1.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_A2A_PATH: &str =
    "runtime_v2/cognitive_being/support/acip_a2a_fixture_set_v1.json";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_SPP_PATH: &str =
    "runtime_v2/cognitive_being/support/structured_planning_prompt.md";
pub const RUNTIME_V2_COGNITIVE_BEING_SUPPORT_SRP_PATH: &str =
    "runtime_v2/cognitive_being/support/structured_review_policy.md";

const EXPECTED_SECTION_IDS: [&str; 6] = [
    "moral_trace_and_trajectory",
    "anti_harm_and_wellbeing",
    "kindness_affect_reframing",
    "moral_resources_and_cultivation",
    "structured_planning_and_review",
    "secure_local_comms",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CognitiveBeingFlagshipSection {
    pub schema_version: String,
    pub section_id: String,
    pub title: String,
    pub summary: String,
    pub primary_artifact_refs: Vec<String>,
    pub validation_refs: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CognitiveBeingFlagshipProofPacket {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub milestone: String,
    pub artifact_path: String,
    pub reviewer_report_ref: String,
    pub section_ref: String,
    pub source_docs: Vec<String>,
    pub section_ids: Vec<String>,
    pub required_artifact_refs: Vec<String>,
    pub reviewer_command: String,
    pub validation_commands: Vec<String>,
    pub proof_summary: String,
    pub proof_classification: String,
    pub non_claims: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CognitiveBeingFlagshipArtifacts {
    pub proof_packet: RuntimeV2CognitiveBeingFlagshipProofPacket,
    pub sections: Vec<RuntimeV2CognitiveBeingFlagshipSection>,
    pub reviewer_report_markdown: String,
}

impl RuntimeV2CognitiveBeingFlagshipArtifacts {
    pub fn prototype() -> Result<Self> {
        validate_moral_trace_examples(&moral_trace_required_examples())?;
        validate_outcome_linkage_examples(&outcome_linkage_required_examples())?;
        anti_harm_trajectory_constraint_packet()?;
        wellbeing_diagnostic_packet()?;
        kindness_review_packet()?;
        humor_and_absurdity_review_packet()?;
        affect_reasoning_control_packet()?;
        moral_resource_review_packet()?;
        cultivating_intelligence_review_packet()?;
        validate_acip_proof_demo_packet_v1(&acip_proof_demo_packet_v1())?;
        validate_acip_invocation_fixture_set_v1(&acip_invocation_fixture_set_v1())?;
        validate_acip_a2a_fixture_set_v1(&acip_a2a_fixture_set_v1())?;

        let sections = flagship_sections();
        let proof_packet = RuntimeV2CognitiveBeingFlagshipProofPacket::from_sections(&sections)?;
        let reviewer_report_markdown =
            render_cognitive_being_flagship_reviewer_report(&proof_packet, &sections)?;
        let artifacts = Self {
            proof_packet,
            sections,
            reviewer_report_markdown,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        for section in &self.sections {
            section.validate()?;
        }
        self.proof_packet.validate_against(&self.sections)?;
        validate_cognitive_being_flagship_report(
            &self.proof_packet,
            &self.sections,
            &self.reviewer_report_markdown,
        )
    }

    pub fn proof_packet_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.proof_packet.validate_against(&self.sections)?;
        serde_json::to_vec_pretty(&self.proof_packet)
            .context("serialize Runtime v2 cognitive-being flagship proof packet")
    }

    pub fn sections_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        for section in &self.sections {
            section.validate()?;
        }
        serde_json::to_vec_pretty(&self.sections)
            .context("serialize Runtime v2 cognitive-being flagship sections")
    }

    pub fn execution_summary(&self) -> Result<String> {
        self.validate()?;
        let mut lines = vec![
            "D13 cognitive-being flagship proof:".to_string(),
            format!("- proof packet: {}", self.proof_packet.artifact_path),
            format!(
                "- reviewer report: {}",
                self.proof_packet.reviewer_report_ref
            ),
            format!("- section roster: {}", self.proof_packet.section_ref),
        ];
        for section in &self.sections {
            lines.push(format!(
                "- {} :: {} -> {}",
                section.section_id,
                section.title,
                section.primary_artifact_refs.join(", ")
            ));
        }
        Ok(lines.join("\n"))
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();

        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_MORAL_TRACE_PATH,
            serde_json::to_vec_pretty(&moral_trace_required_examples())
                .context("serialize moral trace examples")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_OUTCOME_LINKAGE_PATH,
            serde_json::to_vec_pretty(&outcome_linkage_required_examples())
                .context("serialize outcome linkage examples")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_TRAJECTORY_PATH,
            serde_json::to_vec_pretty(&moral_trajectory_review_packet()?)
                .context("serialize moral trajectory review packet")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ANTI_HARM_PATH,
            serde_json::to_vec_pretty(&anti_harm_trajectory_constraint_packet()?)
                .context("serialize anti-harm trajectory constraint packet")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_WELLBEING_PATH,
            serde_json::to_vec_pretty(&wellbeing_diagnostic_packet()?)
                .context("serialize wellbeing diagnostic packet")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_KINDNESS_PATH,
            serde_json::to_vec_pretty(&kindness_review_packet()?)
                .context("serialize kindness review packet")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_HUMOR_PATH,
            serde_json::to_vec_pretty(&humor_and_absurdity_review_packet()?)
                .context("serialize humor and absurdity review packet")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_AFFECT_PATH,
            serde_json::to_vec_pretty(&affect_reasoning_control_packet()?)
                .context("serialize affect reasoning-control packet")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_MORAL_RESOURCES_PATH,
            serde_json::to_vec_pretty(&moral_resource_review_packet()?)
                .context("serialize moral resource review packet")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_CULTIVATION_PATH,
            serde_json::to_vec_pretty(&cultivating_intelligence_review_packet()?)
                .context("serialize cultivating intelligence review packet")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_PROOF_PATH,
            serde_json::to_vec_pretty(&acip_proof_demo_packet_v1())
                .context("serialize ACIP proof demo packet")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_INVOCATION_PATH,
            serde_json::to_vec_pretty(&acip_invocation_fixture_set_v1())
                .context("serialize ACIP invocation fixture set")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_A2A_PATH,
            serde_json::to_vec_pretty(&acip_a2a_fixture_set_v1())
                .context("serialize ACIP A2A fixture set")?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_SPP_PATH,
            synthetic_spp_markdown().into_bytes(),
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_SUPPORT_SRP_PATH,
            synthetic_srp_markdown().into_bytes(),
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_SECTION_PATH,
            self.sections_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_REPORT_PATH,
            self.reviewer_report_markdown.as_bytes().to_vec(),
        )?;
        write_relative(
            root,
            RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_PROOF_PATH,
            self.proof_packet_pretty_json_bytes()?,
        )?;
        self.validate_written_bundle(root)
    }

    pub(crate) fn validate_written_bundle(&self, root: &Path) -> Result<()> {
        let mut required = BTreeSet::from([
            self.proof_packet.artifact_path.clone(),
            self.proof_packet.reviewer_report_ref.clone(),
            self.proof_packet.section_ref.clone(),
        ]);
        required.extend(self.proof_packet.required_artifact_refs.iter().cloned());
        for rel_path in required {
            if !root.join(&rel_path).is_file() {
                return Err(anyhow!(
                    "cognitive-being flagship bundle missing required artifact {}",
                    rel_path
                ));
            }
        }
        Ok(())
    }
}

impl RuntimeV2CognitiveBeingFlagshipProofPacket {
    pub(crate) fn from_sections(
        sections: &[RuntimeV2CognitiveBeingFlagshipSection],
    ) -> Result<Self> {
        validate_expected_sections(sections)?;
        let mut required_artifact_refs = BTreeSet::new();
        for section in sections {
            for artifact in &section.primary_artifact_refs {
                required_artifact_refs.insert(artifact.clone());
            }
        }
        Ok(Self {
            schema_version: RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_PROOF_SCHEMA.to_string(),
            proof_id: "v0-91-d13-cognitive-being-flagship-0001".to_string(),
            demo_id: "D13".to_string(),
            milestone: "v0.91".to_string(),
            artifact_path: RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_PROOF_PATH.to_string(),
            reviewer_report_ref: RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_REPORT_PATH.to_string(),
            section_ref: RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_SECTION_PATH.to_string(),
            source_docs: vec![
                "demos/v0.91/cognitive_being_flagship_demo.md".to_string(),
                "docs/milestones/v0.91/COGNITIVE_BEING_FEATURES_v0.91.md".to_string(),
                "docs/milestones/v0.91/features/WELLBEING_AND_HAPPINESS.md".to_string(),
                "docs/milestones/v0.91/features/KINDNESS.md".to_string(),
                "docs/milestones/v0.91/features/HUMOR_AND_ABSURDITY.md".to_string(),
                "docs/milestones/v0.91/features/AFFECT_REASONING_CONTROL.md".to_string(),
                "docs/milestones/v0.91/features/STRUCTURED_PLANNING_AND_PLAN_REVIEW.md"
                    .to_string(),
                "docs/milestones/v0.91/features/STRUCTURED_REVIEW_POLICY_AND_SRP.md"
                    .to_string(),
                "docs/milestones/v0.91/features/A2A_EXTERNAL_AGENT_ADAPTER.md".to_string(),
            ],
            section_ids: sections.iter().map(|section| section.section_id.clone()).collect(),
            required_artifact_refs: required_artifact_refs.into_iter().collect(),
            reviewer_command:
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 cognitive-being-flagship-demo --out artifacts/v091/demo-d13-cognitive-being-flagship"
                    .to_string(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_cognitive_being_flagship_demo -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml trace_runtime_v2_cognitive_being_flagship_demo -- --nocapture".to_string(),
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 cognitive-being-flagship-demo --out artifacts/v091/demo-d13-cognitive-being-flagship".to_string(),
                "git diff --check".to_string(),
            ],
            proof_summary:
                "D13 composes the v0.91 moral-trace, anti-harm, wellbeing, kindness, affect/reframing, moral-resource, structured-planning/review, and secure local comms surfaces into one reviewable cognitive-being flagship bundle."
                    .to_string(),
            proof_classification: "proving".to_string(),
            non_claims: vec![
                "does not claim a first true birthday or birthday completion event".to_string(),
                "does not claim legal personhood, constitutional standing, or production moral agency".to_string(),
                "does not expose private wellbeing or private state as public reputation".to_string(),
                "does not prove external or cross-polis communication; secure local comms remain intra-polis only".to_string(),
            ],
            claim_boundary:
                "This flagship proves a bounded v0.91 cognitive-being evidence bundle for review and replay. It does not prove consciousness, legal personhood, production moral agency, or cross-polis transport."
                    .to_string(),
        })
    }

    pub(crate) fn validate_against(
        &self,
        sections: &[RuntimeV2CognitiveBeingFlagshipSection],
    ) -> Result<()> {
        if self.schema_version != RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 cognitive-being flagship proof schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D13" {
            return Err(anyhow!(
                "cognitive-being flagship proof must map to demo matrix row D13"
            ));
        }
        if self.milestone != "v0.91" {
            return Err(anyhow!(
                "cognitive-being flagship proof must target milestone v0.91"
            ));
        }
        normalize_id(self.proof_id.clone(), "cognitive_being_flagship.proof_id")?;
        validate_relative_path(
            &self.artifact_path,
            "cognitive_being_flagship.artifact_path",
        )?;
        validate_relative_path(
            &self.reviewer_report_ref,
            "cognitive_being_flagship.reviewer_report_ref",
        )?;
        validate_relative_path(&self.section_ref, "cognitive_being_flagship.section_ref")?;
        if self.proof_classification != "proving" {
            return Err(anyhow!(
                "cognitive-being flagship proof must stay classified as proving"
            ));
        }
        validate_nonempty_text(
            &self.proof_summary,
            "cognitive_being_flagship.proof_summary",
        )?;
        validate_nonempty_text(
            &self.claim_boundary,
            "cognitive_being_flagship.claim_boundary",
        )?;
        if !self.proof_summary.contains("cognitive-being flagship") {
            return Err(anyhow!(
                "cognitive-being flagship proof summary must mention the flagship surface"
            ));
        }
        if !self
            .reviewer_command
            .contains("cognitive-being-flagship-demo")
        {
            return Err(anyhow!(
                "cognitive-being flagship reviewer_command must invoke cognitive-being-flagship-demo"
            ));
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_cognitive_being_flagship_demo"))
        {
            return Err(anyhow!(
                "cognitive-being flagship must include the focused runtime_v2_cognitive_being_flagship_demo test command"
            ));
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("cognitive-being-flagship-demo"))
        {
            return Err(anyhow!(
                "cognitive-being flagship must include the runnable cognitive-being-flagship-demo command"
            ));
        }
        for path in &self.source_docs {
            validate_relative_path(path, "cognitive_being_flagship.source_docs")?;
        }
        let expected_ids = sections
            .iter()
            .map(|section| section.section_id.clone())
            .collect::<Vec<_>>();
        if self.section_ids != expected_ids {
            return Err(anyhow!(
                "cognitive-being flagship section_ids must match the canonical section roster"
            ));
        }
        let required_set = self
            .required_artifact_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        if required_set.len() != self.required_artifact_refs.len() {
            return Err(anyhow!(
                "cognitive-being flagship required_artifact_refs must not contain duplicates"
            ));
        }
        for artifact in &self.required_artifact_refs {
            validate_relative_path(artifact, "cognitive_being_flagship.required_artifact_refs")?;
        }
        for phrase in [
            "birthday",
            "legal personhood",
            "private wellbeing",
            "cross-polis",
        ] {
            if !self.non_claims.iter().any(|claim| claim.contains(phrase)) {
                return Err(anyhow!(
                    "cognitive-being flagship must preserve the {phrase} non-claim"
                ));
            }
        }
        if !self.claim_boundary.contains("production moral agency") {
            return Err(anyhow!(
                "cognitive-being flagship claim boundary must preserve the production moral agency boundary"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2CognitiveBeingFlagshipSection {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_SECTION_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 cognitive-being flagship section schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.section_id.clone(),
            "cognitive_being_flagship.section_id",
        )?;
        validate_nonempty_text(&self.title, "cognitive_being_flagship.title")?;
        validate_nonempty_text(&self.summary, "cognitive_being_flagship.summary")?;
        validate_nonempty_text(
            &self.claim_boundary,
            "cognitive_being_flagship.claim_boundary",
        )?;
        if self.primary_artifact_refs.is_empty() {
            return Err(anyhow!(
                "cognitive-being flagship section {} must include primary_artifact_refs",
                self.section_id
            ));
        }
        for artifact in &self.primary_artifact_refs {
            validate_relative_path(artifact, "cognitive_being_flagship.primary_artifact_refs")?;
        }
        for validation_ref in &self.validation_refs {
            validate_nonempty_text(validation_ref, "cognitive_being_flagship.validation_refs")?;
        }
        Ok(())
    }
}

fn flagship_sections() -> Vec<RuntimeV2CognitiveBeingFlagshipSection> {
    vec![
        section(
            "moral_trace_and_trajectory",
            "Moral Trace And Trajectory",
            "Shows the canonical trace, linkage, and trajectory review surfaces that anchor later cognitive-being review.",
            vec![
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_MORAL_TRACE_PATH,
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_OUTCOME_LINKAGE_PATH,
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_TRAJECTORY_PATH,
            ],
            vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_moral_trace_schema -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_moral_trajectory_review -- --nocapture",
            ],
            "Proves reviewable moral evidence structure, not final moral verdicts or production ethical omniscience.",
        ),
        section(
            "anti_harm_and_wellbeing",
            "Anti-Harm And Wellbeing",
            "Shows trajectory-aware harm refusal and decomposed wellbeing review without turning safety or flourishing into scalar scores.",
            vec![
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ANTI_HARM_PATH,
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_WELLBEING_PATH,
            ],
            vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_anti_harm_trajectory_constraint -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_wellbeing_diagnostic -- --nocapture",
            ],
            "Proves bounded safety and wellbeing diagnostics only; private wellbeing remains governed and non-public.",
        ),
        section(
            "kindness_affect_reframing",
            "Kindness, Affect, And Reframing",
            "Shows support, reframing, and affect-like control as reviewable governance surfaces rather than entertainment or hidden feeling claims.",
            vec![
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_KINDNESS_PATH,
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_HUMOR_PATH,
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_AFFECT_PATH,
            ],
            vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_kindness -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_humor -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_affect -- --nocapture",
            ],
            "Proves bounded kindness, reframing, and operational affect control without claiming subjective emotion or manipulative humor authority.",
        ),
        section(
            "moral_resources_and_cultivation",
            "Moral Resources And Cultivation",
            "Shows care, refusal, dignity-preserving resources, and cultivation posture as durable reviewable evidence.",
            vec![
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_MORAL_RESOURCES_PATH,
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_CULTIVATION_PATH,
            ],
            vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_moral_resource -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_cultivating_intelligence -- --nocapture",
            ],
            "Proves reviewable moral-resource and cultivation surfaces, not full intelligence theory, birthday semantics, or personhood completion.",
        ),
        section(
            "structured_planning_and_review",
            "Structured Planning And Review",
            "Shows durable issue-local planning and review-policy artifacts as part of the flagship cognitive-being workflow discipline.",
            vec![
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_SPP_PATH,
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_SRP_PATH,
            ],
            vec![
                "bash adl/tools/validate_structured_prompt.sh --type spp --phase run --input .adl/v0.91/tasks/issue-2751__v0-91-wp-17-demo-cognitive-being-flagship-demo/spp.md",
                "bash adl/tools/validate_structured_prompt.sh --type srp --phase run --input .adl/v0.91/tasks/issue-2751__v0-91-wp-17-demo-cognitive-being-flagship-demo/srp.md",
            ],
            "Proves durable planning and review policy surfaces, not that planning or review become automatic or infallible.",
        ),
        section(
            "secure_local_comms",
            "Secure Local Comms",
            "Shows authenticated local invocation, review-policy linkage, and A2A boundary fixtures without claiming external transport safety.",
            vec![
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_PROOF_PATH,
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_INVOCATION_PATH,
                RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_A2A_PATH,
            ],
            vec![
                "cargo test --manifest-path adl/Cargo.toml agent_comms -- --nocapture",
            ],
            "Proves secure local comms evidence only; it does not prove external or cross-polis transport and fails closed at that boundary.",
        ),
    ]
}

fn section(
    section_id: &str,
    title: &str,
    summary: &str,
    primary_artifact_refs: Vec<&str>,
    validation_refs: Vec<&str>,
    claim_boundary: &str,
) -> RuntimeV2CognitiveBeingFlagshipSection {
    RuntimeV2CognitiveBeingFlagshipSection {
        schema_version: RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_SECTION_SCHEMA.to_string(),
        section_id: section_id.to_string(),
        title: title.to_string(),
        summary: summary.to_string(),
        primary_artifact_refs: primary_artifact_refs
            .into_iter()
            .map(str::to_string)
            .collect(),
        validation_refs: validation_refs.into_iter().map(str::to_string).collect(),
        claim_boundary: claim_boundary.to_string(),
    }
}

fn validate_expected_sections(sections: &[RuntimeV2CognitiveBeingFlagshipSection]) -> Result<()> {
    let observed = sections
        .iter()
        .map(|section| section.section_id.as_str())
        .collect::<Vec<_>>();
    if observed != EXPECTED_SECTION_IDS {
        return Err(anyhow!(
            "cognitive-being flagship sections must match the canonical D13 section roster"
        ));
    }
    Ok(())
}

fn render_cognitive_being_flagship_reviewer_report(
    packet: &RuntimeV2CognitiveBeingFlagshipProofPacket,
    sections: &[RuntimeV2CognitiveBeingFlagshipSection],
) -> Result<String> {
    packet.validate_against(sections)?;
    let mut lines = vec![
        "# D13 Cognitive-Being Flagship Demo".to_string(),
        "".to_string(),
        "## Summary".to_string(),
        "".to_string(),
        packet.proof_summary.clone(),
        "".to_string(),
        "## Replay".to_string(),
        "".to_string(),
        format!("- command: `{}`", packet.reviewer_command),
        format!("- proof packet: `{}`", packet.artifact_path),
        format!("- section roster: `{}`", packet.section_ref),
        "".to_string(),
        "## Sections".to_string(),
        "".to_string(),
    ];
    for section in sections {
        lines.push(format!("### {}", section.title));
        lines.push(String::new());
        lines.push(section.summary.clone());
        lines.push(String::new());
        lines.push(format!("- id: `{}`", section.section_id));
        lines.push(format!(
            "- artifacts: `{}`",
            section.primary_artifact_refs.join("`, `")
        ));
        lines.push(format!(
            "- validation: `{}`",
            section.validation_refs.join("`, `")
        ));
        lines.push(format!("- claim boundary: {}", section.claim_boundary));
        lines.push(String::new());
    }
    lines.push("## Global Non-Claims".to_string());
    lines.push(String::new());
    for non_claim in &packet.non_claims {
        lines.push(format!("- {}", non_claim));
    }
    lines.push(String::new());
    lines.push("## Claim Boundary".to_string());
    lines.push(String::new());
    lines.push(packet.claim_boundary.clone());
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn validate_cognitive_being_flagship_report(
    packet: &RuntimeV2CognitiveBeingFlagshipProofPacket,
    sections: &[RuntimeV2CognitiveBeingFlagshipSection],
    report: &str,
) -> Result<()> {
    packet.validate_against(sections)?;
    validate_nonempty_text(report, "cognitive_being_flagship.report")?;
    for required in [
        "# D13 Cognitive-Being Flagship Demo\n",
        "\n## Replay\n",
        "\n## Sections\n",
        "\n## Global Non-Claims\n",
        "\n## Claim Boundary\n",
    ] {
        if !report.contains(required) {
            return Err(anyhow!(
                "cognitive-being flagship report must contain '{required}'"
            ));
        }
    }
    for section in sections {
        if !report.contains(&section.title) || !report.contains(&section.section_id) {
            return Err(anyhow!(
                "cognitive-being flagship report must preserve section {}",
                section.section_id
            ));
        }
    }
    Ok(())
}

fn synthetic_spp_markdown() -> String {
    r#"# Structured Plan Prompt

task_id: "issue-2751"
run_id: "issue-2751"
goal: "Assemble the v0.91 D13 cognitive-being flagship bundle from the already-landed moral governance, wellbeing, planning/review, and secure local comms artifacts."
assumptions:
- "All referenced v0.91 packet constructors remain fixture-backed and deterministic."
- "The flagship demo should replay as a bounded artifact bundle, not as live cross-polis execution."
touched_surfaces:
- "adl/src/runtime_v2/cognitive_being_flagship_demo.rs"
- "adl/src/cli/runtime_v2_cmd.rs"
proof_expectations:
- "Produce a reviewable D13 proof packet, section roster, and reviewer report."
- "Preserve bounded non-claims around birthday, personhood, production moral agency, and cross-polis comms."
codex_plan:
- step: "Compose the canonical section roster from landed v0.91 proof packets."
  status: "completed"
- step: "Emit reviewer-facing flagship artifacts and replay instructions."
  status: "completed"
- step: "Validate narrow runtime-v2 tests and CLI output path rules."
  status: "completed"
"#
    .to_string()
}

fn synthetic_srp_markdown() -> String {
    r#"# Structured Review Prompt

task_id: "issue-2751"
review_mode: "pre_pr_independent_review"
scope_basis:
- "Bounded to the D13 cognitive-being flagship runtime-v2 bundle and its demo doc."
evidence_classes:
- "runtime-v2 proof packet"
- "section roster"
- "reviewer report"
- "focused CLI and contract tests"
dispositions:
- "pass"
- "findings"
- "blocked"
refusal_policy:
- "Refuse to widen into claims about legal personhood, birthday completion, production moral agency, or cross-polis comms."
- "Refuse to treat synthetic planning/review artifacts as live operator authority."
constraints:
- "Review only repository-relative artifacts."
- "Require concrete evidence for any claimed drift in non-claims or replayability."
"#
    .to_string()
}

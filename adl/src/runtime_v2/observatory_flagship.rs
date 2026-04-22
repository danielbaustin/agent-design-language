use super::*;
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_SCHEMA: &str =
    "runtime_v2.observatory_flagship_proof_packet.v1";
pub const RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_SCHEMA: &str =
    "runtime_v2.observatory_flagship_walkthrough_step.v1";

pub const RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_PATH: &str =
    "runtime_v2/observatory/flagship_proof_packet.json";
pub const RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH: &str =
    "runtime_v2/observatory/flagship_operator_report.md";
pub const RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_PATH: &str =
    "runtime_v2/observatory/flagship_walkthrough.jsonl";

const EXPECTED_ROOMS: [&str; 4] = [
    "World / Reality",
    "Operator / Governance",
    "Cognition / Internal State",
    "Corporate Investor",
];

const REQUIRED_REF_FRAGMENTS: [&str; 6] = [
    "continuity_witnesses.json",
    "citizen_receipts.json",
    "private_state_projection_packet.json",
    "access_events.json",
    "challenge_artifact.json",
    "flagship_operator_report.md",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ObservatoryFlagshipActor {
    pub actor_id: String,
    pub standing_class: String,
    pub visible_role: String,
    pub evidence_refs: Vec<String>,
    pub prohibited_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ObservatoryFlagshipWalkthroughStep {
    pub schema_version: String,
    pub sequence: u32,
    pub room: String,
    pub lens_or_memory_dot: String,
    pub visible_surface: String,
    pub artifact_ref: String,
    pub continuity_question_answered: String,
    pub proof_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ObservatoryFlagshipProofPacket {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub milestone: String,
    pub artifact_path: String,
    pub operator_report_ref: String,
    pub walkthrough_ref: String,
    pub source_docs: Vec<String>,
    pub actor_roster: Vec<RuntimeV2ObservatoryFlagshipActor>,
    pub required_artifact_refs: Vec<String>,
    pub continuity_refs: Vec<String>,
    pub observatory_refs: Vec<String>,
    pub standing_access_refs: Vec<String>,
    pub challenge_refs: Vec<String>,
    pub operator_report_refs: Vec<String>,
    pub lens_sequence: Vec<RuntimeV2ObservatoryFlagshipWalkthroughStep>,
    pub reviewer_command: String,
    pub validation_commands: Vec<String>,
    pub proof_summary: String,
    pub proof_classification: String,
    pub non_claims: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2ObservatoryFlagshipArtifacts {
    pub challenge_artifacts: RuntimeV2ContinuityChallengeArtifacts,
    pub operator_control_report: RuntimeV2OperatorControlReport,
    pub proof_packet: RuntimeV2ObservatoryFlagshipProofPacket,
    pub operator_report_markdown: String,
}

impl RuntimeV2ObservatoryFlagshipArtifacts {
    pub fn prototype() -> Result<Self> {
        let challenge_artifacts = runtime_v2_continuity_challenge_contract()?;
        let operator_control_report = runtime_v2_operator_control_report_contract()?;
        let lens_sequence =
            observatory_flagship_walkthrough(&challenge_artifacts, &operator_control_report)?;
        let proof_packet = RuntimeV2ObservatoryFlagshipProofPacket::from_artifacts(
            &challenge_artifacts,
            &operator_control_report,
            lens_sequence,
        )?;
        let operator_report_markdown =
            render_observatory_flagship_operator_report(&proof_packet, &challenge_artifacts)?;
        let artifacts = Self {
            challenge_artifacts,
            operator_control_report,
            proof_packet,
            operator_report_markdown,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.challenge_artifacts.validate()?;
        self.operator_control_report.validate()?;
        self.proof_packet
            .validate_against(&self.challenge_artifacts, &self.operator_control_report)?;
        validate_flagship_operator_report(&self.proof_packet, &self.operator_report_markdown)
    }

    pub fn proof_packet_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.proof_packet)
            .context("serialize Runtime v2 Observatory flagship proof packet")
    }

    pub fn walkthrough_jsonl_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut out = Vec::new();
        for step in &self.proof_packet.lens_sequence {
            serde_json::to_writer(&mut out, step)
                .context("serialize Runtime v2 Observatory flagship walkthrough step")?;
            out.push(b'\n');
        }
        Ok(out)
    }

    pub fn execution_summary(&self) -> Result<String> {
        let mut lines = vec![
            "D12 inhabited CSM Observatory flagship proof:".to_string(),
            format!("- proof packet: {}", self.proof_packet.artifact_path),
            format!(
                "- operator report: {}",
                self.proof_packet.operator_report_ref
            ),
            format!("- walkthrough: {}", self.proof_packet.walkthrough_ref),
        ];
        for step in &self.proof_packet.lens_sequence {
            lines.push(format!(
                "- {:02} {} :: {} -> {}",
                step.sequence, step.room, step.lens_or_memory_dot, step.artifact_ref
            ));
        }
        Ok(lines.join("\n"))
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();

        let access = &self.challenge_artifacts.access_control_artifacts;
        let observatory = &access.observatory_artifacts;
        let observatory_sanctuary = &observatory.sanctuary_artifacts;
        let challenge_sanctuary = &self.challenge_artifacts.sanctuary_artifacts;

        access.standing_artifacts.write_to_root(root)?;
        observatory.private_state_artifacts.write_to_root(root)?;
        observatory
            .witness_artifacts
            .lineage_artifacts
            .write_to_root(root)?;
        observatory.witness_artifacts.write_to_root(root)?;
        observatory_sanctuary
            .anti_equivocation_artifacts
            .witness_artifacts
            .lineage_artifacts
            .write_to_root(root)?;
        observatory_sanctuary
            .anti_equivocation_artifacts
            .witness_artifacts
            .write_to_root(root)?;
        observatory_sanctuary
            .anti_equivocation_artifacts
            .write_to_root(root)?;
        observatory_sanctuary.write_to_root(root)?;
        observatory.write_to_root(root)?;
        access.write_to_root(root)?;

        challenge_sanctuary
            .anti_equivocation_artifacts
            .witness_artifacts
            .lineage_artifacts
            .write_to_root(root)?;
        challenge_sanctuary
            .anti_equivocation_artifacts
            .witness_artifacts
            .write_to_root(root)?;
        challenge_sanctuary
            .anti_equivocation_artifacts
            .write_to_root(root)?;
        challenge_sanctuary.write_to_root(root)?;
        self.challenge_artifacts.write_to_root(root)?;
        self.operator_control_report.write_to_root(root)?;

        let mut walkthrough = Vec::new();
        for step in &self.proof_packet.lens_sequence {
            serde_json::to_writer(&mut walkthrough, step)
                .context("serialize Runtime v2 Observatory flagship walkthrough step")?;
            walkthrough.push(b'\n');
        }
        write_relative(
            root,
            RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_PATH,
            walkthrough,
        )?;
        write_relative(
            root,
            RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH,
            self.operator_report_markdown.as_bytes().to_vec(),
        )?;
        write_relative(
            root,
            RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_PATH,
            serde_json::to_vec_pretty(&self.proof_packet)
                .context("serialize Runtime v2 Observatory flagship proof packet")?,
        )
    }
}

impl RuntimeV2ObservatoryFlagshipProofPacket {
    pub fn from_artifacts(
        challenge: &RuntimeV2ContinuityChallengeArtifacts,
        operator: &RuntimeV2OperatorControlReport,
        lens_sequence: Vec<RuntimeV2ObservatoryFlagshipWalkthroughStep>,
    ) -> Result<Self> {
        challenge.validate()?;
        operator.validate()?;
        validate_flagship_walkthrough(&lens_sequence)?;

        let access = &challenge.access_control_artifacts;
        let observatory = &access.observatory_artifacts;
        let standing = &access.standing_artifacts;
        let witness = &observatory.witness_artifacts;
        let sanctuary = &challenge.sanctuary_artifacts;
        let continuity_refs = vec![
            witness.witness_set.artifact_path.clone(),
            witness.receipt_set.artifact_path.clone(),
            witness.lineage_artifacts.ledger.artifact_path.clone(),
            witness
                .lineage_artifacts
                .materialized_head
                .artifact_path
                .clone(),
            sanctuary.quarantine_artifact.artifact_path.clone(),
        ];
        let observatory_refs = vec![
            observatory.redaction_policy.artifact_path.clone(),
            observatory.projection_packet.artifact_path.clone(),
            RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_REPORT_PATH.to_string(),
            RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PROOF_PATH.to_string(),
        ];
        let standing_access_refs = vec![
            standing.policy.artifact_path.clone(),
            standing.event_packet.artifact_path.clone(),
            standing.communication_examples.artifact_path.clone(),
            RUNTIME_V2_STANDING_NEGATIVE_CASES_PATH.to_string(),
            access.authority_matrix.artifact_path.clone(),
            access.event_packet.artifact_path.clone(),
            RUNTIME_V2_ACCESS_DENIAL_FIXTURES_PATH.to_string(),
        ];
        let challenge_refs = vec![
            challenge.challenge.artifact_path.clone(),
            challenge.freeze.artifact_path.clone(),
            challenge.appeal_review.artifact_path.clone(),
            challenge.threat_model.artifact_path.clone(),
            challenge.economics_placement.artifact_path.clone(),
            sanctuary.operator_report.artifact_path.clone(),
        ];
        let operator_report_refs = vec![
            operator.artifact_path.clone(),
            RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH.to_string(),
        ];
        let required_artifact_refs = required_artifact_refs(
            &continuity_refs,
            &observatory_refs,
            &standing_access_refs,
            &challenge_refs,
            &operator_report_refs,
        );

        let packet = Self {
            schema_version: RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_SCHEMA.to_string(),
            proof_id: "d12-inhabited-csm-observatory-flagship-proof-0001".to_string(),
            demo_id: "D12".to_string(),
            milestone: "v0.90.3".to_string(),
            artifact_path: RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_PATH.to_string(),
            operator_report_ref: RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH.to_string(),
            walkthrough_ref: RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_PATH.to_string(),
            source_docs: vec![
                "docs/milestones/v0.90.3/OBSERVATORY_FLAGSHIP_DEMO_v0.90.3.md".to_string(),
                "docs/milestones/v0.90.3/OBSERVATORY_UI_ARCHITECTURE_v0.90.3.md".to_string(),
                "docs/milestones/v0.90.3/DEMO_MATRIX_v0.90.3.md".to_string(),
                "docs/milestones/v0.90.3/REDACTED_OBSERVATORY_PROJECTIONS_v0.90.3.md"
                    .to_string(),
            ],
            actor_roster: vec![
                RuntimeV2ObservatoryFlagshipActor {
                    actor_id: "proto-citizen-alpha".to_string(),
                    standing_class: "citizen".to_string(),
                    visible_role: "identity-bearing citizen with private state, continuity witness, and citizen receipt".to_string(),
                    evidence_refs: vec![
                        witness.receipt_set.artifact_path.clone(),
                        observatory.projection_packet.artifact_path.clone(),
                    ],
                    prohibited_claims: vec![
                        "personhood is not proven".to_string(),
                        "raw private state is not disclosed".to_string(),
                    ],
                },
                RuntimeV2ObservatoryFlagshipActor {
                    actor_id: "guest-operator-candidate".to_string(),
                    standing_class: "guest".to_string(),
                    visible_role: "bounded outside participant denied citizen-only continuity authority"
                        .to_string(),
                    evidence_refs: vec![
                        standing.event_packet.artifact_path.clone(),
                        access.denial_fixtures.event_packet_ref.clone(),
                    ],
                    prohibited_claims: vec![
                        "guest standing cannot silently become citizen standing".to_string(),
                    ],
                },
                RuntimeV2ObservatoryFlagshipActor {
                    actor_id: "operator.cli".to_string(),
                    standing_class: "operator".to_string(),
                    visible_role: "reviewer-visible operator using redacted Observatory surfaces"
                        .to_string(),
                    evidence_refs: vec![
                        operator.artifact_path.clone(),
                        RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH.to_string(),
                    ],
                    prohibited_claims: vec![
                        "operator view is not private-state authority".to_string(),
                    ],
                },
                RuntimeV2ObservatoryFlagshipActor {
                    actor_id: "flagship-service-witness".to_string(),
                    standing_class: "service".to_string(),
                    visible_role: "service actor that emits continuity evidence without acquiring citizen rights"
                        .to_string(),
                    evidence_refs: vec![
                        witness.witness_set.artifact_path.clone(),
                        challenge.challenge.artifact_path.clone(),
                    ],
                    prohibited_claims: vec![
                        "service witness cannot own or fork citizen state".to_string(),
                    ],
                },
            ],
            required_artifact_refs,
            continuity_refs,
            observatory_refs,
            standing_access_refs,
            challenge_refs,
            operator_report_refs,
            lens_sequence,
            reviewer_command:
                "adl runtime-v2 observatory-flagship-demo --out artifacts/v0903/demo-d12-observatory-flagship"
                    .to_string(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_observatory_flagship -- --nocapture"
                    .to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_observatory_flagship_demo -- --nocapture"
                    .to_string(),
                "adl runtime-v2 observatory-flagship-demo --out artifacts/v0903/demo-d12-observatory-flagship"
                    .to_string(),
                "git diff --check".to_string(),
            ],
            proof_summary:
                "D12 integrates WP-03 through WP-13 into one bounded inhabited CSM Observatory proof: citizen private-state continuity, witness and receipt evidence, redacted projection, standing and communication boundary, access-control denial, continuity challenge, sanctuary quarantine, appeal review, operator report, and room/lens walkthrough."
                    .to_string(),
            proof_classification: "proving".to_string(),
            non_claims: vec![
                "does not prove personhood".to_string(),
                "does not claim first true Godel-agent birthday".to_string(),
                "does not expose canonical private citizen state".to_string(),
                "does not implement unbounded live CSM execution".to_string(),
                "does not implement v0.91 civic markets or v0.92 migration semantics".to_string(),
            ],
            claim_boundary:
                "This packet proves the bounded local D12 citizen-state Observatory evidence package and that the artifact is reviewable as a unique continuation scenario; it does not prove personhood, a first true Godel-agent birthday, raw private-state inspection, or unbounded live Runtime v2 execution."
                    .to_string(),
        };
        packet.validate_against(challenge, operator)?;
        Ok(packet)
    }

    pub fn validate_against(
        &self,
        challenge: &RuntimeV2ContinuityChallengeArtifacts,
        operator: &RuntimeV2OperatorControlReport,
    ) -> Result<()> {
        self.validate_shape()?;
        if !self.continuity_refs.iter().any(|artifact| {
            artifact
                == &challenge
                    .access_control_artifacts
                    .observatory_artifacts
                    .witness_artifacts
                    .witness_set
                    .artifact_path
        }) {
            return Err(anyhow!(
                "observatory flagship proof missing continuity witness ref"
            ));
        }
        if !self.continuity_refs.iter().any(|artifact| {
            artifact
                == &challenge
                    .access_control_artifacts
                    .observatory_artifacts
                    .witness_artifacts
                    .receipt_set
                    .artifact_path
        }) {
            return Err(anyhow!(
                "observatory flagship proof missing citizen receipt ref"
            ));
        }
        if !self.observatory_refs.iter().any(|artifact| {
            artifact
                == &challenge
                    .access_control_artifacts
                    .observatory_artifacts
                    .projection_packet
                    .artifact_path
        }) {
            return Err(anyhow!(
                "observatory flagship proof missing redacted projection packet ref"
            ));
        }
        if !self.standing_access_refs.iter().any(|artifact| {
            artifact
                == &challenge
                    .access_control_artifacts
                    .event_packet
                    .artifact_path
        }) {
            return Err(anyhow!(
                "observatory flagship proof missing access event ref"
            ));
        }
        if !self
            .challenge_refs
            .iter()
            .any(|artifact| artifact == &challenge.challenge.artifact_path)
        {
            return Err(anyhow!(
                "observatory flagship proof missing continuity challenge ref"
            ));
        }
        if !self
            .operator_report_refs
            .iter()
            .any(|artifact| artifact == &operator.artifact_path)
        {
            return Err(anyhow!(
                "observatory flagship proof missing operator control report ref"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 Observatory flagship proof schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D12" {
            return Err(anyhow!(
                "observatory flagship proof must map to demo matrix row D12"
            ));
        }
        if self.milestone != "v0.90.3" {
            return Err(anyhow!("observatory flagship proof must target v0.90.3"));
        }
        normalize_id(self.proof_id.clone(), "observatory_flagship.proof_id")?;
        validate_relative_path(&self.artifact_path, "observatory_flagship.artifact_path")?;
        validate_relative_path(
            &self.operator_report_ref,
            "observatory_flagship.operator_report_ref",
        )?;
        validate_relative_path(
            &self.walkthrough_ref,
            "observatory_flagship.walkthrough_ref",
        )?;
        validate_relative_path_list(&self.source_docs, "observatory_flagship.source_docs")?;
        validate_relative_path_list(
            &self.required_artifact_refs,
            "observatory_flagship.required_artifact_refs",
        )?;
        validate_relative_path_list(
            &self.continuity_refs,
            "observatory_flagship.continuity_refs",
        )?;
        validate_relative_path_list(
            &self.observatory_refs,
            "observatory_flagship.observatory_refs",
        )?;
        validate_relative_path_list(
            &self.standing_access_refs,
            "observatory_flagship.standing_access_refs",
        )?;
        validate_relative_path_list(&self.challenge_refs, "observatory_flagship.challenge_refs")?;
        validate_relative_path_list(
            &self.operator_report_refs,
            "observatory_flagship.operator_report_refs",
        )?;
        if self.proof_classification != "proving" {
            return Err(anyhow!(
                "observatory flagship proof must be classified as proving"
            ));
        }
        validate_actor_roster(&self.actor_roster)?;
        validate_flagship_walkthrough(&self.lens_sequence)?;
        validate_required_flagship_refs(&self.required_artifact_refs)?;
        validate_nonempty_text(
            &self.reviewer_command,
            "observatory_flagship.reviewer_command",
        )?;
        if !self.reviewer_command.contains("observatory-flagship-demo") {
            return Err(anyhow!(
                "observatory flagship proof must include its runnable demo command"
            ));
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_observatory_flagship"))
        {
            return Err(anyhow!(
                "observatory flagship proof must include the focused validation command"
            ));
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("observatory-flagship-demo"))
        {
            return Err(anyhow!(
                "observatory flagship proof must include the runnable demo validation command"
            ));
        }
        for command in &self.validation_commands {
            validate_nonempty_text(command, "observatory_flagship.validation_commands")?;
        }
        validate_nonempty_text(&self.proof_summary, "observatory_flagship.proof_summary")?;
        for required_phrase in [
            "WP-03",
            "WP-13",
            "witness",
            "receipt",
            "redacted projection",
        ] {
            if !self.proof_summary.contains(required_phrase) {
                return Err(anyhow!(
                    "observatory flagship proof summary must mention {required_phrase}"
                ));
            }
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("personhood"))
        {
            return Err(anyhow!(
                "observatory flagship proof must preserve the personhood non-claim"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("first true Godel-agent birthday"))
        {
            return Err(anyhow!(
                "observatory flagship proof must preserve the first-birthday non-claim"
            ));
        }
        if !self
            .claim_boundary
            .contains("bounded local D12 citizen-state Observatory evidence package")
        {
            return Err(anyhow!(
                "observatory flagship proof must preserve its bounded D12 claim boundary"
            ));
        }
        Ok(())
    }
}

fn observatory_flagship_walkthrough(
    challenge: &RuntimeV2ContinuityChallengeArtifacts,
    operator: &RuntimeV2OperatorControlReport,
) -> Result<Vec<RuntimeV2ObservatoryFlagshipWalkthroughStep>> {
    challenge.validate()?;
    operator.validate()?;
    let access = &challenge.access_control_artifacts;
    let observatory = &access.observatory_artifacts;
    let witness = &observatory.witness_artifacts;
    Ok(vec![
        walkthrough_step(
            1,
            "World / Reality",
            "inhabited-roster",
            "citizen, guest, service, and operator roles are visible without raw private state",
            access
                .standing_artifacts
                .event_packet
                .artifact_path
                .as_str(),
            "who is present, and what standing do they have?",
            "standing is evidence, not identity transfer",
        ),
        walkthrough_step(
            2,
            "World / Reality",
            "continuity-witness",
            "single-lineage citizen state has witness and receipt evidence",
            witness.witness_set.artifact_path.as_str(),
            "why is proto-citizen-alpha still the same citizen?",
            "witnesses prove bounded transition continuity, not personhood",
        ),
        walkthrough_step(
            3,
            "Operator / Governance",
            "redacted-projection",
            "operator sees continuity state through a redacted Observatory packet",
            observatory.projection_packet.artifact_path.as_str(),
            "what may the operator inspect without violating private state?",
            "projection is not raw private-state authority",
        ),
        walkthrough_step(
            4,
            "Operator / Governance",
            "access-denial",
            "guest request for citizen-only authority is denied and evented",
            access.event_packet.artifact_path.as_str(),
            "can a guest silently acquire citizen rights?",
            "denial evidence proves refusal, not a general access-control system",
        ),
        walkthrough_step(
            5,
            "Operator / Governance",
            "sanctuary-quarantine",
            "ambiguous continuity is frozen into sanctuary/quarantine review",
            challenge
                .sanctuary_artifacts
                .quarantine_artifact
                .artifact_path
                .as_str(),
            "what happens when continuity is ambiguous?",
            "quarantine preserves evidence and blocks destructive transition",
        ),
        walkthrough_step(
            6,
            "Cognition / Internal State",
            "challenge-and-appeal",
            "citizen challenge, freeze, and appeal remain reviewable without disclosure",
            challenge.challenge.artifact_path.as_str(),
            "how can a citizen contest continuity basis?",
            "challenge proof is procedural continuity evidence, not mind inspection",
        ),
        walkthrough_step(
            7,
            "Corporate Investor",
            "fallback-report",
            "non-specialist reviewer sees scope, risks, and non-claims in one report",
            RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH,
            "what does this demo prove and not prove?",
            "fallback report is a review surface, not runtime authority",
        ),
    ])
}

fn walkthrough_step(
    sequence: u32,
    room: &str,
    lens_or_memory_dot: &str,
    visible_surface: &str,
    artifact_ref: &str,
    continuity_question_answered: &str,
    proof_boundary: &str,
) -> RuntimeV2ObservatoryFlagshipWalkthroughStep {
    RuntimeV2ObservatoryFlagshipWalkthroughStep {
        schema_version: RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_SCHEMA.to_string(),
        sequence,
        room: room.to_string(),
        lens_or_memory_dot: lens_or_memory_dot.to_string(),
        visible_surface: visible_surface.to_string(),
        artifact_ref: artifact_ref.to_string(),
        continuity_question_answered: continuity_question_answered.to_string(),
        proof_boundary: proof_boundary.to_string(),
    }
}

fn required_artifact_refs(
    continuity_refs: &[String],
    observatory_refs: &[String],
    standing_access_refs: &[String],
    challenge_refs: &[String],
    operator_report_refs: &[String],
) -> Vec<String> {
    continuity_refs
        .iter()
        .chain(observatory_refs)
        .chain(standing_access_refs)
        .chain(challenge_refs)
        .chain(operator_report_refs)
        .cloned()
        .collect()
}

fn render_observatory_flagship_operator_report(
    proof: &RuntimeV2ObservatoryFlagshipProofPacket,
    challenge: &RuntimeV2ContinuityChallengeArtifacts,
) -> Result<String> {
    proof.validate_shape()?;
    challenge.validate()?;
    Ok(format!(
        concat!(
            "# D12 Inhabited CSM Observatory Flagship\n\n",
            "Proof classification: `{}`\n\n",
            "Primary proof packet: `{}`\n\n",
            "Reviewer command: `{}`\n\n",
            "Citizen continuity basis:\n",
            "- witness set: `{}`\n",
            "- citizen receipt set: `{}`\n",
            "- redacted projection: `{}`\n",
            "- continuity challenge: `{}`\n",
            "- sanctuary/quarantine: `{}`\n\n",
            "Operator-facing result: the Observatory can explain why the citizen-state scenario is reviewable, which authority paths are refused, and which ambiguous continuity transition is frozen without exposing canonical private state.\n\n",
            "Non-claims: personhood, first true Godel-agent birthday, raw private-state inspection, and unbounded live Runtime v2 execution remain outside this proof.\n"
        ),
        proof.proof_classification,
        proof.artifact_path,
        proof.reviewer_command,
        challenge.access_control_artifacts.observatory_artifacts.witness_artifacts.witness_set.artifact_path,
        challenge.access_control_artifacts.observatory_artifacts.witness_artifacts.receipt_set.artifact_path,
        challenge.access_control_artifacts.observatory_artifacts.projection_packet.artifact_path,
        challenge.challenge.artifact_path,
        challenge.sanctuary_artifacts.quarantine_artifact.artifact_path,
    ))
}

fn validate_flagship_operator_report(
    proof: &RuntimeV2ObservatoryFlagshipProofPacket,
    report: &str,
) -> Result<()> {
    proof.validate_shape()?;
    validate_nonempty_text(report, "observatory_flagship.operator_report")?;
    for required in [
        "D12 Inhabited CSM Observatory Flagship",
        proof.artifact_path.as_str(),
        proof.reviewer_command.as_str(),
        "witness set",
        "citizen receipt set",
        "redacted projection",
        "continuity challenge",
        "sanctuary/quarantine",
        "Non-claims",
    ] {
        if !report.contains(required) {
            return Err(anyhow!(
                "observatory flagship operator report missing required text '{required}'"
            ));
        }
    }
    for forbidden in [
        "private_payload_b64",
        "sealed_payload_b64",
        "section_digests",
    ] {
        if report.contains(forbidden) {
            return Err(anyhow!(
                "observatory flagship operator report leaked forbidden private-state token"
            ));
        }
    }
    Ok(())
}

fn validate_actor_roster(actors: &[RuntimeV2ObservatoryFlagshipActor]) -> Result<()> {
    if actors.len() != 4 {
        return Err(anyhow!(
            "observatory flagship proof must include citizen, guest, service, and operator actors"
        ));
    }
    let mut seen_standing = BTreeSet::new();
    for actor in actors {
        normalize_id(actor.actor_id.clone(), "observatory_flagship.actor_id")?;
        normalize_id(
            actor.standing_class.clone(),
            "observatory_flagship.standing_class",
        )?;
        validate_nonempty_text(&actor.visible_role, "observatory_flagship.visible_role")?;
        validate_relative_path_list(&actor.evidence_refs, "observatory_flagship.evidence_refs")?;
        validate_required_texts(
            &actor.prohibited_claims,
            "observatory_flagship.prohibited_claims",
        )?;
        seen_standing.insert(actor.standing_class.as_str());
    }
    for required in ["citizen", "guest", "service", "operator"] {
        if !seen_standing.contains(required) {
            return Err(anyhow!(
                "observatory flagship actor roster missing {required} standing"
            ));
        }
    }
    Ok(())
}

fn validate_flagship_walkthrough(
    steps: &[RuntimeV2ObservatoryFlagshipWalkthroughStep],
) -> Result<()> {
    if steps.len() != 7 {
        return Err(anyhow!(
            "observatory flagship walkthrough must include seven room/lens steps"
        ));
    }
    let mut seen_rooms = BTreeSet::new();
    for (index, step) in steps.iter().enumerate() {
        if step.schema_version != RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 Observatory flagship walkthrough schema '{}'",
                step.schema_version
            ));
        }
        if step.sequence != (index + 1) as u32 {
            return Err(anyhow!(
                "observatory flagship walkthrough sequence must be contiguous"
            ));
        }
        validate_nonempty_text(&step.room, "observatory_flagship.room")?;
        validate_nonempty_text(
            &step.lens_or_memory_dot,
            "observatory_flagship.lens_or_memory_dot",
        )?;
        validate_nonempty_text(
            &step.visible_surface,
            "observatory_flagship.visible_surface",
        )?;
        validate_relative_path(&step.artifact_ref, "observatory_flagship.artifact_ref")?;
        validate_nonempty_text(
            &step.continuity_question_answered,
            "observatory_flagship.continuity_question_answered",
        )?;
        validate_nonempty_text(&step.proof_boundary, "observatory_flagship.proof_boundary")?;
        seen_rooms.insert(step.room.as_str());
    }
    for room in EXPECTED_ROOMS {
        if !seen_rooms.contains(room) {
            return Err(anyhow!(
                "observatory flagship walkthrough missing expected room '{room}'"
            ));
        }
    }
    Ok(())
}

fn validate_required_flagship_refs(refs: &[String]) -> Result<()> {
    for fragment in REQUIRED_REF_FRAGMENTS {
        if !refs.iter().any(|artifact| artifact.contains(fragment)) {
            return Err(anyhow!(
                "observatory flagship proof missing required artifact fragment '{fragment}'"
            ));
        }
    }
    Ok(())
}

fn validate_relative_path_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = BTreeSet::new();
    for value in values {
        validate_relative_path(value, field)?;
        if !seen.insert(value.as_str()) {
            return Err(anyhow!("{field} contains duplicate path '{value}'"));
        }
    }
    Ok(())
}

fn validate_required_texts(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
    }
    Ok(())
}

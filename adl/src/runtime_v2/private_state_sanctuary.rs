//! Runtime-v2 private-state sanctuary and protection envelope.
//!
//! Defines sanctuary-boundary rules and records for sensitive continuity and
//! continuity-state safety checks.

use super::*;
use std::path::Path;

pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_STATE_POLICY_SCHEMA: &str =
    "runtime_v2.private_state_sanctuary_quarantine_policy.v1";
pub const RUNTIME_V2_PRIVATE_STATE_AMBIGUOUS_WAKE_FIXTURE_SCHEMA: &str =
    "runtime_v2.private_state_ambiguous_wake_fixture.v1";
pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_SCHEMA: &str =
    "runtime_v2.private_state_sanctuary_quarantine_artifact.v1";
pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_OPERATOR_REPORT_SCHEMA: &str =
    "runtime_v2.private_state_sanctuary_operator_report.v1";
pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_PROOF_SCHEMA: &str =
    "runtime_v2.private_state_sanctuary_quarantine_proof.v1";

pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_STATE_POLICY_PATH: &str =
    "runtime_v2/private_state/sanctuary_quarantine_state_policy.json";
pub const RUNTIME_V2_PRIVATE_STATE_AMBIGUOUS_WAKE_FIXTURE_PATH: &str =
    "runtime_v2/private_state/ambiguous_wake_fixture.json";
pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_PATH: &str =
    "runtime_v2/private_state/sanctuary_quarantine_artifact.json";
pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_OPERATOR_REPORT_PATH: &str =
    "runtime_v2/private_state/sanctuary_quarantine_operator_report.json";
pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_PROOF_PATH: &str =
    "runtime_v2/private_state/sanctuary_quarantine_negative_cases.json";

mod helpers;
mod policy;
mod reports;

use helpers::*;
pub use policy::*;
pub use reports::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSanctuaryArtifacts {
    pub anti_equivocation_artifacts: RuntimeV2PrivateStateAntiEquivocationArtifacts,
    pub state_policy: RuntimeV2PrivateStateSanctuaryStatePolicy,
    pub ambiguous_wake: RuntimeV2PrivateStateAmbiguousWakeFixture,
    pub quarantine_artifact: RuntimeV2PrivateStateSanctuaryQuarantineArtifact,
    pub operator_report: RuntimeV2PrivateStateSanctuaryOperatorReport,
    pub negative_cases: RuntimeV2PrivateStateSanctuaryProof,
}

impl RuntimeV2PrivateStateSanctuaryArtifacts {
    pub fn prototype() -> Result<Self> {
        let anti_equivocation_artifacts = runtime_v2_private_state_anti_equivocation_contract()?;
        let state_policy = RuntimeV2PrivateStateSanctuaryStatePolicy::from_disposition(
            &anti_equivocation_artifacts.conflict,
            &anti_equivocation_artifacts.disposition,
        )?;
        let ambiguous_wake = RuntimeV2PrivateStateAmbiguousWakeFixture::from_conflict(
            &anti_equivocation_artifacts.conflict,
            &anti_equivocation_artifacts.disposition,
        )?;
        let quarantine_artifact = RuntimeV2PrivateStateSanctuaryQuarantineArtifact::from_fixture(
            &ambiguous_wake,
            &anti_equivocation_artifacts.conflict,
            &anti_equivocation_artifacts.disposition,
        )?;
        let operator_report =
            RuntimeV2PrivateStateSanctuaryOperatorReport::from_quarantine(&quarantine_artifact)?;
        let negative_cases = RuntimeV2PrivateStateSanctuaryProof::prototype();
        let artifacts = Self {
            anti_equivocation_artifacts,
            state_policy,
            ambiguous_wake,
            quarantine_artifact,
            operator_report,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.anti_equivocation_artifacts.validate()?;
        self.state_policy.validate_against(
            &self.anti_equivocation_artifacts.conflict,
            &self.anti_equivocation_artifacts.disposition,
        )?;
        self.ambiguous_wake.validate_against(
            &self.anti_equivocation_artifacts.conflict,
            &self.anti_equivocation_artifacts.disposition,
        )?;
        self.quarantine_artifact.validate_against(
            &self.ambiguous_wake,
            &self.anti_equivocation_artifacts.conflict,
            &self.anti_equivocation_artifacts.disposition,
        )?;
        self.operator_report
            .validate_against(&self.quarantine_artifact)?;
        self.negative_cases.validate_against(
            &self.state_policy,
            &self.ambiguous_wake,
            &self.quarantine_artifact,
            &self.operator_report,
        )
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_SANCTUARY_STATE_POLICY_PATH,
            self.state_policy.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_AMBIGUOUS_WAKE_FIXTURE_PATH,
            self.ambiguous_wake.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_PATH,
            self.quarantine_artifact.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_SANCTUARY_OPERATOR_REPORT_PATH,
            self.operator_report.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_SANCTUARY_PROOF_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

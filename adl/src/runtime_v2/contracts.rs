//! Runtime-v2 shared contract types and validation-facing structures.
//!
//! The public surface in this module is used as the typed backbone for
//! manifold, kernel, snapshot, and citizen artifact contracts.

use super::*;
use once_cell::sync::OnceCell;

fn cached_contract<T, F>(cell: &OnceCell<T>, build: F) -> Result<T>
where
    T: Clone,
    F: FnOnce() -> Result<T>,
{
    cell.get_or_try_init(build).cloned()
}

pub fn runtime_v2_contract_schema_contract() -> Result<RuntimeV2ContractSchemaArtifacts> {
    RuntimeV2ContractSchemaArtifacts::prototype()
}

pub fn runtime_v2_manifold_contract() -> Result<RuntimeV2ManifoldRoot> {
    RuntimeV2ManifoldRoot::prototype("proto-csm-01")
}

pub fn runtime_v2_kernel_loop_contract() -> Result<RuntimeV2KernelLoopArtifacts> {
    RuntimeV2KernelLoopArtifacts::prototype(&runtime_v2_manifold_contract()?)
}

pub fn runtime_v2_citizen_lifecycle_contract() -> Result<RuntimeV2CitizenLifecycleArtifacts> {
    RuntimeV2CitizenLifecycleArtifacts::prototype(&runtime_v2_manifold_contract()?)
}

pub fn runtime_v2_snapshot_rehydration_contract() -> Result<RuntimeV2SnapshotAndRehydrationArtifacts>
{
    let manifold = runtime_v2_manifold_contract()?;
    let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold)?;
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
    RuntimeV2SnapshotAndRehydrationArtifacts::prototype(&manifold, &kernel, &citizens)
}

pub fn runtime_v2_invariant_violation_contract() -> Result<RuntimeV2InvariantViolationArtifact> {
    let manifold = runtime_v2_manifold_contract()?;
    let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold)?;
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
    RuntimeV2InvariantViolationArtifact::duplicate_active_citizen_prototype(
        &manifold, &kernel, &citizens,
    )
}

pub fn runtime_v2_invariant_and_violation_contract(
) -> Result<RuntimeV2InvariantAndViolationContractArtifacts> {
    RuntimeV2InvariantAndViolationContractArtifacts::prototype()
}

pub fn runtime_v2_operator_control_report_contract() -> Result<RuntimeV2OperatorControlReport> {
    let manifold = runtime_v2_manifold_contract()?;
    let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold)?;
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
    let snapshot =
        RuntimeV2SnapshotAndRehydrationArtifacts::prototype(&manifold, &kernel, &citizens)?;
    let violation = RuntimeV2InvariantViolationArtifact::duplicate_active_citizen_prototype(
        &manifold, &kernel, &citizens,
    )?;
    RuntimeV2OperatorControlReport::prototype(&manifold, &kernel, &citizens, &snapshot, &violation)
}

pub fn runtime_v2_security_boundary_proof_contract() -> Result<RuntimeV2SecurityBoundaryProofPacket>
{
    let manifold = runtime_v2_manifold_contract()?;
    let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold)?;
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
    let snapshot =
        RuntimeV2SnapshotAndRehydrationArtifacts::prototype(&manifold, &kernel, &citizens)?;
    let violation = RuntimeV2InvariantViolationArtifact::duplicate_active_citizen_prototype(
        &manifold, &kernel, &citizens,
    )?;
    let operator_report = RuntimeV2OperatorControlReport::prototype(
        &manifold, &kernel, &citizens, &snapshot, &violation,
    )?;
    RuntimeV2SecurityBoundaryProofPacket::refused_resume_without_invariant_prototype(
        &manifold,
        &kernel,
        &violation,
        &operator_report,
    )
}

pub fn runtime_v2_csm_run_packet_contract() -> Result<RuntimeV2CsmRunPacketContract> {
    let manifold = runtime_v2_manifold_contract()?;
    let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold)?;
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
    RuntimeV2CsmRunPacketContract::prototype(&manifold, &kernel, &citizens)
}

pub fn runtime_v2_csm_boot_admission_contract() -> Result<RuntimeV2CsmBootAdmissionArtifacts> {
    RuntimeV2CsmBootAdmissionArtifacts::prototype()
}

pub fn runtime_v2_csm_governed_episode_contract() -> Result<RuntimeV2CsmGovernedEpisodeArtifacts> {
    RuntimeV2CsmGovernedEpisodeArtifacts::prototype()
}

pub fn runtime_v2_csm_freedom_gate_mediation_contract(
) -> Result<RuntimeV2CsmFreedomGateMediationArtifacts> {
    RuntimeV2CsmFreedomGateMediationArtifacts::prototype()
}

pub fn runtime_v2_csm_invalid_action_rejection_contract(
) -> Result<RuntimeV2CsmInvalidActionRejectionArtifacts> {
    RuntimeV2CsmInvalidActionRejectionArtifacts::prototype()
}

pub fn runtime_v2_csm_wake_continuity_contract() -> Result<RuntimeV2CsmWakeContinuityArtifacts> {
    RuntimeV2CsmWakeContinuityArtifacts::prototype()
}

pub fn runtime_v2_csm_observatory_contract() -> Result<RuntimeV2CsmObservatoryArtifacts> {
    RuntimeV2CsmObservatoryArtifacts::prototype()
}

pub fn runtime_v2_csm_recovery_eligibility_contract(
) -> Result<RuntimeV2CsmRecoveryEligibilityArtifacts> {
    RuntimeV2CsmRecoveryEligibilityArtifacts::prototype()
}

pub fn runtime_v2_csm_quarantine_contract() -> Result<RuntimeV2CsmQuarantineArtifacts> {
    RuntimeV2CsmQuarantineArtifacts::prototype()
}

pub fn runtime_v2_csm_hardening_contract() -> Result<RuntimeV2CsmHardeningArtifacts> {
    RuntimeV2CsmHardeningArtifacts::prototype()
}

pub fn runtime_v2_csm_integrated_run_contract() -> Result<RuntimeV2CsmIntegratedRunArtifacts> {
    RuntimeV2CsmIntegratedRunArtifacts::prototype()
}

pub fn runtime_v2_feature_proof_coverage_contract() -> Result<RuntimeV2FeatureProofCoveragePacket> {
    static PACKET: OnceCell<RuntimeV2FeatureProofCoveragePacket> = OnceCell::new();
    cached_contract(&PACKET, RuntimeV2FeatureProofCoveragePacket::prototype)
}

pub fn runtime_v2_foundation_demo_contract() -> Result<RuntimeV2FoundationPrototypeArtifacts> {
    RuntimeV2FoundationPrototypeArtifacts::prototype()
}

pub fn runtime_v2_private_state_contract() -> Result<RuntimeV2PrivateStateArtifacts> {
    RuntimeV2PrivateStateArtifacts::prototype()
}

pub fn runtime_v2_private_state_envelope_contract() -> Result<RuntimeV2PrivateStateEnvelopeArtifacts>
{
    RuntimeV2PrivateStateEnvelopeArtifacts::prototype()
}

pub fn runtime_v2_private_state_sealing_contract() -> Result<RuntimeV2PrivateStateSealingArtifacts>
{
    RuntimeV2PrivateStateSealingArtifacts::prototype()
}

pub fn runtime_v2_private_state_lineage_contract() -> Result<RuntimeV2PrivateStateLineageArtifacts>
{
    RuntimeV2PrivateStateLineageArtifacts::prototype()
}

pub fn runtime_v2_private_state_witness_contract() -> Result<RuntimeV2PrivateStateWitnessArtifacts>
{
    RuntimeV2PrivateStateWitnessArtifacts::prototype()
}

pub fn runtime_v2_private_state_anti_equivocation_contract(
) -> Result<RuntimeV2PrivateStateAntiEquivocationArtifacts> {
    RuntimeV2PrivateStateAntiEquivocationArtifacts::prototype()
}

pub fn runtime_v2_private_state_sanctuary_contract(
) -> Result<RuntimeV2PrivateStateSanctuaryArtifacts> {
    RuntimeV2PrivateStateSanctuaryArtifacts::prototype()
}

pub fn runtime_v2_private_state_observatory_contract(
) -> Result<RuntimeV2PrivateStateObservatoryArtifacts> {
    static ARTIFACTS: OnceCell<RuntimeV2PrivateStateObservatoryArtifacts> = OnceCell::new();
    cached_contract(&ARTIFACTS, RuntimeV2PrivateStateObservatoryArtifacts::prototype)
}

pub fn runtime_v2_standing_contract() -> Result<RuntimeV2StandingArtifacts> {
    RuntimeV2StandingArtifacts::prototype()
}

pub fn runtime_v2_access_control_contract() -> Result<RuntimeV2AccessControlArtifacts> {
    static ARTIFACTS: OnceCell<RuntimeV2AccessControlArtifacts> = OnceCell::new();
    cached_contract(&ARTIFACTS, RuntimeV2AccessControlArtifacts::prototype)
}

pub fn runtime_v2_continuity_challenge_contract() -> Result<RuntimeV2ContinuityChallengeArtifacts> {
    static ARTIFACTS: OnceCell<RuntimeV2ContinuityChallengeArtifacts> = OnceCell::new();
    cached_contract(&ARTIFACTS, RuntimeV2ContinuityChallengeArtifacts::prototype)
}

pub fn runtime_v2_observatory_flagship_contract() -> Result<RuntimeV2ObservatoryFlagshipArtifacts> {
    static ARTIFACTS: OnceCell<RuntimeV2ObservatoryFlagshipArtifacts> = OnceCell::new();
    cached_contract(&ARTIFACTS, RuntimeV2ObservatoryFlagshipArtifacts::prototype)
}

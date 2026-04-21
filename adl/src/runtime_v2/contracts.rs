use super::*;
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

pub fn runtime_v2_foundation_demo_contract() -> Result<RuntimeV2FoundationPrototypeArtifacts> {
    RuntimeV2FoundationPrototypeArtifacts::prototype()
}

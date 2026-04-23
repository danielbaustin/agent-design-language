//! Execution state model for deterministic workflow execution and resume control.

mod contracts;
mod policy;
mod runtime_control;
mod steering;

pub(crate) use contracts::DEFAULT_MAX_CONCURRENCY;
pub use contracts::{
    materialize_inputs, ExecutionBoundary, ExecutionResult, RuntimeLifecyclePhase,
    SchedulerPolicySource, StepExecutionRecord, StepOutput, MATERIALIZE_INPUT_MAX_FILE_BYTES,
};
pub use policy::{stable_failure_kind, ExecutionPolicyError, ExecutionPolicyErrorKind};
pub use runtime_control::{
    derive_runtime_control_state, select_instinct_runtime_candidate, AgencyCandidateRecord,
    AgencySelectionDecisionTemplate, AgencySelectionState, BoundedExecutionIteration,
    BoundedExecutionState, CognitiveArbitrationState, CognitiveSignalsState, DominantInstinct,
    EvaluationControlState, FastSlowPathState, FreedomGateConsequenceContextState,
    FreedomGateEvaluationSignalsState, FreedomGateInputState, FreedomGatePolicyContextState,
    FreedomGateState, MemoryParticipationState, MemoryQueryState, MemoryReadEntry, MemoryReadState,
    MemoryWriteState, ReframingControlState, Route, RuntimeControlState, SelectedPath,
};
pub use steering::{
    apply_steering_patch, steering_record_from_patch, validate_steering_patch, PauseState,
    ResumeState, SteeringPatch, SteeringRecord, STEERING_APPLY_AT_RESUME_BOUNDARY,
    STEERING_PATCH_SCHEMA_VERSION,
};

//! Gödel experiment runtime surface.
//!
//! This subsystem coordinates failure analysis, hypothesis generation, mutation,
//! evaluation, and promotion artifacts for the ADL runtime feedback loop.
pub mod affect_slice;
pub mod canonical_evidence;
pub mod cross_workflow;
pub mod evaluation;
pub mod experiment_record;
pub mod hypothesis;
pub mod mutation;
pub mod obsmem_index;
pub mod policy;
pub mod prioritization;
pub mod promotion;
pub mod stage_loop;
pub mod surface_status;
pub mod workflow_template;

/// Primary execution stages and runner state types for the Gödel pipeline.
pub use stage_loop::{
    GodelStage, GodelStageLoopExecutor, StageLoopConfig, StageLoopError, StageLoopInput,
    StageLoopPersistenceResult, StageLoopRun,
};
/// Runtime status contract emitted by the Gödel status check.
pub use surface_status::{
    load_v08_surface_status, repo_root_from_manifest, GodelRuntimeSurfaceStatus,
    GODEL_RUNTIME_STATUS_VERSION,
};
/// Workflow template parser/validator for Gödel experiment orchestration.
pub use workflow_template::{
    embedded_v08_workflow_template, parse_workflow_template, GodelWorkflowTemplate,
};

pub const IDENTITY_PROFILE_SCHEMA: &str = "identity_profile.v1";
pub const TEMPORAL_CONTEXT_SCHEMA: &str = "temporal_context.v1";
pub const CHRONOSENSE_FOUNDATION_SCHEMA: &str = "chronosense_foundation.v1";
pub const TEMPORAL_SCHEMA_V01: &str = "temporal_schema.v0_1";
pub const CONTINUITY_SEMANTICS_SCHEMA: &str = "continuity_semantics.v1";
pub const TEMPORAL_QUERY_RETRIEVAL_SCHEMA: &str = "temporal_query_retrieval.v1";
pub const COMMITMENT_DEADLINE_SCHEMA: &str = "commitment_deadline_semantics.v1";
pub const TEMPORAL_CAUSALITY_EXPLANATION_SCHEMA: &str = "temporal_causality_explanation.v1";
pub const EXECUTION_POLICY_COST_MODEL_SCHEMA: &str = "execution_policy_cost_model.v1";
pub const PHI_INTEGRATION_METRICS_SCHEMA: &str = "phi_integration_metrics.v1";
pub const INSTINCT_MODEL_SCHEMA: &str = "instinct_model.v1";
pub const INSTINCT_RUNTIME_SURFACE_SCHEMA: &str = "instinct_runtime_surface.v1";

mod causality;
mod commitments;
mod continuity;
mod foundation;
mod instinct;
mod phi;
mod policy_cost;
mod retrieval;
mod temporal_schema;

pub use causality::{
    CausalRelationContract, ExplanationFixture, ExplanationSurfaceContract,
    TemporalCausalityExplanationContract,
};
pub use commitments::{
    CommitmentDeadlineContract, CommitmentLifecycleContract, CommitmentRecordRequirements,
    DeadlineSemanticsContract, MissedCommitmentDetectionContract,
};
pub use continuity::{ContinuitySemanticsContract, ContinuityStateContract, ResumptionRule};
pub use foundation::{
    default_identity_profile_path, load_identity_profile, write_identity_profile,
    ChronosenseFoundation, IdentityProfile, TemporalContext,
};
pub use instinct::{
    InstinctEntryContract, InstinctModelContract, InstinctRepresentationContract,
    InstinctReviewSurfaceContract, InstinctRuntimeProofCase, InstinctRuntimeReviewSurfaceContract,
    InstinctRuntimeSurfaceContract, InstinctSemanticsContract,
};
pub use phi::{
    PhiComparisonFixture, PhiComparisonProfile, PhiIntegrationMetricsContract, PhiMetricDimension,
    PhiReviewSurfaceContract,
};
pub use policy_cost::{
    CostAnchorContract, CostPolicyContract, CostReviewSurfaceContract,
    ExecutionPolicyCostModelContract,
};
pub use retrieval::{
    TemporalQueryPrimitiveSet, TemporalQueryRetrievalContract, TemporalRetrievalSemantics,
};
pub use temporal_schema::{
    CostVectorSchema, ExecutionPolicySchema, ExecutionRealizationSchema, SubjectiveTimeSchema,
    TemporalAnchorSchema, TemporalReferenceFramesSchema, TemporalSchemaContract,
};

#[cfg(test)]
mod tests;

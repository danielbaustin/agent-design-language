mod fixtures;
mod types;
mod validation;

pub use fixtures::{
    acc_v1_authority_fixtures, acc_v1_redaction_examples, acc_v1_visibility_matrix,
};
pub use types::{
    AccActorIdentityV1, AccActorKindV1, AccAuthorityEvidenceKindV1, AccAuthorityEvidenceV1,
    AccAuthorityFixtureV1, AccAuthorityGrantV1, AccCapabilityRequirementV1,
    AccConfirmationRequirementV1, AccDecisionV1, AccDelegationConstraintsV1_1, AccDelegationStepV1,
    AccExecutionSemanticsV1, AccExpectedFixtureOutcomeV1, AccFailurePolicyV1,
    AccFreedomGateDecisionV1, AccFreedomGateRequirementV1, AccGrantStatusV1, AccPolicyCheckV1,
    AccPrivacyRedactionV1, AccRedactionExampleV1, AccRedactionSurfaceV1, AccRoleStandingV1,
    AccToolReferenceV1, AccTracePrivacyPolicyV1, AccTraceReplayV1, AccValidationError,
    AccValidationReport, AccVisibilityAudienceV1, AccVisibilityLevelV1, AccVisibilityMatrixEntryV1,
    AccVisibilityPolicyV1, AdlCapabilityContractV1, AdlCapabilityContractV1_1,
    ACC_MAX_DELEGATION_DEPTH_V1, ACC_SCHEMA_VERSION_V1, ACC_SCHEMA_VERSION_V1_0,
    ACC_SCHEMA_VERSION_V1_1,
};
pub use validation::{
    acc_v1_1_schema_json, acc_v1_schema_json, upgrade_acc_v1_to_v1_1, validate_acc_v1,
    validate_acc_v1_1,
};

#[cfg(test)]
pub(crate) use fixtures::base_contract;
#[cfg(test)]
pub(crate) use validation::{
    redaction_examples_are_safe, redaction_examples_cover_required_surfaces,
    visibility_is_complete, visibility_matrix_covers_required_audiences,
    visibility_matrix_fails_closed,
};

#[cfg(test)]
mod tests;

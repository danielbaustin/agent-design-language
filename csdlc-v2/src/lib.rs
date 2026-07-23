#![recursion_limit = "256"]

pub mod cards;
pub mod cutover;
pub mod doctor;
pub mod eligibility;
pub mod error;
pub mod git;
pub mod github;
pub mod github_token;
pub mod lifecycle;
pub mod migration;
pub mod model;
pub mod operator;
pub mod proof;
pub mod publication;
pub mod pvf;
pub mod readiness;
pub mod registry;
pub mod review;
pub mod schema;
pub mod soak;
pub mod store;

pub use cards::{
    CardKind, CardStatus, CardValues, InitialCardInput, PlanningCollectionField, PlanningProfile,
    SemanticOperation,
};
pub use cutover::{run_cutover, CutoverEvidence, CutoverRequest};
pub use doctor::{diagnose, DoctorReport};
pub use eligibility::{
    eligibility_schema_bundle, evaluate_deletion_eligibility, DeletionApproval, DeletionDecision,
    DeletionEligibilityRequest, DeletionEntry, DeletionManifest, DeletionReason, EntryDisposition,
};
pub use error::{ErrorCode, Result, V2Error};
pub use git::shared_request_path;
pub use lifecycle::{
    amend_claim_scope, bind_issue, heartbeat_claim, initialize_native_json, recover_claim,
    release_closed_claim, transition_active_claim, AmendClaimScopeRequest, BindRequest, BindResult,
    HeartbeatRequest, RecoverClaimRequest, ReleaseClosedClaimRequest, TransitionActiveClaimRequest,
};
pub use migration::{
    compare_shadow, generate_compatibility_view, import_legacy, write_compatibility_view_atomic,
    ImportReport, LegacyImportRequest, NormalizedOutcome, ShadowComparison,
};
pub use model::{
    Claim, ClaimRecovery, DesignReview, IssueRecord, LifecyclePhase, MigrationEvidence,
    NonSubstantiveProof, PublicationEvidence, ReadinessEvidence, ReconcileTerminalRequest,
    ReviewAssignment, ReviewEvidence, ReviewFindingEvidence, TerminalDesignRepairRequest,
    TerminalEvidence, TerminalPlanStepRepairRequest, TerminalReceipt,
    TerminalSorArtifactRepairRequest, TerminalSorValidationRepairRequest,
};
pub use operator::{
    build_and_install_binaries, install_binaries, resolve_operator_generation, verify_coexistence,
    CoexistenceInventory, InstallReceipt, SkillManifest,
};
pub use proof::{run_pre_switch_proof, PreSwitchEvidence, ProofManifest, ProofStep};
pub use publication::{
    prepare_publication, prepare_ready_publication, prepare_ready_reconciliation, reconcile_action,
    record_merged_publication, record_publication, record_ready_publication,
    record_ready_reconciliation, validate_ready_reconciliation_state, validate_ready_remote,
    MergedPublicationReconciliationRequest, PublicationAction, PublicationIntent,
    PublicationRequest, ReadyPublicationReconciliationRequest, ReadyPublicationRequest,
    RemotePullRequest,
};
pub use pvf::{
    classify_schedule, classify_shepherd, execute, finalize, select, ExecutionRequest,
    FinalizeRequest, PvfManifest, ScheduleInput, ShepherdInput,
};
pub use readiness::{
    classify_readiness, closeout_issue, record_readiness, CheckConclusion, CheckObservation,
    CheckRequirement, ConflictState, PostPublicationFinding, ReadinessReport, ReadinessRequest,
    RemoteReviewState, TerminalDisposition, TerminalObservation,
};
pub use review::{
    assign_review, evaluate_publication_review, evaluate_publication_review_in_repo, record_review,
    recover_review, PublicationReviewReport, ReviewAssignmentRequest, ReviewRecordRequest,
    ReviewRecoveryRequest,
};
pub use schema::public_schema_bundle;
pub use soak::{
    decide_cutover, decide_from_evidence, generate_sample_packets, select_generation,
    BudgetEvidence, BudgetKind, CutoverDecision, Generation, GenerationSelector, ParityEvidence,
    SamplePacket, ScenarioEvidence, ScenarioOutcome, SoakDecisionPacket, SoakEvidenceInput,
    SoakScenario,
};
pub use store::{
    approve_design, edit_issue, ApproveDesignRequest, BootstrapRequest, EditRequest,
    RepairIdentityRequest, Store,
};

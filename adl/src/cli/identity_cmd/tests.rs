use super::dispatch::real_identity_in_repo;
use super::helpers::{repo_root, required_value, resolve_identity_path, run_git_capture};
use ::adl::chronosense::{
    default_identity_profile_path, load_identity_profile, TEMPORAL_CONTEXT_SCHEMA,
};
use once_cell::sync::Lazy;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

fn system_git_bin() -> &'static str {
    if Path::new("/usr/bin/git").exists() {
        "/usr/bin/git"
    } else {
        "git"
    }
}

fn temp_repo(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let repo = env::temp_dir().join(format!("adl-{name}-{unique}"));
    fs::create_dir_all(&repo).expect("create repo dir");
    Command::new(system_git_bin())
        .arg("init")
        .current_dir(&repo)
        .status()
        .expect("git init")
        .success()
        .then_some(())
        .expect("git init should succeed");
    repo
}

#[test]
fn identity_init_writes_default_profile_and_show_reads_it() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-init-show");

    real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--created-by".to_string(),
            "daniel".to_string(),
        ],
        &repo,
    )
    .expect("identity init");

    let profile_path = repo.join("adl/identity/identity_profile.v1.json");
    assert!(profile_path.is_file(), "profile should exist");

    let profile = load_identity_profile(&profile_path).expect("profile load");
    assert_eq!(profile.agent_id, "codex");
    assert_eq!(profile.birth_weekday_local, "Monday");

    real_identity_in_repo(&["show".to_string()], &repo).expect("identity show");
}

#[test]
fn identity_now_requires_timezone_without_profile() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-now");

    let err = real_identity_in_repo(&["now".to_string()], &repo)
        .expect_err("should fail without timezone");
    assert!(err
        .to_string()
        .contains("identity now requires --timezone <IANA> when no profile exists"));
}

#[test]
fn identity_now_writes_temporal_context_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-now-out");

    real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
        ],
        &repo,
    )
    .expect("identity init");

    let out_path = repo.join(".adl/state/temporal_context.v1.json");
    real_identity_in_repo(
        &[
            "now".to_string(),
            "--out".to_string(),
            out_path.display().to_string(),
        ],
        &repo,
    )
    .expect("identity now");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], TEMPORAL_CONTEXT_SCHEMA);
    assert_eq!(json["identity_agent_id"], "codex");
}

#[test]
fn identity_foundation_writes_bounded_foundation_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-foundation");
    let out_path = repo.join(".adl/state/chronosense_foundation.v1.json");

    real_identity_in_repo(
        &[
            "foundation".to_string(),
            "--out".to_string(),
            ".adl/state/chronosense_foundation.v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity foundation");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "chronosense_foundation.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/chronosense_foundation.v1.json"
    );
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity foundation"));
}

#[test]
fn identity_requires_subcommand_and_rejects_unknown_subcommand() {
    let repo = temp_repo("identity-subcommands");

    let err = real_identity_in_repo(&[], &repo).expect_err("missing subcommand should fail");
    assert!(err
        .to_string()
        .contains("identity requires a subcommand: init | show | now | foundation | adversarial-runtime | schema"));
    assert!(err.to_string().contains("continuity"));

    let err = real_identity_in_repo(&["nope".to_string()], &repo)
        .expect_err("unknown subcommand should fail");
    assert!(err
        .to_string()
        .contains("unknown identity subcommand 'nope'"));
}

#[test]
fn identity_top_level_help_and_subcommand_help_succeed() {
    let repo = temp_repo("identity-help");

    real_identity_in_repo(&["help".to_string()], &repo).expect("top-level help");
    real_identity_in_repo(&["init".to_string(), "--help".to_string()], &repo).expect("init help");
    real_identity_in_repo(&["now".to_string(), "--help".to_string()], &repo).expect("now help");
    real_identity_in_repo(&["foundation".to_string(), "--help".to_string()], &repo)
        .expect("foundation help");
    real_identity_in_repo(
        &["adversarial-runtime".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("adversarial-runtime help");
    real_identity_in_repo(&["schema".to_string(), "--help".to_string()], &repo)
        .expect("schema help");
    real_identity_in_repo(&["continuity".to_string(), "--help".to_string()], &repo)
        .expect("continuity help");
}

#[test]
fn identity_init_validates_required_and_unknown_args() {
    let repo = temp_repo("identity-init-errors");

    let err = real_identity_in_repo(
        &[
            "init".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
        ],
        &repo,
    )
    .expect_err("missing name should fail");
    assert!(err
        .to_string()
        .contains("identity init requires --name <display-name>"));

    let err = real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
        ],
        &repo,
    )
    .expect_err("missing birthday should fail");
    assert!(err
        .to_string()
        .contains("identity init requires --birthday <RFC3339>"));

    let err = real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
        ],
        &repo,
    )
    .expect_err("missing timezone should fail");
    assert!(err
        .to_string()
        .contains("identity init requires --timezone <IANA>"));

    let err = real_identity_in_repo(&["init".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity init: --bogus"));
}

#[test]
fn identity_init_supports_custom_path_agent_id_and_force() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-init-custom-path");
    let profile_path = repo.join(".adl/state/custom_identity_profile.v1.json");

    real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--agent-id".to_string(),
            "codex-local".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--path".to_string(),
            profile_path.display().to_string(),
        ],
        &repo,
    )
    .expect("custom path init");

    let profile = load_identity_profile(&profile_path).expect("profile load");
    assert_eq!(profile.agent_id, "codex-local");

    let err = real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--path".to_string(),
            profile_path.display().to_string(),
        ],
        &repo,
    )
    .expect_err("existing profile without force should fail");
    assert!(err.to_string().contains("identity profile already exists"));

    real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--path".to_string(),
            profile_path.display().to_string(),
            "--force".to_string(),
        ],
        &repo,
    )
    .expect("force overwrite");
}

#[test]
fn identity_show_supports_custom_path_and_rejects_unknown_args() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-show-path");
    let profile_path = repo.join("identity/custom_profile.v1.json");

    real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--path".to_string(),
            profile_path.display().to_string(),
        ],
        &repo,
    )
    .expect("seed profile");

    real_identity_in_repo(
        &[
            "show".to_string(),
            "--path".to_string(),
            profile_path.display().to_string(),
        ],
        &repo,
    )
    .expect("show custom path");

    let err = real_identity_in_repo(&["show".to_string(), "--bogus".to_string()], &repo)
        .expect_err("show unknown arg");
    assert!(err
        .to_string()
        .contains("unknown arg for identity show: --bogus"));
}

#[test]
fn identity_now_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-now-errors");

    let err = real_identity_in_repo(
        &[
            "now".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--bogus".to_string(),
        ],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity now: --bogus"));

    let err = real_identity_in_repo(
        &[
            "now".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--out".to_string(),
        ],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_foundation_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-foundation-errors");

    let err = real_identity_in_repo(&["foundation".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity foundation: --bogus"));

    let err = real_identity_in_repo(&["foundation".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_adversarial_runtime_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-adversarial-runtime");
    let out_path = repo.join(".adl/state/adversarial_runtime_model_v1.json");

    real_identity_in_repo(
        &[
            "adversarial-runtime".to_string(),
            "--out".to_string(),
            ".adl/state/adversarial_runtime_model_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity adversarial-runtime");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "adversarial_runtime_model.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/adversarial_runtime_model_v1.json"
    );
    assert!(json["adversarial_pressure"]["operating_assumptions"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value
            == "systems are probed continuously rather than only during scheduled review windows"));
    assert!(json["review_surface"]["downstream_boundaries"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value.as_str().expect("string").contains("WP-04")));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity adversarial-runtime"));
}

#[test]
fn identity_adversarial_runtime_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-adversarial-runtime-errors");

    let err = real_identity_in_repo(
        &["adversarial-runtime".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity adversarial-runtime: --bogus"));

    let err = real_identity_in_repo(
        &["adversarial-runtime".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_schema_writes_temporal_schema_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-schema");
    let out_path = repo.join(".adl/state/temporal_schema_v01.json");

    real_identity_in_repo(
        &[
            "schema".to_string(),
            "--out".to_string(),
            ".adl/state/temporal_schema_v01.json".to_string(),
        ],
        &repo,
    )
    .expect("identity schema");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "temporal_schema.v0_1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/temporal_schema_v01.json"
    );
    assert_eq!(
        json["primary_temporal_anchor"]["monotonic_order"],
        "required strictly increasing order token"
    );
    assert!(json["reference_frames"]["internal_reasoning"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "monotonic"));
    assert!(json["execution_policy_trace_hooks"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "run_state.v1.duration_ms"));
}

#[test]
fn identity_schema_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-schema-errors");

    let err = real_identity_in_repo(&["schema".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity schema: --bogus"));

    let err = real_identity_in_repo(&["schema".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_continuity_writes_continuity_semantics_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-continuity");
    let out_path = repo.join(".adl/state/continuity_semantics_v1.json");

    real_identity_in_repo(
        &[
            "continuity".to_string(),
            "--out".to_string(),
            ".adl/state/continuity_semantics_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity continuity");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "continuity_semantics.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/continuity_semantics_v1.json"
    );
    assert!(json["continuity_state_contract"]["continuity_status"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "resume_ready"));
    assert!(json["resumption_rules"]
        .as_array()
        .expect("array")
        .iter()
        .any(|rule| {
            rule["continuity_status"] == "continuity_refused"
                && rule["resume_permitted"] == Value::Bool(false)
        }));
}

#[test]
fn identity_continuity_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-continuity-errors");

    let err = real_identity_in_repo(&["continuity".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity continuity: --bogus"));

    let err = real_identity_in_repo(&["continuity".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_retrieval_writes_temporal_query_retrieval_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-retrieval");
    let out_path = repo.join(".adl/state/temporal_query_retrieval_v1.json");

    real_identity_in_repo(
        &[
            "retrieval".to_string(),
            "--out".to_string(),
            ".adl/state/temporal_query_retrieval_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity retrieval");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "temporal_query_retrieval.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/temporal_query_retrieval_v1.json"
    );
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity retrieval"));
}

#[test]
fn identity_retrieval_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-retrieval-errors");

    let err = real_identity_in_repo(&["retrieval".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity retrieval: --bogus"));

    let err = real_identity_in_repo(&["retrieval".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_commitments_writes_commitment_deadline_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-commitments");
    let out_path = repo.join(".adl/state/commitment_deadline_semantics_v1.json");

    real_identity_in_repo(
        &[
            "commitments".to_string(),
            "--out".to_string(),
            ".adl/state/commitment_deadline_semantics_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity commitments");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "commitment_deadline_semantics.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/commitment_deadline_semantics_v1.json"
    );
    assert!(json["lifecycle"]["states"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "missed"));
    assert!(json["deadline_semantics"]["supported_frames"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "continuity_relative"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity commitments"));
}

#[test]
fn identity_commitments_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-commitments-errors");

    let err = real_identity_in_repo(&["commitments".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity commitments: --bogus"));

    let err = real_identity_in_repo(&["commitments".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_causality_writes_temporal_causality_explanation_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-causality");
    let out_path = repo.join(".adl/state/temporal_causality_explanation_v1.json");

    real_identity_in_repo(
        &[
            "causality".to_string(),
            "--out".to_string(),
            ".adl/state/temporal_causality_explanation_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity causality");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "temporal_causality_explanation.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/temporal_causality_explanation_v1.json"
    );
    assert_eq!(
        json["causal_relations"]["sequence_boundary_rule"],
        "sequence alone is insufficient evidence for causality"
    );
    assert!(json["causal_relations"]["relation_types"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "unknown_relation"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity causality"));
}

#[test]
fn identity_causality_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-causality-errors");

    let err = real_identity_in_repo(&["causality".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity causality: --bogus"));

    let err = real_identity_in_repo(&["causality".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_cost_writes_execution_policy_cost_model_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-cost");
    let out_path = repo.join(".adl/state/execution_policy_cost_model_v1.json");

    real_identity_in_repo(
        &[
            "cost".to_string(),
            "--out".to_string(),
            ".adl/state/execution_policy_cost_model_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity cost");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "execution_policy_cost_model.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/execution_policy_cost_model_v1.json"
    );
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity cost"));
}

#[test]
fn identity_cost_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-cost-errors");

    let err = real_identity_in_repo(&["cost".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity cost: --bogus"));

    let err = real_identity_in_repo(&["cost".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_phi_writes_phi_integration_metrics_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-phi");
    let out_path = repo.join(".adl/state/phi_integration_metrics_v1.json");

    real_identity_in_repo(
        &[
            "phi".to_string(),
            "--out".to_string(),
            ".adl/state/phi_integration_metrics_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity phi");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "phi_integration_metrics.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/phi_integration_metrics_v1.json"
    );
    assert_eq!(
        json["comparison_profiles"].as_array().expect("array").len(),
        3
    );
    assert!(json["review_surface"]["non_goals"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "formal IIT phi calculation"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity phi"));
}

#[test]
fn identity_phi_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-phi-errors");

    let err = real_identity_in_repo(&["phi".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity phi: --bogus"));

    let err = real_identity_in_repo(&["phi".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_instinct_writes_instinct_model_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-instinct");
    let out_path = repo.join(".adl/state/instinct_model_v1.json");

    real_identity_in_repo(
        &[
            "instinct".to_string(),
            "--out".to_string(),
            ".adl/state/instinct_model_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity instinct");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "instinct_model.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/instinct_model_v1.json"
    );
    assert_eq!(json["instinct_set"].as_array().expect("array").len(), 4);
    assert!(json["instinct_set"]
        .as_array()
        .expect("array")
        .iter()
        .any(|entry| {
            entry["instinct_id"] == "integrity"
                && entry["subordinate_to"]
                    .as_array()
                    .expect("array")
                    .iter()
                    .any(|value| value == "policy")
        }));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity instinct"));
}

#[test]
fn identity_instinct_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-instinct-errors");

    let err = real_identity_in_repo(&["instinct".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity instinct: --bogus"));

    let err = real_identity_in_repo(&["instinct".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_instinct_runtime_writes_instinct_runtime_surface_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-instinct-runtime");
    let out_path = repo.join(".adl/state/instinct_runtime_surface_v1.json");

    real_identity_in_repo(
        &[
            "instinct-runtime".to_string(),
            "--out".to_string(),
            ".adl/state/instinct_runtime_surface_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity instinct-runtime");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "instinct_runtime_surface.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/instinct_runtime_surface_v1.json"
    );
    assert!(json["proof_cases"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value["expected_candidate_id"] == "cand-fast-verify"));
    assert!(json["review_surface"]["policy_override_rule"]
        .as_str()
        .expect("string")
        .contains("high-risk slow-path review remains mandatory"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity instinct-runtime"));
}

#[test]
fn identity_instinct_runtime_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-instinct-runtime-errors");

    let err = real_identity_in_repo(
        &["instinct-runtime".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity instinct-runtime: --bogus"));

    let err = real_identity_in_repo(
        &["instinct-runtime".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn required_value_and_git_capture_report_errors() {
    let value = required_value(&["--name".to_string(), "Codex".to_string()], 0, "--name")
        .expect("present flag value should succeed");
    assert_eq!(value, "Codex");

    let err = required_value(&["--name".to_string()], 0, "--name")
        .expect_err("missing flag value should fail");
    assert!(err.to_string().contains("--name requires a value"));

    let git_version = run_git_capture(&["--version"]).expect("git version should succeed");
    assert!(git_version.starts_with("git version "));

    let err = run_git_capture(&["definitely-not-a-real-subcommand"])
        .expect_err("invalid git command should fail");
    assert!(err
        .to_string()
        .contains("git definitely-not-a-real-subcommand failed with status"));
}

#[test]
fn resolve_identity_path_defaults_to_repo_identity_profile_path() {
    let repo = temp_repo("identity-path-default");

    let resolved = resolve_identity_path(&repo, &[]).expect("default path should resolve");

    assert_eq!(resolved, default_identity_profile_path(&repo));
}

#[test]
fn resolve_identity_path_accepts_explicit_path_and_rejects_unknown_args() {
    let repo = temp_repo("identity-path-explicit");

    let resolved = resolve_identity_path(
        &repo,
        &[
            "--path".to_string(),
            "identity/custom_profile.v1.json".to_string(),
        ],
    )
    .expect("explicit path should resolve");
    assert_eq!(resolved, PathBuf::from("identity/custom_profile.v1.json"));

    let err = resolve_identity_path(&repo, &["--bogus".to_string()])
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity show: --bogus"));

    let err = resolve_identity_path(&repo, &["--path".to_string()])
        .expect_err("missing path value should fail");
    assert!(err.to_string().contains("--path requires a value"));
}

#[test]
fn repo_root_matches_git_toplevel() {
    let expected = PathBuf::from(
        run_git_capture(&["rev-parse", "--show-toplevel"])
            .expect("git top level")
            .trim(),
    );

    let resolved = repo_root().expect("repo root should resolve");

    assert_eq!(resolved, expected);
}

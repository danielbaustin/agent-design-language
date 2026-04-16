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
        .contains("identity requires a subcommand: init | show | now | foundation | adversarial-runtime | red-blue-architecture | adversarial-runner | exploit-replay | continuous-verification | operational-skills | skill-composition | delegation-refusal-coordination | provider-extension-packaging | demo-proof-entry-points | schema"));
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
    real_identity_in_repo(
        &["red-blue-architecture".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("red-blue-architecture help");
    real_identity_in_repo(
        &["adversarial-runner".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("adversarial-runner help");
    real_identity_in_repo(&["exploit-replay".to_string(), "--help".to_string()], &repo)
        .expect("exploit-replay help");
    real_identity_in_repo(
        &["continuous-verification".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("continuous-verification help");
    real_identity_in_repo(
        &["demo-proof-entry-points".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("demo-proof-entry-points help");
    real_identity_in_repo(
        &["operational-skills".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("operational-skills help");
    real_identity_in_repo(
        &["skill-composition".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("skill-composition help");
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
fn identity_red_blue_architecture_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-red-blue-architecture");
    let out_path = repo.join(".adl/state/red_blue_agent_architecture_v1.json");

    real_identity_in_repo(
        &[
            "red-blue-architecture".to_string(),
            "--out".to_string(),
            ".adl/state/red_blue_agent_architecture_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity red-blue-architecture");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "red_blue_agent_architecture.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/red_blue_agent_architecture_v1.json"
    );
    assert_eq!(json["red_role"]["role"], "red");
    assert!(json["purple_coordination"]["governance_responsibilities"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "govern replay and escalation order"));
    assert!(json["interaction_model"]["stage_order"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "blue risk evaluation"));
    assert!(json["review_surface"]["downstream_boundaries"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value.as_str().expect("string").contains("WP-04")));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity red-blue-architecture"));
}

#[test]
fn identity_red_blue_architecture_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-red-blue-architecture-errors");

    let err = real_identity_in_repo(
        &["red-blue-architecture".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity red-blue-architecture: --bogus"));

    let err = real_identity_in_repo(
        &["red-blue-architecture".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_adversarial_runner_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-adversarial-runner");
    let out_path = repo.join(".adl/state/adversarial_execution_runner_v1.json");

    real_identity_in_repo(
        &[
            "adversarial-runner".to_string(),
            "--out".to_string(),
            ".adl/state/adversarial_execution_runner_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity adversarial-runner");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "adversarial_execution_runner.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/adversarial_execution_runner_v1.json"
    );
    assert!(json["canonical_stages"]
        .as_array()
        .expect("array")
        .iter()
        .any(|stage| stage["stage_id"] == "attempt_bounded_exploit"
            && stage["blocked_in_postures"]
                .as_array()
                .expect("array")
                .iter()
                .any(|posture| posture == "audit")));
    assert!(json["posture_policy"]["enforcement_rules"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "limit exhaustion produces an explicit defer record"));
    assert!(json["evidence_capture"]["linkage_rules"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value
            .as_str()
            .expect("string")
            .contains("mitigation decisions must cite exploit evidence")));
    assert!(json["review_surface"]["downstream_boundaries"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value.as_str().expect("string").contains("WP-05")));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity adversarial-runner"));
}

#[test]
fn identity_adversarial_runner_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-adversarial-runner-errors");

    let err = real_identity_in_repo(
        &["adversarial-runner".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity adversarial-runner: --bogus"));

    let err = real_identity_in_repo(
        &["adversarial-runner".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_exploit_replay_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-exploit-replay");
    let out_path = repo.join(".adl/state/exploit_artifact_replay_v1.json");

    real_identity_in_repo(
        &[
            "exploit-replay".to_string(),
            "--out".to_string(),
            ".adl/state/exploit_artifact_replay_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity exploit-replay");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "exploit_artifact_replay.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/exploit_artifact_replay_v1.json"
    );
    assert!(json["lifecycle_order"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "AdversarialReplayManifest"));
    assert!(json["replay_manifest"]["replay_modes"]
        .as_array()
        .expect("array")
        .iter()
        .any(|mode| mode["mode"] == "bounded_variance"));
    assert!(json["integrity"]["rules"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "no mitigation without exploit evidence linkage"));
    assert!(json["runner_integration"]["upstream_contracts"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adversarial_execution_runner.v1"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity exploit-replay"));
}

#[test]
fn identity_exploit_replay_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-exploit-replay-errors");

    let err = real_identity_in_repo(
        &["exploit-replay".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity exploit-replay: --bogus"));

    let err = real_identity_in_repo(&["exploit-replay".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_continuous_verification_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-continuous-verification");
    let out_path = repo.join(".adl/state/continuous_verification_self_attack_v1.json");

    real_identity_in_repo(
        &[
            "continuous-verification".to_string(),
            "--out".to_string(),
            ".adl/state/continuous_verification_self_attack_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity continuous-verification");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(
        json["schema_version"],
        "continuous_verification_self_attack.v1"
    );
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/continuous_verification_self_attack_v1.json"
    );
    assert!(json["cadence"]["supported_modes"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "continuous_bounded"));
    assert!(json["lifecycle"]
        .as_array()
        .expect("array")
        .iter()
        .any(|stage| stage["stage_id"] == "validate_replay"));
    assert!(json["self_attack_layers"]
        .as_array()
        .expect("array")
        .iter()
        .any(|layer| layer["layer_id"] == "learning_promotion"));
    assert!(json["policy"]["prohibited_shortcuts"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "self-attack without target allowlist"));
    assert!(json["upstream_contracts"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "exploit_artifact_replay.v1"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity continuous-verification"));
}

#[test]
fn identity_continuous_verification_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-continuous-verification-errors");

    let err = real_identity_in_repo(
        &["continuous-verification".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity continuous-verification: --bogus"));

    let err = real_identity_in_repo(
        &["continuous-verification".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_operational_skills_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-operational-skills");
    let out_path = repo.join(".adl/state/operational_skills_substrate_v1.json");

    real_identity_in_repo(
        &[
            "operational-skills".to_string(),
            "--out".to_string(),
            ".adl/state/operational_skills_substrate_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity operational-skills");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "operational_skills_substrate.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/operational_skills_substrate_v1.json"
    );
    assert_eq!(json["execution_phases"][0]["phase_id"], "plan");
    assert_eq!(json["execution_phases"][4]["phase_id"], "commit");
    assert!(json["invocation_boundary"]["required_fields"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "trace_correlation_id"));
    assert!(json["bounded_arxiv_paper_writer"]["prohibited_actions"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "submit_to_arxiv"));
    assert!(json["review_surface"]["downstream_boundaries"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value.as_str().expect("string").contains("WP-09")));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity operational-skills"));
}

#[test]
fn identity_operational_skills_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-operational-skills-errors");

    let err = real_identity_in_repo(
        &["operational-skills".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity operational-skills: --bogus"));

    let err = real_identity_in_repo(
        &["operational-skills".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_skill_composition_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-skill-composition");
    let out_path = repo.join(".adl/state/skill_composition_model_v1.json");

    real_identity_in_repo(
        &[
            "skill-composition".to_string(),
            "--out".to_string(),
            ".adl/state/skill_composition_model_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity skill-composition");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "skill_composition_model.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/skill_composition_model_v1.json"
    );
    assert!(json["primitive_order"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adjudication"));
    assert!(json["graph_contract"]["prohibited_shapes"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "dynamic_graph_mutation_after_plan_phase"));
    assert!(json["bounded_arxiv_writer_composition"]["gates"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value
            .as_str()
            .expect("string")
            .contains("human_publication_gate")));
    assert!(json["review_surface"]["downstream_boundaries"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value.as_str().expect("string").contains("WP-13")));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity skill-composition"));
}

#[test]
fn identity_skill_composition_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-skill-composition-errors");

    let err = real_identity_in_repo(
        &["skill-composition".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity skill-composition: --bogus"));

    let err = real_identity_in_repo(
        &["skill-composition".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_delegation_refusal_coordination_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-delegation-refusal-coordination");
    let out_path = repo.join(".adl/state/delegation_refusal_coordination_v1.json");

    real_identity_in_repo(
        &[
            "delegation-refusal-coordination".to_string(),
            "--out".to_string(),
            ".adl/state/delegation_refusal_coordination_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity delegation-refusal-coordination");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "delegation_refusal_coordination.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/delegation_refusal_coordination_v1.json"
    );
    assert!(json["outcome_taxonomy"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value["outcome_kind"] == "governed_refusal"));
    assert!(
        json["delegation_refusal_boundary"]["failure_separation_rules"]
            .as_array()
            .expect("array")
            .iter()
            .any(|value| value.as_str().expect("string").contains("governed refusal"))
    );
    assert!(json["coordination_negotiation"]["allowed_outcomes"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "bounded_dissent"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity delegation-refusal-coordination"));
}

#[test]
fn identity_delegation_refusal_coordination_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-delegation-refusal-coordination-errors");

    let err = real_identity_in_repo(
        &[
            "delegation-refusal-coordination".to_string(),
            "--bogus".to_string(),
        ],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity delegation-refusal-coordination: --bogus"));

    let err = real_identity_in_repo(
        &[
            "delegation-refusal-coordination".to_string(),
            "--out".to_string(),
        ],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_provider_extension_packaging_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-provider-extension-packaging");
    let out_path = repo.join(".adl/state/provider_extension_packaging_v1.json");

    real_identity_in_repo(
        &[
            "provider-extension-packaging".to_string(),
            "--out".to_string(),
            ".adl/state/provider_extension_packaging_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity provider-extension-packaging");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "provider_extension_packaging.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/provider_extension_packaging_v1.json"
    );
    assert!(json["scope_decision"]["non_promoted_inputs"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "PROVIDER_SECURITY_CAPABILITIES_EXTENSION.md"));
    assert!(
        json["capability_boundary"]["excluded_security_capabilities"]
            .as_array()
            .expect("array")
            .iter()
            .any(|value| value == "provider attestation")
    );
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity provider-extension-packaging"));
}

#[test]
fn identity_provider_extension_packaging_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-provider-extension-packaging-errors");

    let err = real_identity_in_repo(
        &[
            "provider-extension-packaging".to_string(),
            "--bogus".to_string(),
        ],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity provider-extension-packaging: --bogus"));

    let err = real_identity_in_repo(
        &[
            "provider-extension-packaging".to_string(),
            "--out".to_string(),
        ],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_demo_proof_entry_points_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-demo-proof-entry-points");
    let out_path = repo.join(".adl/state/demo_proof_entry_points_v1.json");

    real_identity_in_repo(
        &[
            "demo-proof-entry-points".to_string(),
            "--out".to_string(),
            ".adl/state/demo_proof_entry_points_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity demo-proof-entry-points");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "demo_proof_entry_points.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/demo_proof_entry_points_v1.json"
    );
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity demo-proof-entry-points"));
    assert!(
        json["package"]["rows"]
            .as_array()
            .expect("rows")
            .iter()
            .any(|row| row["demo_id"] == "D1"
                && row["entry_commands"]
                    .as_array()
                    .expect("entry commands")
                    .iter()
                    .any(|command| command
                        == "adl identity adversarial-runtime --out .adl/state/adversarial_runtime_model_v1.json"))
    );
    assert!(json["package"]["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .any(|row| row["demo_id"] == "D8"
            && row["status"] == "LANDED"
            && row["entry_commands"]
                .as_array()
                .expect("entry commands")
                .iter()
                .any(|command| command == "bash adl/tools/demo_v0891_five_agent_hey_jude.sh")));
    assert!(json["package"]["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .any(|row| row["demo_id"] == "D9"
            && row["status"] == "LANDED"
            && row["primary_proof_surfaces"]
                .as_array()
                .expect("proof surfaces")
                .iter()
                .any(|surface| surface
                    == "artifacts/v0891/arxiv_manuscript_workflow/manuscript_status/three_paper_status.json")));
}

#[test]
fn identity_demo_proof_entry_points_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-demo-proof-entry-points-errors");

    let err = real_identity_in_repo(
        &["demo-proof-entry-points".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity demo-proof-entry-points: --bogus"));

    let err = real_identity_in_repo(
        &["demo-proof-entry-points".to_string(), "--out".to_string()],
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

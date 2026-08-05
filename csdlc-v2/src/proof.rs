use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::error::{ErrorCode, Result, V2Error};
use crate::operator::SkillManifest;
use crate::{select_generation, Generation, GenerationSelector};

const REQUIRED_STEPS: [&str; 5] = [
    "build_binaries",
    "full_suite",
    "samples_parity",
    "quality",
    "v2_install_verify",
];
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProofManifest {
    pub schema: String,
    pub default_generation: Generation,
    pub issue: u64,
    pub opted_in_issues: BTreeSet<u64>,
    pub generation_selector: PathBuf,
    pub steps: Vec<ProofStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProofStep {
    pub id: String,
    pub executable: PathBuf,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StepEvidence {
    pub id: String,
    pub executable: PathBuf,
    pub args: Vec<String>,
    pub exit_code: Option<i32>,
    pub elapsed_millis: u64,
    pub stdout_blake3: String,
    pub stderr_blake3: String,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreSwitchEvidence {
    pub schema: String,
    pub revision: String,
    pub default_before: Generation,
    pub explicit_v2_selected: bool,
    pub rollback_to_v1_selected: bool,
    pub default_after: Generation,
    pub v1_paths_before: bool,
    pub v1_paths_after: bool,
    pub steps: Vec<StepEvidence>,
    pub measurements: ProofMeasurements,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofMeasurements {
    pub rust_loc: u64,
    pub test_count: u64,
    pub debug_binary_bytes: Vec<(String, u64)>,
    pub construction_and_full_suite_millis: u64,
    pub total_proof_millis: u64,
    pub loc_is_reviewable_not_a_hard_cap: bool,
}

impl ProofManifest {
    pub fn validate(&self) -> Result<()> {
        if self.schema != "csdlc.pre_switch_proof_manifest.v1"
            || self.default_generation != Generation::V1
            || !self.opted_in_issues.contains(&self.issue)
            || self.generation_selector != Path::new("csdlc-v2/operator/generation-selector.json")
        {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                "pre-switch proof requires schema v1, v1 default, and explicit v2 opt-in",
            ));
        }
        let ids = self
            .steps
            .iter()
            .map(|step| step.id.as_str())
            .collect::<BTreeSet<_>>();
        if ids != REQUIRED_STEPS.into_iter().collect() || ids.len() != self.steps.len() {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                "proof manifest requires exactly the four typed proof steps",
            ));
        }
        for step in &self.steps {
            let expected = expected_command(&step.id)
                .ok_or_else(|| V2Error::new(ErrorCode::InvalidManifest, "unknown proof step"))?;
            if step.executable != Path::new(expected.0) || step.args != expected.1 {
                return Err(V2Error::new(
                    ErrorCode::InvalidManifest,
                    "proof step executable and argv must match the reviewed typed contract",
                ));
            }
        }
        Ok(())
    }
}

pub fn run_pre_switch_proof(repo: &Path, manifest: &ProofManifest) -> Result<PreSwitchEvidence> {
    manifest.validate()?;
    require_clean_revision(repo)?;
    let selector = read_selector(repo, &manifest.generation_selector)?;
    if selector.default_generation != manifest.default_generation
        || selector.opted_in_issues != manifest.opted_in_issues
    {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "proof manifest must exactly match the tracked generation selector",
        ));
    }
    let default_before = select_generation(&selector, manifest.issue, None)?;
    let explicit_v2_selected =
        select_generation(&selector, manifest.issue, Some(Generation::V2))? == Generation::V2;
    let rollback_to_v1_selected =
        select_generation(&selector, manifest.issue, None)? == Generation::V1;
    // Historical Gate 10B proof only: this pre-switch lane is never the final
    // v1-sunset validation path and intentionally records incumbent presence.
    let v1_paths_before = historical_v1_paths_exist(repo);
    let revision = command_text(repo, "git", &["rev-parse", "HEAD"])?;
    let mut steps = Vec::new();
    for step in &manifest.steps {
        let started = Instant::now();
        let output = Command::new(&step.executable)
            .args(&step.args)
            .current_dir(repo)
            .output()
            .map_err(|error| V2Error::new(ErrorCode::Io, format!("{}: {error}", step.id)))?;
        steps.push(StepEvidence {
            id: step.id.clone(),
            executable: step.executable.clone(),
            args: step.args.clone(),
            exit_code: output.status.code(),
            elapsed_millis: started.elapsed().as_millis().try_into().unwrap_or(u64::MAX),
            stdout_blake3: blake3::hash(&output.stdout).to_hex().to_string(),
            stderr_blake3: blake3::hash(&output.stderr).to_hex().to_string(),
            passed: output.status.success(),
        });
    }
    let selector_after = read_selector(repo, &manifest.generation_selector)?;
    let default_after = select_generation(&selector_after, manifest.issue, None)?;
    let v1_paths_after = historical_v1_paths_exist(repo);
    require_clean_revision(repo)?;
    let measurements = measure(repo, &steps)?;
    let passed = default_before == Generation::V1
        && explicit_v2_selected
        && rollback_to_v1_selected
        && default_after == Generation::V1
        && selector_after == selector
        && v1_paths_before
        && v1_paths_after
        && steps.iter().all(|step| step.passed);
    Ok(PreSwitchEvidence {
        schema: "csdlc.pre_switch_evidence.v1".into(),
        revision,
        default_before,
        explicit_v2_selected,
        rollback_to_v1_selected,
        default_after,
        v1_paths_before,
        v1_paths_after,
        steps,
        measurements,
        passed,
    })
}

fn read_selector(repo: &Path, relative: &Path) -> Result<GenerationSelector> {
    if relative.is_absolute()
        || relative
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "generation selector must be a repo-relative non-traversing path",
        ));
    }
    let path = repo.join(relative);
    if !regular_file(&path) {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "generation selector must be a regular non-symlink file",
        ));
    }
    let tracked = Command::new("git")
        .args(["ls-files", "--error-unmatch", "--"])
        .arg(relative)
        .current_dir(repo)
        .output()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    if !tracked.status.success() {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "generation selector must be tracked at the reviewed path",
        ));
    }
    serde_json::from_slice(&fs::read(path)?).map_err(Into::into)
}

pub fn write_evidence_atomic(path: &Path, evidence: &PreSwitchEvidence) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "evidence output needs a parent"))?;
    fs::create_dir_all(parent)?;
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(evidence)?)?;
    fs::rename(temporary, path)?;
    Ok(())
}

fn historical_v1_paths_exist(repo: &Path) -> bool {
    [
        ("adl/tools/pr.sh", true),
        ("adl/tools/install_owner_binaries.sh", true),
        ("adl/src/bin/adl_csdlc.rs", false),
        ("adl/src/cli/pr_cmd.rs", false),
    ]
    .iter()
    .all(|(path, executable)| {
        let path = repo.join(path);
        regular_file(&path) && (!executable || executable_file(&path))
    })
}

fn expected_command(id: &str) -> Option<(&'static str, Vec<String>)> {
    let values: &[&str] = match id {
        "build_binaries" => &["build", "--manifest-path", "csdlc-v2/Cargo.toml", "--bins"],
        "full_suite" => &["test", "--manifest-path", "csdlc-v2/Cargo.toml"],
        "samples_parity" => &[
            "test",
            "--manifest-path",
            "csdlc-v2/Cargo.toml",
            "--test",
            "gate9",
        ],
        "quality" => &[
            "clippy",
            "--manifest-path",
            "csdlc-v2/Cargo.toml",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
        "v2_install_verify" => return Some(("csdlc-install", vec!["verify".into()])),
        _ => return None,
    };
    Some((
        "cargo",
        values.iter().map(|value| (*value).into()).collect(),
    ))
}

fn regular_file(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|metadata| metadata.file_type().is_file())
        .unwrap_or(false)
}

#[cfg(unix)]
fn executable_file(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn executable_file(path: &Path) -> bool {
    regular_file(path)
}

fn command_text(repo: &Path, executable: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(executable)
        .args(args)
        .current_dir(repo)
        .output()
        .map_err(|error| V2Error::new(ErrorCode::Io, error.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            "revision lookup failed",
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().into())
}

fn measure(repo: &Path, steps: &[StepEvidence]) -> Result<ProofMeasurements> {
    let mut rust_loc = 0;
    let mut test_count = 0;
    for root in [repo.join("csdlc-v2/src"), repo.join("csdlc-v2/tests")] {
        for path in walk(&root)? {
            if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                let text = fs::read_to_string(path)?;
                rust_loc += text.lines().filter(|line| !line.trim().is_empty()).count() as u64;
                test_count += text.matches("#[test]").count() as u64;
            }
        }
    }
    let target = crate::operator::external_cargo_target(repo)?.join("debug");
    let mut debug_binary_bytes = Vec::new();
    for name in SkillManifest::load()?.required_binaries() {
        let path = target.join(&name);
        if !regular_file(&path) || !executable_file(&path) {
            return Err(V2Error::new(
                ErrorCode::ValidationFailed,
                format!("revision-current binary measurement is missing {name}"),
            ));
        }
        debug_binary_bytes.push((name, fs::metadata(path)?.len()));
    }
    debug_binary_bytes.sort();
    Ok(ProofMeasurements {
        rust_loc,
        test_count,
        debug_binary_bytes,
        construction_and_full_suite_millis: steps
            .iter()
            .find(|step| step.id == "full_suite")
            .map(|step| step.elapsed_millis)
            .unwrap_or(0),
        total_proof_millis: steps.iter().map(|step| step.elapsed_millis).sum(),
        loc_is_reviewable_not_a_hard_cap: true,
    })
}

pub fn require_clean_revision(repo: &Path) -> Result<()> {
    let output = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=all"])
        .current_dir(repo)
        .output()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    if !output.status.success() || !output.stdout.is_empty() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "pre-switch proof requires a clean exact revision before execution",
        ));
    }
    Ok(())
}

fn walk(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            if path.is_dir() {
                pending.push(path);
            } else {
                files.push(path);
            }
        }
    }
    Ok(files)
}

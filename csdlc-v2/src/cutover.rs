use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::error::{ErrorCode, Result, V2Error};
use crate::proof::{require_clean_revision, PreSwitchEvidence, StepEvidence};
use crate::{Generation, GenerationSelector};

const SELECTOR_PATH: &str = "csdlc-v2/operator/generation-selector.json";
const PRE_SWITCH_PATH: &str = "docs/architecture/csdlc-v2/gate10b/PRE_SWITCH_EVIDENCE.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CutoverRequest {
    pub schema: String,
    pub issue: u64,
    pub selector_path: PathBuf,
    pub pre_switch_evidence_path: PathBuf,
    pub rollback_window_days: i64,
    pub importer_window_days: i64,
    pub deletion_authorized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CutoverEvidence {
    pub schema: String,
    pub code_revision: String,
    pub pre_switch_evidence_blake3: String,
    pub transitions: Vec<Generation>,
    pub steps: Vec<StepEvidence>,
    pub final_generation: Generation,
    pub explicit_v1_override: bool,
    pub v1_paths_before: bool,
    pub v1_paths_after: bool,
    pub cutover_at: String,
    pub rollback_expires_at: String,
    pub importer_expires_at: String,
    pub deletion_authorized: bool,
    pub passed: bool,
}

impl CutoverRequest {
    pub fn validate(&self) -> Result<()> {
        if self.schema != "csdlc.cutover_request.v1"
            || self.issue != 5294
            || self.selector_path != Path::new(SELECTOR_PATH)
            || self.pre_switch_evidence_path != Path::new(PRE_SWITCH_PATH)
            || self.deletion_authorized
        {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                "cutover request must use fixed Gate 10C paths, issue 5294, and deletion_authorized=false",
            ));
        }
        if self.rollback_window_days != 14 || self.importer_window_days != 30 {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                "rollback and importer expiries must be exactly 14 and 30 days after cutover",
            ));
        }
        Ok(())
    }
}

pub fn run_cutover(repo: &Path, request: &CutoverRequest) -> Result<CutoverEvidence> {
    run_cutover_with(
        repo,
        request,
        run_step,
        write_selector,
        changed_paths,
        OffsetDateTime::now_utc,
    )
}

fn run_cutover_with<F, W, C, N>(
    repo: &Path,
    request: &CutoverRequest,
    mut runner: F,
    mut writer: W,
    mut changes: C,
    now: N,
) -> Result<CutoverEvidence>
where
    F: FnMut(&Path, &str) -> StepEvidence,
    W: FnMut(&Path, &GenerationSelector) -> Result<()>,
    C: FnMut(&Path) -> Result<BTreeSet<PathBuf>>,
    N: Fn() -> OffsetDateTime,
{
    request.validate()?;
    require_clean_revision(repo)?;
    require_tracked(repo, &request.selector_path)?;
    require_tracked(repo, &request.pre_switch_evidence_path)?;
    let code_revision = git_text(repo, &["rev-parse", "HEAD"])?;
    let pre_switch_bytes = fs::read(repo.join(&request.pre_switch_evidence_path))?;
    let pre_switch: PreSwitchEvidence = serde_json::from_slice(&pre_switch_bytes)?;
    if !pre_switch.passed
        || pre_switch.default_before != Generation::V1
        || pre_switch.default_after != Generation::V1
        || !pre_switch.v1_paths_before
        || !pre_switch.v1_paths_after
    {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "current merged Phase B evidence does not authorize reversible cutover",
        ));
    }
    let selector_path = repo.join(&request.selector_path);
    let original_bytes = fs::read(&selector_path)?;
    let mut selector: GenerationSelector = serde_json::from_slice(&original_bytes)?;
    if selector.default_generation != Generation::V1 {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "cutover must begin from v1 default",
        ));
    }
    selector.opted_in_issues.insert(request.issue);
    let v1_paths_before = v1_paths_intact(repo);
    let mut transitions = vec![Generation::V1];
    let mut steps = Vec::new();

    selector.default_generation = Generation::V2;
    if let Err(error) = writer(&selector_path, &selector) {
        restore_after_error(&selector_path, &original_bytes, error)?;
        unreachable!();
    }
    transitions.push(Generation::V2);
    steps.push(runner(repo, "v2_lifecycle_smoke"));
    if !steps.last().is_some_and(|step| step.passed) {
        restore_exact(&selector_path, &original_bytes)?;
        return Ok(failed_evidence(
            repo,
            code_revision,
            &pre_switch_bytes,
            transitions,
            steps,
            v1_paths_before,
        ));
    }

    selector.default_generation = Generation::V1;
    if let Err(error) = writer(&selector_path, &selector) {
        restore_after_error(&selector_path, &original_bytes, error)?;
        unreachable!();
    }
    transitions.push(Generation::V1);
    selector.default_generation = Generation::V2;
    if let Err(error) = writer(&selector_path, &selector) {
        restore_after_error(&selector_path, &original_bytes, error)?;
        unreachable!();
    }
    transitions.push(Generation::V2);
    steps.push(runner(repo, "v2_switch_back_smoke"));
    let final_generation = selector.default_generation;
    let explicit_v1_override =
        resolve_generation(&selector, Some(Generation::V1)) == Generation::V1;
    let v1_paths_after = v1_paths_intact(repo);
    let observed_changes = match changes(repo) {
        Ok(value) => value,
        Err(error) => {
            restore_after_error(&selector_path, &original_bytes, error)?;
            unreachable!();
        }
    };
    let only_selector_changed = observed_changes == BTreeSet::from([request.selector_path.clone()]);
    let passed = steps.iter().all(|step| step.passed)
        && final_generation == Generation::V2
        && explicit_v1_override
        && v1_paths_before
        && v1_paths_after
        && only_selector_changed;
    if !passed {
        restore_exact(&selector_path, &original_bytes)?;
    }
    let cutover = now();
    let format = &time::format_description::well_known::Rfc3339;
    Ok(CutoverEvidence {
        schema: "csdlc.cutover_evidence.v1".into(),
        code_revision,
        pre_switch_evidence_blake3: blake3::hash(&pre_switch_bytes).to_hex().to_string(),
        transitions,
        steps,
        final_generation: if passed {
            Generation::V2
        } else {
            Generation::V1
        },
        explicit_v1_override,
        v1_paths_before,
        v1_paths_after,
        cutover_at: cutover.format(format).map_err(time_error)?,
        rollback_expires_at: (cutover + Duration::days(request.rollback_window_days))
            .format(format)
            .map_err(time_error)?,
        importer_expires_at: (cutover + Duration::days(request.importer_window_days))
            .format(format)
            .map_err(time_error)?,
        deletion_authorized: false,
        passed,
    })
}

pub fn resolve_generation(
    selector: &GenerationSelector,
    requested: Option<Generation>,
) -> Generation {
    requested.unwrap_or(selector.default_generation)
}

pub fn write_evidence_atomic(path: &Path, evidence: &CutoverEvidence) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "evidence output needs a parent"))?;
    fs::create_dir_all(parent)?;
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(evidence)?)?;
    fs::rename(temporary, path)?;
    Ok(())
}

fn failed_evidence(
    repo: &Path,
    revision: String,
    pre: &[u8],
    transitions: Vec<Generation>,
    steps: Vec<StepEvidence>,
    v1_before: bool,
) -> CutoverEvidence {
    CutoverEvidence {
        schema: "csdlc.cutover_evidence.v1".into(),
        code_revision: revision,
        pre_switch_evidence_blake3: blake3::hash(pre).to_hex().to_string(),
        transitions,
        steps,
        final_generation: Generation::V1,
        explicit_v1_override: true,
        v1_paths_before: v1_before,
        v1_paths_after: v1_paths_intact(repo),
        cutover_at: String::new(),
        rollback_expires_at: String::new(),
        importer_expires_at: String::new(),
        deletion_authorized: false,
        passed: false,
    }
}

fn run_step(repo: &Path, id: &str) -> StepEvidence {
    let (executable, args): (&str, Vec<String>) = match id {
        "v2_lifecycle_smoke" | "v2_switch_back_smoke" => (
            "cargo",
            vec![
                "test".into(),
                "--manifest-path".into(),
                "csdlc-v2/Cargo.toml".into(),
                "--test".into(),
                "gate7_lifecycle".into(),
            ],
        ),
        _ => ("", Vec::new()),
    };
    let started = Instant::now();
    let output = Command::new(executable)
        .args(&args)
        .current_dir(repo)
        .output();
    match output {
        Ok(output) => StepEvidence {
            id: id.into(),
            executable: executable.into(),
            args,
            exit_code: output.status.code(),
            elapsed_millis: started.elapsed().as_millis().try_into().unwrap_or(u64::MAX),
            stdout_blake3: blake3::hash(&output.stdout).to_hex().to_string(),
            stderr_blake3: blake3::hash(&output.stderr).to_hex().to_string(),
            passed: output.status.success(),
        },
        Err(error) => StepEvidence {
            id: id.into(),
            executable: executable.into(),
            args,
            exit_code: None,
            elapsed_millis: started.elapsed().as_millis().try_into().unwrap_or(u64::MAX),
            stdout_blake3: blake3::hash(&[]).to_hex().to_string(),
            stderr_blake3: blake3::hash(error.to_string().as_bytes())
                .to_hex()
                .to_string(),
            passed: false,
        },
    }
}

fn write_selector(path: &Path, selector: &GenerationSelector) -> Result<()> {
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(selector)?)?;
    fs::rename(temporary, path)?;
    Ok(())
}

fn restore_exact(path: &Path, bytes: &[u8]) -> Result<()> {
    let temporary = path.with_extension("json.restore.tmp");
    fs::write(&temporary, bytes)?;
    fs::rename(temporary, path)?;
    Ok(())
}

fn restore_after_error(path: &Path, bytes: &[u8], original: V2Error) -> Result<()> {
    match restore_exact(path, bytes) {
        Ok(()) => Err(original),
        Err(restore) => Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!(
                "cutover failed ({original}); exact selector restoration also failed ({restore})"
            ),
        )),
    }
}

fn time_error(error: time::error::Format) -> V2Error {
    V2Error::new(ErrorCode::InvalidInput, error.to_string())
}

fn require_tracked(repo: &Path, path: &Path) -> Result<()> {
    let output = Command::new("git")
        .args(["ls-files", "--error-unmatch", "--"])
        .arg(path)
        .current_dir(repo)
        .output()?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            format!("required cutover input is not tracked: {}", path.display()),
        ));
    }
    Ok(())
}

fn changed_paths(repo: &Path) -> Result<BTreeSet<PathBuf>> {
    let output = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=all"])
        .current_dir(repo)
        .output()?;
    if !output.status.success() {
        return Err(V2Error::new(ErrorCode::GitFailure, "git status failed"));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.get(3..))
        .map(PathBuf::from)
        .collect())
}

fn git_text(repo: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).current_dir(repo).output()?;
    if !output.status.success() {
        return Err(V2Error::new(ErrorCode::GitFailure, "git command failed"));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().into())
}

fn v1_paths_intact(repo: &Path) -> bool {
    [
        "adl/tools/pr.sh",
        "adl/tools/install_owner_binaries.sh",
        "adl/src/bin/adl_csdlc.rs",
        "adl/src/cli/pr_cmd.rs",
    ]
    .iter()
    .all(|path| repo.join(path).is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> CutoverRequest {
        CutoverRequest {
            schema: "csdlc.cutover_request.v1".into(),
            issue: 5294,
            selector_path: SELECTOR_PATH.into(),
            pre_switch_evidence_path: PRE_SWITCH_PATH.into(),
            rollback_window_days: 14,
            importer_window_days: 30,
            deletion_authorized: false,
        }
    }

    fn repo() -> tempfile::TempDir {
        let repo = tempfile::tempdir().unwrap();
        for path in [
            SELECTOR_PATH,
            PRE_SWITCH_PATH,
            "adl/tools/pr.sh",
            "adl/tools/install_owner_binaries.sh",
            "adl/src/bin/adl_csdlc.rs",
            "adl/src/cli/pr_cmd.rs",
        ] {
            fs::create_dir_all(repo.path().join(path).parent().unwrap()).unwrap();
        }
        fs::write(
            repo.path().join(SELECTOR_PATH),
            serde_json::to_vec_pretty(&GenerationSelector {
                schema: "csdlc.generation_selector.v1".into(),
                default_generation: Generation::V1,
                opted_in_issues: BTreeSet::from([5293]),
            })
            .unwrap(),
        )
        .unwrap();
        fs::write(
            repo.path().join(PRE_SWITCH_PATH),
            include_bytes!("../../docs/architecture/csdlc-v2/gate10b/PRE_SWITCH_EVIDENCE.json"),
        )
        .unwrap();
        for path in [
            "adl/tools/pr.sh",
            "adl/tools/install_owner_binaries.sh",
            "adl/src/bin/adl_csdlc.rs",
            "adl/src/cli/pr_cmd.rs",
        ] {
            fs::write(repo.path().join(path), b"v1").unwrap();
        }
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        Command::new("git")
            .args(["config", "user.email", "cutover@example.invalid"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Cutover"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(repo.path())
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "baseline"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        repo
    }

    fn evidence(id: &str, passed: bool) -> StepEvidence {
        StepEvidence {
            id: id.into(),
            executable: "fixture".into(),
            args: Vec::new(),
            exit_code: Some(if passed { 0 } else { 1 }),
            elapsed_millis: 1,
            stdout_blake3: blake3::hash(&[]).to_hex().to_string(),
            stderr_blake3: blake3::hash(&[]).to_hex().to_string(),
            passed,
        }
    }

    fn fixed_now() -> OffsetDateTime {
        OffsetDateTime::parse(
            "2026-07-13T09:00:00Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap()
    }

    fn run_fixture<F>(repo: &Path, runner: F) -> Result<CutoverEvidence>
    where
        F: FnMut(&Path, &str) -> StepEvidence,
    {
        run_cutover_with(
            repo,
            &request(),
            runner,
            write_selector,
            changed_paths,
            fixed_now,
        )
    }

    #[test]
    fn clocks_and_deletion_authority_are_fixed() {
        assert!(request().validate().is_ok());
        let mut invalid = request();
        invalid.deletion_authorized = true;
        assert!(invalid.validate().is_err());
        let mut invalid = request();
        invalid.rollback_window_days = 30;
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn successful_transaction_proves_v2_v1_v2_and_keeps_override() {
        let repo = repo();
        let result = run_fixture(repo.path(), |_, id| evidence(id, true)).unwrap();
        assert!(result.passed, "{result:#?}");
        assert_eq!(
            result.transitions,
            vec![
                Generation::V1,
                Generation::V2,
                Generation::V1,
                Generation::V2
            ]
        );
        assert_eq!(result.final_generation, Generation::V2);
        assert!(result.explicit_v1_override);
        assert!(!result.deletion_authorized);
    }

    #[test]
    fn failed_smoke_restores_v1() {
        let repo = repo();
        let original = fs::read(repo.path().join(SELECTOR_PATH)).unwrap();
        let result = run_fixture(repo.path(), |_, id| evidence(id, false)).unwrap();
        assert!(!result.passed);
        assert_eq!(result.final_generation, Generation::V1);
        let selector: GenerationSelector =
            serde_json::from_slice(&fs::read(repo.path().join(SELECTOR_PATH)).unwrap()).unwrap();
        assert_eq!(selector.default_generation, Generation::V1);
        assert_eq!(fs::read(repo.path().join(SELECTOR_PATH)).unwrap(), original);
    }

    #[test]
    fn every_failed_smoke_restores_exact_original_selector_bytes() {
        for failed_at in 1..=2 {
            let repo = repo();
            let original = fs::read(repo.path().join(SELECTOR_PATH)).unwrap();
            let mut call = 0;
            let result = run_fixture(repo.path(), |_, id| {
                call += 1;
                evidence(id, call != failed_at)
            })
            .unwrap();
            assert!(!result.passed);
            assert_eq!(fs::read(repo.path().join(SELECTOR_PATH)).unwrap(), original);
        }
    }

    #[test]
    fn every_selector_write_error_restores_exact_original_bytes() {
        for failed_at in 1..=3 {
            let repo = repo();
            let original = fs::read(repo.path().join(SELECTOR_PATH)).unwrap();
            let mut call = 0;
            let result = run_cutover_with(
                repo.path(),
                &request(),
                |_, id| evidence(id, true),
                |path, selector| {
                    call += 1;
                    write_selector(path, selector)?;
                    if call == failed_at {
                        Err(V2Error::new(ErrorCode::Io, "injected write failure"))
                    } else {
                        Ok(())
                    }
                },
                changed_paths,
                fixed_now,
            );
            assert!(result.is_err());
            assert_eq!(fs::read(repo.path().join(SELECTOR_PATH)).unwrap(), original);
        }
    }

    #[test]
    fn changed_path_failure_restores_exact_original_bytes() {
        let repo = repo();
        let original = fs::read(repo.path().join(SELECTOR_PATH)).unwrap();
        let result = run_cutover_with(
            repo.path(),
            &request(),
            |_, id| evidence(id, true),
            write_selector,
            |_| {
                Err(V2Error::new(
                    ErrorCode::GitFailure,
                    "injected status failure",
                ))
            },
            fixed_now,
        );
        assert!(result.is_err());
        assert_eq!(fs::read(repo.path().join(SELECTOR_PATH)).unwrap(), original);
    }
}

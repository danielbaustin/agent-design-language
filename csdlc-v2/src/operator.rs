use crate::error::{ErrorCode, Result, V2Error};
use crate::{select_generation, Generation, GenerationSelector};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const SKILLS: &str = include_str!("../operator/skills.json");
const COEXISTENCE: &str = include_str!("../operator/coexistence.json");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SkillManifest {
    pub schema: String,
    pub generation: String,
    pub generation_selector: String,
    pub skills: Vec<SkillRoute>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SkillRoute {
    pub name: String,
    pub binary: String,
    pub subcommand: Option<String>,
    pub mutates_state: bool,
    pub auxiliary_binaries: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CoexistenceInventory {
    pub schema: String,
    pub v1_sunset: bool,
    pub required_v1_paths: Vec<RequiredPath>,
    pub forbidden_v1_paths: Vec<PathBuf>,
    pub required_v2_binaries: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RequiredPath {
    pub path: PathBuf,
    pub executable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoexistenceReport {
    pub schema: String,
    pub pass: bool,
    pub default_generation: Generation,
    pub missing_v1_paths: Vec<PathBuf>,
    pub present_forbidden_v1_paths: Vec<PathBuf>,
    pub missing_v2_binaries: Vec<String>,
    pub skill_count: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InstallReceipt {
    pub schema: String,
    pub destination: PathBuf,
    pub source_revision: String,
    pub binaries: Vec<InstalledBinary>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InstalledBinary {
    pub name: String,
    pub blake3: String,
}

impl SkillManifest {
    pub fn load() -> Result<Self> {
        let manifest: Self = serde_json::from_str(SKILLS).map_err(|e| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("invalid embedded skill manifest: {e}"),
            )
        })?;
        manifest.validate()?;
        Ok(manifest)
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema != "csdlc.operator_skills.v1"
            || self.generation != "v2"
            || self.generation_selector != "csdlc-v2/operator/generation-selector.json"
            || self.skills.len() != 9
        {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                "operator manifest must declare nine v2 skills bound to the tracked generation selector",
            ));
        }
        let mut names = BTreeSet::new();
        for route in &self.skills {
            if !names.insert(&route.name)
                || !route.binary.starts_with("csdlc-")
                || route.binary.contains("python")
                || route.binary.ends_with(".sh")
            {
                return Err(V2Error::new(
                    ErrorCode::InvalidManifest,
                    "skills must be unique typed C-SDLC binary routes",
                ));
            }
            for binary in std::iter::once(&route.binary).chain(&route.auxiliary_binaries) {
                if !binary.starts_with("csdlc-") || binary.contains('/') {
                    return Err(V2Error::new(
                        ErrorCode::InvalidManifest,
                        "all skill executables must be simple typed C-SDLC binary names",
                    ));
                }
            }
        }
        Ok(())
    }

    fn binaries(&self) -> BTreeSet<&str> {
        self.skills
            .iter()
            .flat_map(|route| {
                std::iter::once(route.binary.as_str())
                    .chain(route.auxiliary_binaries.iter().map(String::as_str))
            })
            .collect()
    }

    pub fn required_binaries(&self) -> BTreeSet<String> {
        self.binaries().into_iter().map(str::to_owned).collect()
    }
}

impl CoexistenceInventory {
    pub fn load() -> Result<Self> {
        serde_json::from_str(COEXISTENCE).map_err(Into::into)
    }
}

pub fn verify_coexistence(
    repo: &Path,
    bin_dir: &Path,
    inventory: &CoexistenceInventory,
) -> Result<CoexistenceReport> {
    let manifest = SkillManifest::load()?;
    let reviewed = CoexistenceInventory::load()?;
    if inventory != &reviewed {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "coexistence input must exactly match the embedded reviewed inventory",
        ));
    }
    if inventory.schema != "csdlc.coexistence_inventory.v2" || !inventory.v1_sunset {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "final v2 installation requires the reviewed v1-sunset inventory",
        ));
    }
    let selector_path = checked_repo_path(repo, Path::new(&manifest.generation_selector))?;
    if !is_regular_file(&selector_path) {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "tracked generation selector must be a regular file, not a symlink",
        ));
    }
    let selector: GenerationSelector =
        serde_json::from_slice(&fs::read(&selector_path).map_err(io_error)?)?;
    select_generation(&selector, 0, None)?;
    let missing_v1_paths = if inventory.v1_sunset {
        Vec::new()
    } else {
        inventory
            .required_v1_paths
            .iter()
            .filter_map(|required| match checked_repo_path(repo, &required.path) {
                Ok(path)
                    if is_regular_file(&path) && (!required.executable || is_executable(&path)) =>
                {
                    None
                }
                _ => Some(required.path.clone()),
            })
            .collect::<Vec<_>>()
    };
    let mut present_forbidden_v1_paths = inventory
        .forbidden_v1_paths
        .iter()
        .filter(|path| {
            checked_repo_path(repo, path).is_ok_and(|path| fs::symlink_metadata(path).is_ok())
        })
        .cloned()
        .collect::<Vec<_>>();
    for path in discover_forbidden_v1_paths(repo)? {
        if !present_forbidden_v1_paths.contains(&path) {
            present_forbidden_v1_paths.push(path);
        }
    }
    let missing_v2_binaries = inventory
        .required_v2_binaries
        .iter()
        .filter(|n| {
            let path = bin_dir.join(n);
            !is_regular_file(&path) || !is_executable(&path)
        })
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut missing_v2_binaries = missing_v2_binaries;
    missing_v2_binaries.extend(verify_install_receipt(repo, bin_dir, &manifest)?);
    Ok(CoexistenceReport {
        schema: "csdlc.coexistence_report.v2".into(),
        pass: missing_v1_paths.is_empty()
            && present_forbidden_v1_paths.is_empty()
            && missing_v2_binaries.is_empty(),
        default_generation: selector.default_generation,
        missing_v1_paths,
        present_forbidden_v1_paths,
        missing_v2_binaries: missing_v2_binaries.into_iter().collect(),
        skill_count: manifest.skills.len(),
    })
}

fn discover_forbidden_v1_paths(repo: &Path) -> Result<Vec<PathBuf>> {
    let mut found = Vec::new();
    let roots = [
        repo.join("adl/tools"),
        repo.join("adl/src/cli"),
        repo.join("adl/src/bin"),
    ];
    for root in roots {
        if !root.exists() {
            continue;
        }
        let mut stack = vec![root];
        while let Some(path) = stack.pop() {
            for entry in fs::read_dir(&path).map_err(io_error)? {
                let entry = entry.map_err(io_error)?;
                let candidate = entry.path();
                let relative = candidate.strip_prefix(repo).unwrap_or(&candidate);
                if candidate.is_dir() {
                    stack.push(candidate);
                    continue;
                }
                let text = relative.to_string_lossy();
                let forbidden = (text.starts_with("adl/tools/")
                    && (text.ends_with("/pr.sh")
                        || text.contains("/check_pr_")
                        || text.contains("/test_pr_")
                        || (text.contains("/test_prompt_template") && text.ends_with(".sh"))
                        || text.ends_with("/prompt_template.sh")
                        || text.ends_with("/validate_structured_prompt.sh")
                        || text.ends_with("/card_paths.sh")
                        || text.ends_with("/pr_cards.sh")
                        || text.ends_with("/pr_delegate.sh")
                        || text.ends_with("/pr_usage.sh")))
                    || text.starts_with("adl/src/cli/pr_cmd")
                    || text == "adl/src/csdlc_prompt_editor.rs"
                    || text == "adl/src/pr_dispatch_support.rs"
                    || (text.starts_with("adl/src/bin/")
                        && (text.contains("/adl_pr_")
                            || matches!(
                                text.as_ref(),
                                "adl/src/bin/adl_csdlc.rs"
                                    | "adl/src/bin/csdlc.rs"
                                    | "adl/src/bin/adl_issue.rs"
                                    | "adl/src/bin/adl_session.rs"
                            )))
                    || text == "csdlc-v2/src/bin/csdlc-import.rs";
                if forbidden {
                    found.push(relative.to_path_buf());
                }
            }
        }
    }
    Ok(found)
}

pub fn resolve_operator_generation(
    repo: &Path,
    issue: u64,
    requested: Option<Generation>,
) -> Result<Generation> {
    let coexistence: CoexistenceInventory = serde_json::from_str(COEXISTENCE).map_err(|e| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            format!("invalid embedded coexistence inventory: {e}"),
        )
    })?;
    if coexistence.v1_sunset && requested == Some(Generation::V1) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "explicit v1 generation selection is forbidden after v1 sunset",
        ));
    }
    let manifest = SkillManifest::load()?;
    let selector_path = checked_repo_path(repo, Path::new(&manifest.generation_selector))?;
    if !is_regular_file(&selector_path) {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "tracked generation selector must be a regular file, not a symlink",
        ));
    }
    let selector: GenerationSelector =
        serde_json::from_slice(&fs::read(selector_path).map_err(io_error)?)?;
    select_generation(&selector, issue, requested)
}

pub fn install_binaries(source: &Path, destination: &Path) -> Result<InstallReceipt> {
    install_binaries_with_revision(source, destination, None)
}

pub fn build_and_install_binaries(repo: &Path, destination: &Path) -> Result<InstallReceipt> {
    let manifest_path = repo.join("csdlc-v2/Cargo.toml");
    if !is_regular_file(&manifest_path) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "repository is missing csdlc-v2/Cargo.toml",
        ));
    }
    let before = crate::git::run(repo, &["rev-parse", "HEAD"])?;
    if !csdlc_sources_are_clean(repo)? {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "refusing to stamp owner binaries from dirty csdlc-v2 sources",
        ));
    }
    let status = std::process::Command::new("cargo")
        .args(["build", "--manifest-path"])
        .arg(&manifest_path)
        .arg("--bins")
        .current_dir(repo)
        .status()
        .map_err(io_error)?;
    if !status.success() {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "cargo failed to build the typed C-SDLC binaries",
        ));
    }
    let after = crate::git::run(repo, &["rev-parse", "HEAD"])?;
    if before.stdout != after.stdout || !csdlc_sources_are_clean(repo)? {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "csdlc-v2 source revision changed or became dirty during the build",
        ));
    }
    install_binaries_with_revision(
        &repo.join("csdlc-v2/target/debug"),
        destination,
        Some(format!("git:{}", after.stdout)),
    )
}

fn csdlc_sources_are_clean(repo: &Path) -> Result<bool> {
    let status = crate::git::run(
        repo,
        &[
            "status",
            "--porcelain=v1",
            "--untracked-files=all",
            "--",
            "csdlc-v2",
        ],
    )?;
    Ok(status.stdout.trim().is_empty())
}

fn install_binaries_with_revision(
    source: &Path,
    destination: &Path,
    trusted_revision: Option<String>,
) -> Result<InstallReceipt> {
    let manifest = SkillManifest::load()?;
    if destination.file_name().and_then(|value| value.to_str()) != Some("csdlc-v2") {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "v2 binaries must install into a dedicated generation directory named csdlc-v2 (normally .adl/bin/csdlc-v2); shared directories such as .adl/bin are forbidden",
        ));
    }
    let names = manifest.binaries();
    let mut prepared = Vec::new();
    for name in &names {
        let input = source.join(name);
        if !is_regular_file(&input) || !is_executable(&input) {
            return Err(V2Error::new(
                ErrorCode::Io,
                format!(
                    "built binary is missing, non-regular, or non-executable: {}",
                    input.display()
                ),
            ));
        }
        let bytes = fs::read(&input).map_err(io_error)?;
        let permissions = fs::metadata(&input).map_err(io_error)?.permissions();
        prepared.push((name.to_string(), bytes, permissions));
    }
    let binaries = prepared
        .iter()
        .map(|(name, bytes, _)| InstalledBinary {
            name: name.clone(),
            blake3: blake3::hash(bytes).to_hex().to_string(),
        })
        .collect();
    let source_revision = trusted_revision.unwrap_or_else(|| content_provenance(&prepared));
    let mut receipt = InstallReceipt {
        schema: "csdlc.install_receipt.v1".into(),
        destination: destination.to_path_buf(),
        source_revision,
        binaries,
    };
    let parent = destination.parent().ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidInput,
            "installation destination needs a parent",
        )
    })?;
    fs::create_dir_all(parent).map_err(io_error)?;
    let leaf = destination
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "invalid installation destination"))?;
    let stage = parent.join(format!(".{leaf}.stage-{}", std::process::id()));
    let backup = parent.join(format!(".{leaf}.backup-{}", std::process::id()));
    if stage.exists() || backup.exists() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "stale install stage or backup requires reconciliation",
        ));
    }
    fs::create_dir(&stage).map_err(io_error)?;
    receipt.destination = destination.to_path_buf();
    for (name, bytes, permissions) in prepared {
        let output = stage.join(name);
        fs::write(&output, bytes).map_err(io_error)?;
        fs::set_permissions(&output, permissions).map_err(io_error)?;
    }
    fs::write(
        stage.join("install-receipt.json"),
        serde_json::to_vec_pretty(&receipt)
            .map_err(|e| V2Error::new(ErrorCode::Io, e.to_string()))?,
    )
    .map_err(io_error)?;
    let had_destination = destination.exists();
    if had_destination {
        fs::rename(destination, &backup).map_err(io_error)?;
    }
    if let Err(error) = fs::rename(&stage, destination) {
        if had_destination {
            let _ = fs::rename(&backup, destination);
        }
        return Err(io_error(error));
    }
    if had_destination {
        fs::remove_dir_all(&backup).map_err(io_error)?;
    }
    Ok(receipt)
}

fn checked_repo_path(repo: &Path, relative: &Path) -> Result<PathBuf> {
    if relative.is_absolute()
        || relative
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "coexistence paths must be repo-relative without parent traversal",
        ));
    }
    Ok(repo.join(relative))
}

fn verify_install_receipt(
    repo: &Path,
    bin_dir: &Path,
    manifest: &SkillManifest,
) -> Result<BTreeSet<String>> {
    let path = bin_dir.join("install-receipt.json");
    if !is_regular_file(&path) {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "install receipt must be a regular non-symlink file",
        ));
    }
    let receipt: InstallReceipt = serde_json::from_slice(&fs::read(&path).map_err(io_error)?)?;
    if receipt.schema != "csdlc.install_receipt.v1" || receipt.destination != bin_dir {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "install receipt schema or destination does not match the verified generation directory",
        ));
    }
    let current_revision = crate::git::run(repo, &["rev-parse", "HEAD"])
        .map(|revision| format!("git:{}", revision.stdout))
        .ok();
    if current_revision.as_deref() != Some(receipt.source_revision.as_str()) {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            format!(
                "stale owner-binary provenance: installed {} but repository is {}",
                receipt.source_revision,
                current_revision.as_deref().unwrap_or("unavailable")
            ),
        ));
    }
    let expected = manifest.required_binaries();
    let observed = receipt
        .binaries
        .iter()
        .map(|entry| entry.name.clone())
        .collect::<BTreeSet<_>>();
    if observed != expected || observed.len() != receipt.binaries.len() {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "install receipt must contain the exact unique executable set",
        ));
    }
    let mut failures = BTreeSet::new();
    for entry in receipt.binaries {
        let binary = bin_dir.join(&entry.name);
        if !is_regular_file(&binary)
            || !is_executable(&binary)
            || fs::read(&binary)
                .map(|bytes| blake3::hash(&bytes).to_hex().as_str() != entry.blake3)
                .unwrap_or(true)
        {
            failures.insert(entry.name);
        }
    }
    Ok(failures)
}

fn content_provenance(prepared: &[(String, Vec<u8>, fs::Permissions)]) -> String {
    let mut hasher = blake3::Hasher::new();
    for (name, bytes, _) in prepared {
        hasher.update(name.as_bytes());
        hasher.update(bytes);
    }
    format!("content:{}", hasher.finalize().to_hex())
}

fn is_regular_file(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|metadata| metadata.file_type().is_file())
        .unwrap_or(false)
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}
#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}
fn io_error(error: std::io::Error) -> V2Error {
    V2Error::new(ErrorCode::Io, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::resolve_operator_generation;
    use crate::Generation;

    #[test]
    fn v1_override_is_rejected_after_sunset() {
        let error = resolve_operator_generation(std::path::Path::new("."), 1, Some(Generation::V1))
            .expect_err("sunset must reject explicit v1");
        assert!(error.message.contains("v1 sunset"));
    }
}

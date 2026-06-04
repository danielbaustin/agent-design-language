use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

const CURRENT_ADL_VERSION: &str = "0.91.5";
const REQUIRED_SCHEMA_VERSION: &str = "adl.project.v1";
const REQUIRED_FAILURE_MODE: &str = "fail_with_setup_instructions";
const REQUIRED_PROMPT_REGISTRY: &str = "docs/templates/prompts/current.json";

const PROFILES: &[&str] = &["paper", "spec", "demo", "runtime", "library"];
const ISSUE_AUTHORITIES: &[&str] = &["external_repo", "adl_repo", "split_explicit"];
const CARD_LOCATIONS: &[&str] = &[
    "repo_local_ignored_adl",
    "tracked_public_adl_subset",
    "external_adl_state_root",
];
const WORKTREE_LOCATIONS: &[&str] = &[
    "external_repo_worktrees",
    "adl_managed_worktrees",
    "disabled",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DoctorStatus {
    Pass,
    Warning,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct DoctorCheck {
    pub name: String,
    pub status: DoctorStatus,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PortableProjectDoctorReport {
    pub schema_version: String,
    pub status: DoctorStatus,
    pub project_file: String,
    pub resolved_adl_home: Option<String>,
    pub checks: Vec<DoctorCheck>,
}

pub(crate) fn real_portable_project_doctor(args: &[String]) -> Result<()> {
    let mut project: Option<PathBuf> = None;
    let mut adl_home_override: Option<PathBuf> = None;
    let mut json_output = false;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--project" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing value for --project");
                };
                project = Some(PathBuf::from(value));
                i += 1;
            }
            "--adl-home" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing value for --adl-home");
                };
                adl_home_override = Some(PathBuf::from(value));
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", portable_project_doctor_usage());
                return Ok(());
            }
            other => bail!("unknown arg for portable-project-doctor: {other}"),
        }
        i += 1;
    }

    let Some(project) = project else {
        bail!("missing --project");
    };
    let report = run_portable_project_doctor(&project, adl_home_override.as_deref())?;
    if json_output {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print_human_report(&report);
    }
    if report.status == DoctorStatus::Fail {
        bail!("portable project doctor failed");
    }
    Ok(())
}

fn portable_project_doctor_usage() -> &'static str {
    "adl tooling portable-project-doctor --project <adl_project.json> [--adl-home <path>] [--json]"
}

pub(crate) fn run_portable_project_doctor(
    project_file: &Path,
    adl_home_override: Option<&Path>,
) -> Result<PortableProjectDoctorReport> {
    let project_file = absolutize(project_file)?;
    let project_root = project_file
        .parent()
        .ok_or_else(|| anyhow!("project path has no parent: {}", project_file.display()))?;
    let mut checks = Vec::new();

    let config_text = fs::read_to_string(&project_file).map_err(|err| {
        anyhow!(
            "failed to read adl_project.json {}: {err}",
            project_file.display()
        )
    })?;
    let config: Value = serde_json::from_str(&config_text)
        .map_err(|err| anyhow!("adl_project.json is not valid JSON: {err}"))?;

    check_required_string(
        &config,
        "schema_version",
        Some(REQUIRED_SCHEMA_VERSION),
        &mut checks,
    );
    check_required_string(&config, "project_id", None, &mut checks);
    check_enum(&config, "profile", PROFILES, &mut checks);
    check_required_string(&config, "tooling_ref", None, &mut checks);
    check_min_version(&config, &mut checks);
    check_required_string(
        &config,
        "prompt_template_registry",
        Some(REQUIRED_PROMPT_REGISTRY),
        &mut checks,
    );
    check_issue_tracker(&config, &mut checks);
    check_tooling_discovery(&config, &mut checks);
    check_state_policy(&config, &mut checks);
    check_enum(&config, "validation_profile", PROFILES, &mut checks);
    check_artifact_policy(&config, &mut checks);
    check_agents(project_root, &mut checks);

    let resolved_adl_home = resolve_adl_home(&config, project_root, adl_home_override, &mut checks);
    if let Some(adl_home) = resolved_adl_home.as_deref() {
        check_adl_checkout(adl_home, &config, &mut checks);
    }

    let status = aggregate_status(&checks);
    Ok(PortableProjectDoctorReport {
        schema_version: "adl.portable_project_doctor_report.v1".to_string(),
        status,
        project_file: "adl_project.json".to_string(),
        resolved_adl_home: resolved_adl_home
            .as_deref()
            .and_then(|path| safe_resolved_adl_home_label(path, project_root)),
        checks,
    })
}

fn print_human_report(report: &PortableProjectDoctorReport) {
    println!("Portable ADL project doctor: {:?}", report.status);
    println!("Project: {}", report.project_file);
    if let Some(adl_home) = &report.resolved_adl_home {
        println!("ADL home: {adl_home}");
    }
    for check in &report.checks {
        println!("- {:?}: {} - {}", check.status, check.name, check.message);
    }
}

fn absolutize(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn safe_resolved_adl_home_label(path: &Path, project_root: &Path) -> Option<String> {
    if let Ok(rel) = path.strip_prefix(project_root) {
        return Some(display_path(rel));
    }
    path.file_name()
        .map(|name| name.to_string_lossy().replace('\\', "/"))
        .filter(|name| !name.is_empty())
}

fn object<'a>(config: &'a Value, key: &str, checks: &mut Vec<DoctorCheck>) -> Option<&'a Value> {
    let Some(value) = config.get(key) else {
        checks.push(fail(key, "missing required object"));
        return None;
    };
    if !value.is_object() {
        checks.push(fail(key, "must be an object"));
        return None;
    }
    Some(value)
}

fn string_field<'a>(config: &'a Value, key: &str) -> Option<&'a str> {
    config.get(key).and_then(|v| v.as_str()).map(str::trim)
}

fn check_required_string(
    config: &Value,
    key: &str,
    expected: Option<&str>,
    checks: &mut Vec<DoctorCheck>,
) {
    match string_field(config, key) {
        Some(value) if !value.is_empty() => {
            if let Some(expected) = expected {
                if value == expected {
                    checks.push(pass(key, format!("present and set to {expected}")));
                } else {
                    checks.push(fail(key, format!("expected {expected}, got {value}")));
                }
            } else {
                checks.push(pass(key, "present"));
            }
        }
        _ => checks.push(fail(key, "missing or empty required string")),
    }
}

fn check_enum(config: &Value, key: &str, allowed: &[&str], checks: &mut Vec<DoctorCheck>) {
    match string_field(config, key) {
        Some(value) if allowed.contains(&value) => checks.push(pass(key, format!("valid {value}"))),
        Some(value) => checks.push(fail(
            key,
            format!(
                "invalid value {value}; expected one of {}",
                allowed.join(", ")
            ),
        )),
        None => checks.push(fail(key, "missing required enum string")),
    }
}

fn check_min_version(config: &Value, checks: &mut Vec<DoctorCheck>) {
    match string_field(config, "min_adl_version") {
        Some(version) if version_le(version, CURRENT_ADL_VERSION) => checks.push(pass(
            "min_adl_version",
            format!("satisfied by {CURRENT_ADL_VERSION}"),
        )),
        Some(version) => checks.push(fail(
            "min_adl_version",
            format!("requires {version}, current doctor supports {CURRENT_ADL_VERSION}"),
        )),
        None => checks.push(fail("min_adl_version", "missing required version")),
    }
}

fn version_le(required: &str, current: &str) -> bool {
    parse_version(required)
        .zip(parse_version(current))
        .is_some_and(|(r, c)| r <= c)
}

fn parse_version(version: &str) -> Option<(u64, u64, u64)> {
    let parts = version
        .trim()
        .trim_start_matches('v')
        .split('.')
        .collect::<Vec<_>>();
    if parts.len() != 3 {
        return None;
    }
    Some((
        parts[0].parse().ok()?,
        parts[1].parse().ok()?,
        parts[2].parse().ok()?,
    ))
}

fn check_issue_tracker(config: &Value, checks: &mut Vec<DoctorCheck>) {
    let Some(issue_tracker) = object(config, "issue_tracker", checks) else {
        return;
    };
    match string_field(issue_tracker, "provider") {
        Some("github") => checks.push(pass("issue_tracker.provider", "github")),
        Some(other) => checks.push(fail(
            "issue_tracker.provider",
            format!("unsupported provider {other}"),
        )),
        None => checks.push(fail("issue_tracker.provider", "missing provider")),
    }
    match string_field(issue_tracker, "repo") {
        Some(repo) if valid_github_repo_ref(repo) => {
            checks.push(pass("issue_tracker.repo", "repo authority declared"))
        }
        Some(_) => checks.push(fail("issue_tracker.repo", "must be owner/repo")),
        None => checks.push(fail("issue_tracker.repo", "missing repo")),
    }
}

fn valid_github_repo_ref(repo: &str) -> bool {
    let Some((owner, name)) = repo.split_once('/') else {
        return false;
    };
    !owner.is_empty()
        && !name.is_empty()
        && !owner.contains(char::is_whitespace)
        && !name.contains(char::is_whitespace)
        && !name.contains('/')
}

fn check_tooling_discovery(config: &Value, checks: &mut Vec<DoctorCheck>) {
    let Some(tooling) = object(config, "tooling_discovery", checks) else {
        return;
    };
    match string_field(tooling, "env") {
        Some("ADL_HOME") => checks.push(pass("tooling_discovery.env", "ADL_HOME")),
        Some(other) => checks.push(fail(
            "tooling_discovery.env",
            format!("expected ADL_HOME, got {other}"),
        )),
        None => checks.push(fail("tooling_discovery.env", "missing env")),
    }
    match string_field(tooling, "failure_mode") {
        Some(REQUIRED_FAILURE_MODE) => checks.push(pass(
            "tooling_discovery.failure_mode",
            REQUIRED_FAILURE_MODE,
        )),
        Some(other) => checks.push(fail(
            "tooling_discovery.failure_mode",
            format!("expected {REQUIRED_FAILURE_MODE}, got {other}"),
        )),
        None => checks.push(fail(
            "tooling_discovery.failure_mode",
            "missing failure_mode",
        )),
    }
}

fn check_state_policy(config: &Value, checks: &mut Vec<DoctorCheck>) {
    let Some(policy) = object(config, "state_policy", checks) else {
        return;
    };
    check_enum(policy, "issue_authority", ISSUE_AUTHORITIES, checks);
    check_enum(policy, "cards_location", CARD_LOCATIONS, checks);
    check_enum(policy, "worktree_location", WORKTREE_LOCATIONS, checks);
    check_bool(policy, "tracked_public_evidence", checks);
    check_bool(policy, "local_adl_state_ignored", checks);
}

fn check_bool(config: &Value, key: &str, checks: &mut Vec<DoctorCheck>) {
    if config.get(key).and_then(|v| v.as_bool()).is_some() {
        checks.push(pass(key, "boolean present"));
    } else {
        checks.push(fail(key, "missing required boolean"));
    }
}

fn check_artifact_policy(config: &Value, checks: &mut Vec<DoctorCheck>) {
    let Some(policy) = object(config, "artifact_policy", checks) else {
        return;
    };
    match policy
        .get("forbid_absolute_host_paths")
        .and_then(|v| v.as_bool())
    {
        Some(true) => checks.push(pass(
            "artifact_policy.forbid_absolute_host_paths",
            "enabled",
        )),
        Some(false) => checks.push(fail(
            "artifact_policy.forbid_absolute_host_paths",
            "must be true for portable ADL projects",
        )),
        None => checks.push(fail(
            "artifact_policy.forbid_absolute_host_paths",
            "missing required boolean",
        )),
    }
    check_string_array(policy, "public_outputs", checks);
    check_string_array(policy, "private_outputs", checks);
}

fn check_string_array(config: &Value, key: &str, checks: &mut Vec<DoctorCheck>) {
    match config.get(key).and_then(|v| v.as_array()) {
        Some(values) if values.iter().all(|v| v.as_str().is_some()) => {
            checks.push(pass(key, "string array present"))
        }
        Some(_) => checks.push(fail(key, "must be an array of strings")),
        None => checks.push(fail(key, "missing required string array")),
    }
}

fn check_agents(project_root: &Path, checks: &mut Vec<DoctorCheck>) {
    let path = project_root.join("AGENTS.md");
    let Ok(text) = fs::read_to_string(&path) else {
        checks.push(fail("AGENTS.md", "missing root AGENTS.md"));
        return;
    };
    if text.contains("adl_project.json") && text.contains("SIP -> STP -> SPP -> SRP -> SOR") {
        checks.push(pass("AGENTS.md", "portable ADL instructions present"));
    } else {
        checks.push(fail(
            "AGENTS.md",
            "must reference adl_project.json and the canonical card lifecycle",
        ));
    }
}

fn resolve_adl_home(
    config: &Value,
    project_root: &Path,
    override_home: Option<&Path>,
    checks: &mut Vec<DoctorCheck>,
) -> Option<PathBuf> {
    if let Some(path) = override_home {
        checks.push(pass("tooling_discovery.resolver", "--adl-home override"));
        return Some(normalize_existing_path(path));
    }
    if let Some(value) = std::env::var_os("ADL_HOME") {
        checks.push(pass("tooling_discovery.resolver", "ADL_HOME"));
        return Some(normalize_existing_path(Path::new(&value)));
    }
    let tooling = config.get("tooling_discovery")?;
    if let Some(repo_relative) = string_field(tooling, "repo_relative") {
        if !repo_relative.is_empty() {
            checks.push(pass("tooling_discovery.resolver", "repo_relative"));
            return Some(normalize_existing_path(&project_root.join(repo_relative)));
        }
    }
    if let Some(sibling) = string_field(tooling, "sibling_repo") {
        if !sibling.is_empty() {
            checks.push(pass("tooling_discovery.resolver", "sibling_repo"));
            return Some(normalize_existing_path(&project_root.join(sibling)));
        }
    }
    checks.push(fail(
        "tooling_discovery.resolver",
        "no ADL_HOME, repo_relative, or sibling_repo candidate",
    ));
    None
}

fn normalize_existing_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn check_adl_checkout(adl_home: &Path, config: &Value, checks: &mut Vec<DoctorCheck>) {
    if adl_home.is_dir() {
        checks.push(pass("adl_home.path", "exists"));
    } else {
        checks.push(fail(
            "adl_home.path",
            format!("not a directory: {}", adl_home.display()),
        ));
        return;
    }
    let pr_sh = adl_home.join("adl/tools/pr.sh");
    if pr_sh.is_file() && is_executable(&pr_sh) {
        checks.push(pass("adl_home.pr_sh", "exists and executable"));
    } else {
        checks.push(fail(
            "adl_home.pr_sh",
            "adl/tools/pr.sh missing or not executable",
        ));
    }
    let registry =
        string_field(config, "prompt_template_registry").unwrap_or(REQUIRED_PROMPT_REGISTRY);
    if adl_home.join(registry).is_file() {
        checks.push(pass("adl_home.prompt_template_registry", "exists"));
    } else {
        checks.push(fail(
            "adl_home.prompt_template_registry",
            format!("missing {registry}"),
        ));
    }
    if adl_home.join("adl/Cargo.toml").is_file() {
        checks.push(pass("adl_home.cargo_manifest", "exists"));
    } else {
        checks.push(fail("adl_home.cargo_manifest", "missing adl/Cargo.toml"));
    }
    match read_git_origin(adl_home) {
        Some(origin) if origin.contains("agent-design-language") => checks.push(pass(
            "adl_home.git_origin",
            "matches ADL repository identity",
        )),
        Some(origin) => checks.push(fail(
            "adl_home.git_origin",
            format!("unexpected git origin {origin}"),
        )),
        None => checks.push(fail(
            "adl_home.git_origin",
            "missing git origin for ADL checkout",
        )),
    }
    let tooling_ref = string_field(config, "tooling_ref").unwrap_or_default();
    if tooling_ref == "agent-design-language@v0.91.5" {
        checks.push(pass("tooling_ref", "matches current contract"));
    } else {
        checks.push(warning(
            "tooling_ref",
            format!("expected agent-design-language@v0.91.5, got {tooling_ref}"),
        ));
    }
}

fn read_git_origin(adl_home: &Path) -> Option<String> {
    let git_marker = adl_home.join(".git");
    let config_path = if git_marker.is_dir() {
        git_marker.join("config")
    } else if git_marker.is_file() {
        let marker = fs::read_to_string(&git_marker).ok()?;
        let gitdir = marker.trim().strip_prefix("gitdir:")?.trim();
        let gitdir_path = PathBuf::from(gitdir);
        let resolved = if gitdir_path.is_absolute() {
            gitdir_path
        } else {
            adl_home.join(gitdir_path)
        };
        resolved.join("config")
    } else {
        return None;
    };
    let config = fs::read_to_string(config_path).ok()?;
    parse_origin_url(&config)
}

fn parse_origin_url(config: &str) -> Option<String> {
    let mut in_origin = false;
    for line in config.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_origin = trimmed == r#"[remote "origin"]"#;
            continue;
        }
        if in_origin {
            if let Some(url) = trimmed.strip_prefix("url =") {
                return Some(url.trim().to_string());
            }
        }
    }
    None
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}

fn aggregate_status(checks: &[DoctorCheck]) -> DoctorStatus {
    if checks.iter().any(|c| c.status == DoctorStatus::Fail) {
        DoctorStatus::Fail
    } else if checks.iter().any(|c| c.status == DoctorStatus::Warning) {
        DoctorStatus::Warning
    } else {
        DoctorStatus::Pass
    }
}

fn pass(name: impl Into<String>, message: impl Into<String>) -> DoctorCheck {
    DoctorCheck {
        name: name.into(),
        status: DoctorStatus::Pass,
        message: message.into(),
    }
}

fn warning(name: impl Into<String>, message: impl Into<String>) -> DoctorCheck {
    DoctorCheck {
        name: name.into(),
        status: DoctorStatus::Warning,
        message: message.into(),
    }
}

fn fail(name: impl Into<String>, message: impl Into<String>) -> DoctorCheck {
    DoctorCheck {
        name: name.into(),
        status: DoctorStatus::Fail,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn temp_root(label: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("adl-portable-doctor-{label}-{stamp}"));
        fs::create_dir_all(&root).expect("temp root");
        root
    }

    fn write_adl_home(root: &Path) -> PathBuf {
        let adl_home = root.join("agent-design-language");
        fs::create_dir_all(adl_home.join("adl/tools")).expect("tools dir");
        fs::create_dir_all(adl_home.join("adl")).expect("adl dir");
        fs::create_dir_all(adl_home.join("docs/templates/prompts")).expect("prompt dir");
        fs::create_dir_all(adl_home.join(".git")).expect("git dir");
        fs::write(
            adl_home.join("adl/Cargo.toml"),
            "[package]\nname = \"adl\"\n",
        )
        .expect("cargo");
        fs::write(
            adl_home.join(".git/config"),
            "[remote \"origin\"]\n\turl = https://github.com/danielbaustin/agent-design-language.git\n",
        )
        .expect("git config");
        fs::write(adl_home.join("docs/templates/prompts/current.json"), "{}\n").expect("registry");
        let pr_sh = adl_home.join("adl/tools/pr.sh");
        fs::write(&pr_sh, "#!/usr/bin/env bash\n").expect("pr sh");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&pr_sh).expect("metadata").permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&pr_sh, perms).expect("chmod");
        }
        adl_home
    }

    fn write_project(root: &Path, body: &str) -> PathBuf {
        fs::write(
            root.join("AGENTS.md"),
            "Read adl_project.json. Lifecycle: SIP -> STP -> SPP -> SRP -> SOR.\n",
        )
        .expect("agents");
        let path = root.join("adl_project.json");
        fs::write(&path, body).expect("project");
        path
    }

    fn valid_project_json() -> String {
        serde_json::json!({
            "schema_version": "adl.project.v1",
            "project_id": "paper-project",
            "profile": "paper",
            "tooling_ref": "agent-design-language@v0.91.5",
            "min_adl_version": "0.91.5",
            "prompt_template_registry": "docs/templates/prompts/current.json",
            "issue_tracker": {"provider": "github", "repo": "owner/repo"},
            "tooling_discovery": {
                "env": "ADL_HOME",
                "repo_relative": null,
                "sibling_repo": "../agent-design-language",
                "failure_mode": "fail_with_setup_instructions"
            },
            "state_policy": {
                "issue_authority": "external_repo",
                "cards_location": "repo_local_ignored_adl",
                "worktree_location": "external_repo_worktrees",
                "tracked_public_evidence": true,
                "local_adl_state_ignored": true
            },
            "validation_profile": "paper",
            "artifact_policy": {
                "forbid_absolute_host_paths": true,
                "public_outputs": ["review_packet"],
                "private_outputs": ["local_build"]
            }
        })
        .to_string()
    }

    fn valid_project_value() -> Value {
        serde_json::from_str(&valid_project_json()).expect("valid project json value")
    }

    #[test]
    fn portable_project_doctor_passes_valid_project_with_explicit_adl_home() {
        let root = temp_root("valid");
        let adl_home = write_adl_home(&root);
        let project_root = root.join("paper");
        fs::create_dir_all(&project_root).expect("project root");
        let project = write_project(&project_root, &valid_project_json());

        let report = run_portable_project_doctor(&project, Some(&adl_home)).expect("doctor");
        assert_eq!(report.status, DoctorStatus::Pass);
        assert_eq!(report.project_file, "adl_project.json");
        assert_eq!(
            report.resolved_adl_home,
            Some("agent-design-language".to_string())
        );
        assert!(report
            .checks
            .iter()
            .any(|c| c.name == "adl_home.pr_sh" && c.status == DoctorStatus::Pass));
    }

    #[test]
    fn portable_project_doctor_cli_help_json_and_argument_errors_are_stable() {
        let root = temp_root("cli");
        let adl_home = write_adl_home(&root);
        let project_root = root.join("paper");
        fs::create_dir_all(&project_root).expect("project root");
        let project = write_project(&project_root, &valid_project_json());

        real_portable_project_doctor(&["--help".to_string()]).expect("help");
        real_portable_project_doctor(&[
            "--project".to_string(),
            project.to_string_lossy().to_string(),
            "--adl-home".to_string(),
            adl_home.to_string_lossy().to_string(),
            "--json".to_string(),
        ])
        .expect("json output");
        real_portable_project_doctor(&[
            "--project".to_string(),
            project.to_string_lossy().to_string(),
            "--adl-home".to_string(),
            adl_home.to_string_lossy().to_string(),
        ])
        .expect("human output");

        assert!(real_portable_project_doctor(&[]).is_err());
        assert!(real_portable_project_doctor(&["--project".to_string()]).is_err());
        assert!(real_portable_project_doctor(&["--adl-home".to_string()]).is_err());
        assert!(real_portable_project_doctor(&["--unknown".to_string()]).is_err());
    }

    #[test]
    fn portable_project_doctor_resolves_repo_relative_and_reports_tooling_warning() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        let root = temp_root("repo-relative");
        write_adl_home(&root);
        let project_root = root.join("paper");
        fs::create_dir_all(&project_root).expect("project root");
        let mut config = valid_project_value();
        config["tooling_ref"] = serde_json::json!("agent-design-language@v0.91.4");
        config["tooling_discovery"]["repo_relative"] =
            serde_json::json!("../agent-design-language");
        config["tooling_discovery"]["sibling_repo"] = Value::Null;
        let project = write_project(&project_root, &config.to_string());
        let previous = std::env::var_os("ADL_HOME");
        std::env::remove_var("ADL_HOME");

        let report = run_portable_project_doctor(&project, None).expect("doctor");
        assert_eq!(report.status, DoctorStatus::Warning, "{:#?}", report.checks);
        assert_eq!(
            report.resolved_adl_home,
            Some("agent-design-language".to_string())
        );
        assert!(report
            .checks
            .iter()
            .any(|c| c.name == "tooling_discovery.resolver" && c.message == "repo_relative"));
        assert!(report
            .checks
            .iter()
            .any(|c| c.name == "tooling_ref" && c.status == DoctorStatus::Warning));

        match previous {
            Some(value) => std::env::set_var("ADL_HOME", value),
            None => std::env::remove_var("ADL_HOME"),
        }
    }

    #[test]
    fn portable_project_doctor_resolves_adl_home_env_and_fails_missing_discovery() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        let root = temp_root("env");
        let adl_home = write_adl_home(&root);
        let project_root = root.join("paper");
        fs::create_dir_all(&project_root).expect("project root");
        let mut config = valid_project_value();
        config["tooling_discovery"]["repo_relative"] = Value::Null;
        config["tooling_discovery"]["sibling_repo"] = Value::Null;
        let project = write_project(&project_root, &config.to_string());
        let previous = std::env::var_os("ADL_HOME");

        std::env::set_var("ADL_HOME", &adl_home);
        let report = run_portable_project_doctor(&project, None).expect("doctor");
        assert_eq!(report.status, DoctorStatus::Pass, "{:#?}", report.checks);
        assert!(report
            .checks
            .iter()
            .any(|c| c.name == "tooling_discovery.resolver" && c.message == "ADL_HOME"));

        std::env::remove_var("ADL_HOME");
        let failed = run_portable_project_doctor(&project, None).expect("doctor");
        assert_eq!(failed.status, DoctorStatus::Fail);
        assert!(failed
            .checks
            .iter()
            .any(|c| c.name == "tooling_discovery.resolver" && c.status == DoctorStatus::Fail));

        match previous {
            Some(value) => std::env::set_var("ADL_HOME", value),
            None => std::env::remove_var("ADL_HOME"),
        }
    }

    #[test]
    fn portable_project_doctor_fails_missing_required_fields_and_bad_enums() {
        let root = temp_root("invalid-fields");
        let adl_home = write_adl_home(&root);
        let project_root = root.join("spec");
        fs::create_dir_all(&project_root).expect("project root");
        let bad = serde_json::json!({
            "schema_version": "wrong",
            "project_id": "",
            "profile": "unknown",
            "tooling_ref": "agent-design-language@v0.91.5",
            "min_adl_version": "99.0.0",
            "prompt_template_registry": "docs/templates/prompts/current.json",
            "issue_tracker": {"provider": "gitlab", "repo": "owner/"},
            "tooling_discovery": {"env": "WRONG", "failure_mode": "guess"},
            "state_policy": {
                "issue_authority": "mystery",
                "cards_location": "nowhere",
                "worktree_location": "somewhere",
                "tracked_public_evidence": true,
                "local_adl_state_ignored": true
            },
            "validation_profile": "bad",
            "artifact_policy": {
                "forbid_absolute_host_paths": false,
                "public_outputs": [1],
                "private_outputs": []
            }
        });
        let project = write_project(&project_root, &bad.to_string());

        let report = run_portable_project_doctor(&project, Some(&adl_home)).expect("doctor");
        assert_eq!(report.status, DoctorStatus::Fail);
        assert!(report
            .checks
            .iter()
            .any(|c| c.name == "profile" && c.status == DoctorStatus::Fail));
        assert!(report
            .checks
            .iter()
            .any(|c| c.name == "issue_tracker.repo" && c.status == DoctorStatus::Fail));
        assert!(report
            .checks
            .iter()
            .any(|c| c.name == "artifact_policy.forbid_absolute_host_paths"
                && c.status == DoctorStatus::Fail));
    }

    #[test]
    fn portable_project_doctor_fails_invalid_adl_checkout_and_agents_contract() {
        let root = temp_root("bad-adl");
        let project_root = root.join("demo");
        fs::create_dir_all(&project_root).expect("project root");
        fs::write(
            project_root.join("AGENTS.md"),
            "No portable lifecycle here.\n",
        )
        .expect("agents");
        let project = project_root.join("adl_project.json");
        fs::write(&project, valid_project_json()).expect("project");

        let report =
            run_portable_project_doctor(&project, Some(&root.join("missing-adl"))).expect("doctor");
        assert_eq!(report.status, DoctorStatus::Fail);
        assert!(report
            .checks
            .iter()
            .any(|c| c.name == "adl_home.path" && c.status == DoctorStatus::Fail));
        assert!(report
            .checks
            .iter()
            .any(|c| c.name == "AGENTS.md" && c.status == DoctorStatus::Fail));

        let stub_home = root.join("stub-adl");
        fs::create_dir_all(stub_home.join("adl/tools")).expect("stub tools");
        fs::create_dir_all(stub_home.join("docs/templates/prompts")).expect("stub prompts");
        fs::write(
            stub_home.join("docs/templates/prompts/current.json"),
            "{}\n",
        )
        .expect("stub registry");
        let pr_sh = stub_home.join("adl/tools/pr.sh");
        fs::write(&pr_sh, "#!/usr/bin/env bash\n").expect("stub pr");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&pr_sh).expect("metadata").permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&pr_sh, perms).expect("chmod");
        }
        let stub_report = run_portable_project_doctor(&project, Some(&stub_home)).expect("doctor");
        assert_eq!(stub_report.status, DoctorStatus::Fail);
        assert!(stub_report
            .checks
            .iter()
            .any(|c| c.name == "adl_home.cargo_manifest" && c.status == DoctorStatus::Fail));
        assert!(stub_report
            .checks
            .iter()
            .any(|c| c.name == "adl_home.git_origin" && c.status == DoctorStatus::Fail));
    }
}

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Serialize;

const RUNTIME_ENVIRONMENT_SCHEMA_VERSION: &str = "runtime_environment.v1";
const DEFAULT_RUNTIME_ROOT_NAME: &str = ".adl";
const DEFAULT_RUNS_DIR_NAME: &str = "runs";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimePathSource {
    RepoDefault,
    EnvOverride,
}

impl RuntimePathSource {
    fn as_str(&self) -> &'static str {
        match self {
            Self::RepoDefault => "repo_default",
            Self::EnvOverride => "env_override",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEnvironment {
    repo_root: PathBuf,
    runtime_root: PathBuf,
    runtime_root_source: RuntimePathSource,
    runs_root: PathBuf,
    runs_root_source: RuntimePathSource,
}

#[derive(Debug, Clone, Serialize)]
struct RuntimeEnvironmentMarker {
    schema_version: &'static str,
    repo_root: &'static str,
    runtime_root: String,
    runtime_root_source: &'static str,
    runs_root: String,
    runs_root_source: &'static str,
}

impl RuntimeEnvironment {
    pub fn current() -> Result<Self> {
        let repo_root = repo_root_from_manifest()?;
        let (runtime_root, runtime_root_source) = runtime_root_for_repo(&repo_root);
        let (runs_root, runs_root_source) = runs_root_for_runtime(&runtime_root);
        Ok(Self {
            repo_root,
            runtime_root,
            runtime_root_source,
            runs_root,
            runs_root_source,
        })
    }

    pub fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    pub fn runtime_root(&self) -> &Path {
        &self.runtime_root
    }

    pub fn runtime_root_source(&self) -> &RuntimePathSource {
        &self.runtime_root_source
    }

    pub fn runs_root(&self) -> &Path {
        &self.runs_root
    }

    pub fn runs_root_source(&self) -> &RuntimePathSource {
        &self.runs_root_source
    }

    pub fn marker_path(&self) -> PathBuf {
        self.runtime_root.join("runtime_environment.json")
    }

    pub fn ensure_layout(&self) -> Result<()> {
        std::fs::create_dir_all(&self.runtime_root).with_context(|| {
            format!(
                "failed to create runtime root '{}'",
                self.runtime_root.display()
            )
        })?;
        std::fs::create_dir_all(&self.runs_root).with_context(|| {
            format!("failed to create runs root '{}'", self.runs_root.display())
        })?;
        self.write_marker()
    }

    pub fn write_marker(&self) -> Result<()> {
        let marker = RuntimeEnvironmentMarker {
            schema_version: RUNTIME_ENVIRONMENT_SCHEMA_VERSION,
            repo_root: ".",
            runtime_root: self.sanitized_path_label(&self.runtime_root, "external_runtime_root"),
            runtime_root_source: self.runtime_root_source.as_str(),
            runs_root: self.sanitized_path_label(&self.runs_root, "external_runs_root"),
            runs_root_source: self.runs_root_source.as_str(),
        };
        let bytes = serde_json::to_vec_pretty(&marker).context("serialize runtime marker")?;
        atomic_write(&self.marker_path(), &bytes)
    }

    fn sanitized_path_label(&self, path: &Path, external_label: &str) -> String {
        if let Ok(relative) = path.strip_prefix(&self.repo_root) {
            let relative_str = relative.display().to_string();
            if relative_str.is_empty() {
                ".".to_string()
            } else {
                relative_str
            }
        } else {
            external_label.to_string()
        }
    }
}

fn repo_root_from_manifest() -> Result<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest
        .parent()
        .context("failed to derive repo root from CARGO_MANIFEST_DIR")?;
    Ok(repo_root.to_path_buf())
}

fn runtime_root_for_repo(repo_root: &Path) -> (PathBuf, RuntimePathSource) {
    match trimmed_env_path("ADL_RUNTIME_ROOT") {
        Some(value) => (PathBuf::from(value), RuntimePathSource::EnvOverride),
        None => (
            repo_root.join(DEFAULT_RUNTIME_ROOT_NAME),
            RuntimePathSource::RepoDefault,
        ),
    }
}

fn runs_root_for_runtime(runtime_root: &Path) -> (PathBuf, RuntimePathSource) {
    match trimmed_env_path("ADL_RUNS_ROOT") {
        Some(value) => (PathBuf::from(value), RuntimePathSource::EnvOverride),
        None => (
            runtime_root.join(DEFAULT_RUNS_DIR_NAME),
            RuntimePathSource::RepoDefault,
        ),
    }
}

fn trimmed_env_path(key: &str) -> Option<String> {
    let value = std::env::var_os(key)?;
    let trimmed = value.to_string_lossy().trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .context("runtime environment marker path has no parent")?;
    std::fs::create_dir_all(parent)
        .with_context(|| format!("failed to create marker parent '{}'", parent.display()))?;
    let tmp_path = parent.join(format!(
        ".{}.{}.tmp",
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("runtime-environment"),
        std::process::id()
    ));
    std::fs::write(&tmp_path, bytes)
        .with_context(|| format!("failed to write marker temp file '{}'", tmp_path.display()))?;
    std::fs::rename(&tmp_path, path).with_context(|| {
        format!(
            "failed to atomically move runtime marker '{}' -> '{}'",
            tmp_path.display(),
            path.display()
        )
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EnvGuard {
        key: &'static str,
        prior: Option<std::ffi::OsString>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let prior = std::env::var_os(key);
            unsafe {
                std::env::set_var(key, value);
            }
            Self { key, prior }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.prior {
                unsafe {
                    std::env::set_var(self.key, value);
                }
            } else {
                unsafe {
                    std::env::remove_var(self.key);
                }
            }
        }
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "adl-runtime-env-{label}-pid{}-{n}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("create temp dir");
        path
    }

    #[test]
    fn runtime_environment_defaults_to_repo_local_roots() {
        let _runtime_guard = EnvGuard::set("ADL_RUNTIME_ROOT", "");
        let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", "");

        let env = RuntimeEnvironment::current().expect("runtime environment");
        assert_eq!(
            env.runtime_root()
                .file_name()
                .and_then(|name| name.to_str()),
            Some(".adl")
        );
        assert!(env.runs_root().ends_with(".adl/runs"));
        assert_eq!(env.runtime_root_source(), &RuntimePathSource::RepoDefault);
        assert_eq!(env.runs_root_source(), &RuntimePathSource::RepoDefault);
    }

    #[test]
    fn runtime_environment_respects_env_overrides_without_leaking_absolute_paths_in_marker() {
        let runtime_root = unique_temp_dir("runtime-root");
        let runs_root = unique_temp_dir("runs-root");
        let _runtime_guard = EnvGuard::set("ADL_RUNTIME_ROOT", &runtime_root.to_string_lossy());
        let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

        let env = RuntimeEnvironment::current().expect("runtime environment");
        env.ensure_layout().expect("layout");

        assert_eq!(env.runtime_root(), runtime_root.as_path());
        assert_eq!(env.runs_root(), runs_root.as_path());
        assert_eq!(env.runtime_root_source(), &RuntimePathSource::EnvOverride);
        assert_eq!(env.runs_root_source(), &RuntimePathSource::EnvOverride);

        let marker = std::fs::read_to_string(env.marker_path()).expect("marker");
        assert!(marker.contains("\"runtime_root\": \"external_runtime_root\""));
        assert!(marker.contains("\"runs_root\": \"external_runs_root\""));
        assert!(!marker.contains(&runtime_root.to_string_lossy().to_string()));
        assert!(!marker.contains(&runs_root.to_string_lossy().to_string()));
    }
}

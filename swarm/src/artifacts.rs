use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde::Serialize;

pub const ARTIFACT_MODEL_VERSION: u32 = 1;

#[derive(Debug, Clone)]
pub struct RunArtifactPaths {
    run_id: String,
    runs_root: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
struct ArtifactModelMarker {
    artifact_model_version: u32,
}

impl RunArtifactPaths {
    pub fn for_run(run_id: &str) -> Result<Self> {
        let run_id = run_id.trim();
        if run_id.is_empty() {
            return Err(anyhow!("run_id must not be empty for artifact paths"));
        }
        Ok(Self {
            run_id: run_id.to_string(),
            runs_root: runs_root()?,
        })
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn runs_root(&self) -> &Path {
        &self.runs_root
    }

    pub fn run_dir(&self) -> PathBuf {
        self.runs_root.join(&self.run_id)
    }

    pub fn run_json(&self) -> PathBuf {
        self.run_dir().join("run.json")
    }

    pub fn steps_json(&self) -> PathBuf {
        self.run_dir().join("steps.json")
    }

    pub fn pause_state_json(&self) -> PathBuf {
        self.run_dir().join("pause_state.json")
    }

    pub fn run_summary_json(&self) -> PathBuf {
        self.run_dir().join("run_summary.json")
    }

    pub fn run_status_json(&self) -> PathBuf {
        self.run_dir().join("run_status.json")
    }

    pub fn outputs_dir(&self) -> PathBuf {
        self.run_dir().join("outputs")
    }

    pub fn logs_dir(&self) -> PathBuf {
        self.run_dir().join("logs")
    }

    pub fn learning_dir(&self) -> PathBuf {
        self.run_dir().join("learning")
    }

    pub fn scores_json(&self) -> PathBuf {
        self.learning_dir().join("scores.json")
    }

    pub fn suggestions_json(&self) -> PathBuf {
        self.learning_dir().join("suggestions.json")
    }

    pub fn overlays_dir(&self) -> PathBuf {
        self.learning_dir().join("overlays")
    }

    pub fn meta_dir(&self) -> PathBuf {
        self.run_dir().join("meta")
    }

    pub fn artifact_model_marker_json(&self) -> PathBuf {
        self.meta_dir().join("ARTIFACT_MODEL.json")
    }

    pub fn ensure_layout(&self) -> Result<()> {
        let run_dir = self.run_dir();
        std::fs::create_dir_all(&run_dir).with_context(|| {
            format!("failed to create run artifact dir '{}'", run_dir.display())
        })?;
        std::fs::create_dir_all(self.outputs_dir())
            .with_context(|| "failed to create outputs artifact dir".to_string())?;
        std::fs::create_dir_all(self.logs_dir())
            .with_context(|| "failed to create logs artifact dir".to_string())?;
        std::fs::create_dir_all(self.overlays_dir())
            .with_context(|| "failed to create overlays artifact dir".to_string())?;
        std::fs::create_dir_all(self.meta_dir())
            .with_context(|| "failed to create meta artifact dir".to_string())?;
        Ok(())
    }

    pub fn write_model_marker(&self) -> Result<()> {
        let marker = ArtifactModelMarker {
            artifact_model_version: ARTIFACT_MODEL_VERSION,
        };
        let bytes =
            serde_json::to_vec_pretty(&marker).context("serialize artifact model marker")?;
        atomic_write(&self.artifact_model_marker_json(), &bytes)
    }
}

pub fn runs_root() -> Result<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest
        .parent()
        .context("failed to derive repo root from CARGO_MANIFEST_DIR")?;
    Ok(repo_root.join(".adl").join("runs"))
}

pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    // Best-effort atomic write strategy:
    // 1) write temp file in the same directory as the target
    // 2) rename temp -> target
    //
    // Same-directory rename is atomic on common local filesystems, but full
    // crash-safety/fsync semantics are platform/filesystem dependent.
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("artifact path has no parent: '{}'", path.display()))?;
    std::fs::create_dir_all(parent)
        .with_context(|| format!("failed to create artifact parent '{}'", parent.display()))?;

    let tmp_name = format!(
        ".{}.{}.tmp",
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("artifact"),
        std::process::id()
    );
    let tmp_path = parent.join(tmp_name);
    std::fs::write(&tmp_path, bytes)
        .with_context(|| format!("failed to write temp artifact '{}'", tmp_path.display()))?;
    std::fs::rename(&tmp_path, path).with_context(|| {
        format!(
            "failed to atomically move artifact '{}' -> '{}'",
            tmp_path.display(),
            path.display()
        )
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_paths_are_deterministic_and_timestamp_free() {
        let paths = RunArtifactPaths::for_run("demo-run-1").expect("paths");
        assert!(paths.run_dir().ends_with(".adl/runs/demo-run-1"));
        assert!(paths.run_json().ends_with(".adl/runs/demo-run-1/run.json"));
        assert!(paths
            .run_status_json()
            .ends_with(".adl/runs/demo-run-1/run_status.json"));
        assert!(paths
            .artifact_model_marker_json()
            .ends_with(".adl/runs/demo-run-1/meta/ARTIFACT_MODEL.json"));
        let as_string = paths.run_dir().display().to_string();
        assert!(
            !as_string.contains("T") && !as_string.contains("Z"),
            "path should not embed timestamp-like content: {as_string}"
        );
    }

    #[test]
    fn ensure_layout_creates_reserved_learning_and_meta_subtrees() {
        let run_id = format!("artifact-layout-{}", std::process::id());
        let paths = RunArtifactPaths::for_run(&run_id).expect("paths");
        paths.ensure_layout().expect("layout");
        paths.write_model_marker().expect("marker");

        assert!(paths.outputs_dir().is_dir());
        assert!(paths.logs_dir().is_dir());
        assert!(paths.learning_dir().is_dir());
        assert!(paths.overlays_dir().is_dir());
        assert!(paths.meta_dir().is_dir());
        assert!(paths.artifact_model_marker_json().is_file());

        let _ = std::fs::remove_dir_all(paths.run_dir());
    }
    #[test]
    fn for_run_rejects_empty_or_whitespace_run_id() {
        let err = RunArtifactPaths::for_run("   ").expect_err("whitespace run_id should fail");
        assert!(
            err.to_string().contains("run_id must not be empty"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn atomic_write_requires_parent_path() {
        let err = atomic_write(Path::new("/"), b"x").expect_err("path without parent should fail");
        assert!(
            err.to_string().contains("artifact path has no parent"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn atomic_write_overwrites_existing_file_deterministically() {
        let run_id = format!("artifact-atomic-overwrite-{}", std::process::id());
        let paths = RunArtifactPaths::for_run(&run_id).expect("paths");
        let target = paths.logs_dir().join("atomic-write.txt");

        atomic_write(&target, b"one").expect("first write");
        atomic_write(&target, b"two").expect("overwrite write");
        let actual = std::fs::read_to_string(&target).expect("read back");
        assert_eq!(actual, "two");

        let _ = std::fs::remove_dir_all(paths.run_dir());
    }

    #[test]
    fn write_model_marker_contains_expected_version_only() {
        let run_id = format!("artifact-marker-{}", std::process::id());
        let paths = RunArtifactPaths::for_run(&run_id).expect("paths");
        paths.ensure_layout().expect("layout");
        paths.write_model_marker().expect("marker");

        let raw = std::fs::read_to_string(paths.artifact_model_marker_json())
            .expect("marker should be readable");
        let json: serde_json::Value = serde_json::from_str(&raw).expect("valid marker json");
        assert_eq!(
            json,
            serde_json::json!({ "artifact_model_version": ARTIFACT_MODEL_VERSION })
        );

        let _ = std::fs::remove_dir_all(paths.run_dir());
    }
}

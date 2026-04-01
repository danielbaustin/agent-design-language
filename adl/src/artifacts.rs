use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde::Serialize;

pub const ARTIFACT_MODEL_VERSION: u32 = 1;

pub fn validate_run_id_path_segment(run_id: &str) -> Result<String> {
    let trimmed = run_id.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("run_id must not be empty for artifact paths"));
    }
    if trimmed == "." || trimmed == ".." {
        return Err(anyhow!(
            "run_id must be a safe path segment, not '.' or '..'"
        ));
    }
    if trimmed.contains('/') || trimmed.contains('\\') {
        return Err(anyhow!(
            "run_id must be a safe path segment and must not contain path separators"
        ));
    }
    if trimmed.contains(':') {
        return Err(anyhow!(
            "run_id must be a safe path segment and must not contain drive-like ':' prefixes"
        ));
    }
    Ok(trimmed.to_string())
}

/// Canonical run artifact path builder.
///
/// Produces deterministic, timestamp-free paths under `.adl/runs/<run_id>/`.
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
    /// Construct deterministic artifact paths for a run id.
    pub fn for_run(run_id: &str) -> Result<Self> {
        Self::for_run_in_root(run_id, runs_root()?)
    }

    /// Construct deterministic artifact paths for a run id under an explicit runs root.
    pub fn for_run_in_root(run_id: &str, runs_root: impl Into<PathBuf>) -> Result<Self> {
        let run_id = validate_run_id_path_segment(run_id)?;
        Ok(Self {
            run_id,
            runs_root: runs_root.into(),
        })
    }

    /// Run identifier associated with this path set.
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    /// Root directory containing all run artifacts.
    pub fn runs_root(&self) -> &Path {
        &self.runs_root
    }

    /// Run-scoped directory path.
    pub fn run_dir(&self) -> PathBuf {
        self.runs_root.join(&self.run_id)
    }

    /// Canonical `run.json` path.
    pub fn run_json(&self) -> PathBuf {
        self.run_dir().join("run.json")
    }

    /// Canonical `steps.json` path.
    pub fn steps_json(&self) -> PathBuf {
        self.run_dir().join("steps.json")
    }

    /// Canonical pause-state artifact path.
    pub fn pause_state_json(&self) -> PathBuf {
        self.run_dir().join("pause_state.json")
    }

    /// Canonical run-summary artifact path.
    pub fn run_summary_json(&self) -> PathBuf {
        self.run_dir().join("run_summary.json")
    }

    /// Canonical run-status artifact path.
    pub fn run_status_json(&self) -> PathBuf {
        self.run_dir().join("run_status.json")
    }

    /// Output artifact directory.
    pub fn outputs_dir(&self) -> PathBuf {
        self.run_dir().join("outputs")
    }

    /// Logs artifact directory.
    pub fn logs_dir(&self) -> PathBuf {
        self.run_dir().join("logs")
    }

    /// Canonical activation-log artifact path.
    pub fn activation_log_json(&self) -> PathBuf {
        self.logs_dir().join("activation_log.json")
    }

    /// Canonical bounded cluster-groundwork artifact path.
    pub fn cluster_groundwork_json(&self) -> PathBuf {
        self.meta_dir().join("cluster_groundwork.json")
    }

    /// Learning artifact directory.
    pub fn learning_dir(&self) -> PathBuf {
        self.run_dir().join("learning")
    }

    /// Canonical integrated control-path proof directory.
    pub fn control_path_dir(&self) -> PathBuf {
        self.run_dir().join("control_path")
    }

    pub fn control_path_signals_json(&self) -> PathBuf {
        self.control_path_dir().join("signals.json")
    }

    pub fn control_path_candidate_selection_json(&self) -> PathBuf {
        self.control_path_dir().join("candidate_selection.json")
    }

    pub fn control_path_arbitration_json(&self) -> PathBuf {
        self.control_path_dir().join("arbitration.json")
    }

    pub fn control_path_execution_iterations_json(&self) -> PathBuf {
        self.control_path_dir().join("execution_iterations.json")
    }

    pub fn control_path_evaluation_json(&self) -> PathBuf {
        self.control_path_dir().join("evaluation.json")
    }

    pub fn control_path_reframing_json(&self) -> PathBuf {
        self.control_path_dir().join("reframing.json")
    }

    pub fn control_path_memory_json(&self) -> PathBuf {
        self.control_path_dir().join("memory.json")
    }

    pub fn control_path_freedom_gate_json(&self) -> PathBuf {
        self.control_path_dir().join("freedom_gate.json")
    }

    pub fn control_path_final_result_json(&self) -> PathBuf {
        self.control_path_dir().join("final_result.json")
    }

    pub fn control_path_summary_txt(&self) -> PathBuf {
        self.control_path_dir().join("summary.txt")
    }

    /// Learning scores artifact path.
    pub fn scores_json(&self) -> PathBuf {
        self.learning_dir().join("scores.json")
    }

    /// Learning suggestions artifact path.
    pub fn suggestions_json(&self) -> PathBuf {
        self.learning_dir().join("suggestions.json")
    }

    /// Bounded Adaptive Execution Engine decision artifact path.
    pub fn aee_decision_json(&self) -> PathBuf {
        self.learning_dir().join("aee_decision.json")
    }

    /// Cognitive arbitration artifact path for bounded v0.86 route selection.
    pub fn cognitive_arbitration_json(&self) -> PathBuf {
        self.learning_dir().join("cognitive_arbitration.v1.json")
    }

    /// Fast/slow path artifact path for bounded v0.86 route handoff.
    pub fn fast_slow_path_json(&self) -> PathBuf {
        self.learning_dir().join("fast_slow_path.v1.json")
    }

    /// Agency-selection artifact path for bounded v0.86 candidate generation and selection.
    pub fn agency_selection_json(&self) -> PathBuf {
        self.learning_dir().join("agency_selection.v1.json")
    }

    /// Bounded execution artifact path for v0.86 AEE-lite execution state.
    pub fn bounded_execution_json(&self) -> PathBuf {
        self.learning_dir().join("bounded_execution.v1.json")
    }

    /// Evaluation-signals artifact path for v0.86 bounded evaluation and termination.
    pub fn evaluation_signals_json(&self) -> PathBuf {
        self.learning_dir().join("evaluation_signals.v1.json")
    }

    /// Reframing artifact path for v0.86 bounded frame adequacy and reframing.
    pub fn reframing_json(&self) -> PathBuf {
        self.learning_dir().join("reframing.v1.json")
    }

    /// Freedom Gate artifact path for v0.86 bounded commitment decisions.
    pub fn freedom_gate_json(&self) -> PathBuf {
        self.learning_dir().join("freedom_gate.v1.json")
    }

    /// Memory-read artifact path for v0.86 bounded memory participation.
    pub fn memory_read_json(&self) -> PathBuf {
        self.learning_dir().join("memory_read.v1.json")
    }

    /// Memory-write artifact path for v0.86 bounded memory participation.
    pub fn memory_write_json(&self) -> PathBuf {
        self.learning_dir().join("memory_write.v1.json")
    }

    /// Affect state artifact path for bounded affect-guided adaptation.
    pub fn affect_state_json(&self) -> PathBuf {
        self.learning_dir().join("affect_state.v1.json")
    }

    /// Cognitive signals artifact path for bounded v0.86 signal emission.
    pub fn cognitive_signals_json(&self) -> PathBuf {
        self.learning_dir().join("cognitive_signals.v1.json")
    }

    /// Affect-linked reasoning graph artifact path.
    pub fn reasoning_graph_json(&self) -> PathBuf {
        self.learning_dir().join("reasoning_graph.v1.json")
    }

    /// Learning overlays directory.
    pub fn overlays_dir(&self) -> PathBuf {
        self.learning_dir().join("overlays")
    }

    /// Metadata artifact directory.
    pub fn meta_dir(&self) -> PathBuf {
        self.run_dir().join("meta")
    }

    /// Artifact model marker path.
    pub fn artifact_model_marker_json(&self) -> PathBuf {
        self.meta_dir().join("ARTIFACT_MODEL.json")
    }

    /// Ensure canonical directory layout exists for this run.
    pub fn ensure_layout(&self) -> Result<()> {
        let run_dir = self.run_dir();
        std::fs::create_dir_all(&run_dir).with_context(|| {
            format!("failed to create run artifact dir '{}'", run_dir.display())
        })?;
        std::fs::create_dir_all(self.outputs_dir())
            .with_context(|| "failed to create outputs artifact dir".to_string())?;
        std::fs::create_dir_all(self.logs_dir())
            .with_context(|| "failed to create logs artifact dir".to_string())?;
        std::fs::create_dir_all(self.learning_dir())
            .with_context(|| "failed to create learning artifact dir".to_string())?;
        std::fs::create_dir_all(self.control_path_dir())
            .with_context(|| "failed to create control_path artifact dir".to_string())?;
        std::fs::create_dir_all(self.overlays_dir())
            .with_context(|| "failed to create overlays artifact dir".to_string())?;
        std::fs::create_dir_all(self.meta_dir())
            .with_context(|| "failed to create meta artifact dir".to_string())?;
        Ok(())
    }

    /// Write the artifact model marker file atomically.
    pub fn write_model_marker(&self) -> Result<()> {
        let marker = ArtifactModelMarker {
            artifact_model_version: ARTIFACT_MODEL_VERSION,
        };
        let bytes =
            serde_json::to_vec_pretty(&marker).context("serialize artifact model marker")?;
        atomic_write(&self.artifact_model_marker_json(), &bytes)
    }
}

/// Resolve repository run-artifact root (`.adl/runs`).
pub fn runs_root() -> Result<PathBuf> {
    if let Some(override_root) = std::env::var_os("ADL_RUNS_ROOT") {
        let trimmed = override_root.to_string_lossy().trim().to_string();
        if !trimmed.is_empty() {
            return Ok(PathBuf::from(trimmed));
        }
    }
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest
        .parent()
        .context("failed to derive repo root from CARGO_MANIFEST_DIR")?;
    Ok(repo_root.join(".adl").join("runs"))
}

/// Atomically write bytes to a file using same-directory temp + rename.
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

    fn unique_temp_runs_root(label: &str) -> PathBuf {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "adl-artifacts-{label}-pid{}-{n}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).expect("create temp runs root");
        root
    }

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
        let runs_root = unique_temp_runs_root("layout");
        let paths = RunArtifactPaths::for_run_in_root(&run_id, &runs_root).expect("paths");
        paths.ensure_layout().expect("layout");
        paths.write_model_marker().expect("marker");

        assert!(paths.outputs_dir().is_dir());
        assert!(paths.logs_dir().is_dir());
        assert!(paths.learning_dir().is_dir());
        assert!(paths.control_path_dir().is_dir());
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
    fn for_run_rejects_traversal_separator_and_drive_like_run_ids() {
        for bad in ["..", "a/b", "a\\b", "/tmp/run", "C:\\run"] {
            let err = RunArtifactPaths::for_run(bad).expect_err("unsafe run_id should fail");
            assert!(
                err.to_string().contains("safe path segment"),
                "unexpected error for '{bad}': {err}"
            );
        }
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
        let runs_root = unique_temp_runs_root("overwrite");
        let paths = RunArtifactPaths::for_run_in_root(&run_id, &runs_root).expect("paths");
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
        let runs_root = unique_temp_runs_root("marker");
        let paths = RunArtifactPaths::for_run_in_root(&run_id, &runs_root).expect("paths");
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

    #[test]
    fn path_accessors_cover_all_canonical_artifact_locations() {
        let run_id = "artifact-path-accessors";
        let paths = RunArtifactPaths::for_run(run_id).expect("paths");
        assert_eq!(paths.run_id(), run_id);
        assert!(paths
            .steps_json()
            .ends_with(".adl/runs/artifact-path-accessors/steps.json"));
        assert!(paths
            .pause_state_json()
            .ends_with(".adl/runs/artifact-path-accessors/pause_state.json"));
        assert!(paths
            .run_summary_json()
            .ends_with(".adl/runs/artifact-path-accessors/run_summary.json"));
        assert!(paths
            .activation_log_json()
            .ends_with(".adl/runs/artifact-path-accessors/logs/activation_log.json"));
        assert!(paths
            .scores_json()
            .ends_with(".adl/runs/artifact-path-accessors/learning/scores.json"));
        assert!(paths
            .suggestions_json()
            .ends_with(".adl/runs/artifact-path-accessors/learning/suggestions.json"));
        assert!(paths
            .aee_decision_json()
            .ends_with(".adl/runs/artifact-path-accessors/learning/aee_decision.json"));
        assert!(paths
            .cognitive_arbitration_json()
            .ends_with(".adl/runs/artifact-path-accessors/learning/cognitive_arbitration.v1.json"));
        assert!(paths
            .fast_slow_path_json()
            .ends_with(".adl/runs/artifact-path-accessors/learning/fast_slow_path.v1.json"));
        assert!(paths
            .agency_selection_json()
            .ends_with(".adl/runs/artifact-path-accessors/learning/agency_selection.v1.json"));
        assert!(paths
            .bounded_execution_json()
            .ends_with(".adl/runs/artifact-path-accessors/learning/bounded_execution.v1.json"));
        assert!(paths
            .evaluation_signals_json()
            .ends_with(".adl/runs/artifact-path-accessors/learning/evaluation_signals.v1.json"));
        assert!(paths
            .affect_state_json()
            .ends_with(".adl/runs/artifact-path-accessors/learning/affect_state.v1.json"));
        assert!(paths
            .cognitive_signals_json()
            .ends_with(".adl/runs/artifact-path-accessors/learning/cognitive_signals.v1.json"));
        assert!(paths
            .reasoning_graph_json()
            .ends_with(".adl/runs/artifact-path-accessors/learning/reasoning_graph.v1.json"));
    }

    #[test]
    fn runs_root_points_to_repo_adl_runs_directory() {
        let root = runs_root().expect("runs_root");
        assert!(
            root.ends_with(".adl/runs"),
            "unexpected runs_root: {}",
            root.display()
        );
    }

    #[test]
    fn runs_root_accessor_matches_global_runs_root() {
        let paths = RunArtifactPaths::for_run("artifact-runs-root-accessor").expect("paths");
        assert_eq!(
            paths.runs_root().to_path_buf(),
            runs_root().expect("global runs_root")
        );
    }

    #[test]
    fn atomic_write_creates_nested_parent_directories() {
        let run_id = format!("artifact-parent-create-{}", std::process::id());
        let runs_root = unique_temp_runs_root("parent-create");
        let paths = RunArtifactPaths::for_run_in_root(&run_id, &runs_root).expect("paths");
        let nested = paths
            .run_dir()
            .join("nested")
            .join("dir")
            .join("artifact.json");

        atomic_write(&nested, br#"{"ok":true}"#).expect("atomic write with nested parent");
        let actual = std::fs::read_to_string(&nested).expect("read nested artifact");
        assert_eq!(actual, r#"{"ok":true}"#);

        let _ = std::fs::remove_dir_all(paths.run_dir());
    }

    #[test]
    fn atomic_write_fails_when_parent_is_a_file() {
        let run_id = format!("artifact-parent-file-{}", std::process::id());
        let runs_root = unique_temp_runs_root("parent-file");
        let paths = RunArtifactPaths::for_run_in_root(&run_id, &runs_root).expect("paths");
        let file_parent = paths.run_dir().join("not-a-dir");
        std::fs::create_dir_all(paths.run_dir()).expect("run dir exists");
        std::fs::write(&file_parent, b"x").expect("create file parent");
        let target = file_parent.join("child.txt");

        let err = atomic_write(&target, b"hello").expect_err("parent file should fail");
        assert!(
            err.to_string().contains("failed to create artifact parent"),
            "unexpected error: {err:#}"
        );

        let _ = std::fs::remove_dir_all(paths.run_dir());
    }
}

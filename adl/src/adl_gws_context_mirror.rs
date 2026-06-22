use crate::adl_gws_drive_sync::{
    sync_drive_file_with_transport, WorkspaceDriveFileSyncDisposition,
    WorkspaceDriveFileSyncRequest, WorkspaceDriveFileSyncResult, WorkspaceDriveSyncPolicy,
    WorkspaceDriveTransport,
};
use crate::adl_gws_native::{tracked_path, WorkspaceExecutionMode, WorkspaceSkipReason};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub const ADL_GWS_CONTEXT_MIRROR_REPORT_ARTIFACT_PATH: &str =
    ".adl/tmp/google_workspace_cms/adl_gws_context_mirror_report.json";
pub const ADL_GWS_CONTEXT_MIRROR_STAGING_DIR_ENV: &str = "ADL_GWS_CONTEXT_MIRROR_STAGING_DIR";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceRecursiveMirrorStatus {
    SeedOnly,
    RecursivePending,
    RecursiveLive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceContextMirrorConfig {
    pub repo_root: String,
    pub staging_dir: String,
    pub drive_root_folder_id: String,
    pub drive_seed_folder_id: String,
    pub recursive_sync_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceMilestoneTruthRecord {
    pub chatgpt_facing_current_milestone: String,
    pub planning_sequence: Vec<String>,
    pub v092_activation_blocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceContextMirrorReport {
    pub schema_version: &'static str,
    pub live_mode: WorkspaceExecutionMode,
    pub source_roots: Vec<String>,
    pub drive_root_folder_id: String,
    pub drive_seed_folder_id: String,
    pub files_considered: Vec<String>,
    pub files_created: Vec<String>,
    pub files_updated: Vec<String>,
    pub files_skipped: Vec<String>,
    pub verification_results: Vec<String>,
    pub milestone_truth: WorkspaceMilestoneTruthRecord,
    pub recursive_mirror_status: WorkspaceRecursiveMirrorStatus,
    pub sync_results: Vec<WorkspaceDriveFileSyncResult>,
    pub summary_lines: Vec<String>,
    pub skipped_reason: Option<WorkspaceSkipReason>,
    pub non_claims: Vec<&'static str>,
}

pub fn default_context_mirror_config() -> WorkspaceContextMirrorConfig {
    WorkspaceContextMirrorConfig {
        repo_root: tracked_path("").display().to_string(),
        staging_dir: default_context_mirror_staging_dir().display().to_string(),
        drive_root_folder_id: String::new(),
        drive_seed_folder_id: String::new(),
        recursive_sync_enabled: false,
    }
}

pub fn default_context_mirror_staging_dir() -> PathBuf {
    if let Ok(path) = std::env::var(ADL_GWS_CONTEXT_MIRROR_STAGING_DIR_ENV) {
        if !path.trim().is_empty() {
            return PathBuf::from(path);
        }
    }
    tracked_path(".adl/tmp/google_workspace_cms/generated_seed_files")
}

pub fn context_seed_file_names() -> Vec<&'static str> {
    vec![
        "READ_ME_FIRST_ADL_CURRENT_STATE.md",
        "ADL_GOOGLE_DRIVE_MIRROR_POLICY.md",
        "ADL_GOOGLE_DRIVE_SYNC_INDEX.md",
        "ADL_CURRENT_CONTEXT_BUNDLE_v0.91.6_TO_v0.92.md",
    ]
}

pub async fn run_workspace_context_mirror_with_transport<T: WorkspaceDriveTransport>(
    live_mode: WorkspaceExecutionMode,
    write_approval_present: bool,
    config: WorkspaceContextMirrorConfig,
    transport: &T,
) -> Result<WorkspaceContextMirrorReport> {
    let staging_dir = PathBuf::from(&config.staging_dir);
    let files = context_seed_file_names();
    let mut files_considered = Vec::new();
    let mut files_created = Vec::new();
    let mut files_updated = Vec::new();
    let mut files_skipped = Vec::new();
    let mut verification_results = Vec::new();
    let mut sync_results = Vec::new();
    let milestone_truth = read_milestone_truth(Path::new(&config.repo_root))?;
    let recursive_mirror_status = if config.recursive_sync_enabled {
        WorkspaceRecursiveMirrorStatus::RecursiveLive
    } else {
        WorkspaceRecursiveMirrorStatus::RecursivePending
    };

    if config.drive_root_folder_id.trim().is_empty()
        || config.drive_seed_folder_id.trim().is_empty()
    {
        for file_name in &files {
            files_considered.push((*file_name).to_string());
            files_skipped.push((*file_name).to_string());
            verification_results.push(format!(
                "{}: context mirror requires both drive_root_folder_id and drive_seed_folder_id",
                file_name
            ));
        }
        return Ok(WorkspaceContextMirrorReport {
            schema_version: "adl_gws_context_mirror.v1",
            live_mode,
            source_roots: vec![
                Path::new(&config.repo_root).join("docs").display().to_string(),
                Path::new(&config.repo_root)
                    .join(".adl/docs/TBD")
                    .display()
                    .to_string(),
                config.staging_dir.clone(),
            ],
            drive_root_folder_id: config.drive_root_folder_id,
            drive_seed_folder_id: config.drive_seed_folder_id,
            files_considered,
            files_created,
            files_updated,
            files_skipped,
            verification_results,
            milestone_truth,
            recursive_mirror_status,
            sync_results,
            summary_lines: vec![
                "Seed sync was skipped because the Drive root and seed folder bindings were not both configured.".to_string(),
                "Recursive mirror status remains pending until bounded Drive bindings are supplied.".to_string(),
            ],
            skipped_reason: Some(WorkspaceSkipReason::MissingBinding),
            non_claims: vec![
                "This context mirror does not make Google Workspace canonical repo truth.",
                "This context mirror does not claim recursive docs mirroring unless the report explicitly says recursive status is live.",
                "This context mirror does not authorize canonical tracked repo edits from Workspace state.",
            ],
        });
    }

    ensure_seed_folder_within_root(
        transport,
        &config.drive_root_folder_id,
        &config.drive_seed_folder_id,
    )
    .await?;

    for file_name in files {
        let source_path = staging_dir.join(file_name);
        files_considered.push(file_name.to_string());
        let request = WorkspaceDriveFileSyncRequest {
            source_file: source_path.display().to_string(),
            target: crate::adl_gws_native::WorkspaceScopeBinding {
                root_folder_id: config.drive_seed_folder_id.clone(),
                folder_path: vec![],
                file_name: Some(file_name.to_string()),
                file_id: None,
            },
            target_file_name: file_name.to_string(),
            mime_type: "text/markdown".to_string(),
            policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
        };
        let report = sync_drive_file_with_transport(
            live_mode.clone(),
            write_approval_present,
            request,
            transport,
        )
        .await?;
        match report.result.disposition {
            WorkspaceDriveFileSyncDisposition::Created => files_created.push(file_name.to_string()),
            WorkspaceDriveFileSyncDisposition::Updated => files_updated.push(file_name.to_string()),
            WorkspaceDriveFileSyncDisposition::Skipped => files_skipped.push(file_name.to_string()),
        }
        verification_results.push(format!(
            "{}: {}",
            file_name, report.result.verification_message
        ));
        sync_results.push(report.result);
    }
    let summary_lines = vec![
        format!(
            "Seed sync considered {} files and updated {} total targets.",
            files_considered.len(),
            files_created.len() + files_updated.len()
        ),
        format!(
            "ChatGPT-facing current milestone remains '{}', with truthful sequence {}.",
            milestone_truth.chatgpt_facing_current_milestone,
            milestone_truth.planning_sequence.join(" -> ")
        ),
        format!("Recursive mirror status is {:?}.", recursive_mirror_status),
    ];
    Ok(WorkspaceContextMirrorReport {
        schema_version: "adl_gws_context_mirror.v1",
        live_mode: live_mode.clone(),
        source_roots: vec![
            Path::new(&config.repo_root)
                .join("docs")
                .display()
                .to_string(),
            Path::new(&config.repo_root)
                .join(".adl/docs/TBD")
                .display()
                .to_string(),
            config.staging_dir.clone(),
        ],
        drive_root_folder_id: config.drive_root_folder_id,
        drive_seed_folder_id: config.drive_seed_folder_id,
        files_considered,
        files_created,
        files_updated,
        files_skipped,
        verification_results,
        milestone_truth,
        recursive_mirror_status,
        sync_results,
        summary_lines,
        skipped_reason: if matches!(live_mode, WorkspaceExecutionMode::DryRun) {
            Some(WorkspaceSkipReason::DryRunOnly)
        } else {
            None
        },
        non_claims: vec![
            "This context mirror does not make Google Workspace canonical repo truth.",
            "This context mirror does not claim recursive docs mirroring unless the report explicitly says recursive status is live.",
            "This context mirror does not authorize canonical tracked repo edits from Workspace state.",
        ],
    })
}

async fn ensure_seed_folder_within_root<T: WorkspaceDriveTransport>(
    transport: &T,
    root_folder_id: &str,
    seed_folder_id: &str,
) -> Result<()> {
    if root_folder_id == seed_folder_id {
        return Ok(());
    }

    let mut current_id = seed_folder_id.to_string();
    let mut visited = HashSet::new();
    loop {
        if !visited.insert(current_id.clone()) {
            bail!("seed folder binding parent chain loop detected");
        }
        let metadata = transport.read_file_metadata(&current_id).await?;
        if metadata
            .parent_ids
            .iter()
            .any(|parent| parent == root_folder_id)
        {
            return Ok(());
        }
        let Some(next_parent) = metadata.parent_ids.first() else {
            bail!("seed folder is not within the configured drive root");
        };
        current_id = next_parent.clone();
    }
}

pub async fn write_workspace_context_mirror_report(
    report_path: impl AsRef<Path>,
    report: &WorkspaceContextMirrorReport,
) -> Result<()> {
    let report_path = report_path.as_ref();
    if let Some(parent) = report_path.parent() {
        tokio::fs::create_dir_all(parent).await.with_context(|| {
            format!("create parent directories for '{}'", report_path.display())
        })?;
    }
    tokio::fs::write(report_path, serde_json::to_string_pretty(report)?)
        .await
        .with_context(|| format!("write '{}'", report_path.display()))
}

pub fn read_milestone_truth(repo_root: &Path) -> Result<WorkspaceMilestoneTruthRecord> {
    let readme = std::fs::read_to_string(repo_root.join("README.md"))
        .with_context(|| "read repo README".to_string())?;
    let v0917 = std::fs::read_to_string(repo_root.join("docs/milestones/v0.91.7/README.md"))
        .with_context(|| "read v0.91.7 README".to_string())?;
    let v092_ledger = std::fs::read_to_string(
        repo_root.join("docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md"),
    )
    .with_context(|| "read v0.92 activation ledger".to_string())?;

    let current = detect_current_milestone(&readme);
    let has_v0917 = v0917.contains("v0.91.7");
    let v092_blocked = v092_ledger.contains("activation remains blocked");
    Ok(WorkspaceMilestoneTruthRecord {
        chatgpt_facing_current_milestone: current,
        planning_sequence: if has_v0917 {
            vec![
                "v0.91.6".to_string(),
                "v0.91.7".to_string(),
                "v0.92".to_string(),
            ]
        } else {
            vec!["v0.91.6".to_string(), "v0.92".to_string()]
        },
        v092_activation_blocked: v092_blocked,
    })
}

fn detect_current_milestone(readme: &str) -> String {
    for milestone in ["v0.92", "v0.91.7", "v0.91.6"] {
        let active_patterns = [
            format!("Active milestone: {milestone}"),
            format!("Current milestone state: {milestone}"),
            format!("milestone-{milestone}"),
            format!("### {milestone} - "),
        ];
        if active_patterns
            .iter()
            .any(|pattern| readme.contains(pattern))
        {
            return milestone.to_string();
        }
    }
    "unknown".to_string()
}

#[cfg(test)]
mod tests {
    use super::{
        context_seed_file_names, default_context_mirror_config, detect_current_milestone,
        read_milestone_truth, run_workspace_context_mirror_with_transport,
        write_workspace_context_mirror_report, WorkspaceContextMirrorConfig,
    };
    use crate::adl_gws_drive_sync::WorkspaceDriveTransport;
    use crate::adl_gws_native::WorkspaceExecutionMode;
    use anyhow::Result;
    use async_trait::async_trait;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Default)]
    struct NoopTransport;

    #[async_trait]
    impl WorkspaceDriveTransport for NoopTransport {
        async fn list_children(
            &self,
            _parent_id: &str,
        ) -> Result<Vec<crate::adl_gws_native::WorkspaceFileRef>> {
            Ok(vec![])
        }
        async fn read_file_metadata(
            &self,
            file_id: &str,
        ) -> Result<crate::adl_gws_native::WorkspaceFileRef> {
            Ok(crate::adl_gws_native::WorkspaceFileRef {
                file_id: file_id.to_string(),
                name: "mock.md".to_string(),
                mime_type: "text/markdown".to_string(),
                parent_ids: if file_id == "seed" {
                    vec!["root".to_string()]
                } else {
                    vec!["seed".to_string()]
                },
                modified_time: Some("2026-06-21T23:00:00Z".to_string()),
                web_view_link: None,
            })
        }
        async fn create_folder(
            &self,
            parent_id: &str,
            name: &str,
        ) -> Result<crate::adl_gws_native::WorkspaceFileRef> {
            Ok(crate::adl_gws_native::WorkspaceFileRef {
                file_id: format!("{parent_id}-{name}"),
                name: name.to_string(),
                mime_type: crate::adl_gws_drive_sync::DRIVE_FOLDER_MIME_TYPE.to_string(),
                parent_ids: vec![parent_id.to_string()],
                modified_time: Some("2026-06-21T23:00:00Z".to_string()),
                web_view_link: None,
            })
        }
        async fn create_file(
            &self,
            parent_id: &str,
            name: &str,
            mime_type: &str,
            _bytes: &[u8],
        ) -> Result<crate::adl_gws_native::WorkspaceFileRef> {
            Ok(crate::adl_gws_native::WorkspaceFileRef {
                file_id: format!("{parent_id}-{name}"),
                name: name.to_string(),
                mime_type: mime_type.to_string(),
                parent_ids: vec![parent_id.to_string()],
                modified_time: Some("2026-06-21T23:00:00Z".to_string()),
                web_view_link: None,
            })
        }
        async fn update_file(
            &self,
            file_id: &str,
            name: &str,
            mime_type: &str,
            _bytes: &[u8],
        ) -> Result<crate::adl_gws_native::WorkspaceFileRef> {
            Ok(crate::adl_gws_native::WorkspaceFileRef {
                file_id: file_id.to_string(),
                name: name.to_string(),
                mime_type: mime_type.to_string(),
                parent_ids: vec!["seed".to_string()],
                modified_time: Some("2026-06-21T23:59:59Z".to_string()),
                web_view_link: None,
            })
        }
    }

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("valid time")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.json"))
    }

    #[test]
    fn default_context_mirror_config_uses_repo_local_staging_dir() {
        let config = default_context_mirror_config();
        assert_eq!(
            config.staging_dir,
            crate::adl_gws_native::tracked_path(
                ".adl/tmp/google_workspace_cms/generated_seed_files"
            )
            .display()
            .to_string()
        );
    }

    #[test]
    fn context_seed_file_names_match_expected_surface() {
        assert_eq!(context_seed_file_names().len(), 4);
        assert!(context_seed_file_names().contains(&"ADL_GOOGLE_DRIVE_SYNC_INDEX.md"));
    }

    #[test]
    fn milestone_truth_reads_current_repo_story() {
        let repo_root = crate::adl_gws_native::tracked_path("");
        let truth = read_milestone_truth(&repo_root).expect("milestone truth");
        assert_eq!(truth.chatgpt_facing_current_milestone, "v0.91.6");
        assert_eq!(truth.planning_sequence, vec!["v0.91.6", "v0.91.7", "v0.92"]);
        assert!(truth.v092_activation_blocked);
    }

    #[tokio::test]
    async fn context_mirror_dry_run_report_is_machine_readable() {
        let report_path = unique_temp_path("workspace-context-mirror-report");
        let report = run_workspace_context_mirror_with_transport(
            WorkspaceExecutionMode::DryRun,
            false,
            WorkspaceContextMirrorConfig {
                repo_root: crate::adl_gws_native::tracked_path("")
                    .display()
                    .to_string(),
                staging_dir: crate::adl_gws_native::tracked_path(
                    ".adl/tmp/google_workspace_cms/generated_seed_files",
                )
                .display()
                .to_string(),
                drive_root_folder_id: "root".to_string(),
                drive_seed_folder_id: "seed".to_string(),
                recursive_sync_enabled: false,
            },
            &NoopTransport,
        )
        .await
        .expect("context mirror report");
        write_workspace_context_mirror_report(&report_path, &report)
            .await
            .expect("write report");
        let body = tokio::fs::read_to_string(&report_path)
            .await
            .expect("read report");
        assert!(body.contains("adl_gws_context_mirror.v1"));
        assert!(body.contains("\"v0.91.6\""));
        tokio::fs::remove_file(&report_path)
            .await
            .expect("remove report");
    }

    #[test]
    fn milestone_detection_can_advance_past_v0916() {
        assert_eq!(
            detect_current_milestone("Active milestone: v0.91.7"),
            "v0.91.7"
        );
        assert_eq!(
            detect_current_milestone("Current milestone state: v0.92 planning"),
            "v0.92"
        );
    }
}

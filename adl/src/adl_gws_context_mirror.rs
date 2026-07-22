use crate::adl_gws_drive_sync::{
    sync_drive_file_with_transport, WorkspaceDriveFileSyncDisposition,
    WorkspaceDriveFileSyncRequest, WorkspaceDriveFileSyncResult, WorkspaceDriveSyncPolicy,
    WorkspaceDriveTransport,
};
use crate::adl_gws_native::{tracked_path, WorkspaceExecutionMode, WorkspaceSkipReason};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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
    pub files_unchanged: Vec<String>,
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

/// Regenerate the four bounded context seeds from the current checkout.
///
/// The generated directory is operational cache, never repository authority.
/// Generation completes before any Drive request so a failed generation cannot
/// upload a partially refreshed packet.
pub fn regenerate_context_seed_files(config: &WorkspaceContextMirrorConfig) -> Result<()> {
    let repo_root = Path::new(&config.repo_root);
    let staging_dir = Path::new(&config.staging_dir);
    let milestone = read_milestone_truth(repo_root)?;
    let recursive_files = recursive_markdown_files(repo_root)?;
    let sequence = milestone.planning_sequence.join(" -> ");

    let mut indexed_sources = Vec::with_capacity(recursive_files.len());
    for source in &recursive_files {
        let relative = source.strip_prefix(repo_root).with_context(|| {
            format!("seed source '{}' escaped repository root", source.display())
        })?;
        let bytes = std::fs::read(source)
            .with_context(|| format!("read seed source '{}'", source.display()))?;
        indexed_sources.push((
            relative.display().to_string(),
            format!("{:x}", Sha256::digest(bytes)),
        ));
    }
    let mut inventory_hasher = Sha256::new();
    for (path, digest) in &indexed_sources {
        inventory_hasher.update(path.as_bytes());
        inventory_hasher.update([0]);
        inventory_hasher.update(digest.as_bytes());
        inventory_hasher.update([b'\n']);
    }
    let inventory_digest = format!("{:x}", inventory_hasher.finalize());

    std::fs::create_dir_all(staging_dir)
        .with_context(|| format!("create seed staging directory '{}'", staging_dir.display()))?;

    let read_me = format!(
        "# READ ME FIRST: ADL Current State\n\nGenerated deterministically from repository inventory `{inventory_digest}`.\n\n\
         This folder is a verified Google Drive mirror of selected ADL repository context. \
         The repository remains source truth.\n\n\
         ## Source Truth\n\n- Repository root: `agent-design-language`\n- Source roots: `docs/`, `.adl/docs/TBD/`\n\
         - Drive root folder: `{}`\n- Seed folder: `{}`\n\n\
         ## Current Milestone Truth\n\n- Current milestone: `{}`\n- Planning sequence: `{sequence}`\n\
         - v0.92 activation blocked: `{}`\n\n\
         ## Start Here\n\n1. `READ_ME_FIRST_ADL_CURRENT_STATE.md`\n2. `ADL_GOOGLE_DRIVE_MIRROR_POLICY.md`\n\
         3. `ADL_GOOGLE_DRIVE_SYNC_INDEX.md`\n4. `ADL_CURRENT_CONTEXT_BUNDLE_v0.91.6_TO_v0.92.md`\n\n\
         The mirrored source trees contain {count} Markdown files. When Drive and the repository disagree, the repository wins.\n",
        config.drive_root_folder_id,
        config.drive_seed_folder_id,
        milestone.chatgpt_facing_current_milestone,
        milestone.v092_activation_blocked,
        count = indexed_sources.len(),
    );
    let policy = format!(
        "# ADL Google Drive Mirror Policy\n\nGenerated deterministically from repository inventory `{inventory_digest}`.\n\n\
         ## Authority\n\nThe ADL repository is canonical. Google Drive is a read-oriented context mirror.\n\n\
         ## Source Roots\n\nMirror regular Markdown files recursively from `docs/` and `.adl/docs/TBD/`, \
         preserving repository-relative paths. Symlinks and path escapes fail closed.\n\n\
         ## Write And Verification Contract\n\nA live success requires explicit write approval, an approved external credential source, \
         least-privilege Drive scopes, bounded root and seed folder IDs, and exact post-write metadata and byte readback. \
         Local staging, dry-run, fixture, partial seed sync, or metadata-only readback is not success.\n\n\
         ## Automation Contract\n\nArchive a run only after all four seed files and every selected recursive file verify exactly. \
         Keep one deduplicated actionable task visible for authentication, upload, listing, readback, parity, or recursion failure.\n\n\
         ## Non-Claims\n\nThis mirror does not authorize repository edits from Drive, broaden Drive permissions, \
         or claim v0.92 activation readiness.\n"
    );
    let mut index = format!(
        "# ADL Google Drive Sync Index\n\nGenerated deterministically from repository inventory `{inventory_digest}`.\n\n\
         ## Bound Folders\n\n- Drive root folder: `{}`\n- Seed folder: `{}`\n\n\
         ## Current Source Inventory\n\n- Current milestone: `{}`\n- Planning sequence: `{sequence}`\n\
         - Markdown files selected recursively: {}\n\n| Repository-relative path | SHA-256 |\n| --- | --- |\n",
        config.drive_root_folder_id,
        config.drive_seed_folder_id,
        milestone.chatgpt_facing_current_milestone,
        indexed_sources.len(),
    );
    for (path, digest) in &indexed_sources {
        index.push_str(&format!("| `{path}` | `{digest}` |\n"));
    }
    let bundle = format!(
        "# ADL Current Context Bundle: v0.91.6 to v0.92\n\nGenerated deterministically from repository inventory `{inventory_digest}`.\n\n\
         ## Current Context\n\nADL's current milestone is `{}`. The current planning sequence is `{sequence}`. \
         The repository's active milestone package and exact lifecycle evidence remain authoritative; this file is orientation only.\n\n\
         ## Current Entry Points\n\n- `README.md`\n- `docs/milestones/{}/README.md`\n\
         - `docs/milestones/{}/SPRINT_PLAN_{}.md`\n- `docs/milestones/{}/WP_ISSUE_WAVE_{}.yaml`\n\
         - `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`\n- `.adl/docs/TBD/`\n\n\
         ## Mirror Inventory\n\nThis packet was generated from the same checkout as {count} recursively selected Markdown files. \
         `ADL_GOOGLE_DRIVE_SYNC_INDEX.md` records their repository-relative paths and SHA-256 digests. \
         A run is successful only when the Drive report records exact content verification for this packet and all recursive files.\n",
        milestone.chatgpt_facing_current_milestone,
        milestone.chatgpt_facing_current_milestone,
        milestone.chatgpt_facing_current_milestone,
        milestone.chatgpt_facing_current_milestone,
        milestone.chatgpt_facing_current_milestone,
        milestone.chatgpt_facing_current_milestone,
        count = indexed_sources.len(),
    );

    for (name, contents) in [
        ("READ_ME_FIRST_ADL_CURRENT_STATE.md", read_me),
        ("ADL_GOOGLE_DRIVE_MIRROR_POLICY.md", policy),
        ("ADL_GOOGLE_DRIVE_SYNC_INDEX.md", index),
        ("ADL_CURRENT_CONTEXT_BUNDLE_v0.91.6_TO_v0.92.md", bundle),
    ] {
        let destination = staging_dir.join(name);
        let temporary = staging_dir.join(format!(".{name}.tmp"));
        std::fs::write(&temporary, contents)
            .with_context(|| format!("write temporary seed '{}'", temporary.display()))?;
        std::fs::rename(&temporary, &destination)
            .with_context(|| format!("publish generated seed '{}'", destination.display()))?;
    }
    Ok(())
}

fn recursive_mirror_status(
    config: &WorkspaceContextMirrorConfig,
) -> WorkspaceRecursiveMirrorStatus {
    if config.recursive_sync_enabled {
        WorkspaceRecursiveMirrorStatus::RecursivePending
    } else {
        WorkspaceRecursiveMirrorStatus::SeedOnly
    }
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
    let mut files_unchanged = Vec::new();
    let mut files_skipped = Vec::new();
    let mut verification_results = Vec::new();
    let mut sync_results = Vec::new();
    let milestone_truth = read_milestone_truth(Path::new(&config.repo_root))?;
    let mut recursive_mirror_status = recursive_mirror_status(&config);

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
            files_unchanged,
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

    if matches!(live_mode, WorkspaceExecutionMode::Execute) {
        ensure_seed_folder_within_root(
            transport,
            &config.drive_root_folder_id,
            &config.drive_seed_folder_id,
        )
        .await?;
    }

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
            WorkspaceDriveFileSyncDisposition::Unchanged => {
                files_unchanged.push(file_name.to_string())
            }
            WorkspaceDriveFileSyncDisposition::Skipped => files_skipped.push(file_name.to_string()),
        }
        verification_results.push(format!(
            "{}: {}",
            file_name, report.result.verification_message
        ));
        sync_results.push(report.result);
    }

    if config.recursive_sync_enabled && matches!(live_mode, WorkspaceExecutionMode::Execute) {
        let repo_root = Path::new(&config.repo_root);
        let recursive_files = recursive_markdown_files(repo_root)?;
        let recursive_result_count = recursive_files.len();
        for source_path in recursive_files {
            let relative = source_path.strip_prefix(repo_root).with_context(|| {
                format!(
                    "mirror source '{}' escaped repo root",
                    source_path.display()
                )
            })?;
            let file_name = relative
                .file_name()
                .and_then(|name| name.to_str())
                .context("recursive mirror file name is not UTF-8")?;
            let folder_path = relative
                .parent()
                .into_iter()
                .flat_map(Path::components)
                .map(|component| component.as_os_str().to_string_lossy().into_owned())
                .collect::<Vec<_>>();
            let display = relative.display().to_string();
            files_considered.push(display.clone());
            let request = WorkspaceDriveFileSyncRequest {
                source_file: source_path.display().to_string(),
                target: crate::adl_gws_native::WorkspaceScopeBinding {
                    root_folder_id: config.drive_root_folder_id.clone(),
                    folder_path,
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
                WorkspaceDriveFileSyncDisposition::Created => files_created.push(display.clone()),
                WorkspaceDriveFileSyncDisposition::Updated => files_updated.push(display.clone()),
                WorkspaceDriveFileSyncDisposition::Unchanged => {
                    files_unchanged.push(display.clone())
                }
                WorkspaceDriveFileSyncDisposition::Skipped => files_skipped.push(display.clone()),
            }
            verification_results.push(format!(
                "{}: {}",
                display, report.result.verification_message
            ));
            sync_results.push(report.result);
        }
        if recursive_result_count > 0
            && sync_results
                .iter()
                .rev()
                .take(recursive_result_count)
                .all(|result| result.verification_ok)
        {
            recursive_mirror_status = WorkspaceRecursiveMirrorStatus::RecursiveLive;
        }
    }
    let summary_lines = vec![
        format!(
            "Context mirror considered {} files: {} created, {} updated, {} unchanged, and {} skipped.",
            files_considered.len(),
            files_created.len(),
            files_updated.len(),
            files_unchanged.len(),
            files_skipped.len()
        ),
        format!(
            "ChatGPT-facing current milestone remains '{}', with truthful sequence {}.",
            milestone_truth.chatgpt_facing_current_milestone,
            milestone_truth.planning_sequence.join(" -> ")
        ),
        match recursive_mirror_status {
            WorkspaceRecursiveMirrorStatus::SeedOnly => {
                "Recursive mirror status is SeedOnly; this run performed only bounded seed-file sync."
                    .to_string()
            }
            WorkspaceRecursiveMirrorStatus::RecursivePending => {
                "Recursive mirror status is RecursivePending; this non-execute run did not claim live recursive Drive mutation."
                    .to_string()
            }
            WorkspaceRecursiveMirrorStatus::RecursiveLive => {
                "Recursive mirror status is RecursiveLive.".to_string()
            }
        },
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
        files_unchanged,
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

fn recursive_markdown_files(repo_root: &Path) -> Result<Vec<PathBuf>> {
    let repo_metadata = std::fs::symlink_metadata(repo_root)
        .with_context(|| format!("inspect repository root '{}'", repo_root.display()))?;
    if repo_metadata.file_type().is_symlink() {
        bail!(
            "recursive mirror refuses symlink repository root '{}'.",
            repo_root.display()
        );
    }
    let canonical_repo_root = std::fs::canonicalize(repo_root)
        .with_context(|| format!("canonicalize repository root '{}'", repo_root.display()))?;
    let mut files = Vec::new();
    for relative_root in [Path::new("docs"), Path::new(".adl/docs/TBD")] {
        let root = repo_root.join(relative_root);
        let metadata = std::fs::symlink_metadata(&root)
            .with_context(|| format!("inspect declared mirror root '{}'", root.display()))?;
        if metadata.file_type().is_symlink() {
            bail!(
                "recursive mirror refuses symlink source root '{}'",
                root.display()
            );
        }
        if !metadata.is_dir() {
            bail!(
                "declared mirror root is not a directory: '{}'",
                root.display()
            );
        }
        let canonical_root = std::fs::canonicalize(&root)
            .with_context(|| format!("canonicalize mirror root '{}'", root.display()))?;
        if !canonical_root.starts_with(&canonical_repo_root) {
            bail!(
                "declared mirror root escaped repository: '{}'",
                root.display()
            );
        }
        collect_markdown_files(&root, &mut files)?;
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn collect_markdown_files(directory: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(directory)
        .with_context(|| format!("read recursive mirror directory '{}'", directory.display()))?
    {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let path = entry.path();
        if file_type.is_symlink() {
            bail!("recursive mirror refuses symlink '{}'", path.display());
        }
        if file_type.is_dir() {
            collect_markdown_files(&path, files)?;
        } else if file_type.is_file()
            && path.extension().and_then(|extension| extension.to_str()) == Some("md")
        {
            files.push(path);
        }
    }
    Ok(())
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
    let v092_ledger = std::fs::read_to_string(
        repo_root.join("docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md"),
    )
    .with_context(|| "read v0.92 activation ledger".to_string())?;

    let current = detect_current_milestone(&readme);
    let v092_blocked = v092_ledger.contains("activation remains blocked");
    let planning_sequence = if current == "v0.92" {
        vec!["v0.92".to_string()]
    } else if current == "unknown" {
        vec![]
    } else {
        vec![current.clone(), "v0.92".to_string()]
    };
    Ok(WorkspaceMilestoneTruthRecord {
        chatgpt_facing_current_milestone: current,
        planning_sequence,
        v092_activation_blocked: v092_blocked,
    })
}

fn detect_current_milestone(readme: &str) -> String {
    for milestone in ["v0.92", "v0.91.8", "v0.91.7", "v0.91.6"] {
        let active_patterns = [
            format!("Active milestone: {milestone}"),
            format!("Current milestone state: {milestone}"),
            format!("### {milestone} - Active"),
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
        read_milestone_truth, recursive_mirror_status, run_workspace_context_mirror_with_transport,
        write_workspace_context_mirror_report, WorkspaceContextMirrorConfig,
    };
    use crate::adl_gws_drive_sync::{InMemoryDriveTransportForDemo, WorkspaceDriveTransport};
    use crate::adl_gws_native::WorkspaceExecutionMode;
    use crate::adl_gws_native::WorkspaceSkipReason;
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
        async fn read_file_bytes(&self, _file_id: &str) -> Result<Vec<u8>> {
            Ok(Vec::new())
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

    #[derive(Default)]
    struct BrokenParentTransport;

    #[async_trait]
    impl WorkspaceDriveTransport for BrokenParentTransport {
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
                name: "broken.md".to_string(),
                mime_type: "text/markdown".to_string(),
                parent_ids: vec![],
                modified_time: Some("2026-06-21T23:00:00Z".to_string()),
                web_view_link: None,
            })
        }

        async fn read_file_bytes(&self, _file_id: &str) -> Result<Vec<u8>> {
            Ok(Vec::new())
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
                parent_ids: vec![],
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
        assert!(["v0.91.6", "v0.91.7", "v0.91.8", "v0.92"]
            .contains(&truth.chatgpt_facing_current_milestone.as_str()));
        assert_eq!(
            truth.planning_sequence.first().map(String::as_str),
            Some(truth.chatgpt_facing_current_milestone.as_str())
        );
        assert_eq!(
            truth.planning_sequence.last().map(String::as_str),
            Some("v0.92")
        );
        assert!(truth
            .planning_sequence
            .contains(&truth.chatgpt_facing_current_milestone));
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
        assert!(body.contains(&report.milestone_truth.chatgpt_facing_current_milestone));
        tokio::fs::remove_file(&report_path)
            .await
            .expect("remove report");
    }

    #[tokio::test]
    async fn context_mirror_missing_bindings_reports_skip() {
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
                drive_root_folder_id: String::new(),
                drive_seed_folder_id: String::new(),
                recursive_sync_enabled: false,
            },
            &NoopTransport,
        )
        .await
        .expect("missing binding report");
        assert_eq!(
            report.skipped_reason,
            Some(WorkspaceSkipReason::MissingBinding)
        );
        assert_eq!(report.files_considered.len(), 4);
        assert_eq!(report.files_skipped.len(), 4);
    }

    #[tokio::test]
    async fn context_mirror_recursive_pending_status_is_reported_truthfully() {
        let staging_dir = std::env::temp_dir().join("adl-gws-context-mirror-recursive-live");
        tokio::fs::create_dir_all(&staging_dir)
            .await
            .expect("create staging dir");
        for file_name in context_seed_file_names() {
            tokio::fs::write(staging_dir.join(file_name), format!("# {file_name}\n"))
                .await
                .expect("write staged file");
        }
        let report = run_workspace_context_mirror_with_transport(
            WorkspaceExecutionMode::DryRun,
            false,
            WorkspaceContextMirrorConfig {
                repo_root: crate::adl_gws_native::tracked_path("")
                    .display()
                    .to_string(),
                staging_dir: staging_dir.display().to_string(),
                drive_root_folder_id: "seed".to_string(),
                drive_seed_folder_id: "seed".to_string(),
                recursive_sync_enabled: true,
            },
            &NoopTransport,
        )
        .await
        .expect("recursive live report");
        assert_eq!(
            report.recursive_mirror_status,
            super::WorkspaceRecursiveMirrorStatus::RecursivePending
        );
        assert!(report
            .summary_lines
            .iter()
            .any(|line| line.contains("did not claim live recursive")));
        for file_name in context_seed_file_names() {
            tokio::fs::remove_file(staging_dir.join(file_name))
                .await
                .expect("remove staged file");
        }
        tokio::fs::remove_dir(&staging_dir)
            .await
            .expect("remove staging dir");
    }

    #[tokio::test]
    async fn ensure_seed_folder_within_root_rejects_missing_parent_chain() {
        let error = super::ensure_seed_folder_within_root(&BrokenParentTransport, "root", "orphan")
            .await
            .expect_err("seed folder should not be within root");
        assert!(error
            .to_string()
            .contains("not within the configured drive root"));
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

    #[test]
    fn recursive_status_reflects_seed_only_vs_pending_truthfully() {
        let mut config = default_context_mirror_config();
        assert_eq!(
            recursive_mirror_status(&config),
            super::WorkspaceRecursiveMirrorStatus::SeedOnly
        );
        config.recursive_sync_enabled = true;
        assert_eq!(
            recursive_mirror_status(&config),
            super::WorkspaceRecursiveMirrorStatus::RecursivePending
        );
    }

    #[tokio::test]
    async fn execute_mode_recursively_mirrors_markdown_with_verified_content() {
        let repo_root = unique_temp_path("context-mirror-recursive-repo");
        let staging_dir = repo_root.join("staging");
        tokio::fs::create_dir_all(repo_root.join("docs/nested"))
            .await
            .expect("create docs tree");
        tokio::fs::create_dir_all(repo_root.join(".adl/docs/TBD"))
            .await
            .expect("create TBD tree");
        tokio::fs::create_dir_all(repo_root.join("docs/milestones/v0.91.7"))
            .await
            .expect("create milestone tree");
        tokio::fs::create_dir_all(repo_root.join("docs/milestones/v0.92"))
            .await
            .expect("create v0.92 tree");
        tokio::fs::create_dir_all(&staging_dir)
            .await
            .expect("create staging tree");
        tokio::fs::write(repo_root.join("README.md"), "Active milestone: v0.91.7\n")
            .await
            .expect("write README");
        tokio::fs::write(
            repo_root.join("docs/milestones/v0.91.7/README.md"),
            "# v0.91.7\n",
        )
        .await
        .expect("write milestone README");
        tokio::fs::write(
            repo_root.join("docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md"),
            "activation remains blocked\n",
        )
        .await
        .expect("write activation ledger");
        tokio::fs::write(repo_root.join("docs/nested/a.md"), "# A\n")
            .await
            .expect("write nested doc");
        tokio::fs::write(repo_root.join(".adl/docs/TBD/b.md"), "# B\n")
            .await
            .expect("write TBD doc");
        for file_name in context_seed_file_names() {
            tokio::fs::write(staging_dir.join(file_name), format!("# {file_name}\n"))
                .await
                .expect("write seed file");
        }

        let report = run_workspace_context_mirror_with_transport(
            WorkspaceExecutionMode::Execute,
            true,
            WorkspaceContextMirrorConfig {
                repo_root: repo_root.display().to_string(),
                staging_dir: staging_dir.display().to_string(),
                drive_root_folder_id: "demo-root".to_string(),
                drive_seed_folder_id: "demo-root".to_string(),
                recursive_sync_enabled: true,
            },
            &InMemoryDriveTransportForDemo::new(),
        )
        .await
        .expect("run recursive mirror");

        assert_eq!(
            report.recursive_mirror_status,
            super::WorkspaceRecursiveMirrorStatus::RecursiveLive
        );
        assert!(report
            .files_considered
            .contains(&"docs/nested/a.md".to_string()));
        assert!(report
            .files_considered
            .contains(&".adl/docs/TBD/b.md".to_string()));
        assert!(report
            .sync_results
            .iter()
            .all(|result| result.verification_ok));
        tokio::fs::remove_dir_all(repo_root)
            .await
            .expect("remove recursive mirror fixture");
    }

    #[cfg(unix)]
    #[test]
    fn recursive_mirror_rejects_a_symlink_declared_root() {
        use std::os::unix::fs::symlink;

        let repo_root = unique_temp_path("context-mirror-symlink-root");
        let outside = unique_temp_path("context-mirror-symlink-outside");
        std::fs::create_dir_all(repo_root.join(".adl/docs/TBD")).expect("create declared TBD root");
        std::fs::create_dir_all(&outside).expect("create outside directory");
        std::fs::write(outside.join("outside.md"), "# outside\n").expect("write outside file");
        symlink(&outside, repo_root.join("docs")).expect("link docs outside repo");

        let error = super::recursive_markdown_files(&repo_root)
            .expect_err("symlink source root must fail closed");
        assert!(error.to_string().contains("symlink source root"));
        std::fs::remove_dir_all(repo_root).expect("remove symlink repo fixture");
        std::fs::remove_dir_all(outside).expect("remove outside fixture");
    }
}

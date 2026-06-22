use adl::adl_gws_drive_sync::{
    sync_drive_file_with_transport, write_workspace_drive_sync_report,
    InMemoryDriveTransportForDemo, WorkspaceDriveFileSyncRequest, WorkspaceDriveSyncPolicy,
    ADL_GWS_NATIVE_DRIVE_SYNC_REPORT_ARTIFACT_PATH,
};
use adl::adl_gws_native::{
    parse_workspace_execution_mode_from_env, parse_workspace_write_approval_from_env, tracked_path,
    WorkspaceExecutionMode, WorkspaceScopeBinding,
};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

fn resolve_out_path(arg: Option<String>) -> PathBuf {
    arg.map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(ADL_GWS_NATIVE_DRIVE_SYNC_REPORT_ARTIFACT_PATH))
}

fn parse_arg(flag: &str) -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == flag {
            return args.next();
        }
    }
    None
}

fn seed_demo_source_path() -> PathBuf {
    tracked_path("docs/READ_ME_FIRST_ADL_CURRENT_STATE.md")
}

fn infer_mime_type(path: &Path) -> String {
    mime_guess2::from_path(path)
        .first_raw()
        .unwrap_or("application/octet-stream")
        .to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    let out_path = resolve_out_path(parse_arg("--out"));
    let source = parse_arg("--source")
        .map(PathBuf::from)
        .unwrap_or_else(seed_demo_source_path);
    let target_name = parse_arg("--target-name").unwrap_or_else(|| {
        source
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("READ_ME_FIRST_ADL_CURRENT_STATE.md")
            .to_string()
    });
    let root_folder_id = parse_arg("--root-folder").unwrap_or_else(|| "demo-root".to_string());
    let folder_path = parse_arg("--folder-path")
        .map(|value| {
            value
                .split('/')
                .filter(|segment| !segment.is_empty())
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec!["docs".to_string(), "seed".to_string()]);
    let transport = InMemoryDriveTransportForDemo::new();
    let report = sync_drive_file_with_transport(
        match parse_workspace_execution_mode_from_env() {
            WorkspaceExecutionMode::FixtureBacked => WorkspaceExecutionMode::DryRun,
            mode => mode,
        },
        parse_workspace_write_approval_from_env(),
        WorkspaceDriveFileSyncRequest {
            source_file: source.display().to_string(),
            target: WorkspaceScopeBinding {
                root_folder_id,
                folder_path,
                file_name: Some(target_name.clone()),
                file_id: None,
            },
            target_file_name: target_name,
            mime_type: infer_mime_type(&source),
            policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
        },
        &transport,
    )
    .await
    .with_context(|| format!("build native Drive sync report from '{}'", source.display()))?;
    write_workspace_drive_sync_report(&out_path, &report).await?;
    println!("{}", out_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::resolve_out_path;

    #[test]
    fn native_drive_sync_demo_uses_explicit_argument() {
        let path = resolve_out_path(Some("tmp/native-drive-sync.json".to_string()));
        assert_eq!(path, std::path::PathBuf::from("tmp/native-drive-sync.json"));
    }

    #[test]
    fn native_drive_sync_demo_defaults_to_artifact_path() {
        let path = resolve_out_path(None);
        assert_eq!(
            path,
            std::path::PathBuf::from(
                adl::adl_gws_drive_sync::ADL_GWS_NATIVE_DRIVE_SYNC_REPORT_ARTIFACT_PATH
            )
        );
    }
}

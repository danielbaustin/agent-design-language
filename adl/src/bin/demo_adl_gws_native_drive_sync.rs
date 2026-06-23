use adl::adl_gws_drive_sync::{
    sync_drive_file_with_transport, write_workspace_drive_sync_report,
    InMemoryDriveTransportForDemo, WorkspaceDriveFileSyncRequest, WorkspaceDriveSyncPolicy,
    WorkspaceDriveTransport, ADL_GWS_NATIVE_DRIVE_SYNC_REPORT_ARTIFACT_PATH,
};
use adl::adl_gws_native::{
    parse_workspace_execution_mode_from_env, parse_workspace_write_approval_from_env, tracked_path,
    WorkspaceExecutionMode, WorkspaceScopeBinding,
};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct DemoDriveSyncConfig {
    out_path: PathBuf,
    source: PathBuf,
    target_name: String,
    root_folder_id: String,
    folder_path: Vec<String>,
    live_mode: WorkspaceExecutionMode,
    write_approval_present: bool,
}

fn resolve_out_path(arg: Option<String>) -> PathBuf {
    arg.map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(ADL_GWS_NATIVE_DRIVE_SYNC_REPORT_ARTIFACT_PATH))
}

fn parse_arg(args: &[String], flag: &str) -> Option<String> {
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        if arg == flag {
            return args.next().cloned();
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

fn build_demo_config(
    args: &[String],
    live_mode: WorkspaceExecutionMode,
    write_approval_present: bool,
) -> DemoDriveSyncConfig {
    let out_path = resolve_out_path(parse_arg(args, "--out"));
    let source = parse_arg(args, "--source")
        .map(PathBuf::from)
        .unwrap_or_else(seed_demo_source_path);
    let target_name = parse_arg(args, "--target-name").unwrap_or_else(|| {
        source
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("READ_ME_FIRST_ADL_CURRENT_STATE.md")
            .to_string()
    });
    let root_folder_id =
        parse_arg(args, "--root-folder").unwrap_or_else(|| "demo-root".to_string());
    let folder_path = parse_arg(args, "--folder-path")
        .map(|value| {
            value
                .split('/')
                .filter(|segment| !segment.is_empty())
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec!["docs".to_string(), "seed".to_string()]);

    DemoDriveSyncConfig {
        out_path,
        source,
        target_name,
        root_folder_id,
        folder_path,
        live_mode,
        write_approval_present,
    }
}

async fn run_demo_with_transport<T: WorkspaceDriveTransport>(
    config: &DemoDriveSyncConfig,
    transport: &T,
) -> Result<PathBuf> {
    let report = sync_drive_file_with_transport(
        config.live_mode.clone(),
        config.write_approval_present,
        WorkspaceDriveFileSyncRequest {
            source_file: config.source.display().to_string(),
            target: WorkspaceScopeBinding {
                root_folder_id: config.root_folder_id.clone(),
                folder_path: config.folder_path.clone(),
                file_name: Some(config.target_name.clone()),
                file_id: None,
            },
            target_file_name: config.target_name.clone(),
            mime_type: infer_mime_type(&config.source),
            policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
        },
        transport,
    )
    .await
    .with_context(|| {
        format!(
            "build native Drive sync report from '{}'",
            config.source.display()
        )
    })?;
    write_workspace_drive_sync_report(&config.out_path, &report).await?;
    Ok(config.out_path.clone())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let config = build_demo_config(
        &args,
        match parse_workspace_execution_mode_from_env() {
            WorkspaceExecutionMode::FixtureBacked => WorkspaceExecutionMode::DryRun,
            mode => mode,
        },
        parse_workspace_write_approval_from_env(),
    );
    let transport = InMemoryDriveTransportForDemo::new();
    let out_path = run_demo_with_transport(&config, &transport).await?;
    println!("{}", out_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        build_demo_config, infer_mime_type, resolve_out_path, run_demo_with_transport,
        DemoDriveSyncConfig,
    };
    use adl::adl_gws_drive_sync::InMemoryDriveTransportForDemo;
    use adl::adl_gws_native::WorkspaceExecutionMode;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(prefix: &str, extension: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("valid time")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.{extension}"))
    }

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

    #[test]
    fn native_drive_sync_demo_builds_config_from_args() {
        let args = vec![
            "--source".to_string(),
            "docs/example.md".to_string(),
            "--target-name".to_string(),
            "mirror.md".to_string(),
            "--root-folder".to_string(),
            "seed-root".to_string(),
            "--folder-path".to_string(),
            "docs/seed".to_string(),
            "--out".to_string(),
            "tmp/demo.json".to_string(),
        ];
        let config = build_demo_config(&args, WorkspaceExecutionMode::Execute, true);
        assert_eq!(config.source, std::path::PathBuf::from("docs/example.md"));
        assert_eq!(config.target_name, "mirror.md");
        assert_eq!(config.root_folder_id, "seed-root");
        assert_eq!(config.folder_path, vec!["docs", "seed"]);
        assert_eq!(config.out_path, std::path::PathBuf::from("tmp/demo.json"));
        assert_eq!(config.live_mode, WorkspaceExecutionMode::Execute);
        assert!(config.write_approval_present);
    }

    #[test]
    fn native_drive_sync_demo_infers_markdown_mime_type() {
        assert_eq!(
            infer_mime_type(std::path::Path::new("state.md")),
            "text/markdown"
        );
    }

    #[tokio::test]
    async fn native_drive_sync_demo_runs_and_writes_report() {
        let source = unique_temp_path("native-drive-sync-source", "md");
        let out_path = unique_temp_path("native-drive-sync-report", "json");
        tokio::fs::write(&source, b"seed body")
            .await
            .expect("write source");
        let config = DemoDriveSyncConfig {
            out_path: out_path.clone(),
            source: source.clone(),
            target_name: "mirror.md".to_string(),
            root_folder_id: "demo-root".to_string(),
            folder_path: vec!["docs".to_string()],
            live_mode: WorkspaceExecutionMode::DryRun,
            write_approval_present: false,
        };
        let written = run_demo_with_transport(&config, &InMemoryDriveTransportForDemo::new())
            .await
            .expect("run demo");
        let body = tokio::fs::read_to_string(&written)
            .await
            .expect("read report");
        assert!(body.contains("adl_gws_native_drive_sync.v1"));
        tokio::fs::remove_file(&source)
            .await
            .expect("remove source");
        tokio::fs::remove_file(&out_path)
            .await
            .expect("remove report");
    }
}

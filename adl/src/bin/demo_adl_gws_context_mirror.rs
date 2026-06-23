use adl::adl_gws_context_mirror::{
    default_context_mirror_config, run_workspace_context_mirror_with_transport,
    write_workspace_context_mirror_report, WorkspaceContextMirrorConfig,
    ADL_GWS_CONTEXT_MIRROR_REPORT_ARTIFACT_PATH,
};
use adl::adl_gws_drive_sync::{InMemoryDriveTransportForDemo, WorkspaceDriveTransport};
use adl::adl_gws_native::{
    parse_workspace_execution_mode_from_env, parse_workspace_write_approval_from_env,
    WorkspaceExecutionMode,
};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct DemoContextMirrorRunConfig {
    out_path: PathBuf,
    mirror_config: WorkspaceContextMirrorConfig,
    live_mode: WorkspaceExecutionMode,
    write_approval_present: bool,
}

fn resolve_out_path(arg: Option<String>) -> PathBuf {
    arg.map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(ADL_GWS_CONTEXT_MIRROR_REPORT_ARTIFACT_PATH))
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

fn build_demo_config(
    args: &[String],
    live_mode: WorkspaceExecutionMode,
    write_approval_present: bool,
    recursive_sync_enabled: bool,
) -> DemoContextMirrorRunConfig {
    let out_path = resolve_out_path(parse_arg(args, "--out"));
    let mut mirror_config = default_context_mirror_config();
    if let Some(staging_dir) = parse_arg(args, "--staging-dir") {
        mirror_config.staging_dir = staging_dir;
    }
    if let Some(root_id) = parse_arg(args, "--drive-root-folder-id") {
        mirror_config.drive_root_folder_id = root_id;
    } else if mirror_config.drive_root_folder_id.is_empty() {
        mirror_config.drive_root_folder_id = "demo-root".to_string();
    }
    if let Some(seed_id) = parse_arg(args, "--drive-seed-folder-id") {
        mirror_config.drive_seed_folder_id = seed_id;
    } else if mirror_config.drive_seed_folder_id.is_empty() {
        mirror_config.drive_seed_folder_id = "demo-root".to_string();
    }
    if recursive_sync_enabled {
        mirror_config.recursive_sync_enabled = true;
    }

    DemoContextMirrorRunConfig {
        out_path,
        mirror_config,
        live_mode,
        write_approval_present,
    }
}

async fn run_demo_with_transport<T: WorkspaceDriveTransport>(
    config: &DemoContextMirrorRunConfig,
    transport: &T,
) -> Result<PathBuf> {
    let report = run_workspace_context_mirror_with_transport(
        config.live_mode.clone(),
        config.write_approval_present,
        config.mirror_config.clone(),
        transport,
    )
    .await?;
    write_workspace_context_mirror_report(&config.out_path, &report).await?;
    Ok(config.out_path.clone())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let recursive_sync_enabled = matches!(
        std::env::var("ADL_GWS_RECURSIVE_SYNC")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes" | "enabled"
    );
    let config = build_demo_config(
        &args,
        match parse_workspace_execution_mode_from_env() {
            WorkspaceExecutionMode::FixtureBacked => WorkspaceExecutionMode::DryRun,
            mode => mode,
        },
        parse_workspace_write_approval_from_env(),
        recursive_sync_enabled,
    );
    let transport = InMemoryDriveTransportForDemo::new();
    let out_path = run_demo_with_transport(&config, &transport).await?;
    println!("{}", out_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{build_demo_config, resolve_out_path, run_demo_with_transport};
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
    fn context_mirror_demo_uses_explicit_argument() {
        let path = resolve_out_path(Some("tmp/context-mirror.json".to_string()));
        assert_eq!(path, std::path::PathBuf::from("tmp/context-mirror.json"));
    }

    #[test]
    fn context_mirror_demo_defaults_to_artifact_path() {
        let path = resolve_out_path(None);
        assert_eq!(
            path,
            std::path::PathBuf::from(
                adl::adl_gws_context_mirror::ADL_GWS_CONTEXT_MIRROR_REPORT_ARTIFACT_PATH
            )
        );
    }

    #[test]
    fn context_mirror_demo_builds_config_from_args() {
        let args = vec![
            "--out".to_string(),
            "tmp/context.json".to_string(),
            "--staging-dir".to_string(),
            "tmp/staging".to_string(),
            "--drive-root-folder-id".to_string(),
            "root-1".to_string(),
            "--drive-seed-folder-id".to_string(),
            "seed-1".to_string(),
        ];
        let config = build_demo_config(&args, WorkspaceExecutionMode::Execute, true, true);
        assert_eq!(
            config.out_path,
            std::path::PathBuf::from("tmp/context.json")
        );
        assert_eq!(config.mirror_config.staging_dir, "tmp/staging");
        assert_eq!(config.mirror_config.drive_root_folder_id, "root-1");
        assert_eq!(config.mirror_config.drive_seed_folder_id, "seed-1");
        assert!(config.mirror_config.recursive_sync_enabled);
        assert_eq!(config.live_mode, WorkspaceExecutionMode::Execute);
        assert!(config.write_approval_present);
    }

    #[tokio::test]
    async fn context_mirror_demo_runs_and_writes_report() {
        let staging_dir = unique_temp_path("context-mirror-staging", "dir");
        tokio::fs::create_dir_all(&staging_dir)
            .await
            .expect("create staging dir");
        for file_name in adl::adl_gws_context_mirror::context_seed_file_names() {
            tokio::fs::write(staging_dir.join(file_name), format!("# {file_name}\n"))
                .await
                .expect("write staged file");
        }
        let out_path = unique_temp_path("context-mirror-report", "json");
        let args = vec![
            "--out".to_string(),
            out_path.display().to_string(),
            "--staging-dir".to_string(),
            staging_dir.display().to_string(),
            "--drive-root-folder-id".to_string(),
            "demo-root".to_string(),
            "--drive-seed-folder-id".to_string(),
            "demo-root".to_string(),
        ];
        let config = build_demo_config(&args, WorkspaceExecutionMode::DryRun, false, false);
        let written = run_demo_with_transport(&config, &InMemoryDriveTransportForDemo::new())
            .await
            .expect("run demo");
        let body = tokio::fs::read_to_string(&written)
            .await
            .expect("read report");
        assert!(body.contains("adl_gws_context_mirror.v1"));
        tokio::fs::remove_file(&out_path)
            .await
            .expect("remove report");
        for file_name in adl::adl_gws_context_mirror::context_seed_file_names() {
            tokio::fs::remove_file(staging_dir.join(file_name))
                .await
                .expect("remove staged file");
        }
        tokio::fs::remove_dir(&staging_dir)
            .await
            .expect("remove staging dir");
    }
}

use adl::adl_gws_context_mirror::{
    default_context_mirror_config, regenerate_context_seed_files,
    run_workspace_context_mirror_with_transport, write_workspace_context_mirror_report,
    WorkspaceContextMirrorConfig, ADL_GWS_CONTEXT_MIRROR_REPORT_ARTIFACT_PATH,
};
use adl::adl_gws_drive_sync::{
    InMemoryDriveTransportForDemo, NativeWorkspaceDriveTransport, WorkspaceDriveTransport,
};
use adl::adl_gws_native::{
    parse_workspace_execution_mode_from_env, parse_workspace_write_approval_from_env,
    DefaultWorkspaceAccessTokenProvider, WorkspaceExecutionMode,
};
use anyhow::{bail, Result};
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
    if let Some(repo_root) = parse_arg(args, "--repo-root") {
        mirror_config.repo_root = repo_root;
    }
    if let Some(staging_dir) = parse_arg(args, "--staging-dir") {
        mirror_config.staging_dir = staging_dir;
    }
    if let Some(root_id) = parse_arg(args, "--drive-root-folder-id") {
        mirror_config.drive_root_folder_id = root_id;
    } else if mirror_config.drive_root_folder_id.is_empty()
        && !matches!(live_mode, WorkspaceExecutionMode::Execute)
    {
        mirror_config.drive_root_folder_id = "demo-root".to_string();
    }
    if let Some(seed_id) = parse_arg(args, "--drive-seed-folder-id") {
        mirror_config.drive_seed_folder_id = seed_id;
    } else if mirror_config.drive_seed_folder_id.is_empty()
        && !matches!(live_mode, WorkspaceExecutionMode::Execute)
    {
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
    if let Err(error) = regenerate_context_seed_files(&config.mirror_config) {
        write_failure_report(&config.out_path, config, &error).await?;
        return Err(error);
    }
    let report = match run_workspace_context_mirror_with_transport(
        config.live_mode.clone(),
        config.write_approval_present,
        config.mirror_config.clone(),
        transport,
    )
    .await
    {
        Ok(report) => report,
        Err(error) => {
            write_failure_report(&config.out_path, config, &error).await?;
            return Err(error);
        }
    };
    write_workspace_context_mirror_report(&config.out_path, &report).await?;
    if matches!(config.live_mode, WorkspaceExecutionMode::Execute)
        && (report.skipped_reason.is_some()
            || report.sync_results.is_empty()
            || report
                .sync_results
                .iter()
                .any(|result| !result.verification_ok))
    {
        bail!("context mirror report contains unverified Drive results");
    }
    Ok(config.out_path.clone())
}

async fn write_failure_report(
    out_path: &std::path::Path,
    config: &DemoContextMirrorRunConfig,
    error: &anyhow::Error,
) -> Result<()> {
    if let Some(parent) = out_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let failure = serde_json::json!({
        "schema_version": "adl_gws_context_mirror_failure.v1",
        "status": "failed",
        "live_mode": config.live_mode,
        "drive_root_folder_id": config.mirror_config.drive_root_folder_id,
        "drive_seed_folder_id": config.mirror_config.drive_seed_folder_id,
        "recursive_sync_enabled": config.mirror_config.recursive_sync_enabled,
        "failure_class": "context_mirror_execution",
        "message": error.to_string(),
        "retryable": false,
        "credential_material_included": false
    });
    tokio::fs::write(out_path, serde_json::to_vec_pretty(&failure)?).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let live_mode = match parse_workspace_execution_mode_from_env() {
        WorkspaceExecutionMode::FixtureBacked => WorkspaceExecutionMode::DryRun,
        mode => mode,
    };
    let recursive_sync_enabled = std::env::var("ADL_GWS_RECURSIVE_SYNC")
        .ok()
        .map(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "enabled"
            )
        })
        .unwrap_or_else(|| matches!(live_mode, WorkspaceExecutionMode::Execute));
    let config = build_demo_config(
        &args,
        live_mode,
        parse_workspace_write_approval_from_env(),
        recursive_sync_enabled,
    );
    let out_path = if matches!(config.live_mode, WorkspaceExecutionMode::Execute) {
        let transport = NativeWorkspaceDriveTransport::new(DefaultWorkspaceAccessTokenProvider)?;
        run_demo_with_transport(&config, &transport).await?
    } else {
        let transport = InMemoryDriveTransportForDemo::new();
        run_demo_with_transport(&config, &transport).await?
    };
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
            "--repo-root".to_string(),
            "tmp/repo".to_string(),
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
        assert_eq!(config.mirror_config.repo_root, "tmp/repo");
        assert_eq!(config.mirror_config.drive_root_folder_id, "root-1");
        assert_eq!(config.mirror_config.drive_seed_folder_id, "seed-1");
        assert!(config.mirror_config.recursive_sync_enabled);
        assert_eq!(config.live_mode, WorkspaceExecutionMode::Execute);
        assert!(config.write_approval_present);
    }

    #[tokio::test]
    async fn context_mirror_demo_runs_and_writes_report() {
        let repo_root = unique_temp_path("context-mirror-repo", "dir");
        let staging_dir = repo_root.join("staging");
        tokio::fs::create_dir_all(repo_root.join("docs/milestones/v0.91.8"))
            .await
            .expect("create milestone tree");
        tokio::fs::create_dir_all(repo_root.join("docs/milestones/v0.92"))
            .await
            .expect("create activation tree");
        tokio::fs::create_dir_all(repo_root.join(".adl/docs/TBD"))
            .await
            .expect("create TBD tree");
        tokio::fs::create_dir_all(&staging_dir)
            .await
            .expect("create staging dir");
        tokio::fs::write(repo_root.join("README.md"), "Active milestone: v0.91.8\n")
            .await
            .expect("write README");
        tokio::fs::write(
            repo_root.join("docs/milestones/v0.91.8/README.md"),
            "# v0.91.8\n",
        )
        .await
        .expect("write milestone README");
        tokio::fs::write(
            repo_root.join("docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md"),
            "activation remains blocked\n",
        )
        .await
        .expect("write activation ledger");
        tokio::fs::write(repo_root.join(".adl/docs/TBD/PLAN.md"), "# Plan\n")
            .await
            .expect("write TBD source");
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
        let mut config = build_demo_config(&args, WorkspaceExecutionMode::DryRun, false, false);
        config.mirror_config.repo_root = repo_root.display().to_string();
        let written = run_demo_with_transport(&config, &InMemoryDriveTransportForDemo::new())
            .await
            .expect("run demo");
        let body = tokio::fs::read_to_string(&written)
            .await
            .expect("read report");
        assert!(body.contains("adl_gws_context_mirror.v1"));
        let first_seed = tokio::fs::read(staging_dir.join("ADL_GOOGLE_DRIVE_SYNC_INDEX.md"))
            .await
            .expect("read first generated seed");
        run_demo_with_transport(&config, &InMemoryDriveTransportForDemo::new())
            .await
            .expect("rerun deterministic generation");
        let second_seed = tokio::fs::read(staging_dir.join("ADL_GOOGLE_DRIVE_SYNC_INDEX.md"))
            .await
            .expect("read second generated seed");
        assert_eq!(first_seed, second_seed);
        tokio::fs::remove_file(&out_path)
            .await
            .expect("remove report");
        tokio::fs::remove_dir_all(&repo_root)
            .await
            .expect("remove repo fixture");
    }

    #[tokio::test]
    async fn context_mirror_execution_error_writes_durable_failure_report() {
        let out_path = unique_temp_path("context-mirror-failure", "json");
        let missing_root = unique_temp_path("context-mirror-missing-root", "dir");
        let args = vec![
            "--out".to_string(),
            out_path.display().to_string(),
            "--drive-root-folder-id".to_string(),
            "demo-root".to_string(),
            "--drive-seed-folder-id".to_string(),
            "demo-root".to_string(),
        ];
        let mut config = build_demo_config(&args, WorkspaceExecutionMode::Execute, true, true);
        config.mirror_config.repo_root = missing_root.display().to_string();
        let error = run_demo_with_transport(&config, &InMemoryDriveTransportForDemo::new())
            .await
            .expect_err("missing repository root must fail");
        assert!(error.to_string().contains("read repo README"));
        let body = tokio::fs::read_to_string(&out_path)
            .await
            .expect("read durable failure report");
        assert!(body.contains("adl_gws_context_mirror_failure.v1"));
        assert!(body.contains("\"status\": \"failed\""));
        tokio::fs::remove_file(out_path)
            .await
            .expect("remove failure report");
    }
}

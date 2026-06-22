use adl::adl_gws_context_mirror::{
    default_context_mirror_config, run_workspace_context_mirror_with_transport,
    write_workspace_context_mirror_report, ADL_GWS_CONTEXT_MIRROR_REPORT_ARTIFACT_PATH,
};
use adl::adl_gws_drive_sync::InMemoryDriveTransportForDemo;
use adl::adl_gws_native::{
    parse_workspace_execution_mode_from_env, parse_workspace_write_approval_from_env,
    WorkspaceExecutionMode,
};
use anyhow::Result;
use std::path::PathBuf;

fn resolve_out_path(arg: Option<String>) -> PathBuf {
    arg.map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(ADL_GWS_CONTEXT_MIRROR_REPORT_ARTIFACT_PATH))
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

#[tokio::main]
async fn main() -> Result<()> {
    let out_path = resolve_out_path(parse_arg("--out"));
    let mut config = default_context_mirror_config();
    if let Some(staging_dir) = parse_arg("--staging-dir") {
        config.staging_dir = staging_dir;
    }
    if let Some(root_id) = parse_arg("--drive-root-folder-id") {
        config.drive_root_folder_id = root_id;
    } else if config.drive_root_folder_id.is_empty() {
        config.drive_root_folder_id = "demo-root".to_string();
    }
    if let Some(seed_id) = parse_arg("--drive-seed-folder-id") {
        config.drive_seed_folder_id = seed_id;
    } else if config.drive_seed_folder_id.is_empty() {
        config.drive_seed_folder_id = "demo-root".to_string();
    }
    if matches!(
        std::env::var("ADL_GWS_RECURSIVE_SYNC")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes" | "enabled"
    ) {
        config.recursive_sync_enabled = true;
    }
    let transport = InMemoryDriveTransportForDemo::new();
    let report = run_workspace_context_mirror_with_transport(
        match parse_workspace_execution_mode_from_env() {
            WorkspaceExecutionMode::FixtureBacked => WorkspaceExecutionMode::DryRun,
            mode => mode,
        },
        parse_workspace_write_approval_from_env(),
        config,
        &transport,
    )
    .await?;
    write_workspace_context_mirror_report(&out_path, &report).await?;
    println!("{}", out_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::resolve_out_path;

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
}

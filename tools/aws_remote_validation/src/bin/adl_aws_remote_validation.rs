use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::fs;

#[path = "../aws_remote_validation.rs"]
mod aws_remote_validation;
#[path = "../cli/observability.rs"]
mod observability;

use aws_remote_validation::{
    run_aws_remote_validation, write_summary_artifacts, AwsRemoteResilienceFaultClass,
    AwsRemoteValidationConfig, AwsRemoteValidationSummary, LiveAwsRemoteValidationAdapter,
    RemoteRunStatus,
};
use observability::ProgressHeartbeat;

#[derive(Debug)]
struct ParsedArgs {
    config: AwsRemoteValidationConfig,
    json_output: bool,
    max_spot_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResumeAttemptRecord {
    attempt_index: u32,
    started_at: String,
    finished_at: String,
    summary_path: String,
    artifact_dir: String,
    status: String,
    failure_reason: Option<String>,
    launch_purchase_option: Option<String>,
    launch_instance_type: Option<String>,
    launch_initial_state: Option<String>,
    launch_instance_id_sha256: Option<String>,
    provider_interruption_confirmed: bool,
    retryable: bool,
    next_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResumeState {
    schema_version: String,
    issue: Option<u32>,
    run_id: String,
    repo_url: String,
    git_ref: String,
    command: String,
    output_summary_ref: String,
    artifact_root_ref: String,
    started_at: String,
    updated_at: String,
    max_spot_retries: u32,
    attempts: Vec<ResumeAttemptRecord>,
    interrupted_attempts: u32,
    next_action: String,
    final_status: Option<String>,
}

fn should_retry_remote_validation(
    summary: &AwsRemoteValidationSummary,
    provider_interruption_confirmed: bool,
    attempt_index: u32,
    max_retries: u32,
) -> bool {
    if attempt_index >= max_retries || !summary.resilience.retryable {
        return false;
    }
    if summary.resilience.fault_class == AwsRemoteResilienceFaultClass::SpotInterrupted {
        return matches!(summary.status, RemoteRunStatus::InterruptedByAws)
            && provider_interruption_confirmed;
    }
    matches!(
        summary.resilience.fault_class,
        AwsRemoteResilienceFaultClass::CapacityUnavailable
            | AwsRemoteResilienceFaultClass::TransientNetwork
            | AwsRemoteResilienceFaultClass::SsmUnavailable
    )
}

fn artifact_ref(path: &Path, artifact_root: &Path) -> String {
    path.strip_prefix(artifact_root)
        .ok()
        .and_then(|relative| {
            let value = relative.display().to_string();
            if value.is_empty() {
                None
            } else {
                Some(value)
            }
        })
        .or_else(|| {
            path.file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}

fn remote_status_label(status: &RemoteRunStatus) -> String {
    serde_json::to_value(status)
        .ok()
        .and_then(|value| value.as_str().map(ToOwned::to_owned))
        .unwrap_or_else(|| format!("{status:?}"))
}

struct EnvVarGuard {
    key: &'static str,
    original: Option<String>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: String) -> Self {
        let original = env::var(key).ok();
        env::set_var(key, value);
        Self { key, original }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(value) = self.original.as_deref() {
            env::set_var(self.key, value);
        } else {
            env::remove_var(self.key);
        }
    }
}

fn usage() -> &'static str {
    "adl-aws-remote-validation run --issue <number> --command <shell-command> --ami-id <ami> --subnet-id <subnet> --security-group-id <sg> --instance-profile-name <name> --out <summary.json> [--artifact-dir <dir>] [--instance-type <type> ...] [--budget-name <name>] [--expected-max-cost-usd <usd>] [--repo-url <url>] [--git-ref <ref>] [--cache-bucket <bucket>] [--cache-prefix <prefix>] [--sccache-tarball-url <url>] [--nextest-tarball-url <url>] [--ssh-key-name <name>] [--ssh-private-key-path <path>] [--ssh-user <user>] [--ssh-allowed-cidr <cidr>] [--cache-volume-id <id>] [--cache-volume-name <name>] [--cache-volume-size-gib <gib>] [--cache-volume-type <type>] [--cache-volume-iops <iops>] [--cache-volume-throughput-mbps <mbps>] [--cache-volume-device-name <device>] [--cache-volume-mount-path <path>] [--command-timeout-seconds <seconds>] [--region <region>] [--profile <profile>] [--json]"
}

fn local_git_stdout(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8(output.stdout).ok()?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn detect_default_git_ref() -> String {
    local_git_stdout(&["symbolic-ref", "--quiet", "--short", "HEAD"])
        .or_else(|| local_git_stdout(&["rev-parse", "HEAD"]))
        .unwrap_or_else(|| "origin/main".to_string())
}

fn remote_git_source_preflight(git_ref: &str) -> Result<()> {
    if let Some(status) = local_git_stdout(&["status", "--porcelain", "--untracked-files=all"]) {
        if !status.is_empty() {
            bail!(
                "remote validation uses a remote git checkout and cannot include local uncommitted or untracked changes; clean, commit, and push the worktree before running live AWS validation"
            );
        }
    }

    let remote_branch_ref = format!("refs/heads/{git_ref}");
    let branch_exists = Command::new("git")
        .args([
            "ls-remote",
            "--exit-code",
            "--heads",
            "origin",
            &remote_branch_ref,
        ])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);
    if branch_exists {
        return Ok(());
    }

    let generic_ref_exists = Command::new("git")
        .args(["ls-remote", "--exit-code", "origin", git_ref])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);
    if generic_ref_exists {
        return Ok(());
    }

    bail!(
        "git ref '{git_ref}' is not advertised by origin; push the branch or pass an explicit remote ref before running live AWS validation"
    );
}

fn parse_args(args: &[String]) -> Result<ParsedArgs> {
    let Some(action) = args.first().map(String::as_str) else {
        bail!("{}", usage());
    };
    if action != "run" {
        bail!("unknown action '{action}'\n{}", usage());
    }
    let mut issue = None;
    let mut run_id = None;
    let mut region = env::var("AWS_REGION").unwrap_or_else(|_| "us-west-2".to_string());
    let mut profile = env::var("AWS_PROFILE").ok();
    let mut repo_url = "https://github.com/danielbaustin/agent-design-language.git".to_string();
    let mut git_ref = detect_default_git_ref();
    let mut cache_bucket = env::var("ADL_AWS_REMOTE_VALIDATION_CACHE_BUCKET").ok();
    let mut cache_prefix = env::var("ADL_AWS_REMOTE_VALIDATION_CACHE_PREFIX").ok();
    let mut sccache_tarball_url = env::var("ADL_AWS_REMOTE_VALIDATION_SCCACHE_TARBALL_URL").ok();
    let mut nextest_tarball_url = env::var("ADL_AWS_REMOTE_VALIDATION_NEXTEST_TARBALL_URL").ok();
    let mut ssh_key_name = env::var("ADL_AWS_REMOTE_VALIDATION_SSH_KEY_NAME").ok();
    let mut ssh_private_key_path = env::var("ADL_AWS_REMOTE_VALIDATION_SSH_PRIVATE_KEY_PATH")
        .ok()
        .map(PathBuf::from);
    let mut ssh_user = env::var("ADL_AWS_REMOTE_VALIDATION_SSH_USER").ok();
    let mut ssh_allowed_cidr = env::var("ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR").ok();
    let mut cache_volume_id = env::var("ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_ID").ok();
    let mut cache_volume_name = env::var("ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_NAME").ok();
    let mut cache_volume_size_gib = env::var("ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_SIZE_GIB")
        .ok()
        .and_then(|value| value.parse::<i32>().ok());
    let mut cache_volume_type = env::var("ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_TYPE").ok();
    let mut cache_volume_iops = env::var("ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_IOPS")
        .ok()
        .and_then(|value| value.parse::<i32>().ok());
    let mut cache_volume_throughput_mbps =
        env::var("ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_THROUGHPUT_MBPS")
            .ok()
            .and_then(|value| value.parse::<i32>().ok());
    let mut cache_volume_device_name =
        env::var("ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_DEVICE_NAME").ok();
    let mut cache_volume_mount_path =
        env::var("ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_MOUNT_PATH").ok();
    let mut command = None;
    let mut out_path = None;
    let mut artifact_dir = None;
    let mut ami_id = None;
    let mut subnet_id = None;
    let mut security_group_id = None;
    let mut instance_profile_name = None;
    let mut instance_types: Vec<String> = Vec::new();
    let mut budget_name = None;
    let mut expected_max_cost_usd = None;
    let mut command_timeout_seconds = None;
    let mut json_output = false;
    let mut max_spot_retries = 2u32;

    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => {
                i += 1;
                issue = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--issue requires a value"))?
                        .parse::<u32>()
                        .map_err(|_| anyhow!("invalid --issue"))?,
                );
            }
            "--run-id" => {
                i += 1;
                run_id = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--run-id requires a value"))?
                        .to_string(),
                );
            }
            "--region" => {
                i += 1;
                region = args
                    .get(i)
                    .ok_or_else(|| anyhow!("--region requires a value"))?
                    .to_string();
            }
            "--profile" => {
                i += 1;
                profile = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--profile requires a value"))?
                        .to_string(),
                );
            }
            "--repo-url" => {
                i += 1;
                repo_url = args
                    .get(i)
                    .ok_or_else(|| anyhow!("--repo-url requires a value"))?
                    .to_string();
            }
            "--git-ref" => {
                i += 1;
                git_ref = args
                    .get(i)
                    .ok_or_else(|| anyhow!("--git-ref requires a value"))?
                    .to_string();
            }
            "--cache-bucket" => {
                i += 1;
                cache_bucket = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--cache-bucket requires a value"))?
                        .to_string(),
                );
            }
            "--cache-prefix" => {
                i += 1;
                cache_prefix = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--cache-prefix requires a value"))?
                        .to_string(),
                );
            }
            "--sccache-tarball-url" => {
                i += 1;
                sccache_tarball_url = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--sccache-tarball-url requires a value"))?
                        .to_string(),
                );
            }
            "--nextest-tarball-url" => {
                i += 1;
                nextest_tarball_url = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--nextest-tarball-url requires a value"))?
                        .to_string(),
                );
            }
            "--ssh-key-name" => {
                i += 1;
                ssh_key_name = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--ssh-key-name requires a value"))?
                        .to_string(),
                );
            }
            "--ssh-private-key-path" => {
                i += 1;
                ssh_private_key_path =
                    Some(PathBuf::from(args.get(i).ok_or_else(|| {
                        anyhow!("--ssh-private-key-path requires a value")
                    })?));
            }
            "--ssh-user" => {
                i += 1;
                ssh_user = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--ssh-user requires a value"))?
                        .to_string(),
                );
            }
            "--ssh-allowed-cidr" => {
                i += 1;
                ssh_allowed_cidr = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--ssh-allowed-cidr requires a value"))?
                        .to_string(),
                );
            }
            "--cache-volume-id" => {
                i += 1;
                cache_volume_id = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--cache-volume-id requires a value"))?
                        .to_string(),
                );
            }
            "--cache-volume-name" => {
                i += 1;
                cache_volume_name = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--cache-volume-name requires a value"))?
                        .to_string(),
                );
            }
            "--cache-volume-size-gib" => {
                i += 1;
                cache_volume_size_gib = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--cache-volume-size-gib requires a value"))?
                        .parse::<i32>()
                        .map_err(|_| anyhow!("invalid --cache-volume-size-gib"))?,
                );
            }
            "--cache-volume-type" => {
                i += 1;
                cache_volume_type = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--cache-volume-type requires a value"))?
                        .to_string(),
                );
            }
            "--cache-volume-iops" => {
                i += 1;
                cache_volume_iops = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--cache-volume-iops requires a value"))?
                        .parse::<i32>()
                        .map_err(|_| anyhow!("invalid --cache-volume-iops"))?,
                );
            }
            "--cache-volume-throughput-mbps" => {
                i += 1;
                cache_volume_throughput_mbps = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--cache-volume-throughput-mbps requires a value"))?
                        .parse::<i32>()
                        .map_err(|_| anyhow!("invalid --cache-volume-throughput-mbps"))?,
                );
            }
            "--cache-volume-device-name" => {
                i += 1;
                cache_volume_device_name = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--cache-volume-device-name requires a value"))?
                        .to_string(),
                );
            }
            "--cache-volume-mount-path" => {
                i += 1;
                cache_volume_mount_path = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--cache-volume-mount-path requires a value"))?
                        .to_string(),
                );
            }
            "--command" => {
                i += 1;
                command = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--command requires a value"))?
                        .to_string(),
                );
            }
            "--out" => {
                i += 1;
                out_path = Some(PathBuf::from(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--out requires a value"))?,
                ));
            }
            "--artifact-dir" => {
                i += 1;
                artifact_dir = Some(PathBuf::from(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--artifact-dir requires a value"))?,
                ));
            }
            "--ami-id" => {
                i += 1;
                ami_id = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--ami-id requires a value"))?
                        .to_string(),
                );
            }
            "--subnet-id" => {
                i += 1;
                subnet_id = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--subnet-id requires a value"))?
                        .to_string(),
                );
            }
            "--security-group-id" => {
                i += 1;
                security_group_id = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--security-group-id requires a value"))?
                        .to_string(),
                );
            }
            "--instance-profile-name" => {
                i += 1;
                instance_profile_name = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--instance-profile-name requires a value"))?
                        .to_string(),
                );
            }
            "--instance-type" => {
                i += 1;
                instance_types.push(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--instance-type requires a value"))?
                        .to_string(),
                );
            }
            "--budget-name" => {
                i += 1;
                budget_name = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--budget-name requires a value"))?
                        .to_string(),
                );
            }
            "--expected-max-cost-usd" => {
                i += 1;
                expected_max_cost_usd = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--expected-max-cost-usd requires a value"))?
                        .parse::<f64>()
                        .map_err(|_| anyhow!("invalid --expected-max-cost-usd"))?,
                );
            }
            "--command-timeout-seconds" => {
                i += 1;
                command_timeout_seconds = Some(
                    args.get(i)
                        .ok_or_else(|| anyhow!("--command-timeout-seconds requires a value"))?
                        .parse::<u64>()
                        .map_err(|_| anyhow!("invalid --command-timeout-seconds"))?,
                );
            }
            "--max-spot-retries" => {
                i += 1;
                max_spot_retries = args
                    .get(i)
                    .ok_or_else(|| anyhow!("--max-spot-retries requires a value"))?
                    .parse::<u32>()
                    .map_err(|_| anyhow!("invalid --max-spot-retries"))?;
            }
            "--json" => json_output = true,
            other => bail!("unknown argument '{other}'"),
        }
        i += 1;
    }

    if instance_types.is_empty() {
        instance_types = vec![
            "c7i.large".to_string(),
            "c7i.xlarge".to_string(),
            "c7i.2xlarge".to_string(),
        ];
    }
    let run_id =
        run_id.unwrap_or_else(|| format!("adl-aws-remote-{}", Utc::now().format("%Y%m%d%H%M%S")));
    let out_path = out_path.ok_or_else(|| anyhow!("--out is required"))?;
    let artifact_dir = artifact_dir.unwrap_or_else(|| {
        out_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(format!("{run_id}.artifacts"))
    });

    Ok(ParsedArgs {
        config: AwsRemoteValidationConfig {
            issue,
            run_id,
            region,
            profile,
            repo_url,
            git_ref,
            cache_bucket,
            cache_prefix,
            sccache_tarball_url,
            nextest_tarball_url,
            ssh_key_name,
            ssh_private_key_path,
            ssh_user,
            ssh_allowed_cidr,
            cache_volume_id,
            cache_volume_name,
            cache_volume_size_gib,
            cache_volume_type,
            cache_volume_iops,
            cache_volume_throughput_mbps,
            cache_volume_device_name,
            cache_volume_mount_path,
            command: command.ok_or_else(|| anyhow!("--command is required"))?,
            out_path,
            artifact_dir,
            ami_id: ami_id.unwrap_or_default(),
            subnet_id: subnet_id.unwrap_or_default(),
            security_group_id: security_group_id.unwrap_or_default(),
            instance_profile_name: instance_profile_name.unwrap_or_default(),
            instance_types,
            budget_name,
            expected_max_cost_usd,
            poll_interval_seconds: 15,
            ssm_ready_timeout_seconds: 600,
            command_timeout_seconds,
            termination_timeout_seconds: 300,
        },
        json_output,
        max_spot_retries,
    })
}

async fn write_resume_state(path: &Path, state: &ResumeState) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::write(path, serde_json::to_string_pretty(state)? + "\n").await?;
    Ok(())
}

fn main() -> Result<()> {
    let raw_args: Vec<String> = env::args().skip(1).collect();
    if raw_args.is_empty()
        || matches!(
            raw_args.first().map(|value| value.as_str()),
            Some("--help" | "-h" | "help")
        )
    {
        println!("{}", usage());
        return Ok(());
    }
    let parsed = parse_args(&raw_args)?;
    let runtime = tokio::runtime::Runtime::new()?;
    let config = parsed.config;
    let json_output = parsed.json_output;
    let max_spot_retries = parsed.max_spot_retries;
    let out_path_display = config.out_path.display().to_string();
    remote_git_source_preflight(&config.git_ref)?;
    let (summary, _) = runtime.block_on(async move {
        let adapter = LiveAwsRemoteValidationAdapter::new(&config).await?;
        let prepared = adapter.prepare_launch_surface(&config).await?;
        let mut effective_config = config.clone();
        effective_config.ami_id = prepared.record.ami_id.clone();
        effective_config.subnet_id = prepared.record.subnet_id.clone();
        effective_config.security_group_id = prepared.record.security_group_id.clone();
        effective_config.instance_profile_name = prepared.record.instance_profile_name.clone();
        fs::create_dir_all(&effective_config.artifact_dir).await?;
        let resume_state_path = effective_config.artifact_dir.join("resume-state.json");
        let run_started_at = Utc::now().to_rfc3339();
        let mut resume_state = ResumeState {
            schema_version: "adl.aws_remote_validation_resume_state.v1".to_string(),
            issue: effective_config.issue,
            run_id: effective_config.run_id.clone(),
            repo_url: effective_config.repo_url.clone(),
            git_ref: effective_config.git_ref.clone(),
            command: effective_config.command.clone(),
            output_summary_ref: artifact_ref(
                &effective_config.out_path,
                &effective_config.artifact_dir,
            ),
            artifact_root_ref: ".".to_string(),
            started_at: run_started_at.clone(),
            updated_at: run_started_at,
            max_spot_retries,
            attempts: Vec::new(),
            interrupted_attempts: 0,
            next_action: "start_attempt_0".to_string(),
            final_status: None,
        };
        write_resume_state(&resume_state_path, &resume_state).await?;
        let mut final_pair = None;
        for attempt_index in 0..=max_spot_retries {
            let attempt_dir = effective_config
                .artifact_dir
                .join(format!("attempt-{}", attempt_index));
            let attempt_out = attempt_dir.join("summary.json");
            let mut attempt_config = effective_config.clone();
            attempt_config.artifact_dir = attempt_dir.clone();
            attempt_config.out_path = attempt_out.clone();
            let _obs_log = EnvVarGuard::set(
                "ADL_OBSERVABILITY_LOG",
                attempt_dir.join("command-status.log").display().to_string(),
            );
            let _obs_stderr = EnvVarGuard::set("ADL_OBSERVABILITY_STDERR", "0".to_string());
            let _obs_heartbeat =
                EnvVarGuard::set("ADL_OBSERVABILITY_HEARTBEAT_MS", "5000".to_string());
            let _obs_root = EnvVarGuard::set(
                "ADL_OBSERVABILITY_REPO_ROOT",
                std::env::current_dir()?.display().to_string(),
            );
            let issue_string = effective_config.issue.unwrap_or_default().to_string();
            let attempt_string = attempt_index.to_string();
            let attempt_started_at = Utc::now().to_rfc3339();
            let heartbeat = ProgressHeartbeat::start(
                "adl-aws-remote-validation",
                "attempt",
                &[
                    ("issue", issue_string.as_str()),
                    ("run_id", effective_config.run_id.as_str()),
                    ("attempt", attempt_string.as_str()),
                ],
            );
            resume_state.next_action = format!("run_attempt_{}", attempt_index);
            resume_state.updated_at = Utc::now().to_rfc3339();
            write_resume_state(&resume_state_path, &resume_state).await?;
            let attempt_result = run_aws_remote_validation(&adapter, &attempt_config).await;
            let (mut summary, events) = match attempt_result {
                Ok(pair) => {
                    heartbeat.completed(&[("result", "completed")]);
                    pair
                }
                Err(err) => {
                    heartbeat.failed(&[("result", "failed"), ("detail", &err.to_string())]);
                    let _cleanup = adapter.cleanup_launch_surface(&prepared).await;
                    return Err(err);
                }
            };
            summary.launch_surface = Some(prepared.record.clone());
            if let Err(err) =
                write_summary_artifacts(&summary, &events, &attempt_out, &attempt_dir).await
            {
                let _cleanup = adapter.cleanup_launch_surface(&prepared).await;
                return Err(err);
            }
            let provider_interruption_confirmed = summary
                .spot_termination_evidence
                .as_ref()
                .map(|evidence| evidence.provider_interruption_confirmed)
                .unwrap_or(false);
            let should_retry = should_retry_remote_validation(
                &summary,
                provider_interruption_confirmed,
                attempt_index,
                max_spot_retries,
            );
            let interrupted = matches!(summary.status, RemoteRunStatus::InterruptedByAws);
            if interrupted {
                resume_state.interrupted_attempts =
                    resume_state.interrupted_attempts.saturating_add(1);
            }
            let next_action = if should_retry {
                format!("retry_after_interruption_{}", attempt_index + 1)
            } else if interrupted {
                "interrupted_no_retry_remaining".to_string()
            } else {
                "finalize".to_string()
            };
            resume_state.attempts.push(ResumeAttemptRecord {
                attempt_index,
                started_at: attempt_started_at,
                finished_at: Utc::now().to_rfc3339(),
                summary_path: artifact_ref(&attempt_out, &effective_config.artifact_dir),
                artifact_dir: artifact_ref(&attempt_dir, &effective_config.artifact_dir),
                status: remote_status_label(&summary.status),
                failure_reason: summary.failure_reason.clone(),
                launch_purchase_option: summary
                    .launch
                    .as_ref()
                    .map(|launch| launch.purchase_option.to_string()),
                launch_instance_type: summary
                    .launch
                    .as_ref()
                    .map(|launch| launch.instance_type.clone()),
                launch_initial_state: summary
                    .launch
                    .as_ref()
                    .map(|launch| launch.initial_state.clone()),
                launch_instance_id_sha256: summary
                    .launch
                    .as_ref()
                    .map(|launch| launch.instance_id_sha256.clone()),
                provider_interruption_confirmed,
                retryable: summary.resilience.retryable,
                next_action: next_action.clone(),
            });
            if should_retry {
                resume_state.next_action = next_action;
                resume_state.updated_at = Utc::now().to_rfc3339();
                write_resume_state(&resume_state_path, &resume_state).await?;
                continue;
            }
            if matches!(summary.status, RemoteRunStatus::Passed)
                && resume_state.interrupted_attempts > 0
            {
                summary.status = RemoteRunStatus::ResumedAfterInterruption;
            }
            final_pair = Some((summary, events));
            break;
        }
        let cleanup = adapter.cleanup_launch_surface(&prepared).await;
        let (mut summary, events) =
            final_pair.ok_or_else(|| anyhow!("no attempt summary recorded"))?;
        summary.launch_surface = Some(prepared.record);
        summary.launch_surface_cleanup = Some(cleanup);
        resume_state.final_status = Some(remote_status_label(&summary.status));
        resume_state.next_action = "complete".to_string();
        resume_state.updated_at = Utc::now().to_rfc3339();
        write_resume_state(&resume_state_path, &resume_state).await?;
        write_summary_artifacts(
            &summary,
            &events,
            &effective_config.out_path,
            &effective_config.artifact_dir,
        )
        .await?;
        Result::<_, anyhow::Error>::Ok((summary, events))
    })?;
    if json_output {
        println!("{}", serde_json::to_string_pretty(&summary)?);
    } else {
        println!("aws_remote_validation_summary={out_path_display}");
    }
    if matches!(
        summary.status,
        RemoteRunStatus::Passed | RemoteRunStatus::ResumedAfterInterruption
    ) {
        Ok(())
    } else {
        bail!(
            "aws remote validation did not complete successfully: {}",
            summary
                .failure_reason
                .unwrap_or_else(|| format!("{:?}", summary.status))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn retry_summary(
        status: RemoteRunStatus,
        fault_class: AwsRemoteResilienceFaultClass,
        retryable: bool,
    ) -> AwsRemoteValidationSummary {
        AwsRemoteValidationSummary {
            schema_version: "adl.aws_remote_validation_run.v1".to_string(),
            issue: Some(4782),
            run_id: "retry-test".to_string(),
            status,
            region: "us-west-2".to_string(),
            profile: Some("agent-logic-admin".to_string()),
            started_at: "2026-07-06T00:00:00Z".to_string(),
            finished_at: "2026-07-06T00:00:01Z".to_string(),
            account_identity: None,
            quota_snapshot: aws_remote_validation::QuotaSnapshot {
                spot_vcpu_quota: Some(32.0),
                on_demand_vcpu_quota: Some(64.0),
                notes: vec![],
            },
            attempts: vec![],
            launch: None,
            cache_volume: None,
            command: None,
            cleanup: aws_remote_validation::CleanupRecord {
                termination_attempted: false,
                final_instance_state: None,
                termination_error: None,
            },
            launch_surface: None,
            launch_surface_cleanup: None,
            resilience: aws_remote_validation::AwsRemoteResilienceRecord {
                schema_version: "adl.aws_remote_validation.resilience.v1".to_string(),
                policy: aws_remote_validation::AwsRemoteResiliencePolicyRecord {
                    policy_id: "test".to_string(),
                    max_launch_attempts: 2,
                    spot_fallback_enabled: true,
                    cleanup_required: true,
                    cost_evidence_required: true,
                    retryable_classes: vec![fault_class.clone()],
                    operator_gated_classes: vec![],
                },
                fault_class,
                disposition: aws_remote_validation::AwsRemoteResilienceDisposition::Retryable,
                retryable,
                cleanup_recorded: true,
                cleanup_complete: true,
                cost_evidence_recorded: false,
                summary: "retry test".to_string(),
                evidence_refs: vec![],
                operator_action: None,
            },
            spot_termination_evidence: None,
            timings: aws_remote_validation::TimingRecord {
                total_seconds: 1,
                launch_seconds: 0,
                ssm_ready_seconds: None,
                remote_command_seconds: None,
                teardown_seconds: None,
            },
            remote_summary: None,
            cost_explorer: None,
            budget_snapshot: None,
            expected_max_cost_usd: None,
            artifact_dir: "artifacts".to_string(),
            event_log_path: "events.jsonl".to_string(),
            non_claims: vec![],
            failure_reason: Some("retry fixture".to_string()),
        }
    }

    #[test]
    fn retry_decision_uses_retryable_resilience_classes() {
        let summary = retry_summary(
            RemoteRunStatus::Failed,
            AwsRemoteResilienceFaultClass::SsmUnavailable,
            true,
        );
        assert!(should_retry_remote_validation(&summary, false, 0, 2));
        assert!(!should_retry_remote_validation(&summary, false, 2, 2));
    }

    #[test]
    fn retry_decision_requires_confirmed_provider_interruption_for_spot() {
        let summary = retry_summary(
            RemoteRunStatus::InterruptedByAws,
            AwsRemoteResilienceFaultClass::SpotInterrupted,
            true,
        );
        assert!(!should_retry_remote_validation(&summary, false, 0, 2));
        assert!(should_retry_remote_validation(&summary, true, 0, 2));
    }

    #[test]
    fn retry_decision_stops_operator_gated_failures() {
        let summary = retry_summary(
            RemoteRunStatus::Failed,
            AwsRemoteResilienceFaultClass::AuthPermissionFailure,
            false,
        );
        assert!(!should_retry_remote_validation(&summary, false, 0, 2));
    }

    #[test]
    fn retry_decision_stops_unknown_validation_failures() {
        let summary = retry_summary(
            RemoteRunStatus::Failed,
            AwsRemoteResilienceFaultClass::Unknown,
            true,
        );
        assert!(!should_retry_remote_validation(&summary, false, 0, 2));
    }

    #[test]
    fn resume_state_artifact_refs_are_relative_when_under_artifact_root() {
        let root = PathBuf::from("/tmp/adl/aws-spot/artifacts");
        assert_eq!(
            artifact_ref(&root.join("attempt-0/summary.json"), &root),
            "attempt-0/summary.json"
        );
        assert_eq!(artifact_ref(&root, &root), "artifacts");
    }

    #[test]
    fn remote_status_label_uses_machine_status_values() {
        assert_eq!(
            remote_status_label(&RemoteRunStatus::InterruptedByAws),
            "interrupted_by_aws"
        );
        assert_eq!(
            remote_status_label(&RemoteRunStatus::ResumedAfterInterruption),
            "resumed_after_interruption"
        );
    }

    fn parse_ok(args: &[&str]) -> ParsedArgs {
        parse_args(
            &args
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>(),
        )
        .expect("args")
    }

    #[test]
    fn parse_args_defaults_instance_pool_and_artifact_dir() {
        let parsed = parse_ok(&[
            "run",
            "--issue",
            "4603",
            "--command",
            "cargo check --manifest-path tools/aws_remote_validation/Cargo.toml --bin adl-aws-remote-validation",
            "--ami-id",
            "ami-123",
            "--subnet-id",
            "subnet-123",
            "--security-group-id",
            "sg-123",
            "--instance-profile-name",
            "profile-123",
            "--out",
            "tmp/aws-remote/summary.json",
        ]);
        assert_eq!(parsed.config.issue, Some(4603));
        assert_eq!(
            parsed.config.instance_types,
            vec![
                "c7i.large".to_string(),
                "c7i.xlarge".to_string(),
                "c7i.2xlarge".to_string()
            ]
        );
        assert_eq!(
            parsed.config.artifact_dir,
            PathBuf::from("tmp/aws-remote").join(format!("{}.artifacts", parsed.config.run_id))
        );
        assert!(!parsed.json_output);
    }

    #[test]
    fn parse_args_honors_explicit_instance_pool_and_json_output() {
        let parsed = parse_ok(&[
            "run",
            "--command",
            "bash adl/tools/run_owner_validation_lane.sh runtime",
            "--cache-bucket",
            "adl-aws-remote-tool-cache-agentlogic",
            "--cache-prefix",
            "adl/remote-validation/4603",
            "--sccache-tarball-url",
            "https://example.com/sccache.tar.gz",
            "--nextest-tarball-url",
            "https://example.com/cargo-nextest.tar.gz",
            "--cache-volume-id",
            "vol-retained",
            "--cache-volume-name",
            "adl-retained-cache",
            "--ami-id",
            "ami-123",
            "--subnet-id",
            "subnet-123",
            "--security-group-id",
            "sg-123",
            "--instance-profile-name",
            "profile-123",
            "--out",
            "summary.json",
            "--artifact-dir",
            "artifacts",
            "--instance-type",
            "c7i.large",
            "--instance-type",
            "c7i.xlarge",
            "--json",
        ]);
        assert_eq!(
            parsed.config.instance_types,
            vec!["c7i.large".to_string(), "c7i.xlarge".to_string()]
        );
        assert_eq!(
            parsed.config.cache_bucket.as_deref(),
            Some("adl-aws-remote-tool-cache-agentlogic")
        );
        assert_eq!(
            parsed.config.cache_prefix.as_deref(),
            Some("adl/remote-validation/4603")
        );
        assert_eq!(
            parsed.config.sccache_tarball_url.as_deref(),
            Some("https://example.com/sccache.tar.gz")
        );
        assert_eq!(
            parsed.config.nextest_tarball_url.as_deref(),
            Some("https://example.com/cargo-nextest.tar.gz")
        );
        assert_eq!(
            parsed.config.cache_volume_id.as_deref(),
            Some("vol-retained")
        );
        assert_eq!(
            parsed.config.cache_volume_name.as_deref(),
            Some("adl-retained-cache")
        );
        assert_eq!(parsed.config.artifact_dir, PathBuf::from("artifacts"));
        assert!(parsed.json_output);
    }

    #[test]
    fn parse_args_honors_max_spot_retries() {
        let parsed = parse_ok(&[
            "run",
            "--command",
            "echo hi",
            "--out",
            "summary.json",
            "--max-spot-retries",
            "4",
        ]);
        assert_eq!(parsed.max_spot_retries, 4);
    }

    #[test]
    fn parse_args_rejects_missing_required_fields() {
        let err = parse_args(&[
            "run".to_string(),
            "--command".to_string(),
            "echo hi".to_string(),
            "--out".to_string(),
            "summary.json".to_string(),
        ])
        .expect("parse should succeed and let runtime auto-prepare aws launch inputs");
        assert!(err.config.ami_id.is_empty());
        assert!(err.config.subnet_id.is_empty());
        assert!(err.config.security_group_id.is_empty());
        assert!(err.config.instance_profile_name.is_empty());
    }
}

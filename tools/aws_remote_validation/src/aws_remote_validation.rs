use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_budgets as budgets;
use aws_sdk_costexplorer as costexplorer;
use aws_sdk_ec2 as ec2;
use aws_sdk_ec2::error::ProvideErrorMetadata;
use aws_sdk_iam as iam;
use aws_sdk_servicequotas as servicequotas;
use aws_sdk_ssm as ssm;
use aws_sdk_sts as sts;
use chrono::{DateTime, Datelike, Duration as ChronoDuration, Timelike, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command as StdCommand;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command as TokioCommand};
use tokio::time::sleep;

const SPOT_QUOTA_NAME: &str = "All Standard (A, C, D, H, I, M, R, T, Z) Spot Instance Requests";
const ON_DEMAND_QUOTA_NAME: &str =
    "Running On-Demand Standard (A, C, D, H, I, M, R, T, Z) instances";
const CACHE_ROLE_POLICY_NAME: &str = "AdlAwsRemoteValidationCacheAccess";

static LIVE_EVENT_LOG_PATH: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Mutex::new(None));
static LIVE_COMMAND_STATUS_LOG_PATH: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Mutex::new(None));

fn spawn_tail_pump<R>(reader: R, sink: Arc<Mutex<std::fs::File>>)
where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        let mut lines = BufReader::new(reader).lines();
        loop {
            match lines.next_line().await {
                Ok(Some(line)) => {
                    eprintln!("{line}");
                    if let Ok(mut file) = sink.lock() {
                        let _ = writeln!(file, "{line}");
                    }
                }
                Ok(None) | Err(_) => break,
            }
        }
    });
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PurchaseOption {
    Spot,
    OnDemand,
}

impl fmt::Display for PurchaseOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spot => write!(f, "spot"),
            Self::OnDemand => write!(f, "on_demand"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RemoteRunStatus {
    Passed,
    Failed,
    InterruptedByAws,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AwsRemoteValidationConfig {
    pub issue: Option<u32>,
    pub run_id: String,
    pub region: String,
    pub profile: Option<String>,
    pub repo_url: String,
    pub git_ref: String,
    pub cache_bucket: Option<String>,
    pub cache_prefix: Option<String>,
    pub sccache_tarball_url: Option<String>,
    pub nextest_tarball_url: Option<String>,
    pub ssh_key_name: Option<String>,
    pub ssh_private_key_path: Option<PathBuf>,
    pub ssh_user: Option<String>,
    pub ssh_allowed_cidr: Option<String>,
    pub cache_volume_name: Option<String>,
    pub cache_volume_size_gib: Option<i32>,
    pub cache_volume_type: Option<String>,
    pub cache_volume_iops: Option<i32>,
    pub cache_volume_throughput_mbps: Option<i32>,
    pub cache_volume_device_name: Option<String>,
    pub cache_volume_mount_path: Option<String>,
    pub command: String,
    pub out_path: PathBuf,
    pub artifact_dir: PathBuf,
    pub ami_id: String,
    pub subnet_id: String,
    pub security_group_id: String,
    pub instance_profile_name: String,
    pub instance_types: Vec<String>,
    pub budget_name: Option<String>,
    pub expected_max_cost_usd: Option<f64>,
    pub poll_interval_seconds: u64,
    pub ssm_ready_timeout_seconds: u64,
    pub command_timeout_seconds: Option<u64>,
    pub termination_timeout_seconds: u64,
}

#[derive(Debug, Clone)]
struct SshDebugConfig {
    private_key_path: PathBuf,
    user: String,
    allowed_cidr: String,
}

impl AwsRemoteValidationConfig {
    pub fn validate(&self) -> Result<()> {
        if self.command.trim().is_empty() {
            return Err(anyhow!("command must not be empty"));
        }
        if self.instance_types.is_empty() {
            return Err(anyhow!("at least one --instance-type is required"));
        }
        let cache_volume_enabled = self.cache_volume_name.is_some()
            || self.cache_volume_size_gib.is_some()
            || self.cache_volume_type.is_some()
            || self.cache_volume_iops.is_some()
            || self.cache_volume_throughput_mbps.is_some()
            || self.cache_volume_device_name.is_some()
            || self.cache_volume_mount_path.is_some();
        if cache_volume_enabled && self.cache_volume_name.as_deref().unwrap_or("").trim().is_empty()
        {
            return Err(anyhow!(
                "cache_volume_name is required when cache-volume options are set"
            ));
        }
        if self.ami_id.trim().is_empty()
            || self.subnet_id.trim().is_empty()
            || self.security_group_id.trim().is_empty()
            || self.instance_profile_name.trim().is_empty()
        {
            return Err(anyhow!(
                "ami_id, subnet_id, security_group_id, and instance_profile_name are required"
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AwsAccountIdentity {
    pub account_id: Option<String>,
    pub account_id_sha256: Option<String>,
    pub arn: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuotaSnapshot {
    pub spot_vcpu_quota: Option<f64>,
    pub on_demand_vcpu_quota: Option<f64>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BudgetSnapshot {
    pub budget_name: String,
    pub limit_amount: Option<String>,
    pub limit_unit: Option<String>,
    pub actual_spend_amount: Option<String>,
    pub actual_spend_unit: Option<String>,
    pub forecast_spend_amount: Option<String>,
    pub forecast_spend_unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostExplorerSnapshot {
    pub start: String,
    pub end: String,
    pub service: String,
    pub amount: Option<String>,
    pub unit: Option<String>,
    pub delayed_billing_boundary: bool,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttemptRecord {
    pub instance_type: String,
    pub purchase_option: PurchaseOption,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaunchRecord {
    pub purchase_option: PurchaseOption,
    pub instance_type: String,
    pub instance_id: String,
    pub instance_id_sha256: String,
    pub launched_at: String,
    pub initial_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandRecord {
    pub command_id: String,
    pub status: String,
    pub response_code: Option<i32>,
    pub stdout_path: String,
    pub stderr_path: String,
    pub output_preview: String,
    pub stderr_preview: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CleanupRecord {
    pub termination_attempted: bool,
    pub final_instance_state: Option<String>,
    pub termination_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaunchSurfaceRecord {
    pub provisioning_mode: String,
    pub ami_id: String,
    pub ami_source: String,
    pub vpc_id: String,
    pub subnet_id: String,
    pub availability_zone: Option<String>,
    pub security_group_id: String,
    pub security_group_name: Option<String>,
    pub security_group_created: bool,
    pub instance_profile_name: String,
    pub role_name: Option<String>,
    pub instance_profile_created: bool,
    pub ssh_debug_enabled: bool,
    pub ssh_allowed_cidr: Option<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheVolumeRequest {
    pub name: String,
    pub size_gib: i32,
    pub volume_type: String,
    pub iops: Option<i32>,
    pub throughput_mbps: Option<i32>,
    pub device_name: String,
    pub mount_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheVolumeRecord {
    pub name: String,
    pub volume_id: String,
    pub availability_zone: String,
    pub size_gib: i32,
    pub volume_type: String,
    pub iops: Option<i32>,
    pub throughput_mbps: Option<i32>,
    pub device_name: String,
    pub mount_path: String,
    pub created: bool,
    pub attachment_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaunchSurfaceCleanupRecord {
    pub security_group_deleted: Option<bool>,
    pub instance_profile_deleted: Option<bool>,
    pub role_deleted: Option<bool>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpotTerminationEvidence {
    pub instance_id: String,
    pub instance_lifecycle: Option<String>,
    pub state_reason_code: Option<String>,
    pub state_reason_message: Option<String>,
    pub state_transition_reason: Option<String>,
    pub spot_instance_request_id: Option<String>,
    pub spot_request_status_code: Option<String>,
    pub spot_request_status_message: Option<String>,
    pub provider_interruption_confirmed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimingRecord {
    pub total_seconds: u64,
    pub launch_seconds: u64,
    pub ssm_ready_seconds: Option<u64>,
    pub remote_command_seconds: Option<u64>,
    pub teardown_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SccacheStats {
    pub compile_requests: Option<String>,
    pub compile_requests_executed: Option<String>,
    pub cache_hits: Option<String>,
    pub cache_misses: Option<String>,
    pub raw_excerpt: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RemoteCommandSummary {
    pub status: String,
    pub bootstrap_seconds: Option<u64>,
    pub command_seconds: Option<u64>,
    pub interruption_detected: bool,
    pub interruption_notice: Option<String>,
    pub resolved_commit: Option<String>,
    pub rustc_version: Option<String>,
    pub cargo_version: Option<String>,
    pub sccache_version: Option<String>,
    #[serde(default)]
    pub sccache_degraded: bool,
    #[serde(default)]
    pub sccache_degraded_reason: Option<String>,
    pub sccache_stats: Option<SccacheStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AwsRemoteValidationSummary {
    pub schema_version: String,
    pub issue: Option<u32>,
    pub run_id: String,
    pub status: RemoteRunStatus,
    pub region: String,
    pub profile: Option<String>,
    pub started_at: String,
    pub finished_at: String,
    pub account_identity: Option<AwsAccountIdentity>,
    pub quota_snapshot: QuotaSnapshot,
    pub attempts: Vec<AttemptRecord>,
    pub launch: Option<LaunchRecord>,
    pub cache_volume: Option<CacheVolumeRecord>,
    pub command: Option<CommandRecord>,
    pub cleanup: CleanupRecord,
    pub launch_surface: Option<LaunchSurfaceRecord>,
    pub launch_surface_cleanup: Option<LaunchSurfaceCleanupRecord>,
    pub spot_termination_evidence: Option<SpotTerminationEvidence>,
    pub timings: TimingRecord,
    pub remote_summary: Option<RemoteCommandSummary>,
    pub cost_explorer: Option<CostExplorerSnapshot>,
    pub budget_snapshot: Option<BudgetSnapshot>,
    pub expected_max_cost_usd: Option<f64>,
    pub artifact_dir: String,
    pub event_log_path: String,
    pub non_claims: Vec<String>,
    pub failure_reason: Option<String>,
}

fn is_valid_spot_interruption_notice(notice: Option<&str>) -> bool {
    let Some(notice) = notice.map(str::trim).filter(|notice| !notice.is_empty()) else {
        return false;
    };
    if notice.starts_with('<')
        || notice.contains("<html")
        || notice.contains("<!DOCTYPE")
        || notice.contains("404 - Not Found")
    {
        return false;
    }
    match serde_json::from_str::<serde_json::Value>(notice) {
        Ok(value) => {
            let action = value
                .get("action")
                .and_then(|value| value.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty());
            let time = value
                .get("time")
                .and_then(|value| value.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty());
            action.is_some() && time.is_some()
        }
        Err(_) => false,
    }
}

fn remote_summary_reports_valid_interruption(summary: &RemoteCommandSummary) -> bool {
    summary.interruption_detected
        && is_valid_spot_interruption_notice(summary.interruption_notice.as_deref())
}

#[derive(Debug, Clone, Serialize)]
pub struct EventRecord {
    pub timestamp: String,
    pub stage: String,
    pub status: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchSpec {
    pub instance_type: String,
    pub purchase_option: PurchaseOption,
    pub ami_id: String,
    pub subnet_id: String,
    pub security_group_id: String,
    pub instance_profile_name: String,
    pub ssh_key_name: Option<String>,
    pub cache_volume: Option<CacheVolumeRequest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchResult {
    pub instance_id: String,
    pub initial_state: String,
    pub cache_volume: Option<CacheVolumeRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SsmReadyResult {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandExecutionResult {
    pub command_id: String,
    pub status: String,
    pub response_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwsAdapterError {
    pub code: Option<String>,
    pub message: String,
    pub spot_fallback_permitted: bool,
}

impl fmt::Display for AwsAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code.as_deref() {
            Some(code) => write!(f, "{code}: {}", self.message),
            None => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for AwsAdapterError {}

#[async_trait]
pub trait AwsRemoteValidationAdapter {
    async fn caller_identity(&self) -> Result<AwsAccountIdentity>;
    async fn quota_snapshot(&self) -> Result<QuotaSnapshot>;
    async fn launch_instance(
        &self,
        spec: &LaunchSpec,
    ) -> std::result::Result<LaunchResult, AwsAdapterError>;
    async fn wait_for_ssm_online(
        &self,
        instance_id: &str,
        timeout: Duration,
        poll_interval: Duration,
    ) -> std::result::Result<SsmReadyResult, AwsAdapterError>;
    async fn run_remote_command(
        &self,
        instance_id: &str,
        command: &str,
        timeout: Option<Duration>,
        poll_interval: Duration,
    ) -> std::result::Result<CommandExecutionResult, AwsAdapterError>;
    async fn instance_state(
        &self,
        instance_id: &str,
    ) -> std::result::Result<Option<String>, AwsAdapterError>;
    async fn terminate_instance(
        &self,
        instance_id: &str,
    ) -> std::result::Result<(), AwsAdapterError>;
    async fn wait_for_termination(
        &self,
        instance_id: &str,
        timeout: Duration,
        poll_interval: Duration,
    ) -> std::result::Result<Option<String>, AwsAdapterError>;
    async fn spot_termination_evidence(
        &self,
        instance_id: &str,
    ) -> Result<Option<SpotTerminationEvidence>>;
    async fn cost_snapshot(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Option<CostExplorerSnapshot>>;
    async fn budget_snapshot(&self, budget_name: &str) -> Result<Option<BudgetSnapshot>>;
}

pub async fn run_aws_remote_validation<A: AwsRemoteValidationAdapter>(
    adapter: &A,
    config: &AwsRemoteValidationConfig,
) -> Result<(AwsRemoteValidationSummary, Vec<EventRecord>)> {
    config.validate()?;
    initialize_live_log_paths(&config.artifact_dir)?;
    let start_wall = Utc::now();
    let total_timer = Instant::now();
    let mut events = Vec::new();
    let mut attempts = Vec::new();
    let mut launch: Option<LaunchRecord> = None;
    let mut command_record: Option<CommandRecord> = None;
    let mut remote_summary: Option<RemoteCommandSummary> = None;
    let mut failure_reason = None;
    let mut cleanup = CleanupRecord {
        termination_attempted: false,
        final_instance_state: None,
        termination_error: None,
    };
    let mut spot_termination_evidence = None;
    let mut ssm_ready_seconds = None;
    let mut remote_command_seconds = None;
    let mut teardown_seconds = None;
    let mut status = RemoteRunStatus::Failed;

    record_event(
        &mut events,
        "start",
        "started",
        format!(
            "issue={:?} region={} instance_types={} out={}",
            config.issue,
            config.region,
            config.instance_types.join(","),
            config.out_path.display()
        ),
    );

    let account_identity = match adapter.caller_identity().await {
        Ok(identity) => {
            record_event(
                &mut events,
                "identity",
                "ok",
                "caller identity resolved".to_string(),
            );
            Some(identity)
        }
        Err(err) => {
            record_event(
                &mut events,
                "identity",
                "warn",
                format!("caller identity unavailable: {err}"),
            );
            None
        }
    };

    let quota_snapshot = match adapter.quota_snapshot().await {
        Ok(snapshot) => {
            record_event(
                &mut events,
                "quota",
                "ok",
                "quota snapshot captured".to_string(),
            );
            snapshot
        }
        Err(err) => {
            record_event(
                &mut events,
                "quota",
                "warn",
                format!("quota snapshot unavailable: {err}"),
            );
            QuotaSnapshot {
                spot_vcpu_quota: None,
                on_demand_vcpu_quota: None,
                notes: vec![format!("quota capture unavailable: {err}")],
            }
        }
    };

    let cache_volume_request = build_cache_volume_request(config);
    let mut instance_id: Option<String> = None;
    let mut cache_volume: Option<CacheVolumeRecord> = None;
    let launch_timer = Instant::now();
    'launch: for instance_type in &config.instance_types {
        let spot_spec = LaunchSpec {
            instance_type: instance_type.clone(),
            purchase_option: PurchaseOption::Spot,
            ami_id: config.ami_id.clone(),
            subnet_id: config.subnet_id.clone(),
            security_group_id: config.security_group_id.clone(),
            instance_profile_name: config.instance_profile_name.clone(),
            ssh_key_name: config.ssh_key_name.clone(),
            cache_volume: cache_volume_request.clone(),
        };
        record_event(
            &mut events,
            "launch_attempt",
            "started",
            format!("instance_type={} market=spot", instance_type),
        );
        match adapter.launch_instance(&spot_spec).await {
            Ok(result) => {
                let launched_at = Utc::now();
                let hashed = sha256_hex(&result.instance_id);
                launch = Some(LaunchRecord {
                    purchase_option: PurchaseOption::Spot,
                    instance_type: instance_type.clone(),
                    instance_id: result.instance_id.clone(),
                    instance_id_sha256: hashed,
                    launched_at: launched_at.to_rfc3339(),
                    initial_state: result.initial_state,
                });
                instance_id = Some(result.instance_id);
                cache_volume = result.cache_volume;
                attempts.push(AttemptRecord {
                    instance_type: instance_type.clone(),
                    purchase_option: PurchaseOption::Spot,
                    status: "launched".to_string(),
                    message: "spot launch succeeded".to_string(),
                });
                record_event(
                    &mut events,
                    "launch_attempt",
                    "ok",
                    format!("instance_type={} market=spot launched", instance_type),
                );
                break 'launch;
            }
            Err(err) => {
                attempts.push(AttemptRecord {
                    instance_type: instance_type.clone(),
                    purchase_option: PurchaseOption::Spot,
                    status: "failed".to_string(),
                    message: err.to_string(),
                });
                record_event(
                    &mut events,
                    "launch_attempt",
                    "warn",
                    format!(
                        "instance_type={} market=spot failed: {}",
                        instance_type, err
                    ),
                );
                if !err.spot_fallback_permitted {
                    failure_reason = Some(format!(
                        "spot launch failed without permitted fallback: {err}"
                    ));
                    break 'launch;
                }
            }
        }

        let on_demand_spec = LaunchSpec {
            purchase_option: PurchaseOption::OnDemand,
            ..spot_spec
        };
        record_event(
            &mut events,
            "launch_attempt",
            "started",
            format!("instance_type={} market=on_demand", instance_type),
        );
        match adapter.launch_instance(&on_demand_spec).await {
            Ok(result) => {
                let launched_at = Utc::now();
                let hashed = sha256_hex(&result.instance_id);
                launch = Some(LaunchRecord {
                    purchase_option: PurchaseOption::OnDemand,
                    instance_type: instance_type.clone(),
                    instance_id: result.instance_id.clone(),
                    instance_id_sha256: hashed,
                    launched_at: launched_at.to_rfc3339(),
                    initial_state: result.initial_state,
                });
                instance_id = Some(result.instance_id);
                cache_volume = result.cache_volume;
                attempts.push(AttemptRecord {
                    instance_type: instance_type.clone(),
                    purchase_option: PurchaseOption::OnDemand,
                    status: "launched".to_string(),
                    message: "on-demand launch succeeded after spot fallback".to_string(),
                });
                record_event(
                    &mut events,
                    "launch_attempt",
                    "ok",
                    format!("instance_type={} market=on_demand launched", instance_type),
                );
                break 'launch;
            }
            Err(err) => {
                attempts.push(AttemptRecord {
                    instance_type: instance_type.clone(),
                    purchase_option: PurchaseOption::OnDemand,
                    status: "failed".to_string(),
                    message: err.to_string(),
                });
                record_event(
                    &mut events,
                    "launch_attempt",
                    "warn",
                    format!(
                        "instance_type={} market=on_demand failed: {}",
                        instance_type, err
                    ),
                );
                failure_reason = Some(format!(
                    "failed to launch instance_type={} after spot and on-demand attempts: {}",
                    instance_type, err
                ));
            }
        }
    }
    let launch_seconds = launch_timer.elapsed().as_secs();

    if let Some(instance_id_ref) = instance_id.as_deref() {
        let ssm_timer = Instant::now();
        match adapter
            .wait_for_ssm_online(
                instance_id_ref,
                Duration::from_secs(config.ssm_ready_timeout_seconds),
                Duration::from_secs(config.poll_interval_seconds),
            )
            .await
        {
            Ok(result) => {
                ssm_ready_seconds = Some(ssm_timer.elapsed().as_secs());
                record_event(
                    &mut events,
                    "ssm",
                    "ok",
                    format!("instance became SSM-ready with status {}", result.status),
                );
            }
            Err(err) => {
                failure_reason = Some(format!("instance never became SSM-ready: {err}"));
                record_event(&mut events, "ssm", "failed", err.to_string());
            }
        }
    }

    if failure_reason.is_none() {
        if let Some(instance_id_ref) = instance_id.as_deref() {
            let remote_script = build_remote_command_script(config);
            let remote_timer = Instant::now();
            match adapter
                .run_remote_command(
                    instance_id_ref,
                    &remote_script,
                    config.command_timeout_seconds.map(Duration::from_secs),
                    Duration::from_secs(config.poll_interval_seconds),
                )
                .await
            {
                Ok(result) => {
                    remote_command_seconds = Some(remote_timer.elapsed().as_secs());
                    let stdout_path = config.artifact_dir.join("command-stdout.log");
                    let stderr_path = config.artifact_dir.join("command-stderr.log");
                    fs::create_dir_all(&config.artifact_dir)
                        .await
                        .with_context(|| {
                            format!(
                                "failed to create artifact dir '{}'",
                                config.artifact_dir.display()
                            )
                        })?;
                    fs::write(&stdout_path, &result.stdout)
                        .await
                        .with_context(|| format!("failed to write '{}'", stdout_path.display()))?;
                    fs::write(&stderr_path, &result.stderr)
                        .await
                        .with_context(|| format!("failed to write '{}'", stderr_path.display()))?;
                    let parsed = parse_remote_summary(&result.stdout)
                        .or_else(|| parse_remote_summary(&result.stderr));
                    let interruption_detected = parsed
                        .as_ref()
                        .map(remote_summary_reports_valid_interruption)
                        .unwrap_or(false);
                    command_record = Some(CommandRecord {
                        command_id: result.command_id.clone(),
                        status: result.status.clone(),
                        response_code: result.response_code,
                        stdout_path: stdout_path.display().to_string(),
                        stderr_path: stderr_path.display().to_string(),
                        output_preview: preview(&result.stdout),
                        stderr_preview: preview(&result.stderr),
                    });
                    remote_summary = parsed;
                    if interruption_detected {
                        status = RemoteRunStatus::InterruptedByAws;
                        failure_reason = Some(
                            "spot interruption notice detected during remote execution".to_string(),
                        );
                        record_event(
                            &mut events,
                            "remote_command",
                            "interrupted",
                            "interruption notice detected in remote summary".to_string(),
                        );
                    } else if remote_summary
                        .as_ref()
                        .map(|summary| summary.sccache_degraded)
                        .unwrap_or(false)
                    {
                        status = RemoteRunStatus::Failed;
                        let reason = remote_summary
                            .as_ref()
                            .and_then(|summary| summary.sccache_degraded_reason.clone())
                            .unwrap_or_else(|| "unknown_sccache_degradation".to_string());
                        failure_reason =
                            Some(format!("remote command lost sccache integrity: {reason}"));
                        record_event(
                            &mut events,
                            "remote_command",
                            "failed",
                            failure_reason.clone().unwrap_or_default(),
                        );
                    } else if result.response_code.unwrap_or(1) == 0
                        && result.status.eq_ignore_ascii_case("Success")
                    {
                        status = RemoteRunStatus::Passed;
                        record_event(
                            &mut events,
                            "remote_command",
                            "ok",
                            format!("remote command completed with {}", result.status),
                        );
                    } else {
                        status = RemoteRunStatus::Failed;
                        failure_reason = Some(format!(
                            "remote command failed with status={} response_code={:?}",
                            result.status, result.response_code
                        ));
                        record_event(
                            &mut events,
                            "remote_command",
                            "failed",
                            failure_reason.clone().unwrap_or_default(),
                        );
                    }
                }
                Err(err) => {
                    let current_state = adapter
                        .instance_state(instance_id_ref)
                        .await
                        .ok()
                        .flatten()
                        .unwrap_or_else(|| "unknown".to_string());
                    spot_termination_evidence = adapter
                        .spot_termination_evidence(instance_id_ref)
                        .await
                        .ok()
                        .flatten();
                    let provider_interruption_confirmed = spot_termination_evidence
                        .as_ref()
                        .map(|evidence| evidence.provider_interruption_confirmed)
                        .unwrap_or(false);
                    let interrupted = matches!(
                        current_state.as_str(),
                        "shutting-down" | "terminated" | "stopping" | "stopped"
                    ) || provider_interruption_confirmed;
                    status = if interrupted {
                        RemoteRunStatus::InterruptedByAws
                    } else {
                        RemoteRunStatus::Failed
                    };
                    failure_reason = Some(format!("remote command dispatch failed: {err}"));
                    record_event(
                        &mut events,
                        "remote_command",
                        if interrupted { "interrupted" } else { "failed" },
                        format!(
                            "dispatch failed while instance_state={current_state} provider_interruption_confirmed={provider_interruption_confirmed}: {err}"
                        ),
                    );
                }
            }
        }
    }

    if let Some(instance_id_ref) = instance_id.as_deref() {
        let teardown_timer = Instant::now();
        cleanup.termination_attempted = true;
        match adapter.terminate_instance(instance_id_ref).await {
            Ok(()) => {
                record_event(
                    &mut events,
                    "cleanup",
                    "started",
                    "termination requested".to_string(),
                );
                match adapter
                    .wait_for_termination(
                        instance_id_ref,
                        Duration::from_secs(config.termination_timeout_seconds),
                        Duration::from_secs(config.poll_interval_seconds),
                    )
                    .await
                {
                    Ok(final_state) => {
                        cleanup.final_instance_state = final_state;
                        record_event(
                            &mut events,
                            "cleanup",
                            "ok",
                            format!(
                                "termination observed state={}",
                                cleanup
                                    .final_instance_state
                                    .clone()
                                    .unwrap_or_else(|| "unknown".to_string())
                            ),
                        );
                    }
                    Err(err) => {
                        cleanup.termination_error = Some(err.to_string());
                        record_event(
                            &mut events,
                            "cleanup",
                            "warn",
                            format!("termination wait failed: {err}"),
                        );
                    }
                }
            }
            Err(err) => {
                cleanup.termination_error = Some(err.to_string());
                record_event(
                    &mut events,
                    "cleanup",
                    "failed",
                    format!("termination request failed: {err}"),
                );
            }
        }
        teardown_seconds = Some(teardown_timer.elapsed().as_secs());
    }

    let month_start = start_of_month(start_wall);
    let month_end = Utc::now() + ChronoDuration::days(1);
    let cost_explorer = match adapter.cost_snapshot(month_start, month_end).await {
        Ok(snapshot) => snapshot,
        Err(err) => {
            record_event(
                &mut events,
                "cost",
                "warn",
                format!("cost snapshot unavailable: {err}"),
            );
            None
        }
    };
    let budget_snapshot = match config.budget_name.as_deref() {
        Some(budget_name) => match adapter.budget_snapshot(budget_name).await {
            Ok(snapshot) => snapshot,
            Err(err) => {
                record_event(
                    &mut events,
                    "budget",
                    "warn",
                    format!("budget snapshot unavailable: {err}"),
                );
                None
            }
        },
        None => None,
    };

    if failure_reason.is_none()
        && !matches!(
            status,
            RemoteRunStatus::Passed | RemoteRunStatus::InterruptedByAws
        )
    {
        status = RemoteRunStatus::Passed;
    }
    let finished_at = Utc::now();
    let summary = AwsRemoteValidationSummary {
        schema_version: "adl.aws_remote_validation_run.v1".to_string(),
        issue: config.issue,
        run_id: config.run_id.clone(),
        status,
        region: config.region.clone(),
        profile: config.profile.clone(),
        started_at: start_wall.to_rfc3339(),
        finished_at: finished_at.to_rfc3339(),
        account_identity,
        quota_snapshot,
        attempts,
        launch,
        cache_volume,
        command: command_record,
        cleanup,
        launch_surface: None,
        launch_surface_cleanup: None,
        spot_termination_evidence,
        timings: TimingRecord {
            total_seconds: total_timer.elapsed().as_secs(),
            launch_seconds,
            ssm_ready_seconds,
            remote_command_seconds,
            teardown_seconds,
        },
        remote_summary,
        cost_explorer,
        budget_snapshot,
        expected_max_cost_usd: config.expected_max_cost_usd,
        artifact_dir: config.artifact_dir.display().to_string(),
        event_log_path: config.artifact_dir.join("events.jsonl").display().to_string(),
        non_claims: vec![
            "This summary does not claim broad CI migration to EC2.".to_string(),
            "This summary does not claim Spot savings unless a Spot-backed run retained cost and timing evidence.".to_string(),
            "This summary does not claim per-run final AWS billing line items when Cost Explorer data is delayed.".to_string(),
        ],
        failure_reason,
    };

    clear_live_log_paths();
    Ok((summary, events))
}

pub async fn write_summary_artifacts(
    summary: &AwsRemoteValidationSummary,
    events: &[EventRecord],
    out_path: &Path,
    artifact_dir: &Path,
) -> Result<()> {
    fs::create_dir_all(artifact_dir)
        .await
        .with_context(|| format!("failed to create '{}'", artifact_dir.display()))?;
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .await
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }
    let event_log_path = artifact_dir.join("events.jsonl");
    let mut jsonl = String::new();
    for event in events {
        jsonl.push_str(&serde_json::to_string(event)?);
        jsonl.push('\n');
    }
    fs::write(&event_log_path, jsonl)
        .await
        .with_context(|| format!("failed to write '{}'", event_log_path.display()))?;
    fs::write(out_path, serde_json::to_string_pretty(summary)? + "\n")
        .await
        .with_context(|| format!("failed to write '{}'", out_path.display()))?;
    Ok(())
}

fn record_event(events: &mut Vec<EventRecord>, stage: &str, status: &str, detail: String) {
    let event = EventRecord {
        timestamp: Utc::now().to_rfc3339(),
        stage: stage.to_string(),
        status: status.to_string(),
        detail,
    };
    append_jsonl_line(&LIVE_EVENT_LOG_PATH, &event);
    events.push(event);
}

fn initialize_live_log_paths(artifact_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(artifact_dir)
        .with_context(|| format!("failed to create '{}'", artifact_dir.display()))?;
    set_live_log_path(&LIVE_EVENT_LOG_PATH, artifact_dir.join("events.jsonl"));
    set_live_log_path(
        &LIVE_COMMAND_STATUS_LOG_PATH,
        artifact_dir.join("command-status.log"),
    );
    Ok(())
}

fn clear_live_log_paths() {
    set_live_log_path(&LIVE_EVENT_LOG_PATH, None::<PathBuf>);
    set_live_log_path(&LIVE_COMMAND_STATUS_LOG_PATH, None::<PathBuf>);
}

fn set_live_log_path<P>(slot: &Lazy<Mutex<Option<PathBuf>>>, path: P)
where
    P: Into<Option<PathBuf>>,
{
    if let Ok(mut guard) = slot.lock() {
        *guard = path.into();
    }
}

fn append_jsonl_line<T: Serialize>(slot: &Lazy<Mutex<Option<PathBuf>>>, value: &T) {
    let Some(path) = slot.lock().ok().and_then(|guard| guard.clone()) else {
        return;
    };
    let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) else {
        return;
    };
    let Ok(line) = serde_json::to_string(value) else {
        return;
    };
    let _ = writeln!(file, "{line}");
}

fn append_command_status_line(status: &str, detail: impl Into<String>) {
    let Some(path) = LIVE_COMMAND_STATUS_LOG_PATH
        .lock()
        .ok()
        .and_then(|guard| guard.clone())
    else {
        return;
    };
    let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) else {
        return;
    };
    let timestamp = Utc::now().to_rfc3339();
    let _ = writeln!(file, "{timestamp} status={status} {}", detail.into());
}

fn sha256_hex(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn preview(text: &str) -> String {
    text.lines().take(8).collect::<Vec<_>>().join(" | ")
}

fn build_ssh_debug_config(config: &AwsRemoteValidationConfig) -> Result<Option<SshDebugConfig>> {
    let Some(_key_name) = config.ssh_key_name.as_deref() else {
        return Ok(None);
    };
    let private_key_path = config
        .ssh_private_key_path
        .clone()
        .ok_or_else(|| anyhow!("ssh_private_key_path is required when ssh_key_name is set"))?;
    let user = config
        .ssh_user
        .clone()
        .unwrap_or_else(|| "ec2-user".to_string());
    let allowed_cidr = match config.ssh_allowed_cidr.clone() {
        Some(value) => value,
        None => {
            let output = StdCommand::new("curl")
                .args(["-fsSL", "https://checkip.amazonaws.com"])
                .output()
                .context("failed to detect public IP for SSH debug mode")?;
            if !output.status.success() {
                return Err(anyhow!("failed to detect public IP for SSH debug mode"));
            }
            let ip = String::from_utf8(output.stdout)
                .context("failed to decode public IP response for SSH debug mode")?;
            format!("{}/32", ip.trim())
        }
    };
    Ok(Some(SshDebugConfig {
        private_key_path,
        user,
        allowed_cidr,
    }))
}

fn build_cache_volume_request(config: &AwsRemoteValidationConfig) -> Option<CacheVolumeRequest> {
    let name = config.cache_volume_name.as_ref()?.trim();
    if name.is_empty() {
        return None;
    }
    Some(CacheVolumeRequest {
        name: name.to_string(),
        size_gib: config.cache_volume_size_gib.unwrap_or(200),
        volume_type: config
            .cache_volume_type
            .clone()
            .unwrap_or_else(|| "gp3".to_string()),
        iops: config.cache_volume_iops,
        throughput_mbps: config.cache_volume_throughput_mbps,
        device_name: config
            .cache_volume_device_name
            .clone()
            .unwrap_or_else(|| "/dev/sdf".to_string()),
        mount_path: config
            .cache_volume_mount_path
            .clone()
            .unwrap_or_else(|| "/mnt/adl-cache".to_string()),
    })
}

fn start_of_month(now: DateTime<Utc>) -> DateTime<Utc> {
    now.with_day(1)
        .and_then(|value| value.with_hour(0))
        .and_then(|value| value.with_minute(0))
        .and_then(|value| value.with_second(0))
        .and_then(|value| value.with_nanosecond(0))
        .unwrap_or(now)
}

fn build_remote_command_script(config: &AwsRemoteValidationConfig) -> String {
    let escaped_command = shell_single_quote(&config.command);
    let escaped_repo_url = shell_single_quote(&config.repo_url);
    let escaped_git_ref = shell_single_quote(&config.git_ref);
    let escaped_cache_bucket = shell_single_quote(config.cache_bucket.as_deref().unwrap_or(""));
    let escaped_cache_prefix = shell_single_quote(config.cache_prefix.as_deref().unwrap_or(""));
    let escaped_sccache_tarball_url =
        shell_single_quote(config.sccache_tarball_url.as_deref().unwrap_or(""));
    let escaped_nextest_tarball_url =
        shell_single_quote(config.nextest_tarball_url.as_deref().unwrap_or(""));
    let cache_volume_enabled = build_cache_volume_request(config).is_some();
    let escaped_cache_volume_device_name = shell_single_quote(
        config
            .cache_volume_device_name
            .as_deref()
            .unwrap_or("/dev/sdf"),
    );
    let escaped_cache_volume_mount_path = shell_single_quote(
        config
            .cache_volume_mount_path
            .as_deref()
            .unwrap_or("/mnt/adl-cache"),
    );
    let needs_nextest = if config.command.contains("nextest") {
        "1"
    } else {
        "0"
    };
    let cache_volume_enabled_flag = if cache_volume_enabled { "1" } else { "0" };
    format!(
        r#"set -euo pipefail
PROGRESS_ROOT="/tmp/adl-aws-remote-validation-bootstrap/{run_id}"
mkdir -p "$PROGRESS_ROOT"
BOOTSTRAP_START="$(date +%s)"
CURRENT_STAGE="bootstrap"

emit_debug_log() {{
  local label="$1"
  local path="$2"
  if [ -f "$path" ]; then
    local line_count
    line_count="$(wc -l < "$path" 2>/dev/null || echo 0)"
    echo "ADL_REMOTE_LOG_BEGIN:$label" >&2
    sed -n '1,160p' "$path" >&2 || true
    if [ "$line_count" -gt 160 ]; then
      echo "ADL_REMOTE_LOG_MIDDLE_ELIDED:$label:$line_count" >&2
      tail -n 160 "$path" >&2 || true
    fi
    echo "ADL_REMOTE_LOG_END:$label" >&2
  fi
}}

log_progress() {{
  local message="$1"
  local timestamp
  local log_root
  timestamp="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  log_root="${{RUN_ROOT:-$PROGRESS_ROOT}}"
  mkdir -p "$log_root"
  printf '%s %s\n' "$timestamp" "$message" | tee -a "$log_root/progress.log" >&2
}}

on_error() {{
  local exit_code="$?"
  echo "ADL_REMOTE_FAILURE_STAGE=$CURRENT_STAGE" >&2
  emit_debug_log rustup /tmp/adl-rustup.log
  emit_debug_log build_toolchain /tmp/adl-build-toolchain.log
  emit_debug_log sccache_install /tmp/adl-sccache-install.log
  emit_debug_log nextest_install /tmp/adl-nextest-install.log
  emit_debug_log git_clone /tmp/adl-git-clone.log
  emit_debug_log git_fetch /tmp/adl-git-fetch.log
  emit_debug_log git_checkout /tmp/adl-git-checkout.log
  emit_debug_log command_stdout "$RUN_ROOT/command.log"
  emit_debug_log command_stderr "$RUN_ROOT/command.err"
  emit_debug_log sccache_stats "$RUN_ROOT/sccache-stats.log"
  exit "$exit_code"
}}
trap on_error ERR

release_target_triple() {{
  local arch
  arch="$(uname -m)"
  case "$arch" in
    x86_64) printf '%s\n' "x86_64-unknown-linux-musl" ;;
    aarch64|arm64) printf '%s\n' "aarch64-unknown-linux-musl" ;;
    *) return 1 ;;
  esac
}}

install_github_release_binary() {{
  local repo_name binary_name target api_url asset_url archive_path extract_dir release_bin
  repo_name="$1"
  binary_name="$2"
  if [ -n "${{3:-}}" ]; then
    target="$3"
  else
    target="$(release_target_triple)" || return 1
  fi
  api_url="https://api.github.com/repos/$repo_name/releases/latest"
  asset_url="$(curl -fsSL "$api_url" | python3 -c 'import json, sys
repo = sys.argv[1]
binary = sys.argv[2]
target = sys.argv[3]
data = json.load(sys.stdin)
for asset in data.get("assets", []):
    url = asset.get("browser_download_url", "")
    if binary in url and target in url and url.endswith(".tar.gz"):
        print(url)
        break
' "$repo_name" "$binary_name" "$target")"
  [ -n "$asset_url" ] || return 1
  archive_path="/tmp/adl-$binary_name-release.tar.gz"
  extract_dir="/tmp/adl-$binary_name-release"
  curl -fsSL "$asset_url" -o "$archive_path"
  rm -rf "$extract_dir"
  mkdir -p "$extract_dir"
  tar -xzf "$archive_path" -C "$extract_dir"
  release_bin="$(find "$extract_dir" -type f -name "$binary_name" | head -n 1)"
  [ -n "$release_bin" ] || return 1
  install -m 0755 "$release_bin" "$HOME/.cargo/bin/$binary_name"
}}

install_sccache_release() {{
  local target
  target="$(release_target_triple)" || return 1
  case "$target" in
    x86_64-unknown-linux-musl) target="x86_64-unknown-linux-gnu" ;;
    aarch64-unknown-linux-musl) target="aarch64-unknown-linux-gnu" ;;
    *) return 1 ;;
  esac
  install_github_release_binary "mozilla/sccache" "sccache" "$target"
}}

ensure_aws_cli() {{
  if command -v aws >/dev/null 2>&1; then
    return 0
  fi
  sudo dnf install -y awscli >/tmp/adl-awscli-install.log 2>&1 \
    || sudo yum install -y awscli >/tmp/adl-awscli-install.log 2>&1
}}

archive_installed_binary() {{
  local binary_name archive_path package_dir
  binary_name="$1"
  archive_path="$2"
  package_dir="/tmp/adl-$binary_name-package"
  rm -rf "$package_dir"
  mkdir -p "$package_dir"
  cp "$HOME/.cargo/bin/$binary_name" "$package_dir/$binary_name"
  tar -czf "$archive_path" -C "$package_dir" "$binary_name"
}}

install_binary_from_tarball_url() {{
  local binary_name tarball_url extract_dir archive_path release_bin
  binary_name="$1"
  tarball_url="$2"
  [ -n "$tarball_url" ] || return 1
  archive_path="/tmp/adl-$binary_name-cache.tar.gz"
  extract_dir="/tmp/adl-$binary_name-cache"
  curl -fsSL "$tarball_url" -o "$archive_path"
  rm -rf "$extract_dir"
  mkdir -p "$extract_dir"
  tar -xzf "$archive_path" -C "$extract_dir"
  release_bin="$(find "$extract_dir" -type f -name "$binary_name" | head -n 1)"
  [ -n "$release_bin" ] || return 1
  install -m 0755 "$release_bin" "$HOME/.cargo/bin/$binary_name"
}}

install_binary_from_s3_cache() {{
  local binary_name bucket prefix object_uri archive_path tool_prefix
  binary_name="$1"
  bucket="$2"
  prefix="$3"
  [ -n "$bucket" ] || return 1
  ensure_aws_cli || return 1
  archive_path="/tmp/adl-$binary_name-cache.tar.gz"
  tool_prefix="$prefix/tools"
  object_uri="s3://$bucket/$tool_prefix/$binary_name.tar.gz"
  aws s3 cp "$object_uri" "$archive_path" >/tmp/adl-$binary_name-s3-download.log 2>&1 || return 1
  install_binary_from_tarball_url "$binary_name" "file://$archive_path"
}}

upload_binary_to_s3_cache() {{
  local binary_name bucket prefix archive_path object_uri tool_prefix
  binary_name="$1"
  bucket="$2"
  prefix="$3"
  [ -n "$bucket" ] || return 0
  ensure_aws_cli || return 1
  archive_path="/tmp/adl-$binary_name-upload.tar.gz"
  tool_prefix="$prefix/tools"
  object_uri="s3://$bucket/$tool_prefix/$binary_name.tar.gz"
  archive_installed_binary "$binary_name" "$archive_path" || return 1
  aws s3 cp "$archive_path" "$object_uri"
}}

verify_sccache_binary() {{
  command -v sccache >/dev/null 2>&1 || return 1
  sccache --version >/dev/null 2>&1 || return 1
  sccache --start-server >/dev/null 2>&1 || return 1
  sccache --zero-stats >/dev/null 2>&1 || return 1
}}

remove_installed_binary() {{
  local binary_name
  binary_name="$1"
  rm -f "$HOME/.cargo/bin/$binary_name"
}}

verify_nextest_binary() {{
  cargo nextest --version >/dev/null 2>&1
}}

install_nextest_release() {{
  local target
  target="$(release_target_triple)" || return 1
  case "$target" in
    x86_64-unknown-linux-musl) target="x86_64-unknown-linux-gnu" ;;
    aarch64-unknown-linux-musl) target="aarch64-unknown-linux-gnu" ;;
    *) return 1 ;;
  esac
  install_github_release_binary "nextest-rs/nextest" "cargo-nextest" "$target"
}}

export HOME="${{HOME:-/root}}"
CACHE_BUCKET='{cache_bucket}'
CACHE_PREFIX='{cache_prefix}'
SCCACHE_TARBALL_URL='{sccache_tarball_url}'
NEXTEST_TARBALL_URL='{nextest_tarball_url}'
CACHE_VOLUME_ENABLED="{cache_volume_enabled}"
CACHE_VOLUME_DEVICE_NAME='{cache_volume_device_name}'
CACHE_VOLUME_MOUNT_PATH='{cache_volume_mount_path}'
NEEDS_NEXTEST="{needs_nextest}"
if [ "$CACHE_VOLUME_ENABLED" = "1" ]; then
  CURRENT_STAGE="prepare_cache_volume"
  log_progress "stage=prepare_cache_volume"
  ROOT_SOURCE="$(findmnt -n -o SOURCE / || true)"
  ROOT_DISK="$(lsblk -no PKNAME "$ROOT_SOURCE" 2>/dev/null | head -n 1 || true)"
  resolve_cache_device() {{
    local attempt candidate basename
    for attempt in $(seq 1 60); do
      for candidate in "$CACHE_VOLUME_DEVICE_NAME" /dev/nvme1n1 /dev/nvme2n1 /dev/xvdf /dev/xvdg; do
        [ -b "$candidate" ] || continue
        basename="$(basename "$candidate")"
        if [ -n "$ROOT_DISK" ] && [ "$basename" = "$ROOT_DISK" ]; then
          continue
        fi
        readlink -f "$candidate" 2>/dev/null || printf '%s\n' "$candidate"
        return 0
      done
      sleep 2
    done
    return 1
  }}
  CACHE_DEVICE="$(resolve_cache_device)"
  sudo mkdir -p "$CACHE_VOLUME_MOUNT_PATH"
  if ! sudo blkid "$CACHE_DEVICE" >/dev/null 2>&1; then
    sudo mkfs.ext4 -F "$CACHE_DEVICE" >/tmp/adl-cache-volume-format.log 2>&1
  fi
  CACHE_UUID="$(sudo blkid -s UUID -o value "$CACHE_DEVICE")"
  if ! grep -q "$CACHE_UUID" /etc/fstab 2>/dev/null; then
    echo "UUID=$CACHE_UUID $CACHE_VOLUME_MOUNT_PATH ext4 defaults,nofail 0 2" | sudo tee -a /etc/fstab >/dev/null
  fi
  sudo mountpoint -q "$CACHE_VOLUME_MOUNT_PATH" || sudo mount "$CACHE_VOLUME_MOUNT_PATH"
  sudo chown -R "$USER":"$USER" "$CACHE_VOLUME_MOUNT_PATH"
  RUN_ROOT="$CACHE_VOLUME_MOUNT_PATH/adl-aws-remote-validation/{run_id}"
else
  RUN_ROOT="/tmp/adl-aws-remote-validation/{run_id}"
fi
REPO_DIR="$RUN_ROOT/agent-design-language"
TARGET_DIR="$RUN_ROOT/target"
SCCACHE_DIR="$RUN_ROOT/sccache"
mkdir -p "$RUN_ROOT" "$TARGET_DIR" "$SCCACHE_DIR"
CURRENT_STAGE="ensure_git"
log_progress "stage=ensure_git"
if ! command -v git >/dev/null 2>&1; then
  sudo dnf install -y git >/dev/null 2>&1 || sudo yum install -y git >/dev/null 2>&1 || true
fi
CURRENT_STAGE="ensure_curl"
log_progress "stage=ensure_curl"
if ! command -v curl >/dev/null 2>&1; then
  sudo dnf install -y curl >/dev/null 2>&1 || sudo yum install -y curl >/dev/null 2>&1 || true
fi
CURRENT_STAGE="ensure_build_toolchain"
log_progress "stage=ensure_build_toolchain"
if ! command -v cc >/dev/null 2>&1; then
  sudo dnf install -y gcc gcc-c++ make pkgconf-pkg-config openssl-devel >/tmp/adl-build-toolchain.log 2>&1 \
    || sudo yum install -y gcc gcc-c++ make pkgconfig openssl-devel >/tmp/adl-build-toolchain.log 2>&1
fi
CURRENT_STAGE="ensure_rustup"
log_progress "stage=ensure_rustup"
if ! command -v cargo >/dev/null 2>&1; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal >/tmp/adl-rustup.log 2>&1
fi
if [ -f "$HOME/.cargo/env" ]; then
  . "$HOME/.cargo/env"
fi
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR="$TARGET_DIR"
export SCCACHE_DIR="$SCCACHE_DIR"
if [ -n "$CACHE_BUCKET" ]; then
  export SCCACHE_BUCKET="$CACHE_BUCKET"
  export SCCACHE_REGION="{region}"
  export SCCACHE_S3_KEY_PREFIX="$CACHE_PREFIX/sccache"
fi
CURRENT_STAGE="ensure_sccache"
log_progress "stage=ensure_sccache"
if ! command -v sccache >/dev/null 2>&1; then
  SCCACHE_CACHE_HIT=0
  if install_binary_from_s3_cache sccache "$CACHE_BUCKET" "$CACHE_PREFIX" >/tmp/adl-sccache-install.log 2>&1 && verify_sccache_binary >>/tmp/adl-sccache-install.log 2>&1; then
    SCCACHE_CACHE_HIT=1
  elif install_binary_from_tarball_url sccache "$SCCACHE_TARBALL_URL" >>/tmp/adl-sccache-install.log 2>&1 && verify_sccache_binary >>/tmp/adl-sccache-install.log 2>&1; then
    SCCACHE_CACHE_HIT=1
  elif install_sccache_release >>/tmp/adl-sccache-install.log 2>&1 && verify_sccache_binary >>/tmp/adl-sccache-install.log 2>&1; then
    :
  else
    remove_installed_binary sccache
    cargo install sccache --locked --force >>/tmp/adl-sccache-install.log 2>&1
    verify_sccache_binary >>/tmp/adl-sccache-install.log 2>&1
  fi
  if [ "$SCCACHE_CACHE_HIT" -eq 0 ]; then
    upload_binary_to_s3_cache sccache "$CACHE_BUCKET" "$CACHE_PREFIX" >>/tmp/adl-sccache-install.log 2>&1 || true
  fi
fi
CURRENT_STAGE="ensure_nextest"
log_progress "stage=ensure_nextest"
if [ "$NEEDS_NEXTEST" = "1" ] && ! cargo nextest --version >/dev/null 2>&1; then
  NEXTEST_CACHE_HIT=0
  if install_binary_from_s3_cache cargo-nextest "$CACHE_BUCKET" "$CACHE_PREFIX" >/tmp/adl-nextest-install.log 2>&1 && verify_nextest_binary >>/tmp/adl-nextest-install.log 2>&1; then
    NEXTEST_CACHE_HIT=1
  elif install_binary_from_tarball_url cargo-nextest "$NEXTEST_TARBALL_URL" >>/tmp/adl-nextest-install.log 2>&1 && verify_nextest_binary >>/tmp/adl-nextest-install.log 2>&1; then
    NEXTEST_CACHE_HIT=1
  elif install_nextest_release >>/tmp/adl-nextest-install.log 2>&1 && verify_nextest_binary >>/tmp/adl-nextest-install.log 2>&1; then
    :
  else
    cargo install cargo-nextest --locked >>/tmp/adl-nextest-install.log 2>&1
    verify_nextest_binary >>/tmp/adl-nextest-install.log 2>&1
  fi
  if [ "$NEXTEST_CACHE_HIT" -eq 0 ]; then
    upload_binary_to_s3_cache cargo-nextest "$CACHE_BUCKET" "$CACHE_PREFIX" >>/tmp/adl-nextest-install.log 2>&1 || true
  fi
fi
export RUSTC_WRAPPER="sccache"

CURRENT_STAGE="clone_repo"
log_progress "stage=clone_repo"
if [ ! -d "$REPO_DIR/.git" ]; then
  git clone '{repo_url}' "$REPO_DIR" >/tmp/adl-git-clone.log 2>&1
fi
CURRENT_STAGE="fetch_repo"
log_progress "stage=fetch_repo"
git -C "$REPO_DIR" fetch --all --tags >/tmp/adl-git-fetch.log 2>&1
CURRENT_STAGE="checkout_ref"
log_progress "stage=checkout_ref ref={git_ref}"
git -C "$REPO_DIR" checkout '{git_ref}' >/tmp/adl-git-checkout.log 2>&1
RESOLVED_COMMIT="$(git -C "$REPO_DIR" rev-parse HEAD)"
RUSTC_VERSION="$(rustc --version 2>/dev/null || true)"
CARGO_VERSION="$(cargo --version 2>/dev/null || true)"
SCCACHE_VERSION="$(sccache --version 2>/dev/null || true)"
sccache --start-server >/dev/null 2>&1 || true
sccache --zero-stats >/dev/null 2>&1 || true
SCCACHE_DEGRADED=0
SCCACHE_DEGRADED_REASON=""

watch_sccache_health() {{
  while true; do
    if ! sccache --show-stats >/dev/null 2>&1; then
      printf '%s sccache_watch_restart\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$RUN_ROOT/sccache-watch.log"
      sccache --start-server >/dev/null 2>&1 || true
    fi
    sleep 5
  done
}}
watch_sccache_health >/tmp/adl-sccache-watch.log 2>&1 &
SCCACHE_WATCH_PID="$!"

INTERRUPTION_NOTICE=""
watch_spot_notice() {{
  while true; do
    TOKEN="$(curl -sS -X PUT http://169.254.169.254/latest/api/token -H 'X-aws-ec2-metadata-token-ttl-seconds: 60' || true)"
    if [ -n "$TOKEN" ]; then
      NOTICE="$(curl -fsS -H "X-aws-ec2-metadata-token: $TOKEN" http://169.254.169.254/latest/meta-data/spot/instance-action || true)"
    else
      NOTICE="$(curl -fsS http://169.254.169.254/latest/meta-data/spot/instance-action || true)"
    fi
    if [ -n "$NOTICE" ]; then
      printf '%s\n' "$NOTICE" > "$RUN_ROOT/spot-interruption.log"
      break
    fi
    sleep 5
  done
}}
watch_spot_notice >/tmp/adl-spot-watch.log 2>&1 &
WATCH_PID="$!"

BOOTSTRAP_END="$(date +%s)"
COMMAND_START="$(date +%s)"
CURRENT_STAGE="validation_command"
log_progress "stage=validation_command command={command}"
set +e
( cd "$REPO_DIR" && bash -lc '{command}' ) >"$RUN_ROOT/command.log" 2>"$RUN_ROOT/command.err"
COMMAND_EXIT="$?"
set -e
COMMAND_END="$(date +%s)"
kill "$WATCH_PID" >/dev/null 2>&1 || true
wait "$WATCH_PID" >/dev/null 2>&1 || true
kill "$SCCACHE_WATCH_PID" >/dev/null 2>&1 || true
wait "$SCCACHE_WATCH_PID" >/dev/null 2>&1 || true
sccache --show-stats >"$RUN_ROOT/sccache-stats.log" 2>&1 || true
[ -f "$RUN_ROOT/spot-interruption.log" ] && INTERRUPTION_NOTICE="$(cat "$RUN_ROOT/spot-interruption.log")"
if grep -Fq "sccache: warning: The server looks like it shut down unexpectedly" "$RUN_ROOT/command.err"; then
  SCCACHE_DEGRADED=1
  SCCACHE_DEGRADED_REASON="server_shut_down_unexpectedly"
elif grep -Fq "sccache: error:" "$RUN_ROOT/command.err"; then
  SCCACHE_DEGRADED=1
  SCCACHE_DEGRADED_REASON="client_or_server_error"
fi
if [ ! -s "$RUN_ROOT/sccache-stats.log" ]; then
  SCCACHE_DEGRADED=1
  if [ -z "$SCCACHE_DEGRADED_REASON" ]; then
    SCCACHE_DEGRADED_REASON="missing_stats"
  fi
fi

export ADL_RUN_ROOT="$RUN_ROOT"
export COMMAND_EXIT BOOTSTRAP_START BOOTSTRAP_END COMMAND_START COMMAND_END
export INTERRUPTION_NOTICE RESOLVED_COMMIT RUSTC_VERSION CARGO_VERSION SCCACHE_VERSION
export SCCACHE_DEGRADED SCCACHE_DEGRADED_REASON
python3 - <<'PY'
import json
import os
from pathlib import Path
run_root = Path(os.environ["ADL_RUN_ROOT"])
payload = {{
  "status": "passed" if int(os.environ["COMMAND_EXIT"]) == 0 else "failed",
  "bootstrap_seconds": int(os.environ["BOOTSTRAP_END"]) - int(os.environ["BOOTSTRAP_START"]),
  "command_seconds": int(os.environ["COMMAND_END"]) - int(os.environ["COMMAND_START"]),
  "interruption_detected": bool(os.environ.get("INTERRUPTION_NOTICE", "")),
  "interruption_notice": os.environ.get("INTERRUPTION_NOTICE") or None,
  "resolved_commit": os.environ.get("RESOLVED_COMMIT") or None,
  "rustc_version": os.environ.get("RUSTC_VERSION") or None,
  "cargo_version": os.environ.get("CARGO_VERSION") or None,
  "sccache_version": os.environ.get("SCCACHE_VERSION") or None,
  "sccache_degraded": os.environ.get("SCCACHE_DEGRADED") == "1",
  "sccache_degraded_reason": os.environ.get("SCCACHE_DEGRADED_REASON") or None,
  "sccache_stats": {{"raw_excerpt": run_root.joinpath("sccache-stats.log").read_text(errors="replace").splitlines()[:16] if run_root.joinpath("sccache-stats.log").exists() else []}}
}}
print("ADL_AWS_REMOTE_SUMMARY_BEGIN")
print(json.dumps(payload))
print("ADL_AWS_REMOTE_SUMMARY_END")
PY
cat "$RUN_ROOT/command.log"
cat "$RUN_ROOT/command.err" >&2
exit "$COMMAND_EXIT""#,
        run_id = config.run_id,
        repo_url = escaped_repo_url,
        git_ref = escaped_git_ref,
        cache_bucket = escaped_cache_bucket,
        cache_prefix = escaped_cache_prefix,
        sccache_tarball_url = escaped_sccache_tarball_url,
        nextest_tarball_url = escaped_nextest_tarball_url,
        cache_volume_enabled = cache_volume_enabled_flag,
        cache_volume_device_name = escaped_cache_volume_device_name,
        cache_volume_mount_path = escaped_cache_volume_mount_path,
        needs_nextest = needs_nextest,
        command = escaped_command,
        region = config.region,
    )
}

fn shell_single_quote(value: &str) -> String {
    value.replace('\'', r#"'"'"'"#)
}

fn extract_run_root(command: &str) -> Option<String> {
    let marker = "RUN_ROOT=\"";
    let start = command.find(marker)? + marker.len();
    let end = command[start..].find('"')? + start;
    Some(command[start..end].to_string())
}

fn temp_resource_name(issue: Option<u32>, run_id: &str, prefix: &str, max_len: usize) -> String {
    let normalized = run_id
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    let mut value = match issue {
        Some(issue_number) => format!("{prefix}-{issue_number}-{normalized}"),
        None => format!("{prefix}-{normalized}"),
    };
    while value.contains("--") {
        value = value.replace("--", "-");
    }
    value.truncate(max_len);
    value.trim_end_matches('-').to_string()
}

fn parse_remote_summary(text: &str) -> Option<RemoteCommandSummary> {
    let start = text.find("ADL_AWS_REMOTE_SUMMARY_BEGIN")?;
    let end = text.find("ADL_AWS_REMOTE_SUMMARY_END")?;
    if end <= start {
        return None;
    }
    let json_body = text[start + "ADL_AWS_REMOTE_SUMMARY_BEGIN".len()..end].trim();
    serde_json::from_str::<RemoteCommandSummary>(json_body).ok()
}

#[derive(Debug, Clone)]
pub struct PreparedLaunchSurface {
    pub record: LaunchSurfaceRecord,
    pub created_role_name: Option<String>,
    pub created_instance_profile_name: Option<String>,
}

pub struct LiveAwsRemoteValidationAdapter {
    ec2: ec2::Client,
    iam: iam::Client,
    ssm: ssm::Client,
    quotas: servicequotas::Client,
    costs: costexplorer::Client,
    budgets: budgets::Client,
    sts: sts::Client,
    ssh_debug: Option<SshDebugConfig>,
}

impl LiveAwsRemoteValidationAdapter {
    pub async fn new(config: &AwsRemoteValidationConfig) -> Result<Self> {
        let region_provider =
            RegionProviderChain::first_try(Some(aws_config::Region::new(config.region.clone())));
        let timeout_config = aws_config::timeout::TimeoutConfig::builder()
            .connect_timeout(Duration::from_secs(10))
            .operation_timeout(Duration::from_secs(60))
            .operation_attempt_timeout(Duration::from_secs(30))
            .build();
        let loader = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .timeout_config(timeout_config);
        let shared_config = if let Some(profile_name) = config.profile.as_deref() {
            loader.profile_name(profile_name).load().await
        } else {
            loader.load().await
        };
        let ssh_debug = build_ssh_debug_config(config)?;
        Ok(Self {
            ec2: ec2::Client::new(&shared_config),
            iam: iam::Client::new(&shared_config),
            ssm: ssm::Client::new(&shared_config),
            quotas: servicequotas::Client::new(&shared_config),
            costs: costexplorer::Client::new(&shared_config),
            budgets: budgets::Client::new(&shared_config),
            sts: sts::Client::new(&shared_config),
            ssh_debug,
        })
    }

    async fn instance_public_ip(&self, instance_id: &str) -> Result<Option<String>> {
        let resp = self
            .ec2
            .describe_instances()
            .instance_ids(instance_id)
            .send()
            .await?;
        Ok(resp
            .reservations()
            .first()
            .and_then(|reservation| reservation.instances().first())
            .and_then(|instance| instance.public_ip_address())
            .map(ToOwned::to_owned))
    }

    async fn subnet_availability_zone(&self, subnet_id: &str) -> Result<String> {
        let subnet = self
            .ec2
            .describe_subnets()
            .subnet_ids(subnet_id)
            .send()
            .await?;
        subnet
            .subnets()
            .first()
            .and_then(|entry| entry.availability_zone())
            .map(ToOwned::to_owned)
            .ok_or_else(|| anyhow!("subnet '{subnet_id}' did not report an availability zone"))
    }

    async fn ensure_cache_volume(
        &self,
        request: &CacheVolumeRequest,
        availability_zone: &str,
    ) -> std::result::Result<CacheVolumeRecord, AwsAdapterError> {
        let describe = self
            .ec2
            .describe_volumes()
            .filters(
                ec2::types::Filter::builder()
                    .name("tag:Name")
                    .values(request.name.clone())
                    .build(),
            )
            .filters(
                ec2::types::Filter::builder()
                    .name("availability-zone")
                    .values(availability_zone.to_string())
                    .build(),
            )
            .send()
            .await
            .map_err(classify_ec2_error)?;
        if let Some(volume) = describe.volumes().first() {
            let volume_id = volume.volume_id().unwrap_or_default().to_string();
            let state = volume
                .state()
                .map(|value| value.as_str().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            if state != "available" && state != "in-use" {
                return Err(AwsAdapterError {
                    code: Some("CacheVolumeNotAttachable".to_string()),
                    message: format!(
                        "cache volume {} is in state '{}' and cannot be reused",
                        volume_id, state
                    ),
                    spot_fallback_permitted: false,
                });
            }
            return Ok(CacheVolumeRecord {
                name: request.name.clone(),
                volume_id,
                availability_zone: availability_zone.to_string(),
                size_gib: volume.size().unwrap_or(request.size_gib),
                volume_type: volume
                    .volume_type()
                    .map(|value| value.as_str().to_string())
                    .unwrap_or_else(|| request.volume_type.clone()),
                iops: volume.iops(),
                throughput_mbps: volume.throughput(),
                device_name: request.device_name.clone(),
                mount_path: request.mount_path.clone(),
                created: false,
                attachment_state: state,
            });
        }

        let mut create = self
            .ec2
            .create_volume()
            .availability_zone(availability_zone)
            .size(request.size_gib)
            .volume_type(ec2::types::VolumeType::from(request.volume_type.as_str()))
            .tag_specifications(
                ec2::types::TagSpecification::builder()
                    .resource_type(ec2::types::ResourceType::Volume)
                    .tags(ec2::types::Tag::builder().key("Name").value(&request.name).build())
                    .build(),
            );
        if let Some(iops) = request.iops {
            create = create.iops(iops);
        }
        if let Some(throughput) = request.throughput_mbps {
            create = create.throughput(throughput);
        }
        let created = create.send().await.map_err(classify_ec2_error)?;
        let volume_id = created.volume_id().ok_or_else(|| AwsAdapterError {
            code: Some("CacheVolumeCreateMissingVolumeId".to_string()),
            message: "create_volume returned no volume id".to_string(),
            spot_fallback_permitted: false,
        })?;
        Ok(CacheVolumeRecord {
            name: request.name.clone(),
            volume_id: volume_id.to_string(),
            availability_zone: availability_zone.to_string(),
            size_gib: created.size().unwrap_or(request.size_gib),
            volume_type: created
                .volume_type()
                .map(|value: &ec2::types::VolumeType| value.as_str().to_string())
                .unwrap_or_else(|| request.volume_type.clone()),
            iops: created.iops(),
            throughput_mbps: created.throughput(),
            device_name: request.device_name.clone(),
            mount_path: request.mount_path.clone(),
            created: true,
            attachment_state: created
                .state()
                .map(|value: &ec2::types::VolumeState| value.as_str().to_string())
                .unwrap_or_else(|| "creating".to_string()),
        })
    }

    async fn wait_for_volume_available(
        &self,
        volume_id: &str,
    ) -> std::result::Result<(), AwsAdapterError> {
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(180) {
            let response = self
                .ec2
                .describe_volumes()
                .volume_ids(volume_id)
                .send()
                .await
                .map_err(classify_ec2_error)?;
            let Some(volume) = response.volumes().first() else {
                break;
            };
            let state = volume
                .state()
                .map(|value| value.as_str())
                .unwrap_or("unknown");
            if state == "available" || state == "in-use" {
                return Ok(());
            }
            sleep(Duration::from_secs(3)).await;
        }
        Err(AwsAdapterError {
            code: Some("CacheVolumeNotReady".to_string()),
            message: format!("cache volume {volume_id} did not become ready within 180s"),
            spot_fallback_permitted: false,
        })
    }

    async fn attach_cache_volume(
        &self,
        instance_id: &str,
        record: &mut CacheVolumeRecord,
    ) -> std::result::Result<(), AwsAdapterError> {
        self.wait_for_volume_available(&record.volume_id).await?;
        self.ec2
            .attach_volume()
            .device(record.device_name.clone())
            .instance_id(instance_id)
            .volume_id(record.volume_id.clone())
            .send()
            .await
            .map_err(classify_ec2_error)?;
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(180) {
            let response = self
                .ec2
                .describe_volumes()
                .volume_ids(record.volume_id.clone())
                .send()
                .await
                .map_err(classify_ec2_error)?;
            if let Some(state) = response
                .volumes()
                .first()
                .and_then(|volume| volume.attachments().first())
                .and_then(|attachment| attachment.state())
                .map(|value| value.as_str().to_string())
            {
                record.attachment_state = state.clone();
                if state == "attached" {
                    return Ok(());
                }
            }
            sleep(Duration::from_secs(3)).await;
        }
        Err(AwsAdapterError {
            code: Some("CacheVolumeAttachTimedOut".to_string()),
            message: format!(
                "cache volume {} did not attach to instance {} within 180s",
                record.volume_id, instance_id
            ),
            spot_fallback_permitted: false,
        })
    }

    async fn run_ssh_debug_repair_command(
        &self,
        instance_id: &str,
        poll_interval: Duration,
    ) -> std::result::Result<(), AwsAdapterError> {
        let commands = vec![
            "sudo systemctl enable --now sshd || sudo systemctl restart sshd || sudo service sshd restart || true".to_string(),
            "sudo systemctl is-active sshd || true".to_string(),
            "sudo ss -ltn | grep ':22 ' || true".to_string(),
        ];
        let output = self
            .ssm
            .send_command()
            .document_name("AWS-RunShellScript")
            .instance_ids(instance_id)
            .parameters("commands", commands)
            .send()
            .await
            .map_err(classify_ssm_error)?;
        let command_id = output
            .command()
            .and_then(|command| command.command_id())
            .unwrap_or_default()
            .to_string();
        append_command_status_line(
            "ssh_debug_repair_started",
            format!("instance_id={instance_id} command_id={command_id}"),
        );
        let start = Instant::now();
        loop {
            let invocation = self
                .ssm
                .get_command_invocation()
                .command_id(&command_id)
                .instance_id(instance_id)
                .send()
                .await
                .map_err(classify_ssm_error)?;
            let status = invocation
                .status_details()
                .or_else(|| invocation.status().map(|value| value.as_str()))
                .unwrap_or("Unknown")
                .to_string();
            append_command_status_line(
                "ssh_debug_repair_poll",
                format!(
                    "instance_id={instance_id} command_id={command_id} status={} response_code={}",
                    status,
                    invocation.response_code()
                ),
            );
            let terminal = matches!(
                status.as_str(),
                "Success" | "Cancelled" | "TimedOut" | "Failed" | "Cancelling"
            );
            if terminal {
                if invocation.response_code() == 0 && status.eq_ignore_ascii_case("Success") {
                    append_command_status_line(
                        "ssh_debug_repair_ready",
                        format!("instance_id={instance_id} command_id={command_id}"),
                    );
                    return Ok(());
                }
                return Err(AwsAdapterError {
                    code: Some("SshDebugRepairFailed".to_string()),
                    message: format!(
                        "ssh debug repair command {} finished with status={} response_code={}",
                        command_id,
                        status,
                        invocation.response_code()
                    ),
                    spot_fallback_permitted: false,
                });
            }
            if start.elapsed() >= Duration::from_secs(90) {
                return Err(AwsAdapterError {
                    code: Some("SshDebugRepairTimedOut".to_string()),
                    message: format!(
                        "ssh debug repair command {} did not reach terminal state within 90s",
                        command_id
                    ),
                    spot_fallback_permitted: false,
                });
            }
            sleep(poll_interval).await;
        }
    }

    async fn wait_for_ssh_debug_ready(
        &self,
        instance_id: &str,
        timeout: Duration,
        poll_interval: Duration,
    ) -> std::result::Result<(), AwsAdapterError> {
        let Some(ssh_debug) = self.ssh_debug.as_ref() else {
            return Ok(());
        };
        let Some(public_ip) =
            self.instance_public_ip(instance_id)
                .await
                .map_err(|err| AwsAdapterError {
                    code: Some("SshDebugPublicIpUnavailable".to_string()),
                    message: format!("failed to resolve public IP for ssh debug: {err}"),
                    spot_fallback_permitted: false,
                })?
        else {
            return Err(AwsAdapterError {
                code: Some("SshDebugPublicIpUnavailable".to_string()),
                message: format!("instance {instance_id} has no public IP for ssh debug"),
                spot_fallback_permitted: false,
            });
        };

        self.run_ssh_debug_repair_command(instance_id, poll_interval)
            .await?;

        let start = Instant::now();
        let mut attempt = 0_u32;
        let mut last_error = String::new();
        while start.elapsed() < timeout {
            attempt += 1;
            append_command_status_line(
                "ssh_debug_probe",
                format!("instance_id={instance_id} public_ip={public_ip} attempt={attempt}"),
            );
            let output = StdCommand::new("ssh")
                .arg("-o")
                .arg("BatchMode=yes")
                .arg("-o")
                .arg("StrictHostKeyChecking=no")
                .arg("-o")
                .arg("UserKnownHostsFile=/dev/null")
                .arg("-o")
                .arg("ConnectTimeout=10")
                .arg("-o")
                .arg("ServerAliveInterval=5")
                .arg("-o")
                .arg("ServerAliveCountMax=1")
                .arg("-i")
                .arg(&ssh_debug.private_key_path)
                .arg(format!("{}@{}", ssh_debug.user, public_ip))
                .arg("true")
                .output();
            match output {
                Ok(result) if result.status.success() => {
                    append_command_status_line(
                        "ssh_debug_ready",
                        format!(
                            "instance_id={instance_id} public_ip={public_ip} attempt={attempt}"
                        ),
                    );
                    return Ok(());
                }
                Ok(result) => {
                    let stderr_preview = String::from_utf8_lossy(&result.stderr)
                        .lines()
                        .take(4)
                        .collect::<Vec<_>>()
                        .join(" | ");
                    last_error = format!(
                        "exit_status={} stderr={}",
                        result
                            .status
                            .code()
                            .map(|code| code.to_string())
                            .unwrap_or_else(|| "signal".to_string()),
                        stderr_preview
                    );
                }
                Err(err) => {
                    last_error = err.to_string();
                }
            }
            if attempt == 1 || attempt % 3 == 0 {
                let _ = self
                    .run_ssh_debug_repair_command(instance_id, poll_interval)
                    .await;
            }
            sleep(poll_interval).await;
        }
        Err(AwsAdapterError {
            code: Some("SshDebugNotReady".to_string()),
            message: format!(
                "ssh debug channel to {} was not ready within {:?}: {}",
                public_ip,
                timeout,
                if last_error.is_empty() {
                    "no successful banner/auth exchange".to_string()
                } else {
                    last_error
                }
            ),
            spot_fallback_permitted: false,
        })
    }

    async fn start_ssh_tail(&self, instance_id: &str, run_root: &str) -> Result<Option<Child>> {
        let Some(ssh_debug) = self.ssh_debug.as_ref() else {
            return Ok(None);
        };
        let Some(public_ip) = self.instance_public_ip(instance_id).await? else {
            append_command_status_line(
                "ssh_tail_skip",
                format!("instance_id={instance_id} reason=no_public_ip"),
            );
            return Ok(None);
        };
        let Some(status_log_path) = LIVE_COMMAND_STATUS_LOG_PATH
            .lock()
            .ok()
            .and_then(|guard| guard.clone())
        else {
            return Ok(None);
        };
        let tail_log_path = status_log_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("remote-tail.log");
        let tail_log = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&tail_log_path)
            .with_context(|| format!("open remote tail log at {}", tail_log_path.display()))?;
        let tail_sink = Arc::new(Mutex::new(tail_log));
        let run_root_escaped = shell_single_quote(run_root);
        let progress_log_escaped = shell_single_quote(&format!("{run_root}/progress.log"));
        let command_log_escaped = shell_single_quote(&format!("{run_root}/command.log"));
        let command_err_escaped = shell_single_quote(&format!("{run_root}/command.err"));
        let remote_command = format!(
            "while [ ! -d {run_root} ]; do sleep 1; done; tail -n +1 -F {progress} {command_log} {command_err}",
            run_root = run_root_escaped,
            progress = progress_log_escaped,
            command_log = command_log_escaped,
            command_err = command_err_escaped,
        );
        let mut command = TokioCommand::new("ssh");
        command
            .arg("-o")
            .arg("StrictHostKeyChecking=no")
            .arg("-o")
            .arg("UserKnownHostsFile=/dev/null")
            .arg("-i")
            .arg(&ssh_debug.private_key_path)
            .arg(format!("{}@{}", ssh_debug.user, public_ip))
            .arg(remote_command)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        let mut child = command.spawn()?;
        if let Some(stdout) = child.stdout.take() {
            spawn_tail_pump(stdout, Arc::clone(&tail_sink));
        }
        if let Some(stderr) = child.stderr.take() {
            spawn_tail_pump(stderr, tail_sink);
        }
        append_command_status_line(
            "ssh_tail_started",
            format!(
                "instance_id={instance_id} public_ip={public_ip} tail_log={}",
                tail_log_path.display()
            ),
        );
        Ok(Some(child))
    }

    pub async fn prepare_launch_surface(
        &self,
        config: &AwsRemoteValidationConfig,
    ) -> Result<PreparedLaunchSurface> {
        let (vpc_id, subnet_id, availability_zone, mut notes) =
            self.resolve_vpc_and_subnet(config).await?;
        let ami_id = if config.ami_id.trim().is_empty() {
            notes.push("AMI id resolved from public Amazon Linux 2023 SSM parameter".to_string());
            self.resolve_default_ami().await?
        } else {
            config.ami_id.clone()
        };

        let (security_group_id, created_security_group_name, security_group_created) =
            if config.security_group_id.trim().is_empty() {
                let name = temp_resource_name(
                    config.issue,
                    &config.run_id,
                    "adl-aws-remote-validation-sg",
                    255,
                );
                let group_id = self.create_security_group(&vpc_id, &name, config).await?;
                notes.push("Created temporary security group for disposable builder".to_string());
                (group_id, Some(name), true)
            } else {
                (config.security_group_id.clone(), None, false)
            };
        self.ensure_ssh_debug_ingress(&security_group_id).await?;
        let ssh_allowed_cidr = self
            .ssh_debug
            .as_ref()
            .map(|config| config.allowed_cidr.clone());

        let (
            instance_profile_name,
            created_instance_profile_name,
            created_role_name,
            instance_profile_created,
        ) = if config.instance_profile_name.trim().is_empty() {
            let role_name = temp_resource_name(
                config.issue,
                &config.run_id,
                "ADLAwsRemoteValidationRole",
                64,
            );
            let profile_name = temp_resource_name(
                config.issue,
                &config.run_id,
                "ADLAwsRemoteValidationProfile",
                128,
            );
            self.create_instance_profile_with_ssm_role(&role_name, &profile_name, config)
                .await?;
            notes.push(
                "Created temporary IAM role and instance profile with AmazonSSMManagedInstanceCore"
                    .to_string(),
            );
            (
                profile_name.clone(),
                Some(profile_name),
                Some(role_name),
                true,
            )
        } else {
            (config.instance_profile_name.clone(), None, None, false)
        };

        Ok(PreparedLaunchSurface {
            record: LaunchSurfaceRecord {
                provisioning_mode: if security_group_created || instance_profile_created {
                    "ephemeral_aws_surface".to_string()
                } else {
                    "reviewed_baseline_inputs".to_string()
                },
                ami_id,
                ami_source: if config.ami_id.trim().is_empty() {
                    "ssm_public_parameter".to_string()
                } else {
                    "explicit_input".to_string()
                },
                vpc_id,
                subnet_id,
                availability_zone: Some(availability_zone),
                security_group_id,
                security_group_name: created_security_group_name.clone(),
                security_group_created,
                instance_profile_name,
                role_name: created_role_name.clone(),
                instance_profile_created,
                ssh_debug_enabled: self.ssh_debug.is_some(),
                ssh_allowed_cidr,
                notes,
            },
            created_role_name,
            created_instance_profile_name,
        })
    }

    pub async fn cleanup_launch_surface(
        &self,
        prepared: &PreparedLaunchSurface,
    ) -> LaunchSurfaceCleanupRecord {
        let mut cleanup = LaunchSurfaceCleanupRecord {
            security_group_deleted: None,
            instance_profile_deleted: None,
            role_deleted: None,
            notes: Vec::new(),
        };

        if let Some(profile_name) = prepared.created_instance_profile_name.as_deref() {
            if let Some(role_name) = prepared.created_role_name.as_deref() {
                match self
                    .iam
                    .remove_role_from_instance_profile()
                    .instance_profile_name(profile_name)
                    .role_name(role_name)
                    .send()
                    .await
                {
                    Ok(_) => cleanup.notes.push(format!(
                        "Removed role '{}' from instance profile '{}'",
                        role_name, profile_name
                    )),
                    Err(err) => cleanup.notes.push(format!(
                        "Failed removing role '{}' from profile '{}': {}",
                        role_name, profile_name, err
                    )),
                }
            }
            match self
                .iam
                .delete_instance_profile()
                .instance_profile_name(profile_name)
                .send()
                .await
            {
                Ok(_) => cleanup.instance_profile_deleted = Some(true),
                Err(err) => {
                    cleanup.instance_profile_deleted = Some(false);
                    cleanup.notes.push(format!(
                        "Failed deleting instance profile '{}': {}",
                        profile_name, err
                    ));
                }
            }
        }

        if let Some(role_name) = prepared.created_role_name.as_deref() {
            let _ = self
                .iam
                .delete_role_policy()
                .role_name(role_name)
                .policy_name(CACHE_ROLE_POLICY_NAME)
                .send()
                .await;
            let _ = self
                .iam
                .detach_role_policy()
                .role_name(role_name)
                .policy_arn("arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore")
                .send()
                .await;
            match self.iam.delete_role().role_name(role_name).send().await {
                Ok(_) => cleanup.role_deleted = Some(true),
                Err(err) => {
                    cleanup.role_deleted = Some(false);
                    cleanup
                        .notes
                        .push(format!("Failed deleting role '{}': {}", role_name, err));
                }
            }
        }

        if prepared.record.security_group_created {
            let mut deleted = false;
            let mut last_err = None;
            for _ in 0..3 {
                match self
                    .ec2
                    .delete_security_group()
                    .group_id(&prepared.record.security_group_id)
                    .send()
                    .await
                {
                    Ok(_) => {
                        deleted = true;
                        break;
                    }
                    Err(err) => {
                        last_err = Some(err.to_string());
                        sleep(Duration::from_secs(5)).await;
                    }
                }
            }
            if deleted {
                cleanup.security_group_deleted = Some(true);
            } else {
                cleanup.security_group_deleted = Some(false);
                cleanup.notes.push(format!(
                    "Failed deleting security group '{}': {}",
                    prepared.record.security_group_id,
                    last_err.unwrap_or_else(|| "unknown error".to_string())
                ));
            }
        }

        cleanup
    }

    async fn resolve_vpc_and_subnet(
        &self,
        config: &AwsRemoteValidationConfig,
    ) -> Result<(String, String, String, Vec<String>)> {
        let mut notes = Vec::new();
        if !config.subnet_id.trim().is_empty() {
            let subnet = self
                .ec2
                .describe_subnets()
                .subnet_ids(config.subnet_id.clone())
                .send()
                .await?;
            let subnet_entry = subnet
                .subnets()
                .first()
                .ok_or_else(|| anyhow!("subnet '{}' was not found", config.subnet_id))?;
            let vpc_id = subnet_entry
                .vpc_id()
                .ok_or_else(|| anyhow!("subnet '{}' did not report a VPC id", config.subnet_id))?
                .to_string();
            let availability_zone = subnet_entry
                .availability_zone()
                .ok_or_else(|| {
                    anyhow!("subnet '{}' did not report an availability zone", config.subnet_id)
                })?
                .to_string();
            return Ok((vpc_id, config.subnet_id.clone(), availability_zone, notes));
        }

        let vpc_id = self
            .ec2
            .describe_vpcs()
            .filters(
                ec2::types::Filter::builder()
                    .name("isDefault")
                    .values("true")
                    .build(),
            )
            .send()
            .await?
            .vpcs()
            .first()
            .and_then(|vpc| vpc.vpc_id())
            .map(ToOwned::to_owned)
            .ok_or_else(|| anyhow!("default VPC not found in target region"))?;
        let mut subnets = self
            .ec2
            .describe_subnets()
            .filters(
                ec2::types::Filter::builder()
                    .name("vpc-id")
                    .values(vpc_id.clone())
                    .build(),
            )
            .send()
            .await?
            .subnets()
            .iter()
            .filter_map(|subnet| {
                Some((
                    subnet.subnet_id()?.to_string(),
                    subnet.availability_zone()?.to_string(),
                ))
            })
            .collect::<Vec<_>>();
        subnets.sort();
        let (subnet_id, availability_zone) = subnets
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("no subnets found in default VPC '{}'", vpc_id))?;
        notes.push("Subnet auto-resolved from default VPC".to_string());
        Ok((vpc_id, subnet_id, availability_zone, notes))
    }

    async fn resolve_default_ami(&self) -> Result<String> {
        let parameter = self
            .ssm
            .get_parameter()
            .name("/aws/service/ami-amazon-linux-latest/al2023-ami-kernel-default-x86_64")
            .send()
            .await?;
        parameter
            .parameter()
            .and_then(|value| value.value())
            .map(ToOwned::to_owned)
            .ok_or_else(|| anyhow!("failed to resolve public Amazon Linux 2023 AMI id"))
    }

    async fn create_security_group(
        &self,
        vpc_id: &str,
        group_name: &str,
        config: &AwsRemoteValidationConfig,
    ) -> Result<String> {
        let output = self
            .ec2
            .create_security_group()
            .group_name(group_name)
            .description("ADL temporary remote validation security group")
            .vpc_id(vpc_id)
            .tag_specifications(
                ec2::types::TagSpecification::builder()
                    .resource_type(ec2::types::ResourceType::SecurityGroup)
                    .tags(
                        ec2::types::Tag::builder()
                            .key("Name")
                            .value(group_name)
                            .build(),
                    )
                    .tags(
                        ec2::types::Tag::builder()
                            .key("adl:issue")
                            .value(config.issue.unwrap_or_default().to_string())
                            .build(),
                    )
                    .tags(
                        ec2::types::Tag::builder()
                            .key("adl:run_id")
                            .value(&config.run_id)
                            .build(),
                    )
                    .build(),
            )
            .send()
            .await?;
        let group_id = output
            .group_id()
            .map(ToOwned::to_owned)
            .ok_or_else(|| anyhow!("create_security_group did not return a group id"))?;
        self.ensure_ssh_debug_ingress(&group_id).await?;
        Ok(group_id)
    }

    async fn ensure_ssh_debug_ingress(&self, group_id: &str) -> Result<()> {
        if let Some(ssh_debug) = self.ssh_debug.as_ref() {
            match self
                .ec2
                .authorize_security_group_ingress()
                .group_id(group_id)
                .ip_permissions(
                    ec2::types::IpPermission::builder()
                        .ip_protocol("tcp")
                        .from_port(22)
                        .to_port(22)
                        .ip_ranges(
                            ec2::types::IpRange::builder()
                                .cidr_ip(&ssh_debug.allowed_cidr)
                                .description("ADL SSH debug access")
                                .build(),
                        )
                        .build(),
                )
                .send()
                .await
            {
                Ok(_) => {}
                Err(err) => {
                    let code = err.code().unwrap_or_default().to_string();
                    let detail = err.to_string();
                    if code != "InvalidPermission.Duplicate"
                        && !detail.contains("InvalidPermission.Duplicate")
                    {
                        return Err(err.into());
                    }
                }
            }
        }
        Ok(())
    }

    async fn create_instance_profile_with_ssm_role(
        &self,
        role_name: &str,
        profile_name: &str,
        config: &AwsRemoteValidationConfig,
    ) -> Result<()> {
        let trust_policy = r#"{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": { "Service": "ec2.amazonaws.com" },
      "Action": "sts:AssumeRole"
    }
  ]
}"#;
        self.iam
            .create_role()
            .role_name(role_name)
            .assume_role_policy_document(trust_policy)
            .tags(
                iam::types::Tag::builder()
                    .key("adl:issue")
                    .value(config.issue.unwrap_or_default().to_string())
                    .build()?,
            )
            .tags(
                iam::types::Tag::builder()
                    .key("adl:run_id")
                    .value(&config.run_id)
                    .build()?,
            )
            .send()
            .await?;
        self.iam
            .attach_role_policy()
            .role_name(role_name)
            .policy_arn("arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore")
            .send()
            .await?;
        if let Some(cache_bucket) = config.cache_bucket.as_deref() {
            let cache_prefix = config
                .cache_prefix
                .as_deref()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("adl/aws-remote-validation");
            let policy_document = format!(
                r#"{{
  "Version": "2012-10-17",
  "Statement": [
    {{
      "Effect": "Allow",
      "Action": ["s3:ListBucket"],
      "Resource": "arn:aws:s3:::{bucket}",
      "Condition": {{
        "StringLike": {{
          "s3:prefix": [
            "{prefix}",
            "{prefix}/*"
          ]
        }}
      }}
    }},
    {{
      "Effect": "Allow",
      "Action": ["s3:GetObject", "s3:PutObject", "s3:AbortMultipartUpload"],
      "Resource": "arn:aws:s3:::{bucket}/{prefix}/*"
    }}
  ]
}}"#,
                bucket = cache_bucket,
                prefix = cache_prefix
            );
            self.iam
                .put_role_policy()
                .role_name(role_name)
                .policy_name(CACHE_ROLE_POLICY_NAME)
                .policy_document(policy_document)
                .send()
                .await?;
        }
        self.iam
            .create_instance_profile()
            .instance_profile_name(profile_name)
            .tags(
                iam::types::Tag::builder()
                    .key("adl:issue")
                    .value(config.issue.unwrap_or_default().to_string())
                    .build()?,
            )
            .tags(
                iam::types::Tag::builder()
                    .key("adl:run_id")
                    .value(&config.run_id)
                    .build()?,
            )
            .send()
            .await?;
        self.iam
            .add_role_to_instance_profile()
            .instance_profile_name(profile_name)
            .role_name(role_name)
            .send()
            .await?;
        sleep(Duration::from_secs(10)).await;
        Ok(())
    }
}

#[async_trait]
impl AwsRemoteValidationAdapter for LiveAwsRemoteValidationAdapter {
    async fn caller_identity(&self) -> Result<AwsAccountIdentity> {
        let output = self.sts.get_caller_identity().send().await?;
        let account_id = output.account().map(ToOwned::to_owned);
        Ok(AwsAccountIdentity {
            account_id_sha256: account_id.as_ref().map(|value| sha256_hex(value)),
            account_id,
            arn: output.arn().map(ToOwned::to_owned),
            user_id: output.user_id().map(ToOwned::to_owned),
        })
    }

    async fn quota_snapshot(&self) -> Result<QuotaSnapshot> {
        let mut spot_vcpu_quota = None;
        let mut on_demand_vcpu_quota = None;
        let mut notes = Vec::new();
        let resp = self
            .quotas
            .list_service_quotas()
            .service_code("ec2")
            .send()
            .await?;
        for quota in resp.quotas() {
            if let Some(name) = quota.quota_name() {
                if name == SPOT_QUOTA_NAME {
                    spot_vcpu_quota = quota.value();
                } else if name == ON_DEMAND_QUOTA_NAME {
                    on_demand_vcpu_quota = quota.value();
                }
            }
        }
        if spot_vcpu_quota.is_none() {
            notes.push(format!("spot quota '{}' not found", SPOT_QUOTA_NAME));
        }
        if on_demand_vcpu_quota.is_none() {
            notes.push(format!(
                "on-demand quota '{}' not found",
                ON_DEMAND_QUOTA_NAME
            ));
        }
        Ok(QuotaSnapshot {
            spot_vcpu_quota,
            on_demand_vcpu_quota,
            notes,
        })
    }

    async fn launch_instance(
        &self,
        spec: &LaunchSpec,
    ) -> std::result::Result<LaunchResult, AwsAdapterError> {
        let mut builder = self
            .ec2
            .run_instances()
            .image_id(&spec.ami_id)
            .instance_type(ec2::types::InstanceType::from(spec.instance_type.as_str()))
            .max_count(1)
            .min_count(1)
            .subnet_id(&spec.subnet_id)
            .iam_instance_profile(
                ec2::types::IamInstanceProfileSpecification::builder()
                    .name(&spec.instance_profile_name)
                    .build(),
            )
            .security_group_ids(spec.security_group_id.clone())
            .tag_specifications(
                ec2::types::TagSpecification::builder()
                    .resource_type(ec2::types::ResourceType::Instance)
                    .tags(
                        ec2::types::Tag::builder()
                            .key("Name")
                            .value("adl-aws-remote-validation")
                            .build(),
                    )
                    .build(),
            );
        if let Some(key_name) = spec.ssh_key_name.as_deref() {
            builder = builder.key_name(key_name);
        }
        if spec.purchase_option == PurchaseOption::Spot {
            builder = builder.instance_market_options(
                ec2::types::InstanceMarketOptionsRequest::builder()
                    .market_type(ec2::types::MarketType::Spot)
                    .spot_options(
                        ec2::types::SpotMarketOptions::builder()
                            .spot_instance_type(ec2::types::SpotInstanceType::OneTime)
                            .instance_interruption_behavior(
                                ec2::types::InstanceInterruptionBehavior::Terminate,
                            )
                            .build(),
                    )
                    .build(),
            );
        }
        let output = builder.send().await.map_err(classify_run_instances_error)?;
        let instance = output.instances().first().ok_or_else(|| AwsAdapterError {
            code: Some("MissingInstance".to_string()),
            message: "run_instances returned no instances".to_string(),
            spot_fallback_permitted: spec.purchase_option == PurchaseOption::Spot,
        })?;
        let instance_id = instance.instance_id().unwrap_or_default().to_string();
        let mut cache_volume = if let Some(request) = spec.cache_volume.as_ref() {
            let availability_zone =
                self.subnet_availability_zone(&spec.subnet_id)
                    .await
                    .map_err(|err| AwsAdapterError {
                        code: Some("CacheVolumeAvailabilityZoneUnavailable".to_string()),
                        message: err.to_string(),
                        spot_fallback_permitted: false,
                    })?;
            let mut record = self.ensure_cache_volume(request, &availability_zone).await?;
            self.attach_cache_volume(&instance_id, &mut record).await?;
            Some(record)
        } else {
            None
        };
        Ok(LaunchResult {
            instance_id,
            initial_state: instance
                .state()
                .and_then(|state| state.name())
                .map(|state| state.as_str().to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            cache_volume: cache_volume.take(),
        })
    }

    async fn wait_for_ssm_online(
        &self,
        instance_id: &str,
        timeout: Duration,
        poll_interval: Duration,
    ) -> std::result::Result<SsmReadyResult, AwsAdapterError> {
        let start = Instant::now();
        loop {
            let resp = self
                .ssm
                .describe_instance_information()
                .filters(
                    ssm::types::InstanceInformationStringFilter::builder()
                        .key("InstanceIds")
                        .values(instance_id)
                        .build()
                        .map_err(classify_ssm_error)?,
                )
                .send()
                .await
                .map_err(classify_ssm_error)?;
            if let Some(info) = resp.instance_information_list().first() {
                let status = info
                    .ping_status()
                    .map(|value| value.as_str())
                    .unwrap_or("Unknown");
                if status == "Online" {
                    append_command_status_line(
                        "ssm_online",
                        format!("instance_id={instance_id} ping_status={status}"),
                    );
                    return Ok(SsmReadyResult {
                        status: status.to_string(),
                    });
                }
                append_command_status_line(
                    "ssm_wait",
                    format!("instance_id={instance_id} ping_status={status}"),
                );
            }
            if start.elapsed() >= timeout {
                return Err(AwsAdapterError {
                    code: Some("SsmNotOnline".to_string()),
                    message: format!(
                        "instance {instance_id} did not become SSM-online within {:?}",
                        timeout
                    ),
                    spot_fallback_permitted: false,
                });
            }
            sleep(poll_interval).await;
        }
    }

    async fn run_remote_command(
        &self,
        instance_id: &str,
        command: &str,
        timeout: Option<Duration>,
        poll_interval: Duration,
    ) -> std::result::Result<CommandExecutionResult, AwsAdapterError> {
        let run_root = extract_run_root(command);
        if run_root.is_some() && self.ssh_debug.is_some() {
            self.wait_for_ssh_debug_ready(instance_id, Duration::from_secs(90), poll_interval)
                .await?;
        }
        let mut ssh_tail_child = match run_root {
            Some(run_root) => match self.start_ssh_tail(instance_id, &run_root).await {
                Ok(child) => child,
                Err(err) => {
                    return Err(AwsAdapterError {
                        code: Some("SshTailStartFailed".to_string()),
                        message: format!("failed to start ssh tail: {err}"),
                        spot_fallback_permitted: false,
                    });
                }
            },
            None => None,
        };
        let output = self
            .ssm
            .send_command()
            .document_name("AWS-RunShellScript")
            .instance_ids(instance_id)
            .parameters("commands", vec![command.to_string()])
            .send()
            .await
            .map_err(classify_ssm_error)?;
        let command_id = output
            .command()
            .and_then(|command| command.command_id())
            .unwrap_or_default()
            .to_string();
        append_command_status_line(
            "command_sent",
            format!("instance_id={instance_id} command_id={command_id}"),
        );
        let start = Instant::now();
        loop {
            let invocation = match self
                .ssm
                .get_command_invocation()
                .command_id(&command_id)
                .instance_id(instance_id)
                .send()
                .await
            {
                Ok(invocation) => invocation,
                Err(err) => {
                    let detail = err.to_string();
                    let propagation_window = start.elapsed() < Duration::from_secs(90);
                    if propagation_window {
                        append_command_status_line(
                            "command_poll_retry",
                            format!(
                                "instance_id={instance_id} command_id={command_id} detail={detail}"
                            ),
                        );
                        sleep(poll_interval).await;
                        continue;
                    }
                    return Err(classify_ssm_error(err));
                }
            };
            let status = invocation
                .status_details()
                .or_else(|| invocation.status().map(|value| value.as_str()))
                .unwrap_or("Unknown")
                .to_string();
            append_command_status_line(
                "command_poll",
                format!(
                    "instance_id={instance_id} command_id={command_id} status={} response_code={}",
                    status,
                    invocation.response_code()
                ),
            );
            let terminal = matches!(
                status.as_str(),
                "Success" | "Cancelled" | "TimedOut" | "Failed" | "Cancelling"
            );
            if terminal {
                if let Some(child) = ssh_tail_child.as_mut() {
                    let _ = child.start_kill();
                }
                return Ok(CommandExecutionResult {
                    command_id,
                    status,
                    response_code: Some(invocation.response_code()),
                    stdout: invocation
                        .standard_output_content()
                        .unwrap_or_default()
                        .to_string(),
                    stderr: invocation
                        .standard_error_content()
                        .unwrap_or_default()
                        .to_string(),
                });
            }
            let instance_state = self.instance_state(instance_id).await?;
            if matches!(
                instance_state.as_deref(),
                Some("shutting-down") | Some("terminated") | Some("stopping") | Some("stopped")
            ) {
                if let Some(child) = ssh_tail_child.as_mut() {
                    let _ = child.start_kill();
                }
                return Err(AwsAdapterError {
                    code: Some("InstanceUnavailableDuringCommand".to_string()),
                    message: format!(
                        "instance {instance_id} entered state '{}' while command {command_id} was still running",
                        instance_state.unwrap_or_else(|| "unknown".to_string())
                    ),
                    spot_fallback_permitted: false,
                });
            }
            if let Some(timeout) = timeout {
                if start.elapsed() >= timeout {
                    if let Some(child) = ssh_tail_child.as_mut() {
                        let _ = child.start_kill();
                    }
                    return Err(AwsAdapterError {
                        code: Some("CommandTimedOut".to_string()),
                        message: format!(
                            "command {command_id} did not reach terminal state within {:?}",
                            timeout
                        ),
                        spot_fallback_permitted: false,
                    });
                }
            }
            sleep(poll_interval).await;
        }
    }

    async fn instance_state(
        &self,
        instance_id: &str,
    ) -> std::result::Result<Option<String>, AwsAdapterError> {
        let resp = self
            .ec2
            .describe_instances()
            .instance_ids(instance_id)
            .send()
            .await
            .map_err(classify_ec2_error)?;
        let state = resp
            .reservations()
            .first()
            .and_then(|reservation| reservation.instances().first())
            .and_then(|instance| instance.state())
            .and_then(|state| state.name())
            .map(|name| name.as_str().to_string());
        Ok(state)
    }

    async fn terminate_instance(
        &self,
        instance_id: &str,
    ) -> std::result::Result<(), AwsAdapterError> {
        self.ec2
            .terminate_instances()
            .instance_ids(instance_id)
            .send()
            .await
            .map_err(classify_ec2_error)?;
        Ok(())
    }

    async fn wait_for_termination(
        &self,
        instance_id: &str,
        timeout: Duration,
        poll_interval: Duration,
    ) -> std::result::Result<Option<String>, AwsAdapterError> {
        let start = Instant::now();
        loop {
            let state = self.instance_state(instance_id).await?;
            if matches!(state.as_deref(), Some("terminated")) {
                return Ok(state);
            }
            if start.elapsed() >= timeout {
                return Err(AwsAdapterError {
                    code: Some("TerminationTimeout".to_string()),
                    message: format!(
                        "instance {instance_id} did not terminate within {:?}",
                        timeout
                    ),
                    spot_fallback_permitted: false,
                });
            }
            sleep(poll_interval).await;
        }
    }

    async fn spot_termination_evidence(
        &self,
        instance_id: &str,
    ) -> Result<Option<SpotTerminationEvidence>> {
        let resp = self
            .ec2
            .describe_instances()
            .instance_ids(instance_id)
            .send()
            .await?;
        let Some(instance) = resp
            .reservations()
            .first()
            .and_then(|reservation| reservation.instances().first())
        else {
            return Ok(None);
        };
        let instance_lifecycle = instance
            .instance_lifecycle()
            .map(|v| v.as_str().to_string());
        let state_reason_code = instance
            .state_reason()
            .and_then(|reason| reason.code())
            .map(ToOwned::to_owned);
        let state_reason_message = instance
            .state_reason()
            .and_then(|reason| reason.message())
            .map(ToOwned::to_owned);
        let state_transition_reason = instance.state_transition_reason().map(ToOwned::to_owned);
        let spot_instance_request_id = instance.spot_instance_request_id().map(ToOwned::to_owned);
        let mut spot_request_status_code = None;
        let mut spot_request_status_message = None;
        if let Some(request_id) = spot_instance_request_id.as_deref() {
            let requests = self
                .ec2
                .describe_spot_instance_requests()
                .spot_instance_request_ids(request_id)
                .send()
                .await?;
            if let Some(request) = requests.spot_instance_requests().first() {
                spot_request_status_code = request
                    .status()
                    .and_then(|status| status.code())
                    .map(ToOwned::to_owned);
                spot_request_status_message = request
                    .status()
                    .and_then(|status| status.message())
                    .map(ToOwned::to_owned);
            }
        }
        let provider_interruption_confirmed = matches!(
            state_reason_code.as_deref(),
            Some("Server.SpotInstanceTermination")
        ) || matches!(
            spot_request_status_code.as_deref(),
            Some("instance-terminated-no-capacity")
                | Some("instance-terminated-by-price")
                | Some("instance-terminated-by-service")
        );
        Ok(Some(SpotTerminationEvidence {
            instance_id: instance_id.to_string(),
            instance_lifecycle,
            state_reason_code,
            state_reason_message,
            state_transition_reason,
            spot_instance_request_id,
            spot_request_status_code,
            spot_request_status_message,
            provider_interruption_confirmed,
        }))
    }

    async fn cost_snapshot(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Option<CostExplorerSnapshot>> {
        let output = self
            .costs
            .get_cost_and_usage()
            .time_period(
                costexplorer::types::DateInterval::builder()
                    .start(start.format("%Y-%m-%d").to_string())
                    .end(end.format("%Y-%m-%d").to_string())
                    .build()
                    .map_err(|err| anyhow!("failed to build Cost Explorer date interval: {err}"))?,
            )
            .granularity(costexplorer::types::Granularity::Monthly)
            .metrics("UnblendedCost")
            .filter(
                costexplorer::types::Expression::builder()
                    .dimensions(
                        costexplorer::types::DimensionValues::builder()
                            .key(costexplorer::types::Dimension::Service)
                            .values("Amazon Elastic Compute Cloud - Compute")
                            .build(),
                    )
                    .build(),
            )
            .send()
            .await?;
        let amount = output
            .results_by_time()
            .first()
            .and_then(|row| row.total())
            .and_then(|totals| totals.get("UnblendedCost"))
            .and_then(|metric| metric.amount().map(ToOwned::to_owned));
        let unit = output
            .results_by_time()
            .first()
            .and_then(|row| row.total())
            .and_then(|totals| totals.get("UnblendedCost"))
            .and_then(|metric| metric.unit().map(ToOwned::to_owned));
        Ok(Some(CostExplorerSnapshot {
            start: start.to_rfc3339(),
            end: end.to_rfc3339(),
            service: "Amazon Elastic Compute Cloud - Compute".to_string(),
            amount,
            unit,
            delayed_billing_boundary: true,
            note: "Cost Explorer reflects account spend on AWS delay; treat this as supporting cost evidence rather than exact per-run billing.".to_string(),
        }))
    }

    async fn budget_snapshot(&self, budget_name: &str) -> Result<Option<BudgetSnapshot>> {
        let account_id = self
            .caller_identity()
            .await?
            .account_id
            .ok_or_else(|| anyhow!("caller identity did not include account id"))?;
        let output = self
            .budgets
            .describe_budget()
            .account_id(account_id)
            .budget_name(budget_name)
            .send()
            .await?;
        let budget = match output.budget() {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(BudgetSnapshot {
            budget_name: budget_name.to_string(),
            limit_amount: budget
                .budget_limit()
                .map(|limit| limit.amount())
                .map(ToOwned::to_owned),
            limit_unit: budget
                .budget_limit()
                .map(|limit| limit.unit())
                .map(ToOwned::to_owned),
            actual_spend_amount: budget
                .calculated_spend()
                .and_then(|spend| spend.actual_spend())
                .map(|spend| spend.amount())
                .map(ToOwned::to_owned),
            actual_spend_unit: budget
                .calculated_spend()
                .and_then(|spend| spend.actual_spend())
                .map(|spend| spend.unit())
                .map(ToOwned::to_owned),
            forecast_spend_amount: budget
                .calculated_spend()
                .and_then(|spend| spend.forecasted_spend())
                .map(|spend| spend.amount())
                .map(ToOwned::to_owned),
            forecast_spend_unit: budget
                .calculated_spend()
                .and_then(|spend| spend.forecasted_spend())
                .map(|spend| spend.unit())
                .map(ToOwned::to_owned),
        }))
    }
}

fn classify_run_instances_error(
    err: ec2::error::SdkError<ec2::operation::run_instances::RunInstancesError>,
) -> AwsAdapterError {
    let fallback_message = err.to_string();
    let service_message = err
        .as_service_error()
        .and_then(|service_error| service_error.message())
        .map(ToOwned::to_owned);
    let code = err
        .as_service_error()
        .and_then(|service_error| service_error.code())
        .map(ToOwned::to_owned)
        .or_else(|| {
            fallback_message
                .split(':')
                .next()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
        });
    let message = service_message.unwrap_or(fallback_message);
    let fallbackable = matches!(
        code.as_deref(),
        Some("MaxSpotInstanceCountExceeded")
            | Some("InsufficientInstanceCapacity")
            | Some("UnfulfillableCapacity")
            | Some("SpotMaxPriceTooLow")
            | Some("RequestLimitExceeded")
    ) || message.contains("MaxSpotInstanceCountExceeded")
        || message.contains("InsufficientInstanceCapacity")
        || message.contains("UnfulfillableCapacity")
        || message.contains("RequestLimitExceeded")
        || message.contains("vcpu limit")
        || message.contains("vCPU limit")
        || message.contains("Spot")
        || message.contains("capacity");
    AwsAdapterError {
        code,
        message,
        spot_fallback_permitted: fallbackable,
    }
}

fn classify_ssm_error<E: fmt::Display>(err: E) -> AwsAdapterError {
    AwsAdapterError {
        code: Some("SsmError".to_string()),
        message: err.to_string(),
        spot_fallback_permitted: false,
    }
}

fn classify_ec2_error<E: fmt::Display>(err: E) -> AwsAdapterError {
    AwsAdapterError {
        code: Some("Ec2Error".to_string()),
        message: err.to_string(),
        spot_fallback_permitted: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::sync::Mutex;

    struct FakeAdapter {
        quota: QuotaSnapshot,
        identity: AwsAccountIdentity,
        launch_results: Mutex<VecDeque<std::result::Result<LaunchResult, AwsAdapterError>>>,
        ssm_ready: std::result::Result<SsmReadyResult, AwsAdapterError>,
        command_result: std::result::Result<CommandExecutionResult, AwsAdapterError>,
        terminate_result: std::result::Result<(), AwsAdapterError>,
        final_state: std::result::Result<Option<String>, AwsAdapterError>,
        cost: Option<CostExplorerSnapshot>,
        budget: Option<BudgetSnapshot>,
    }

    #[async_trait]
    impl AwsRemoteValidationAdapter for FakeAdapter {
        async fn caller_identity(&self) -> Result<AwsAccountIdentity> {
            Ok(self.identity.clone())
        }

        async fn quota_snapshot(&self) -> Result<QuotaSnapshot> {
            Ok(self.quota.clone())
        }

        async fn launch_instance(
            &self,
            _spec: &LaunchSpec,
        ) -> std::result::Result<LaunchResult, AwsAdapterError> {
            self.launch_results
                .lock()
                .expect("launch results mutex")
                .pop_front()
                .expect("launch result")
        }

        async fn wait_for_ssm_online(
            &self,
            _instance_id: &str,
            _timeout: Duration,
            _poll_interval: Duration,
        ) -> std::result::Result<SsmReadyResult, AwsAdapterError> {
            self.ssm_ready.clone()
        }

        async fn run_remote_command(
            &self,
            _instance_id: &str,
            _command: &str,
            _timeout: Option<Duration>,
            _poll_interval: Duration,
        ) -> std::result::Result<CommandExecutionResult, AwsAdapterError> {
            self.command_result.clone()
        }

        async fn instance_state(
            &self,
            _instance_id: &str,
        ) -> std::result::Result<Option<String>, AwsAdapterError> {
            self.final_state.clone()
        }

        async fn terminate_instance(
            &self,
            _instance_id: &str,
        ) -> std::result::Result<(), AwsAdapterError> {
            self.terminate_result.clone()
        }

        async fn wait_for_termination(
            &self,
            _instance_id: &str,
            _timeout: Duration,
            _poll_interval: Duration,
        ) -> std::result::Result<Option<String>, AwsAdapterError> {
            self.final_state.clone()
        }

        async fn cost_snapshot(
            &self,
            _start: DateTime<Utc>,
            _end: DateTime<Utc>,
        ) -> Result<Option<CostExplorerSnapshot>> {
            Ok(self.cost.clone())
        }

        async fn budget_snapshot(&self, _budget_name: &str) -> Result<Option<BudgetSnapshot>> {
            Ok(self.budget.clone())
        }

        async fn spot_termination_evidence(
            &self,
            _instance_id: &str,
        ) -> Result<Option<SpotTerminationEvidence>> {
            Ok(None)
        }
    }

    fn sample_config(tmp: &Path) -> AwsRemoteValidationConfig {
        AwsRemoteValidationConfig {
            issue: Some(4603),
            run_id: "test-run".to_string(),
            region: "us-west-2".to_string(),
            profile: Some("agent-logic-admin".to_string()),
            repo_url: "https://github.com/danielbaustin/agent-design-language.git".to_string(),
            git_ref: "origin/main".to_string(),
            cache_bucket: Some("adl-aws-remote-tool-cache-agentlogic".to_string()),
            cache_prefix: Some("adl/remote-validation/4603".to_string()),
            sccache_tarball_url: None,
            nextest_tarball_url: None,
            ssh_key_name: None,
            ssh_private_key_path: None,
            ssh_user: None,
            ssh_allowed_cidr: None,
            cache_volume_name: None,
            cache_volume_size_gib: None,
            cache_volume_type: None,
            cache_volume_iops: None,
            cache_volume_throughput_mbps: None,
            cache_volume_device_name: None,
            cache_volume_mount_path: None,
            command:
                "cargo test --manifest-path tools/aws_remote_validation/Cargo.toml --bin adl-aws-remote-validation -- --nocapture"
                    .to_string(),
            out_path: tmp.join("summary.json"),
            artifact_dir: tmp.join("artifacts"),
            ami_id: "ami-test".to_string(),
            subnet_id: "subnet-test".to_string(),
            security_group_id: "sg-test".to_string(),
            instance_profile_name: "profile-test".to_string(),
            instance_types: vec!["c7i.large".to_string()],
            budget_name: Some("Agent Logic Monthly".to_string()),
            expected_max_cost_usd: Some(20.0),
            poll_interval_seconds: 1,
            ssm_ready_timeout_seconds: 10,
            command_timeout_seconds: Some(20),
            termination_timeout_seconds: 10,
        }
    }

    #[tokio::test]
    async fn remote_validation_prefers_spot_and_records_success() {
        let tmp =
            std::env::temp_dir().join(format!("adl-aws-remote-validation-{}", std::process::id()));
        let remote_json = serde_json::json!({
            "status": "passed",
            "bootstrap_seconds": 12,
            "command_seconds": 34,
            "interruption_detected": false,
            "interruption_notice": null,
            "resolved_commit": "abc123",
            "rustc_version": "rustc fixture",
            "cargo_version": "cargo fixture",
            "sccache_version": "sccache fixture",
            "sccache_degraded": false,
            "sccache_degraded_reason": null,
            "sccache_stats": {
                "compile_requests": "10",
                "compile_requests_executed": "4",
                "cache_hits": "6",
                "cache_misses": "4",
                "raw_excerpt": ["Compile requests 10", "Cache hits 6"]
            }
        });
        let stdout = format!(
            "ADL_AWS_REMOTE_SUMMARY_BEGIN\n{}\nADL_AWS_REMOTE_SUMMARY_END\ncommand output",
            remote_json
        );
        let adapter = FakeAdapter {
            quota: QuotaSnapshot {
                spot_vcpu_quota: Some(32.0),
                on_demand_vcpu_quota: Some(64.0),
                notes: vec![],
            },
            identity: AwsAccountIdentity {
                account_id: Some("123456789012".to_string()),
                account_id_sha256: Some("hash".to_string()),
                arn: Some("arn:aws:sts::123456789012:assumed-role/example".to_string()),
                user_id: Some("AIDAEXAMPLE".to_string()),
            },
            launch_results: Mutex::new(VecDeque::from(vec![Ok(LaunchResult {
                instance_id: "i-1234567890".to_string(),
                initial_state: "pending".to_string(),
                cache_volume: None,
            })])),
            ssm_ready: Ok(SsmReadyResult {
                status: "Online".to_string(),
            }),
            command_result: Ok(CommandExecutionResult {
                command_id: "cmd-1".to_string(),
                status: "Success".to_string(),
                response_code: Some(0),
                stdout,
                stderr: String::new(),
            }),
            terminate_result: Ok(()),
            final_state: Ok(Some("terminated".to_string())),
            cost: None,
            budget: None,
        };
        let (summary, events) = run_aws_remote_validation(&adapter, &sample_config(&tmp))
            .await
            .expect("summary");
        assert_eq!(summary.status, RemoteRunStatus::Passed);
        assert_eq!(
            summary
                .launch
                .as_ref()
                .map(|launch| launch.purchase_option.clone()),
            Some(PurchaseOption::Spot)
        );
        assert!(summary.remote_summary.is_some());
        assert!(events
            .iter()
            .any(|event| event.stage == "remote_command" && event.status == "ok"));
    }

    #[tokio::test]
    async fn remote_validation_falls_back_to_on_demand_after_spot_capacity_failure() {
        let tmp = std::env::temp_dir().join(format!(
            "adl-aws-remote-validation-fallback-{}",
            std::process::id()
        ));
        let adapter = FakeAdapter {
            quota: QuotaSnapshot {
                spot_vcpu_quota: Some(32.0),
                on_demand_vcpu_quota: Some(64.0),
                notes: vec![],
            },
            identity: AwsAccountIdentity {
                account_id: Some("123456789012".to_string()),
                account_id_sha256: Some("hash".to_string()),
                arn: None,
                user_id: None,
            },
            launch_results: Mutex::new(VecDeque::from(vec![
                Err(AwsAdapterError {
                    code: Some("InsufficientInstanceCapacity".to_string()),
                    message: "InsufficientInstanceCapacity: no spot capacity".to_string(),
                    spot_fallback_permitted: true,
                }),
                Ok(LaunchResult {
                    instance_id: "i-fallback".to_string(),
                    initial_state: "pending".to_string(),
                    cache_volume: None,
                }),
            ])),
            ssm_ready: Ok(SsmReadyResult {
                status: "Online".to_string(),
            }),
            command_result: Ok(CommandExecutionResult {
                command_id: "cmd-2".to_string(),
                status: "Success".to_string(),
                response_code: Some(0),
                stdout: "ADL_AWS_REMOTE_SUMMARY_BEGIN\n{\"status\":\"passed\",\"bootstrap_seconds\":1,\"command_seconds\":2,\"interruption_detected\":false,\"interruption_notice\":null,\"resolved_commit\":\"abc\",\"rustc_version\":null,\"cargo_version\":null,\"sccache_version\":null,\"sccache_degraded\":false,\"sccache_degraded_reason\":null,\"sccache_stats\":null}\nADL_AWS_REMOTE_SUMMARY_END\n".to_string(),
                stderr: String::new(),
            }),
            terminate_result: Ok(()),
            final_state: Ok(Some("terminated".to_string())),
            cost: None,
            budget: None,
        };

        let (summary, _) = run_aws_remote_validation(&adapter, &sample_config(&tmp))
            .await
            .expect("summary");

        assert_eq!(summary.status, RemoteRunStatus::Passed);
        assert_eq!(
            summary
                .launch
                .as_ref()
                .map(|launch| launch.purchase_option.clone()),
            Some(PurchaseOption::OnDemand)
        );
        assert_eq!(summary.attempts.len(), 2);
        assert_eq!(summary.attempts[0].purchase_option, PurchaseOption::Spot);
        assert_eq!(summary.attempts[0].status, "failed");
        assert_eq!(
            summary.attempts[1].purchase_option,
            PurchaseOption::OnDemand
        );
        assert_eq!(summary.attempts[1].status, "launched");
    }

    #[tokio::test]
    async fn remote_validation_classifies_spot_interruptions_truthfully() {
        let tmp = std::env::temp_dir().join(format!(
            "adl-aws-remote-validation-interruption-{}",
            std::process::id()
        ));
        let stdout = "ADL_AWS_REMOTE_SUMMARY_BEGIN\n{\"status\":\"failed\",\"bootstrap_seconds\":1,\"command_seconds\":2,\"interruption_detected\":true,\"interruption_notice\":\"{\\\"action\\\":\\\"terminate\\\",\\\"time\\\":\\\"2026-07-01T20:00:00Z\\\"}\",\"resolved_commit\":\"abc\",\"rustc_version\":null,\"cargo_version\":null,\"sccache_version\":null,\"sccache_degraded\":false,\"sccache_degraded_reason\":null,\"sccache_stats\":null}\nADL_AWS_REMOTE_SUMMARY_END\n";
        let adapter = FakeAdapter {
            quota: QuotaSnapshot {
                spot_vcpu_quota: Some(32.0),
                on_demand_vcpu_quota: Some(64.0),
                notes: vec![],
            },
            identity: AwsAccountIdentity {
                account_id: Some("123456789012".to_string()),
                account_id_sha256: Some("hash".to_string()),
                arn: None,
                user_id: None,
            },
            launch_results: Mutex::new(VecDeque::from(vec![Ok(LaunchResult {
                instance_id: "i-interrupted".to_string(),
                initial_state: "pending".to_string(),
                cache_volume: None,
            })])),
            ssm_ready: Ok(SsmReadyResult {
                status: "Online".to_string(),
            }),
            command_result: Ok(CommandExecutionResult {
                command_id: "cmd-3".to_string(),
                status: "Failed".to_string(),
                response_code: Some(1),
                stdout: stdout.to_string(),
                stderr: String::new(),
            }),
            terminate_result: Ok(()),
            final_state: Ok(Some("terminated".to_string())),
            cost: None,
            budget: None,
        };

        let (summary, events) = run_aws_remote_validation(&adapter, &sample_config(&tmp))
            .await
            .expect("summary");

        assert_eq!(summary.status, RemoteRunStatus::InterruptedByAws);
        assert!(summary
            .failure_reason
            .as_deref()
            .is_some_and(|reason| reason.contains("interruption")));
        assert!(events
            .iter()
            .any(|event| event.stage == "remote_command" && event.status == "interrupted"));
    }

    #[test]
    fn build_remote_command_script_uses_environment_backed_python_summary() {
        let tmp = std::env::temp_dir().join(format!(
            "adl-aws-remote-validation-script-{}",
            std::process::id()
        ));
        let script = build_remote_command_script(&sample_config(&tmp));
        assert!(script.contains("os.environ[\"COMMAND_EXIT\"]"));
        assert!(script.contains("export ADL_RUN_ROOT=\"$RUN_ROOT\""));
        assert!(!script.contains("int(\"${COMMAND_EXIT}\")"));
        assert!(script.contains("curl -fsS"));
    }

    #[test]
    fn build_remote_command_script_uses_boolean_nextest_flag() {
        let tmp = std::env::temp_dir().join(format!(
            "adl-aws-remote-validation-nextest-{}",
            std::process::id()
        ));
        let mut config = sample_config(&tmp);
        config.command =
            "bash -lc 'cargo build --manifest-path adl/Cargo.toml --locked --bin adl-pr-doctor && cargo test --manifest-path adl/Cargo.toml --locked provider_communication -- --nocapture'"
                .to_string();
        let script = build_remote_command_script(&config);
        assert!(script.contains("NEEDS_NEXTEST=\"0\""));
        assert!(script.contains("if [ \"$NEEDS_NEXTEST\" = \"1\" ]"));
        assert!(!script.contains("[[ \"bash -lc"));
    }

    #[test]
    fn build_remote_command_script_tracks_sccache_degradation() {
        let tmp = std::env::temp_dir().join(format!(
            "adl-aws-remote-validation-sccache-{}",
            std::process::id()
        ));
        let script = build_remote_command_script(&sample_config(&tmp));
        assert!(script.contains("watch_sccache_health"));
        assert!(script.contains("SCCACHE_DEGRADED=0"));
        assert!(script.contains("server_shut_down_unexpectedly"));
        assert!(
            script.contains("\"sccache_degraded\": os.environ.get(\"SCCACHE_DEGRADED\") == \"1\"")
        );
    }

    #[tokio::test]
    async fn remote_validation_fails_when_sccache_degrades() {
        let tmp = std::env::temp_dir().join(format!(
            "adl-aws-remote-validation-sccache-failed-{}",
            std::process::id()
        ));
        let stdout = "ADL_AWS_REMOTE_SUMMARY_BEGIN\n{\"status\":\"passed\",\"bootstrap_seconds\":1,\"command_seconds\":2,\"interruption_detected\":false,\"interruption_notice\":null,\"resolved_commit\":\"abc\",\"rustc_version\":null,\"cargo_version\":null,\"sccache_version\":\"sccache fixture\",\"sccache_degraded\":true,\"sccache_degraded_reason\":\"server_shut_down_unexpectedly\",\"sccache_stats\":null}\nADL_AWS_REMOTE_SUMMARY_END\n";
        let adapter = FakeAdapter {
            quota: QuotaSnapshot {
                spot_vcpu_quota: Some(32.0),
                on_demand_vcpu_quota: Some(64.0),
                notes: vec![],
            },
            identity: AwsAccountIdentity {
                account_id: Some("123456789012".to_string()),
                account_id_sha256: Some("hash".to_string()),
                arn: None,
                user_id: None,
            },
            launch_results: Mutex::new(VecDeque::from(vec![Ok(LaunchResult {
                instance_id: "i-sccache".to_string(),
                initial_state: "pending".to_string(),
                cache_volume: None,
            })])),
            ssm_ready: Ok(SsmReadyResult {
                status: "Online".to_string(),
            }),
            command_result: Ok(CommandExecutionResult {
                command_id: "cmd-sccache".to_string(),
                status: "Success".to_string(),
                response_code: Some(0),
                stdout: stdout.to_string(),
                stderr: String::new(),
            }),
            terminate_result: Ok(()),
            final_state: Ok(Some("terminated".to_string())),
            cost: None,
            budget: None,
        };

        let (summary, events) = run_aws_remote_validation(&adapter, &sample_config(&tmp))
            .await
            .expect("summary");

        assert_eq!(summary.status, RemoteRunStatus::Failed);
        assert!(summary
            .failure_reason
            .as_deref()
            .is_some_and(|reason| reason.contains("sccache")));
        assert!(events
            .iter()
            .any(|event| event.stage == "remote_command" && event.status == "failed"));
    }

    #[tokio::test]
    async fn remote_validation_ignores_html_spot_notice_false_positives() {
        let tmp = std::env::temp_dir().join(format!(
            "adl-aws-remote-validation-html-notice-{}",
            std::process::id()
        ));
        let stdout = "ADL_AWS_REMOTE_SUMMARY_BEGIN\n{\"status\":\"passed\",\"bootstrap_seconds\":1,\"command_seconds\":2,\"interruption_detected\":true,\"interruption_notice\":\"<?xml version=\\\"1.0\\\"?><html><title>404 - Not Found</title></html>\",\"resolved_commit\":\"abc\",\"rustc_version\":null,\"cargo_version\":null,\"sccache_version\":null,\"sccache_degraded\":false,\"sccache_degraded_reason\":null,\"sccache_stats\":null}\nADL_AWS_REMOTE_SUMMARY_END\n";
        let adapter = FakeAdapter {
            quota: QuotaSnapshot {
                spot_vcpu_quota: Some(32.0),
                on_demand_vcpu_quota: Some(64.0),
                notes: vec![],
            },
            identity: AwsAccountIdentity {
                account_id: Some("123456789012".to_string()),
                account_id_sha256: Some("hash".to_string()),
                arn: None,
                user_id: None,
            },
            launch_results: Mutex::new(VecDeque::from(vec![Ok(LaunchResult {
                instance_id: "i-html-false-positive".to_string(),
                initial_state: "pending".to_string(),
                cache_volume: None,
            })])),
            ssm_ready: Ok(SsmReadyResult {
                status: "Online".to_string(),
            }),
            command_result: Ok(CommandExecutionResult {
                command_id: "cmd-html".to_string(),
                status: "Success".to_string(),
                response_code: Some(0),
                stdout: stdout.to_string(),
                stderr: String::new(),
            }),
            terminate_result: Ok(()),
            final_state: Ok(Some("terminated".to_string())),
            cost: None,
            budget: None,
        };

        let (summary, events) = run_aws_remote_validation(&adapter, &sample_config(&tmp))
            .await
            .expect("summary");

        assert_eq!(summary.status, RemoteRunStatus::Passed);
        assert!(summary.failure_reason.is_none());
        assert!(events
            .iter()
            .any(|event| event.stage == "remote_command" && event.status == "ok"));
    }

    #[test]
    fn parse_remote_summary_extracts_json_block() {
        let input = "prefix\nADL_AWS_REMOTE_SUMMARY_BEGIN\n{\"status\":\"passed\",\"bootstrap_seconds\":1,\"command_seconds\":2,\"interruption_detected\":false,\"interruption_notice\":null,\"resolved_commit\":\"abc\",\"rustc_version\":null,\"cargo_version\":null,\"sccache_version\":null,\"sccache_degraded\":false,\"sccache_degraded_reason\":null,\"sccache_stats\":null}\nADL_AWS_REMOTE_SUMMARY_END\nsuffix";
        let summary = parse_remote_summary(input).expect("summary");
        assert_eq!(summary.status, "passed");
        assert_eq!(summary.bootstrap_seconds, Some(1));
        assert!(!summary.interruption_detected);
        assert!(!summary.sccache_degraded);
    }
}

//! Typed supervision policies and the reusable CSM component supervisor.

use std::collections::VecDeque;
use std::fs::{self, OpenOptions};
use std::future::Future;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::time::Duration;

use fs2::FileExt;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinSet;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

pub const SUPERVISION_SCHEMA: &str = "adl.csm.supervision_policy.v1";
pub const SUPERVISION_LIFECYCLE_SCHEMA: &str = "adl.csm.supervision_lifecycle_event.v1";
pub const RECENT_LIFECYCLE_EVENT_LIMIT: usize = 256;
const LIFECYCLE_ACK_TIMEOUT: Duration = Duration::from_secs(2);

/// CSM owns its process-wide panic reporting boundary and installs it at startup.
pub fn install_csm_redacting_panic_hook() {
    std::panic::set_hook(Box::new(|_| {
        eprintln!(
            "adl_event schema=adl.csm.supervision.panic.v1 result=panicked payload=<redacted>"
        );
    }));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentId {
    RuntimeApi,
    Chronosense,
    Scheduler,
    Weather,
    AcipCarrier,
    CuriosityEngine,
    FreedomGate,
    ReasoningRuntime,
    ConstructabilityGate,
    Aee,
    Checkpoint,
    CloudBridge,
    Lifelog,
    Observability,
}

impl ComponentId {
    pub const ALL: [ComponentId; 14] = [
        ComponentId::RuntimeApi,
        ComponentId::Chronosense,
        ComponentId::Scheduler,
        ComponentId::Weather,
        ComponentId::AcipCarrier,
        ComponentId::CuriosityEngine,
        ComponentId::FreedomGate,
        ComponentId::ReasoningRuntime,
        ComponentId::ConstructabilityGate,
        ComponentId::Aee,
        ComponentId::Checkpoint,
        ComponentId::CloudBridge,
        ComponentId::Lifelog,
        ComponentId::Observability,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            ComponentId::RuntimeApi => "runtime_api",
            ComponentId::Chronosense => "chronosense",
            ComponentId::Scheduler => "scheduler",
            ComponentId::Weather => "weather",
            ComponentId::AcipCarrier => "acip_carrier",
            ComponentId::CuriosityEngine => "curiosity_engine",
            ComponentId::FreedomGate => "freedom_gate",
            ComponentId::ReasoningRuntime => "reasoning_runtime",
            ComponentId::ConstructabilityGate => "constructability_gate",
            ComponentId::Aee => "aee",
            ComponentId::Checkpoint => "checkpoint",
            ComponentId::CloudBridge => "cloud_bridge",
            ComponentId::Lifelog => "lifelog",
            ComponentId::Observability => "observability",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentRestartPolicy {
    RestartWithBackoff,
    DegradeAndContinue,
    QuarantineOffendingWork,
    EscalateFailClosed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentDecision {
    ContinueHealthy,
    RestartWithBackoff,
    EscalateAndRestart,
    DegradeAndContinue,
    Quarantine,
    EscalateFailClosed,
    GovernedStop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentExit {
    Healthy,
    Failed,
    Panicked,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentReadiness {
    Ready,
    Degraded,
    NotReady,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleEventKind {
    Start,
    Healthy,
    Degraded,
    RestartScheduled,
    Escalated,
    Quarantined,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleRetention {
    Retained,
    NotRetained,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedLifecycleEvent {
    pub schema: String,
    pub sequence: u64,
    pub component: ComponentId,
    pub attempt: u32,
    pub event: LifecycleEventKind,
    pub readiness: ComponentReadiness,
    pub retention: LifecycleRetention,
    pub reason_code: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ComponentSupervisionPolicy {
    pub component: ComponentId,
    pub restart_policy: ComponentRestartPolicy,
    pub backoff_base_ms: u64,
    pub backoff_cap_ms: u64,
    /// Emit an escalation every N consecutive failures without terminating CSM.
    pub escalation_interval_failures: u32,
    pub degradation_behavior: &'static str,
    pub escalation_target: &'static str,
    pub readiness_impact: &'static str,
    pub critical_for_continuity: bool,
    pub telemetry_can_degrade: bool,
}

pub fn default_component_supervision() -> Vec<ComponentSupervisionPolicy> {
    vec![
        ComponentSupervisionPolicy {
            component: ComponentId::RuntimeApi,
            restart_policy: ComponentRestartPolicy::RestartWithBackoff,
            backoff_base_ms: 100,
            backoff_cap_ms: 5_000,
            escalation_interval_failures: 3,
            degradation_behavior: "readiness_false_while_api_unavailable",
            escalation_target: "operator_and_runtime_observability",
            readiness_impact: "ready_false_when_unavailable",
            critical_for_continuity: false,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::Chronosense,
            restart_policy: ComponentRestartPolicy::DegradeAndContinue,
            backoff_base_ms: 250,
            backoff_cap_ms: 10_000,
            escalation_interval_failures: 1,
            degradation_behavior: "continue_with_stale_time_confidence",
            escalation_target: "block_time_sensitive_admission_below_confidence_floor",
            readiness_impact: "ready_degraded_for_time_sensitive_work",
            critical_for_continuity: true,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::Scheduler,
            restart_policy: ComponentRestartPolicy::RestartWithBackoff,
            backoff_base_ms: 250,
            backoff_cap_ms: 10_000,
            escalation_interval_failures: 2,
            degradation_behavior: "quiesce_admission_until_schedule_state_recovers",
            escalation_target: "operator_and_continuity_control",
            readiness_impact: "ready_false_while_admission_quiesced",
            critical_for_continuity: true,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::Weather,
            restart_policy: ComponentRestartPolicy::EscalateFailClosed,
            backoff_base_ms: 250,
            backoff_cap_ms: 5_000,
            escalation_interval_failures: 1,
            degradation_behavior:
                "serialize_runtime_state_and_stop_when_host_pressure_crosses_threshold",
            escalation_target: "checkpoint_continuity_and_operator_notice",
            readiness_impact: "ready_false_when_graceful_stop_required",
            critical_for_continuity: true,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::AcipCarrier,
            restart_policy: ComponentRestartPolicy::RestartWithBackoff,
            backoff_base_ms: 250,
            backoff_cap_ms: 10_000,
            escalation_interval_failures: 1,
            degradation_behavior: "fail_closed_carrier_admission_without_stopping_core_runtime",
            escalation_target: "runtime_api_auth_freedom_gate_cav_and_operator_notice",
            readiness_impact: "ready_false_for_acip_admission",
            critical_for_continuity: false,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::CuriosityEngine,
            restart_policy: ComponentRestartPolicy::DegradeAndContinue,
            backoff_base_ms: 500,
            backoff_cap_ms: 10_000,
            escalation_interval_failures: 1,
            degradation_behavior: "disable_curiosity_admission_and_continue_core_runtime",
            escalation_target: "operator_notice_if_curiosity_cannot_recover",
            readiness_impact: "ready_degraded_for_curiosity_only",
            critical_for_continuity: false,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::FreedomGate,
            restart_policy: ComponentRestartPolicy::RestartWithBackoff,
            backoff_base_ms: 100,
            backoff_cap_ms: 5_000,
            escalation_interval_failures: 1,
            degradation_behavior: "close_execution_admission_until_freedom_gate_recovers",
            escalation_target: "operator_and_constitutional_control",
            readiness_impact: "ready_false_for_execution_admission",
            critical_for_continuity: true,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::ReasoningRuntime,
            restart_policy: ComponentRestartPolicy::QuarantineOffendingWork,
            backoff_base_ms: 250,
            backoff_cap_ms: 5_000,
            escalation_interval_failures: 1,
            degradation_behavior: "quarantine_offending_graph_and_preserve_input_evidence",
            escalation_target: "recoverable_agent_state_with_quarantine_notice",
            readiness_impact: "ready_true_when_unaffected_graphs_continue",
            critical_for_continuity: false,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::ConstructabilityGate,
            restart_policy: ComponentRestartPolicy::EscalateFailClosed,
            backoff_base_ms: 250,
            backoff_cap_ms: 5_000,
            escalation_interval_failures: 1,
            degradation_behavior: "block_publication_and_admission_without_retained_evidence",
            escalation_target: "constructability_decision_ledger_and_operator_review",
            readiness_impact: "ready_false_until_constructability_status_passes",
            critical_for_continuity: true,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::Aee,
            restart_policy: ComponentRestartPolicy::RestartWithBackoff,
            backoff_base_ms: 250,
            backoff_cap_ms: 10_000,
            escalation_interval_failures: 2,
            degradation_behavior: "close_execution_admission_until_aee_recovers",
            escalation_target: "operator_and_execution_control",
            readiness_impact: "ready_false_for_execution_admission",
            critical_for_continuity: true,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::Checkpoint,
            restart_policy: ComponentRestartPolicy::EscalateFailClosed,
            backoff_base_ms: 250,
            backoff_cap_ms: 5_000,
            escalation_interval_failures: 1,
            degradation_behavior: "block_admission_before_continuity_loss",
            escalation_target: "emergency_safe_fail_serialization",
            readiness_impact: "ready_false_until_checkpoint_persistence_recovers",
            critical_for_continuity: true,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::CloudBridge,
            restart_policy: ComponentRestartPolicy::DegradeAndContinue,
            backoff_base_ms: 500,
            backoff_cap_ms: 30_000,
            escalation_interval_failures: 1,
            degradation_behavior: "buffer_notices_without_advancing_publish_cursors",
            escalation_target: "fail_closed_cloud_status_with_retained_notice_evidence",
            readiness_impact: "ready_degraded_for_external_routes",
            critical_for_continuity: false,
            telemetry_can_degrade: true,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::Lifelog,
            restart_policy: ComponentRestartPolicy::EscalateFailClosed,
            backoff_base_ms: 250,
            backoff_cap_ms: 5_000,
            escalation_interval_failures: 1,
            degradation_behavior: "block_lifecycle_completion_when_append_fails",
            escalation_target: "runtime_readiness_degraded_until_journal_persists",
            readiness_impact: "ready_false_for_lifecycle_completion",
            critical_for_continuity: true,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: ComponentId::Observability,
            restart_policy: ComponentRestartPolicy::DegradeAndContinue,
            backoff_base_ms: 250,
            backoff_cap_ms: 10_000,
            escalation_interval_failures: 1,
            degradation_behavior: "shed_low_priority_telemetry_but_retain_audit_events",
            escalation_target: "operator_if_audit_events_cannot_be_retained_locally",
            readiness_impact: "ready_degraded_for_metrics_only",
            critical_for_continuity: false,
            telemetry_can_degrade: true,
        },
    ]
}

pub fn policy_for(component: ComponentId) -> ComponentSupervisionPolicy {
    default_component_supervision()
        .into_iter()
        .find(|policy| policy.component == component)
        .expect("default supervision policy must cover every ComponentId")
}

pub fn backoff_for_failure(policy: &ComponentSupervisionPolicy, failures: u32) -> Duration {
    let exponent = failures.saturating_sub(1).min(20);
    let multiplier = 1_u64.checked_shl(exponent).unwrap_or(u64::MAX);
    Duration::from_millis(
        policy
            .backoff_base_ms
            .saturating_mul(multiplier)
            .min(policy.backoff_cap_ms),
    )
}

pub fn decide_after_exit(
    policy: &ComponentSupervisionPolicy,
    exit: ComponentExit,
    consecutive_failures: u32,
) -> ComponentDecision {
    match exit {
        ComponentExit::Healthy => ComponentDecision::ContinueHealthy,
        ComponentExit::Cancelled => ComponentDecision::GovernedStop,
        ComponentExit::Failed | ComponentExit::Panicked => match policy.restart_policy {
            ComponentRestartPolicy::RestartWithBackoff
                if policy.escalation_interval_failures > 0
                    && consecutive_failures > 0
                    && consecutive_failures.is_multiple_of(policy.escalation_interval_failures) =>
            {
                ComponentDecision::EscalateAndRestart
            }
            ComponentRestartPolicy::RestartWithBackoff => ComponentDecision::RestartWithBackoff,
            ComponentRestartPolicy::DegradeAndContinue => ComponentDecision::DegradeAndContinue,
            ComponentRestartPolicy::QuarantineOffendingWork => ComponentDecision::Quarantine,
            ComponentRestartPolicy::EscalateFailClosed => ComponentDecision::EscalateFailClosed,
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentFailure {
    Failed(&'static str),
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleReplay {
    pub events: Vec<RetainedLifecycleEvent>,
    pub invalid_lines: usize,
    pub read_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LifecycleSink {
    sender: mpsc::Sender<JournalCommand>,
    acknowledgement_timeout: Duration,
}

#[derive(Debug)]
struct JournalCommand {
    event: RetainedLifecycleEvent,
    response: oneshot::Sender<JournalAppend>,
}

#[derive(Debug)]
struct JournalAppend {
    event: RetainedLifecycleEvent,
    errors: Vec<String>,
}

impl LifecycleSink {
    /// Starts the one writer that all component supervisors clone and share.
    pub fn start(path: impl Into<PathBuf>) -> Self {
        let (sender, receiver) = mpsc::channel(256);
        let path = path.into();
        let _ = std::thread::Builder::new()
            .name("csm-lifecycle-writer".to_string())
            .spawn(move || run_journal_writer(path, receiver));
        Self {
            sender,
            acknowledgement_timeout: LIFECYCLE_ACK_TIMEOUT,
        }
    }

    async fn append(&self, event: RetainedLifecycleEvent) -> JournalAppend {
        let (response, receive) = oneshot::channel();
        let mut fallback_event = event.clone();
        if let Err(error) = self.sender.try_send(JournalCommand { event, response }) {
            let (mut failed_event, reason) = match error {
                mpsc::error::TrySendError::Full(command) => {
                    (command.event, "lifecycle_writer_backpressure")
                }
                mpsc::error::TrySendError::Closed(command) => {
                    (command.event, "lifecycle_writer_unavailable")
                }
            };
            failed_event.retention = LifecycleRetention::NotRetained;
            return JournalAppend {
                event: failed_event,
                errors: vec![reason.to_string()],
            };
        }
        match tokio::time::timeout(self.acknowledgement_timeout, receive).await {
            Ok(Ok(result)) => result,
            Ok(Err(_)) => {
                fallback_event.retention = LifecycleRetention::Unknown;
                JournalAppend {
                    event: fallback_event,
                    errors: vec!["lifecycle_writer_acknowledgement_lost".to_string()],
                }
            }
            Err(_) => {
                fallback_event.retention = LifecycleRetention::Unknown;
                JournalAppend {
                    event: fallback_event,
                    errors: vec!["lifecycle_writer_acknowledgement_timeout".to_string()],
                }
            }
        }
    }
}

#[derive(Debug)]
struct LifecycleJournalWriter {
    file: fs::File,
    _writer_lock: fs::File,
    next_sequence: u64,
    startup_errors: Vec<String>,
}

impl LifecycleJournalWriter {
    fn open(path: &Path) -> Result<Self, String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("lifecycle_parent_create_failed:{error}"))?;
        }
        let writer_lock_path = lifecycle_writer_lock_path(path);
        let writer_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&writer_lock_path)
            .map_err(|error| format!("lifecycle_writer_lock_open_failed:{error}"))?;
        FileExt::try_lock_exclusive(&writer_lock)
            .map_err(|error| format!("lifecycle_writer_lock_failed:{error}"))?;

        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(path)
            .map_err(|error| format!("lifecycle_open_failed:{error}"))?;
        FileExt::lock_exclusive(&file)
            .map_err(|error| format!("lifecycle_data_lock_failed:{error}"))?;

        file.seek(SeekFrom::Start(0))
            .map_err(|error| format!("lifecycle_seek_failed:{error}"))?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|error| format!("lifecycle_read_failed:{error}"))?;
        let mut startup_errors = Vec::new();
        if !bytes.is_empty() && bytes.last() != Some(&b'\n') {
            file.seek(SeekFrom::End(0))
                .map_err(|error| format!("lifecycle_seek_failed:{error}"))?;
            file.write_all(b"\n")
                .and_then(|_| file.flush())
                .and_then(|_| file.sync_data())
                .map_err(|error| format!("lifecycle_torn_tail_repair_failed:{error}"))?;
            startup_errors.push("recovered_torn_final_line".to_string());
        }

        let replay = replay_lifecycle_bytes(&bytes);
        if replay.invalid_lines > 0 {
            startup_errors.push(format!("invalid_journal_lines:{}", replay.invalid_lines));
        }
        let next_sequence = replay
            .events
            .iter()
            .map(|event| event.sequence)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        FileExt::unlock(&file).map_err(|error| format!("lifecycle_data_unlock_failed:{error}"))?;
        Ok(Self {
            file,
            _writer_lock: writer_lock,
            next_sequence,
            startup_errors,
        })
    }

    fn append(&mut self, mut event: RetainedLifecycleEvent) -> JournalAppend {
        event.sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        event.retention = LifecycleRetention::Retained;
        let result = FileExt::lock_exclusive(&self.file).and_then(|_| {
            let write_result = serde_json::to_vec(&event)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
                .and_then(|mut bytes| {
                    bytes.push(b'\n');
                    self.file.write_all(&bytes)
                })
                .and_then(|_| self.file.flush())
                .and_then(|_| self.file.sync_data());
            let unlock_result = FileExt::unlock(&self.file);
            write_result.and(unlock_result)
        });
        let mut errors = self.startup_errors.clone();
        if let Err(error) = result {
            event.retention = LifecycleRetention::NotRetained;
            errors.push(format!("lifecycle_append_failed:{error}"));
        }
        JournalAppend { event, errors }
    }
}

fn lifecycle_writer_lock_path(path: &Path) -> PathBuf {
    let mut lock_path = path.as_os_str().to_os_string();
    lock_path.push(".writer.lock");
    PathBuf::from(lock_path)
}

fn run_journal_writer(path: PathBuf, mut receiver: mpsc::Receiver<JournalCommand>) {
    let mut writer = LifecycleJournalWriter::open(&path);
    while let Some(command) = receiver.blocking_recv() {
        let result = match writer.as_mut() {
            Ok(writer) => writer.append(command.event),
            Err(error) => {
                let mut event = command.event;
                event.retention = LifecycleRetention::NotRetained;
                JournalAppend {
                    event,
                    errors: vec![error.clone()],
                }
            }
        };
        let _ = command.response.send(result);
    }
}

pub fn replay_lifecycle_journal(path: &Path) -> LifecycleReplay {
    let mut file = match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return replay_lifecycle_bytes(&[]),
        Err(error) => {
            return LifecycleReplay {
                events: Vec::new(),
                invalid_lines: 0,
                read_error: Some(format!("lifecycle_read_failed:{error}")),
            };
        }
    };
    if let Err(error) = FileExt::lock_shared(&file) {
        return LifecycleReplay {
            events: Vec::new(),
            invalid_lines: 0,
            read_error: Some(format!("lifecycle_read_lock_failed:{error}")),
        };
    }
    let mut text = String::new();
    let read_result = file.read_to_string(&mut text);
    let unlock_result = FileExt::unlock(&file);
    if let Err(error) = read_result.and(unlock_result) {
        return LifecycleReplay {
            events: Vec::new(),
            invalid_lines: 0,
            read_error: Some(format!("lifecycle_read_failed:{error}")),
        };
    }
    let mut replay = replay_lifecycle_bytes(text.as_bytes());
    replay.read_error = None;
    replay
}

fn replay_lifecycle_bytes(bytes: &[u8]) -> LifecycleReplay {
    let text = String::from_utf8_lossy(bytes);
    let mut events = Vec::new();
    let mut invalid_lines = 0;
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        match serde_json::from_str(line) {
            Ok(event) => events.push(event),
            Err(_) => invalid_lines += 1,
        }
    }
    LifecycleReplay {
        events,
        invalid_lines,
        read_error: None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupervisionOutcome {
    pub component: ComponentId,
    pub decision: ComponentDecision,
    pub readiness: ComponentReadiness,
    pub attempts: u32,
    pub consecutive_failures: u32,
    pub lifecycle_events: Vec<RetainedLifecycleEvent>,
    pub dropped_in_memory_events: u64,
    pub retention_errors: Vec<String>,
}

impl SupervisionOutcome {
    pub fn retention_degraded(&self) -> bool {
        !self.retention_errors.is_empty()
            || self
                .lifecycle_events
                .iter()
                .any(|event| event.retention != LifecycleRetention::Retained)
    }
}

#[derive(Debug, Default)]
struct LifecycleWindow {
    recent: VecDeque<RetainedLifecycleEvent>,
    dropped: u64,
}

impl LifecycleWindow {
    fn push(&mut self, event: RetainedLifecycleEvent) {
        if self.recent.len() == RECENT_LIFECYCLE_EVENT_LIMIT {
            self.recent.pop_front();
            self.dropped = self.dropped.saturating_add(1);
        }
        self.recent.push_back(event);
    }
}

pub async fn supervise_component<F, Fut>(
    component: ComponentId,
    cancellation: CancellationToken,
    lifecycle_sink: LifecycleSink,
    mut run_component: F,
) -> SupervisionOutcome
where
    F: FnMut(u32, CancellationToken) -> Fut,
    Fut: Future<Output = Result<(), ComponentFailure>> + Send + 'static,
{
    let policy = policy_for(component);
    let mut lifecycle_events = LifecycleWindow::default();
    let mut retention_errors = Vec::new();
    let mut attempt = 0_u32;
    let mut consecutive_failures = 0_u32;

    loop {
        if cancellation.is_cancelled() {
            retain_event(
                &lifecycle_sink,
                &mut lifecycle_events,
                &mut retention_errors,
                lifecycle_event(
                    component,
                    attempt,
                    LifecycleEventKind::Stopped,
                    ComponentReadiness::Stopped,
                    "governed_cancellation",
                ),
            )
            .await;
            return outcome(
                &policy,
                ComponentDecision::GovernedStop,
                ComponentReadiness::Stopped,
                attempt,
                consecutive_failures,
                lifecycle_events,
                retention_errors,
            );
        }

        attempt = attempt.saturating_add(1);
        retain_event(
            &lifecycle_sink,
            &mut lifecycle_events,
            &mut retention_errors,
            lifecycle_event(
                component,
                attempt,
                LifecycleEventKind::Start,
                ComponentReadiness::NotReady,
                "component_attempt_started",
            ),
        )
        .await;

        let child_token = cancellation.child_token();
        let constructed = catch_unwind(AssertUnwindSafe(|| {
            run_component(attempt, child_token.clone())
        }));
        let exit = match constructed {
            Err(_) => ComponentExit::Panicked,
            Ok(future) => {
                let mut join_set = JoinSet::new();
                join_set.spawn(future);
                tokio::select! {
                    _ = cancellation.cancelled() => {
                        child_token.cancel();
                        join_set.abort_all();
                        while join_set.join_next().await.is_some() {}
                        ComponentExit::Cancelled
                    }
                    joined = join_set.join_next() => match joined {
                        Some(Ok(Ok(()))) => ComponentExit::Healthy,
                        Some(Ok(Err(ComponentFailure::Cancelled))) => ComponentExit::Cancelled,
                        Some(Ok(Err(ComponentFailure::Failed(_)))) => ComponentExit::Failed,
                        Some(Err(error)) if error.is_panic() => ComponentExit::Panicked,
                        Some(Err(_)) | None => ComponentExit::Failed,
                    }
                }
            }
        };

        if exit == ComponentExit::Healthy {
            retain_event(
                &lifecycle_sink,
                &mut lifecycle_events,
                &mut retention_errors,
                lifecycle_event(
                    component,
                    attempt,
                    LifecycleEventKind::Healthy,
                    ComponentReadiness::Ready,
                    "component_healthy",
                ),
            )
            .await;
            return outcome(
                &policy,
                ComponentDecision::ContinueHealthy,
                ComponentReadiness::Ready,
                attempt,
                0,
                lifecycle_events,
                retention_errors,
            );
        }
        if exit == ComponentExit::Cancelled {
            retain_event(
                &lifecycle_sink,
                &mut lifecycle_events,
                &mut retention_errors,
                lifecycle_event(
                    component,
                    attempt,
                    LifecycleEventKind::Stopped,
                    ComponentReadiness::Stopped,
                    "governed_cancellation",
                ),
            )
            .await;
            return outcome(
                &policy,
                ComponentDecision::GovernedStop,
                ComponentReadiness::Stopped,
                attempt,
                consecutive_failures,
                lifecycle_events,
                retention_errors,
            );
        }

        consecutive_failures = consecutive_failures.saturating_add(1);
        let decision = decide_after_exit(&policy, exit, consecutive_failures);
        let failure_code = if exit == ComponentExit::Panicked {
            "component_task_panicked"
        } else {
            "component_task_failed"
        };

        match decision {
            ComponentDecision::RestartWithBackoff | ComponentDecision::EscalateAndRestart => {
                let kind = if decision == ComponentDecision::EscalateAndRestart {
                    LifecycleEventKind::Escalated
                } else {
                    LifecycleEventKind::RestartScheduled
                };
                retain_event(
                    &lifecycle_sink,
                    &mut lifecycle_events,
                    &mut retention_errors,
                    lifecycle_event(
                        component,
                        attempt,
                        kind,
                        ComponentReadiness::NotReady,
                        failure_code,
                    ),
                )
                .await;
                let backoff = backoff_for_failure(&policy, consecutive_failures);
                tokio::select! {
                    _ = cancellation.cancelled() => {},
                    _ = sleep(backoff) => continue,
                }
            }
            ComponentDecision::DegradeAndContinue => {
                retain_event(
                    &lifecycle_sink,
                    &mut lifecycle_events,
                    &mut retention_errors,
                    lifecycle_event(
                        component,
                        attempt,
                        LifecycleEventKind::Degraded,
                        ComponentReadiness::Degraded,
                        policy.degradation_behavior,
                    ),
                )
                .await;
                return outcome(
                    &policy,
                    decision,
                    ComponentReadiness::Degraded,
                    attempt,
                    consecutive_failures,
                    lifecycle_events,
                    retention_errors,
                );
            }
            ComponentDecision::Quarantine => {
                retain_event(
                    &lifecycle_sink,
                    &mut lifecycle_events,
                    &mut retention_errors,
                    lifecycle_event(
                        component,
                        attempt,
                        LifecycleEventKind::Quarantined,
                        ComponentReadiness::Degraded,
                        policy.degradation_behavior,
                    ),
                )
                .await;
                return outcome(
                    &policy,
                    decision,
                    ComponentReadiness::Degraded,
                    attempt,
                    consecutive_failures,
                    lifecycle_events,
                    retention_errors,
                );
            }
            ComponentDecision::EscalateFailClosed => {
                retain_event(
                    &lifecycle_sink,
                    &mut lifecycle_events,
                    &mut retention_errors,
                    lifecycle_event(
                        component,
                        attempt,
                        LifecycleEventKind::Escalated,
                        ComponentReadiness::NotReady,
                        policy.escalation_target,
                    ),
                )
                .await;
                return outcome(
                    &policy,
                    decision,
                    ComponentReadiness::NotReady,
                    attempt,
                    consecutive_failures,
                    lifecycle_events,
                    retention_errors,
                );
            }
            ComponentDecision::ContinueHealthy | ComponentDecision::GovernedStop => {
                unreachable!("non-terminal failure cannot produce healthy or stopped decision")
            }
        }

        retain_event(
            &lifecycle_sink,
            &mut lifecycle_events,
            &mut retention_errors,
            lifecycle_event(
                component,
                attempt,
                LifecycleEventKind::Stopped,
                ComponentReadiness::Stopped,
                "governed_cancellation_during_backoff",
            ),
        )
        .await;
        return outcome(
            &policy,
            ComponentDecision::GovernedStop,
            ComponentReadiness::Stopped,
            attempt,
            consecutive_failures,
            lifecycle_events,
            retention_errors,
        );
    }
}

fn lifecycle_event(
    component: ComponentId,
    attempt: u32,
    event: LifecycleEventKind,
    readiness: ComponentReadiness,
    reason_code: impl Into<String>,
) -> RetainedLifecycleEvent {
    RetainedLifecycleEvent {
        schema: SUPERVISION_LIFECYCLE_SCHEMA.to_string(),
        sequence: 0,
        component,
        attempt,
        event,
        readiness,
        retention: LifecycleRetention::NotRetained,
        reason_code: reason_code.into(),
    }
}

async fn retain_event(
    sink: &LifecycleSink,
    events: &mut LifecycleWindow,
    retention_errors: &mut Vec<String>,
    event: RetainedLifecycleEvent,
) {
    let appended = sink.append(event).await;
    for error in appended.errors {
        if !retention_errors.contains(&error) {
            retention_errors.push(error);
        }
    }
    events.push(appended.event);
}

fn outcome(
    policy: &ComponentSupervisionPolicy,
    mut decision: ComponentDecision,
    mut readiness: ComponentReadiness,
    attempts: u32,
    consecutive_failures: u32,
    lifecycle_events: LifecycleWindow,
    retention_errors: Vec<String>,
) -> SupervisionOutcome {
    let LifecycleWindow {
        recent: lifecycle_events,
        dropped: dropped_in_memory_events,
    } = lifecycle_events;
    let lifecycle_events: Vec<_> = lifecycle_events.into_iter().collect();
    let retention_degraded = !retention_errors.is_empty()
        || lifecycle_events
            .iter()
            .any(|event| event.retention != LifecycleRetention::Retained);
    if retention_degraded && readiness == ComponentReadiness::Ready {
        if policy.telemetry_can_degrade {
            decision = ComponentDecision::DegradeAndContinue;
            readiness = ComponentReadiness::Degraded;
        } else {
            decision = ComponentDecision::EscalateFailClosed;
            readiness = ComponentReadiness::NotReady;
        }
    }
    SupervisionOutcome {
        component: policy.component,
        decision,
        readiness,
        attempts,
        consecutive_failures,
        lifecycle_events,
        dropped_in_memory_events,
        retention_errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(name: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time")
                .as_nanos();
            Self(std::env::temp_dir().join(format!("adl-supervision-{name}-{unique}")))
        }

        fn journal(&self) -> PathBuf {
            self.0.join("lifecycle.jsonl")
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn matrix_covers_every_runtime_component_with_explicit_policy() {
        let policies = default_component_supervision();
        assert_eq!(policies.len(), ComponentId::ALL.len());
        for component in ComponentId::ALL {
            let policy = policy_for(component);
            assert_eq!(policy.component, component);
            assert!(policy.backoff_cap_ms >= policy.backoff_base_ms);
            assert!(policy.escalation_interval_failures > 0);
            assert!(!policy.degradation_behavior.is_empty());
            assert!(!policy.escalation_target.is_empty());
            assert!(!policy.readiness_impact.is_empty());
        }
    }

    #[test]
    fn backoff_is_exponential_and_capped_without_a_stop_budget() {
        let policy = policy_for(ComponentId::RuntimeApi);
        assert_eq!(backoff_for_failure(&policy, 1), Duration::from_millis(100));
        assert_eq!(backoff_for_failure(&policy, 2), Duration::from_millis(200));
        assert_eq!(
            backoff_for_failure(&policy, 99),
            Duration::from_millis(5_000)
        );
        assert_eq!(
            decide_after_exit(&policy, ComponentExit::Failed, 3),
            ComponentDecision::EscalateAndRestart
        );
        assert_eq!(
            decide_after_exit(&policy, ComponentExit::Failed, 4),
            ComponentDecision::RestartWithBackoff
        );
    }

    #[test]
    fn in_memory_lifecycle_window_is_bounded_while_durable_history_remains_external() {
        let mut window = LifecycleWindow::default();
        for attempt in 1..=(RECENT_LIFECYCLE_EVENT_LIMIT as u32 + 10) {
            window.push(lifecycle_event(
                ComponentId::RuntimeApi,
                attempt,
                LifecycleEventKind::RestartScheduled,
                ComponentReadiness::NotReady,
                "component_task_failed",
            ));
        }
        assert_eq!(window.recent.len(), RECENT_LIFECYCLE_EVENT_LIMIT);
        assert_eq!(window.dropped, 10);
        assert_eq!(window.recent.front().unwrap().attempt, 11);
    }

    #[tokio::test]
    async fn restartable_component_recovers_and_replays_durable_lifecycle() {
        let root = TestRoot::new("recover");
        let attempts = Arc::new(AtomicU32::new(0));
        let run_attempts = Arc::clone(&attempts);
        let outcome = supervise_component(
            ComponentId::RuntimeApi,
            CancellationToken::new(),
            LifecycleSink::start(root.journal()),
            move |_, _| {
                let run_attempts = Arc::clone(&run_attempts);
                async move {
                    if run_attempts.fetch_add(1, Ordering::SeqCst) < 2 {
                        Err(ComponentFailure::Failed("bind_failed"))
                    } else {
                        Ok(())
                    }
                }
            },
        )
        .await;

        assert_eq!(outcome.decision, ComponentDecision::ContinueHealthy);
        assert_eq!(outcome.readiness, ComponentReadiness::Ready);
        assert_eq!(outcome.attempts, 3);
        assert!(!outcome.retention_degraded());
        let replay = replay_lifecycle_journal(&root.journal());
        assert_eq!(replay.invalid_lines, 0);
        assert_eq!(replay.events, outcome.lifecycle_events);
        assert!(replay.events.iter().any(|event| {
            event.event == LifecycleEventKind::RestartScheduled
                && event.readiness == ComponentReadiness::NotReady
        }));
        assert_eq!(
            replay.events.last().unwrap().event,
            LifecycleEventKind::Healthy
        );
    }

    #[tokio::test]
    async fn repeated_failure_escalates_but_never_exhausts_into_runtime_stop() {
        let root = TestRoot::new("repeat");
        let cancellation = CancellationToken::new();
        let cancel_from_task = cancellation.clone();
        let outcome = supervise_component(
            ComponentId::RuntimeApi,
            cancellation,
            LifecycleSink::start(root.journal()),
            move |attempt, _| {
                let cancel_from_task = cancel_from_task.clone();
                async move {
                    if attempt == 4 {
                        cancel_from_task.cancel();
                    }
                    Err(ComponentFailure::Failed("bind_failed"))
                }
            },
        )
        .await;

        assert_eq!(outcome.decision, ComponentDecision::GovernedStop);
        assert_eq!(outcome.readiness, ComponentReadiness::Stopped);
        assert_eq!(outcome.attempts, 4);
        assert!(outcome
            .lifecycle_events
            .iter()
            .any(|event| event.event == LifecycleEventKind::Escalated));
        assert!(!outcome
            .lifecycle_events
            .iter()
            .any(|event| event.event == LifecycleEventKind::Healthy));
    }

    #[tokio::test]
    async fn component_panic_is_governed_and_then_restarted() {
        install_csm_redacting_panic_hook();
        let root = TestRoot::new("panic");
        let outcome = supervise_component(
            ComponentId::Scheduler,
            CancellationToken::new(),
            LifecycleSink::start(root.journal()),
            |attempt, _| async move {
                if attempt == 1 {
                    panic!("test component panic");
                }
                Ok(())
            },
        )
        .await;

        assert_eq!(outcome.decision, ComponentDecision::ContinueHealthy);
        assert_eq!(outcome.attempts, 2);
        assert!(outcome.lifecycle_events.iter().any(|event| {
            event.event == LifecycleEventKind::RestartScheduled
                && event.reason_code == "component_task_panicked"
        }));
    }

    #[tokio::test]
    async fn cancellation_aborts_uncooperative_child_and_records_stop() {
        let root = TestRoot::new("cancel");
        let cancellation = CancellationToken::new();
        cancellation.cancel();
        let outcome = supervise_component(
            ComponentId::Aee,
            cancellation,
            LifecycleSink::start(root.journal()),
            |_, _| async {
                sleep(Duration::from_secs(60)).await;
                Ok(())
            },
        )
        .await;
        assert_eq!(outcome.decision, ComponentDecision::GovernedStop);
        assert_eq!(outcome.attempts, 0);
        assert_eq!(outcome.lifecycle_events.len(), 1);
        assert_eq!(
            outcome.lifecycle_events[0].event,
            LifecycleEventKind::Stopped
        );
    }

    #[tokio::test]
    async fn checkpoint_failure_is_fail_closed_without_panicking_or_false_health() {
        let root = TestRoot::new("checkpoint");
        let outcome = supervise_component(
            ComponentId::Checkpoint,
            CancellationToken::new(),
            LifecycleSink::start(root.journal()),
            |_, _| async { Err(ComponentFailure::Failed("storage_unavailable")) },
        )
        .await;
        assert_eq!(outcome.decision, ComponentDecision::EscalateFailClosed);
        assert_eq!(outcome.readiness, ComponentReadiness::NotReady);
        assert_eq!(outcome.attempts, 1);
        assert_eq!(
            outcome.lifecycle_events.last().unwrap().reason_code,
            "emergency_safe_fail_serialization"
        );
    }

    #[tokio::test]
    async fn chronosense_failure_degrades_truthfully() {
        let root = TestRoot::new("chronosense");
        let outcome = supervise_component(
            ComponentId::Chronosense,
            CancellationToken::new(),
            LifecycleSink::start(root.journal()),
            |_, _| async { Err(ComponentFailure::Failed("time_source_unavailable")) },
        )
        .await;
        assert_eq!(outcome.decision, ComponentDecision::DegradeAndContinue);
        assert_eq!(outcome.readiness, ComponentReadiness::Degraded);
        assert!(!outcome
            .lifecycle_events
            .iter()
            .any(|event| event.event == LifecycleEventKind::Healthy));
    }

    #[tokio::test]
    async fn corrupt_prior_jsonl_does_not_stop_component_supervision() {
        let root = TestRoot::new("corrupt");
        fs::create_dir_all(&root.0).unwrap();
        fs::write(root.journal(), b"not-json\n").unwrap();
        let outcome = supervise_component(
            ComponentId::Observability,
            CancellationToken::new(),
            LifecycleSink::start(root.journal()),
            |_, _| async { Ok(()) },
        )
        .await;
        assert_eq!(outcome.decision, ComponentDecision::DegradeAndContinue);
        assert_eq!(outcome.readiness, ComponentReadiness::Degraded);
        assert!(outcome
            .retention_errors
            .iter()
            .any(|error| error == "invalid_journal_lines:1"));
        let replay = replay_lifecycle_journal(&root.journal());
        assert_eq!(replay.invalid_lines, 1);
        assert_eq!(replay.events.len(), 2);
    }

    #[tokio::test]
    async fn unwritable_lifecycle_target_degrades_retention_without_crashing_component() {
        let root = TestRoot::new("unwritable");
        fs::create_dir_all(root.journal()).unwrap();
        let outcome = supervise_component(
            ComponentId::RuntimeApi,
            CancellationToken::new(),
            LifecycleSink::start(root.journal()),
            |_, _| async { Ok(()) },
        )
        .await;
        assert_eq!(outcome.decision, ComponentDecision::EscalateFailClosed);
        assert_eq!(outcome.readiness, ComponentReadiness::NotReady);
        assert!(outcome.retention_degraded());
        assert!(outcome
            .lifecycle_events
            .iter()
            .all(|event| event.retention == LifecycleRetention::NotRetained));
    }

    #[tokio::test]
    async fn torn_final_jsonl_record_is_preserved_and_separated_from_new_events() {
        let root = TestRoot::new("torn");
        fs::create_dir_all(&root.0).unwrap();
        fs::write(root.journal(), b"torn-json").unwrap();
        let outcome = supervise_component(
            ComponentId::Observability,
            CancellationToken::new(),
            LifecycleSink::start(root.journal()),
            |_, _| async { Ok(()) },
        )
        .await;
        assert_eq!(outcome.decision, ComponentDecision::DegradeAndContinue);
        assert!(outcome
            .retention_errors
            .iter()
            .any(|error| error == "recovered_torn_final_line"));
        let replay = replay_lifecycle_journal(&root.journal());
        assert_eq!(replay.invalid_lines, 1);
        assert_eq!(replay.events.len(), 2);
        assert_eq!(replay.events[0].sequence, 1);
        assert_eq!(replay.events[1].sequence, 2);
    }

    #[tokio::test]
    async fn shared_lifecycle_sink_serializes_concurrent_component_events() {
        let root = TestRoot::new("concurrent");
        let sink = LifecycleSink::start(root.journal());
        let (api, scheduler) = tokio::join!(
            supervise_component(
                ComponentId::RuntimeApi,
                CancellationToken::new(),
                sink.clone(),
                |_, _| async { Ok(()) },
            ),
            supervise_component(
                ComponentId::Scheduler,
                CancellationToken::new(),
                sink,
                |_, _| async { Ok(()) },
            )
        );
        assert_eq!(api.readiness, ComponentReadiness::Ready);
        assert_eq!(scheduler.readiness, ComponentReadiness::Ready);
        let replay = replay_lifecycle_journal(&root.journal());
        assert_eq!(replay.invalid_lines, 0);
        assert_eq!(replay.events.len(), 4);
        assert_eq!(
            replay
                .events
                .iter()
                .map(|event| event.sequence)
                .collect::<Vec<_>>(),
            vec![1, 2, 3, 4]
        );
    }

    #[tokio::test]
    async fn synchronous_future_construction_panic_is_governed_and_restarted() {
        install_csm_redacting_panic_hook();
        let root = TestRoot::new("construction-panic");
        let outcome = supervise_component(
            ComponentId::RuntimeApi,
            CancellationToken::new(),
            LifecycleSink::start(root.journal()),
            |attempt, _| {
                if attempt == 1 {
                    panic!("future construction panic");
                }
                std::future::ready(Ok(()))
            },
        )
        .await;
        assert_eq!(outcome.decision, ComponentDecision::ContinueHealthy);
        assert_eq!(outcome.attempts, 2);
        assert!(outcome.lifecycle_events.iter().any(|event| {
            event.event == LifecycleEventKind::RestartScheduled
                && event.reason_code == "component_task_panicked"
        }));
    }

    #[tokio::test]
    async fn acknowledgement_timeout_reports_unknown_instead_of_false_retention() {
        let (sender, mut receiver) = mpsc::channel::<JournalCommand>(1);
        let writer = std::thread::spawn(move || {
            let command = receiver.blocking_recv().unwrap();
            std::thread::sleep(Duration::from_millis(100));
            let mut event = command.event;
            event.sequence = 7;
            event.retention = LifecycleRetention::Retained;
            let _ = command.response.send(JournalAppend {
                event,
                errors: Vec::new(),
            });
        });
        let sink = LifecycleSink {
            sender,
            acknowledgement_timeout: Duration::from_millis(50),
        };
        let result = sink
            .append(lifecycle_event(
                ComponentId::RuntimeApi,
                1,
                LifecycleEventKind::Start,
                ComponentReadiness::NotReady,
                "component_attempt_started",
            ))
            .await;
        assert_eq!(result.event.retention, LifecycleRetention::Unknown);
        assert_eq!(result.event.sequence, 0);
        assert_eq!(
            result.errors,
            vec!["lifecycle_writer_acknowledgement_timeout"]
        );
        writer.join().unwrap();
    }

    #[tokio::test]
    async fn replay_waits_for_data_lock_and_never_reads_partial_record() {
        let root = TestRoot::new("replay-lock");
        fs::create_dir_all(&root.0).unwrap();
        let mut event = lifecycle_event(
            ComponentId::RuntimeApi,
            1,
            LifecycleEventKind::Healthy,
            ComponentReadiness::Ready,
            "component_healthy",
        );
        event.sequence = 1;
        event.retention = LifecycleRetention::Retained;
        fs::write(
            root.journal(),
            format!("{}\n", serde_json::to_string(&event).unwrap()),
        )
        .unwrap();
        let lock_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(root.journal())
            .unwrap();
        FileExt::lock_exclusive(&lock_file).unwrap();
        let path = root.journal();
        let replay = tokio::task::spawn_blocking(move || replay_lifecycle_journal(&path));
        sleep(Duration::from_millis(25)).await;
        FileExt::unlock(&lock_file).unwrap();
        let replay = replay.await.unwrap();
        assert_eq!(replay.invalid_lines, 0);
        assert_eq!(replay.events, vec![event]);
    }

    #[test]
    fn freedom_gate_policy_is_runtime_critical_and_fail_closed() {
        let freedom_gate = default_component_supervision()
            .into_iter()
            .find(|policy| policy.component == ComponentId::FreedomGate)
            .expect("freedom gate policy");
        assert!(freedom_gate.critical_for_continuity);
        assert!(!freedom_gate.telemetry_can_degrade);
        assert_eq!(
            freedom_gate.restart_policy,
            ComponentRestartPolicy::RestartWithBackoff
        );
        assert_eq!(
            freedom_gate.degradation_behavior,
            "close_execution_admission_until_freedom_gate_recovers"
        );
    }
}

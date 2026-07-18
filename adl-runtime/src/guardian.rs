//! External Runtime v3 process guardian.
//!
//! This module owns the narrow OS-child boundary for
//! `adl-runtime-kernel serve --init <init-path> <continuity-path>`. It intentionally does not
//! become a platform service manager and does not supervise Runtime v2.

use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use tokio::process::{Child, Command};
use tokio::task::JoinHandle;
use tokio::time::{sleep, timeout};
use tokio_util::sync::CancellationToken;

pub const GUARDIAN_SCHEMA: &str = "adl.runtime_v3.external_guardian.v2";
pub const MAX_CAPTURE_BYTES: u64 = 64 * 1024;
const CAPTURE_DRAIN_GRACE: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardianConfig {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub restart_budget: u32,
    pub backoff_base_ms: u64,
    pub backoff_cap_ms: u64,
    pub shutdown_grace_ms: u64,
    pub configuration_exit_codes: Vec<i32>,
}

impl GuardianConfig {
    pub fn runtime_kernel(
        program: impl Into<PathBuf>,
        continuity_path: impl Into<String>,
        init_path: impl Into<String>,
    ) -> Self {
        Self {
            program: program.into(),
            args: vec![
                "serve".to_string(),
                "--init".to_string(),
                init_path.into(),
                "--capsule".to_string(),
                continuity_path.into(),
            ],
            env: Vec::new(),
            restart_budget: 3,
            backoff_base_ms: 100,
            backoff_cap_ms: 5_000,
            shutdown_grace_ms: 10_000,
            configuration_exit_codes: vec![64, 78],
        }
    }

    pub fn validate(&self) -> Result<(), GuardianConfigError> {
        if self.program.as_os_str().is_empty() {
            return Err(GuardianConfigError::MissingProgram);
        }
        if self
            .env
            .iter()
            .any(|(name, _)| name.is_empty() || name.contains('='))
        {
            return Err(GuardianConfigError::InvalidEnvironmentName);
        }
        if self.backoff_base_ms == 0 {
            return Err(GuardianConfigError::ZeroBackoff);
        }
        if self.backoff_cap_ms < self.backoff_base_ms {
            return Err(GuardianConfigError::BackoffCapBelowBase);
        }
        if self.shutdown_grace_ms == 0 {
            return Err(GuardianConfigError::ZeroShutdownGrace);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardianConfigError {
    MissingProgram,
    InvalidEnvironmentName,
    ZeroBackoff,
    BackoffCapBelowBase,
    ZeroShutdownGrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardianTerminalState {
    ExitedSuccessfully,
    ConfigurationExit,
    RestartBudgetExhausted,
    ShutdownForwarded,
    SpawnFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardianAttempt {
    pub attempt: u32,
    pub pid: Option<u32>,
    pub exit_code: Option<i32>,
    pub signal: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub reason_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardianOutcome {
    pub schema: String,
    pub terminal_state: GuardianTerminalState,
    pub attempts: u32,
    pub restarts: u32,
    pub attempts_detail: Vec<GuardianAttempt>,
}

impl GuardianOutcome {
    pub fn last_reason(&self) -> Option<&str> {
        self.attempts_detail
            .last()
            .map(|attempt| attempt.reason_code.as_str())
    }
}

pub async fn run_guardian(
    config: GuardianConfig,
    shutdown: CancellationToken,
) -> Result<GuardianOutcome, GuardianConfigError> {
    config.validate()?;
    let mut attempts_detail = Vec::new();
    let mut attempts = 0_u32;
    let mut restarts = 0_u32;

    loop {
        attempts = attempts.saturating_add(1);
        let spawned = spawn_child(&config);
        let mut child = match spawned {
            Ok(child) => child,
            Err(error) => {
                attempts_detail.push(GuardianAttempt {
                    attempt: attempts,
                    pid: None,
                    exit_code: None,
                    signal: None,
                    stdout: String::new(),
                    stderr: String::new(),
                    reason_code: format!("spawn_failed:{error}"),
                });
                return Ok(outcome(
                    &config,
                    GuardianTerminalState::SpawnFailed,
                    attempts,
                    restarts,
                    attempts_detail,
                ));
            }
        };

        let pid = child.id();
        let stdout = child
            .stdout
            .take()
            .map(capture_pipe)
            .unwrap_or_else(|| tokio::spawn(async { String::new() }));
        let stderr = child
            .stderr
            .take()
            .map(capture_pipe)
            .unwrap_or_else(|| tokio::spawn(async { String::new() }));

        let attempt_exit = tokio::select! {
            _ = shutdown.cancelled() => {
                let signal = terminate_child_tree(&mut child);
                let wait_result = timeout(Duration::from_millis(config.shutdown_grace_ms), child.wait()).await;
                if wait_result.is_err() {
                    force_kill_child_tree(&mut child);
                    let _ = child.kill().await;
                    let _ = child.wait().await;
                }
                let attempt = attempt_record(
                    attempts,
                    pid,
                    None,
                    signal,
                    stdout,
                    stderr,
                    "shutdown_signal_forwarded",
                ).await;
                attempts_detail.push(attempt);
                AttemptExit::Terminal(GuardianTerminalState::ShutdownForwarded)
            }
            status = child.wait() => {
                match status {
                    Ok(status) => {
                        let code = status.code();
                        let signal = exit_signal(&status);
                        graceful_terminate_process_group(pid);
                        let attempt_exit = classify_exit(&config, code, restarts);
                        let reason_code = reason_code_for_exit(&config, code, restarts);
                        let attempt = attempt_record(
                            attempts,
                            pid,
                            code,
                            signal,
                            stdout,
                            stderr,
                            reason_code,
                        ).await;
                        attempts_detail.push(attempt);
                        attempt_exit
                    }
                    Err(error) => {
                        let attempt = attempt_record(
                            attempts,
                            pid,
                            None,
                            None,
                            stdout,
                            stderr,
                            format!("wait_failed:{error}"),
                        ).await;
                        attempts_detail.push(attempt);
                        AttemptExit::Terminal(GuardianTerminalState::SpawnFailed)
                    }
                }
            }
        };

        match attempt_exit {
            AttemptExit::Restart => {
                restarts = restarts.saturating_add(1);
                tokio::select! {
                    _ = shutdown.cancelled() => {
                        attempts_detail.push(GuardianAttempt {
                            attempt: attempts,
                            pid: None,
                            exit_code: None,
                            signal: None,
                            stdout: String::new(),
                            stderr: String::new(),
                            reason_code: "shutdown_during_restart_backoff".to_string(),
                        });
                        return Ok(outcome(
                            &config,
                            GuardianTerminalState::ShutdownForwarded,
                            attempts,
                            restarts,
                            attempts_detail,
                        ));
                    }
                    _ = sleep(backoff(&config, restarts)) => {}
                }
            }
            AttemptExit::Terminal(terminal) => {
                return Ok(outcome(
                    &config,
                    terminal,
                    attempts,
                    restarts,
                    attempts_detail,
                ));
            }
        }
    }
}

#[cfg(unix)]
pub async fn run_guardian_with_os_signals(
    config: GuardianConfig,
) -> Result<GuardianOutcome, GuardianConfigError> {
    use tokio::signal::unix::{signal, SignalKind};

    let shutdown = CancellationToken::new();
    let signal_shutdown = shutdown.clone();
    let signal_task = tokio::spawn(async move {
        let mut interrupt = signal(SignalKind::interrupt()).ok();
        let mut terminate = signal(SignalKind::terminate()).ok();
        match (interrupt.as_mut(), terminate.as_mut()) {
            (Some(interrupt), Some(terminate)) => {
                tokio::select! {
                    _ = interrupt.recv() => {}
                    _ = terminate.recv() => {}
                }
            }
            (Some(interrupt), None) => {
                let _ = interrupt.recv().await;
            }
            (None, Some(terminate)) => {
                let _ = terminate.recv().await;
            }
            (None, None) => return,
        }
        signal_shutdown.cancel();
    });
    let outcome = run_guardian(config, shutdown).await;
    signal_task.abort();
    outcome
}

#[cfg(not(unix))]
pub async fn run_guardian_with_os_signals(
    config: GuardianConfig,
) -> Result<GuardianOutcome, GuardianConfigError> {
    run_guardian(config, CancellationToken::new()).await
}

fn spawn_child(config: &GuardianConfig) -> std::io::Result<Child> {
    let mut command = Command::new(&config.program);
    command
        .args(&config.args)
        .envs(config.env.iter().map(|(name, value)| (name, value)))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    #[cfg(unix)]
    command.process_group(0);
    command.spawn()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AttemptExit {
    Restart,
    Terminal(GuardianTerminalState),
}

fn classify_exit(config: &GuardianConfig, code: Option<i32>, restarts: u32) -> AttemptExit {
    if code == Some(0) {
        return AttemptExit::Terminal(GuardianTerminalState::ExitedSuccessfully);
    }
    if code
        .map(|code| config.configuration_exit_codes.contains(&code))
        .unwrap_or(false)
    {
        return AttemptExit::Terminal(GuardianTerminalState::ConfigurationExit);
    }
    if restarts >= config.restart_budget {
        AttemptExit::Terminal(GuardianTerminalState::RestartBudgetExhausted)
    } else {
        AttemptExit::Restart
    }
}

fn reason_code_for_exit(config: &GuardianConfig, code: Option<i32>, restarts: u32) -> String {
    if code == Some(0) {
        return "child_exited_successfully".to_string();
    }
    if code
        .map(|code| config.configuration_exit_codes.contains(&code))
        .unwrap_or(false)
    {
        return "configuration_exit".to_string();
    }
    if restarts >= config.restart_budget {
        "restart_budget_exhausted".to_string()
    } else {
        "child_failed_restart_scheduled".to_string()
    }
}

async fn attempt_record(
    attempt: u32,
    pid: Option<u32>,
    exit_code: Option<i32>,
    signal: Option<i32>,
    stdout: JoinHandle<String>,
    stderr: JoinHandle<String>,
    reason_code: impl Into<String>,
) -> GuardianAttempt {
    let (stdout, stderr) = tokio::join!(
        bounded_capture(stdout, CAPTURE_DRAIN_GRACE),
        bounded_capture(stderr, CAPTURE_DRAIN_GRACE),
    );
    if stdout.1 || stderr.1 || process_group_alive(pid) {
        force_kill_process_group(pid);
    }
    GuardianAttempt {
        attempt,
        pid,
        exit_code,
        signal,
        stdout: stdout.0,
        stderr: stderr.0,
        reason_code: reason_code.into(),
    }
}

async fn bounded_capture(mut capture: JoinHandle<String>, grace: Duration) -> (String, bool) {
    match timeout(grace, &mut capture).await {
        Ok(result) => (result.unwrap_or_default(), false),
        Err(_) => {
            capture.abort();
            ("<adl_guardian_capture_deadline_exceeded>".to_owned(), true)
        }
    }
}

fn capture_pipe<R>(mut pipe: R) -> JoinHandle<String>
where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        let mut bytes = Vec::new();
        let mut truncated = false;
        let mut scratch = [0_u8; 8192];
        loop {
            match pipe.read(&mut scratch).await {
                Ok(0) => break,
                Ok(read) => {
                    let remaining = (MAX_CAPTURE_BYTES as usize).saturating_sub(bytes.len());
                    if remaining > 0 {
                        bytes.extend_from_slice(&scratch[..read.min(remaining)]);
                    }
                    if read > remaining {
                        truncated = true;
                    }
                }
                Err(error) => return format!("capture_failed:{error}"),
            }
        }
        let mut text = String::from_utf8_lossy(&bytes).into_owned();
        if truncated {
            text.push_str("\n<adl_guardian_output_truncated>");
        }
        text
    })
}

fn backoff(config: &GuardianConfig, restarts: u32) -> Duration {
    let exponent = restarts.saturating_sub(1).min(20);
    let multiplier = 1_u64.checked_shl(exponent).unwrap_or(u64::MAX);
    Duration::from_millis(
        config
            .backoff_base_ms
            .saturating_mul(multiplier)
            .min(config.backoff_cap_ms),
    )
}

fn outcome(
    _config: &GuardianConfig,
    terminal_state: GuardianTerminalState,
    attempts: u32,
    restarts: u32,
    attempts_detail: Vec<GuardianAttempt>,
) -> GuardianOutcome {
    GuardianOutcome {
        schema: GUARDIAN_SCHEMA.to_string(),
        terminal_state,
        attempts,
        restarts,
        attempts_detail,
    }
}

#[cfg(unix)]
fn terminate_child_tree(child: &mut Child) -> Option<i32> {
    let pid = child.id()?;
    let signal = libc::SIGTERM;
    terminate_process_group(Some(pid), signal);
    Some(signal)
}

#[cfg(not(unix))]
fn terminate_child_tree(child: &mut Child) -> Option<i32> {
    let _ = child.start_kill();
    None
}

#[cfg(unix)]
fn force_kill_child_tree(child: &mut Child) {
    force_kill_process_group(child.id());
}

#[cfg(not(unix))]
fn force_kill_child_tree(child: &mut Child) {
    let _ = child.start_kill();
}

#[cfg(unix)]
fn terminate_process_group(pid: Option<u32>, signal: i32) {
    let Some(process_group) = pid.and_then(|pid| i32::try_from(pid).ok()) else {
        return;
    };
    unsafe {
        libc::kill(-process_group, signal);
    }
}

#[cfg(not(unix))]
fn terminate_process_group(_pid: Option<u32>, _signal: i32) {}

#[cfg(unix)]
fn force_kill_process_group(pid: Option<u32>) {
    terminate_process_group(pid, libc::SIGKILL);
}

#[cfg(not(unix))]
fn force_kill_process_group(_pid: Option<u32>) {}

#[cfg(unix)]
fn process_group_alive(pid: Option<u32>) -> bool {
    let Some(process_group) = pid.and_then(|pid| i32::try_from(pid).ok()) else {
        return false;
    };
    (unsafe { libc::kill(-process_group, 0) == 0 })
        || std::io::Error::last_os_error().raw_os_error() == Some(libc::EPERM)
}

#[cfg(not(unix))]
fn process_group_alive(_pid: Option<u32>) -> bool {
    false
}

#[cfg(unix)]
fn graceful_terminate_process_group(pid: Option<u32>) {
    terminate_process_group(pid, libc::SIGTERM);
}

#[cfg(not(unix))]
fn graceful_terminate_process_group(_pid: Option<u32>) {}

#[cfg(unix)]
fn exit_signal(status: &std::process::ExitStatus) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt;
    status.signal()
}

#[cfg(not(unix))]
fn exit_signal(_status: &std::process::ExitStatus) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(name: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time")
                .as_nanos();
            let root = std::env::temp_dir().join(format!("adl-guardian-{name}-{unique}"));
            fs::create_dir_all(&root).expect("test root");
            Self(root)
        }

        fn path(&self, name: &str) -> PathBuf {
            self.0.join(name)
        }

        #[cfg(unix)]
        fn script(&self, name: &str, body: &str) -> PathBuf {
            let path = self.path(name);
            fs::write(&path, body).expect("write script");
            let mut permissions = fs::metadata(&path).expect("metadata").permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&path, permissions).expect("chmod");
            path
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn runtime_kernel_config_preserves_init_and_capsule_paths() {
        let config = GuardianConfig::runtime_kernel(
            "adl-runtime-kernel",
            "continuity.json",
            "runtime-init.toml",
        );
        assert_eq!(
            config.args,
            [
                "serve",
                "--init",
                "runtime-init.toml",
                "--capsule",
                "continuity.json"
            ]
        );
        assert_eq!(config.configuration_exit_codes, [64, 78]);
        assert_eq!(config.validate(), Ok(()));
    }

    #[test]
    fn invalid_environment_name_fails_closed() {
        let mut config = GuardianConfig::runtime_kernel(
            "adl-runtime-kernel",
            "continuity.json",
            "runtime-init.toml",
        );
        config
            .env
            .push(("BAD=NAME".to_string(), "value".to_string()));
        assert_eq!(
            config.validate(),
            Err(GuardianConfigError::InvalidEnvironmentName)
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn captured_output_is_bounded() {
        let root = TestRoot::new("bounded-output");
        let script = root.script("noisy.sh", "#!/bin/sh\nyes x | head -c 70000\nexit 0\n");
        let mut config = GuardianConfig::runtime_kernel(script, "unused", "runtime-init.toml");
        config.args.clear();

        let outcome = run_guardian(config, CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ExitedSuccessfully
        );
        let stdout = &outcome.attempts_detail[0].stdout;
        assert!(stdout.len() <= MAX_CAPTURE_BYTES as usize + 40);
        assert!(stdout.contains("<adl_guardian_output_truncated>"));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn injects_configured_environment() {
        let root = TestRoot::new("env");
        let script = root.script(
            "env.sh",
            "#!/bin/sh\necho \"$ADL_GUARDIAN_ENV_PROBE\"\nexit 0\n",
        );
        let mut config = GuardianConfig::runtime_kernel(script, "unused", "runtime-init.toml");
        config.args.clear();
        config
            .env
            .push(("ADL_GUARDIAN_ENV_PROBE".to_string(), "injected".to_string()));

        let outcome = run_guardian(config, CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ExitedSuccessfully
        );
        assert_eq!(outcome.attempts_detail[0].stdout, "injected\n");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn restarts_until_child_exits_successfully_and_captures_output() {
        let root = TestRoot::new("recover");
        let counter = root.path("counter");
        let script = root.script(
            "recover.sh",
            &format!(
                "#!/bin/sh\ncount=$(cat {counter} 2>/dev/null || echo 0)\ncount=$((count + 1))\necho $count > {counter}\necho stdout-$count\necho stderr-$count >&2\nif [ \"$count\" -lt 3 ]; then exit 7; fi\nexit 0\n",
                counter = counter.display()
            ),
        );
        let mut config = GuardianConfig::runtime_kernel(script, "unused", "runtime-init.toml");
        config.args.clear();
        config.restart_budget = 3;
        config.backoff_base_ms = 1;
        config.backoff_cap_ms = 1;

        let outcome = run_guardian(config, CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ExitedSuccessfully
        );
        assert_eq!(outcome.attempts, 3);
        assert_eq!(outcome.restarts, 2);
        assert_eq!(outcome.last_reason(), Some("child_exited_successfully"));
        assert!(outcome
            .attempts_detail
            .iter()
            .any(|attempt| attempt.stdout.contains("stdout-3")));
        assert!(outcome
            .attempts_detail
            .iter()
            .any(|attempt| attempt.stderr.contains("stderr-2")));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn restart_budget_exhaustion_is_terminal() {
        let root = TestRoot::new("budget");
        let script = root.script("fail.sh", "#!/bin/sh\necho failed >&2\nexit 7\n");
        let mut config = GuardianConfig::runtime_kernel(script, "unused", "runtime-init.toml");
        config.args.clear();
        config.restart_budget = 2;
        config.backoff_base_ms = 1;
        config.backoff_cap_ms = 1;

        let outcome = run_guardian(config, CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::RestartBudgetExhausted
        );
        assert_eq!(outcome.attempts, 3);
        assert_eq!(outcome.restarts, 2);
        assert_eq!(outcome.last_reason(), Some("restart_budget_exhausted"));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn configuration_exit_does_not_restart() {
        let root = TestRoot::new("config");
        let script = root.script("config.sh", "#!/bin/sh\necho bad-config >&2\nexit 64\n");
        let mut config = GuardianConfig::runtime_kernel(script, "unused", "runtime-init.toml");
        config.args.clear();
        config.restart_budget = 5;

        let outcome = run_guardian(config, CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ConfigurationExit
        );
        assert_eq!(outcome.attempts, 1);
        assert_eq!(outcome.restarts, 0);
        assert_eq!(outcome.last_reason(), Some("configuration_exit"));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn shutdown_forwards_sigterm_and_reaps_child() {
        let root = TestRoot::new("shutdown");
        let term_file = root.path("term-seen");
        let script = root.script(
            "trap.sh",
            &format!(
                "#!/bin/sh\ntrap 'echo term > {term_file}; exit 0' TERM\nwhile true; do sleep 1; done\n",
                term_file = term_file.display()
            ),
        );
        let mut config = GuardianConfig::runtime_kernel(script, "unused", "runtime-init.toml");
        config.args.clear();
        config.shutdown_grace_ms = 2_000;
        let shutdown = CancellationToken::new();
        let cancel = shutdown.clone();

        let task = tokio::spawn(run_guardian(config, shutdown));
        sleep(Duration::from_millis(100)).await;
        cancel.cancel();
        let outcome = task.await.unwrap().unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ShutdownForwarded
        );
        assert_eq!(outcome.attempts, 1);
        assert_eq!(outcome.restarts, 0);
        assert_eq!(outcome.last_reason(), Some("shutdown_signal_forwarded"));
        assert_eq!(fs::read_to_string(term_file).unwrap(), "term\n");
        assert_eq!(outcome.attempts_detail[0].signal, Some(libc::SIGTERM));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn child_exit_terminates_descendants_and_bounds_inherited_pipe_capture() {
        let root = TestRoot::new("descendant-cleanup");
        let descendant_file = root.path("descendant-pid");
        let script = root.script(
            "descendant.sh",
            &format!(
                "#!/bin/sh\n(trap '' TERM; while true; do sleep 1; done) &\necho $! > {descendant_file}\nexit 0\n",
                descendant_file = descendant_file.display()
            ),
        );
        let mut config = GuardianConfig::runtime_kernel(script, "unused", "runtime-init.toml");
        config.args.clear();

        let outcome = timeout(
            Duration::from_secs(2),
            run_guardian(config, CancellationToken::new()),
        )
        .await
        .expect("guardian capture must be bounded")
        .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ExitedSuccessfully
        );
        let descendant = fs::read_to_string(descendant_file)
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        let deadline = std::time::Instant::now() + Duration::from_secs(1);
        while unsafe { libc::kill(descendant, 0) } == 0 && std::time::Instant::now() < deadline {
            sleep(Duration::from_millis(10)).await;
        }
        assert_eq!(
            unsafe { libc::kill(descendant, 0) },
            -1,
            "guardian descendant survived process-group cleanup"
        );
        assert!(outcome.attempts_detail[0]
            .stdout
            .contains("<adl_guardian_capture_deadline_exceeded>"));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn child_exit_terminates_descendants_that_close_inherited_pipes() {
        let root = TestRoot::new("detached-descendant-cleanup");
        let descendant_file = root.path("descendant-pid");
        let script = root.script(
            "detached-descendant.sh",
            &format!(
                "#!/bin/sh\n(trap '' TERM; exec >/dev/null 2>&1; while true; do sleep 1; done) &\necho $! > {descendant_file}\nexit 0\n",
                descendant_file = descendant_file.display()
            ),
        );
        let mut config = GuardianConfig::runtime_kernel(script, "unused", "runtime-init.toml");
        config.args.clear();

        let outcome = timeout(
            Duration::from_secs(2),
            run_guardian(config, CancellationToken::new()),
        )
        .await
        .expect("guardian cleanup must be bounded")
        .unwrap();

        let descendant = fs::read_to_string(descendant_file)
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        let deadline = std::time::Instant::now() + Duration::from_secs(1);
        while unsafe { libc::kill(descendant, 0) } == 0 && std::time::Instant::now() < deadline {
            sleep(Duration::from_millis(10)).await;
        }
        assert_eq!(unsafe { libc::kill(descendant, 0) }, -1);
        assert!(!outcome.attempts_detail[0]
            .stdout
            .contains("<adl_guardian_capture_deadline_exceeded>"));
    }
}

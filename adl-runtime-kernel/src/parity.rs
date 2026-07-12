use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

pub const SHADOW_REPORT_SCHEMA: &str = "adl.runtime.shadow_report.v1";
pub const MAX_SHADOW_FIXTURE_BYTES: usize = 1_048_576;
const CANONICAL_PARITY_MATRIX: &[u8] =
    include_bytes!("../../docs/architecture/runtime_v3_parity_matrix.v1.json");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoverageContract {
    hash: String,
    required_capabilities: BTreeSet<String>,
}

impl CoverageContract {
    pub fn canonical() -> Result<Self, ParityError> {
        let matrix: serde_json::Value = serde_json::from_slice(CANONICAL_PARITY_MATRIX)
            .map_err(|_| ParityError::InvalidCoverageContract)?;
        let required_capabilities = matrix
            .get("capabilities")
            .and_then(serde_json::Value::as_array)
            .ok_or(ParityError::InvalidCoverageContract)?
            .iter()
            .map(|capability| {
                capability
                    .get("id")
                    .and_then(serde_json::Value::as_str)
                    .filter(|id| !id.trim().is_empty())
                    .map(str::to_owned)
                    .ok_or(ParityError::InvalidCoverageContract)
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        if required_capabilities.is_empty() {
            return Err(ParityError::InvalidCoverageContract);
        }
        Ok(Self {
            hash: blake3::hash(CANONICAL_PARITY_MATRIX).to_hex().to_string(),
            required_capabilities,
        })
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn required_capabilities(&self) -> &BTreeSet<String> {
        &self.required_capabilities
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SharedFixture {
    pub id: String,
    pub capability: String,
    pub input: serde_json::Value,
    pub expected: ExpectedRelation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpectedRelation {
    Equivalent,
    IntentionalRedesign,
    Unsupported,
    Blocked,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NormalizedOutcome {
    pub lifecycle: Vec<String>,
    pub decision: String,
    pub replay: Vec<String>,
    pub snapshot_hash: Option<String>,
    pub error_code: Option<String>,
    pub evidence: Vec<String>,
}

impl NormalizedOutcome {
    pub fn canonicalize(mut self) -> Self {
        self.evidence.sort();
        self
    }
}

#[async_trait]
pub trait ShadowBackend: Send + Sync {
    fn generation(&self) -> RuntimeGeneration;
    async fn execute(&self, fixture: &SharedFixture) -> Result<NormalizedOutcome, BackendFailure>;
}

pub struct RecordedBackend {
    generation: RuntimeGeneration,
    outcomes: BTreeMap<String, Result<NormalizedOutcome, BackendFailure>>,
}

impl RecordedBackend {
    pub fn new(
        generation: RuntimeGeneration,
        outcomes: BTreeMap<String, Result<NormalizedOutcome, BackendFailure>>,
    ) -> Self {
        Self {
            generation,
            outcomes,
        }
    }
}

#[async_trait]
impl ShadowBackend for RecordedBackend {
    fn generation(&self) -> RuntimeGeneration {
        self.generation
    }

    async fn execute(&self, fixture: &SharedFixture) -> Result<NormalizedOutcome, BackendFailure> {
        self.outcomes.get(&fixture.id).cloned().unwrap_or_else(|| {
            Err(BackendFailure::classified(
                BackendFailureKind::Unsupported,
                "unsupported_fixture",
            ))
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BackendFailure {
    pub kind: BackendFailureKind,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl BackendFailure {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            kind: BackendFailureKind::Other,
            code: code.into(),
            detail: None,
        }
    }

    pub fn with_detail(code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            kind: BackendFailureKind::Other,
            code: code.into(),
            detail: Some(detail.into()),
        }
    }

    pub fn classified(kind: BackendFailureKind, code: impl Into<String>) -> Self {
        Self {
            kind,
            code: code.into(),
            detail: None,
        }
    }

    pub fn classified_with_detail(
        kind: BackendFailureKind,
        code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code: code.into(),
            detail: Some(detail.into()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendFailureKind {
    InvalidInput,
    Unsupported,
    Timeout,
    ProcessStart,
    ProcessExit,
    ProcessTree,
    OutputLimit,
    OutputIo,
    MalformedOutput,
    Normalization,
    Dependency,
    Other,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessOutput {
    StdoutJson,
    FileJson,
}

pub type OutcomeNormalizer = fn(&serde_json::Value) -> Result<NormalizedOutcome, BackendFailure>;
pub type FixtureValidator = fn(&SharedFixture) -> Result<(), BackendFailure>;

pub struct ProcessBackendConfig {
    pub generation: RuntimeGeneration,
    pub program: PathBuf,
    pub args: Vec<String>,
    pub output: ProcessOutput,
    pub output_root: PathBuf,
    pub timeout: Duration,
    pub max_output_bytes: usize,
}

pub struct ProcessBackend {
    generation: RuntimeGeneration,
    program: PathBuf,
    args: Vec<String>,
    output: ProcessOutput,
    output_root: PathBuf,
    timeout: Duration,
    max_output_bytes: usize,
    normalize: OutcomeNormalizer,
    validate_fixture: FixtureValidator,
}

impl ProcessBackend {
    pub fn new(
        config: ProcessBackendConfig,
        normalize: OutcomeNormalizer,
        validate_fixture: FixtureValidator,
    ) -> Result<Self, ParityError> {
        if config.program.as_os_str().is_empty()
            || config.output_root.as_os_str().is_empty()
            || config.args.iter().any(|arg| arg.trim().is_empty())
            || config.args.iter().any(|arg| arg.contains("{fixture_id}"))
            || config.timeout.is_zero()
            || config.max_output_bytes == 0
            || is_shell_interpreter(&config.program)
        {
            return Err(ParityError::InvalidHarness);
        }
        Ok(Self {
            generation: config.generation,
            program: config.program,
            args: config.args,
            output: config.output,
            output_root: config.output_root,
            timeout: config.timeout,
            max_output_bytes: config.max_output_bytes,
            normalize,
            validate_fixture,
        })
    }
}

#[async_trait]
impl ShadowBackend for ProcessBackend {
    fn generation(&self) -> RuntimeGeneration {
        self.generation
    }

    async fn execute(&self, fixture: &SharedFixture) -> Result<NormalizedOutcome, BackendFailure> {
        if !safe_fixture_id(&fixture.id) || fixture.capability.trim().is_empty() {
            return Err(BackendFailure::new("invalid_fixture"));
        }
        (self.validate_fixture)(fixture)?;
        tokio::fs::create_dir_all(&self.output_root)
            .await
            .map_err(|_| BackendFailure::new("output_root"))?;
        let output_path = self.output_root.join(format!(
            "adl-shadow-{}-{}.json",
            fixture.id,
            uuid::Uuid::new_v4()
        ));
        let result = self.execute_inner(fixture, &output_path).await;
        let _ = tokio::fs::remove_file(&output_path).await;
        result
    }
}

impl ProcessBackend {
    async fn execute_inner(
        &self,
        fixture: &SharedFixture,
        output_path: &std::path::Path,
    ) -> Result<NormalizedOutcome, BackendFailure> {
        let args = self
            .args
            .iter()
            .map(|arg| arg.replace("{output}", &output_path.to_string_lossy()))
            .collect::<Vec<_>>();
        let fixture_json = serde_json::to_string(&fixture.input)
            .map_err(|_| BackendFailure::new("fixture_encoding"))?;
        if fixture_json.len() > self.max_output_bytes.min(MAX_SHADOW_FIXTURE_BYTES) {
            return Err(BackendFailure::classified(
                BackendFailureKind::InvalidInput,
                "fixture_limit",
            ));
        }
        let mut command = Command::new(&self.program);
        command
            .args(args)
            .env_clear()
            .env("ADL_SHADOW_FIXTURE_ID", &fixture.id)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true);
        #[cfg(unix)]
        {
            command.process_group(0);
            let file_limit = self.max_output_bytes as libc::rlim_t;
            unsafe {
                command.pre_exec(move || {
                    let limit = libc::rlimit {
                        rlim_cur: file_limit,
                        rlim_max: file_limit,
                    };
                    if libc::setrlimit(libc::RLIMIT_FSIZE, &limit) == 0 {
                        Ok(())
                    } else {
                        Err(std::io::Error::last_os_error())
                    }
                });
            }
        }
        let mut child = command.spawn().map_err(|_| {
            BackendFailure::classified(BackendFailureKind::ProcessStart, "process_start")
        })?;
        let process_id = child.id().ok_or_else(|| {
            BackendFailure::classified(BackendFailureKind::ProcessStart, "process_id")
        })?;
        let mut process_group = ProcessGroupGuard::new(process_id);
        let mut stdin = child.stdin.take().ok_or_else(|| {
            BackendFailure::classified(BackendFailureKind::OutputIo, "stdin_pipe")
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            BackendFailure::classified(BackendFailureKind::OutputIo, "stdout_pipe")
        })?;
        let stderr = child.stderr.take().ok_or_else(|| {
            BackendFailure::classified(BackendFailureKind::OutputIo, "stderr_pipe")
        })?;
        let mut stdout_task = tokio::spawn(read_bounded(stdout, self.max_output_bytes));
        let mut stderr_task = tokio::spawn(read_bounded(stderr, self.max_output_bytes));
        let mut stdin_task = tokio::spawn(async move {
            let result = stdin.write_all(fixture_json.as_bytes()).await;
            drop(stdin);
            result
        });
        let (status, fixture_write_failed) = match tokio::time::timeout(self.timeout, async {
            let (status, fixture_write) = tokio::join!(child.wait(), &mut stdin_task);
            let status = status.map_err(|_| {
                BackendFailure::classified(BackendFailureKind::ProcessExit, "process_wait")
            })?;
            let fixture_write_failed = fixture_write
                .map_err(|_| {
                    BackendFailure::classified(BackendFailureKind::OutputIo, "fixture_write_join")
                })?
                .is_err();
            Ok::<_, BackendFailure>((status, fixture_write_failed))
        })
        .await
        {
            Ok(result) => result?,
            Err(_) => {
                process_group.terminate();
                let _ = tokio::time::timeout(Duration::from_secs(2), child.wait()).await;
                stdin_task.abort();
                stdout_task.abort();
                stderr_task.abort();
                let _ = stdout_task.await;
                let _ = stderr_task.await;
                return Err(BackendFailure::classified(
                    BackendFailureKind::Timeout,
                    "timeout",
                ));
            }
        };
        // The direct child is complete. Remove any ordinary descendants that
        // detached their streams but remained in the harness-owned group.
        process_group.terminate();
        let drain_timeout = self.timeout.min(Duration::from_secs(1));
        let ((stdout, stdout_exceeded), (stderr, stderr_exceeded)) =
            match tokio::time::timeout(drain_timeout, async {
                let stdout = (&mut stdout_task).await.map_err(|_| {
                    BackendFailure::classified(BackendFailureKind::OutputIo, "stdout_join")
                })??;
                let stderr = (&mut stderr_task).await.map_err(|_| {
                    BackendFailure::classified(BackendFailureKind::OutputIo, "stderr_join")
                })??;
                Ok::<_, BackendFailure>((stdout, stderr))
            })
            .await
            {
                Ok(result) => result?,
                Err(_) => {
                    process_group.terminate();
                    stdout_task.abort();
                    stderr_task.abort();
                    let _ = stdout_task.await;
                    let _ = stderr_task.await;
                    return Err(BackendFailure::classified(
                        BackendFailureKind::ProcessTree,
                        "descendant_process",
                    ));
                }
            };
        if stdout_exceeded || stderr_exceeded {
            return Err(BackendFailure::classified_with_detail(
                BackendFailureKind::OutputLimit,
                "output_limit",
                bounded_diagnostic(&stdout, &stderr),
            ));
        }
        let file_at_limit = if self.output == ProcessOutput::FileJson {
            tokio::fs::metadata(output_path)
                .await
                .map(|metadata| {
                    metadata.len() > self.max_output_bytes as u64
                        || (!status.success() && metadata.len() >= self.max_output_bytes as u64)
                })
                .unwrap_or(false)
        } else {
            false
        };
        if file_at_limit {
            return Err(BackendFailure::classified(
                BackendFailureKind::OutputLimit,
                "output_limit",
            ));
        }
        if !status.success() {
            return Err(BackendFailure::classified_with_detail(
                BackendFailureKind::ProcessExit,
                format!("process_exit_{}", status.code().unwrap_or(-1)),
                bounded_diagnostic(&stdout, &stderr),
            ));
        }
        if fixture_write_failed {
            return Err(BackendFailure::classified(
                BackendFailureKind::OutputIo,
                "fixture_write",
            ));
        }
        let (bytes, file_exceeded) = match self.output {
            ProcessOutput::StdoutJson => (stdout, false),
            ProcessOutput::FileJson => {
                let metadata = tokio::fs::metadata(output_path)
                    .await
                    .map_err(|_| BackendFailure::new("output_missing"))?;
                if metadata.len() > self.max_output_bytes as u64 {
                    return Err(BackendFailure::classified(
                        BackendFailureKind::OutputLimit,
                        "output_limit",
                    ));
                }
                let mut options = tokio::fs::OpenOptions::new();
                options.read(true);
                #[cfg(unix)]
                options.custom_flags(libc::O_NOFOLLOW);
                let file = options
                    .open(&output_path)
                    .await
                    .map_err(|_| BackendFailure::new("output_missing"))?;
                if !file
                    .metadata()
                    .await
                    .map_err(|_| BackendFailure::new("output_metadata"))?
                    .is_file()
                {
                    return Err(BackendFailure::classified(
                        BackendFailureKind::OutputIo,
                        "output_not_regular",
                    ));
                }
                read_bounded(file, self.max_output_bytes).await?
            }
        };
        if file_exceeded {
            return Err(BackendFailure::classified(
                BackendFailureKind::OutputLimit,
                "output_limit",
            ));
        }
        let value = serde_json::from_slice(&bytes).map_err(|_| {
            BackendFailure::classified(BackendFailureKind::MalformedOutput, "output_invalid_json")
        })?;
        (self.normalize)(&value).map(NormalizedOutcome::canonicalize)
    }
}

fn is_shell_interpreter(program: &std::path::Path) -> bool {
    program
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, "sh" | "bash" | "dash" | "zsh" | "fish"))
}

fn bounded_diagnostic(stdout: &[u8], stderr: &[u8]) -> String {
    let stream = if stderr.is_empty() { stdout } else { stderr };
    format!(
        "redacted_blake3:{}:bytes={}",
        blake3::hash(stream).to_hex(),
        stream.len()
    )
}

#[cfg(unix)]
fn terminate_process_group(process_id: u32) -> bool {
    let Ok(process_group) = i32::try_from(process_id) else {
        return false;
    };
    // The child is spawned as its own process-group leader. A negative PID targets
    // the complete group so fixture descendants cannot outlive the harness run.
    unsafe { libc::kill(-process_group, libc::SIGKILL) == 0 }
}

#[cfg(not(unix))]
fn terminate_process_group(_process_id: u32) -> bool {
    false
}

struct ProcessGroupGuard {
    process_id: u32,
    armed: bool,
}

impl ProcessGroupGuard {
    fn new(process_id: u32) -> Self {
        Self {
            process_id,
            armed: true,
        }
    }

    fn terminate(&mut self) {
        if self.armed {
            terminate_process_group(self.process_id);
            self.armed = false;
        }
    }
}

impl Drop for ProcessGroupGuard {
    fn drop(&mut self) {
        self.terminate();
    }
}

async fn read_bounded<R: tokio::io::AsyncRead + Unpin>(
    mut reader: R,
    limit: usize,
) -> Result<(Vec<u8>, bool), BackendFailure> {
    let mut retained = Vec::with_capacity(limit.min(8192));
    let mut buffer = [0_u8; 8192];
    let mut total = 0_usize;
    loop {
        let count = reader
            .read(&mut buffer)
            .await
            .map_err(|_| BackendFailure::classified(BackendFailureKind::OutputIo, "output_read"))?;
        if count == 0 {
            break;
        }
        total = total.saturating_add(count);
        let remaining = limit.saturating_sub(retained.len());
        retained.extend_from_slice(&buffer[..count.min(remaining)]);
    }
    Ok((retained, total > limit))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DivergenceClass {
    Equivalent,
    Defect,
    IntentionalRedesign,
    Unsupported,
    Blocked,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FixtureComparison {
    pub fixture: String,
    pub capability: String,
    pub class: DivergenceClass,
    pub v2: Result<NormalizedOutcome, BackendFailure>,
    pub v3: Result<NormalizedOutcome, BackendFailure>,
    pub v2_duration_micros: u64,
    pub v3_duration_micros: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ShadowReport {
    schema: String,
    comparisons: Vec<FixtureComparison>,
    required_capabilities: BTreeSet<String>,
    covered_capabilities: BTreeSet<String>,
    coverage_contract_hash: Option<String>,
    cutover_eligible: bool,
}

impl ShadowReport {
    pub fn comparisons(&self) -> &[FixtureComparison] {
        &self.comparisons
    }

    pub fn cutover_eligible(&self) -> bool {
        self.cutover_eligible
    }

    fn proves_policy(&self, hash: &str, capabilities: &BTreeSet<String>) -> bool {
        if self.schema != SHADOW_REPORT_SCHEMA
            || self.comparisons.is_empty()
            || self.coverage_contract_hash.as_deref() != Some(hash)
            || &self.required_capabilities != capabilities
            || self
                .comparisons
                .iter()
                .any(|comparison| comparison.class != DivergenceClass::Equivalent)
        {
            return false;
        }
        let covered = self
            .comparisons
            .iter()
            .map(|comparison| comparison.capability.clone())
            .collect::<BTreeSet<_>>();
        self.covered_capabilities == covered
            && capabilities.is_subset(&covered)
            && self.cutover_eligible
    }
}

pub struct ShadowHarness {
    v2: Arc<dyn ShadowBackend>,
    v3: Arc<dyn ShadowBackend>,
    max_fixtures: usize,
    concurrency: usize,
    coverage_contract: Option<CoverageContract>,
    sequential_backends: bool,
}

impl ShadowHarness {
    pub fn new(
        v2: Arc<dyn ShadowBackend>,
        v3: Arc<dyn ShadowBackend>,
        max_fixtures: usize,
        concurrency: usize,
    ) -> Result<Self, ParityError> {
        if v2.generation() != RuntimeGeneration::V2
            || v3.generation() != RuntimeGeneration::V3
            || max_fixtures == 0
            || concurrency == 0
        {
            return Err(ParityError::InvalidHarness);
        }
        Ok(Self {
            v2,
            v3,
            max_fixtures,
            concurrency,
            coverage_contract: None,
            sequential_backends: false,
        })
    }

    pub fn sequential_backends(mut self) -> Self {
        self.sequential_backends = true;
        self
    }

    pub fn with_coverage_contract(mut self, contract: CoverageContract) -> Self {
        self.coverage_contract = Some(contract);
        self
    }

    pub async fn compare(&self, fixtures: Vec<SharedFixture>) -> Result<ShadowReport, ParityError> {
        if fixtures.is_empty()
            || fixtures.len() > self.max_fixtures
            || fixtures.iter().any(|fixture| {
                !safe_fixture_id(&fixture.id) || fixture.capability.trim().is_empty()
            })
        {
            return Err(ParityError::InvalidFixtures);
        }
        let mut comparisons = stream::iter(fixtures.into_iter().map(|fixture| {
            let v2 = self.v2.clone();
            let v3 = self.v3.clone();
            let sequential_backends = self.sequential_backends;
            async move {
                let v2_run = async {
                    let started = Instant::now();
                    (v2.execute(&fixture).await, elapsed_micros(started))
                };
                let v3_run = async {
                    let started = Instant::now();
                    (v3.execute(&fixture).await, elapsed_micros(started))
                };
                let ((v2, v2_duration), (v3, v3_duration)) = if sequential_backends {
                    (v2_run.await, v3_run.await)
                } else {
                    tokio::join!(v2_run, v3_run)
                };
                compare_fixture(fixture, v2, v3, v2_duration, v3_duration)
            }
        }))
        .buffer_unordered(self.concurrency)
        .collect::<Vec<_>>()
        .await;
        comparisons.sort_by(|left, right| left.fixture.cmp(&right.fixture));
        let covered_capabilities = comparisons
            .iter()
            .filter(|item| item.class == DivergenceClass::Equivalent)
            .map(|item| item.capability.clone())
            .collect::<BTreeSet<_>>();
        let required_capabilities = self
            .coverage_contract
            .as_ref()
            .map(|contract| contract.required_capabilities.clone())
            .unwrap_or_default();
        let cutover_eligible = !required_capabilities.is_empty()
            && required_capabilities.is_subset(&covered_capabilities)
            && comparisons
                .iter()
                .all(|item| item.class == DivergenceClass::Equivalent);
        Ok(ShadowReport {
            schema: SHADOW_REPORT_SCHEMA.to_owned(),
            comparisons,
            required_capabilities,
            covered_capabilities,
            coverage_contract_hash: self
                .coverage_contract
                .as_ref()
                .map(|contract| contract.hash.clone()),
            cutover_eligible,
        })
    }
}

fn safe_fixture_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

fn compare_fixture(
    fixture: SharedFixture,
    v2: Result<NormalizedOutcome, BackendFailure>,
    v3: Result<NormalizedOutcome, BackendFailure>,
    v2_duration_micros: u64,
    v3_duration_micros: u64,
) -> FixtureComparison {
    let v2 = v2.map(NormalizedOutcome::canonicalize);
    let v3 = v3.map(NormalizedOutcome::canonicalize);
    let class = match fixture.expected {
        ExpectedRelation::Equivalent if matches!((&v2, &v3), (Ok(v2), Ok(v3)) if v2 == v3) => {
            DivergenceClass::Equivalent
        }
        ExpectedRelation::Equivalent => DivergenceClass::Defect,
        ExpectedRelation::IntentionalRedesign if matches!((&v2, &v3), (Ok(v2), Ok(v3)) if v2 != v3) => {
            DivergenceClass::IntentionalRedesign
        }
        ExpectedRelation::Unsupported if matches!((&v2, &v3), (Ok(_), Err(error)) if error.kind == BackendFailureKind::Unsupported) => {
            DivergenceClass::Unsupported
        }
        ExpectedRelation::Blocked if matches!((&v2, &v3), (Err(error), _) | (_, Err(error)) if error.kind == BackendFailureKind::Dependency) => {
            DivergenceClass::Blocked
        }
        ExpectedRelation::IntentionalRedesign
        | ExpectedRelation::Unsupported
        | ExpectedRelation::Blocked => DivergenceClass::Defect,
    };
    FixtureComparison {
        fixture: fixture.id,
        capability: fixture.capability,
        class,
        v2,
        v3,
        v2_duration_micros,
        v3_duration_micros,
    }
}

fn elapsed_micros(started: Instant) -> u64 {
    started.elapsed().as_micros().try_into().unwrap_or(u64::MAX)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeGeneration {
    V2,
    V3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatibilityRoute {
    pub command: String,
    pub v2_supported: bool,
    pub v3_supported: bool,
}

pub struct CompatibilityFacade {
    selected: RuntimeGeneration,
    routes: BTreeMap<String, CompatibilityRoute>,
    v2: Option<Arc<dyn ShadowBackend>>,
    v3: Option<Arc<dyn ShadowBackend>>,
    cutover_policy: Option<CoverageContract>,
}

impl CompatibilityFacade {
    pub fn new(routes: impl IntoIterator<Item = CompatibilityRoute>) -> Result<Self, ParityError> {
        let mut indexed = BTreeMap::new();
        for route in routes {
            if route.command.trim().is_empty()
                || indexed.insert(route.command.clone(), route).is_some()
            {
                return Err(ParityError::InvalidRoute);
            }
        }
        Ok(Self {
            selected: RuntimeGeneration::V2,
            routes: indexed,
            v2: None,
            v3: None,
            cutover_policy: None,
        })
    }

    pub fn with_cutover_policy(mut self, contract: CoverageContract) -> Self {
        self.cutover_policy = Some(contract);
        self
    }

    pub fn bind_backends(
        mut self,
        v2: Arc<dyn ShadowBackend>,
        v3: Arc<dyn ShadowBackend>,
    ) -> Result<Self, ParityError> {
        if v2.generation() != RuntimeGeneration::V2 || v3.generation() != RuntimeGeneration::V3 {
            return Err(ParityError::InvalidHarness);
        }
        self.v2 = Some(v2);
        self.v3 = Some(v3);
        Ok(self)
    }

    pub fn selected(&self) -> RuntimeGeneration {
        self.selected
    }

    pub fn opt_in_v3(&mut self, report: &ShadowReport) -> Result<(), ParityError> {
        let Some(contract) = &self.cutover_policy else {
            return Err(ParityError::CutoverIneligible);
        };
        if !report.proves_policy(contract.hash(), contract.required_capabilities()) {
            return Err(ParityError::CutoverIneligible);
        }
        self.selected = RuntimeGeneration::V3;
        Ok(())
    }

    pub fn rollback(&mut self) {
        self.selected = RuntimeGeneration::V2;
    }

    pub fn resolve(&self, command: &str) -> Result<RuntimeGeneration, ParityError> {
        let route = self
            .routes
            .get(command)
            .ok_or(ParityError::UnsupportedRoute)?;
        let supported = match self.selected {
            RuntimeGeneration::V2 => route.v2_supported,
            RuntimeGeneration::V3 => route.v3_supported,
        };
        supported
            .then_some(self.selected)
            .ok_or(ParityError::UnsupportedRoute)
    }

    pub async fn execute(
        &self,
        command: &str,
        fixture: &SharedFixture,
    ) -> Result<NormalizedOutcome, ParityError> {
        let generation = self.resolve(command)?;
        let backend = match generation {
            RuntimeGeneration::V2 => self.v2.as_ref(),
            RuntimeGeneration::V3 => self.v3.as_ref(),
        }
        .ok_or(ParityError::MissingBackend)?;
        backend.execute(fixture).await.map_err(ParityError::Backend)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Footprint {
    pub implementation_loc: usize,
    pub direct_dependencies: usize,
    pub tests: usize,
    pub build_millis: Option<u64>,
    pub fixture_runtime_micros: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FootprintComparison {
    pub v2: Footprint,
    pub v3: Footprint,
    pub loc_reduction: isize,
    pub test_reduction: isize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModuleClosure {
    pub module: String,
    pub capability: String,
    pub disposition: String,
    pub proof: String,
}

pub fn close_baseline_modules(
    modules: impl IntoIterator<Item = String>,
    routes: &BTreeMap<String, (String, String)>,
) -> Vec<ModuleClosure> {
    let mut closure = modules
        .into_iter()
        .map(|module| {
            let capability = capability_for_module(&module).to_owned();
            let (disposition, proof) = routes
                .get(&capability)
                .cloned()
                .unwrap_or_else(|| ("unmapped".to_owned(), "unmapped".to_owned()));
            ModuleClosure {
                capability,
                module,
                disposition,
                proof,
            }
        })
        .collect::<Vec<_>>();
    closure.sort_by(|left, right| left.module.cmp(&right.module));
    closure
}

fn capability_for_module(module: &str) -> &'static str {
    let path = module.to_ascii_lowercase();
    if path.contains("private_state")
        || path.contains("access_control")
        || path.ends_with("/security.rs")
        || path.contains("security_boundary")
    {
        "private_state.security"
    } else if path.contains("reasoning_graph") || path.contains("loop_runtime") {
        "reasoning.graphs_and_loops"
    } else if path.contains("learning") || path.contains("evaluation_selection") {
        "learning.adaptive_dag"
    } else if path.contains("a2a") || path.contains("acip") || path.contains("networking") {
        "network.acip_a2a_cloud"
    } else if path.contains("observatory")
        || path.contains("observability")
        || path.contains("operator")
        || path.contains("feature_proof")
    {
        "control.health_observability"
    } else if path.contains("contract")
        || path.contains("delegation")
        || path.contains("resource_stewardship")
        || path.contains("economics")
        || path.contains("counterparty")
        || path.contains("bid_schema")
        || path.contains("guild")
        || path.contains("outcome_linkage")
        || path.contains("codefriend_adapter")
    {
        "contracts.delegation_resources"
    } else if path.contains("boot_admission")
        || path.contains("transition_authority")
        || path.contains("aee_")
        || path.contains("governed_tools")
        || path.contains("constructability")
        || path.contains("anti_harm")
    {
        "governance.freedom_gate_aee"
    } else if path.contains("snapshot")
        || path.contains("recovery")
        || path.contains("quarantine")
        || path.contains("wake_continuity")
        || path.contains("determinism")
    {
        "continuity.replay_recovery"
    } else if path.contains("agent_lifecycle") || path.contains("memory_identity") {
        "clock.checkpoint_lifelog"
    } else if path.contains("resident_agent")
        || path.contains("godel_agent")
        || path.contains("csm_run")
        || path.contains("runtime_inhabitant")
    {
        "agents.providers_scheduler"
    } else if path.contains("cognitive_being_flagship") || path.contains("citizen") {
        "citizen.identity_memory"
    } else if path.contains("moral")
        || path.contains("affect")
        || path.contains("wellbeing")
        || path.contains("kindness")
        || path.contains("humor")
    {
        "moral_affect_wellbeing"
    } else if path.contains("curiosity")
        || path.contains("intelligence")
        || path.contains("theory_of_mind")
        || path.contains("manifold")
        || path.contains("challenge")
    {
        "curiosity_intelligence_theory_of_mind"
    } else if path.contains("backpressure") || path.contains("topology") {
        "kernel.topology_and_backpressure"
    } else if path.contains("kernel")
        || path.contains("supervision")
        || path.contains("standing")
        || path.contains("hardening")
        || path.contains("invariant")
        || path.contains("foundation")
    {
        "kernel.lifecycle"
    } else if path.contains("governed_episode")
        || path.contains("freedom_gate")
        || path.contains("invalid_action")
    {
        "governance.freedom_gate_aee"
    } else if path.contains("minimal_integrated_runtime_path") {
        "migration.shadow_parity"
    } else if path.ends_with("/lib.rs")
        || path.ends_with("/mod.rs")
        || path.ends_with("/tests.rs")
        || path.ends_with("/tests/common.rs")
        || path.ends_with("/types.rs")
        || path.ends_with("/validators.rs")
        || path.ends_with("/runtime_api.rs")
    {
        "service.contracts_and_configuration"
    } else {
        "unmapped"
    }
}

impl FootprintComparison {
    pub fn new(v2: Footprint, v3: Footprint) -> Self {
        Self {
            loc_reduction: v2.implementation_loc as isize - v3.implementation_loc as isize,
            test_reduction: v2.tests as isize - v3.tests as isize,
            v2,
            v3,
        }
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ParityError {
    #[error("shadow harness configuration is invalid")]
    InvalidHarness,
    #[error("canonical parity coverage contract is invalid")]
    InvalidCoverageContract,
    #[error("shared fixture set is invalid or exceeds its bound")]
    InvalidFixtures,
    #[error("compatibility route declaration is invalid")]
    InvalidRoute,
    #[error("runtime route is unsupported by the selected generation")]
    UnsupportedRoute,
    #[error("selected runtime backend is not bound")]
    MissingBackend,
    #[error("selected runtime backend failed: {0:?}")]
    Backend(BackendFailure),
    #[error("runtime v3 cutover is not eligible under the supplied parity report")]
    CutoverIneligible,
}

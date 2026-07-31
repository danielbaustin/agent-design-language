use std::{
    fmt,
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use semver::Version;

use crate::{
    Capability, ClockAuthority, Component, ComponentContext, ComponentError, ComponentFactory,
    ComponentId, ComponentSpec, DeterminismClass, FailurePolicy, LifecycleGuarantees,
    RuntimeRecorder, ServiceContract, SERVICE_CONTRACT_SCHEMA,
};

const INITIAL_REASON: &str = "time qualification pending";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeSample {
    pub source: String,
    pub unix_millis: u64,
    pub offset_millis: i64,
    pub round_trip: Duration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeQualificationBounds {
    pub timeout: Duration,
    pub max_offset: Duration,
    pub max_round_trip: Duration,
    pub retry_delay: Duration,
    pub refresh_interval: Duration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeSampleError {
    message: String,
}

impl TimeSampleError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for TimeSampleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for TimeSampleError {}

#[async_trait]
pub trait TimeSampleSource: Send + Sync {
    async fn sample(&self) -> Result<TimeSample, TimeSampleError>;
}

pub fn initial_clock_authority() -> ClockAuthority {
    degraded(INITIAL_REASON)
}

pub async fn qualify_time(
    source: &dyn TimeSampleSource,
    bounds: TimeQualificationBounds,
    cancellation: &CancellationToken,
) -> ClockAuthority {
    if cancellation.is_cancelled() {
        return degraded("time qualification cancelled");
    }

    let sample = tokio::select! {
        biased;
        _ = cancellation.cancelled() => {
            return degraded("time qualification cancelled");
        }
        result = tokio::time::timeout(bounds.timeout, source.sample()) => {
            match result {
                Ok(Ok(sample)) => sample,
                Ok(Err(error)) => {
                    return degraded(format!("time qualification failed: {error}"));
                }
                Err(_) => {
                    return degraded("time qualification timed out");
                }
            }
        }
    };

    if sample.offset_millis.unsigned_abs() > duration_millis(bounds.max_offset) {
        return degraded("time qualification offset exceeds bound");
    }
    if sample.round_trip > bounds.max_round_trip {
        return degraded("time qualification round-trip exceeds bound");
    }

    ClockAuthority::Authoritative {
        source: sample.source,
        unix_millis: sample.unix_millis,
    }
}

fn duration_millis(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

fn degraded(reason: impl Into<String>) -> ClockAuthority {
    ClockAuthority::Degraded {
        reason: reason.into(),
    }
}

/// Runs a blocking clock client behind the async sampling interface.
///
/// This is the adapter boundary for clients such as rsntp 4.1.2, whose
/// `SntpClient::synchronize` operation is synchronous.
pub struct BlockingTimeSampleSource<F> {
    sampler: Arc<F>,
}

pub struct RsntpTimeSampleSource {
    server: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemTimeSampleSource;

#[async_trait]
impl TimeSampleSource for SystemTimeSampleSource {
    async fn sample(&self) -> Result<TimeSample, TimeSampleError> {
        let unix_millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| TimeSampleError::new(error.to_string()))?
            .as_millis()
            .try_into()
            .map_err(|_| TimeSampleError::new("system clock exceeds supported range"))?;
        Ok(TimeSample {
            source: "host_system_clock".to_owned(),
            unix_millis,
            offset_millis: 0,
            round_trip: Duration::ZERO,
        })
    }
}

impl RsntpTimeSampleSource {
    pub fn new(server: impl Into<String>) -> Self {
        Self {
            server: server.into(),
        }
    }
}

#[async_trait]
impl TimeSampleSource for RsntpTimeSampleSource {
    async fn sample(&self) -> Result<TimeSample, TimeSampleError> {
        let server = self.server.clone();
        BlockingTimeSampleSource::new(move || {
            let result = rsntp::SntpClient::new()
                .synchronize(server.as_str())
                .map_err(|error| TimeSampleError::new(error.to_string()))?;
            let offset = result.clock_offset();
            let unix_millis = result
                .datetime()
                .unix_timestamp()
                .map_err(|error| TimeSampleError::new(error.to_string()))?
                .as_millis()
                .try_into()
                .unwrap_or(u64::MAX);
            let offset_millis = (offset.as_secs_f64() * 1_000.0).round();
            if !offset_millis.is_finite()
                || offset_millis < i64::MIN as f64
                || offset_millis > i64::MAX as f64
            {
                return Err(TimeSampleError::new(
                    "SNTP offset is outside supported bounds",
                ));
            }
            Ok(TimeSample {
                source: format!("sntp:{server}"),
                unix_millis,
                offset_millis: offset_millis as i64,
                round_trip: result
                    .round_trip_delay()
                    .abs_as_std_duration()
                    .map_err(|error| TimeSampleError::new(error.to_string()))?,
            })
        })
        .sample()
        .await
    }
}

#[derive(Clone)]
pub struct QualifiedTimeFactory {
    source: Arc<dyn TimeSampleSource>,
    bounds: TimeQualificationBounds,
}

impl QualifiedTimeFactory {
    pub fn new(source: Arc<dyn TimeSampleSource>, bounds: TimeQualificationBounds) -> Self {
        Self { source, bounds }
    }

    pub fn contract() -> ServiceContract {
        let spec = Self::specification();
        ServiceContract {
            schema: SERVICE_CONTRACT_SCHEMA.to_owned(),
            component: spec.id,
            service: "trusted_time".to_owned(),
            version: Version::new(1, 0, 0),
            config_schema: "adl.runtime.trusted_time.config.v1".to_owned(),
            determinism: DeterminismClass::GovernedNondeterministicShell,
            lifecycle: LifecycleGuarantees {
                readiness_required: true,
                bounded_shutdown_millis: 1_000,
                restart_safe: true,
                idempotent_start: true,
            },
            provides: vec![Capability {
                name: "runtime.trusted_time".to_owned(),
                version: Version::new(1, 0, 0),
            }],
            requires: vec![],
            inputs: spec.inputs,
            outputs: spec.outputs,
            failure_policy: spec.failure_policy,
        }
    }

    fn specification() -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::new("trusted_time"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Degrade,
        }
    }
}

impl ComponentFactory for QualifiedTimeFactory {
    fn spec(&self) -> ComponentSpec {
        Self::specification()
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(QualifiedTimeComponent {
            source: self.source.clone(),
            bounds: self.bounds,
        })
    }
}

struct QualifiedTimeComponent {
    source: Arc<dyn TimeSampleSource>,
    bounds: TimeQualificationBounds,
}

#[async_trait]
impl Component for QualifiedTimeComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        let bootstrap =
            qualify_time(&SystemTimeSampleSource, self.bounds, &context.cancellation).await;
        context.recorder.set_clock_authority(bootstrap);
        context.ready();

        loop {
            let authority =
                qualify_time(self.source.as_ref(), self.bounds, &context.cancellation).await;
            let delay = match authority {
                ClockAuthority::Authoritative { .. } => {
                    context.recorder.set_clock_authority(authority);
                    self.bounds.refresh_interval
                }
                ClockAuthority::Degraded { ref reason }
                    if reason == "time qualification cancelled" =>
                {
                    return Ok(());
                }
                ClockAuthority::Degraded { reason } => {
                    tracing::warn!(
                        event = "trusted_time_refresh_failed",
                        reason,
                        "SNTP refresh failed; retaining the last authoritative clock"
                    );
                    self.bounds.retry_delay
                }
            };

            tokio::select! {
                _ = context.cancellation.cancelled() => return Ok(()),
                _ = tokio::time::sleep(delay) => {}
            }
        }
    }
}

#[derive(Clone)]
pub struct RecorderTrustedTime {
    recorder: RuntimeRecorder,
    state: Arc<Mutex<TrustedTimeState>>,
}

struct TrustedTimeState {
    anchor: Option<(u64, u64, Instant)>,
    high_water: u64,
}

impl RecorderTrustedTime {
    pub fn new(recorder: RuntimeRecorder) -> Self {
        Self {
            recorder,
            state: Arc::new(Mutex::new(TrustedTimeState {
                anchor: None,
                high_water: 0,
            })),
        }
    }
}

impl crate::TrustedTime for RecorderTrustedTime {
    fn now_unix_millis(&self) -> u64 {
        match self.recorder.snapshot().clock {
            ClockAuthority::Authoritative { unix_millis, .. } => {
                let mut state = self.state.lock().expect("trusted time mutex poisoned");
                let reset = !matches!(state.anchor, Some((source, _, _)) if source == unix_millis);
                if reset {
                    let base = state.high_water.max(unix_millis);
                    state.anchor = Some((unix_millis, base, Instant::now()));
                }
                let (_, base, observed) = state.anchor.expect("authoritative anchor exists");
                let candidate = base.saturating_add(
                    observed
                        .elapsed()
                        .as_millis()
                        .try_into()
                        .unwrap_or(u64::MAX),
                );
                state.high_water = state.high_water.max(candidate);
                state.high_water
            }
            ClockAuthority::Degraded { .. } => {
                self.state
                    .lock()
                    .expect("trusted time mutex poisoned")
                    .anchor = None;
                0
            }
        }
    }
}

impl<F> BlockingTimeSampleSource<F> {
    pub fn new(sampler: F) -> Self {
        Self {
            sampler: Arc::new(sampler),
        }
    }
}

#[async_trait]
impl<F> TimeSampleSource for BlockingTimeSampleSource<F>
where
    F: Fn() -> Result<TimeSample, TimeSampleError> + Send + Sync + 'static,
{
    async fn sample(&self) -> Result<TimeSample, TimeSampleError> {
        let sampler = Arc::clone(&self.sampler);
        tokio::task::spawn_blocking(move || sampler())
            .await
            .map_err(|error| TimeSampleError::new(format!("time sampler task failed: {error}")))?
    }
}

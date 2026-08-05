use std::{future::pending, time::Duration};

use adl_runtime_kernel::{
    initial_clock_authority, qualify_time, BlockingTimeSampleSource, ClockAuthority,
    RecorderTrustedTime, RuntimeRecorder, SystemTimeSampleSource, TimeQualificationBounds,
    TimeSample, TimeSampleError, TimeSampleSource, TrustedTime,
};
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

const BOUNDS: TimeQualificationBounds = TimeQualificationBounds {
    timeout: Duration::from_millis(10),
    max_offset: Duration::from_millis(100),
    max_round_trip: Duration::from_millis(50),
    retry_delay: Duration::from_millis(1),
    refresh_interval: Duration::from_secs(60),
};

enum TestSource {
    Sample(TimeSample),
    Error(&'static str),
    Pending,
}

#[async_trait]
impl TimeSampleSource for TestSource {
    async fn sample(&self) -> Result<TimeSample, TimeSampleError> {
        match self {
            Self::Sample(sample) => Ok(sample.clone()),
            Self::Error(message) => Err(TimeSampleError::new(*message)),
            Self::Pending => pending().await,
        }
    }
}

fn sample(offset_millis: i64, round_trip_millis: u64) -> TimeSample {
    TimeSample {
        source: "injected-sntp".to_owned(),
        unix_millis: 1_720_000_000_123,
        offset_millis,
        round_trip: Duration::from_millis(round_trip_millis),
    }
}

fn reason(authority: ClockAuthority) -> String {
    match authority {
        ClockAuthority::Degraded { reason } => reason,
        ClockAuthority::Authoritative { .. } => panic!("expected degraded clock authority"),
    }
}

#[test]
fn clock_authority_is_initially_degraded() {
    assert_eq!(
        initial_clock_authority(),
        ClockAuthority::Degraded {
            reason: "time qualification pending".to_owned(),
        }
    );
}

#[tokio::test]
async fn host_system_clock_is_an_immediately_qualified_portable_source() {
    let sample = SystemTimeSampleSource.sample().await.unwrap();

    assert_eq!(sample.source, "host_system_clock");
    assert!(sample.unix_millis > 1_700_000_000_000);
    assert_eq!(sample.offset_millis, 0);
    assert_eq!(sample.round_trip, Duration::ZERO);
    assert!(matches!(
        qualify_time(&SystemTimeSampleSource, BOUNDS, &CancellationToken::new()).await,
        ClockAuthority::Authoritative { source, .. } if source == "host_system_clock"
    ));
}

#[tokio::test]
async fn trusted_time_refuses_degraded_state_and_advances_monotonically() {
    let recorder = RuntimeRecorder::new(8);
    let trusted = RecorderTrustedTime::new(recorder.clone());
    assert_eq!(trusted.now_unix_millis(), 0);
    recorder.set_clock_authority(ClockAuthority::Authoritative {
        source: "test-sntp".to_owned(),
        unix_millis: 1_000,
    });
    assert_eq!(trusted.now_unix_millis(), 1_000);
    tokio::time::sleep(Duration::from_millis(2)).await;
    let advanced = trusted.now_unix_millis();
    assert!(advanced >= 1_002);
    recorder.set_clock_authority(ClockAuthority::Authoritative {
        source: "test-sntp".to_owned(),
        unix_millis: 500,
    });
    assert!(trusted.now_unix_millis() >= advanced);
    recorder.set_clock_authority(ClockAuthority::Authoritative {
        source: "test-sntp".to_owned(),
        unix_millis: 2_000,
    });
    let corrected = trusted.now_unix_millis();
    assert!(corrected >= 2_000);
    recorder.set_clock_authority(ClockAuthority::Degraded {
        reason: "resynchronizing".to_owned(),
    });
    assert_eq!(trusted.now_unix_millis(), 0);
    recorder.set_clock_authority(ClockAuthority::Authoritative {
        source: "test-sntp".to_owned(),
        unix_millis: 100,
    });
    assert!(trusted.now_unix_millis() >= corrected);
}

#[tokio::test]
async fn bounded_success_is_authoritative() {
    let authority = qualify_time(
        &TestSource::Sample(sample(-100, 50)),
        BOUNDS,
        &CancellationToken::new(),
    )
    .await;

    assert_eq!(
        authority,
        ClockAuthority::Authoritative {
            source: "injected-sntp".to_owned(),
            unix_millis: 1_720_000_000_123,
        }
    );
}

#[tokio::test]
async fn blocking_client_adapter_produces_a_qualifiable_sample() {
    let source = BlockingTimeSampleSource::new(|| Ok(sample(5, 10)));

    let authority = qualify_time(&source, BOUNDS, &CancellationToken::new()).await;

    assert!(matches!(authority, ClockAuthority::Authoritative { .. }));
}

#[tokio::test]
async fn sampling_error_remains_degraded() {
    let authority = qualify_time(
        &TestSource::Error("transport unavailable"),
        BOUNDS,
        &CancellationToken::new(),
    )
    .await;

    assert_eq!(
        reason(authority),
        "time qualification failed: transport unavailable"
    );
}

#[tokio::test]
async fn timeout_remains_degraded() {
    let authority = qualify_time(&TestSource::Pending, BOUNDS, &CancellationToken::new()).await;

    assert_eq!(reason(authority), "time qualification timed out");
}

#[tokio::test]
async fn excessive_positive_or_negative_offset_remains_degraded() {
    for offset_millis in [-101, 101] {
        let authority = qualify_time(
            &TestSource::Sample(sample(offset_millis, 1)),
            BOUNDS,
            &CancellationToken::new(),
        )
        .await;

        assert_eq!(reason(authority), "time qualification offset exceeds bound");
    }
}

#[tokio::test]
async fn excessive_round_trip_remains_degraded() {
    let authority = qualify_time(
        &TestSource::Sample(sample(0, 51)),
        BOUNDS,
        &CancellationToken::new(),
    )
    .await;

    assert_eq!(
        reason(authority),
        "time qualification round-trip exceeds bound"
    );
}

#[tokio::test]
async fn in_flight_cancellation_remains_degraded() {
    let cancellation = CancellationToken::new();
    let cancel = cancellation.clone();

    let (authority, ()) = tokio::join!(
        qualify_time(&TestSource::Pending, BOUNDS, &cancellation),
        async move {
            tokio::task::yield_now().await;
            cancel.cancel();
        }
    );

    assert_eq!(reason(authority), "time qualification cancelled");
}

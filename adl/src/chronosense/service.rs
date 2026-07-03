//! Runtime Chronosense service for integrated temporal capture.
//!
//! This module turns the earlier Chronosense contracts into a reusable runtime
//! service. It intentionally keeps the first integration small: callers provide
//! epoch-millisecond runtime timestamps and receive validated UTC/local/lifetime
//! frames backed by the existing `TemporalContext` contract.

use anyhow::{anyhow, Context, Result};
use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};

use super::{
    ChronosenseFoundation, IdentityProfile, TemporalContext, CHRONOSENSE_CLOCK_STACK_SCHEMA,
    CHRONOSENSE_RUNTIME_SERVICE_SCHEMA,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChronosenseRuntimeServiceConfig {
    pub schema_version: String,
    pub timezone: String,
    pub identity: Option<IdentityProfile>,
    pub started_at_epoch_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChronosenseClockStack {
    pub schema_version: String,
    pub utc_timestamp_rfc3339: String,
    pub local_timestamp_rfc3339: String,
    pub timezone: String,
    pub utc_offset: String,
    pub lifetime_elapsed_ms: u128,
    pub monotonic_elapsed_ms: u128,
    pub reference_frames: Vec<String>,
    pub temporal_context: TemporalContext,
}

#[derive(Debug, Clone)]
pub struct ChronosenseRuntimeService {
    config: ChronosenseRuntimeServiceConfig,
}

impl ChronosenseRuntimeServiceConfig {
    pub fn utc(started_at_epoch_ms: u128) -> Self {
        Self {
            schema_version: CHRONOSENSE_RUNTIME_SERVICE_SCHEMA.to_string(),
            timezone: "UTC".to_string(),
            identity: None,
            started_at_epoch_ms,
        }
    }

    pub fn with_identity(
        timezone: impl Into<String>,
        identity: IdentityProfile,
        started_at_epoch_ms: u128,
    ) -> Self {
        Self {
            schema_version: CHRONOSENSE_RUNTIME_SERVICE_SCHEMA.to_string(),
            timezone: timezone.into(),
            identity: Some(identity),
            started_at_epoch_ms,
        }
    }
}

impl ChronosenseRuntimeService {
    pub fn new(config: ChronosenseRuntimeServiceConfig) -> Result<Self> {
        if config.schema_version != CHRONOSENSE_RUNTIME_SERVICE_SCHEMA {
            return Err(anyhow!(
                "unsupported chronosense runtime service schema version '{}'",
                config.schema_version
            ));
        }
        let started_at = utc_datetime_from_epoch_millis(config.started_at_epoch_ms)?;
        TemporalContext::from_now(started_at, &config.timezone, config.identity.as_ref())
            .with_context(|| "invalid chronosense runtime service configuration")?;
        Ok(Self { config })
    }

    pub fn config(&self) -> &ChronosenseRuntimeServiceConfig {
        &self.config
    }

    pub fn foundation(&self) -> ChronosenseFoundation {
        ChronosenseFoundation::bounded_v088()
    }

    pub fn capture_epoch_millis(&self, epoch_ms: u128) -> Result<ChronosenseClockStack> {
        let now_utc = utc_datetime_from_epoch_millis(epoch_ms)?;
        let temporal_context = TemporalContext::from_now(
            now_utc,
            &self.config.timezone,
            self.config.identity.as_ref(),
        )
        .with_context(|| "failed to capture chronosense temporal context")?;
        let elapsed_ms = epoch_ms.saturating_sub(self.config.started_at_epoch_ms);

        Ok(ChronosenseClockStack {
            schema_version: CHRONOSENSE_CLOCK_STACK_SCHEMA.to_string(),
            utc_timestamp_rfc3339: now_utc.to_rfc3339(),
            local_timestamp_rfc3339: temporal_context.local_timestamp_rfc3339.clone(),
            timezone: temporal_context.timezone.clone(),
            utc_offset: temporal_context.utc_offset.clone(),
            lifetime_elapsed_ms: elapsed_ms,
            monotonic_elapsed_ms: elapsed_ms,
            reference_frames: vec![
                "utc_epoch_millis".to_string(),
                "local_civil_time".to_string(),
                "runtime_lifetime".to_string(),
                "runtime_monotonic_elapsed".to_string(),
            ],
            temporal_context,
        })
    }

    pub fn rfc3339_for_epoch_millis(&self, epoch_ms: u128) -> Result<String> {
        Ok(self.capture_epoch_millis(epoch_ms)?.utc_timestamp_rfc3339)
    }
}

fn utc_datetime_from_epoch_millis(epoch_ms: u128) -> Result<chrono::DateTime<Utc>> {
    let epoch_ms = i64::try_from(epoch_ms)
        .with_context(|| format!("epoch millis value {epoch_ms} exceeds supported i64 range"))?;
    Utc.timestamp_millis_opt(epoch_ms)
        .single()
        .ok_or_else(|| anyhow!("epoch millis value {epoch_ms} is not representable as UTC time"))
}

use std::future::Future;
use std::time::{Duration, Instant};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const RESILIENCE_RETRY_POLICY_SCHEMA_V1: &str = "adl.resilience.retry_policy.v1";
const MAX_BACKOFF_EXPONENT: u32 = 20;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RetryPolicyV1 {
    pub max_attempts: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_elapsed_ms: Option<u64>,
}

impl RetryPolicyV1 {
    pub const fn new(max_attempts: u32, backoff_ms: Option<u64>) -> Self {
        Self {
            max_attempts,
            backoff_ms,
            max_elapsed_ms: None,
        }
    }

    pub fn validate(&self) -> Result<(), RetryPolicyError> {
        if self.max_attempts == 0 {
            Err(RetryPolicyError::NoAttempts)
        } else {
            Ok(())
        }
    }

    fn delay(&self, started: Instant) -> Option<Duration> {
        let delay = Duration::from_millis(self.backoff_ms.unwrap_or(0));
        if self
            .max_elapsed_ms
            .is_some_and(|max| started.elapsed().saturating_add(delay) > Duration::from_millis(max))
        {
            None
        } else {
            Some(delay)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryPolicyError {
    NoAttempts,
    ExhaustedWithoutError,
}

pub fn capped_exponential_backoff_ms(base_ms: u64, cap_ms: u64, failures: u32) -> u64 {
    // Keep the left shift comfortably below u64 width; cap_ms still owns the
    // public saturation behavior for callers.
    let exponent = failures.saturating_sub(1).min(MAX_BACKOFF_EXPONENT);
    let multiplier = 1_u64.checked_shl(exponent).unwrap_or(u64::MAX);
    base_ms.saturating_mul(multiplier).min(cap_ms)
}

pub fn capped_exponential_backoff(base_ms: u64, cap_ms: u64, failures: u32) -> Duration {
    Duration::from_millis(capped_exponential_backoff_ms(base_ms, cap_ms, failures))
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RetryTerminalReasonV1 {
    Succeeded,
    RetryBudgetExhausted,
    RetryTimeBudgetExhausted,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RetryAttemptRecordV1 {
    pub attempt_index: u32,
    pub retry_allowed: bool,
    pub scheduled_backoff_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_reason: Option<RetryTerminalReasonV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RetryExecutionTraceV1 {
    pub policy_schema: String,
    pub attempts: Vec<RetryAttemptRecordV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryExecution<T, E> {
    pub result: Result<T, E>,
    pub trace: RetryExecutionTraceV1,
}

pub fn execute_retry_policy_sync<T, E, F, S>(
    policy: &RetryPolicyV1,
    operation: F,
    sleep: S,
) -> Result<RetryExecution<T, E>, RetryPolicyError>
where
    F: FnMut(u32) -> Result<T, E>,
    S: FnMut(Duration),
{
    execute_retry_policy_sync_with_classifier(policy, operation, |_| true, sleep)
}

pub fn execute_retry_policy_sync_with_classifier<T, E, F, C, S>(
    policy: &RetryPolicyV1,
    mut operation: F,
    mut should_retry: C,
    mut sleep: S,
) -> Result<RetryExecution<T, E>, RetryPolicyError>
where
    F: FnMut(u32) -> Result<T, E>,
    C: FnMut(&E) -> bool,
    S: FnMut(Duration),
{
    policy.validate()?;
    let started = Instant::now();
    let mut attempts = Vec::new();
    let mut last_error = None;
    for attempt_index in 1..=policy.max_attempts {
        match operation(attempt_index) {
            Ok(value) => {
                attempts.push(success_attempt(attempt_index));
                return Ok(RetryExecution {
                    result: Ok(value),
                    trace: trace(attempts),
                });
            }
            Err(error) => {
                let retryable = should_retry(&error);
                last_error = Some(error);
                let record = failed_attempt(policy, started, attempt_index, retryable);
                let retry_allowed = record.retry_allowed;
                let delay = Duration::from_millis(record.scheduled_backoff_ms);
                attempts.push(record);
                if retry_allowed {
                    sleep(delay);
                } else {
                    break;
                }
            }
        }
    }
    let error = last_error.ok_or(RetryPolicyError::ExhaustedWithoutError)?;
    Ok(RetryExecution {
        result: Err(error),
        trace: trace(attempts),
    })
}

pub async fn execute_retry_policy_async<T, E, F, Fut, S, SleepFut>(
    policy: &RetryPolicyV1,
    operation: F,
    sleep: S,
) -> Result<RetryExecution<T, E>, RetryPolicyError>
where
    F: FnMut(u32) -> Fut,
    Fut: Future<Output = Result<T, E>>,
    S: FnMut(Duration) -> SleepFut,
    SleepFut: Future<Output = ()>,
{
    execute_retry_policy_async_with_classifier(policy, operation, |_| true, sleep).await
}

pub async fn execute_retry_policy_async_with_classifier<T, E, F, Fut, C, S, SleepFut>(
    policy: &RetryPolicyV1,
    mut operation: F,
    mut should_retry: C,
    mut sleep: S,
) -> Result<RetryExecution<T, E>, RetryPolicyError>
where
    F: FnMut(u32) -> Fut,
    Fut: Future<Output = Result<T, E>>,
    C: FnMut(&E) -> bool,
    S: FnMut(Duration) -> SleepFut,
    SleepFut: Future<Output = ()>,
{
    policy.validate()?;
    let started = Instant::now();
    let mut attempts = Vec::new();
    let mut last_error = None;
    for attempt_index in 1..=policy.max_attempts {
        match operation(attempt_index).await {
            Ok(value) => {
                attempts.push(success_attempt(attempt_index));
                return Ok(RetryExecution {
                    result: Ok(value),
                    trace: trace(attempts),
                });
            }
            Err(error) => {
                let retryable = should_retry(&error);
                last_error = Some(error);
                let record = failed_attempt(policy, started, attempt_index, retryable);
                let retry_allowed = record.retry_allowed;
                let delay = Duration::from_millis(record.scheduled_backoff_ms);
                attempts.push(record);
                if retry_allowed {
                    sleep(delay).await;
                } else {
                    break;
                }
            }
        }
    }
    let error = last_error.ok_or(RetryPolicyError::ExhaustedWithoutError)?;
    Ok(RetryExecution {
        result: Err(error),
        trace: trace(attempts),
    })
}

fn success_attempt(attempt_index: u32) -> RetryAttemptRecordV1 {
    RetryAttemptRecordV1 {
        attempt_index,
        retry_allowed: false,
        scheduled_backoff_ms: 0,
        terminal_reason: Some(RetryTerminalReasonV1::Succeeded),
    }
}

fn failed_attempt(
    policy: &RetryPolicyV1,
    started: Instant,
    attempt_index: u32,
    retryable: bool,
) -> RetryAttemptRecordV1 {
    let within_attempt_budget = attempt_index < policy.max_attempts.max(1);
    let delay = policy.delay(started);
    let retry_allowed = retryable && within_attempt_budget && delay.is_some();
    RetryAttemptRecordV1 {
        attempt_index,
        retry_allowed,
        scheduled_backoff_ms: if retry_allowed {
            delay.unwrap_or_default().as_millis() as u64
        } else {
            0
        },
        terminal_reason: if retry_allowed {
            None
        } else if within_attempt_budget {
            Some(RetryTerminalReasonV1::RetryTimeBudgetExhausted)
        } else {
            Some(RetryTerminalReasonV1::RetryBudgetExhausted)
        },
    }
}

fn trace(attempts: Vec<RetryAttemptRecordV1>) -> RetryExecutionTraceV1 {
    RetryExecutionTraceV1 {
        policy_schema: RESILIENCE_RETRY_POLICY_SCHEMA_V1.to_string(),
        attempts,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sync_retry_retries_until_success() {
        let policy = RetryPolicyV1::new(3, Some(1));
        let mut attempts = 0;
        let mut sleeps = Vec::new();
        let execution = execute_retry_policy_sync(
            &policy,
            |_| {
                attempts += 1;
                if attempts < 2 {
                    Err("not yet")
                } else {
                    Ok("done")
                }
            },
            |delay| sleeps.push(delay.as_millis() as u64),
        )
        .expect("policy");
        assert_eq!(execution.result, Ok("done"));
        assert_eq!(sleeps, vec![1]);
        assert_eq!(execution.trace.attempts.len(), 2);
    }

    #[test]
    fn sync_retry_stops_at_attempt_budget() {
        let policy = RetryPolicyV1::new(2, Some(1));
        let execution: RetryExecution<(), &str> =
            execute_retry_policy_sync(&policy, |_| Err("still failing"), |_| {}).expect("policy");
        assert_eq!(execution.result, Err("still failing"));
        assert_eq!(
            execution.trace.attempts.last().unwrap().terminal_reason,
            Some(RetryTerminalReasonV1::RetryBudgetExhausted)
        );
    }

    #[test]
    fn capped_exponential_backoff_is_saturating_and_capped() {
        assert_eq!(capped_exponential_backoff_ms(100, 5_000, 0), 100);
        assert_eq!(capped_exponential_backoff_ms(100, 5_000, 1), 100);
        assert_eq!(capped_exponential_backoff_ms(100, 5_000, 2), 200);
        assert_eq!(capped_exponential_backoff_ms(100, 5_000, 99), 5_000);
    }

    #[test]
    fn capped_exponential_backoff_is_monotonic_until_cap() {
        let mut previous = 0;
        for failures in 0..64 {
            let next = capped_exponential_backoff_ms(100, 5_000, failures);
            assert!(next >= previous);
            assert!(next <= 5_000);
            previous = next;
        }
        assert_eq!(previous, 5_000);
    }
}

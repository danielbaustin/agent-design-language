use anyhow::Result;
use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Deterministic bounded-executor failure classes.
pub enum BoundedExecutorErrorKind {
    /// Invalid parallelism configuration was provided (`max_parallel < 1`).
    ///
    /// Recovery: supply `max_parallel >= 1`.
    InvalidParallelism,
    /// Shared queue mutex was poisoned by a panic in another worker.
    ///
    /// Recovery: treat as execution failure and retry from a clean run state.
    QueuePoisoned,
    /// A worker thread panicked while executing a job.
    ///
    /// Recovery: inspect step/provider logic and rerun after fixing the panic.
    WorkerPanic,
    /// Executor completed with a mismatched output count.
    ///
    /// Recovery: treat as integrity failure and rerun deterministically.
    OutputCountMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedExecutorError {
    pub kind: BoundedExecutorErrorKind,
    pub message: String,
}

impl std::fmt::Display for BoundedExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BoundedExecutorError {}

impl BoundedExecutorError {
    fn new(kind: BoundedExecutorErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

pub fn stable_failure_kind(err: &anyhow::Error) -> Option<&'static str> {
    for cause in err.chain() {
        if let Some(exec_err) = cause.downcast_ref::<BoundedExecutorError>() {
            return Some(match exec_err.kind {
                BoundedExecutorErrorKind::WorkerPanic | BoundedExecutorErrorKind::QueuePoisoned => {
                    "panic"
                }
                BoundedExecutorErrorKind::InvalidParallelism => "schema_error",
                BoundedExecutorErrorKind::OutputCountMismatch => "io_error",
            });
        }
    }
    None
}

struct Job<T> {
    index: usize,
    run: Box<dyn FnOnce() -> T + Send + 'static>,
}

pub fn run_bounded<T: Send + 'static>(
    max_parallel: usize,
    jobs: Vec<Box<dyn FnOnce() -> T + Send + 'static>>,
) -> Result<Vec<T>> {
    if max_parallel == 0 {
        return Err(BoundedExecutorError::new(
            BoundedExecutorErrorKind::InvalidParallelism,
            "max_parallel must be >= 1",
        )
        .into());
    }
    if jobs.is_empty() {
        return Ok(Vec::new());
    }

    let expected_count = jobs.len();
    let worker_count = max_parallel.min(expected_count);
    let queue: VecDeque<Job<T>> = jobs
        .into_iter()
        .enumerate()
        .map(|(index, run)| Job { index, run })
        .collect();

    let queue = Arc::new(Mutex::new(queue));
    enum WorkerMsg<T> {
        Output(usize, T),
        Error(&'static str),
    }
    let (tx, rx) = mpsc::channel::<WorkerMsg<T>>();

    let mut handles = Vec::with_capacity(worker_count);
    for _ in 0..worker_count {
        let queue = Arc::clone(&queue);
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || loop {
            let job = {
                match queue.lock() {
                    Ok(mut q) => q.pop_front(),
                    Err(_) => {
                        let _ = tx.send(WorkerMsg::Error("bounded executor queue lock poisoned"));
                        break;
                    }
                }
            };
            let Some(job) = job else {
                break;
            };
            let out = (job.run)();
            if tx.send(WorkerMsg::Output(job.index, out)).is_err() {
                break;
            }
        }));
    }
    drop(tx);

    let mut out: Vec<(usize, T)> = Vec::new();
    let mut worker_error: Option<&'static str> = None;
    for item in rx {
        match item {
            WorkerMsg::Output(index, value) => out.push((index, value)),
            WorkerMsg::Error(msg) => {
                if worker_error.is_none() {
                    worker_error = Some(msg);
                }
            }
        }
    }

    for h in handles {
        if h.join().is_err() {
            return Err(BoundedExecutorError::new(
                BoundedExecutorErrorKind::WorkerPanic,
                "bounded executor worker panicked",
            )
            .into());
        }
    }
    if let Some(msg) = worker_error {
        return Err(BoundedExecutorError::new(BoundedExecutorErrorKind::QueuePoisoned, msg).into());
    }

    if out.len() != expected_count {
        return Err(BoundedExecutorError::new(
            BoundedExecutorErrorKind::OutputCountMismatch,
            format!(
                "bounded executor output count mismatch (expected {expected_count}, got {})",
                out.len()
            ),
        )
        .into());
    }

    out.sort_by_key(|(idx, _)| *idx);
    Ok(out.into_iter().map(|(_, v)| v).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn run_bounded_rejects_zero_parallelism() {
        let err = run_bounded::<usize>(0, vec![]).unwrap_err();
        assert!(err.to_string().contains("max_parallel"), "{err:#}");
    }

    #[test]
    fn run_bounded_preserves_job_output_order() {
        let jobs: Vec<Box<dyn FnOnce() -> usize + Send + 'static>> = vec![
            Box::new(|| 10usize),
            Box::new(|| 20usize),
            Box::new(|| 30usize),
        ];
        let out = run_bounded(2, jobs).expect("run_bounded should succeed");
        assert_eq!(out, vec![10, 20, 30]);
    }

    #[test]
    fn run_bounded_respects_parallelism_limit() {
        let active = Arc::new(AtomicUsize::new(0));
        let observed_max = Arc::new(AtomicUsize::new(0));

        let mut jobs: Vec<Box<dyn FnOnce() -> usize + Send + 'static>> = Vec::new();
        for i in 0..8usize {
            let active = Arc::clone(&active);
            let observed_max = Arc::clone(&observed_max);
            jobs.push(Box::new(move || {
                let now = active.fetch_add(1, Ordering::SeqCst) + 1;
                loop {
                    let prev = observed_max.load(Ordering::SeqCst);
                    if now <= prev {
                        break;
                    }
                    if observed_max
                        .compare_exchange(prev, now, Ordering::SeqCst, Ordering::SeqCst)
                        .is_ok()
                    {
                        break;
                    }
                }
                std::thread::sleep(Duration::from_millis(30));
                active.fetch_sub(1, Ordering::SeqCst);
                i
            }));
        }

        let out = run_bounded(3, jobs).expect("run_bounded should succeed");
        assert_eq!(out.len(), 8);
        assert!(
            observed_max.load(Ordering::SeqCst) <= 3,
            "observed max parallel workers exceeded bound"
        );
    }

    #[test]
    fn run_bounded_errors_when_job_panics() {
        let jobs: Vec<Box<dyn FnOnce() -> usize + Send + 'static>> = vec![
            Box::new(|| 1usize),
            Box::new(|| panic!("simulated panic")),
            Box::new(|| 3usize),
        ];

        let err = run_bounded(2, jobs).unwrap_err();
        assert!(
            err.to_string().contains("worker panicked")
                || err.to_string().contains("output count mismatch"),
            "{err:#}"
        );
    }

    #[test]
    fn run_bounded_returns_all_outputs() {
        let jobs: Vec<Box<dyn FnOnce() -> usize + Send + 'static>> = (0..11usize)
            .map(|i| Box::new(move || i) as Box<dyn FnOnce() -> usize + Send + 'static>)
            .collect();
        let out = run_bounded(4, jobs).expect("run_bounded should succeed");
        assert_eq!(out.len(), 11);
    }

    #[test]
    fn run_bounded_returns_empty_for_empty_jobs() {
        let out = run_bounded::<usize>(3, Vec::new()).expect("empty queue should succeed");
        assert!(out.is_empty());
    }

    #[test]
    fn stable_failure_kind_returns_none_for_unrelated_error() {
        let err = anyhow::anyhow!("not a bounded executor error");
        assert_eq!(stable_failure_kind(&err), None);
    }

    #[test]
    fn stable_failure_kind_maps_invalid_parallelism_to_schema_error() {
        let err = run_bounded::<usize>(0, vec![]).expect_err("zero parallelism should fail");
        assert_eq!(stable_failure_kind(&err), Some("schema_error"));
    }

    #[test]
    fn stable_failure_kind_maps_worker_panic_to_panic() {
        let err = BoundedExecutorError::new(
            BoundedExecutorErrorKind::WorkerPanic,
            "bounded executor worker panicked",
        );
        let wrapped: anyhow::Error = err.into();
        assert_eq!(stable_failure_kind(&wrapped), Some("panic"));
    }

    #[test]
    fn stable_failure_kind_maps_queue_poisoned_to_panic() {
        let err = BoundedExecutorError::new(
            BoundedExecutorErrorKind::QueuePoisoned,
            "queue lock poisoned",
        );
        let wrapped: anyhow::Error = err.into();
        assert_eq!(stable_failure_kind(&wrapped), Some("panic"));
    }

    #[test]
    fn stable_failure_kind_maps_output_count_mismatch_to_io_error() {
        let err = BoundedExecutorError::new(
            BoundedExecutorErrorKind::OutputCountMismatch,
            "output count mismatch",
        );
        let wrapped: anyhow::Error = err.into();
        assert_eq!(stable_failure_kind(&wrapped), Some("io_error"));
    }
}

use anyhow::{anyhow, Result};
use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};

struct Job<T> {
    index: usize,
    run: Box<dyn FnOnce() -> T + Send + 'static>,
}

pub fn run_bounded<T: Send + 'static>(
    max_parallel: usize,
    jobs: Vec<Box<dyn FnOnce() -> T + Send + 'static>>,
) -> Result<Vec<T>> {
    if max_parallel == 0 {
        return Err(anyhow!("max_parallel must be >= 1"));
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
    let (tx, rx) = mpsc::channel::<(usize, T)>();

    let mut handles = Vec::with_capacity(worker_count);
    for _ in 0..worker_count {
        let queue = Arc::clone(&queue);
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || loop {
            let job = {
                let mut q = queue.lock().expect("bounded executor queue lock poisoned");
                q.pop_front()
            };
            let Some(job) = job else {
                break;
            };
            let out = (job.run)();
            if tx.send((job.index, out)).is_err() {
                break;
            }
        }));
    }
    drop(tx);

    let mut out: Vec<(usize, T)> = Vec::new();
    for item in rx {
        out.push(item);
    }

    for h in handles {
        if h.join().is_err() {
            return Err(anyhow!("bounded executor worker panicked"));
        }
    }

    if out.len() != expected_count {
        return Err(anyhow!(
            "bounded executor output count mismatch (expected {expected_count}, got {})",
            out.len()
        ));
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
}

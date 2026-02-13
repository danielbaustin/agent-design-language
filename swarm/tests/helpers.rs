use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
static TEMP_DIR_SEQ: AtomicU64 = AtomicU64::new(0);

fn env_lock() -> MutexGuard<'static, ()> {
    match ENV_LOCK.get_or_init(|| Mutex::new(())).lock() {
        Ok(g) => g,
        // If a previous test panicked while holding the lock, recover so subsequent
        // tests can still run (these tests serialize env var changes).
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn sanitize_prefix(prefix: &str) -> String {
    prefix
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Create a unique per-test temp directory in a deterministic way.
///
/// Uses process id + monotonic counter (no clock/random dependency), and retries
/// if a stale directory from an earlier process exists.
pub fn unique_test_temp_dir(prefix: &str) -> PathBuf {
    let mut root = env::temp_dir();
    root.push("swarm-test-temp");
    fs::create_dir_all(&root).expect("create swarm-test-temp root");

    let prefix = sanitize_prefix(prefix);
    let pid = std::process::id();

    loop {
        let seq = TEMP_DIR_SEQ.fetch_add(1, Ordering::Relaxed);
        let dir = root.join(format!("{prefix}-pid{pid}-n{seq}"));
        match fs::create_dir(&dir) {
            Ok(()) => return dir,
            Err(err) if err.kind() == ErrorKind::AlreadyExists => continue,
            Err(err) => panic!("failed to create temp dir {}: {err}", dir.display()),
        }
    }
}

/// RAII guard for test-only env var mutation.
///
/// Best-effort parallel safety: all mutations using this guard are serialized
/// with a global lock, but external mutations (other crates/tests not using this
/// helper) are not controlled.
#[must_use]
pub struct EnvVarGuard {
    key: String,
    old: Option<OsString>,
    _lock: MutexGuard<'static, ()>,
}

#[allow(dead_code)]
pub struct EnvVarGuardMulti {
    entries: Vec<(String, Option<OsString>)>,
    _lock: MutexGuard<'static, ()>,
}
#[allow(dead_code)]
impl EnvVarGuard {
    pub fn set<K: Into<String>, V: AsRef<OsStr>>(key: K, value: V) -> Self {
        let key = key.into();
        let lock = env_lock();
        let old = env::var_os(&key);

        // NOTE: On recent Rust toolchains, env var mutation is marked unsafe due to
        // potential undefined behavior when used concurrently across threads.
        // We guard with ENV_LOCK to keep tests deterministic.
        unsafe {
            env::set_var(&key, value);
        }

        Self {
            key,
            old,
            _lock: lock,
        }
    }

    #[allow(dead_code)]
    pub fn unset<K: Into<String>>(key: K) -> Self {
        let key = key.into();
        let lock = env_lock();
        let old = env::var_os(&key);

        unsafe {
            env::remove_var(&key);
        }

        Self {
            key,
            old,
            _lock: lock,
        }
    }

    #[allow(dead_code)]
    pub fn set_many(pairs: &[(&str, &OsStr)]) -> EnvVarGuardMulti {
        let lock = env_lock();
        let mut entries = Vec::with_capacity(pairs.len());
        for (key, value) in pairs.iter() {
            let key_string = (*key).to_string();
            let old = env::var_os(&key_string);
            unsafe {
                env::set_var(&key_string, value);
            }
            entries.push((key_string, old));
        }

        EnvVarGuardMulti {
            entries,
            _lock: lock,
        }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        unsafe {
            match &self.old {
                Some(v) => env::set_var(&self.key, v),
                None => env::remove_var(&self.key),
            }
        }
    }
}

impl Drop for EnvVarGuardMulti {
    fn drop(&mut self) {
        for (key, old) in self.entries.iter() {
            unsafe {
                match old {
                    Some(v) => env::set_var(key, v),
                    None => env::remove_var(key),
                }
            }
        }
    }
}

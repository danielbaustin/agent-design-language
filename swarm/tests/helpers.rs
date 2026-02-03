use std::env;
use std::ffi::{OsStr, OsString};
use std::sync::{Mutex, MutexGuard, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> MutexGuard<'static, ()> {
    match ENV_LOCK.get_or_init(|| Mutex::new(())).lock() {
        Ok(g) => g,
        // If a previous test panicked while holding the lock, recover so subsequent
        // tests can still run (these tests serialize env var changes).
        Err(poisoned) => poisoned.into_inner(),
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

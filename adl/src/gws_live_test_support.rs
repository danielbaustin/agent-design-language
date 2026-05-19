use std::sync::{Mutex, MutexGuard};

static GWS_LIVE_TEST_ENV_LOCK: Mutex<()> = Mutex::new(());

pub(crate) fn lock_gws_live_test_env() -> MutexGuard<'static, ()> {
    GWS_LIVE_TEST_ENV_LOCK
        .lock()
        .expect("lock gws live test env")
}

pub(crate) struct EnvVarGuard {
    key: &'static str,
    original: Option<String>,
}

impl EnvVarGuard {
    pub(crate) fn set(key: &'static str, value: impl Into<String>) -> Self {
        let original = std::env::var(key).ok();
        unsafe {
            std::env::set_var(key, value.into());
        }
        Self { key, original }
    }

    pub(crate) fn remove(key: &'static str) -> Self {
        let original = std::env::var(key).ok();
        unsafe {
            std::env::remove_var(key);
        }
        Self { key, original }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match &self.original {
            Some(value) => unsafe {
                std::env::set_var(self.key, value);
            },
            None => unsafe {
                std::env::remove_var(self.key);
            },
        }
    }
}

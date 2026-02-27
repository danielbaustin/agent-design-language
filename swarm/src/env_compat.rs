use std::collections::BTreeSet;
use std::sync::{Mutex, OnceLock};

static WARNED_LEGACY_ENV_VARS: OnceLock<Mutex<BTreeSet<String>>> = OnceLock::new();

fn warn_legacy_once(legacy: &str, canonical: &str) {
    let warned = WARNED_LEGACY_ENV_VARS.get_or_init(|| Mutex::new(BTreeSet::new()));
    let mut guard = warned.lock().expect("legacy env warning mutex poisoned");
    if guard.insert(legacy.to_string()) {
        eprintln!(
            "DEPRECATION: env var '{}' is deprecated; use '{}' instead.",
            legacy, canonical
        );
    }
}

pub fn var_os(canonical: &str, legacy: &str) -> Option<std::ffi::OsString> {
    if let Some(v) = std::env::var_os(canonical) {
        return Some(v);
    }
    let legacy_val = std::env::var_os(legacy)?;
    warn_legacy_once(legacy, canonical);
    Some(legacy_val)
}

pub fn var(canonical: &str, legacy: &str) -> Option<String> {
    if let Ok(v) = std::env::var(canonical) {
        return Some(v);
    }
    let legacy_val = std::env::var(legacy).ok()?;
    warn_legacy_once(legacy, canonical);
    Some(legacy_val)
}

pub fn bool_var(canonical: &str, legacy: &str) -> bool {
    var(canonical, legacy)
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

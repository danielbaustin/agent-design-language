use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use ::adl::{artifacts, execute, failure_taxonomy, instrumentation, resolve, trace};

pub(crate) use super::run_artifacts_types::*;

mod cognitive;
mod runtime;
mod summary;

#[allow(unused_imports)]
pub(crate) use cognitive::*;
pub(crate) use runtime::*;
pub(crate) use summary::*;

pub(crate) fn stable_fingerprint_hex(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub(crate) fn execution_plan_hash<T: Serialize>(plan: &T) -> Result<String> {
    let plan_json = serde_json::to_vec(plan).context("serialize execution plan for hashing")?;
    Ok(stable_fingerprint_hex(&plan_json))
}

pub(crate) fn classify_failure_kind(err: &anyhow::Error) -> Option<&'static str> {
    failure_taxonomy::classify(err)
}

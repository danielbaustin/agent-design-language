use super::dispatch::real_identity_in_repo;
use super::helpers::{repo_root, required_value, resolve_identity_path, run_git_capture};
use ::adl::chronosense::{
    default_identity_profile_path, load_identity_profile, TEMPORAL_CONTEXT_SCHEMA,
};
use once_cell::sync::Lazy;
use serde_json::Value;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

fn system_git_bin() -> &'static str {
    if Path::new("/usr/bin/git").exists() {
        "/usr/bin/git"
    } else {
        "git"
    }
}

fn temp_repo(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let repo = env::temp_dir().join(format!("adl-{name}-{unique}"));
    fs::create_dir_all(&repo).expect("create repo dir");
    Command::new(system_git_bin())
        .arg("init")
        .current_dir(&repo)
        .status()
        .expect("git init")
        .success()
        .then_some(())
        .expect("git init should succeed");
    repo
}

mod adversarial_contracts;
mod dispatch;
mod helper_functions;
mod profile;
mod skill_contracts;
mod temporal_contracts;

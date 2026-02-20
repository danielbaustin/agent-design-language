use std::fs;
use std::path::Path;
use std::process::Command;

mod helpers;
use helpers::{unique_test_temp_dir, EnvVarGuard};

fn tmp_dir(prefix: &str) -> std::path::PathBuf {
    unique_test_temp_dir(prefix)
}

fn run_swarm(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_swarm");
    Command::new(exe).args(args).output().unwrap()
}

fn write_mock_ollama(dir: &Path) -> std::path::PathBuf {
    let bin = dir.join("ollama");
    let script = r#"#!/bin/sh
set -eu
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
cat
exit 0
"#;
    fs::write(&bin, script.as_bytes()).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&bin).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&bin, perms).unwrap();
    }
    bin
}

fn prepend_path(dir: &Path) -> std::ffi::OsString {
    let current = std::env::var_os("PATH").unwrap_or_default();
    let mut parts = vec![dir.as_os_str().to_os_string()];
    parts.extend(std::env::split_paths(&current).map(|p| p.into_os_string()));
    std::env::join_paths(parts).unwrap()
}

fn write_unsigned_fixture(base: &Path) -> std::path::PathBuf {
    let yaml = r#"
version: "0.5"

providers:
  local:
    type: "ollama"
    config:
      model: "phi4-mini"

agents:
  a1:
    provider: "local"
    model: "phi4-mini"

tasks:
  t:
    prompt:
      user: "HELLO {{name}}"

run:
  name: "signed-run"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t"
        inputs:
          name: "world"
"#;
    let path = base.join("unsigned.adl.yaml");
    fs::write(&path, yaml).unwrap();
    path
}

#[test]
fn sign_verify_roundtrip_and_tamper_detection() {
    let base = tmp_dir("sign-roundtrip");
    let unsigned = write_unsigned_fixture(&base);
    let key_dir = base.join(".keys");
    let signed = base.join("signed.adl.yaml");

    let keygen = run_swarm(&["keygen", "--out-dir", key_dir.to_str().unwrap()]);
    assert!(
        keygen.status.success(),
        "keygen failed: {}",
        String::from_utf8_lossy(&keygen.stderr)
    );

    let sign = run_swarm(&[
        "sign",
        unsigned.to_str().unwrap(),
        "--key",
        key_dir.join("ed25519-private.b64").to_str().unwrap(),
        "--key-id",
        "dev-local",
        "--out",
        signed.to_str().unwrap(),
    ]);
    assert!(
        sign.status.success(),
        "sign failed: {}",
        String::from_utf8_lossy(&sign.stderr)
    );

    let verify = run_swarm(&[
        "verify",
        signed.to_str().unwrap(),
        "--key",
        key_dir.join("ed25519-public.b64").to_str().unwrap(),
    ]);
    assert!(
        verify.status.success(),
        "verify failed: {}",
        String::from_utf8_lossy(&verify.stderr)
    );

    // Tamper non-signature content => verification must fail.
    let tampered = base.join("tampered.adl.yaml");
    let mut content = fs::read_to_string(&signed).unwrap();
    content = content.replace("world", "tampered");
    fs::write(&tampered, content).unwrap();
    let verify_tampered = run_swarm(&[
        "verify",
        tampered.to_str().unwrap(),
        "--key",
        key_dir.join("ed25519-public.b64").to_str().unwrap(),
    ]);
    assert!(
        !verify_tampered.status.success(),
        "tampered doc should fail verification"
    );
}

#[test]
fn verify_ignores_signature_metadata_changes() {
    let base = tmp_dir("sign-metadata");
    let unsigned = write_unsigned_fixture(&base);
    let key_dir = base.join(".keys");
    let signed = base.join("signed.adl.yaml");
    run_swarm(&["keygen", "--out-dir", key_dir.to_str().unwrap()]);
    let sign = run_swarm(&[
        "sign",
        unsigned.to_str().unwrap(),
        "--key",
        key_dir.join("ed25519-private.b64").to_str().unwrap(),
        "--out",
        signed.to_str().unwrap(),
    ]);
    assert!(sign.status.success());

    let mut content = fs::read_to_string(&signed).unwrap();
    content = content.replace("key_id: dev-local", "key_id: changed-metadata");
    fs::write(&signed, content).unwrap();

    let verify = run_swarm(&[
        "verify",
        signed.to_str().unwrap(),
        "--key",
        key_dir.join("ed25519-public.b64").to_str().unwrap(),
    ]);
    assert!(
        verify.status.success(),
        "metadata-only signature changes should not fail verify"
    );
}

#[test]
fn run_enforces_signature_by_default_and_allows_override() {
    let base = tmp_dir("sign-enforce");
    let _bin = write_mock_ollama(&base);
    let _path_guard = EnvVarGuard::set("PATH", prepend_path(&base));
    let unsigned = write_unsigned_fixture(&base);

    let out_default = run_swarm(&[unsigned.to_str().unwrap(), "--run"]);
    assert!(
        !out_default.status.success(),
        "unsigned run should fail by default"
    );
    let stderr_default = String::from_utf8_lossy(&out_default.stderr);
    assert!(
        stderr_default.contains("signature enforcement failed"),
        "stderr:\n{stderr_default}"
    );

    let out_override = run_swarm(&[unsigned.to_str().unwrap(), "--run", "--allow-unsigned"]);
    assert!(
        out_override.status.success(),
        "allow-unsigned should bypass enforcement:\n{}",
        String::from_utf8_lossy(&out_override.stderr)
    );
}

#[test]
fn signed_run_executes_without_allow_unsigned() {
    let base = tmp_dir("sign-enforce-signed");
    let _bin = write_mock_ollama(&base);
    let _path_guard = EnvVarGuard::set("PATH", prepend_path(&base));
    let unsigned = write_unsigned_fixture(&base);
    let key_dir = base.join(".keys");
    let signed = base.join("signed.adl.yaml");

    let keygen = run_swarm(&["keygen", "--out-dir", key_dir.to_str().unwrap()]);
    assert!(keygen.status.success());
    let sign = run_swarm(&[
        "sign",
        unsigned.to_str().unwrap(),
        "--key",
        key_dir.join("ed25519-private.b64").to_str().unwrap(),
        "--out",
        signed.to_str().unwrap(),
    ]);
    assert!(sign.status.success(), "sign failed");

    let out = run_swarm(&[signed.to_str().unwrap(), "--run"]);
    assert!(
        out.status.success(),
        "signed run should pass without --allow-unsigned:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

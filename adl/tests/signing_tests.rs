use std::fs;
use std::path::Path;
use std::process::Command;

use ::adl::adl;
use ::adl::signing::{
    enforce_verification_profile, verify_doc_with_profile, VerificationErrorKind,
    VerificationKeySource, VerificationMetadata, VerificationProfile,
};

mod helpers;
use helpers::{unique_test_temp_dir, EnvVarGuard};

fn tmp_dir(prefix: &str) -> std::path::PathBuf {
    unique_test_temp_dir(prefix)
}

fn run_swarm(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_adl");
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
fn verify_rejects_signed_header_mutation() {
    let base = tmp_dir("sign-header-tamper");
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

    let content = fs::read_to_string(&signed).unwrap();
    let mut yaml: serde_yaml::Value = serde_yaml::from_str(&content).unwrap();
    let map = yaml.as_mapping_mut().unwrap();
    let signature = map
        .get_mut(serde_yaml::Value::String("signature".to_string()))
        .and_then(serde_yaml::Value::as_mapping_mut)
        .expect("signature mapping");
    let header = signature
        .get_mut(serde_yaml::Value::String("signed_header".to_string()))
        .and_then(serde_yaml::Value::as_mapping_mut)
        .expect("signed_header mapping");
    header.insert(
        serde_yaml::Value::String("adl_version".to_string()),
        serde_yaml::Value::String("0.9".to_string()),
    );
    fs::write(&signed, serde_yaml::to_string(&yaml).unwrap()).unwrap();

    let verify = run_swarm(&[
        "verify",
        signed.to_str().unwrap(),
        "--key",
        key_dir.join("ed25519-public.b64").to_str().unwrap(),
    ]);
    assert!(
        !verify.status.success(),
        "signed_header mutation must fail verification"
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
fn signed_run_requires_trusted_signature_key_by_default() {
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
        !out.status.success(),
        "signed run with only an embedded key should not satisfy runtime trust by default"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("SIGN_POLICY_DISALLOWED_KEY_SOURCE"),
        "stderr:\n{stderr}"
    );
    assert!(
        stderr.contains("--signature-key"),
        "stderr should guide operator toward trusted key source:\n{stderr}"
    );
}

#[test]
fn signed_run_executes_with_explicit_trusted_signature_key() {
    let base = tmp_dir("sign-enforce-explicit-key");
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

    let out = run_swarm(&[
        signed.to_str().unwrap(),
        "--run",
        "--signature-key",
        key_dir.join("ed25519-public.b64").to_str().unwrap(),
    ]);
    assert!(
        out.status.success(),
        "signed run should pass with an explicit trusted key:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn signed_run_executes_with_explicit_embedded_key_dev_override() {
    let base = tmp_dir("sign-enforce-embedded-dev");
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

    let out = run_swarm(&[
        signed.to_str().unwrap(),
        "--run",
        "--allow-embedded-signature-key",
    ]);
    assert!(
        out.status.success(),
        "explicit embedded-key dev override should preserve self-signed workflow runs:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn load_signed_doc_for_testing(base: &Path) -> (std::path::PathBuf, adl::AdlDoc) {
    let unsigned = write_unsigned_fixture(base);
    let key_dir = base.join(".keys");
    let signed = base.join("signed.adl.yaml");

    let keygen = run_swarm(&["keygen", "--out-dir", key_dir.to_str().unwrap()]);
    assert!(keygen.status.success(), "keygen failed");
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
    let raw = fs::read_to_string(&signed).expect("read signed doc");
    let doc: adl::AdlDoc = serde_yaml::from_str(&raw).expect("parse signed doc");
    (signed, doc)
}

#[test]
fn enforce_profile_rejects_missing_alg_for_signed_metadata() {
    let metadata = VerificationMetadata {
        signed: true,
        key_id: Some("dev-local"),
        alg: None,
        key_source: Some(VerificationKeySource::Embedded),
    };
    let profile = VerificationProfile::default();
    let err = enforce_verification_profile(&metadata, &profile).expect_err("missing alg must fail");
    assert_eq!(err.kind, VerificationErrorKind::MalformedSignatureMaterial);
    assert_eq!(err.code, "SIGN_MALFORMED_ALGORITHM");
}

#[test]
fn enforce_profile_rejects_disallowed_key_source() {
    let metadata = VerificationMetadata {
        signed: true,
        key_id: Some("dev-local"),
        alg: Some("ed25519"),
        key_source: Some(VerificationKeySource::Embedded),
    };
    let profile = VerificationProfile {
        require_signature: true,
        require_key_id: false,
        allowed_algs: vec!["ed25519".to_string()],
        allowed_key_sources: vec![VerificationKeySource::ExplicitKey],
    };
    let err =
        enforce_verification_profile(&metadata, &profile).expect_err("embedded key source denied");
    assert_eq!(err.kind, VerificationErrorKind::PolicyViolation);
    assert_eq!(err.code, "SIGN_POLICY_DISALLOWED_KEY_SOURCE");
    assert!(err.message.contains("embedded"), "message={}", err.message);
}

#[test]
fn verify_doc_with_profile_allows_unsigned_when_profile_allows_unsigned_without_sig_material() {
    let base = tmp_dir("verify-unsigned-no-sig");
    let unsigned = write_unsigned_fixture(&base);
    let raw = fs::read_to_string(&unsigned).expect("read unsigned doc");
    let doc: adl::AdlDoc = serde_yaml::from_str(&raw).expect("parse unsigned doc");
    let profile = VerificationProfile {
        require_signature: false,
        ..VerificationProfile::default()
    };
    verify_doc_with_profile(&doc, None, &profile)
        .expect("unsigned document should be allowed when signature is optional");
}

#[test]
fn verify_doc_with_profile_rejects_missing_key_source_for_signed_doc() {
    let base = tmp_dir("verify-missing-key-source");
    let (_, mut doc) = load_signed_doc_for_testing(&base);
    doc.signature.as_mut().expect("signature").public_key_b64 = None;

    let err = verify_doc_with_profile(&doc, None, &VerificationProfile::default())
        .expect_err("missing key source must fail");
    assert_eq!(err.kind, VerificationErrorKind::PolicyViolation);
    assert_eq!(err.code, "SIGN_POLICY_MISSING_KEY_SOURCE");
}

#[test]
fn verify_doc_with_profile_rejects_invalid_embedded_public_key_and_signature_material() {
    let base = tmp_dir("verify-invalid-embedded-pubkey");
    let (_, mut doc) = load_signed_doc_for_testing(&base);
    doc.signature.as_mut().expect("signature").public_key_b64 =
        Some("!!!not-base64!!!".to_string());

    let err = verify_doc_with_profile(&doc, None, &VerificationProfile::default())
        .expect_err("invalid embedded key must fail");
    assert_eq!(err.kind, VerificationErrorKind::MalformedSignatureMaterial);
    assert_eq!(err.code, "SIGN_MALFORMED_PUBLIC_KEY");

    let base2 = tmp_dir("verify-invalid-sig-material");
    let (_, mut doc2) = load_signed_doc_for_testing(&base2);
    doc2.signature.as_mut().expect("signature").sig_b64 = "not-base64".to_string();
    let err2 = verify_doc_with_profile(&doc2, None, &VerificationProfile::default())
        .expect_err("invalid signature b64 must fail");
    assert_eq!(err2.kind, VerificationErrorKind::MalformedSignatureMaterial);
    assert_eq!(err2.code, "SIGN_MALFORMED_SIGNATURE");
}

#[test]
fn verify_doc_with_profile_rejects_invalid_explicit_public_key_file() {
    assert_eq!(VerificationKeySource::ExplicitKey.as_str(), "explicit_key");

    let base = tmp_dir("verify-invalid-explicit-pubkey");
    let (_, doc) = load_signed_doc_for_testing(&base);
    let bad_key = base.join("bad-public-key.b64");
    fs::write(&bad_key, "not-base64-material").expect("write bad key");

    let err = verify_doc_with_profile(&doc, Some(&bad_key), &VerificationProfile::default())
        .expect_err("invalid explicit public key should fail");
    assert_eq!(err.kind, VerificationErrorKind::MalformedSignatureMaterial);
    assert_eq!(err.code, "SIGN_MALFORMED_PUBLIC_KEY");
}

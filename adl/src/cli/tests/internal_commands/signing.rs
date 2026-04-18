use super::super::*;

#[test]
fn cli_internal_keygen_sign_verify_roundtrip_succeeds() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base = std::env::temp_dir().join(format!("adl-main-keygen-{now}"));
    let key_dir = base.join("keys");
    std::fs::create_dir_all(&base).expect("create base dir");
    real_keygen(&[
        "--out-dir".to_string(),
        key_dir.to_string_lossy().to_string(),
    ])
    .expect("keygen should succeed");

    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/v0-5-pattern-linear.adl.yaml");
    let signed = base.join("signed.adl.yaml");
    real_sign(&[
        source.to_string_lossy().to_string(),
        "--key".to_string(),
        key_dir
            .join("ed25519-private.b64")
            .to_string_lossy()
            .to_string(),
        "--key-id".to_string(),
        "test-main".to_string(),
        "--out".to_string(),
        signed.to_string_lossy().to_string(),
    ])
    .expect("sign should succeed");

    real_verify(&[
        signed.to_string_lossy().to_string(),
        "--key".to_string(),
        key_dir
            .join("ed25519-public.b64")
            .to_string_lossy()
            .to_string(),
    ])
    .expect("verify should succeed");

    let _ = std::fs::remove_dir_all(base);
}

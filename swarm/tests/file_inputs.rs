use std::collections::HashMap;
use std::fs;
use std::path::Path;

use swarm::execute::materialize_inputs;

fn tmp_dir() -> std::path::PathBuf {
    std::env::temp_dir().join("swarm-file-input-tests")
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

#[test]
fn relative_path_resolves_against_base_dir() {
    let base = tmp_dir();
    let file = base.join("input.txt");
    write_file(&file, "hello world");

    let mut inputs = HashMap::new();
    inputs.insert("doc".to_string(), "@file:input.txt".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc"], "hello world");
}

#[test]
fn empty_file_directive_is_rejected() {
    let base = tmp_dir();

    let mut inputs = HashMap::new();
    inputs.insert("doc".to_string(), "@file:   ".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    assert!(
        err.to_string().contains("empty path"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_file_is_clear_error() {
    let base = tmp_dir();

    let mut inputs = HashMap::new();
    inputs.insert("doc".to_string(), "@file:no_such_file.txt".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = err.to_string();

    assert!(
        msg.contains("failed to stat input file"),
        "unexpected error: {msg}"
    );
}

#[test]
fn directory_is_rejected() {
    let base = tmp_dir();
    let dir = base.join("subdir");
    fs::create_dir_all(&dir).unwrap();

    let mut inputs = HashMap::new();
    inputs.insert("doc".to_string(), "@file:subdir".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = err.to_string();

    assert!(
        msg.contains("non-file path"),
        "unexpected error: {msg}"
    );
}

#[test]
fn invalid_utf8_is_rejected() {
    let base = tmp_dir();
    let file = base.join("binary.bin");

    // Write invalid UTF-8 bytes
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&file, vec![0xff, 0xfe, 0xfd]).unwrap();

    let mut inputs = HashMap::new();
    inputs.insert("doc".to_string(), "@file:binary.bin".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = err.to_string();

    assert!(
        msg.contains("not valid UTF-8"),
        "unexpected error: {msg}"
    );
}

#[test]
fn windows_newlines_are_normalized() {
    let base = tmp_dir();
    let file = base.join("crlf.txt");
    write_file(&file, "a\r\nb\r\nc\r\n");

    let mut inputs = HashMap::new();
    inputs.insert("doc".to_string(), "@file:crlf.txt".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc"], "a\nb\nc\n");
}
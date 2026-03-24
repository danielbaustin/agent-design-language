use super::*;

#[test]
fn materialize_inputs_leaves_non_file_values_unchanged() {
    let base = tmp_dir("mat-unchanged");
    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "hello".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc_1"], "hello");
}

#[test]
fn materialize_inputs_reads_relative_path_against_base_dir() {
    let base = tmp_dir("mat-rel");
    write_file(&base, "docs/doc_1.txt", b"abc");

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/doc_1.txt".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc_1"], "abc");
}

#[test]
fn materialize_inputs_accepts_quoted_paths() {
    let base = tmp_dir("mat-quoted");
    write_file(&base, "docs/doc_1.txt", b"abc");

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:'docs/doc_1.txt'".to_string());
    inputs.insert("doc_2".to_string(), "@file:\"docs/doc_1.txt\"".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc_1"], "abc");
    assert_eq!(out["doc_2"], "abc");
}

#[test]
fn materialize_inputs_rejects_empty_path() {
    let base = tmp_dir("mat-emptypath");
    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:   ".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("empty path"), "msg was: {msg}");
    assert!(msg.contains("doc_1"), "msg was: {msg}");
}

#[test]
fn materialize_inputs_errors_on_missing_file() {
    let base = tmp_dir("mat-missing");
    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/nope.txt".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("failed to stat input file"), "msg was: {msg}");
    assert!(msg.contains("doc_1"), "msg was: {msg}");
}

#[test]
fn materialize_inputs_normalizes_windows_newlines() {
    let base = tmp_dir("mat-newlines");
    write_file(&base, "docs/doc_1.txt", b"line1\r\nline2\r\n");

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/doc_1.txt".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc_1"], "line1\nline2\n");
}

#[test]
fn materialize_inputs_rejects_non_utf8() {
    let base = tmp_dir("mat-nonutf8");
    // Invalid UTF-8 byte sequence
    write_file(&base, "docs/bad.bin", &[0xff, 0xfe, 0xfd]);

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/bad.bin".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("not valid UTF-8"), "msg was: {msg}");
    assert!(msg.contains("doc_1"), "msg was: {msg}");
}

#[test]
fn materialize_inputs_accepts_exact_max_file_input_size() {
    let base = tmp_dir("mat-maxsize-exact");
    // Boundary check for the @file: materialization size limit.
    let max = MATERIALIZE_INPUT_MAX_FILE_BYTES as usize;
    let exact = vec![b'a'; max];
    write_file(&base, "docs/exact.txt", &exact);

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/exact.txt".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(
        out.get("doc_1").map(|s| s.len()),
        Some(max),
        "exact materialization MAX payload should be accepted"
    );
}

#[test]
fn materialize_inputs_rejects_max_plus_one_file_input_size() {
    let base = tmp_dir("mat-maxsize");
    // Boundary check for the @file: materialization size limit (MAX + 1).
    let max_plus_one = MATERIALIZE_INPUT_MAX_FILE_BYTES as usize + 1;
    let big = vec![b'a'; max_plus_one];
    write_file(&base, "docs/big.txt", &big);

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/big.txt".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("too large"), "msg was: {msg}");
    assert!(msg.contains("doc_1"), "msg was: {msg}");
}

use super::*;
use sha2::Digest;

fn test_args() -> CodeReviewArgs {
    CodeReviewArgs {
        out: std::path::PathBuf::from("artifacts/review"),
        backend: ReviewerBackend::Fixture,
        visibility_mode: VisibilityMode::PacketOnly,
        base_ref: "origin/main".to_string(),
        head_ref: "HEAD".to_string(),
        issue_number: Some(2603),
        writer_session: "writer".to_string(),
        reviewer_session: Some("reviewer".to_string()),
        model: None,
        allow_live_ollama: false,
        ollama_url: "http://127.0.0.1:11434".to_string(),
        timeout_secs: 120,
        include_working_tree: false,
        fixture_case: FixtureCase::Clean,
        max_diff_bytes: DEFAULT_REVIEW_EXCERPT_BYTES,
        include_files: Vec::new(),
    }
}

fn test_packet() -> ReviewPacket {
    ReviewPacket {
        schema_version: PACKET_SCHEMA,
        issue_number: Some(2603),
        branch: "codex/test".to_string(),
        base_ref: "origin/main".to_string(),
        head_ref: "HEAD".to_string(),
        visibility_mode: VisibilityMode::PacketOnly,
        changed_files: vec!["docs/example.md".to_string()],
        diff_summary: DiffSummary {
            files_changed: 1,
            max_diff_bytes: DEFAULT_REVIEW_EXCERPT_BYTES,
            max_diff_files: MAX_REVIEW_DIFF_FILES,
            max_context_files: MAX_REVIEW_CONTEXT_FILES,
            file_limit_truncated: false,
            truncated_hunks: false,
        },
        focused_diff_hunks: vec![DiffHunk {
            file: "docs/example.md".to_string(),
            diff_excerpt: "+example".to_string(),
            truncated: false,
        }],
        file_contexts: vec![FileContext {
            file: "docs/example.md".to_string(),
            current_excerpt: "example".to_string(),
            truncated: false,
            read_error: None,
        }],
        validation_evidence: Vec::new(),
        static_analysis_evidence: Vec::new(),
        repo_slice_manifest: RepoSliceManifest {
            read_only: false,
            write_allowed: false,
            tool_execution_allowed: false,
            files: vec!["docs/example.md".to_string()],
        },
        review_scope: "test review scope".to_string(),
        non_scope: vec!["do not edit files".to_string()],
        known_risks: Vec::new(),
        redaction_status: RedactionStatus {
            absolute_host_paths_present: false,
            secret_like_values_present: false,
        },
    }
}

fn unique_temp_path(label: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "adl-code-review-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time")
            .as_nanos()
    ))
}

fn init_temp_git_repo(label: &str) -> std::path::PathBuf {
    let root = unique_temp_path(label);
    std::fs::create_dir_all(&root).expect("create temp repo");
    std::process::Command::new("git")
        .arg("init")
        .current_dir(&root)
        .output()
        .expect("git init");
    std::process::Command::new("git")
        .args(["config", "user.email", "codex@example.test"])
        .current_dir(&root)
        .output()
        .expect("git config email");
    std::process::Command::new("git")
        .args(["config", "user.name", "Codex Test"])
        .current_dir(&root)
        .output()
        .expect("git config name");
    root
}

fn init_temp_git_repo_with_changed_file(
    label: &str,
    relative_path: &str,
    initial: &str,
    changed: &str,
) -> std::path::PathBuf {
    let root = init_temp_git_repo(label);
    let path = root.join(relative_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create file parent");
    }
    std::fs::write(&path, initial).expect("write initial file");
    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(&root)
        .output()
        .expect("git add initial");
    std::process::Command::new("git")
        .args(["commit", "-m", "initial"])
        .current_dir(&root)
        .output()
        .expect("git commit initial");
    std::fs::write(&path, changed).expect("write changed file");
    root
}

#[test]
fn parse_args_preserves_backend_after_out_and_accepts_timeout() {
    let args = vec![
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--backend".to_string(),
        "ollama".to_string(),
        "--timeout-secs".to_string(),
        "240".to_string(),
        "--include-working-tree".to_string(),
        "--file".to_string(),
        "adl/src/cli/tooling_cmd/code_review.rs".to_string(),
    ];
    let parsed = parse_args(&args).expect("args should parse");
    assert_eq!(parsed.backend, ReviewerBackend::Ollama);
    assert_eq!(parsed.timeout_secs, 240);
    assert!(parsed.include_working_tree);
    assert_eq!(
        parsed.include_files,
        vec!["adl/src/cli/tooling_cmd/code_review.rs"]
    );
}

#[test]
fn parse_args_excludes_working_tree_by_default() {
    let args = vec!["--out".to_string(), "artifacts/review".to_string()];
    let parsed = parse_args(&args).expect("args should parse");
    assert!(!parsed.include_working_tree);
}

#[test]
fn parse_args_rejects_invalid_values_and_missing_required_out() {
    assert!(parse_args(&["--backend".to_string(), "bad".to_string()]).is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--visibility".to_string(),
        "bad".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--fixture-case".to_string(),
        "bad".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--timeout-secs".to_string(),
        "0".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--max-diff-bytes".to_string(),
        "255".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--max-diff-bytes".to_string(),
        (MAX_REVIEW_EXCERPT_BYTES + 1).to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--file".to_string(),
        "../secret.txt".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--file".to_string(),
        "/tmp/secret.txt".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--file".to_string(),
        "adl\\\\src\\\\lib.rs".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--file".to_string(),
        "docs/file with spaces.md".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--file".to_string(),
        "docs/ref:path.md".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--file".to_string(),
        ".env".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--file".to_string(),
        "config/private.pem".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--file".to_string(),
        ".ssh/config".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--head".to_string(),
        "--help".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--head".to_string(),
        "HEAD:secret".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--head".to_string(),
        "HEAD --cached".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--base".to_string(),
        "origin/main..topic".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--base".to_string(),
        "953e913f^".to_string(),
    ])
    .is_err());
    assert!(parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--base".to_string(),
        "953e913f^^1".to_string(),
    ])
    .is_err());
    assert!(parse_args(&["--writer-session".to_string(), "".to_string()]).is_err());
    assert!(parse_args(&["--out".to_string()]).is_err());
    assert!(parse_args(&["--unknown".to_string()]).is_err());
}

#[test]
fn parse_args_accepts_safe_parent_and_ancestor_git_refs() {
    let parsed = parse_args(&[
        "--out".to_string(),
        "artifacts/review".to_string(),
        "--base".to_string(),
        "953e913f^1".to_string(),
        "--head".to_string(),
        "HEAD~1".to_string(),
    ])
    .expect("safe parent and ancestor refs should parse");
    assert_eq!(parsed.base_ref, "953e913f^1");
    assert_eq!(parsed.head_ref, "HEAD~1");
}

#[test]
fn changed_files_accepts_file_filter_inside_changed_set() {
    let root = init_temp_git_repo_with_changed_file(
        "changed-files-filter",
        "src/sample.rs",
        "fn sample() { println!(\"v1\"); }\n",
        "fn sample() { println!(\"v2\"); }\n",
    );
    let mut args = test_args();
    args.base_ref = "HEAD".to_string();
    args.head_ref = "HEAD".to_string();
    args.include_working_tree = true;
    args.include_files = vec!["src/sample.rs".to_string()];

    let files = changed_files(&root, &args).expect("changed file filter should pass");
    assert_eq!(files, vec!["src/sample.rs".to_string()]);
    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn changed_files_rejects_file_filter_outside_changed_set() {
    let root = init_temp_git_repo_with_changed_file(
        "changed-files-reject",
        "src/sample.rs",
        "fn sample() { println!(\"v1\"); }\n",
        "fn sample() { println!(\"v2\"); }\n",
    );
    let mut args = test_args();
    args.base_ref = "HEAD".to_string();
    args.head_ref = "HEAD".to_string();
    args.include_working_tree = true;
    args.include_files = vec!["src/other.rs".to_string()];

    let err = changed_files(&root, &args).expect_err("unchanged explicit file should fail");
    assert!(err
        .to_string()
        .contains("--file 'src/other.rs' is not in the changed file set"));
    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn build_packet_covers_repo_slice_context_and_static_evidence() {
    let root = init_temp_git_repo_with_changed_file(
        "build-packet-static",
        "src/sample.rs",
        "fn sample() { println!(\"v1\"); }\n",
        "fn sample() { println!(\"v2\"); }\n",
    );
    let mut args = test_args();
    args.base_ref = "HEAD".to_string();
    args.head_ref = "HEAD".to_string();
    args.include_working_tree = true;
    args.visibility_mode = VisibilityMode::ReadOnlyRepo;
    args.include_files = vec!["src/sample.rs".to_string()];

    let packet = code_review_build::build_packet(&root, &args).expect("build review packet");
    assert_eq!(
        packet.schema_version,
        code_review_types::CODE_REVIEW_PACKET_SCHEMA
    );
    assert_eq!(packet.changed_files, vec!["src/sample.rs".to_string()]);
    assert_eq!(packet.diff_summary.files_changed, 1);
    assert!(!packet.branch.trim().is_empty());
    assert!(packet
        .file_contexts
        .first()
        .expect("file context")
        .current_excerpt
        .contains("println!(\"v2\")"));
    assert_eq!(packet.static_analysis_evidence.len(), 2);
    assert_eq!(packet.static_analysis_evidence[0].status, "PASS");
    assert_eq!(packet.static_analysis_evidence[1].status, "PASS");
    assert!(packet.repo_slice_manifest.read_only);
    assert!(!packet.repo_slice_manifest.write_allowed);
    assert!(!packet.redaction_status.absolute_host_paths_present);
    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn build_packet_includes_staged_and_unstaged_worktree_diffs() {
    let root = init_temp_git_repo_with_changed_file(
        "worktree-diff",
        "src/sample.rs",
        "fn sample() {\n    println!(\"v1\");\n}\n",
        "fn sample() {\n    println!(\"v1 working tree\");\n}\n",
    );
    let source_file = root.join("src/sample.rs");
    std::fs::write(
        &source_file,
        "fn sample() {\n    println!(\"v2 staged\");\n}\n",
    )
    .expect("write staged content");
    std::process::Command::new("git")
        .args(["add", "src/sample.rs"])
        .current_dir(&root)
        .output()
        .expect("git add staged content");
    std::fs::write(
        &source_file,
        "fn sample() {\n    println!(\"v2 staged\");\n    println!(\"v3 unstaged\");\n}\n",
    )
    .expect("write unstaged content");

    let mut args = test_args();
    args.base_ref = "HEAD".to_string();
    args.head_ref = "HEAD".to_string();
    args.include_working_tree = true;
    args.include_files = vec!["src/sample.rs".to_string()];
    args.max_diff_bytes = 4096;

    let packet = code_review_build::build_packet(&root, &args).expect("build review packet");
    let hunk = packet.focused_diff_hunks.first().expect("diff hunk");
    assert!(hunk
        .diff_excerpt
        .contains("--- staged working tree diff ---"));
    assert!(hunk
        .diff_excerpt
        .contains("--- unstaged working tree diff ---"));
    assert!(packet.file_contexts[0]
        .current_excerpt
        .contains("v3 unstaged"));
    assert_eq!(packet.static_analysis_evidence.len(), 2);
    assert_eq!(
        packet.static_analysis_evidence[0].summary,
        "working tree whitespace check"
    );
    assert_eq!(
        packet.static_analysis_evidence[1].summary,
        "committed diff whitespace check"
    );

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn real_code_review_fixture_run_writes_expected_artifacts() {
    let out = unique_temp_path("fixture-run");
    let args = vec![
        "--out".to_string(),
        out.display().to_string(),
        "--backend".to_string(),
        "fixture".to_string(),
        "--base".to_string(),
        "origin/main".to_string(),
        "--head".to_string(),
        "HEAD".to_string(),
        "--file".to_string(),
        "adl/src/cli/tooling_cmd/code_review.rs".to_string(),
    ];

    real_code_review(&args).expect("run fixture code review");

    let packet = std::fs::read_to_string(out.join("review_packet.json")).expect("read packet");
    let result = std::fs::read_to_string(out.join("review_result.json")).expect("read result");
    let gate = std::fs::read_to_string(out.join("gate_result.json")).expect("read gate");
    let summary = std::fs::read_to_string(out.join("run_summary.json")).expect("read summary");
    let packet_json: serde_json::Value =
        serde_json::from_str(&packet).expect("parse review packet");
    let result_json: serde_json::Value =
        serde_json::from_str(&result).expect("parse review result");
    let gate_json: serde_json::Value = serde_json::from_str(&gate).expect("parse gate result");
    let summary_json: serde_json::Value =
        serde_json::from_str(&summary).expect("parse run summary");
    let expected_packet_id_seed = format!(
        "{}:{}:{}:{}",
        packet_json
            .get("issue_number")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or_default(),
        packet_json
            .get("branch")
            .and_then(serde_json::Value::as_str)
            .expect("packet branch"),
        packet_json
            .get("changed_files")
            .and_then(serde_json::Value::as_array)
            .map(std::vec::Vec::len)
            .unwrap_or_default(),
        packet_json
            .get("diff_summary")
            .and_then(serde_json::Value::as_object)
            .and_then(|summary| summary.get("truncated_hunks"))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    );
    let expected_packet_id = format!(
        "{:x}",
        sha2::Sha256::digest(expected_packet_id_seed.as_bytes())
    )[..16]
        .to_string();

    assert!(packet.contains(code_review_types::CODE_REVIEW_PACKET_SCHEMA));
    assert!(result.contains(code_review_types::CODE_REVIEW_RESULT_SCHEMA));
    assert_eq!(
        result_json
            .get("packet_id")
            .and_then(serde_json::Value::as_str),
        Some(expected_packet_id.as_str())
    );
    assert_eq!(
        gate_json
            .get("pr_open_allowed")
            .and_then(serde_json::Value::as_bool),
        Some(false)
    );
    assert!(summary.contains(code_review_types::CODE_REVIEW_SUMMARY_SCHEMA));
    assert_eq!(
        summary_json
            .get("backend")
            .and_then(serde_json::Value::as_str),
        Some("fixture")
    );

    std::fs::remove_dir_all(&out).ok();
}

#[test]
fn read_file_prefix_bounds_file_context_memory() {
    let path =
        std::env::temp_dir().join(format!("adl-code-review-prefix-{}.txt", std::process::id()));
    std::fs::write(&path, "abcdef").expect("write temp");
    let text = read_file_prefix(&path, 3).expect("read prefix");
    std::fs::remove_file(&path).ok();
    assert_eq!(text, "abc");
}

#[test]
fn helpers_cover_safe_worktree_git_output_git_show_and_write_json() {
    let root = super::super::common::repo_root().expect("repo root");
    let canonical_root = root.canonicalize().expect("canonical root");
    let excerpt = code_review_helpers::safe_read_worktree_file(
        &root,
        &canonical_root,
        "adl/src/cli/tooling_cmd/code_review.rs",
        96,
    )
    .expect("safe read");
    assert!(excerpt.contains("mod code_review_args;"));

    let head_excerpt = code_review_helpers::git_show_file_prefix(
        &root,
        "HEAD",
        "adl/src/cli/tooling_cmd/code_review.rs",
        96,
    )
    .expect("git show excerpt");
    assert!(head_excerpt.contains("mod code_review_args;"));

    let head = code_review_helpers::git_output(&root, &["rev-parse", "--abbrev-ref", "HEAD"])
        .expect("git output");
    assert!(!head.trim().is_empty());

    let out = std::env::temp_dir().join(format!(
        "adl-code-review-write-json-{}-{}.json",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time")
            .as_nanos()
    ));
    code_review_helpers::write_json(&out, &test_packet()).expect("write review packet");
    let written = std::fs::read_to_string(&out).expect("read written packet");
    std::fs::remove_file(&out).ok();
    assert!(written.contains("\"schema_version\": \"adl.pr_review_packet.v1\""));
    assert!(written.contains("\"review_scope\": \"test review scope\""));
}

#[test]
fn fixture_review_covers_blocked_and_same_session_paths() {
    let packet = test_packet();
    let packet_id = "packet-id";

    let mut blocked_args = test_args();
    blocked_args.fixture_case = FixtureCase::Blocked;
    let blocked = fixture_review(&blocked_args, &packet, packet_id);
    assert_eq!(blocked.disposition, ReviewDisposition::Blocked);
    assert_eq!(blocked.findings.len(), 1);

    let clean = fixture_review(&test_args(), &packet, packet_id);
    assert_eq!(clean.disposition, ReviewDisposition::NonProving);
    let clean_gate = evaluate_gate(&clean, &packet);
    assert!(!clean_gate.pr_open_allowed);

    let mut same_session_args = test_args();
    same_session_args.reviewer_session = Some("writer".to_string());
    let same_session = fixture_review(&same_session_args, &packet, packet_id);
    assert_eq!(same_session.disposition, ReviewDisposition::NonProving);
    assert!(same_session.same_session_as_writer);
}

#[test]
fn reviewer_result_helpers_cover_skipped_non_proving_and_run_reviewer() {
    let args = test_args();
    let packet = test_packet();

    let skipped = code_review_reviewer::skipped_review_result(
        &args,
        &packet,
        "packet-id",
        "writer".to_string(),
        "gemma4:test".to_string(),
        "live review disabled".to_string(),
    );
    assert_eq!(skipped.disposition, ReviewDisposition::Skipped);
    assert!(skipped.same_session_as_writer);
    assert_eq!(skipped.reviewer_backend, "fixture");

    let non_proving = code_review_reviewer::non_proving_review_result(
        &args,
        &packet,
        "packet-id",
        "reviewer".to_string(),
        "gemma4:test".to_string(),
        "packet contained redaction risk".to_string(),
    );
    assert_eq!(non_proving.disposition, ReviewDisposition::NonProving);
    assert!(!non_proving.same_session_as_writer);

    let fixture = run_reviewer(&args, &packet, "packet-id").expect("fixture run");
    assert_eq!(fixture.disposition, ReviewDisposition::NonProving);
    assert_eq!(fixture.packet_id, "packet-id");
}

#[test]
fn ollama_without_live_returns_skipped_blocker() {
    let mut args = test_args();
    args.backend = ReviewerBackend::Ollama;
    args.model = Some("gemma4:test".to_string());
    let packet = test_packet();
    let result = run_reviewer(&args, &packet, "packet-id").expect("skipped review");
    assert_eq!(result.disposition, ReviewDisposition::Skipped);
    let gate = evaluate_gate(&result, &packet);
    assert!(!gate.pr_open_allowed);
    assert!(gate.reasons.iter().any(|reason| reason.contains("skipped")));
}

#[test]
fn ollama_live_review_suppresses_redaction_failures() {
    let mut args = test_args();
    args.backend = ReviewerBackend::Ollama;
    args.allow_live_ollama = true;
    args.model = Some("gemma4:test".to_string());
    let mut packet = test_packet();
    packet.redaction_status.secret_like_values_present = true;

    let result =
        run_reviewer(&args, &packet, "packet-id").expect("redaction failure should not call");
    assert_eq!(result.disposition, ReviewDisposition::NonProving);
    assert!(result
        .residual_risk
        .iter()
        .any(|risk| { risk.contains("live model invocation suppressed") }));
    let gate = evaluate_gate(&result, &packet);
    assert!(!gate.pr_open_allowed);
}

#[test]
fn evaluate_gate_covers_static_failure_and_blocking_finding() {
    let mut packet = test_packet();
    packet.static_analysis_evidence.push(ValidationEvidence {
        command: "git diff --check".to_string(),
        status: "FAIL".to_string(),
        summary: "whitespace error".to_string(),
    });
    packet.redaction_status.absolute_host_paths_present = true;
    let mut result = review_result(
        &test_args(),
        &packet,
        "packet-id",
        ReviewResultPartsCompat {
            reviewer_session: "reviewer".to_string(),
            reviewer_model: "fixture".to_string(),
            same_session: false,
            disposition: ReviewDisposition::Blessed,
            findings: vec![ReviewFinding {
                title: "Blocking issue".to_string(),
                priority: "P2".to_string(),
                file: "docs/example.md".to_string(),
                line: Some(1),
                body: "A concrete problem exists.".to_string(),
                evidence: vec!["path:docs/example.md".to_string()],
                heuristic_ids: vec!["T1".to_string()],
                confidence: "high".to_string(),
                blocking: true,
                suggested_fix_scope: "issue_local".to_string(),
            }],
            residual_risk: Vec::new(),
        },
    );
    result.same_session_as_writer = true;
    let gate = evaluate_gate(&result, &packet);
    assert!(!gate.pr_open_allowed);
    assert!(gate
        .reasons
        .iter()
        .any(|reason| reason.contains("static analysis")));
    assert!(gate
        .reasons
        .iter()
        .any(|reason| reason.contains("absolute host paths")));
    assert!(gate
        .reasons
        .iter()
        .any(|reason| reason.contains("blocking P2 finding")));
    assert!(gate
        .reasons
        .iter()
        .any(|reason| reason.contains("matches writer session")));

    result.disposition = ReviewDisposition::NonProving;
    result.same_session_as_writer = false;
    packet.static_analysis_evidence.clear();
    let non_proving_gate = evaluate_gate(&result, &packet);
    assert!(!non_proving_gate.pr_open_allowed);
    assert!(non_proving_gate
        .reasons
        .iter()
        .any(|reason| reason.contains("non_proving")));
    assert!(!non_proving_gate
        .reasons
        .iter()
        .any(|reason| reason.contains("blocking P2 finding")));
}

#[test]
fn reviewer_types_and_gate_cover_enum_and_schema_surfaces() {
    assert_eq!(ReviewerBackend::Fixture.as_str(), "fixture");
    assert_eq!(ReviewerBackend::Ollama.as_str(), "ollama");

    let packet_json =
        serde_json::to_string(&test_packet()).expect("serialize packet for schema visibility");
    assert!(packet_json.contains(code_review_types::CODE_REVIEW_PACKET_SCHEMA));

    let result = review_result(
        &test_args(),
        &test_packet(),
        "packet-id",
        ReviewResultPartsCompat {
            reviewer_session: "reviewer".to_string(),
            reviewer_model: "gemma4:test".to_string(),
            same_session: false,
            disposition: ReviewDisposition::Blessed,
            findings: Vec::new(),
            residual_risk: vec![
                "Reviewed docs/example.md context and found no actionable defects.".to_string(),
            ],
        },
    );
    let gate = evaluate_gate(&result, &test_packet());
    assert!(gate.pr_open_allowed);
    assert_eq!(gate.gate_disposition, "allow_with_evidence");

    let result_json = serde_json::to_string(&result).expect("serialize review result");
    assert!(result_json.contains(code_review_types::CODE_REVIEW_RESULT_SCHEMA));
    assert!(result_json.contains("\"visibility_mode\":\"packet_only\""));
}

#[test]
fn parse_model_review_json_accepts_fenced_json_and_filters_string_arrays() {
    let raw = r#"```json
{
  "disposition": "blocked",
  "findings": [
    {
      "title": "Missing evidence",
      "priority": "P3",
      "file": "docs/example.md",
      "line": 12,
      "body": "The example references a fact without evidence.",
      "evidence": ["path:docs/example.md", 42],
      "heuristic_ids": ["D1", false],
      "confidence": "medium",
      "blocking": false,
      "suggested_fix_scope": "issue_local"
    }
  ]
}
"#;
    let parsed = parse_model_review_json(raw).expect("parse review json");
    assert_eq!(parsed.disposition, ReviewDisposition::Blocked);
    assert_eq!(parsed.findings[0].evidence, vec!["path:docs/example.md"]);
    assert_eq!(parsed.findings[0].heuristic_ids, vec!["D1"]);
    assert_eq!(parsed.findings[0].line, Some(12));
    assert!(parse_model_review_json("{not-json").is_none());
    assert!(parse_model_review_json(r#"{"disposition":"unexpected"}"#).is_none());
}

#[test]
fn helpers_cover_url_normalization_prompt_and_unicode_truncation() {
    assert_eq!(
        ollama_generate_url("http://127.0.0.1:11434").expect("url"),
        "http://127.0.0.1:11434/api/generate"
    );
    assert_eq!(
        ollama_generate_url("http://127.0.0.1:11434/api/generate").expect("url"),
        "http://127.0.0.1:11434/api/generate"
    );
    assert!(ollama_generate_url("not a url").is_err());

    let prompt = reviewer_prompt(&test_packet());
    assert!(prompt.contains("actionable risks"));
    assert!(prompt.contains("adl.pr_review_result.v1"));
    assert_eq!(
        redact_absolute_host_paths_for_prompt("/tmp/secret.txt /Users/alice/repo C:\\\\secret"),
        "[REDACTED_HOST_PATH]/secret.txt [REDACTED_HOST_PATH]/alice/repo [REDACTED_HOST_PATH]\\\\\\secret"
    );
    assert!(!contains_review_absolute_host_path("Expected signal:\\\\n"));
    assert!(contains_review_absolute_host_path("C:\\\\secret"));
    assert!(!contains_review_absolute_host_path("tokenC:\\\\secret"));

    let (truncated, was_truncated) = truncate("éclair", 3);
    assert!(was_truncated);
    assert_eq!(truncated, "éc");
}

#[test]
fn normalize_model_review_rejects_placeholder_findings() {
    let finding = ReviewFinding {
        title: "Untitled model finding".to_string(),
        priority: "P3".to_string(),
        file: "unknown".to_string(),
        line: None,
        body: String::new(),
        evidence: Vec::new(),
        heuristic_ids: Vec::new(),
        confidence: "medium".to_string(),
        blocking: false,
        suggested_fix_scope: "issue_local".to_string(),
    };

    let (disposition, findings, residual) = normalize_model_review(
            ParsedModelReview {
                disposition: ReviewDisposition::Blessed,
                findings: vec![finding],
                residual_risk: vec![
                    "Reviewed docs/example.md finding evidence and no packet truncation flags were present."
                        .to_string(),
                ],
            },
            &test_packet(),
        );
    assert_eq!(disposition, ReviewDisposition::NonProving);
    assert_eq!(findings.len(), 1);
    assert!(residual
        .iter()
        .any(|risk| risk.contains("malformed findings")));
    assert!(residual
        .iter()
        .any(|risk| risk.contains("missing concrete evidence")));
}

#[test]
fn normalize_model_review_rejects_praise_as_findings() {
    let finding = ReviewFinding {
        title: "Correctly separates proposal and execution".to_string(),
        priority: "P3".to_string(),
        file: "docs/example.md".to_string(),
        line: None,
        body: "The change correctly preserves the intended safety boundary.".to_string(),
        evidence: vec!["diff excerpt".to_string()],
        heuristic_ids: vec!["ADL-REVIEW".to_string()],
        confidence: "high".to_string(),
        blocking: false,
        suggested_fix_scope: "None. This is already correct.".to_string(),
    };

    let (disposition, findings, residual) = normalize_model_review(
        ParsedModelReview {
            disposition: ReviewDisposition::Blessed,
            findings: vec![finding],
            residual_risk: Vec::new(),
        },
        &test_packet(),
    );
    assert_eq!(disposition, ReviewDisposition::NonProving);
    assert_eq!(findings.len(), 1);
    assert!(residual
        .iter()
        .any(|risk| risk.contains("non-actionable praise")));
}

#[test]
fn normalize_model_review_accepts_specific_evidenced_finding() {
    let finding = ReviewFinding {
        title: "Missing reviewer evidence link".to_string(),
        priority: "P3".to_string(),
        file: "docs/example.md".to_string(),
        line: Some(12),
        body: "The review packet mentions a follow-up but does not link the evidence.".to_string(),
        evidence: vec!["path:docs/example.md".to_string()],
        heuristic_ids: vec!["C1".to_string()],
        confidence: "medium".to_string(),
        blocking: false,
        suggested_fix_scope: "issue_local".to_string(),
    };

    let (disposition, findings, residual) = normalize_model_review(
            ParsedModelReview {
                disposition: ReviewDisposition::Blessed,
                findings: vec![finding],
                residual_risk: vec![
                    "Reviewed docs/example.md finding evidence and no packet truncation flags were present."
                        .to_string(),
                ],
            },
            &test_packet(),
        );
    assert_eq!(disposition, ReviewDisposition::Blessed);
    assert_eq!(findings.len(), 1);
    assert_eq!(residual.len(), 1);
}

#[test]
fn normalize_model_review_requires_bypass_for_blocking_security_findings() {
    let finding = ReviewFinding {
        title: "Possible path traversal".to_string(),
        priority: "P0".to_string(),
        file: "adl/src/cli/tooling_cmd/code_review.rs".to_string(),
        line: None,
        body: "The path might be exploitable without a concrete bypass.".to_string(),
        evidence: vec!["safe_read_worktree_file".to_string()],
        heuristic_ids: vec!["SEC-PATH-TRAVERSAL".to_string()],
        confidence: "medium".to_string(),
        blocking: true,
        suggested_fix_scope: "function_local".to_string(),
    };

    let (disposition, _, residual) = normalize_model_review(
        ParsedModelReview {
            disposition: ReviewDisposition::Blocked,
            findings: vec![finding],
            residual_risk: Vec::new(),
        },
        &test_packet(),
    );

    assert_eq!(disposition, ReviewDisposition::NonProving);
    assert!(residual
        .iter()
        .any(|risk| risk.contains("without concrete bypass evidence")));
}

#[test]
fn normalize_model_review_rejects_bypass_rejected_by_cli_validators() {
    let finding = ReviewFinding {
        title: "Path traversal bypass".to_string(),
        priority: "P1".to_string(),
        file: "adl/src/cli/tooling_cmd/code_review.rs".to_string(),
        line: None,
        body: "The model claims --file traversal is accepted.".to_string(),
        evidence: vec!["bypass: --file ../../etc/passwd".to_string()],
        heuristic_ids: vec!["SEC-PATH-TRAVERSAL".to_string()],
        confidence: "high".to_string(),
        blocking: true,
        suggested_fix_scope: "function_local".to_string(),
    };

    let (disposition, _, residual) = normalize_model_review(
        ParsedModelReview {
            disposition: ReviewDisposition::Blocked,
            findings: vec![finding],
            residual_risk: Vec::new(),
        },
        &test_packet(),
    );

    assert_eq!(disposition, ReviewDisposition::NonProving);
    assert!(residual
        .iter()
        .any(|risk| risk.contains("rejected by the current CLI validators")));
}

#[test]
fn normalize_model_review_rejects_empty_blessing_for_truncated_packet() {
    let mut packet = test_packet();
    packet.diff_summary.truncated_hunks = true;
    packet.focused_diff_hunks[0].truncated = true;
    packet.file_contexts[0].truncated = true;

    let (disposition, findings, residual) = normalize_model_review(
        ParsedModelReview {
            disposition: ReviewDisposition::Blessed,
            findings: Vec::new(),
            residual_risk: Vec::new(),
        },
        &packet,
    );

    assert_eq!(disposition, ReviewDisposition::NonProving);
    assert!(findings.is_empty());
    assert!(residual
        .iter()
        .any(|risk| risk.contains("incomplete packet")));
    assert!(residual
        .iter()
        .any(|risk| risk.contains("no residual-review rationale")));
}

#[test]
fn normalize_model_review_accepts_empty_blessing_with_rationale_for_complete_packet() {
    let (disposition, findings, residual) = normalize_model_review(
            ParsedModelReview {
                disposition: ReviewDisposition::Blessed,
                findings: Vec::new(),
                residual_risk: vec![
                    "Reviewed complete docs/example.md diff, including file_contexts and focused_diff_hunks; no actionable defect evidence was present."
                        .to_string(),
                ],
            },
            &test_packet(),
        );

    assert_eq!(disposition, ReviewDisposition::Blessed);
    assert!(findings.is_empty());
    assert_eq!(residual.len(), 1);
}

#[test]
fn normalize_model_review_rejects_blessed_incomplete_packet_with_findings() {
    let finding = ReviewFinding {
        title: "Missing reviewer evidence link".to_string(),
        priority: "P3".to_string(),
        file: "docs/example.md".to_string(),
        line: Some(12),
        body: "The review packet mentions a follow-up but does not link the evidence.".to_string(),
        evidence: vec!["path:docs/example.md".to_string()],
        heuristic_ids: vec!["C1".to_string()],
        confidence: "medium".to_string(),
        blocking: false,
        suggested_fix_scope: "issue_local".to_string(),
    };
    let mut packet = test_packet();
    packet.diff_summary.truncated_hunks = true;

    let (disposition, findings, residual) = normalize_model_review(
        ParsedModelReview {
            disposition: ReviewDisposition::Blessed,
            findings: vec![finding],
            residual_risk: vec![
                "Reviewed docs/example.md finding evidence and packet limits.".to_string(),
            ],
        },
        &packet,
    );

    assert_eq!(disposition, ReviewDisposition::NonProving);
    assert_eq!(findings.len(), 1);
    assert!(residual
        .iter()
        .any(|risk| risk.contains("incomplete packet")));
}

#[test]
fn normalize_model_review_rejects_empty_blessing_with_praise_rationale() {
    let (disposition, findings, residual) = normalize_model_review(
        ParsedModelReview {
            disposition: ReviewDisposition::Blessed,
            findings: Vec::new(),
            residual_risk: vec![
                "The implementation is logically sound and correctly tested.".to_string(),
            ],
        },
        &test_packet(),
    );

    assert_eq!(disposition, ReviewDisposition::NonProving);
    assert!(findings.is_empty());
    assert!(residual
        .iter()
        .any(|risk| risk.contains("praise-like residual rationale")));
}

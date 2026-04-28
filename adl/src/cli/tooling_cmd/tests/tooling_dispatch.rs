use super::support::*;
use super::*;

#[test]
fn tooling_dispatch_and_help_paths_cover_public_entrypoint() {
    let repo = TempRepo::new("dispatch");
    let input = repo.write_rel(
        ".tmp/tooling_cmd_tests/input.md",
        &valid_input_card_text(1374, ".tmp/tooling_cmd_tests/output.md"),
    );
    let output = repo.write_rel(".tmp/tooling_cmd_tests/output.md", &valid_sor_text());
    let review = repo.write_rel(".tmp/tooling_cmd_tests/review.md", &valid_review_markdown());
    let review_output = repo.write_rel(
        ".tmp/tooling_cmd_tests/review-output.yaml",
        &valid_review_output_yaml(repo.path()),
    );
    let stp = repo.write_rel(".tmp/tooling_cmd_tests/stp.md", &valid_stp_text());
    let sip = repo.write_rel(
        ".tmp/tooling_cmd_tests/sip.md",
        &valid_sip_text(1374, repo.path()),
    );
    let prompt_out = repo.path().join("prompt.txt");

    assert!(real_tooling(&[]).is_err());
    real_tooling(&["help".to_string()]).expect("help should succeed");
    assert!(real_tooling(&["unknown".to_string()]).is_err());
    real_tooling(&["code-review".to_string(), "--help".to_string()])
        .expect("code-review help should succeed without --out");

    real_tooling(&[
        "card-prompt".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--out".to_string(),
        prompt_out.to_string_lossy().to_string(),
    ])
    .expect("card-prompt dispatch should succeed");
    assert!(prompt_out.is_file());

    let code_review_out = repo.path().join("code-review-clean");
    real_tooling(&[
        "code-review".to_string(),
        "--out".to_string(),
        code_review_out.to_string_lossy().to_string(),
        "--backend".to_string(),
        "fixture".to_string(),
        "--visibility".to_string(),
        "packet-only".to_string(),
        "--writer-session".to_string(),
        "writer-a".to_string(),
        "--reviewer-session".to_string(),
        "reviewer-a".to_string(),
    ])
    .expect("code-review fixture dispatch should succeed");
    assert!(code_review_out.join("review_packet.json").is_file());
    assert!(code_review_out.join("review_result.json").is_file());
    assert!(code_review_out.join("gate_result.json").is_file());

    let wbs = repo.write_rel(
        ".tmp/tooling_cmd_tests/wbs.md",
        "# Work Breakdown Structure (WBS) - v0.88\n\n## Work Packages\n\n| ID | Work Package | Description | Deliverable | Dependencies | Issue |\n|---|---|---|---|---|---|\n| WP-01 | Canonical planning package | docs | docs | none | `#1` |\n| WP-02 | Chronosense foundation | chrono | proof hook | `WP-01` | execution issue to be seeded |\n| WP-14 | Coverage / quality gate | quality | green gate | `WP-13` | closeout issue to be seeded |\n",
    );
    let sprint = repo.write_rel(
        ".tmp/tooling_cmd_tests/sprint.md",
        "# Sprint Plan - v0.88\n\n## Sprint Overview\n\n| Sprint | Purpose | WPs | Current status |\n|---|---|---|---|\n| `v0.88-s1` | temporal | `WP-01` through `WP-08` | active |\n| `v0.88-s3` | closeout | `WP-14` through `WP-20` | not started |\n",
    );
    let wave_out = repo.path().join("wave.yaml");
    real_tooling(&[
        "generate-wp-issue-wave".to_string(),
        "--version".to_string(),
        "v0.88".to_string(),
        "--wbs".to_string(),
        wbs.to_string_lossy().to_string(),
        "--sprint".to_string(),
        sprint.to_string_lossy().to_string(),
        "--out".to_string(),
        wave_out.to_string_lossy().to_string(),
    ])
    .expect("wave generation dispatch should succeed");
    let wave_text = fs::read_to_string(&wave_out).expect("wave output");
    assert!(wave_text.contains("schema: adl.wp_issue_wave.v1"));
    assert!(wave_text.contains("title: '[v0.88][WP-02] Chronosense foundation'"));
    assert!(wave_text.contains("area:quality"));

    real_tooling(&[
        "lint-prompt-spec".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
    ])
    .expect("lint dispatch should succeed");
    real_tooling(&[
        "validate-structured-prompt".to_string(),
        "--type".to_string(),
        "stp".to_string(),
        "--input".to_string(),
        stp.to_string_lossy().to_string(),
    ])
    .expect("stp dispatch should succeed");
    real_tooling(&[
        "validate-structured-prompt".to_string(),
        "--type".to_string(),
        "sip".to_string(),
        "--input".to_string(),
        sip.to_string_lossy().to_string(),
    ])
    .expect("sip dispatch should succeed");
    real_tooling(&[
        "review-card-surface".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--output".to_string(),
        output.to_string_lossy().to_string(),
    ])
    .expect("review surface dispatch should succeed");
    real_tooling(&[
        "verify-review-output-provenance".to_string(),
        "--review".to_string(),
        review_output.to_string_lossy().to_string(),
    ])
    .expect("review output provenance dispatch should succeed");
    real_tooling(&[
        "verify-repo-review-contract".to_string(),
        "--review".to_string(),
        review.to_string_lossy().to_string(),
    ])
    .expect("repo review contract dispatch should succeed");
}

#[test]
fn code_review_fixture_backend_writes_blocking_gate_artifacts() {
    let repo = TempRepo::new("code-review");
    let out = repo.path().join("blocked");
    real_tooling(&[
        "code-review".to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
        "--backend".to_string(),
        "fixture".to_string(),
        "--fixture-case".to_string(),
        "blocked".to_string(),
        "--visibility".to_string(),
        "read-only-repo".to_string(),
        "--writer-session".to_string(),
        "writer-session".to_string(),
        "--reviewer-session".to_string(),
        "reviewer-session".to_string(),
    ])
    .expect("blocked fixture review should write artifacts");

    let gate: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(out.join("gate_result.json")).expect("gate"))
            .expect("gate json");
    assert_eq!(gate["schema_version"], "adl.pr_review_gate.v1");
    assert_eq!(gate["pr_open_allowed"], false);

    let result: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(out.join("review_result.json")).expect("result"))
            .expect("result json");
    assert_eq!(result["visibility_mode"], "read_only_repo");
    assert_eq!(result["repo_access"]["write_allowed"], false);
    assert_eq!(result["repo_access"]["tool_execution_allowed"], false);
}

#[test]
fn code_review_ollama_without_live_gate_records_skipped_blocker() {
    let repo = TempRepo::new("code-review-ollama-skip");
    let out = repo.path().join("ollama-skip");
    real_tooling(&[
        "code-review".to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
        "--backend".to_string(),
        "ollama".to_string(),
        "--model".to_string(),
        "gemma4:latest".to_string(),
        "--writer-session".to_string(),
        "writer-session".to_string(),
        "--reviewer-session".to_string(),
        "ollama-reviewer".to_string(),
    ])
    .expect("ollama skip should still write artifacts");

    let gate: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(out.join("gate_result.json")).expect("gate"))
            .expect("gate json");
    assert_eq!(gate["pr_open_allowed"], false);
    assert!(gate["reasons"][0]
        .as_str()
        .expect("reason")
        .contains("skipped"));
}

#[test]
fn code_review_filter_covers_tooling_dispatch_help_and_errors() {
    assert!(real_tooling(&["help".to_string()]).is_ok());
    assert!(real_tooling(&["--help".to_string()]).is_ok());
    assert!(real_tooling(&["code-review".to_string(), "--help".to_string()]).is_ok());

    let missing = real_tooling(&["code-review".to_string()]).expect_err("missing out");
    assert!(missing.to_string().contains("missing --out"));

    let unknown = real_tooling(&["unknown-code-review-subcommand".to_string()])
        .expect_err("unknown subcommand");
    assert!(unknown.to_string().contains("unknown tooling subcommand"));
}

#[test]
fn tooling_dispatch_accepts_help_and_rejects_unknown_subcommands() {
    assert!(real_tooling(&["help".to_string()]).is_ok());
    assert!(real_tooling(&["--help".to_string()]).is_ok());
    assert!(real_tooling(&[]).is_err());
    assert!(real_tooling(&["unknown-subcommand".to_string()]).is_err());
}

#[test]
fn tooling_dispatch_routes_public_subcommands() {
    let repo = TempRepo::new("dispatch");
    let input = repo.write_rel(
        ".tmp/tooling_cmd_tests/input.md",
        &valid_input_card_text(1374, ".tmp/tooling_cmd_tests/output.md"),
    );
    let output = repo.write_rel(".tmp/tooling_cmd_tests/output.md", &valid_sor_text());
    let review = repo.write_rel(".tmp/tooling_cmd_tests/review.md", &valid_review_markdown());
    let review_output = repo.write_rel(
        ".tmp/tooling_cmd_tests/review-output.yaml",
        &valid_review_output_yaml(repo.path()),
    );
    let prompt_out = repo.path().join("prompt.txt");

    assert!(real_tooling(&[
        "card-prompt".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--out".to_string(),
        prompt_out.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(prompt_out.is_file());

    assert!(real_tooling(&[
        "lint-prompt-spec".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_tooling(&[
        "validate-structured-prompt".to_string(),
        "--type".to_string(),
        "sor".to_string(),
        "--input".to_string(),
        output.to_string_lossy().to_string(),
        "--phase".to_string(),
        "completed".to_string(),
    ])
    .is_ok());
    assert!(real_tooling(&[
        "review-card-surface".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--output".to_string(),
        output.to_string_lossy().to_string(),
    ])
    .is_ok());
    let review_root = write_runtime_review_fixture(&repo);
    assert!(real_tooling(&[
        "review-runtime-surface".to_string(),
        "--review-root".to_string(),
        review_root.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_tooling(&[
        "verify-review-output-provenance".to_string(),
        "--review".to_string(),
        review_output.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_tooling(&[
        "verify-repo-review-contract".to_string(),
        "--review".to_string(),
        review.to_string_lossy().to_string(),
    ])
    .is_ok());
}

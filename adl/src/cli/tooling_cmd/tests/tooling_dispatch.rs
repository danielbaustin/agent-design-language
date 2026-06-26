use super::support::*;
use super::*;

#[test]
fn tooling_cmd_dispatch_and_help_paths_cover_public_entrypoint() {
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
    let srp = repo.write_rel(".tmp/tooling_cmd_tests/srp.md", &valid_srp_text(4559));
    let sor = repo.write_rel(".tmp/tooling_cmd_tests/sor.md", &valid_sor_text());
    let srp_sor_facts = repo.write_rel(
        ".tmp/tooling_cmd_tests/srp-sor-facts.yaml",
        r#"review:
  findings_status: resolved
  recommended_outcome: PASS
validation:
  status: PASS
  commands:
    - command: "git diff --check"
      purpose: "whitespace proof"
      result: PASS
integration:
  integration_state: pr_open
  verification_scope: worktree
  result: PASS
"#,
    );
    let stp = repo.write_rel(".tmp/tooling_cmd_tests/stp.md", &valid_stp_text());
    let sip = repo.write_rel(
        ".tmp/tooling_cmd_tests/sip.md",
        &valid_sip_text(1374, repo.path()),
    );
    let prompt_out = repo.path().join("prompt.txt");
    let editor_model_out = repo.path().join("editor_model.js");
    let editor_samples_out = repo.path().join("editor_samples");
    let usage_status = repo.write_rel(
        ".tmp/tooling_cmd_tests/usage-status.txt",
        "Context: 37% left (161,634 used / 258K)\n5h limit: 4% left (resets 4:04 PM)\n7d limit: 3% left (resets Jun 24)\n",
    );

    assert!(real_tooling(&[]).is_err());
    real_tooling(&["help".to_string()]).expect("help should succeed");
    let unknown_err = real_tooling(&["unknown".to_string()]).expect_err("unknown command");
    assert!(unknown_err.to_string().contains("codex-usage-watch"));
    real_tooling(&["code-review".to_string(), "--help".to_string()])
        .expect("code-review help should succeed without --out");
    real_tooling(&["codex-usage-watch".to_string(), "--help".to_string()])
        .expect("codex-usage-watch help should succeed");
    real_tooling(&["portable-project-doctor".to_string(), "--help".to_string()])
        .expect("portable-project-doctor help should succeed");
    real_tooling(&["srp-sor-update".to_string(), "--help".to_string()])
        .expect("srp-sor-update help should succeed");

    let ci_logs = repo.path().join("ci-logs");
    fs::create_dir_all(&ci_logs).expect("ci log dir");
    fs::write(
        ci_logs.join("step.txt"),
        "2026-06-19T18:00:00.0000000Z start\n2026-06-19T18:01:10.0000000Z end\n",
    )
    .expect("ci log");
    let ci_manifest = repo.path().join("ci-log-manifest.json");
    real_tooling(&[
        "ci-log-archive".to_string(),
        "summarize".to_string(),
        "--logs-dir".to_string(),
        ci_logs.to_string_lossy().to_string(),
        "--out".to_string(),
        ci_manifest.to_string_lossy().to_string(),
        "--s3-prefix".to_string(),
        "s3://adl-ci-logs/v0.91.6".to_string(),
        "--repo".to_string(),
        "danielbaustin/agent-design-language".to_string(),
        "--pr".to_string(),
        "4152".to_string(),
        "--run-id".to_string(),
        "27840922589".to_string(),
    ])
    .expect("ci-log-archive dispatch should succeed");
    let ci_manifest_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&ci_manifest).expect("ci log archive manifest"))
            .expect("ci manifest json");
    assert_eq!(
        ci_manifest_json["schema_version"],
        "adl.ci_log_archive_manifest.v1"
    );
    assert_eq!(ci_manifest_json["timing_summary"]["b_large_count"], 1);

    real_tooling(&[
        "card-prompt".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--out".to_string(),
        prompt_out.to_string_lossy().to_string(),
    ])
    .expect("card-prompt dispatch should succeed");
    assert!(prompt_out.is_file());

    real_tooling(&[
        "codex-usage-watch".to_string(),
        "parse".to_string(),
        "--input".to_string(),
        usage_status.to_string_lossy().to_string(),
        "--json".to_string(),
    ])
    .expect("codex usage watcher dispatch should succeed");
    real_tooling(&["issue-resource-telemetry".to_string(), "--help".to_string()])
        .expect("issue resource telemetry help should succeed");

    real_tooling(&[
        "csdlc-prompt-editor".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--emit-model-js".to_string(),
        editor_model_out.to_string_lossy().to_string(),
        "--render-samples".to_string(),
        editor_samples_out.to_string_lossy().to_string(),
    ])
    .expect("csdlc prompt editor dispatch should succeed");
    assert!(editor_model_out.is_file());
    assert!(editor_samples_out.join("sip.md").is_file());

    let code_review_out = repo.path().join("code-review-clean");
    real_tooling(&[
        "code-review".to_string(),
        "--out".to_string(),
        code_review_out.to_string_lossy().to_string(),
        "--base".to_string(),
        "HEAD".to_string(),
        "--head".to_string(),
        "HEAD".to_string(),
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
        "srp-sor-update".to_string(),
        "--facts".to_string(),
        srp_sor_facts.to_string_lossy().to_string(),
        "--srp".to_string(),
        srp.to_string_lossy().to_string(),
        "--sor".to_string(),
        sor.to_string_lossy().to_string(),
    ])
    .expect("srp-sor-update dispatch should succeed");
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
        "--base".to_string(),
        "HEAD".to_string(),
        "--head".to_string(),
        "HEAD".to_string(),
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
    assert_eq!(result["repo_access"]["read_only"], false);
    assert_eq!(result["repo_access"]["write_allowed"], false);
    assert_eq!(result["repo_access"]["tool_execution_allowed"], false);
}

#[test]
fn srp_sor_update_populates_review_validation_integration_and_metrics() {
    let repo = TempRepo::new("srp-sor-update");
    let facts = repo.write_rel(
        "facts.yaml",
        r#"review:
  findings_status: resolved
  recommended_outcome: PASS
  notes: "bounded review completed; no open findings"
validation:
  status: PASS
  commands:
    - command: "cargo test --manifest-path adl/Cargo.toml srp_sor_update"
      purpose: "exercise SRP/SOR fact updater"
      result: PASS
integration:
  main_repo_paths_updated:
    - "adl/src/cli/tooling_cmd.rs"
    - "adl/src/cli/tooling_cmd/srp_sor_update.rs"
  worktree_only_paths_remaining: none
  integration_state: pr_open
  verification_scope: worktree
  integration_method: draft PR publication
  verification_performed:
    - "git diff --check"
    - "cargo test --manifest-path adl/Cargo.toml srp_sor_update"
  result: PASS
metrics:
  actual_elapsed_seconds: "900"
  actual_total_tokens: "45000"
  actual_validation_seconds: "12"
  goal_metrics_data_source: "codex_goal_tool"
  goal_metrics_source_ref: ".adl/v0.91.6/tasks/issue-4559__srp-sor-update/sor.md"
  data_source_confidence: high
"#,
    );
    let srp = repo.write_rel("srp.md", &valid_srp_text(4559));
    let sor = repo.write_rel("sor.md", &valid_sor_text());

    real_srp_sor_update(&[
        "--facts".into(),
        facts.display().to_string(),
        "--srp".into(),
        srp.display().to_string(),
        "--sor".into(),
        sor.display().to_string(),
    ])
    .expect("update SRP/SOR");

    let srp_text = fs::read_to_string(&srp).expect("read srp");
    let sor_text = fs::read_to_string(&sor).expect("read sor");

    assert!(srp_text.contains("review_results:"));
    assert!(srp_text.contains("findings_status: no_findings"));
    assert!(srp_text.contains("recommended_outcome: pass"));
    assert!(!srp_text.contains("review_results_exception"));
    assert!(sor_text.contains("- Actual elapsed seconds: `900`"));
    assert!(sor_text.contains("- Integration state: pr_open"));
    assert!(sor_text.contains("  - `adl/src/cli/tooling_cmd/srp_sor_update.rs`"));
    assert!(sor_text.contains("- `cargo test --manifest-path adl/Cargo.toml srp_sor_update` - exercise SRP/SOR fact updater"));
    assert!(sor_text.contains("    status: PASS"));

    validate_srp_text(&srp_text).expect("srp validates");
    validate_sor_text(&sor_text, None).expect("sor validates");
}

#[test]
fn srp_sor_update_is_idempotent_when_facts_already_applied() {
    let repo = TempRepo::new("srp-sor-update-idempotent");
    let facts = repo.write_rel(
        "facts.yaml",
        r#"review:
  findings_status: resolved
  recommended_outcome: PASS
validation:
  status: PASS
"#,
    );
    let srp = repo.write_rel("srp.md", &valid_srp_text(4559));
    let sor = repo.write_rel("sor.md", &valid_sor_text());

    let args = vec![
        "--facts".to_string(),
        facts.display().to_string(),
        "--srp".to_string(),
        srp.display().to_string(),
        "--sor".to_string(),
        sor.display().to_string(),
    ];

    real_srp_sor_update(&args).expect("first update");
    let srp_once = fs::read_to_string(&srp).expect("read srp once");
    let sor_once = fs::read_to_string(&sor).expect("read sor once");
    real_srp_sor_update(&args).expect("second update");

    assert_eq!(srp_once, fs::read_to_string(&srp).expect("read srp twice"));
    assert_eq!(sor_once, fs::read_to_string(&sor).expect("read sor twice"));
}

#[test]
fn srp_sor_update_fails_closed_for_empty_fact_packet() {
    let repo = TempRepo::new("srp-sor-update-empty");
    let facts = repo.write_rel("facts.yaml", "{}\n");
    let srp = repo.write_rel("srp.md", &valid_srp_text(4559));
    let sor = repo.write_rel("sor.md", &valid_sor_text());

    let err = real_srp_sor_update(&[
        "--facts".into(),
        facts.display().to_string(),
        "--srp".into(),
        srp.display().to_string(),
        "--sor".into(),
        sor.display().to_string(),
    ])
    .expect_err("empty facts fail");

    assert!(err
        .to_string()
        .contains("facts file contains no SRP/SOR updates"));
}

#[test]
fn code_review_ollama_without_live_gate_records_skipped_blocker() {
    let repo = TempRepo::new("code-review-ollama-skip");
    let out = repo.path().join("ollama-skip");
    real_tooling(&[
        "code-review".to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
        "--base".to_string(),
        "HEAD".to_string(),
        "--head".to_string(),
        "HEAD".to_string(),
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
    assert!(gate["reasons"]
        .as_array()
        .expect("reasons")
        .iter()
        .any(|reason| reason.as_str().expect("reason").contains("skipped")));
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
fn csdlc_prompt_editor_cli_dispatch_usage_and_errors_are_covered() {
    let repo = TempRepo::new("csdlc-prompt-editor-dispatch");
    let model_out = repo.path().join("editor_model.js");
    let samples_out = repo.path().join("samples");

    assert!(super::super::tooling_usage().contains("adl tooling csdlc-prompt-editor"));
    assert!(crate::cli::usage::usage().contains("adl tooling csdlc-prompt-editor"));
    assert!(real_tooling(&["csdlc-prompt-editor".to_string(), "--help".to_string()]).is_ok());

    let missing_action =
        real_tooling(&["csdlc-prompt-editor".to_string()]).expect_err("missing action should fail");
    assert!(missing_action
        .to_string()
        .contains("requires --emit-model-js and/or --render-samples"));

    let unknown = real_tooling(&["csdlc-prompt-editor".to_string(), "--unknown".to_string()])
        .expect_err("unknown arg should fail");
    assert!(unknown
        .to_string()
        .contains("unknown arg for tooling csdlc-prompt-editor"));

    let bad_root = real_tooling(&[
        "csdlc-prompt-editor".to_string(),
        "--repo-root".to_string(),
        repo.path().to_string_lossy().to_string(),
        "--emit-model-js".to_string(),
        model_out.to_string_lossy().to_string(),
    ])
    .expect_err("non-repo root should fail");
    assert!(bad_root
        .to_string()
        .contains("repo root must contain adl/Cargo.toml"));

    real_tooling(&[
        "csdlc-prompt-editor".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--emit-model-js".to_string(),
        model_out.to_string_lossy().to_string(),
        "--render-samples".to_string(),
        samples_out.to_string_lossy().to_string(),
    ])
    .expect("csdlc prompt editor dispatch should render model and samples");

    let model = fs::read_to_string(&model_out).expect("editor model");
    assert!(model.contains("window.CSDLC_PROMPT_EDITOR_MODEL"));
    assert!(samples_out.join("sip.md").is_file());
    assert!(samples_out.join("sor.md").is_file());
}

#[test]
fn prompt_template_tooling_dispatch_and_usage_are_covered() {
    let repo = TempRepo::new("prompt-template-dispatch");
    let values_dir = repo.path().join("values");
    let rendered_dir = repo.path().join("rendered");

    assert!(super::super::tooling_usage().contains("adl tooling prompt-template render"));
    assert!(crate::cli::usage::usage().contains("adl tooling <card-prompt|code-review|csdlc-prompt-editor|generate-wp-issue-wave|lint-prompt-spec|prompt-template|srp-sor-update|validate-structured-prompt"));

    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--out-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
    ])
    .expect("prompt-template dispatch should write sample values");
    assert!(values_dir.join("sip.values.yaml").is_file());

    real_tooling(&[
        "prompt-template".to_string(),
        "render-all".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--values-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
        "--out-dir".to_string(),
        rendered_dir.to_string_lossy().to_string(),
    ])
    .expect("prompt-template dispatch should render all cards");
    assert!(rendered_dir.join("sip.md").is_file());
    assert!(rendered_dir.join("sor.md").is_file());
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
    let editor_model_out = repo.path().join("editor_model.js");

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
        "csdlc-prompt-editor".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--emit-model-js".to_string(),
        editor_model_out.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(editor_model_out.is_file());

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

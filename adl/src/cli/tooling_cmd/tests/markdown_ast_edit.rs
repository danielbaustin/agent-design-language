use super::support::*;
use super::*;

#[test]
fn markdown_ast_edit_replaces_section_and_preserves_structural_surfaces() {
    let repo = TempRepo::new("markdown-ast-edit");
    let input = repo.write_rel(
        ".tmp/tooling_cmd_tests/doc.md",
        "---\ntitle: Demo\n---\n# Demo\n\n## Summary\n\nOld summary.\n\n## Evidence\n\n| Key | Value |\n|---|---|\n| link | [source](https://example.com) |\n\n```rust\nfn main() {}\n```\n",
    );
    let replacement = repo.write_rel(
        ".tmp/tooling_cmd_tests/replacement.md",
        "New summary with [review](https://example.com/review).\n",
    );
    let out = repo.path().join(".tmp/tooling_cmd_tests/out.md");

    real_tooling(&[
        "markdown-ast-edit".to_string(),
        "replace-section".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--heading".to_string(),
        "Summary".to_string(),
        "--replacement".to_string(),
        replacement.to_string_lossy().to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
    ])
    .expect("AST-guarded section replacement should succeed");

    let edited = fs::read_to_string(out).expect("edited markdown");
    assert!(edited.contains("---\ntitle: Demo\n---"));
    assert!(edited.contains("## Summary\n\nNew summary"));
    assert!(edited.contains("| Key | Value |"));
    assert!(edited.contains("[source](https://example.com)"));
    assert!(edited.contains("```rust\nfn main() {}\n```"));
}

#[test]
fn markdown_ast_edit_fails_closed_for_lifecycle_cards_and_writes_repair_note() {
    let repo = TempRepo::new("markdown-ast-card-guard");
    let input = repo.write_rel(
        ".adl/v0.91.5/tasks/issue-3715__demo/sor.md",
        "# SOR\n\n## Summary\n\nOld.\n",
    );
    let replacement = repo.write_rel(".tmp/tooling_cmd_tests/replacement.md", "New.\n");
    let out = repo.path().join(".tmp/tooling_cmd_tests/out.md");
    let note = repo.path().join(".tmp/tooling_cmd_tests/repair.md");

    let err = real_tooling(&[
        "markdown-ast-edit".to_string(),
        "replace-section".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--heading".to_string(),
        "Summary".to_string(),
        "--replacement".to_string(),
        replacement.to_string_lossy().to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
        "--repair-note-out".to_string(),
        note.to_string_lossy().to_string(),
    ])
    .expect_err("lifecycle card edit should fail closed");

    assert!(err.to_string().contains("lifecycle-card input or output path"));
    assert!(!out.exists());
    let note_text = fs::read_to_string(note).expect("repair note");
    assert!(note_text.contains("Status: not_mutated"));
    assert!(note_text.contains("prompt-template/card-editor authority"));
}

#[test]
fn markdown_ast_edit_fails_closed_when_output_targets_lifecycle_card() {
    let repo = TempRepo::new("markdown-ast-card-output-guard");
    let input = repo.write_rel(
        ".tmp/tooling_cmd_tests/doc.md",
        "# Demo\n\n## Summary\n\nOld.\n",
    );
    let replacement = repo.write_rel(".tmp/tooling_cmd_tests/replacement.md", "New.\n");
    let out = repo.path().join(".adl/v0.91.5/tasks/issue-3715__demo/sor.md");
    let note = repo.path().join(".tmp/tooling_cmd_tests/repair.md");

    let err = real_tooling(&[
        "markdown-ast-edit".to_string(),
        "replace-section".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--heading".to_string(),
        "Summary".to_string(),
        "--replacement".to_string(),
        replacement.to_string_lossy().to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
        "--repair-note-out".to_string(),
        note.to_string_lossy().to_string(),
    ])
    .expect_err("lifecycle card output edit should fail closed");

    assert!(err.to_string().contains("lifecycle-card"));
    assert!(!out.exists());
    let note_text = fs::read_to_string(note).expect("repair note");
    assert!(note_text.contains("input or output path"));
}

#[test]
fn markdown_ast_edit_rejects_replacements_that_remove_existing_protected_nodes() {
    let repo = TempRepo::new("markdown-ast-protected-node-guard");
    let input = repo.write_rel(
        ".tmp/tooling_cmd_tests/doc.md",
        "# Demo\n\n## Summary\n\nOld.\n\n## Evidence\n\n[source](https://example.com)\n\n```rust\nfn main() {}\n```\n",
    );
    let replacement = repo.write_rel(".tmp/tooling_cmd_tests/replacement.md", "No protected nodes.\n");
    let out = repo.path().join(".tmp/tooling_cmd_tests/out.md");

    let err = real_tooling(&[
        "markdown-ast-edit".to_string(),
        "replace-section".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--heading".to_string(),
        "Evidence".to_string(),
        "--replacement".to_string(),
        replacement.to_string_lossy().to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
    ])
    .expect_err("replacement that removes protected nodes should fail");

    assert!(err.to_string().contains("code fence preservation guard"));
    assert!(!out.exists());
}

#[test]
fn markdown_ast_edit_rejects_unsupported_html_with_repair_note() {
    let repo = TempRepo::new("markdown-ast-html-guard");
    let input = repo.write_rel(
        ".tmp/tooling_cmd_tests/doc.md",
        "# Demo\n\n## Summary\n\nOld.\n\n<div>unsupported</div>\n",
    );
    let replacement = repo.write_rel(".tmp/tooling_cmd_tests/replacement.md", "New.\n");
    let out = repo.path().join(".tmp/tooling_cmd_tests/out.md");
    let note = repo.path().join(".tmp/tooling_cmd_tests/repair.md");

    let err = real_tooling(&[
        "markdown-ast-edit".to_string(),
        "replace-section".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--heading".to_string(),
        "Summary".to_string(),
        "--replacement".to_string(),
        replacement.to_string_lossy().to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
        "--repair-note-out".to_string(),
        note.to_string_lossy().to_string(),
    ])
    .expect_err("raw HTML should be unsupported");

    assert!(err.to_string().contains("unsupported raw HTML"));
    assert!(!out.exists());
    let note_text = fs::read_to_string(note).expect("repair note");
    assert!(note_text.contains("unsupported raw HTML"));
}

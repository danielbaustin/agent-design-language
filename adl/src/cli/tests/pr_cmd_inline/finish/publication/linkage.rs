use super::*;
use crate::cli::pr_cmd::github::{
    current_pr_url, ensure_or_repair_pr_closing_linkage, ensure_pr_closing_linkage,
    pr_has_closing_linkage,
};

#[test]
fn finish_helper_paths_cover_pr_lookup_and_closing_linkage() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-gh-helpers");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1159\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let pr = current_pr_url(
        "danielbaustin/agent-design-language",
        "codex/1153-rust-finish-test",
    )
    .expect("pr url");
    assert_eq!(
        pr.as_deref(),
        Some("https://github.com/danielbaustin/agent-design-language/pull/1159")
    );
    assert!(pr_has_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153
    )
    .expect("closing linkage"));
    ensure_pr_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153,
        false,
    )
    .expect("ensure linkage");
    ensure_pr_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153,
        true,
    )
    .expect("no close skips");

    unsafe {
        env::set_var("PATH", old_path);
    }
    let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_calls.contains("pr list -R danielbaustin/agent-design-language --head codex/1153-rust-finish-test --state open --json url --jq .[0].url"));
}

#[test]
fn ensure_or_repair_pr_closing_linkage_repairs_live_pr_body() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-repair-linkage");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let state_body = temp.join("pr_body.txt");
    fs::write(&state_body, "Refs #1153\n").expect("seed body");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    if grep -q 'Closes #1153' '{}'; then\n      printf '1153\\n'\n    fi\n    exit 0\n  fi\n  if printf '%s ' \"$@\" | grep -q ' --json body '; then\n    cat '{}'\n    exit 0\n  fi\nfi\nif [ \"$1 $2\" = 'pr edit' ]; then\n  body_file=''\n  while [ $# -gt 0 ]; do\n    if [ \"$1\" = '--body-file' ]; then\n      body_file=\"$2\"\n      shift 2\n    else\n      shift\n    fi\n  done\n  cp \"$body_file\" '{}'\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            state_body.display(),
            state_body.display(),
            state_body.display()
        ),
    );

    let body_file = temp.join("desired.md");
    fs::write(&body_file, "Closes #1153\n\n## Summary\nrepaired\n").expect("desired body");

    let old_path = env::var("PATH").unwrap_or_default();
    let old_entries = env::split_paths(&old_path).collect::<Vec<_>>();
    let mut new_entries = vec![bin_dir.clone()];
    new_entries.extend(old_entries);
    unsafe {
        env::set_var("PATH", env::join_paths(new_entries).expect("join PATH"));
    }

    ensure_or_repair_pr_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153,
        false,
        &body_file,
    )
    .expect("repair should succeed");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let repaired = fs::read_to_string(&state_body).expect("read repaired body");
    assert!(repaired.contains("Closes #1153"));
    let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
    assert!(gh_calls.contains("pr edit -R danielbaustin/agent-design-language https://github.com/danielbaustin/agent-design-language/pull/1159 --body-file"));
}

#[test]
fn ensure_pr_closing_linkage_errors_when_pr_body_has_no_issue_reference() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-missing-linkage");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    exit 0\n  fi\n  if printf '%s ' \"$@\" | grep -q ' --json body '; then\n    printf 'Refs #9999\\n'\n    exit 0\n  fi\nfi\nexit 1\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let err = ensure_pr_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153,
        false,
    )
    .expect_err("missing linkage should fail");

    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err
        .to_string()
        .contains("missing closing linkage to issue #1153"));
}

#[test]
fn ensure_or_repair_pr_closing_linkage_is_noop_when_no_close_is_set() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-no-close");
    let body_file = temp.join("desired.md");
    fs::write(&body_file, "Refs #1153\n").expect("body");

    let repaired = ensure_or_repair_pr_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153,
        true,
        &body_file,
    )
    .expect("no-close should skip linkage repair");

    assert!(!repaired);
}

#[test]
fn ensure_or_repair_pr_closing_linkage_is_noop_when_linkage_already_exists() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-linkage-already-present");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n    exit 0\n  fi\n  if printf '%s ' \"$@\" | grep -q ' --json body '; then\n    printf 'Closes #1153\\n'\n    exit 0\n  fi\nfi\nif [ \"$1 $2\" = 'pr edit' ]; then\n  exit 99\nfi\nexit 1\n",
            gh_log.display()
        ),
    );
    let body_file = temp.join("desired.md");
    fs::write(&body_file, "Closes #1153\n").expect("body");

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let repaired = ensure_or_repair_pr_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153,
        false,
        &body_file,
    )
    .expect("existing linkage should not require repair");

    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(!repaired);
    let gh_log = fs::read_to_string(&gh_log).expect("gh log");
    assert!(!gh_log.contains("pr edit -R"));
}

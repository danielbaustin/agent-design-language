use super::*;
use crate::cli::tests::env_lock as cli_env_lock;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn unique_temp_dir(label: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("{label}-{now}-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

pub(crate) fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    let guard = cli_env_lock();
    unsafe {
        env::set_var("ADL_PR_JANITOR_DISABLE", "1");
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "1");
    }
    guard
}

pub(crate) struct GithubCliFixtureGuard {
    old_fixture: Option<std::ffi::OsString>,
}

impl Drop for GithubCliFixtureGuard {
    fn drop(&mut self) {
        unsafe {
            match &self.old_fixture {
                Some(value) => env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", value),
                None => env::remove_var("ADL_TEST_GITHUB_CLI_FIXTURE"),
            }
        }
    }
}

impl GithubCliFixtureGuard {
    pub(crate) fn set(path: &Path) -> Self {
        let old_fixture = env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        unsafe {
            env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", path);
        }
        Self { old_fixture }
    }
}

pub(crate) struct GithubTransportEnvGuard {
    old_values: Vec<(&'static str, Option<std::ffi::OsString>)>,
}

impl Drop for GithubTransportEnvGuard {
    fn drop(&mut self) {
        unsafe {
            for (key, value) in &self.old_values {
                match value {
                    Some(value) => env::set_var(key, value),
                    None => env::remove_var(key),
                }
            }
        }
    }
}

pub(crate) fn force_gh_cli_transport_env() -> GithubTransportEnvGuard {
    let keys = [
        "ADL_GITHUB_CLIENT",
        "ADL_GITHUB_DISABLE_GH_FALLBACK",
        "ADL_GITHUB_OCTOCRAB_BASE_URI",
        "ADL_TEST_GITHUB_CLI_FIXTURE",
        "GITHUB_TOKEN",
        "GH_TOKEN",
        "ADL_GITHUB_TOKEN_FILE",
        "ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE",
        "ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT",
    ];
    let old_values = keys
        .into_iter()
        .map(|key| (key, env::var_os(key)))
        .collect::<Vec<_>>();
    unsafe {
        for key in keys {
            env::remove_var(key);
        }
    }
    GithubTransportEnvGuard { old_values }
}

pub(crate) fn install_issue_label_fixture(repo: &Path) -> GithubCliFixtureGuard {
    let fixture_dir = repo.join(".adl/test-fixtures");
    fs::create_dir_all(&fixture_dir).expect("fixture dir");
    let fixture = fixture_dir.join("github-cli-fixture");
    write_executable(
        &fixture,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.86][tools] Init test\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
    );
    GithubCliFixtureGuard::set(&fixture)
}

pub(crate) fn has_prompt_template_placeholder(text: &str) -> bool {
    let mut chars = text.char_indices().peekable();
    while let Some((start, ch)) = chars.next() {
        if ch != '<' {
            continue;
        }
        let mut end = None;
        while let Some(&(idx, next)) = chars.peek() {
            chars.next();
            if next == '>' {
                end = Some(idx);
                break;
            }
        }
        let Some(end_idx) = end else {
            break;
        };
        let candidate = &text[start + 1..end_idx];
        if !candidate.is_empty()
            && candidate
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            return true;
        }
    }
    false
}

pub(crate) fn assert_no_prompt_template_residue(kind: &str, text: &str) {
    assert!(
        !has_prompt_template_placeholder(text),
        "{kind} should not retain prompt-template placeholders"
    );
    assert!(
        !text.contains("[summary truncated]"),
        "{kind} should not retain truncated summary sentinels"
    );
}

#[test]
pub(crate) fn env_lock_disables_post_merge_closeout_by_default() {
    let _guard = env_lock();
    assert_eq!(
        env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok().as_deref(),
        Some("1")
    );
}

pub(crate) fn write_executable(path: &Path, content: &str) {
    let content = if path.file_name().and_then(|name| name.to_str()) == Some("gh")
        && !content.contains("ADL_GITHUB_TEST_FIXTURE")
    {
        content.replacen(
            "#!/usr/bin/env bash\n",
            "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\n",
            1,
        )
    } else {
        content.to_string()
    };
    fs::write(path, content).expect("write executable");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path).expect("metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).expect("chmod");
    }
}

pub(crate) fn init_git_repo(dir: &Path) {
    assert!(Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(dir)
        .status()
        .expect("git init")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "add",
            "origin",
            "https://github.com/owner/repo.git"
        ])
        .current_dir(dir)
        .status()
        .expect("git remote add")
        .success());
}

pub(crate) fn copy_bootstrap_support_files(repo: &Path) {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf();
    env::set_var("ADL_TOOLING_MANIFEST_ROOT", &workspace_root);
    let tools_dir = repo.join("adl/tools");
    let config_dir = repo.join("adl/config");
    let templates_dir = repo.join("adl/templates/cards");
    let schemas_dir = repo.join("adl/schemas");
    fs::create_dir_all(&tools_dir).expect("tools dir");
    fs::create_dir_all(&config_dir).expect("config dir");
    fs::create_dir_all(&templates_dir).expect("templates dir");
    fs::create_dir_all(&schemas_dir).expect("schemas dir");

    let files = [
        (
            workspace_root.join("adl/tools/card_paths.sh"),
            tools_dir.join("card_paths.sh"),
        ),
        (
            workspace_root.join("adl/tools/validate_structured_prompt.sh"),
            tools_dir.join("validate_structured_prompt.sh"),
        ),
        (
            workspace_root.join("adl/tools/lint_prompt_spec.sh"),
            tools_dir.join("lint_prompt_spec.sh"),
        ),
        (
            workspace_root.join("adl/tools/validation_manager.py"),
            tools_dir.join("validation_manager.py"),
        ),
        (
            workspace_root.join("adl/tools/validation_manager.sh"),
            tools_dir.join("validation_manager.sh"),
        ),
        (
            workspace_root.join("adl/tools/test_validation_manager.sh"),
            tools_dir.join("test_validation_manager.sh"),
        ),
        (
            workspace_root.join("adl/tools/select_validation_lanes.py"),
            tools_dir.join("select_validation_lanes.py"),
        ),
        (
            workspace_root.join("adl/tools/select_validation_lanes.sh"),
            tools_dir.join("select_validation_lanes.sh"),
        ),
        (
            workspace_root.join("adl/tools/run_pr_fast_test_lane.sh"),
            tools_dir.join("run_pr_fast_test_lane.sh"),
        ),
        (
            workspace_root.join("adl/tools/run_slow_proof_family.sh"),
            tools_dir.join("run_slow_proof_family.sh"),
        ),
        (
            workspace_root.join("adl/tools/run_owner_validation_lane.sh"),
            tools_dir.join("run_owner_validation_lane.sh"),
        ),
        (
            workspace_root.join("adl/tools/check_no_tracked_adl_issue_record_residue.sh"),
            tools_dir.join("check_no_tracked_adl_issue_record_residue.sh"),
        ),
        (
            workspace_root.join("adl/tools/attach_post_merge_closeout.sh"),
            tools_dir.join("attach_post_merge_closeout.sh"),
        ),
        (
            workspace_root.join("adl/templates/cards/input_card_template.md"),
            templates_dir.join("input_card_template.md"),
        ),
        (
            workspace_root.join("adl/templates/cards/output_card_template.md"),
            templates_dir.join("output_card_template.md"),
        ),
        (
            workspace_root.join("adl/schemas/structured_task_prompt.contract.yaml"),
            schemas_dir.join("structured_task_prompt.contract.yaml"),
        ),
        (
            workspace_root.join("adl/schemas/structured_implementation_prompt.contract.yaml"),
            schemas_dir.join("structured_implementation_prompt.contract.yaml"),
        ),
        (
            workspace_root.join("adl/schemas/structured_output_record.contract.yaml"),
            schemas_dir.join("structured_output_record.contract.yaml"),
        ),
        (
            workspace_root.join("adl/config/validation_lane_selector.v0.91.6.json"),
            config_dir.join("validation_lane_selector.v0.91.6.json"),
        ),
        (
            workspace_root.join("adl/config/slow_proof_families.v0.91.6.json"),
            config_dir.join("slow_proof_families.v0.91.6.json"),
        ),
    ];

    for (src, dst) in files {
        fs::copy(src, &dst).expect("copy support file");
        #[cfg(unix)]
        if dst.extension().is_none() || dst.to_string_lossy().ends_with(".sh") {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&dst).expect("metadata").permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&dst, perms).expect("chmod");
        }
    }
    copy_versioned_prompt_templates(repo);
}

pub(crate) fn copy_versioned_prompt_templates(repo: &Path) {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf();
    let source_root = workspace_root.join("docs/templates/prompts");
    let target_root = repo.join("docs/templates/prompts");
    fs::create_dir_all(target_root.join("1.0.0")).expect("create prompt template dir");
    fs::create_dir_all(target_root.join("1.0.0/schemas")).expect("create prompt template dir");
    fs::create_dir_all(target_root.join("1.0.1")).expect("create prompt template dir");
    fs::create_dir_all(target_root.join("1.0.1/schemas")).expect("create prompt template dir");
    fs::create_dir_all(target_root.join("1.0.2")).expect("create prompt template dir");
    fs::create_dir_all(target_root.join("1.0.2/schemas")).expect("create prompt template dir");
    fs::create_dir_all(target_root.join("1.0.3")).expect("create prompt template dir");
    fs::create_dir_all(target_root.join("1.0.3/schemas")).expect("create prompt template dir");
    for rel in [
        "current.json",
        "1.0.0/sip.md",
        "1.0.0/stp.md",
        "1.0.0/spp.md",
        "1.0.0/srp.md",
        "1.0.0/sor.md",
        "1.0.0/schemas/sip.structure.json",
        "1.0.0/schemas/stp.structure.json",
        "1.0.0/schemas/spp.structure.json",
        "1.0.0/schemas/srp.structure.json",
        "1.0.0/schemas/sor.structure.json",
        "1.0.1/sip.md",
        "1.0.1/stp.md",
        "1.0.1/spp.md",
        "1.0.1/srp.md",
        "1.0.1/sor.md",
        "1.0.1/schemas/sip.structure.json",
        "1.0.1/schemas/stp.structure.json",
        "1.0.1/schemas/spp.structure.json",
        "1.0.1/schemas/srp.structure.json",
        "1.0.1/schemas/sor.structure.json",
        "1.0.2/sip.md",
        "1.0.2/stp.md",
        "1.0.2/spp.md",
        "1.0.2/srp.md",
        "1.0.2/sor.md",
        "1.0.2/schemas/sip.structure.json",
        "1.0.2/schemas/stp.structure.json",
        "1.0.2/schemas/spp.structure.json",
        "1.0.2/schemas/srp.structure.json",
        "1.0.2/schemas/sor.structure.json",
        "1.0.3/sip.md",
        "1.0.3/stp.md",
        "1.0.3/spp.md",
        "1.0.3/vpp.md",
        "1.0.3/srp.md",
        "1.0.3/sor.md",
        "1.0.3/schemas/sip.structure.json",
        "1.0.3/schemas/stp.structure.json",
        "1.0.3/schemas/spp.structure.json",
        "1.0.3/schemas/vpp.structure.json",
        "1.0.3/schemas/srp.structure.json",
        "1.0.3/schemas/sor.structure.json",
    ] {
        fs::copy(source_root.join(rel), target_root.join(rel)).expect("copy prompt template");
    }
}

pub(crate) fn write_alternate_stp_prompt_template(repo: &Path) {
    let alternate_dir = repo.join("docs/templates/prompts/1.0.2");
    fs::create_dir_all(&alternate_dir).expect("create alternate prompt template dir");
    let base = fs::read_to_string(repo.join("docs/templates/prompts/1.0.2/stp.md"))
        .expect("read base stp template");
    fs::write(
        alternate_dir.join("stp.md"),
        base + "\n\nRegistry route proof: alternate STP template.\n",
    )
    .expect("write alternate stp template");
    fs::write(
        repo.join("docs/templates/prompts/current.json"),
        r#"{
  "schema": "adl.csdlc.prompt_template_registry.v1",
  "csdlc_prompt_template_set": "1.0.2",
  "semver": "1.0.2",
  "status": "active",
  "object_kind": "csdlc_prompt_template_set",
  "lifecycle": ["SIP", "STP", "SPP", "SRP", "SOR"],
  "templates": {
    "stp": {
      "semantic_role": "Structured Task Prompt",
      "path": "docs/templates/prompts/1.0.2/stp.md"
    }
  }
}
"#,
    )
    .expect("write alternate registry");
}

pub(crate) fn write_authored_issue_prompt(repo: &Path, issue_ref: &IssueRef, title: &str) {
    let path = issue_ref.issue_prompt_path(repo);
    fs::create_dir_all(path.parent().expect("issue prompt parent")).expect("create body dir");
    let content = format!(
            "---\nissue_card_schema: adl.issue.v1\nwp: \"unassigned\"\nslug: \"{slug}\"\ntitle: \"{title}\"\nlabels:\n  - \"track:roadmap\"\n  - \"area:tools\"\n  - \"type:task\"\n  - \"version:v0.86\"\nissue_number: {issue}\nstatus: \"active\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"unplanned\"\nrequired_outcome_type:\n  - \"code\"\nrepo_inputs:\n  - \"https://github.com/example/repo/issues/{issue}\"\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"Authored for test coverage.\"\npr_start:\n  enabled: true\n  slug: \"{slug}\"\n---\n\n# {title}\n\n## Summary\n\nAuthored prompt for lifecycle validation tests.\n\n## Goal\n\nMake the issue prompt authored enough that lifecycle commands should accept it.\n\n## Required Outcome\n\nThis test issue ships code only.\n\n## Deliverables\n\n- authored issue prompt content\n\n## Acceptance Criteria\n\n- lifecycle validation accepts this source prompt\n\n## Repo Inputs\n\n- https://github.com/example/repo/issues/{issue}\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- bootstrap placeholder content\n\n## Issue-Graph Notes\n\n- test fixture\n\n## Notes\n\n- generated inside unit tests\n\n## Tooling Notes\n\n- authored fixture, not bootstrap fallback\n",
            slug = issue_ref.slug(),
            title = title,
            issue = issue_ref.issue_number()
        );
    fs::write(path, content).expect("write authored prompt");
}

pub(crate) fn write_authored_sip(
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    source_prompt: &Path,
    repo_root: &Path,
) {
    let source_rel = path_relative_to_repo(repo_root, source_prompt);
    let content = format!(
            "# ADL Input Card\n\nTask ID: {task_id}\nRun ID: {task_id}\nVersion: v0.86\nTitle: {title}\nBranch: {branch}\n\nContext:\n- Issue: https://github.com/example/repo/issues/{issue}\n- PR: https://github.com/example/repo/pull/{issue}\n- Source Issue Prompt: {source_rel}\n- Docs: none\n- Other: none\n\n## Agent Execution Rules\n- Do not run `pr start`; the branch and worktree already exist.\n- Only modify files required for the issue.\n\n## Prompt Spec\n```yaml\nprompt_schema: adl.v1\nactor:\n  role: execution_agent\n  name: codex\nmodel:\n  id: gpt-5-codex\n  determinism_mode: stable\ninputs:\n  sections:\n    - goal\n    - required_outcome\n    - acceptance_criteria\n    - inputs\n    - target_files_surfaces\n    - validation_plan\n    - demo_proof_requirements\n    - constraints_policies\n    - system_invariants\n    - reviewer_checklist\n    - non_goals_out_of_scope\n    - notes_risks\n    - instructions_to_agent\noutputs:\n  output_card: .adl/v0.86/tasks/{bundle}/sor.md\n  summary_style: concise_structured\nconstraints:\n  include_system_invariants: true\n  include_reviewer_checklist: true\n  disallow_secrets: true\n  disallow_absolute_host_paths: true\nautomation_hints:\n  source_issue_prompt_required: true\n  target_files_surfaces_recommended: true\n  validation_plan_required: true\n  required_outcome_type_supported: true\nreview_surfaces:\n  - card_review_checklist.v1\n  - card_review_output.v1\n  - card_reviewer_gpt.v1.1\n```\n\nExecution:\n- Agent: codex\n- Provider: openai\n- Tools allowed: git, cargo\n- Sandbox / approvals: workspace-write\n- Source issue-prompt slug: {slug}\n- Required outcome type: code\n- Demo required: false\n\n## Goal\n\nBlock lifecycle execution when prompts are still bootstrap stubs.\n\n## Required Outcome\n\n- This issue must ship code and tests.\n\n## Acceptance Criteria\n\n- lifecycle commands reject placeholder prompt content\n\n## Inputs\n- issue body\n- task bundle cards\n\n## Target Files / Surfaces\n- adl/src/cli/pr_cmd.rs\n- adl/tools/pr.sh\n\n## Validation Plan\n- Required commands: cargo test --manifest-path Cargo.toml pr_cmd -- --nocapture\n- Required tests: targeted lifecycle validation coverage\n- Required artifacts / traces: none\n- Required reviewer or demo checks: none\n\n## Demo / Proof Requirements\n- Required demo(s): none\n- Required proof surface(s): command failure behavior and tests\n- If no demo is required, say why: tooling guardrail only\n\n## Constraints / Policies\n- Determinism requirements: stable error messages for the same stub input\n- Security / privacy requirements: no secrets or absolute host paths\n- Resource limits (time/CPU/memory/network): standard local test limits\n\n## System Invariants (must remain true)\n- Deterministic execution for identical inputs.\n- No hidden state or undeclared side effects.\n- Artifacts remain replay-compatible with the replay runner.\n- Trace artifacts contain no secrets, prompts, tool arguments, or absolute host paths.\n- Artifact schema changes are explicit and approved.\n\n## Reviewer Checklist (machine-readable hints)\n```yaml\ndeterminism_required: true\nnetwork_allowed: false\nartifact_schema_change: false\nreplay_required: true\nsecurity_sensitive: true\nci_validation_required: true\n```\n\n## Card Automation Hooks (prompt generation)\n- Prompt source fields:\n  - Goal\n  - Required Outcome\n  - Acceptance Criteria\n- Generation requirements:\n  - Deterministic output for identical input card content\n  - Preserve traceability back to the source issue prompt\n\n## Non-goals / Out of scope\n- rewriting historical issues automatically\n\n## Notes / Risks\n- none\n\n## Instructions to the Agent\n- Read the linked source issue prompt before starting work.\n- Do the work described above.\n- Write results to the paired output card file.\n",
            task_id = issue_ref.task_issue_id(),
            title = title,
            branch = branch,
            issue = issue_ref.issue_number(),
            source_rel = source_rel,
            bundle = issue_ref.task_bundle_dir_name(),
            slug = issue_ref.slug(),
        );
    fs::write(path, content).expect("write authored sip");
}

pub(crate) fn write_authored_spp(
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    repo_root: &Path,
) {
    let stp_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_stp_path(repo_root));
    let sip_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_input_path(repo_root));
    let spp_rel = path_relative_to_repo(repo_root, path);
    let content = format!(
        r#"---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "{slug}-execution-plan"
issue: {issue}
task_id: "{task_id}"
run_id: "{task_id}"
version: v0.86
title: "{title}"
branch: "{branch}"
status: "reviewed"
activation_state: "reviewed"
plan_revision: 1
initial_pvf_lane: "prompt_template"
planned_pvf_lane: "prompt_template"
planned_pvf_lane_source: "matched_initial_issue_lane"
source_refs:
  - kind: "issue"
    ref: "https://github.com/example/repo/issues/{issue}"
  - kind: "stp"
    ref: "{stp_rel}"
  - kind: "sip"
    ref: "{sip_rel}"
scope:
  files:
    - "{stp_rel}"
    - "{sip_rel}"
  components:
    - "{slug}"
  out_of_scope:
    - "implementation beyond the approved issue scope"
constraints:
  - "read_only_until_execution_is_approved"
confidence: "medium"
plan_summary: "Authored planning surface for finish-path tests."
assumptions:
  - "The linked STP and SIP remain canonical."
proposed_steps:
  - id: "step-1"
    description: "Review the bundle and tighten the execution sequence."
    expected_output: "{spp_rel}"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Review the bundle and tighten the execution sequence."
    status: "pending"
affected_areas:
  - "{slug}"
invariants_to_preserve:
  - "Do not claim implementation work inside the plan."
risks_and_edge_cases:
  - "Validation inputs may need tightening before execution."
test_strategy:
  - "Review the proposed validation commands before execution."
execution_handoff: "Use this artifact as the durable plan-of-record before execution."
required_permissions:
  - "workspace-write after execution approval"
stop_conditions:
  - "Stop and re-plan if the touched-file set changes."
alternatives_considered:
  - description: "Use transient chat planning only."
    reason_not_chosen: "That would not leave a durable review surface."
review_hooks:
  - "Check scope truthfulness and validation sufficiency."
notes: "test note"
---

# Structured Plan Prompt

## Plan Summary

test

## PVF Lane Plan

- Initial PVF lane from issue creation: `prompt_template`
- Planned PVF lane for execution: `prompt_template`
- Planning lane source: `matched_initial_issue_lane`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Estimate confidence: `unknown`
- Estimate data source: `unknown`
- Estimate source ref: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Codex Plan

1. [pending] test

## Assumptions

- test

## Proposed Steps

1. test

## Affected Areas

- test

## Invariants To Preserve

- test

## Risks And Edge Cases

- test

## Test Strategy

- test

## Execution Handoff

test

## Stop Conditions

- test

## Notes

test
"#,
        slug = issue_ref.slug(),
        issue = issue_ref.issue_number(),
        task_id = issue_ref.task_issue_id(),
        title = title,
        branch = branch,
        stp_rel = stp_rel,
        sip_rel = sip_rel,
        spp_rel = spp_rel,
    );
    fs::write(path, content).expect("write authored spp");
}

pub(crate) fn write_authored_srp(
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    repo_root: &Path,
) {
    let stp_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_stp_path(repo_root));
    let sip_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_input_path(repo_root));
    let sor_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_output_path(repo_root));
    let content = format!(
        r#"---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "{slug}-review-prompt"
issue: {issue}
task_id: "{task_id}"
version: v0.86
title: "{title}"
branch: "{branch}"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/example/repo/issues/{issue}"
  - kind: "stp"
    ref: "{stp_rel}"
  - kind: "sip"
    ref: "{sip_rel}"
  - kind: "sor"
    ref: "{sor_rel}"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - "{stp_rel}"
  - "{sip_rel}"
in_scope_surfaces:
  - "tracked changes for this issue branch"
evidence_policy:
  - "Use repository evidence and issue-local validation only."
validation_inputs:
  - "Issue-local proofs recorded in the SOR."
allowed_dispositions:
  - "PASS"
  - "BLOCK"
reviewer_constraints:
  - "Do not widen issue scope."
refusal_policy:
  - "Refuse claims that are unsupported by repository evidence."
follow_up_routing:
  - "Route actionable findings back to the issue branch."
non_claims:
  - "This prompt does not claim review has already run."
  - "This prompt does not guarantee review quality by itself."
policy_refs:
  - "{stp_rel}"
review_results_exception: "explicit policy exception: fixture review results are not run."
notes: "test note"
---

# Structured Review Prompt

## Review Summary

test

## Scope Basis

- test

## In-Scope Surfaces

- test

## Evidence Rules

- test

## Validation Inputs

- test

## Allowed Dispositions

- PASS
- BLOCK

## Reviewer Constraints

- test

## Refusal Policy

- test

## Follow-up Routing

- test

## Non-Claims

- test

## Review Results

### Findings

- test

### Dispositions

- test

### Recommended Outcome

- test

## Notes

test
"#,
        slug = issue_ref.slug(),
        issue = issue_ref.issue_number(),
        task_id = issue_ref.task_issue_id(),
        title = title,
        branch = branch,
        stp_rel = stp_rel,
        sip_rel = sip_rel,
        sor_rel = sor_rel,
    );
    fs::write(path, content).expect("write authored srp");
}

pub(crate) fn write_design_time_ready_cards(
    repo: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
) {
    let source_path = issue_ref.issue_prompt_path(repo);
    ensure_task_bundle_stp(repo, issue_ref, &source_path).expect("ensure task bundle stp");
    ensure_bootstrap_cards(repo, issue_ref, title, branch, &source_path)
        .expect("bootstrap design-time cards");
    write_authored_sip(
        &issue_ref.task_bundle_input_path(repo),
        issue_ref,
        title,
        branch,
        &source_path,
        repo,
    );
    write_authored_spp(
        &issue_ref.task_bundle_plan_path(repo),
        issue_ref,
        title,
        branch,
        repo,
    );
    write_authored_srp(
        &issue_ref.task_bundle_review_policy_path(repo),
        issue_ref,
        title,
        branch,
        repo,
    );
}

pub(crate) fn write_completed_sor_fixture(path: &Path, branch: &str) {
    let body = format!(
        r#"# rust-finish-test

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1153
Run ID: issue-1153
Version: v0.86
Title: rust-finish-test
Branch: {branch}
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: Test
- Start Time: 2026-03-29T20:19:06Z
- End Time: 2026-03-29T20:19:09Z

## Summary

Finish test summary.

## PVF Lane Truth
- Initial PVF lane: `prompt_template`
- Planned PVF lane: `prompt_template`
- Final PVF lane: `prompt_template`
- Lane change reason: `no_lane_change`

## Issue Metrics Truth
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Artifacts produced
- Code:
  - `adl/src/cli/pr_cmd.rs`
- Generated runtime artifacts: not_applicable for this tooling task

## Actions taken
- Added Rust finish handling.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local validation before draft PR publication
- Verification performed:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
    Verified Rust `pr` command tests.
- Result: PASS

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout <branch> -- <path>` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- `Integration state` describes lifecycle state of the integrated artifact set, not where verification happened.
- `Verification scope` describes where the verification commands were run.
- `worktree_only` means at least one required path still exists only outside the main repository path.
- `pr_open` should pair with truthful `Worktree-only paths remaining` content; list those paths when they still exist only in the worktree or say `none` only when the branch contents are fully represented in the main repository path.
- If `Integration state` is `pr_open`, verify the actual proof artifacts rather than only the containing directory or card path.
- If `Integration method used` is `direct write in main repo`, `Verification scope` should normally be `main_repo` unless the deviation is explained.
- If `Verification scope` and `Integration method used` differ in a non-obvious way, explain the difference in one line.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
    Verified Rust `pr` command tests.
- Results:
  - PASS

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

Rules:
- Replace the example values below with one actual final value per field.
- Do not leave pipe-delimited enum menus or placeholder text in a finished record.

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo test --manifest-path adl/Cargo.toml pr_cmd"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: PASS
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
- Fixtures or scripts used:
  - direct Rust unit coverage
- Replay verification (same inputs -> same artifacts/order):
  - PASS
- Ordering guarantees (sorting / tie-break rules used):
  - Stable section ordering.
- Artifact stability notes:
  - not_applicable beyond deterministic record rendering.

## Security / Privacy Checks
- Secret leakage scan performed:
  - Verified test output uses repo-relative paths only.
- Prompt / tool argument redaction verified:
  - Verified issue template text is not emitted in PR bodies.
- Absolute path leakage check:
  - PASS
- Sandbox / policy invariants preserved:
  - PASS

## Replay Artifacts
- Trace bundle path(s): not_applicable for this tooling task
- Run artifact root: not_applicable for this tooling task
- Replay command used for verification:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
- Replay result:
  - PASS

## Artifact Verification
- Primary proof surface:
  - `adl/src/cli/pr_cmd.rs`
- Required artifacts present:
  - yes
- Artifact schema/version checks:
  - none
- Hash/byte-stability checks:
  - not_applicable
- Missing/optional artifacts and rationale:
  - none

## Decisions / Deviations
- Kept the fixture minimal while satisfying completed-phase validation.

## Follow-ups / Deferred work
- none
"#
    );
    fs::write(path, body).expect("write completed sor fixture");
}

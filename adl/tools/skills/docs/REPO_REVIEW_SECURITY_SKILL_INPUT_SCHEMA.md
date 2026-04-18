# Repo Review Security Skill Input Schema

```yaml
skill_input_schema: repo_review_security.v1
mode: review_repository | review_path | review_branch | review_diff | review_packet
repo_root: /absolute/path/to/repo
target:
  target_path: <repo-relative or absolute path optional>
  branch: <string optional>
  diff_base: <git ref optional>
  review_packet_path: <path optional>
  changed_paths:
    - <path optional>
  artifact_root: <path optional>
policy:
  review_depth: quick | standard | deep
  validation_mode: targeted | inspect_only | none
  write_review_artifact: true | false
  stop_after_review: true
```

## Purpose

Use this schema when invoking `repo-review-security` as a bounded security specialist in the multi-agent repository review suite. The skill focuses on trust boundaries, secret handling, injection risks, privilege failures, unsafe file/network IO, deserialization, supply-chain exposure, and abuse paths.

This document is the per-skill invocation contract. The suite-level orchestration and shared review artifact shape live in `MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md`.

## Supported Modes

- `review_repository` or `synthesize_specialist_artifacts` for whole-scope review/synthesis where applicable.
- `review_path` for a subtree or single file.
- `review_branch` for branch-oriented review.
- `review_diff` for diff-oriented review.
- `review_packet` or `synthesize_review_packet` for a prebuilt review packet or specialist artifact set.

## Required Top-Level Fields

- `skill_input_schema`
- `mode`
- `repo_root`
- `target`
- `policy`

## Mode And Policy Requirements

- `review_path` requires `target.target_path`.
- `review_branch` requires `target.branch`.
- `review_diff` requires `target.diff_base`.
- `review_packet` requires `target.review_packet_path`.
- `policy.review_depth` must be explicit.
- `policy.stop_after_review` must be `true`.

## Output Contract

Default artifact path:

```text
.adl/reviews/<timestamp>-repo-review-security.md
```

The artifact must follow the local output contract at:

```text
adl/tools/skills/repo-review-security/references/output-contract.md
```

It must also remain compatible with the shared suite contract in:

```text
adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md
```

## Stop Boundary

The skill must stop after producing the findings-only specialist review artifact. It must not edit code, tests, docs, configuration, issue trackers, PRs, or publication surfaces. It must not claim approval, merge readiness, remediation, or compliance certification.

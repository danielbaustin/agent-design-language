# Repo Review Synthesis Skill Input Schema

```yaml
skill_input_schema: repo_review_synthesis.v1
mode: synthesize_specialist_artifacts | synthesize_review_packet
repo_root: /absolute/path/to/repo
target:
  target_path: <path optional>
  branch: <string optional>
  diff_base: <git ref optional>
  specialist_artifacts:
    code: <path optional>
    security: <path optional>
    tests: <path optional>
    docs: <path optional>
    architecture: <path optional>
    dependency: <path optional>
  artifact_root: <path optional>
policy:
  required_roles:
    - code
    - security
    - tests
    - docs
  severity_policy: preserve_highest | preserve_role_severity
  write_review_artifact: true | false
  stop_after_synthesis: true
```

## Purpose

Use this schema when invoking `repo-review-synthesis` as a bounded synthesis reviewer in the multi-agent repository review suite. The skill focuses on severity-preserving merge of specialist findings, dedupe notes, disagreement handling, coverage matrix, and residual risk.

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

- `target.specialist_artifacts` must contain at least one specialist artifact.
- `policy.stop_after_synthesis` must be `true`.

## Output Contract

Default artifact path:

```text
.adl/reviews/<timestamp>-repo-review-synthesis.md
```

The artifact must follow the local output contract at:

```text
adl/tools/skills/repo-review-synthesis/references/output-contract.md
```

It must also remain compatible with the shared suite contract in:

```text
adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md
```

## Stop Boundary

The skill must stop after producing the synthesis artifact. It must not edit code, tests, docs, configuration, issue trackers, PRs, or publication surfaces. It must not claim approval, merge readiness, remediation, or compliance certification.

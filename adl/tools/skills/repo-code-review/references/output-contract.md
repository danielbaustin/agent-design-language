# Output Contract

Use the canonical review-surface structure from:
- `docs/tooling/review-surface-format.md`

The default repo-review profile is markdown with these exact sections in this order:

```md
## Metadata
- Review Type: repo_review
- Subject: <repo path or review target>
- Reviewer: <human or skill id>
- Date: <UTC timestamp or calendar date>
- Input Surfaces:
  - <path or command>
- Output Location: <path or none>

## Scope
- Reviewed: <high-signal reviewed areas>
- Not Reviewed: <explicit exclusions>
- Review Mode: code | docs | trace | mixed
- Gate: pre-merge | post-merge | release-tail

## Findings
1. [P1] Short title
Location: <path:line or artifact>
Impact: <behavioral consequence>
Trigger: <how it appears>
Evidence: <concrete evidence>
Fix Direction: <bounded repair direction>

## System-Level Assessment
<synthesis paragraph>

## Recommended Action Plan
- Fix now: <items>
- Fix before milestone closeout: <items>
- Defer: <items>

## Follow-ups / Deferred Work
- <follow-up or explicit none>

## Final Assessment
<bounded conclusion>
```

## Rules

- `Findings` must be the first analytical section after `Metadata` and `Scope`.
- If there are no significant findings, say `No material findings.` explicitly in the `Findings` section and use `Final Assessment` to record residual risk.
- Do not claim dynamic validation unless commands were actually run.
- If tests were not run, say so explicitly in `Scope`, `System-Level Assessment`, or `Final Assessment`.
- Use `P4` and `P5` only for concrete, non-speculative issues.
- Top-level manifests and dependency/build/toolchain config must be reflected in the reviewed scope unless the user explicitly scoped them out.
- Do not emit absolute host paths, secrets, raw prompts, or raw tool arguments.

## Validator

Repo-local validator:

```text
adl/tools/verify_repo_review_contract.rb
```

Fixture test:

```text
adl/tools/test_repo_review_contract.sh
```

## Default Artifact Location

When writing the review to disk by default, use:

```text
.adl/reviews/<timestamp>-repo-review.md
```

Use a timestamp formatted like `YYYYMMDD-HHMMSS`.

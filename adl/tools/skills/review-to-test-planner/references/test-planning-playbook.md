# Review To Test Planning Playbook

Use this reference when converting review findings into test work.

## Mapping Rules

- Prefer one finding to one focused test task.
- Merge findings only when they point to the same behavior and source path.
- Keep the behavior under test concrete and observable.
- Treat missing source path, missing behavior, or broad architectural concern as
  `deferred`.
- Treat real secrets, production accounts, destructive operations, billing,
  deployment, or unbounded mutation as `unsafe`.
- Prefer targeted validation commands over whole-repo commands unless the repo
  has no narrower truthful check.

## Test Location Heuristics

- Rust source: nearby module test or `adl/tests/<stem>_tests.rs`.
- Python source: `tests/test_<stem>.py`.
- JavaScript or TypeScript source: nearby `<stem>.test.ts` or
  `<stem>.test.tsx`.
- Shell script: `tests/<stem>.bats` or existing shell contract test.
- Markdown/docs command truth: docs validation or smoke test near existing docs
  tooling.
- Unknown file: defer until implementation context is available.

## Assertion Heuristics

- Parsing finding: assert accepted valid input and rejected invalid input.
- Permission finding: assert allowed and denied paths.
- Retry/timeout finding: assert retry count, terminal status, and sanitized
  error.
- Redaction finding: assert no secrets, absolute host paths, or prompt/tool
  argument leakage.
- Docs command finding: assert the documented command runs or mark docs-only
  correction as outside test generation.
- Dependency finding: assert install/check command or manifest policy behavior.

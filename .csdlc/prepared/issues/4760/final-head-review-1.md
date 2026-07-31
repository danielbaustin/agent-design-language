# #4760 Exact-Head Pre-PR Review

- Reviewed revision: `80a098937899ff4602d0c91d46ac61cff9453486`
- Reviewer: `codex:exact-head-reviewer-4760`
- Scope:
  - `adl/src/memory_palace.rs`
  - `adl/src/lib.rs`
  - `adl/src/long_lived_agent.rs`
  - `adl/tests/memory_palace_tests.rs`
  - `adl/tests/fixtures/memory_palace/long_running_context.json`
  - `.csdlc/issues/4760`
  - `.csdlc/evidence/4760`
  - `.csdlc/prepared/issues/4760`

## Findings

No open actionable findings at reviewed revision
`80a098937899ff4602d0c91d46ac61cff9453486`.

## Fixed Before This Review

- `review-prep-001`: canonical input hashing originally sorted trace references
  only by `event_sequence`, which could preserve input order for duplicate
  sequence numbers. The reviewed revision fixes this by ordering trace refs by
  sequence, kind, step id, and delegation id, and adds a regression assertion to
  the deterministic packet test.

## Residual Risk

- `cargo test --locked` remains unavailable because the branch's checked-in
  `adl/Cargo.lock` is stale for the current manifest graph. The issue-local
  validation wrapper refuses pre-existing lock dirtiness, runs the focused
  offline proof, and restores the transient Cargo lock refresh.
- Review was bounded to the #4760 Memory Palace implementation, fixture,
  long-lived-agent consumer hook, issue-local lifecycle state, and retained
  evidence. Broader long-lived-agent runtime behavior was not exhaustively
  revalidated.

## Validation Observed

- `.csdlc/prepared/issues/4760/validate_memory_palace.sh`: PASS at reviewed
  revision after the determinism fix.
- `git diff --check`: PASS before commit and before review recording.

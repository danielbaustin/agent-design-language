# ADL v0.5.0 Release Notes (Draft)

## Metadata
- Product: `Agent Design Language (ADL)`
- Version: `v0.5.0`
- Release date: `TBD`
- Tag: `v0.5.0` (pending publish)

---

## Summary

ADL v0.5.0 focuses on deterministic execution quality, bounded concurrency behavior, workflow signing enforcement, remote execution MVP boundaries, and hardened release tooling.

This document is release-ready content for the WP-11 closeout flow and should be finalized when tag + release publication are complete.

---

## Highlights

- Deterministic scheduler with bounded concurrency enforcement
- Workflow signing and enforcement (Ed25519-based)
- Remote execution MVP (explicit trust boundary, documented limitations)
- Include expansion before schema validation (composition fixes)
- Hardened git automation tooling (`pr.sh` + `pr_smoke.sh`)
- Deterministic test stabilization (no wall-clock dependency)

---

## What's New In Detail

### Runtime & Scheduling
- Deterministic execution ordering preserved across runs
- Configurable bounded concurrency (`max_concurrency`)
- Bounded-parallelism test stabilized with ordering assertions (no wall-clock timing gate)
- Concurrency control paths hardened (no panic-based `.expect()` failures)

### Signing & Enforcement
- Ed25519-based workflow signing
- Enforcement integrated into runtime execution
- Expanded tests for signing and enforcement behavior

### Remote Execution (MVP)
- Local scheduler / remote step execution boundary
- 5 MiB request size cap
- Explicit trust model and security limitations documented
- No authn/authz or request signing yet (tracked for v0.6)

### Tooling & Developer Workflow
- `pr.sh start` is worktree-first and idempotent
- Upstream mismatch detection with actionable remediation
- Non-destructive `pr_smoke.sh` validation script
- Regression tests for git automation scripts

### CLI & Schema Fixes
- Top-level include expansion before schema validation
- Composition workflows no longer rejected by CLI path gating

---

## Upgrade Notes

- No schema-breaking changes from v0.4.
- Remote execution remains MVP and must not be exposed publicly without external protections.
- Git automation scripts changed behavior; teams should use the updated `pr.sh` workflow.

---

## Known Limitations

- Remote execution lacks authentication and request signing (planned for v0.6).
- No checkpoint/recovery engine yet.
- No distributed multi-node execution.
- Runtime crate remains named `swarm` (rename deferred).

---

## Validation Notes

- Release-candidate validation gate for WP-11:
  - `cargo fmt --all`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test`
- Bounded-parallelism flake tracked and resolved in #393.

---

## What's Next (v0.6 Direction)

- First-class pattern constructs (planner/executor, debate, hierarchical)
- Minimal HITL pause/resume
- Streaming output support
- Provider profile registry
- Remote request signing + security envelope hardening
- Determinism strict mode and replay tooling

---

## Exit Criteria

- Notes reflect only shipped v0.5 behavior.
- Known limitations clearly separated from future work.
- Text is ready for GitHub Release UI at tag publish time.

# v0.90 Milestone Compression Pilot

## Status

WP-11 pilot for `#2030`.

## Purpose

This directory defines a small read-only milestone state model and drift-check
pilot for `v0.90`.

The goal is to reduce milestone ceremony time by making stale issue mappings,
proof-packet status, and release-tail truth mechanically visible before the
release tail. It is not an autonomous merge, release, or closeout system.

## Files

- `CANONICAL_MILESTONE_STATE_v0.90.yaml`: the minimal milestone state model.
- `DRIFT_CHECK_REPORT_v0.90.md`: pilot run result and known classifications.
- `FINISH_VALIDATION_PROFILES_v0.90.md`: compression-safe validation profile
  rules for low-risk finish paths.

## Drift Checker

Run from the repository root:

```bash
python3 adl/tools/check_v090_milestone_state.py
```

The checker is deliberately read-only. It validates:

- the v0.90 issue wave has the expected work package count;
- each work package issue number is visible in core milestone docs;
- the repo visibility and milestone compression proof packet statuses match
  tracked directories;
- planned demo rows remain truthfully classified as planned until their
  implementation WPs land.

## Finish Validation Profiles

Compression is split into two lanes:

- execution compression keeps issue scope, worktree binding, and drift checks
  small enough to move quickly;
- validation compression chooses the smallest truthful local validation set for
  the changed surface while keeping CI required before merge.

Low-risk docs/static-tooling issues may use the `FOCUSED_LOCAL_CI_GATED`
profile defined in `FINISH_VALIDATION_PROFILES_v0.90.md`. That profile requires
explicit changed paths, focused checks, SOR truth, root cleanliness, and CI
handoff. It must never be described as full local validation.

Runtime, schema, security, release, broad tooling, and ambiguous changes stay on
the `FULL_LOCAL` path unless a human explicitly records a different decision.

## Boundaries

- No GitHub network calls.
- No mutation of docs, issues, branches, PRs, or release state.
- No autonomous release approval.
- No replacement for human ceremony decisions.
- No claim that focused local validation is equivalent to full local validation.

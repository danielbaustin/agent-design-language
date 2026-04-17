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

## Boundaries

- No GitHub network calls.
- No mutation of docs, issues, branches, PRs, or release state.
- No autonomous release approval.
- No replacement for human ceremony decisions.

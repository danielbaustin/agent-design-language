# Milestone Compression Dashboard

This directory contains a reusable static HTML dashboard for milestone
compression visibility.

## Purpose

The dashboard gives one operator-facing, read-only view of:

- active WP wave state
- issue and PR/check posture
- validation profile expectations
- review-tail gates
- release blockers
- deferred findings and guarded non-claims
- immediate next operator action

It is a visibility layer only. It does not mutate GitHub issues, PRs, branches,
task cards, release state, or closeout records. It is not release authority and
must not hide review findings or replace the canonical milestone-compression
YAML, milestone docs, task cards, PR checks, or human release ceremony.

## Current Dataset

The bundled dataset is `v0.90.2`, refreshed from:

- `docs/milestones/v0.90.2/WBS_v0.90.2.md`
- `docs/milestones/v0.90.2/WP_ISSUE_WAVE_v0.90.2.yaml`
- `docs/milestones/v0.90.2/MILESTONE_CHECKLIST_v0.90.2.md`
- `docs/milestones/v0.90.2/RELEASE_PLAN_v0.90.2.md`
- `docs/milestones/v0.90.2/RUNTIME_V2_INHERITANCE_AND_COMPRESSION_AUDIT_v0.90.2.md`
- `docs/milestones/v0.90/milestone_compression/README.md`
- `docs/milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md`

Unknown, stale, or unverifiable evidence must be marked unknown/stale rather
than treated as green.

## Files

- `index.html` - static dashboard shell
- `style.css` - visual system and responsive layout
- `dashboard.js` - milestone dataset and rendering logic

## Validation

Run:

```bash
bash adl/tools/test_milestone_dashboard.sh
```

The validation checks that the static dashboard has the required compression
sections, is no longer a legacy-milestone-only artifact, does not contain private local
paths or obvious secret markers, and has JavaScript syntax that can be parsed
when `node` is available.

## Usage

Open `index.html` in a browser.

To adapt it for another milestone, update the `milestoneData` object in
`dashboard.js` from canonical milestone docs and keep the read-only boundary
visible.

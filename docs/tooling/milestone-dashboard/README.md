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

The bundled dataset is `v0.90.3`, refreshed from:

- `docs/milestones/v0.90.3/WBS_v0.90.3.md`
- `docs/milestones/v0.90.3/WP_ISSUE_WAVE_v0.90.3.yaml`
- `docs/milestones/v0.90.3/MILESTONE_CHECKLIST_v0.90.3.md`
- `docs/milestones/v0.90.3/RELEASE_PLAN_v0.90.3.md`
- `docs/milestones/v0.90/milestone_compression/README.md`
- `docs/milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md`

Unknown, stale, or unverifiable evidence must be marked unknown/stale rather
than treated as green.

## Refresh Rule

Refresh the dashboard during WP-01 or the first milestone-compression
readiness pass for each new milestone. At minimum, update:

- the `milestoneData` object in `dashboard.js`
- this Current Dataset section
- WP-to-issue mappings, status signals, blockers, next actions, and guarded
  non-claims
- validation expectations in `adl/tools/test_milestone_dashboard.sh` when the
  dashboard contract changes

The dashboard may remain static for now, but stale milestone truth should be
treated as a dashboard bug rather than an acceptable cache state.

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

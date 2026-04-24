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

The bundled dataset is `v0.90.4`, mirrored in `data/v0.90.4.js` and refreshed
from:

- `docs/milestones/v0.90.4/README.md`
- `docs/milestones/v0.90.4/WBS_v0.90.4.md`
- `docs/milestones/v0.90.4/WP_ISSUE_WAVE_v0.90.4.yaml`
- `docs/milestones/v0.90.4/MILESTONE_CHECKLIST_v0.90.4.md`
- `docs/milestones/v0.90.4/RELEASE_PLAN_v0.90.4.md`
- `docs/milestones/v0.90/milestone_compression/README.md`
- `docs/milestones/v0.90/milestone_compression/CANONICAL_MILESTONE_STATE_v0.90.yaml`
- `docs/milestones/v0.90/milestone_compression/DRIFT_CHECK_REPORT_v0.90.md`
- `docs/milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md`
- a bounded read-only GitHub snapshot of the live v0.90.4 issue wave and PR
  posture taken at refresh time

Unknown, stale, or unverifiable evidence must be marked unknown/stale rather
than treated as green.

## Refresh Rule

Refresh the dashboard during WP-01 or the first milestone-compression
readiness pass for each new milestone. At minimum, update:

- the current milestone data file under `data/`
- the `<script src>` target in `index.html` when switching milestones
- this Current Dataset section
- WP-to-issue mappings, freshness signals, PR/check posture, blockers, next
  actions, and guarded non-claims
- validation expectations in `adl/tools/test_milestone_dashboard.sh` when the
  dashboard contract changes

The dashboard may remain static for now, but stale milestone truth should be
treated as a dashboard bug rather than an acceptable cache state.

## Files

- `index.html` - static dashboard shell
- `style.css` - visual system and responsive layout
- `dashboard.js` - renderer and stale/unknown-state handling
- `data/` - bounded milestone snapshot data files

## Validation

Run:

```bash
bash adl/tools/test_milestone_dashboard.sh
```

The validation checks that the static dashboard has the required compression
sections, is no longer a legacy-milestone-only artifact, does not contain
private local paths or obvious secret markers, and has JavaScript syntax that
can be parsed when `node` is available.

## Usage

Open `index.html` in a browser.

To adapt it for another milestone, add or refresh a dataset file in `data/`,
update the dataset script reference in `index.html`, and keep the read-only
boundary visible. The renderer should stay generic; the milestone snapshot
should carry the current mirrored truth and explicit unknown/stale notes.

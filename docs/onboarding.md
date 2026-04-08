# Contributor Onboarding (Docs + Reports)

Use this page when you need to orient quickly in the ADL repo.

## Where to Add or Update Docs

- Project overview: `README.md`
- Tooling workflow docs: `adl/tools/README.md`
- Language docs: `adl-spec/`
- Contributor planning docs: `docs/`

## Where Reports Live

- `.adl/reports/burst/<timestamp_utc_z>/` (burst artifacts)
- `.adl/reports/pr-cycle/<issue>/<timestamp_utc_z>/` (per-issue cycle reports)
- `.adl/reports/INDEX.md` (report directory orientation)

## Workflow Context

Default workflow uses `adl_pr_cycle` with the real authoring control plane:
- `pr init`
- `pr ready`
- `pr run`
- `pr finish`

Canonical local STPs live under `.adl/v0.85/tasks/<task-id>__<slug>/`, compatibility cards live under `.adl/cards/<issue>/`, and repo-local execution clones live under `.worktrees/adl-wp-<issue>/`.
GitHub issue state is the source of truth for whether a card is active or complete. Active/current cards stay flat under `.adl/cards/<issue>/` while milestone work is in flight; completed cards may be archived later under `.adl/cards/completed/<milestone>/<issue>/`.
The browser/editor adapter remains narrower than the full control plane; execution work should follow the `pr ready` -> `pr run` path rather than the older `pr start` model.

## Reading Order

1. `README.md`
2. `adl/tools/README.md`
3. `.adl/reports/INDEX.md`

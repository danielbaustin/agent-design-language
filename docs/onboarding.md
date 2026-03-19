# Contributor Onboarding (Docs + Reports)

Use this page when you need to orient quickly in the ADL repo.

## Where to Add or Update Docs

- Project overview: `README.md`
- Tooling workflow docs: `swarm/tools/README.md`
- Language docs: `adl-spec/`
- Contributor planning docs: `docs/`

## Where Reports Live

- `.adl/reports/burst/<timestamp>/` (burst artifacts)
- `.adl/reports/pr-cycle/<issue>/<timestamp>/` (per-issue cycle reports)
- `.adl/reports/INDEX.md` (report directory orientation)

## Workflow Context

Default workflow uses `adl_pr_cycle` with canonical local task bundles under `.adl/<scope>/tasks/<task-id>__<slug>/`, plus compatibility links under `.adl/cards/<issue>/`.
Default workflow currently seeds compatibility card paths under `.adl/cards/<issue>/`, while the canonical local draft prompt bundles live under `.adl/v0.85/tasks/<task-id>__<slug>/`.
The canonical local draft prompt bundles live under `.adl/<scope>/tasks/<task-id>__<slug>/`, and compatibility links under `.adl/cards/<issue>/` remain available during migration.

## Reading Order

1. `README.md`
2. `swarm/tools/README.md`
3. `.adl/reports/INDEX.md`

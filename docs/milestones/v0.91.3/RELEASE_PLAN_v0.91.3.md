# v0.91.3 Release Plan

## Metadata

- Milestone: `v0.91.3`
- Version: `v0.91.3`
- Release date: pending release ceremony
- Release manager: ADL maintainers

## How To Use

Execute this plan during the release tail. Check items only when evidence
exists. Ceremony is confirmation and publication, not hidden implementation.

## 0. Release-Tail Convergence

- [x] Demo/proof coverage complete or gaps routed:
  [DEMO_MATRIX_v0.91.3.md](DEMO_MATRIX_v0.91.3.md)
- [x] Quality gate complete:
  [QUALITY_GATE_v0.91.3.md](QUALITY_GATE_v0.91.3.md)
- [x] Docs review pass complete and findings resolved or routed.
- [ ] Internal review complete.
- [ ] External / third-party review complete.
- [ ] Review remediation complete.
- [ ] Next milestone planning complete:
  [NEXT_MILESTONE_HANDOFF_v0.91.3.md](NEXT_MILESTONE_HANDOFF_v0.91.3.md)
- [ ] Final next-milestone review pass complete.
- [ ] Closed issue/card truth refreshed for the full issue wave.
- [ ] Release-truth docs aligned:
  - `README.md`
  - `CHANGELOG.md`
  - `adl/Cargo.toml`
  - [README](README.md)
  - [MILESTONE_CHECKLIST](MILESTONE_CHECKLIST_v0.91.3.md)
  - [RELEASE_NOTES](RELEASE_NOTES_v0.91.3.md)

## 1. Release Readiness

- [ ] Milestone checklist complete:
  [MILESTONE_CHECKLIST_v0.91.3.md](MILESTONE_CHECKLIST_v0.91.3.md)
- [ ] Release notes approved:
  [RELEASE_NOTES_v0.91.3.md](RELEASE_NOTES_v0.91.3.md)
- [ ] Go/no-go decision recorded in:
  [DECISIONS_v0.91.3.md](DECISIONS_v0.91.3.md)

## 2. Branch And Tag Preparation

- [ ] Target branch confirmed: `main`
- [ ] Working tree clean.
- [ ] Version strings validated.
- [ ] Tag created: `v0.91.3`
- [ ] Tag pushed and verified.

## 3. GitHub Release Steps

- [ ] GitHub Release draft created from `v0.91.3`.
- [ ] Release body populated from approved release notes.
- [ ] Links to key PRs/issues included.
- [ ] Release visibility confirmed.
- [ ] Release published.

## 4. Verification

- [ ] Post-release CI status checked.
- [ ] Release links tested.
- [ ] Immediate regressions triaged and tracked.

## 5. Communication

- [ ] Roadmap/status docs updated.
- [ ] `v0.91.4` start state confirmed.
- [ ] Deferred items routed to the next milestone backlog.

## Exit Criteria

- No hidden implementation remains in ceremony.
- Tag and release are published and accessible.
- Verification is complete with no unknown critical failures.
- The `v0.91.4` handoff is concrete.

# ADL v0.5 Milestone Checklist

## Metadata
- Milestone: `v0.5`
- Version: `0.5.0`
- Target release date: `TBD`
- Owner: `Daniel Austin`

## Purpose
Ship/no-ship gate for v0.5. Check items only when evidence exists (links, PRs, CI runs, demo logs).

## Status snapshot (2026-02-21)

Closed v0.5 issues during closeout:
- #373 — include expansion / CLI schema gating fixed (merged)
- #378 — bulletproof git automation scripts (`swarm/tools/pr.sh` + `pr_smoke.sh`) (merged)
- #393 — bounded-parallelism test stabilized (merged)
- #362 — WP-09 Documentation pass merged
- #363 — WP-10 Review/regression audit merged
- #308 — v0.5 epic closed

Open v0.5 issues remaining:
- #364 — WP-11 Closing ceremony (release + cleanup)

v0.6 issues remain open and are out of scope for the v0.5 ship gate.

---

## Planning
- [x] Milestone goal defined (`docs/milestones/v0.5/DESIGN_v0.5.md`)
- [x] Scope + non-goals documented (`docs/milestones/v0.5/DESIGN_v0.5.md`)
- [x] WBS created and mapped to work packages (`docs/milestones/v0.5/WBS_v0.5.md`)
- [x] Decision log initialized (`docs/milestones/v0.5/DECISIONS_v0.5.md`)
- [x] Sprint plan created (`docs/milestones/v0.5/SPRINT_v0.5.md`)
- [x] v0.5 epic(s) created and linked in WBS / docs (GitHub: #308)

## Execution Discipline
- [x] Each issue has input/output cards under `.adl/cards/<issue>/` (enforced in v0.5 workflow)
- [ ] Each burst writes artifacts under `.adl/reports/burst/<timestamp>/`
- [ ] Draft PR opened for each issue before merge
- [ ] Transient failures retried and documented
- [ ] "Green-only merge" policy followed
- [ ] Work unit boundaries respected (no drift beyond WBS scope freeze)

## Quality Gates
- [ ] `cargo fmt` passes (release candidate)
- [ ] `cargo clippy --all-targets -- -D warnings` passes (release candidate)
- [ ] `cargo test` passes (release candidate)
- [x] Flaky timing tests eliminated or stabilized (see #393)
- [ ] CI is green on the merge target (`swarm-ci`, `swarm-coverage`)
- [ ] Coverage signal is not red (or exception documented)
- [ ] No unresolved high-priority blockers (link: `.adl/reports/triage/` or issue list snapshot)

## Feature Gates (v0.5)
### Language surface
- [ ] Explicit schemas exist for all 6 primitives:
  - [ ] Agents
  - [ ] Runs
  - [ ] Providers
  - [ ] Tasks
  - [ ] Tools
  - [ ] Workflows
- [ ] Composition rules defined and enforced (validation + deterministic ordering)
- [ ] Examples exist for each primitive alone and composed

### Patterns + compilation
- [ ] Pattern schema v0.1 exists and is validated
- [ ] Pattern → ExecutionPlan compilation implemented
- [ ] Deterministic multi-agent turn ordering guaranteed and tested

### Scheduler configurability
- [ ] Configurable concurrency limit (schema → runtime) implemented
- [ ] Determinism preserved across limits (tests for 1, 2, N)
- [ ] Trace includes scheduler/config context

### Distributed + trust model (original v0.3 goals)
- [ ] Remote execution MVP implemented (reference server + client)
- [ ] Mixed placement workflow example exists (local + remote)
- [ ] Workflow signing implemented:
  - [ ] `adl sign` works
  - [ ] `adl verify` works
  - [ ] `adl run` enforces signing (dev override documented)
- [ ] Trace export + replay continues to work with new features

## Demo Generation Pass
- [ ] Demo matrix complete (primitive-alone + composition)
- [ ] Structural demos exist and run end-to-end:
  - [ ] Linear
  - [ ] Multi-step chain
  - [ ] Fork/join
  - [ ] Hierarchical
  - [ ] Pattern-based debate
  - [ ] Planner-executor
  - [ ] Mixed local/remote placement
  - [ ] Signed workflow execution
  - [ ] Deterministic replay
- [ ] All demos are one-command runnable and documented
- [ ] Demos provide readable timestamps + user-visible progress output

## Documentation Pass
- [ ] Root README updated for v0.5 (what ADL is, why it matters, quickstart)
- [ ] `swarm/README.md` updated and consistent with root README
- [ ] Spec docs updated (`adl-spec/` docs where applicable)
- [ ] All demo commands in docs are copy/paste verified

## Review Pass
- [ ] Nightly coverage automation run and report attached (target: 100%)
- [ ] Nightly docs consistency automation run and report attached
- [ ] Regression scan performed (no drift from v0.4 demos)
- [ ] External “new user” walkthrough performed from README

## Release Packaging
- [ ] Release notes finalized (`docs/milestones/v0.5/RELEASE_NOTES_v0.5.md`)
- [ ] Tag verified: `v0.5.0`
- [ ] GitHub Release drafted and published (body based on release notes)
- [ ] Links validated in release body
- [ ] Release announcement draft prepared (`docs/milestones/v0.5/RELEASE_NOTES_v0.5.md`)

## Post-Release
- [ ] Milestone/epic issues closed with release links
- [ ] Deferred items moved to next milestone backlog
- [ ] Follow-up bugs/tech debt captured as issues
- [ ] Roadmap/status docs updated
- [ ] Retrospective summary recorded (in release plan or separate doc)

## Exit Criteria
- All required gates are checked, or each exception has an owner + due date.
- Milestone can be audited end-to-end via the links captured above.

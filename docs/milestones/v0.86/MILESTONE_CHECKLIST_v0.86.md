# Milestone Checklist — v0.86

## Metadata
- Milestone: v0.86
- Version: 0.86
- Target release date: pending manual release ceremony
- Owner: Daniel Austin

## Purpose
Ship/no-ship gate for v0.86.

This milestone must prove a working **bounded cognitive system**, not just concept alignment or a partial control layer.

---

## Planning
- [x] Milestone goal defined (VISION + DESIGN aligned to bounded cognitive system)
- [x] Scope + non-goals documented (full v0.86 cognitive stack and loop included)
- [x] WBS created and mapped to issues (WP-01 through WP-23)
- [x] Decision log initialized
- [x] Sprint plan created and aligned to WPs

---

## Execution Discipline
- [x] Each issue has input/output cards under `.adl/cards/<issue>/`
- [ ] Each burst writes artifacts under `.adl/reports/burst/<timestamp_utc_z>/`
- [x] Draft or review PR opened for each issue before merge
- [ ] Transient failures retried and documented
- [x] "Green-only merge" policy followed

---

## Quality Gates
- [x] `cargo fmt` passes
- [x] `cargo clippy --all-targets -- -D warnings` passes
- [x] `cargo test` passes
- [x] CI is green on the merge target
- [x] Coverage signal is not red (or exception documented)
- [ ] No unresolved high-priority blockers

Sprint 7 gate note (2026-04-01):
- Local repo-enforced gate commands passed in the `WP-17` worktree.
- Current GitHub Actions `ci` and nightly coverage runs on `main` are green.
- Coverage is over the enforced limits: workspace line coverage is `91.50%` against a `90%` threshold, and the per-file gate passed at `>= 80%`.
- The quality-gate run exposed a workflow-path drift in this branch snapshot: `.github/workflows/ci.yaml` referenced `tools/check_release_notes_commands.sh`, while the actual current script path is `adl/tools/check_release_notes_commands.sh`, which passed locally when invoked directly.
- The release-tail version correction landed via `#1290`.

---

## Functional / System Proof (v0.86-specific)

### Cognitive Foundations
- [x] Cognitive loop executes end-to-end (all canonical stages present)
- [x] Cognitive stack matches loop implementation (no competing definitions)
- [x] Instinct and affect signals are present and influence execution

### Arbitration + Reasoning
- [x] Arbitration produces explicit routing decisions (`route_selected`, `confidence`, `risk_class`)
- [x] Fast and slow paths are both implemented and selectable
- [x] Routing decisions are influenced by signals and context

### Agency + Execution
- [x] Candidate generation + selection is observable (agency is real)
- [x] Bounded execution (AEE-lite) performs at least one visible iteration
- [x] Evaluation signals (progress, contradiction, failure) are emitted
- [x] Evaluation signals influence behavior or termination

### Adaptation + Memory + Control
- [x] Frame adequacy is assessed in at least one scenario
- [x] Reframing/adaptation occurs in at least one scenario
- [x] Memory participation (ObsMem-lite) is visible in outputs or control flow
- [x] Freedom Gate produces allow / defer / refuse decisions with artifacts

### Integrated Cognitive System
- [x] One canonical bounded cognitive path exists
- [x] End-to-end run traverses signals → arbitration → reasoning → execution → evaluation → reframing → memory → Freedom Gate
- [x] No competing or parallel cognitive paths exist

### Artifacts + Proof Surfaces
- [x] All major stages emit schema-compliant artifacts
- [x] Artifact set is coherent and inspectable end-to-end
- [x] Stage outputs are stable and consistently named

---

## Demo / Proof Surface
- [x] At least one local agent demo exercises the full bounded cognitive system
- [x] Demo proves signals, arbitration, fast/slow reasoning, candidate selection, bounded execution, evaluation, reframing, memory participation, and Freedom Gate
- [x] Demo matrix is accurate and reviewable
- [x] Reviewers can run one obvious command and inspect outputs

---

## Documentation Integrity
- [x] DESIGN, VISION, WBS, SPRINT, and planning docs are consistent with implementation
- [x] No conceptual drift between docs and runtime behavior
- [x] Demo matrix matches actual runnable demos
- [x] All v0.86 planning docs are implemented (not aspirational)

---

## Review
- [x] Internal review completed and findings recorded
- [x] External / 3rd-party review package prepared
- [x] Review findings resolved or explicitly deferred with ownership

---

## Release Packaging
- [x] Release notes finalized
- [ ] Tag verified: v0.86
- [ ] GitHub Release drafted
- [x] Links validated in release notes / draft-release source surfaces
- [ ] Release published

---

## Post-Release
- [ ] Milestone/epic issues closed with release links
- [x] Deferred items moved to next milestone backlog
- [x] Follow-up bugs/tech debt captured as issues
- [x] Roadmap/status docs updated
- [ ] Retrospective summary recorded

---

## Exit Criteria
- The bounded cognitive system executes as a coherent whole, not isolated components
- End-to-end demo proves the full cognitive loop (signals → arbitration → reasoning → execution → evaluation → reframing → memory → Freedom Gate)
- Signals, arbitration, agency, bounded execution, evaluation, reframing, memory participation, and Freedom Gate behavior are all observable
- All artifacts are present, consistent, and inspectable
- Docs and implementation are in agreement
- Repo is clean and ready for release

Prepared-state note:
- Manual ceremony items still pending here are the tag, GitHub release creation/publish, and final post-release link closure.

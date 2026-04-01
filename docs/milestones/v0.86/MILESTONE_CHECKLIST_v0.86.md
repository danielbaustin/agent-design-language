# Milestone Checklist — v0.86

## Metadata
- Milestone: v0.86
- Version: 0.86
- Target release date: end of 2-day execution window
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
- [ ] Each issue has input/output cards under `.adl/cards/<issue>/`
- [ ] Each burst writes artifacts under `.adl/reports/burst/<timestamp_utc_z>/`
- [ ] Draft PR opened for each issue before merge
- [ ] Transient failures retried and documented
- [ ] "Green-only merge" policy followed

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
- This branch is behind `origin/main` and its checked-in `.github/workflows/ci.yaml` still references `tools/check_release_notes_commands.sh`; the actual current script path is `adl/tools/check_release_notes_commands.sh`, which passed locally when invoked directly.

---

## Functional / System Proof (v0.86-specific)

### Cognitive Foundations
- [ ] Cognitive loop executes end-to-end (all canonical stages present)
- [ ] Cognitive stack matches loop implementation (no competing definitions)
- [ ] Instinct and affect signals are present and influence execution

### Arbitration + Reasoning
- [ ] Arbitration produces explicit routing decisions (`route_selected`, `confidence`, `risk_class`)
- [ ] Fast and slow paths are both implemented and selectable
- [ ] Routing decisions are influenced by signals and context

### Agency + Execution
- [ ] Candidate generation + selection is observable (agency is real)
- [ ] Bounded execution (AEE-lite) performs at least one visible iteration
- [ ] Evaluation signals (progress, contradiction, failure) are emitted
- [ ] Evaluation signals influence behavior or termination

### Adaptation + Memory + Control
- [ ] Frame adequacy is assessed in at least one scenario
- [ ] Reframing/adaptation occurs in at least one scenario
- [ ] Memory participation (ObsMem-lite) is visible in outputs or control flow
- [ ] Freedom Gate produces allow / defer / refuse decisions with artifacts

### Integrated Cognitive System
- [ ] One canonical bounded cognitive path exists
- [ ] End-to-end run traverses signals → arbitration → reasoning → execution → evaluation → reframing → memory → Freedom Gate
- [ ] No competing or parallel cognitive paths exist

### Artifacts + Proof Surfaces
- [ ] All major stages emit schema-compliant artifacts
- [ ] Artifact set is coherent and inspectable end-to-end
- [ ] Stage outputs are stable and consistently named

---

## Demo / Proof Surface
- [x] At least one local agent demo exercises the full bounded cognitive system
- [x] Demo proves signals, arbitration, fast/slow reasoning, candidate selection, bounded execution, evaluation, reframing, memory participation, and Freedom Gate
- [x] Demo matrix is accurate and reviewable
- [x] Reviewers can run one obvious command and inspect outputs

---

## Documentation Integrity
- [ ] DESIGN, VISION, WBS, SPRINT, and planning docs are consistent with implementation
- [ ] No conceptual drift between docs and runtime behavior
- [ ] Demo matrix matches actual runnable demos
- [ ] All v0.86 planning docs are implemented (not aspirational)

---

## Review
- [ ] Internal review completed and findings recorded
- [ ] External / 3rd-party review package prepared
- [ ] Review findings resolved or explicitly deferred with ownership

---

## Release Packaging
- [ ] Release notes finalized
- [ ] Tag verified: v0.86
- [ ] GitHub Release drafted
- [ ] Links validated in release body
- [ ] Release published

---

## Post-Release
- [ ] Milestone/epic issues closed with release links
- [ ] Deferred items moved to next milestone backlog
- [ ] Follow-up bugs/tech debt captured as issues
- [ ] Roadmap/status docs updated
- [ ] Retrospective summary recorded

---

## Exit Criteria
- The bounded cognitive system executes as a coherent whole, not isolated components
- End-to-end demo proves the full cognitive loop (signals → arbitration → reasoning → execution → evaluation → reframing → memory → Freedom Gate)
- Signals, arbitration, agency, bounded execution, evaluation, reframing, memory participation, and Freedom Gate behavior are all observable
- All artifacts are present, consistent, and inspectable
- Docs and implementation are in agreement
- Repo is clean and ready for release

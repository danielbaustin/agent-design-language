# Milestone Checklist — v0.75

## Metadata
- Milestone: v0.75
- Version: 0.75
- Target release window: Week of 2026-03-09 (tentative)
- Owner: Daniel / Agent Logic team
- Reconciled: 2026-03-06 (issue #657)

## Purpose
Ship / no-ship gate for v0.75.

v0.75 is complete only when the deterministic substrate (EPIC‑A) is frozen and ObsMem v1 (EPIC‑B) is integrated, demonstrable, and covered by tests + CI gates.

Check items only when objective evidence exists.

---

## Planning
- [x] VISION_0.75.md finalized and aligned with redistributed scope (A+B only)
- [x] DESIGN_0.75.md contains no placeholders and documents frozen contracts
- [x] WBS_0.75.md mapped to concrete GitHub issues (WP-01..WP-16)
- [x] DECISIONS_0.75.md initialized and key design decisions recorded
- [x] SPRINT_0.75.md created and reflects current execution order
- [x] RELEASE_PLAN_0.75.md drafted

---

## Deterministic Substrate (EPIC‑A)
- [x] Activation log schema frozen and documented
- [x] Replay runner deterministically reproduces outputs (excluding run-id/timestamps)
- [x] Trace bundle v2 spec documented and versioned
- [x] Trace bundle export implemented and tested
- [x] Trace bundle import works and replay-from-bundle demonstrated
- [x] Failure taxonomy stabilized with deterministic classification codes
- [x] No secrets persisted in artifacts or bundles
- [x] No absolute host paths persisted

Evidence:
- Links to relevant tests
- Example bundle artifact path

---

## ObsMem v1 (EPIC‑B)
- [x] Index schema versioned and documented
- [x] Bundle ingestion is deterministic and tested
- [x] Structured query returns deterministically ordered results
- [x] Tie-break rules documented and enforced
- [x] Optional hybrid retrieval records model/config and remains deterministic
- [x] Retrieval explanations + citations implemented
- [x] Operational report surfaces produce stable, deterministic output

Evidence:
- Example query command + output
- Example citation referencing a trace artifact

---

## Demo Matrix (v0.75)
- [x] Demo A — Determinism + Replay
- [x] Demo B — Ingest + Similarity Query + Citations
- [x] Demo C — Operational Report
- [x] All demos run from docs on a fresh checkout
- [x] Artifact trees verified as stable where applicable

Reference:
- `docs/milestones/v0.75/DEMO_MATRIX.md`

---

## Quality Gates
- [x] `cargo fmt` passes
- [x] `cargo clippy --all-targets -- -D warnings` passes
- [x] `cargo test --workspace` passes (run twice consecutively)
- [x] Coverage meets or exceeds ratchet threshold (workspace line coverage ≥ target)
- [x] Per-file runtime line coverage floor (>= 80%) satisfied with explicit documented exclusions
- [x] CI required checks are green on merge target
- [x] No flaky tests
- [x] No unresolved high-priority blockers

---

## Documentation & Review
- [x] All cross-links between v0.75 / v0.8 / v0.85 planning docs are consistent
- [x] No stale references to cluster or Gödel in v0.75 docs
- [x] Architecture doc updated to reflect milestone slicing
- [x] WP-15A doc alignment PR merged (#601)
- [x] 3rd-party review pass findings addressed via review-fix batch (#644, #645, #646, #647, #648)
- [x] Coverage policy doc aligned with CI thresholds and exclusions (`docs/milestones/v0.75/COVERAGE_POLICY_0.75.md`)

---

## Release Packaging
- [ ] RELEASE_NOTES_0.75.md finalized
- [ ] Tag verified: v0.75
- [ ] GitHub Release drafted and links validated
- [ ] Release published

---

## Post-Release
- [ ] Milestone issues closed with release links
- [ ] Deferred items moved to v0.8 or v0.85 backlog
- [ ] Follow-up bugs/tech debt captured as issues
- [ ] Roadmap/status docs updated
- [ ] Retrospective summary recorded

---

## Evidence Snapshot
- Planning/doc alignment completed and frozen pre-review in issue #601.
- Review-fix batch merged in issues #644–#648.
- Demo execution and artifact expectations are captured in `docs/milestones/v0.75/DEMO_MATRIX.md`.
- Coverage and exclusions policy is captured in `docs/milestones/v0.75/COVERAGE_POLICY_0.75.md`.
- Release-ceremony and post-release items remain intentionally unchecked until ceremony execution.

---

## Exit Criteria
- All required gates are checked OR each unchecked item has an owner and due date.
- v0.75 can be audited end-to-end via links captured above.
- v0.8 planning can proceed without reopening substrate or ObsMem contracts.

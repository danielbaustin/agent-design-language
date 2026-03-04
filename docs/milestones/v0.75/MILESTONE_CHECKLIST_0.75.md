# Milestone Checklist — v0.75

## Metadata
- Milestone: v0.75
- Version: 0.75
- Target release window: Week of 2026-03-09 (tentative)
- Owner: Daniel / Agent Logic team

## Purpose
Ship / no-ship gate for v0.75.

v0.75 is complete only when the deterministic substrate (EPIC‑A) is frozen and ObsMem v1 (EPIC‑B) is integrated, demonstrable, and covered by tests + CI gates.

Check items only when objective evidence exists.

---

## Planning
- [ ] VISION_0.75.md finalized and aligned with redistributed scope (A+B only)
- [ ] DESIGN_0.75.md contains no placeholders and documents frozen contracts
- [ ] WBS_0.75.md mapped to concrete GitHub issues (WP-01..WP-16)
- [ ] DECISIONS_0.75.md initialized and key design decisions recorded
- [ ] SPRINT_0.75.md created and reflects current execution order
- [ ] RELEASE_PLAN_0.75.md drafted

---

## Deterministic Substrate (EPIC‑A)
- [ ] Activation log schema frozen and documented
- [ ] Replay runner deterministically reproduces outputs (excluding run-id/timestamps)
- [ ] Trace bundle v2 spec documented and versioned
- [ ] Trace bundle export implemented and tested
- [ ] Trace bundle import works and replay-from-bundle demonstrated
- [ ] Failure taxonomy stabilized with deterministic classification codes
- [ ] No secrets persisted in artifacts or bundles
- [ ] No absolute host paths persisted

Evidence:
- Links to relevant tests
- Example bundle artifact path

---

## ObsMem v1 (EPIC‑B)
- [ ] Index schema versioned and documented
- [ ] Bundle ingestion is deterministic and tested
- [ ] Structured query returns deterministically ordered results
- [ ] Tie-break rules documented and enforced
- [ ] Optional hybrid retrieval records model/config and remains deterministic
- [ ] Retrieval explanations + citations implemented
- [ ] Operational report surfaces produce stable, deterministic output

Evidence:
- Example query command + output
- Example citation referencing a trace artifact

---

## Demo Matrix (v0.75)
- [ ] Demo A — Determinism + Replay
- [ ] Demo B — Ingest + Similarity Query + Citations
- [ ] Demo C — Operational Report
- [ ] All demos run from docs on a fresh checkout
- [ ] Artifact trees verified as stable where applicable

Reference:
- `docs/milestones/v0.75/DEMOS_v0.75.md`

---

## Quality Gates
- [ ] `cargo fmt` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes (run twice consecutively)
- [ ] Coverage meets or exceeds ratchet threshold (workspace line coverage ≥ target)
- [ ] CI required checks are green on merge target
- [ ] No flaky tests
- [ ] No unresolved high-priority blockers

---

## Documentation & Review
- [ ] All cross-links between v0.75 / v0.8 / v0.85 planning docs are consistent
- [ ] No stale references to cluster or Gödel in v0.75 docs
- [ ] Architecture doc updated to reflect milestone slicing
- [ ] WP-15A doc alignment PR merged
- [ ] Review notes addressed

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

## Exit Criteria
- All required gates are checked OR each unchecked item has an owner and due date.
- v0.75 can be audited end-to-end via links captured above.
- v0.8 planning can proceed without reopening substrate or ObsMem contracts.

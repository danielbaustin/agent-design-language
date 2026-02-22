# Release Plan — v0.6

## Metadata
- Milestone: v0.6
- Version: v0.6.0
- Owner: ADL core (Daniel + Codex-assisted implementation)
- Governing WPs: #401–#411
- Coverage Gate: WP-H2 (#409)

---

# Release Identity

v0.6 is a **stabilize + formalize** release.

It does not introduce adaptive learning or distributed systems.
It strengthens the runtime foundation by:

- Expanding the pattern compiler surface (WP-A)
- Introducing minimal HITL pause/resume (WP-B)
- Adding streaming output without breaking determinism (WP-C)
- Formalizing provider profiles (WP-D)
- Logging structured delegation metadata (WP-E)
- Hardening scheduler and determinism invariants (WP-F)
- Improving instrumentation and replay tooling (WP-G)
- Validating demos and integration matrix (WP-H)
- Establishing a coverage ratchet discipline (WP-H2)
- Completing docs + review pass (WP-I)

v0.6 must ship clean, reproducible, and architecturally coherent.

---

# Entry Criteria (Release Candidate Gate)

Before release branch/tag creation:

## 1. WP Completion

- All WPs #401–#411 marked complete or explicitly deferred.
- Any deferral includes:
  - Rationale
  - Linked issue
  - Owner

## 2. Determinism Validation

- Concurrency tests pass consistently across multiple runs.
- Replay/diff tooling (WP-G) confirms byte-stable plan output.
- No streaming feature alters execution ordering or artifact bytes.
- Delegation remains metadata-only (no policy enforcement).

## 3. Coverage Gate (WP-H2 #409)

- >80% coverage per file, OR
- Documented exception with:
  - Explicit issue
  - Owner
  - Justification

Coverage status must be recorded in the milestone checklist.

## 4. CI Clean

- `cargo fmt --all`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- CI green on `main`

No red pipelines allowed at tag time.

---

# Release Procedure

## Phase 1 — Final Freeze

1. Merge final WP PR.
2. Run full test suite locally.
3. Verify no open `version:v0.6` runtime blockers.
4. Re-run coverage audit.
5. Confirm milestone checklist fully satisfied.

Freeze window begins after checklist confirmation.

---

## Phase 2 — Version + Tag

1. Confirm version number (v0.6.0).
2. Ensure working tree clean from fresh checkout.
3. Tag:

   ```
   git tag -a v0.6.0 -m "ADL v0.6.0"
   git push origin v0.6.0
   ```

4. Verify tag points to intended commit.

---

## Phase 3 — Release Publication

1. Finalize `RELEASE_NOTES_v0.6.md`
   - No template placeholders
   - Clear summary of:
     - New runtime surface
     - Determinism guarantees
     - Coverage ratchet
     - Explicit non-goals (no distributed, no checkpointing, no learning)

2. Create GitHub release:
   - Title: `ADL v0.6.0`
   - Body from release notes
   - Verify links

3. Confirm release appears in:
   - GitHub releases
   - Repo tags

---

# Post-Release Actions

1. Close WP-J (#411) and milestone docs bootstrap issue (#416).
2. Close WP-J (#411).
3. Move any remaining `version:v0.6` items to `version:v0.7` with explanation.
4. Update roadmap documentation.
5. Begin Sprint 1 execution on v0.7 EPIC-A (#412).

---

# Rollback Plan

If a critical issue is discovered after tag but before announcement:

- Create hotfix branch from tag.
- Patch deterministically.
- Tag `v0.6.1`.
- Document delta clearly.

No force-push or tag rewrite allowed.

---

# Exit Criteria

v0.6 is considered successfully released when:

- Tag v0.6.0 exists and is immutable.
- GitHub release published.
- All WPs #401–#411 closed or deferred.
- Coverage gate satisfied.
- Determinism invariants verified.
- Documentation reflects actual shipped behavior.

v0.6 is a foundation release.  
Stability and clarity outweigh feature count.

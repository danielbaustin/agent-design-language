# WP-18 Refactor Behavior-Preservation Decision

Date: `2026-06-17`
Issue: `#3956`
Milestone: `v0.91.5`
Status: `bounded_evidence_recorded`

## Purpose

Record the WP-18 decision for the external-review refactor caveat without
claiming behavior equivalence beyond the tracked evidence.

## Decision

WP-18 has enough tracked evidence to keep the mini-sprint's bounded
behavior-preserving claims, but not enough evidence to claim full
pre-refactor/post-refactor behavior equivalence across the whole v0.91.5
refactor wave.

Therefore:

- keep the existing bounded claims for `#3747`, `#3748`, and `#3749`;
- do not add a new broad equivalence claim to WP-18 closeout truth;
- do not rerun expensive broad validation for this issue alone; and
- treat any stronger whole-wave behavior-equivalence claim as explicitly
  deferred unless a future issue creates a dedicated characterization-proof
  plan.

## Evidence Inventory

### Safety and proof baseline

- [REFACTOR_SAFETY_BASELINE_3593.md](../../REFACTOR_SAFETY_BASELINE_3593.md)
  established the trusted focused proof lanes for the refactor mini-sprint and
  explicitly did not claim that the full Rust suite had already been run.

### Approved refactor slices and proof posture

- [RUST_REFACTOR_SEMANTIC_AUDIT_3746.md](../../RUST_REFACTOR_SEMANTIC_AUDIT_3746.md)
  approved one semantic boundary per child issue and named the expected proof
  posture for each slice rather than treating "green enough" CI as sufficient
  by itself.

### Merged slice outcomes and recorded validation

- [RUST_REFACTOR_MINI_SPRINT_CLOSEOUT_3751.md](../../RUST_REFACTOR_MINI_SPRINT_CLOSEOUT_3751.md)
  records that `#3747`, `#3748`, and `#3749` landed as bounded semantic
  refactors with merged PR proof, plus the focused validation used for the
  sprint closeout:
  - `bash adl/tools/report_large_rust_modules.sh`
  - `bash adl/tools/report_module_navigability.sh --top 12 --format tsv`
  - sequencing checks against `SPRINT_v0.91.5.md`, `WBS_v0.91.5.md`, and
    `WP_ISSUE_WAVE_v0.91.5.yaml`

### External review caveat being resolved here

- [V0915_EXTERNAL_REVIEW_FINDINGS_2026-06-17.md](V0915_EXTERNAL_REVIEW_FINDINGS_2026-06-17.md)
  explicitly says the review did not run the test suite against both
  pre-refactor and post-refactor trees and therefore did not prove full
  behavior equivalence. It requires WP-18 to classify the proof as covered,
  deferred, or rerun.

## Covered Claims

The tracked evidence supports these narrower claims:

1. The refactor mini-sprint used issue-bounded semantic slices instead of a
   broad "split big files" pass.
2. Each landed child issue carried a focused proof posture appropriate to its
   slice, and the closeout packet records merged child outcomes plus focused
   validation evidence.
3. The sprint closeout is justified in saying the scoped production surfaces
   were refactored with bounded proof and that the remaining larger surfaces
   were left as explicit follow-ons or no-op cases.

## Not Covered

The tracked evidence does not support these stronger claims:

1. That the entire v0.91.5 refactor wave is behavior-equivalent in the broad
   pre-tree/post-tree sense.
2. That every affected command family or proof surface was re-executed under a
   dedicated final-preflight characterization pass after all refactor children
   merged.
3. That green CI alone is sufficient proof of whole-wave behavior preservation.

## Additional Validation Decision

No additional validation was run in `#3956`.

Rationale:

- this issue is scoped to review/preflight evidence truth, not new refactor
  implementation;
- the existing tracked packets already record the focused proof used for the
  landed slices; and
- rerunning a broad Rust suite now would widen scope while still not proving
  the stronger pre-tree/post-tree equivalence claim the external reviewer
  distinguished from ordinary green CI.

## Deferred Claim Record

Deferred item: broad whole-wave refactor behavior-equivalence proof

- Owner: WP-18 final preflight truth
- Disposition: deferred, not claimed
- Rationale: the current packets prove bounded slice behavior-preservation
  posture, but they do not establish a dedicated before/after characterization
  proof across the full refactor wave
- Target milestone if a stronger claim is later required: `v0.91.6`

## WP-18 Closeout Guidance

WP-18 may cite the refactor mini-sprint as bounded, reviewable, and supported
by focused validation evidence. WP-18 should not say that the full refactor was
behavior-equivalence proven unless a later issue adds and records that stronger
proof explicitly.

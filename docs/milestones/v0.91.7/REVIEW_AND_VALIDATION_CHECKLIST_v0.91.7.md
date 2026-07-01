# v0.91.7 Review And Validation Checklist

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-21`
- Setup lineage: `#3801`, `#3825`, `#4368`

## Status

Candidate review checklist for docs/planning and second-tranche bridge work.

## Required Local Validation

For planning-doc-only changes:

- `git diff --check`
- `PLANNING_SOURCE_CAPTURE_v0.91.7.md` exists and is linked from `README.md`
- `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md` exists and is linked from `README.md`
- required-file check for `README.md`, `WBS_v0.91.7.md`,
  `FEATURE_DOCS_v0.91.7.md`, `MILESTONE_CHECKLIST_v0.91.7.md`,
  `REVIEW_AND_VALIDATION_CHECKLIST_v0.91.7.md`, and
  `WP_ISSUE_WAVE_v0.91.7.yaml`
- YAML parse check for `WP_ISSUE_WAVE_v0.91.7.yaml`
- added-line scan for host-local paths, obvious secret markers, and local
  authoring-workspace links
- required-surface scan for Curiosity, Constructability, reasoning graph,
  loop runtime, `adl.skill.v1`, security, ACIP/A2A, protobuf, affect,
  happiness, Godel mechanics, economics, `#3780`, and `v0.92`

## Review Questions

- Does every source-capture row have an integrated/proven, already-closed-with-evidence, operator-approved non-claim, or evidence-backed blocker disposition?
- Does the change preserve the `#3778` bridge-ledger contract?
- Does it consume the `v0.91.6` first-tranche package without duplicating it?
- Does any doc claim `v0.92` readiness without evidence?
- Are second-tranche outputs concrete enough for C-SDLC issue execution?
- Are Curiosity and Constructability feature docs issue-ready?
- Is reasoning graph / `adl.skill.v1` scoped as a bridge rather than a full
  final standard?
- Are security and ACIP/A2A residuals fixed, explicitly non-claimed with operator approval, or blocked with evidence?
- Are SEP/VPP/PVF, goal/metrics, scheduler/provider, build/validation, runtime Soak #2, and Observatory/demo surfaces proven or explicitly blocked/non-claimed for v0.92?
- Does WP-01 consume failed-but-closed v0.91.6 WP-15 truth, closed final WP-16 truth, closed `#4620` / `#4621`, closed `#4622` PR-inventory proof, and closed WP-14A remediation truth before dependent v0.91.7 execution starts?
- Does the build/validation plan distinguish EC2 Spot or alternate remote-builder proof from an accepted release-critical validation lane?
- Are affect/happiness, Godel mechanics, and economics context bounded by
  safe-test and non-claim language?

## Finding Dispositions

Allowed dispositions:

- `fixed_in_scope`
- `operator_approved_non_claim_with_risk`
- `blocked_pending_operator_decision`

Disallowed dispositions:

- vague future work
- hidden runtime implementation inside planning docs
- closing by narrative without tracked evidence

## Closeout Truth

Closeout must record:

- what `v0.91.7` completed
- what `v0.91.7` explicitly non-claimed or blocked with evidence
- what `#3780` may consume for `v0.92`
- which validations ran locally
- which CI checks were relied on after PR publication

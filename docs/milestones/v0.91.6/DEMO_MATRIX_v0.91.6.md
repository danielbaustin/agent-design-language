# v0.91.6 Demo Matrix

## Status

current release-tail proof matrix with downstream runtime and demo residuals explicit

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Owner: ADL maintainers
- Related issue: `#3976`

## Purpose

Record the current proof surfaces for the first pre-`v0.92` bridge tranche.
`v0.91.6` does not claim birthday-demo completion, but it now has bounded
issue-local proof lanes that later milestone work may consume only with named
limits.

## Scope

In scope:

- docs existence and cross-link proof;
- bridge-surface classification;
- WP-local proof packets and routed open proof lanes;
- bounded WP-09 Observatory/Unity proof surfaces;
- birthday-visible non-claim boundaries for runtime/demo behavior.

Out of scope:

- birthday demo execution;
- final Unity/Observatory runtime rehearsal;
- provider benchmark reruns;
- public prompt export execution.

## Runtime Preconditions

Working directory:

```bash
git rev-parse --show-toplevel
```

No provider credentials or runtime services are required for the retained proof
rows listed here. Individual issue-owned implementation lanes may define their
own stronger proof requirements later.

## Related Docs

- Design contract: `DESIGN_v0.91.6.md`
- WBS: `WBS_v0.91.6.md`
- Sprint plan: `SPRINT_PLAN_v0.91.6.md`
- Checklist: `MILESTONE_CHECKLIST_v0.91.6.md`
- Feature index: `FEATURE_DOCS_v0.91.6.md`

## Demo Coverage Summary

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| D1 | Documentation package proof | `#3824` docs package exists and links truthfully | `find docs/milestones/v0.91.6 -maxdepth 2 -type f` | tracked docs | Expected planning and feature docs are present | deterministic filesystem check | current |
| D2 | Bridge overclaim scan | Docs do not claim runtime or `v0.92` readiness | `rg "ready|complete|shipped" docs/milestones/v0.91.6` | review notes | claims are bounded by non-goals and consumption rules | deterministic text review | current |
| D3 | Residual routing proof | `v0.91.7` residuals remain explicit | `rg "v0.91.7|#3825|residual" docs/milestones/v0.91.6` | index and feature docs | residual routes are visible | deterministic text review | current |
| D4 | Unity Observatory bounded closeout proof | WP-09 closeout packet and classification surfaces are refreshed to the current bounded closeout posture and justify umbrella closeout without overclaiming production readiness | `rg "#4030|#4031|#4032|#4033|#4034|#4035|#4341|#3974" docs/milestones/v0.91.6/review/observatory/WP09_WORKING_UNITY_OBSERVATORY_CLOSEOUT_4035.md docs/milestones/v0.91.6/features/OBSERVATORY_UNITY_CONSUMPTION_CLASSIFICATION_v0.91.6.md` | `docs/milestones/v0.91.6/review/observatory/WP09_WORKING_UNITY_OBSERVATORY_CLOSEOUT_4035.md` | closeout packet and classification surfaces preserve the closed-child/downstream-open split and retain explicit WP-09 ownership boundaries | deterministic doc-truth review | proved |
| D5 | Portable governed Observatory proof | portable reviewer-facing Observatory surface exists as a landed bounded proof surface while richer downstream runtime/release-tail convergence remains separate | `bash adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh` | `demos/v0.90.4/csm_observatory_governed_prototype.html` | governed Observatory prototype loads and remains available as a bounded reviewer-facing surface with the mobile-capable proof lane landed in `#4341` | deterministic local demo smoke | proved |
| D6 | Observatory security and consumption boundary proof | WP-09 may consume a bounded security and event-stream floor without pretending Unity/HTML implementation is closed | `rg "Unity Observatory|Observatory|reviewed_and_routed|consumption rule" docs/milestones/v0.91.6/review/security/UNITY_OBSERVATORY_INHABITANT_READINESS_SECURITY_REVIEW_4023.md docs/milestones/v0.91.6/review/logging_observability/OTEL_OBSERVATORY_CONSUMPTION_PROOF_3999.md` | WP-07 security review plus OTel Observatory consumption proof | Later consumers can cite reviewed redaction/security vocabulary and example-stream limits only | deterministic text review plus bounded proof packet inspection | current |
| D7 | Birthday-visible Observatory proof boundary | Birthday-visible proof surfaces are explicit: classification and retained proof exist, but broader runtime/release-tail convergence remains separate | `rg "birthday|v0.92|may not consume|operator/reviewer/mobile|residual" docs/milestones/v0.91.6/features/OBSERVATORY_UNITY_CONSUMPTION_CLASSIFICATION_v0.91.6.md docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md docs/milestones/v0.91.6/review/observatory/WP09_WORKING_UNITY_OBSERVATORY_CLOSEOUT_4035.md` | Observatory feature doc, retained closeout packet, and `v0.92` bridge ledger | Birthday planning can distinguish visible bounded proof from broader runtime and release-tail residual work without treating closed child issues as open | deterministic cross-doc review | current |
| D8 | HTML Observatory companion-lane proof | The portable HTML/mobile Observatory lane is named as a first-class WP-09 companion rather than a hidden prototype | `bash adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh` | `demos/v0.90.4/csm_observatory_governed_prototype.html` and closed issue `#4341` | Reviewers can see that HTML/mobile proof landed as a bounded companion surface and is not being counted as completed Unity proof | deterministic local demo smoke plus issue/doc routing review | proved |

## Coverage Rules

- Runnable demos are not required for every `v0.91.6` surface, but active
  implementation lanes must still leave named proof or routed-open rows.
- Each future implementation issue must define its own proof surface.
- Substitute proof is acceptable only when docs state the non-runtime boundary.
- Closed child proof does not by itself prove umbrella closure.
- A routed-open row is acceptable only when the owning issue is named and the
  matrix does not imply that the implementation is already complete.

## Known Limits

- This matrix proves documentation and retained proof posture, not broad
  runtime completion.
- Provider/model, ACIP, public export, and most Observatory/Unity implementation
  surfaces remain issue-owned work unless explicitly completed later.
- WP-09 child proof is closed, but this matrix still does not prove broader
  runtime integration, demo convergence, release-tail readiness, or birthday
  activation readiness.

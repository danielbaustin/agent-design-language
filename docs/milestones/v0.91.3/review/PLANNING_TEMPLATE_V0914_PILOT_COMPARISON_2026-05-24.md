# v0.91.4 Planning Template Pilot Comparison

Issue: #3312
Date: 2026-05-24
Status: rerun complete against stabilized template branch; generated drafts remain non-authoritative

## Scope

This pilot generated the 10 canonical milestone planning documents from the stabilized planning-template branch and compared those generated drafts against the existing v0.91.4 planning package.

Template-shape stabilization is split into #3315 / PR #3316. This pilot PR is stacked on that branch so #3312 remains a pilot/comparison issue rather than a hidden template-change issue.

The final generated drafts were first written to ignored scratch space and then copied into tracked review evidence for PR inspection:

- `docs/milestones/v0.91.3/review/planning_template_pilot_evidence/v0914_rerun_3/generated_full_packet/`
- `docs/milestones/v0.91.3/review/planning_template_pilot_evidence/v0914_rerun_3/full_packet_generation_summary.json`


No authoritative v0.91.4 planning documents were modified.

## Non-Claims

- This pilot does not approve generated planning docs for use as milestone truth.
- This pilot does not replace the existing v0.91.4 planning package.
- This pilot does not prove semantic correctness of generated prose.
- This pilot does not evaluate `feature_doc`; that template is present in the registry, but this run targeted the 10 canonical milestone packet documents.

## Template Fixes Applied Before Rerun

The first pilot proved that all 10 templates could generate and validate, but it also exposed avoidable shape drift against the real v0.91.4 package. The rerun fixed those issues before regenerating the packet.

Fixed surfaces:

- README now includes the real milestone-package sections: `Status`, `Milestone Role`, `Boundaries`, `Source Map`, a value-driven sidecar heading, and `Success Criteria`.
- WBS now includes `Status`, `Candidate WP Sequence`, a value-driven sidecar mini-sprint heading, and `Sequencing Notes`.
- SPRINT now includes `Status`, `Sprint Overview`, a value-driven sidecar mini-sprint heading, `Sprint Goals`, `Execution Policy`, and `Closeout Bar`.
- VISION no longer emits repeated top-level headings for numbered goal areas. Milestone-specific strategic headings are value-driven, so the v0.91.4 pilot can preserve the v0.91.4 section shape without baking those headings into the canonical template.
- DESIGN now uses `Interfaces And Contracts`, matching the existing milestone doc.
- DEMO_MATRIX now includes a `Status` surface and cleaner demo-detail headings.
- RELEASE_PLAN now uses the same numbered heading style as the existing release plan.
- All generated top-level titles now use the milestone name instead of generic `Template` titles.
- The planning registry required-section lists were updated to match the fixed template shapes.

## Generated Packet

All 10 canonical milestone planning templates generated and passed structural validation after the fixes:

| Template | Generated draft | Structural validation |
| --- | --- | --- |
| `readme` | `docs/milestones/v0.91.3/review/planning_template_pilot_evidence/v0914_rerun_3/generated_full_packet/README_GENERATED.md` | passed |
| `wbs` | `docs/milestones/v0.91.3/review/planning_template_pilot_evidence/v0914_rerun_3/generated_full_packet/WBS_GENERATED.md` | passed |
| `sprint` | `docs/milestones/v0.91.3/review/planning_template_pilot_evidence/v0914_rerun_3/generated_full_packet/SPRINT_GENERATED.md` | passed |
| `vision` | `docs/milestones/v0.91.3/review/planning_template_pilot_evidence/v0914_rerun_3/generated_full_packet/VISION_GENERATED.md` | passed |
| `design` | `docs/milestones/v0.91.3/review/planning_template_pilot_evidence/v0914_rerun_3/generated_full_packet/DESIGN_GENERATED.md` | passed |
| `decisions` | `docs/milestones/v0.91.3/review/planning_template_pilot_evidence/v0914_rerun_3/generated_full_packet/DECISIONS_GENERATED.md` | passed |
| `demo_matrix` | `docs/milestones/v0.91.3/review/planning_template_pilot_evidence/v0914_rerun_3/generated_full_packet/DEMO_MATRIX_GENERATED.md` | passed |
| `milestone_checklist` | `docs/milestones/v0.91.3/review/planning_template_pilot_evidence/v0914_rerun_3/generated_full_packet/MILESTONE_CHECKLIST_GENERATED.md` | passed |
| `release_plan` | `docs/milestones/v0.91.3/review/planning_template_pilot_evidence/v0914_rerun_3/generated_full_packet/RELEASE_PLAN_GENERATED.md` | passed |
| `release_notes` | `docs/milestones/v0.91.3/review/planning_template_pilot_evidence/v0914_rerun_3/generated_full_packet/RELEASE_NOTES_GENERATED.md` | passed |

## Findings

### P1: Fixed - the 10-doc template packet now matches the existing v0.91.4 section surface

After the template-shape fixes, every generated canonical milestone document shares all headings with its corresponding existing v0.91.4 document.

This is the main pilot result: the template system is now able to produce a complete structurally aligned 10-doc milestone packet without modifying authoritative milestone docs.


### P2: Fixed - milestone-specific headings are value-driven, not hard-coded canonical requirements

The final fix keeps the canonical templates reusable. VISION strategic headings and sidecar headings are supplied by values for this pilot rather than required as universal registry sections. This lets the v0.91.4 generated packet align with the existing v0.91.4 section surface without making future milestones inherit v0.91.4-specific names by construction.

### P2: Remaining semantic review is still required before generated docs become milestone truth

Structural alignment is not the same as semantic correctness. Generated values can still be generic, repetitive, or weaker than hand-authored milestone content.

This is expected and appropriate for the current tool boundary. Template filling proves scaffold completeness and placeholder resolution. A planning-doc editor or human review must still decide whether generated text is true enough to become authoritative milestone documentation.

### P2: VISION and DEMO_MATRIX are intentionally richer than some current v0.91.4 docs

The generated VISION and DEMO_MATRIX now preserve the existing v0.91.4 heading surface for this pilot, but their templates are richer than the compact current docs in some places.

That is not a blocker. It means these templates are useful for future milestone setup and for expanding proof surfaces, but regenerating an existing mature milestone package still requires editorial judgment.


### P2: Existing v0.91.4 docs outside the 10-doc template packet need explicit routing

The source issue required the pilot to identify existing v0.91.4 docs that do not map cleanly to the current template family. The 10 generated milestone-packet docs map to the 10 canonical milestone templates, while these existing v0.91.4 docs remain outside the exercised packet:

| Existing v0.91.4 document | Current template mapping | Classification | Recommended routing |
| --- | --- | --- | --- |
| `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md` | No direct 10-doc template counterpart | Unmapped quality/proof gate surface | Keep as milestone-specific proof doc or add a future `quality_gate` template if repeated across milestones |
| `docs/milestones/v0.91.4/NEXT_MILESTONE_HANDOFF_v0.91.4.md` | No direct 10-doc template counterpart | Unmapped handoff/transition surface | Keep as milestone-specific handoff doc or add a future `next_milestone_handoff` template if repeated across milestones |
| `docs/milestones/v0.91.4/FEATURE_PROOF_COVERAGE_v0.91.4.md` | Adjacent to `feature_doc`, but not exercised here | Feature/proof-specific surface outside 10-doc milestone packet | Use a separate feature-doc pilot before claiming coverage |
| `docs/milestones/v0.91.4/WP_ISSUE_WAVE_v0.91.4.yaml` | No Markdown planning-template counterpart | Operational issue-wave data, not a planning doc template | Keep as structured milestone execution data, not a generated Markdown planning doc |

This means the 10-doc packet is structurally complete for canonical milestone planning, but the full v0.91.4 planning ecosystem still has milestone-specific proof, handoff, feature-coverage, and issue-wave surfaces that should not be silently forced into the 10-doc template family.

### P3: The `feature_doc` template remains available but unproven by this 10-doc pilot

The registry includes `feature_doc`, so the available planning-template surface contains 11 templates. This pilot intentionally exercised the 10 canonical milestone packet docs only.

Recommendation: run a separate feature-doc pilot against one real feature planning surface before treating the 11th template as operationally proven.

## Final Comparison Summary

| Document | Existing lines | Generated lines | Shared headings | Remaining missing existing headings |
| --- | ---: | ---: | ---: | --- |
| README | 174 | 148 | 9/9 | none |
| WBS | 78 | 104 | 6/6 | none |
| SPRINT | 67 | 110 | 7/7 | none |
| VISION | 96 | 261 | 11/11 | none |
| DESIGN | 146 | 85 | 20/20 | none |
| DECISIONS | 39 | 41 | 6/6 | none |
| DEMO_MATRIX | 22 | 270 | 2/2 | none |
| MILESTONE_CHECKLIST | 80 | 62 | 9/9 | none |
| RELEASE_PLAN | 85 | 82 | 10/10 | none |
| RELEASE_NOTES | 95 | 68 | 15/15 | none |

## What Passed

- The active planning registry exposes the 10 canonical milestone packet templates.
- The registry also exposes the separate `feature_doc` template.
- All 10 canonical milestone documents generated from explicit JSON values.
- All 10 generated documents passed `validate_planning_template.py` structural validation.
- All 10 generated documents now preserve the existing v0.91.4 heading surface.
- Generated drafts are preserved in tracked review evidence; ignored scratch is not required for PR review.
- Existing v0.91.4 milestone docs were not modified.

## Recommended Next Steps

1. Keep the current template set as the canonical substrate for draft generation.
2. Add or document a milestone-package values model so values can be shared coherently across all 10 docs.
3. Add planning-doc editor guidance for converting generated drafts into authoritative milestone docs.
4. Decide whether repeated quality-gate and next-milestone handoff docs should become first-class templates.
5. Run a separate `feature_doc` pilot against one feature-level planning surface.
6. Decide whether future milestones should use the richer demo matrix template from the start rather than compact status-only demo matrices.

## Bottom Line

The planning-template system is now ready to produce complete, structurally aligned first-draft milestone packets. The next improvement is not basic template coverage; it is semantic normalization through the planning-doc editor and better milestone-package values.

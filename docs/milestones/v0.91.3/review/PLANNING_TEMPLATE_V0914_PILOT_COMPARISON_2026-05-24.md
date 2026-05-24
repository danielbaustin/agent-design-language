# v0.91.4 Planning Template Pilot Comparison

Issue: #3312
Date: 2026-05-24
Status: pilot complete; generated drafts are non-authoritative

## Scope

This pilot generated the 10 canonical milestone planning documents from the active planning-template registry and compared those generated drafts against the existing v0.91.4 planning package.

The generated drafts were written only to ignored scratch space:

- `.adl/runs/planning-template-pilot/v0.91.4/generated_full_packet/`
- `.adl/runs/planning-template-pilot/v0.91.4/values_full_packet/`
- `.adl/runs/planning-template-pilot/v0.91.4/full_packet_generation_summary.json`

No authoritative v0.91.4 planning documents were modified.

## Non-Claims

- This pilot does not approve generated planning docs for use as milestone truth.
- This pilot does not replace the existing v0.91.4 planning package.
- This pilot does not prove semantic correctness of generated text.
- This pilot does not evaluate `feature_doc`; that template is present in the registry, but this run targeted the 10 canonical milestone packet documents.

## Generated Packet

All 10 canonical milestone planning templates generated and passed structural validation:

| Template | Generated draft | Structural validation |
| --- | --- | --- |
| `readme` | `.adl/runs/planning-template-pilot/v0.91.4/generated_full_packet/README_GENERATED.md` | passed |
| `wbs` | `.adl/runs/planning-template-pilot/v0.91.4/generated_full_packet/WBS_GENERATED.md` | passed |
| `sprint` | `.adl/runs/planning-template-pilot/v0.91.4/generated_full_packet/SPRINT_GENERATED.md` | passed |
| `vision` | `.adl/runs/planning-template-pilot/v0.91.4/generated_full_packet/VISION_GENERATED.md` | passed |
| `design` | `.adl/runs/planning-template-pilot/v0.91.4/generated_full_packet/DESIGN_GENERATED.md` | passed |
| `decisions` | `.adl/runs/planning-template-pilot/v0.91.4/generated_full_packet/DECISIONS_GENERATED.md` | passed |
| `demo_matrix` | `.adl/runs/planning-template-pilot/v0.91.4/generated_full_packet/DEMO_MATRIX_GENERATED.md` | passed |
| `milestone_checklist` | `.adl/runs/planning-template-pilot/v0.91.4/generated_full_packet/MILESTONE_CHECKLIST_GENERATED.md` | passed |
| `release_plan` | `.adl/runs/planning-template-pilot/v0.91.4/generated_full_packet/RELEASE_PLAN_GENERATED.md` | passed |
| `release_notes` | `.adl/runs/planning-template-pilot/v0.91.4/generated_full_packet/RELEASE_NOTES_GENERATED.md` | passed |

## Findings

### P1: The 10-doc template packet is structurally complete, but generated output is not yet semantically safe to substitute for existing v0.91.4 docs

The active registry contains the expected 10 milestone packet templates plus the separate `feature_doc` template. The generator can fill all 10 canonical milestone docs and the validator accepts each generated draft.

That shows the template substrate is functional for first-draft generation. It does not prove that generated docs can replace a mature milestone package. The existing v0.91.4 documents contain milestone-specific planning semantics around C-SDLC routing, transition operation, durable trace/memory, proof latency, and CodeFriend sidecar work that generic values do not recover automatically.

Recommendation: use generated packets as draft scaffolds, then route them through planning-doc editor review before they become milestone truth.

### P2: README, WBS, and SPRINT lose important milestone-specific structure

The existing README is organized around `Status`, `Purpose`, `Milestone Role`, `Boundaries`, `Source Map`, `Document Map`, `CodeFriend Sidecar`, and `Success Criteria`. The generated README is structurally broader, with `Metadata`, `How To Use`, `Overview`, `Scope Summary`, `Execution Model`, `Demo and Validation Surface`, and related generic sections.

The WBS and SPRINT templates generate valid scaffolds, but the existing v0.91.4 WBS and sprint plan encode a 21-WP sequence, CodeFriend sidecar mini-sprint, sequencing notes, execution policy, and closeout bar. Those are not inferred by the template fill process.

Recommendation: keep these templates, but add a milestone-package values model that can represent sidecar work, ordered WP ranges, closeout policy, and special milestone themes explicitly.

### P2: VISION and DEMO_MATRIX reveal template/value-model pressure

The generated VISION draft is structurally valid but much longer than the existing v0.91.4 vision and repeats generic goal text across multiple placeholder slots. That is a values-model problem, not a registry problem: the template asks for detailed semantic content that a generic pilot values file cannot supply.

The generated DEMO_MATRIX is also structurally valid, but the existing v0.91.4 demo matrix is currently a compact status surface while the template expects a detailed proof matrix. This may be an improvement target, but it should be handled deliberately rather than treated as a no-op regeneration.

Recommendation: before using generated VISION or DEMO_MATRIX drafts authoritatively, require a human/editor pass that checks repetition, proof specificity, and milestone fit.

### P2: Several templates are close structural matches and look usable for normal draft scaffolding

`design`, `decisions`, `milestone_checklist`, `release_plan`, and `release_notes` align closely with the corresponding existing v0.91.4 documents at the section level.

The generated versions still need semantic review, but the template shapes are credible. These are good candidates for early operational use in future milestone setup.

### P3: The feature-doc template is present but was not exercised in this 10-doc packet pilot

The registry includes `feature_doc`, bringing the available planning-template surface to 11 templates. This pilot generated the 10 canonical milestone planning documents only, matching the milestone packet rather than a feature-specific planning doc.

Recommendation: run a follow-on feature-doc pilot against one v0.91.4 feature surface before treating the 11th template as operationally proven.

## Comparison Summary

| Document | Existing doc | Generated doc | Result |
| --- | --- | --- | --- |
| README | 174 lines, 9 headings | 143 lines, 15 headings | Structurally valid, semantic gap |
| WBS | 78 lines, 6 headings | 78 lines, 8 headings | Structurally valid, needs richer WP values |
| SPRINT | 67 lines, 7 headings | 60 lines, 10 headings | Structurally valid, loses sidecar/execution-policy detail |
| VISION | 96 lines, 11 headings | 253 lines, 16 headings | Structurally valid, values are too generic/repetitive |
| DESIGN | 146 lines, 20 headings | 85 lines, 20 headings | Good structural match, needs semantic review |
| DECISIONS | 39 lines, 6 headings | 41 lines, 7 headings | Good structural match |
| DEMO_MATRIX | 22 lines, 2 headings | 266 lines, 18 headings | Template is richer than current doc; requires deliberate migration |
| MILESTONE_CHECKLIST | 80 lines, 9 headings | 62 lines, 9 headings | Good structural match |
| RELEASE_PLAN | 85 lines, 10 headings | 82 lines, 10 headings | Good structural match |
| RELEASE_NOTES | 95 lines, 15 headings | 68 lines, 15 headings | Good structural match, needs semantic review |

## What Passed

- The active planning registry exposes the 10 canonical milestone packet templates.
- The registry also exposes the separate `feature_doc` template.
- All 10 canonical milestone documents generated from explicit JSON values.
- All 10 generated documents passed `validate_planning_template.py` structural validation.
- The pilot kept generated drafts in ignored scratch space.
- The pilot did not modify existing v0.91.4 milestone docs.

## Recommended Next Steps

1. Keep the current template set as the canonical substrate for draft generation.
2. Add or document a milestone-package values model so values can be shared coherently across all 10 docs.
3. Add planning-doc editor guidance for converting generated drafts into authoritative milestone docs.
4. Run a separate `feature_doc` pilot against one feature-level planning surface.
5. Decide whether the v0.91.4 demo matrix should stay compact or migrate to the richer template shape.

## Bottom Line

The planning-template system can produce complete first-draft milestone packets. It is not yet ready to regenerate a complex existing milestone package without editor review. That is a good outcome for this stage: the substrate works, and the next improvement is semantic normalization rather than basic template coverage.

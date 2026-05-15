# Review Heuristics Promotion Packet

## Purpose

Promote the ADL review heuristics from the planning note in
`.adl/docs/TBD/workflow_tooling/ADL_REVIEW_HEURISTICS.md` into the active
`v0.91.2` review-skill references and demo surface without claiming that
heuristics alone guarantee review quality.

## Inputs

- `.adl/docs/TBD/workflow_tooling/ADL_REVIEW_HEURISTICS.md`
- `adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md`
- `adl/tools/skills/review-quality-evaluator/SKILL.md`
- `adl/tools/skills/repo-review-synthesis/SKILL.md`
- `docs/milestones/v0.91.2/review/codefriend_productization/review_packet_workflow_package.md`

## Promotion Map

| Heuristic Surface | Active Skill/Demo Surface | Promotion Rule |
| --- | --- | --- |
| Functional correctness, security, testing, docs, architecture, dependency review domains | `adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md` | Treat the heuristics domains as the bounded reasoning taxonomy behind the specialist lanes; do not treat the suite doc as proof that every rule is automated. |
| Evidence-first findings, severity/confidence discipline, non-claim posture | `adl/tools/skills/review-quality-evaluator/SKILL.md` | Require review packets and reports to show evidence, justified severity, caveats, and unsupported-claim rejection before customer-facing use. |
| Dedupe, disagreement handling, coverage matrix, residual risk | `adl/tools/skills/repo-review-synthesis/SKILL.md` | Preserve role-specific context and missing-coverage truth instead of flattening the review into one unstructured verdict. |
| Review packet workflow and product-report alignment | `docs/milestones/v0.91.2/review/codefriend_productization/` | Reuse the packet workflow proved by `WP-06`; `WP-07` adds the heuristic and demo layer on top of that packet flow. |

## Heuristics That Matter For WP-07

The promoted review surface must preserve these behaviors from the source
heuristics note:

- findings must cite source evidence, not intuition
- severity must be justified by impact, not tone
- role-specific review coverage must stay visible
- contradictions and disagreements must be surfaced explicitly
- review output must remain structured enough for later synthesis and quality
  gating
- non-reviewed surfaces and residual risks must remain visible

## Bounded Non-Claims

- This packet does not claim that the heuristics are fully machine-executable.
- This packet does not claim that every review skill implements every rule from
  the source note directly.
- This packet does not claim that review quality can be guaranteed by one
  checklist or one skill.
- This packet does not replace bounded specialist review, synthesis, or human
  judgment.

# WP-13 Publication And Paper Boundary (#4757)

## Scope

Issue `#4757` records the publication and paper delivery boundary that closes
the WP-13 publication lane without publishing papers, launch copy, customer
reports, or CodeFriend product materials.

This packet is the retained evidence that paper/publication work is not a
`v0.92` birthday blocker unless a later tracked issue explicitly promotes a
bounded publication slice with proof and human approval.

## Implemented Documentation Surface

- Retained boundary packet:
  `docs/milestones/v0.91.7/review/wp13_publication_boundary_4757.md`
- Machine-readable evidence packet:
  `docs/milestones/v0.91.7/review/wp13_publication_boundary_4757/boundary_packet.json`
- Consuming milestone surfaces:
  - `docs/milestones/v0.91.7/FEATURE_DOCS_v0.91.7.md`
  - `docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md`
  - `docs/milestones/v0.91.7/MILESTONE_CHECKLIST_v0.91.7.md`
  - `docs/milestones/v0.91.7/RELEASE_NOTES_v0.91.7.md`
  - `docs/milestones/v0.91.7/RELEASE_PLAN_v0.91.7.md`
  - `docs/milestones/v0.91.7/WBS_v0.91.7.md`
  - `docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml`

## Source Evidence

| Source | Publication relevance |
| --- | --- |
| `docs/milestones/v0.91.7/review/wp13_affect_happiness_boundary_4752.md` | Public affect/happiness language must stay operational and must not imply subjective feeling, happiness, wellbeing, suffering, or consciousness. |
| `docs/milestones/v0.91.7/review/wp13_godel_constructability_boundary_4753.md` | Godel birthday/public claims require retained runtime evidence, constructability anchors, and operator review; live hosted invocation and adaptive DAG completion remain non-claims. |
| `docs/milestones/v0.91.7/review/wp13_economics_civilization_boundary_4754.md` | Economics/civilization remains context-only for `v0.92`; payments, settlement, market mechanisms, autonomous economy, and civilization runtime are non-claims. |
| `docs/milestones/v0.91.7/review/wp13_guild_foundation_boundary_4755.md` | Guild language is governance handoff context only; constitutional citizenship, polis authority, delegated authority, and public product readiness remain non-claims. |
| `docs/milestones/v0.91.7/review/wp13_codefriend_adapter_obligations_4756.md` | CodeFriend v1 and adapter v2 are tracked handoff truth only before `v0.92`; external-repo proof packaging and product/publication readiness remain future obligations. |
| `docs/planning/codefriend/CODEFRIEND_V1_BUILD_PLAN.md` | CodeFriend v1 publication requires human review, redaction, non-claim checks, publication manifest, and retained product-release evidence before customer-facing export. |

## Decision

Paper and publication delivery surfaces are scoped out of `v0.92` birthday
activation for this milestone by the tracked #4757 boundary issue. This is not
external publication approval; it is an issue-bound scope decision that keeps
papers, launch copy, reports, and product publication claims behind the
promotion gates below.

`v0.92` may consume:

- the WP-13 source-backed non-claim ledger in this packet;
- launch and birthday copy guardrails that cite the WP-13 proof packets;
- CodeFriend/publication handoff truth as future product planning context;
- a requirement that any public artifact pass human review, redaction, and
  evidence-boundary checks before external publication.

`v0.92` may not consume:

- a published paper;
- customer-facing CodeFriend or product report readiness;
- public launch approval;
- autonomous review authority;
- subjective affect or consciousness claims;
- unbounded recursive self-improvement or live hosted Godel invocation claims;
- payments, market, civilization, or governance authority claims;
- v0.93 governance or v0.95 CodeFriend MVP completion claims.

## Public Claim Rule

Any `v0.92`, launch, birthday, release-note, report, paper, website, or demo
copy that mentions WP-13 surfaces must include nearby evidence boundaries:

- Cite the owning proof packet for the surface.
- Use only the allowed `v0.92` consumption language in that packet.
- Preserve the non-claims from this packet.
- Treat publication approval as a separate human decision, not an automatic
  consequence of issue or PR closure.

## Required Promotion Gates

A future paper or publication issue may promote a bounded slice only after it
provides:

- tracked issue scope and lifecycle cards;
- source-backed manuscript, report, launch-copy, or release-copy target;
- evidence inventory with all cited proof packets;
- redaction and privacy review;
- public-claim/non-claim checklist;
- human approval record for external publication;
- retained final artifact and validation record.

## Residual Risk

This packet does not prove any external publication artifact. It prevents
publication readiness from being inferred from adjacent WP-13 implementation
truth. That is intentional scope control for `v0.92`; publication remains a
reviewed follow-on unless a later tracked issue promotes a specific artifact.

## Validation Plan

Focused local validation for this packet:

```sh
git diff --check
ruby -e 'require "json"; JSON.parse(File.read("docs/milestones/v0.91.7/review/wp13_publication_boundary_4757/boundary_packet.json")); puts "json ok"'
rg -n "paper/publication|Paper and publication surfaces|wp13_publication_boundary_4757" docs/milestones/v0.91.7/FEATURE_DOCS_v0.91.7.md docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md docs/milestones/v0.91.7/MILESTONE_CHECKLIST_v0.91.7.md docs/milestones/v0.91.7/RELEASE_NOTES_v0.91.7.md docs/milestones/v0.91.7/RELEASE_PLAN_v0.91.7.md
ruby -e 'require "yaml"; YAML.load_file("docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml"); puts "yaml ok"'
rg -n "4752|4753|4754|4755|4756|4757|publication" docs/milestones/v0.91.7/WBS_v0.91.7.md docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml
```

The SOR for `#4757` records the exact commands run and their results.

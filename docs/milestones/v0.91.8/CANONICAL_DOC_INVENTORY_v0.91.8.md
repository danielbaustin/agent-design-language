# v0.91.8 Canonical Document Inventory

Status: planned audit surface. This file validates that the v0.91.8
next-milestone documentation packet is present, internally linked, and bounded
to planned-vs-proven truth before WP-21A can close.

WP-21A fails closed if any required canonical document or feature surface is
missing, contradicts the `version:v0.91.8` routing authority, omits WP-10A,
misstates dependencies, treats planned work as proven, reverses v0.91.8 bridge
precedence, or leaves current blocker/register truth stale.

## Current Superseding Truth

- v0.91.8 is the bridge prerequisite for v0.92. v0.92 remains downstream and
  must consume exact v0.91.8 revisions, non-claims, and blockers rather than
  bypassing this bridge.
- GitHub milestone configuration is not the operative routing authority for
  this packet. The `version:v0.91.8` label plus
  [WP_ISSUE_WAVE_v0.91.8.yaml](WP_ISSUE_WAVE_v0.91.8.yaml) are the current
  routing authority.
- WP-10A is canonical v0.91.8 scope through umbrella `#5497` and children
  `#5499`, `#5498`, `#5500`, `#5502`, and `#5501`.
- #5408 is no longer a current blocker. PR #5419 merged at
  `6fcd3accafc15e3b6cc8064d836293b4495983de`; #5408 is closed with typed
  terminal generation 216, phase `closed_out`, reviewed head `05ba1f2b`, and
  merged publication state. Any closeout receipt identifier is observed
  lifecycle metadata, not a tracked canonical-document path in this packet.
- The retained #4906 `blocked_with_evidence` gate remains unresolved unless a
  separate issue explicitly dispositions it.

## Canonical Planning And Release Entrypoints

| Surface | Path | Required routing/dependency truth | Planned/proven state | WP-21A validation |
| --- | --- | --- | --- | --- |
| README | [README.md](README.md) | Names #5383 closed setup authority, #5384 WP-14A, #4641 restored v0.91.7 WP-14, and v0.92 downstream bridge. | Planned only | Must link this inventory and not claim release readiness. |
| Vision | [VISION_v0.91.8.md](VISION_v0.91.8.md) | Keeps ADL v2, Runtime v3, and C-SDLC v2 ownership separate. | Planned only | Must preserve bridge-before-v0.92 precedence. |
| Design | [DESIGN_v0.91.8.md](DESIGN_v0.91.8.md) | Three-product acceptance boundary and deletion budget remain non-approval. | Planned only | Must not pre-approve deletion or deployment. |
| Decisions | [DECISIONS_v0.91.8.md](DECISIONS_v0.91.8.md) | #5383 is historical/closed setup authority; #5335 is stale predecessor; #5384 owns WP-14A. | Planned/current routing decisions | Must not call #5383 active. |
| Work breakdown | [WBS_v0.91.8.md](WBS_v0.91.8.md) | Includes WP-10A and makes WP-11 depend on WP-10 plus completed WP-10A live proof. | Planned only | Must match issue-wave dependencies. |
| Sprint plan | [SPRINT_PLAN_v0.91.8.md](SPRINT_PLAN_v0.91.8.md) | Serial WP-04 -> WP-05 -> WP-06 -> WP-07; WP-10 and WP-10A fan out after WP-09. | Planned only | Must link the parallel plan. |
| Sprint alias | [SPRINT_v0.91.8.md](SPRINT_v0.91.8.md) | Points to the active sprint plan. | Planned only | Link target must exist. |
| Parallel execution | [PARALLEL_EXECUTION_PLAN_v0.91.8.md](PARALLEL_EXECUTION_PLAN_v0.91.8.md) | Defines Wave 0 card factory, global writable WIP cap, WP-10A order, acceptance-gated deletion, and serialized gates. | Planned only | Must include #5343/#5347 readiness projections and no implementation claims. |
| Issue wave | [WP_ISSUE_WAVE_v0.91.8.yaml](WP_ISSUE_WAVE_v0.91.8.yaml) | `version:v0.91.8` routing authority, WP-10A map, sidecar children, readiness controls, and WP dependencies. | Planned only | YAML must parse and targeted assertions must pass. |
| Execution readiness | [WP_EXECUTION_READINESS_v0.91.8.md](WP_EXECUTION_READINESS_v0.91.8.md) | #5383 closed; WP-10A sequence; WP-13 waits on #5358/#5361 acceptance. | Planned only | Must not mark waiting WPs ready without proof. |
| Feature proof coverage | [FEATURE_PROOF_COVERAGE_v0.91.8.md](FEATURE_PROOF_COVERAGE_v0.91.8.md) | Maps feature areas to owning issues including WP-10A. | Planned only | Every feature doc surface must have an issue/proof row. |
| Quality gate | [QUALITY_GATE_v0.91.8.md](QUALITY_GATE_v0.91.8.md) | Gates route through existing issues and do not pass from planning text alone. | Planned only | Must not imply this packet satisfies a gate. |
| Demo matrix | [DEMO_MATRIX_v0.91.8.md](DEMO_MATRIX_v0.91.8.md) | Names planned demos and proof boundaries. | Planned only | Must preserve Unity/blocker and public-claim limits. |
| Milestone checklist | [MILESTONE_CHECKLIST_v0.91.8.md](MILESTONE_CHECKLIST_v0.91.8.md) | Requires exact proof before acceptance, deletion, release, or v0.92 handoff. | Planned checklist | Only closed setup/routing rows may be checked; implementation/review/release rows remain unchecked until proven. |
| Review index | [review/README.md](review/README.md) | Names planned formal review surfaces and distinguishes them from per-issue shadow reviews. | Planned only | Must not claim review performed. |
| Third-party review handoff | [review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md](review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md) | Formal review owner is WP-19 / #5357; WP-20 through WP-23 remain downstream. | Prepared not sent | Must require exact revision and landed implementation/proof manifest before send. |
| ADR plan | [ADR_PLAN_v0.91.8.md](ADR_PLAN_v0.91.8.md) | ADRs are required only where owning issues make architectural claims. | Planned only | Must not accept ADRs from planning text. |
| Release plan | [RELEASE_PLAN_v0.91.8.md](RELEASE_PLAN_v0.91.8.md) | Release requires merged implementation, review, remediation, and closeout. | Planned only | Must not claim release approval. |
| Release notes draft | [RELEASE_NOTES_v0.91.8.md](RELEASE_NOTES_v0.91.8.md) | Placeholder notes must be rewritten from merged evidence before release. | Draft placeholders | Must remain explicitly non-final. |
| v0.92 handoff | [NEXT_MILESTONE_HANDOFF_v0.91.8.md](NEXT_MILESTONE_HANDOFF_v0.91.8.md) | v0.92 consumes exact v0.91.8 revisions and non-claims. | Planned only | Must not claim birthday readiness. |
| v0.92 activation map | [V092_ACTIVATION_TEST_MAP_v0.91.8.md](V092_ACTIVATION_TEST_MAP_v0.91.8.md) | Missing evidence becomes blocker or non-claim for v0.92. | Planned only | Must preserve bridge precedence. |

## Feature Documents

| Feature surface | Path | Owning issues | Dependency/status truth | WP-21A validation |
| --- | --- | --- | --- | --- |
| Feature index | [features/README.md](features/README.md) | Index only | Lists every feature doc in this directory. | Must include all existing feature docs. |
| ADL v2 core | [features/ADL_V2_CORE_v0.91.8.md](features/ADL_V2_CORE_v0.91.8.md) | #5336, #5337, #5338, #5339, #5340, #5342, #5345, #5350 | ADL owns language/compiler/records/CLI only. | Must exclude Runtime v3 and C-SDLC ownership. |
| Runtime v3 adapter | [features/RUNTIME_V3_ADAPTER_v0.91.8.md](features/RUNTIME_V3_ADAPTER_v0.91.8.md) | #5341, #5361, #5349, #5501 | Runtime v3 remains execution authority; #5361 closure consumes #5501 live workcell proof. | Must require exact revision and operational proof. |
| C-SDLC v2 acceptance | [features/CSDLC_V2_ACCEPTANCE_v0.91.8.md](features/CSDLC_V2_ACCEPTANCE_v0.91.8.md) | #5358, #5540, #5541, #5558 | Typed v2 lifecycle authority only; #5540 is closed, #5541/#5558 remain inventory. | Must require exact-revision lifecycle proof and defect disposition. |
| Platform acceptance and deployment | [features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md](features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md) | #5384 plus WP-14A children | Child topology must close or remain operator-approved blocked. | Must preserve #4906 as unresolved retained gate. |
| Deletion and cutover | [features/DELETION_AND_CUTOVER_v0.91.8.md](features/DELETION_AND_CUTOVER_v0.91.8.md) | #5344, #5343, #5346, #5347 | Soak/rollback/selector precede disjoint deletion. | Must not approve deletion from planning text. |
| v0.92 handoff feature | [features/V092_HANDOFF_v0.91.8.md](features/V092_HANDOFF_v0.91.8.md) | #5352, #5362, #5355, #5359 | Exact-revision consumption packet only. | Must not claim birthday implementation. |

## Validation Matrix

| Check | Command or evidence | Required result |
| --- | --- | --- |
| Required docs exist | `test -f` over every path listed above | All listed canonical and feature docs exist. |
| README links inventory | `rg "CANONICAL_DOC_INVENTORY_v0.91.8.md" docs/milestones/v0.91.8/README.md` | README links this matrix. |
| README links parallel plan | `rg "PARALLEL_EXECUTION_PLAN_v0.91.8.md" docs/milestones/v0.91.8/README.md` | README links the planned parallel execution overlay. |
| Review handoff linked | `rg "THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md" docs/milestones/v0.91.8/README.md docs/milestones/v0.91.8/review/README.md docs/milestones/v0.91.8/CANONICAL_DOC_INVENTORY_v0.91.8.md` | Handoff is reachable from canonical entrypoints. |
| WP-21A closeout packet present | `test -f docs/milestones/v0.91.7/review/V0917_WP21A_NEXT_MILESTONE_DOCS_CLOSEOUT_5489.md && test -f docs/milestones/v0.91.7/review/wp21a_next_milestone_docs_5489/README.md` | Current #5489 closeout/review packet surfaces exist. |
| Review packet digest procedure | `docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md` Digest Procedure | Send-time reviewer can reproduce the sorted tracked mode/type/hash/path object-record digest plus normalized synthetic handoff record while excluding untracked/local artifacts and avoiding self-inclusion of the digest value. |
| YAML parses | Ruby `YAML.safe_load(..., aliases: true)` | `WP_ISSUE_WAVE_v0.91.8.yaml` parses. |
| Routing assertions | Ruby assertions over `wp_issue_map`, `work_packages`, and `parallel_execution` | WP-10A, WP-11, WP-13, WIP cap, #5343/#5347, and routing authority are consistent. |
| Stale setup wording | `! rg 'active setup issue|Use \`#5383\` as the active' docs/milestones/v0.91.8/README.md docs/milestones/v0.91.8/DECISIONS_v0.91.8.md docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md docs/milestones/v0.91.8/setup/5383/DESIGN.md` | No stale active-setup wording remains in current v0.91.8 authority entrypoints. |
| #5408 current blocker cleanup | `! rg 'Resolve #5408' docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md docs/milestones/v0.91.7/FEATURE_DOCS_v0.91.7.md docs/milestones/v0.91.7/SPRINT_PLAN_v0.91.7.md docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml` | No current register or entrypoint action still routes to #5408. |
| #4906 preserved | `rg "#4906" docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md docs/milestones/v0.91.8/CANONICAL_DOC_INVENTORY_v0.91.8.md` | #4906 remains visible as retained blocked-with-evidence truth. |

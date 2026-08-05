# WP-17 Documentation Update Inventory

Issue: `#5360`
Purpose: define the complete, bounded documentation surface that WP-17 must
reconcile before formal review.
Status: preparation inventory only; this file does not authorize edits to the
shared documentation paths below.

## Authority And Prior-Issue Lessons

This inventory applies the repository-wide tail-review checklist in root
`REVIEW.md`, especially its canonical update surface. That checklist is broader
than the v0.91.8 milestone inventory and explicitly includes repository
entrypoints, all active milestone Markdown, next-milestone bridge documents,
process documentation, manifests, and any document named by review or release
evidence.

Prior issues show why the broader surface is required:

- v0.91.7 WP-17 `#4644` expanded from a short milestone list to repository
  entrypoints, every current milestone status document, feature and review
  indexes, ADR navigation, all relevant README files, and broken historical
  links discovered by the complete audit;
- `#5489` expanded from next-milestone preparation into the complete v0.91.8
  canonical package, feature index, review index, third-party handoff, and
  current predecessor truth;
- `#5542` was required after merge because issue/PR status, bridge precedence,
  and date freshness still disagreed across canonical entrypoints.

WP-17 must therefore audit the complete surface first, then amend its typed
claim to the exact subset that actually needs edits. A short initial claim is
not evidence that omitted canonical documents are current.

## How To Use This Inventory

WP-17 must classify every path below at its exact execution revision as either:

- `update`: the document contains milestone status, routing, ownership, proof,
  deployment, review, release, or handoff statements that must match landed
  v0.91.8 evidence;
- `verify`: the document may remain byte-identical only after its claims,
  links, identifiers, and machine-readable companion data are checked; or
- `not_applicable`: no change is required, with a short source-backed reason in
  the WP-17 evidence packet.

The existing future implementation list in `design.md` is narrower than this
inventory. Before editing any path not already protected there, the WP-17 owner
must amend the typed claim and obtain bounded review of the expanded path set.

## Repository And Milestone Entrypoints

| Path | Default disposition | Required WP-17 reconciliation |
| --- | --- | --- |
| `README.md` | update | Replace stale active-milestone, product-status, and entrypoint claims with exact v0.91.8 truth. |
| `docs/milestones/v0.91.8/README.md` | update | Make this the accurate milestone index and link every current canonical, feature, review, release, and handoff surface. |
| `docs/milestones/v0.91.8/CANONICAL_DOC_INVENTORY_v0.91.8.md` | update | Replace the old planned/WP-21A framing with the final WP-17 audit inventory and current issue ownership. |
| `docs/milestones/v0.91.8/VISION_v0.91.8.md` | verify | Preserve the intended bridge outcome while removing any status claim contradicted by landed work. |
| `docs/milestones/v0.91.8/DESIGN_v0.91.8.md` | update | Reconcile the actual ADL v2, Runtime v3, and C-SDLC v2 boundaries and the final deletion/cutover posture. |
| `docs/milestones/v0.91.8/DECISIONS_v0.91.8.md` | update | Record current decisions and remove superseded setup, sequencing, tooling, or ownership language. |
| `docs/milestones/v0.91.8/BASELINE_AND_OWNERSHIP_v0.91.8.md` | update | Reconcile exact product ownership and current source baselines without turning inventory counts into deletion approval. |

## Repository-Wide Canonical Surfaces

| Path | Default disposition | Required WP-17 reconciliation |
| --- | --- | --- |
| `REVIEW.md` | update | Change the active review entrypoint from the v0.91.7 closeout tail to v0.91.8 and preserve the broad canonical-doc checklist. |
| `CHANGELOG.md` | update | Record only merged v0.91.8 outcomes and keep unreleased or residual work explicit. |
| `AGENTS.md` | verify | Confirm the agent contract matches the landed typed-v2, worktree, review, publication, GitHub-closing-keyword, and asynchronous-closeout behavior; edit only for real policy drift. |
| `docs/README.md` | update | Point documentation navigation at the current v0.91.8 milestone, feature, review, architecture, and v0.92 handoff entrypoints. |
| `docs/architecture/adr/README.md` | update | Ensure every v0.91.8 ADR is indexed with accurate accepted/proposed/superseded status. |
| `docs/adr/README.md` | verify | Keep the legacy ADR entrypoint correctly linked to the active architecture ADR index without presenting superseded records as current. |
| `adl/Cargo.toml` | verify | Check package version, binaries, features, and dependency truth against the release docs; change only if the manifest is stale. |
| `adl/Cargo.lock` | verify | Confirm it is the lockfile for the exact release dependency graph; do not churn it for docs-only reasons. |

There is no repository-root `Cargo.toml` or `Cargo.lock`; the canonical ADL
package/version surfaces are `adl/Cargo.toml` and `adl/Cargo.lock`.

## Planning, Routing, And Readiness

| Path | Default disposition | Required WP-17 reconciliation |
| --- | --- | --- |
| `docs/milestones/v0.91.8/WBS_v0.91.8.md` | update | Match the final work-package graph, issue assignments, and predecessor order. |
| `docs/milestones/v0.91.8/SPRINT_v0.91.8.md` | update | Point to current sprint authority and remove stale execution status. |
| `docs/milestones/v0.91.8/SPRINT_PLAN_v0.91.8.md` | update | Reconcile actual sequencing, completed waves, remaining gates, and allowed parallel work. |
| `docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md` | update | Reflect the final parallelization model and remove obsolete WIP, claim, or predecessor assumptions. |
| `docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml` | update | Make issue-to-WP routing, dependencies, and statuses agree with the canonical GitHub issue graph. |
| `docs/milestones/v0.91.8/WP_EXECUTION_READINESS_v0.91.8.md` | update | Replace preparation-era readiness with exact current dispositions and explicit remaining blockers. |
| `docs/milestones/v0.91.8/MILESTONE_CHECKLIST_v0.91.8.md` | update | Check only evidence-backed completed items and leave unmet review/release items open. |
| `docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md` | update | Reconcile each gate with the exact proving issue, validation surface, and result. |
| `docs/milestones/v0.91.8/DEMO_MATRIX_v0.91.8.md` | update | Align named demonstrations with real, non-fixture proof and explicit non-claims. |

## Product, Feature, And Proof Truth

| Path | Default disposition | Required WP-17 reconciliation |
| --- | --- | --- |
| `docs/planning/ADL_FEATURE_LIST.md` | update | Ensure every v0.91.8-relevant feature has a current owner and disposition. |
| `docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md` | update | Map each claimed feature to exact landed proof or an explicit blocker/non-claim. |
| `docs/milestones/v0.91.8/FEATURE_PRESERVATION_CROSSWALK_v0.91.8.md` | update | Reconcile preservation, deletion, deferral, and ownership decisions with the final implementation. |
| `docs/milestones/v0.91.8/RUNTIME_V3_FUNCTIONAL_PARITY_PLAN_v0.91.8.md` | update | Replace planning assumptions with the final Runtime v3 parity and residual-risk truth. |
| `docs/milestones/v0.91.8/features/README.md` | update | List every current feature document and remove stale index entries. |
| `docs/milestones/v0.91.8/features/ADL_V2_CORE_v0.91.8.md` | update | Describe the landed language, compiler, records, and CLI boundary only. |
| `docs/milestones/v0.91.8/features/RUNTIME_V3_ADAPTER_v0.91.8.md` | update | Reconcile the operational Runtime v3 adapter and provider boundary with exact proof. |
| `docs/milestones/v0.91.8/features/RUNTIME_V3_FUNCTIONAL_PARITY_v0.91.8.md` | update | State only demonstrated parity and list every remaining non-claim or residual risk. |
| `docs/milestones/v0.91.8/features/CSDLC_V2_ACCEPTANCE_v0.91.8.md` | update | Reflect final typed-v2 authority, supported GitHub operations, and resolved or retained defects. |
| `docs/milestones/v0.91.8/features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md` | update | Reconcile deployment, rollback, runtime launch, and platform-proof truth. |
| `docs/milestones/v0.91.8/features/DELETION_AND_CUTOVER_v0.91.8.md` | update | Match the actual cutover boundary and defer deletion claims until their owning WP lands. |
| `docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md` | update | Carry exact v0.91.8 revisions, supported surfaces, residual risks, and non-claims into v0.92. |

## Architecture, Review, Release, And Handoff

| Path | Default disposition | Required WP-17 reconciliation |
| --- | --- | --- |
| `docs/milestones/v0.91.8/ADR_PLAN_v0.91.8.md` | update | Match the ADR set actually required and produced for this milestone. |
| `docs/milestones/v0.91.8/review/README.md` | update | Index the exact formal-review inputs and distinguish them from issue-local reviews. |
| `docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md` | update | Provide a reproducible exact-revision review packet with current scope, proofs, and residual risks. |
| `docs/milestones/v0.91.8/RELEASE_PLAN_v0.91.8.md` | update | Reconcile the remaining review, remediation, release, and rollback sequence. |
| `docs/milestones/v0.91.8/RELEASE_NOTES_v0.91.8.md` | update | Replace placeholders with merged, user-visible outcomes and explicit limitations. |
| `docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md` | update | Pin what v0.92 may consume and what remains blocked, deferred, or explicitly unclaimed. |
| `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md` | update | Tie v0.92 activation tests to the exact v0.91.8 interfaces and evidence they consume. |

## v0.92 Bridge Documents

Because v0.91.8 explicitly gates v0.92, WP-17 must inspect the following
next-milestone documents and update only statements whose v0.91.8 inputs,
revisions, blockers, or activation order changed:

- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/VISION_v0.92.md`
- `docs/milestones/v0.92/DESIGN_v0.92.md`
- `docs/milestones/v0.92/DECISIONS_v0.92.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/milestones/v0.92/SPRINT_v0.92.md`
- `docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml`
- `docs/milestones/v0.92/MILESTONE_CHECKLIST_v0.92.md`
- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`
- `docs/milestones/v0.92/ADR_PLAN_v0.92.md`
- `docs/milestones/v0.92/RELEASE_PLAN_v0.92.md`
- `docs/milestones/v0.92/RELEASE_NOTES_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`
- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/features/README.md`
- every tracked feature document under `docs/milestones/v0.92/features/` whose
  dependency or consumption contract names a v0.91.8 surface.

`docs/milestones/v0.92/V092_DOCS_PREP_DOGFOOD_NOTES.md` is a retained planning
note. Verify its authority label and links, but do not rewrite historical
observations as current proof.

## Process And Tooling Documentation

The `REVIEW.md` checklist requires these surfaces when the milestone changes
process, cards, validation, closeout, or review entrypoints. v0.91.8 changed
those areas, so WP-17 must inspect and update the applicable files:

- `docs/templates/prompts/README.md` and the current prompt-template registry;
- `docs/cognitive-sdlc/README.md`, `card-lifecycle.md`,
  `tracked-workflow-state.md`, and `transition-schema.md`;
- `docs/tooling/README.md`;
- `docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md`;
- `docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md`;
- `docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md`;
- `docs/tooling/OWNER_BINARY_INSTALLATION.md`;
- `docs/tooling/ISSUE_BOUND_GOAL_TERMINAL_STATE_POLICY.md`;
- `docs/tooling/card-lifecycle.md` and `adl_pr_cycle_skill.md`;
- the nine active `csdlc-v2/operator/skills/*/SKILL.md` contracts.

Historical rescue, migration, or removed-v1 guides should be labeled
historical or superseded rather than rewritten as active authority. A tooling
implementation defect remains owned by its tooling issue; WP-17 aligns the
documentation but does not absorb the code repair.

## Repository-Owned Skills And Skill References

WP-17 must run a complete process-consistency audit over the repository-owned
skill package, not just the nine typed-v2 operator entrypoints. At the current
inventory revision this consists of:

- 61 `SKILL.md` entrypoints under `adl/tools/skills/` and
  `csdlc-v2/operator/skills/`;
- 94 Markdown reference contracts under
  `adl/tools/skills/*/references/`; and
- 54 shared skill-support documents under `adl/tools/skills/docs/`.

Every entry receives `update`, `verify`, `historical`, or `not_applicable` in
the WP-17 evidence packet. The audit must check, at minimum:

1. **Authority:** active skills route C-SDLC lifecycle work only through the
   independent Rust v2 binaries and do not revive removed v1 wrappers,
   `pr.sh`, prompt wrappers, or `csdlc-import`.
2. **Issue and worktree safety:** tracked implementation starts from a real
   issue, uses a normal dedicated bound worktree, and never writes tracked
   issue changes on `main` or in disposable temporary directories.
3. **Claims and coordination:** claims protect only concrete overlapping write
   paths, do not serialize disjoint work, and do not turn preparation or
   asynchronous closeout into an unrelated implementation blocker.
4. **Review:** one bounded exact-revision review occurs after implementation
   and validation and before PR publication; substantive changes after review
   invalidate that review.
5. **Publication:** implementation PRs include `Closes #<issue>`, use the
   intended base, and do not claim readiness from draft, stale-head, or pending
   CI evidence.
6. **Merge and GitHub closure:** supported repository tools cover issue, PR,
   check, merge, and closure actions without requiring an undocumented
   connector or wrapper.
7. **Closeout:** GitHub issue closure follows merge through the PR closing
   keyword; typed closeout remains truthful asynchronous lifecycle work and is
   not a prerequisite for starting unrelated, non-overlapping issues.
8. **Validation:** skills select the smallest proving PVF lane, avoid reflexive
   full builds for docs-only work, use stable owner binaries when provenance is
   current, and never count fixtures, simulations, or metadata as production
   proof.
9. **Commands and paths:** examples use current binary names, current repo
   paths, repo-relative durable artifacts, portable target directories, and no
   machine-specific or `/private/tmp` assumptions.
10. **Output contracts:** `SKILL.md`, reference schemas, examples, and shared
    support docs agree on statuses, stop boundaries, handoffs, and
    machine-readable output fields.

The audit scope is the complete contents of these directories:

- `adl/tools/skills/`
- `csdlc-v2/operator/skills/`

This directory-level inventory intentionally avoids duplicating 209 individual
paths in the plan. The WP-17 validation packet must generate and retain the
sorted exact path list and disposition for each file at its execution revision.

## Machine-Readable Companions

These are not prose documents, but WP-17 must keep them semantically aligned
with their human-readable owners whenever the corresponding claims change.

| Path | Human-readable owner | Required check |
| --- | --- | --- |
| `docs/milestones/v0.91.8/baseline_and_ownership_v0.91.8.json` | `BASELINE_AND_OWNERSHIP_v0.91.8.md` | Values, ownership, revision, and classification parity. |
| `docs/milestones/v0.91.8/feature_preservation_crosswalk_5594.v1.json` | `FEATURE_PRESERVATION_CROSSWALK_v0.91.8.md` | Row count, identities, owners, dispositions, and evidence parity. |
| `docs/milestones/v0.91.8/runtime_v3_functional_parity_plan_v0.91.8.json` | Runtime parity plan and feature doc | Capability, status, proof, and non-claim parity. |
| `docs/milestones/v0.91.8/review/wp01_execution_readiness_5594.v1.json` | WP-01 readiness report | Verify as historical source; update only if it is still presented as current authority. |

## Historical Inputs To Verify, Not Rewrite

- `docs/milestones/v0.91.8/review/V0918_WP01_EXECUTION_READINESS_5594.md`
  remains historical WP-01 evidence. WP-17 may correct links or its authority
  label, but must not rewrite the historical observations as if made later.
- Earlier milestone documents and issue-local evidence may be cited as source
  material. They are not part of the WP-17 edit set unless a separate,
  explicitly reviewed correction is required.
- `docs/milestones/v0.91.8/setup/5383/DESIGN.md` is retained setup history. It
  must not be presented as current execution authority.
- Generated C-SDLC records are lifecycle evidence, not milestone prose. They
  remain issue-local and are not folded into this documentation inventory.

## Completion Check

WP-17 is documentation-complete only when every row above has a recorded
disposition, every changed claim cites exact source evidence, Markdown links and
structured companions validate, product ownership remains separate, and any
unsupported statement is removed or labeled as a blocker, deferral, residual
risk, or explicit non-claim.

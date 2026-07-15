# C-SDLC v2 Documentation and Lifecycle Review

## Findings

### P1: Active workflow documentation still sends operators to deleted v1 commands

- File: `docs/default_workflow.md`
- Role: docs
- Scenario: A contributor follows the repository's named default workflow, onboarding page, Codex playbook, or active prompt-template documentation after Gate 10D2.
- Impact: The documented bootstrap, bind, validation, publication, and closeout path fails immediately or encourages an operator to bypass the sole typed-v2 authority. This is an active operator-safety defect, not a historical-reference problem.
- Evidence:
  - `AGENTS.md:43-45` says `workflow-conductor` and `pr.sh` are historical and invalid for C-SDLC v2; `csdlc-v2/operator/coexistence.json` forbids `adl/tools/pr.sh`, and the path is absent at the reviewed revision.
  - `docs/default_workflow.md:1-5`, `docs/default_workflow.md:27-50`, and `docs/default_workflow.md:57-60` call `pr.sh` the default workflow and active control plane. The same file contains many runnable `bash ./adl/tools/pr.sh ...` examples.
  - `docs/onboarding.md:18-34` names `pr run`, `pr finish`, and `workflow-conductor` as current routes.
  - `docs/codex_playbook.md:31-45` provides a copy/paste `adl/tools/pr.sh` lifecycle and says it still works.
  - `docs/templates/prompts/README.md:80-99` instructs maintainers to use deleted `csdlc tooling prompt-template` and `adl/target/debug/csdlc` surfaces.
  - This was independently derived from current tracked documentation. It overlaps the subject of testing discovery #5366, but #5366 was not used as the finding source.

### P1: The active six-card template deterministically generates sunset lifecycle instructions

- File: `docs/templates/prompts/1.0.3/sip.md`
- Role: docs
- Scenario: Root policy directs a new issue through `docs/templates/prompts/current.json`, which declares template set `1.0.3` active.
- Impact: Every newly rendered SIP can embed commands that the same root policy declares invalid, so cleanly generated lifecycle state starts internally contradictory and can no longer be executed as written.
- Evidence:
  - `AGENTS.md:76-83` requires new cards to come from the current registry and says v1 prompt-template wrappers are sunset.
  - `docs/templates/prompts/current.json:1-12` marks `1.0.3` active and selects `docs/templates/prompts/1.0.3/sip.md`.
  - The selected SIP tells agents to use `pr run` at lines 21-24, `workflow-conductor` and `pr run` at lines 119-125, and the repo-native `pr run` flow at lines 171-175.
  - The matching locked structure schema preserves those statements, so structure-valid rendering reproduces the stale commands rather than rejecting them.

### P1: Terminal lifecycle truth is stale, internally contradictory, and not retained with the reviewed revision

- File: `.adl/v0.91.7/tasks/issue-5306__v0-91-7-csdlc-v2-gate-10d2-approved-bounded-v1-deletion-wave/sor.md`
- Role: docs
- Scenario: A reviewer audits the six lifecycle cards for the 18 closed sprint issues after all implementation PRs have merged.
- Impact: The cards cannot establish review, publication, integration, or closeout truth for the sprint. A future reviewer can see closed GitHub issues and final tracked evidence while the nominal SRP/SOR authority still says work never started, publication is pending, or closeout never ran.
- Evidence:
  - All six named cards exist for all 18 issues in the primary checkout, but `.gitignore:4` ignores `.adl/`; `git ls-files` returns no card for the reviewed issue set, and the cards are absent from the #5375 worktree. Their contents are machine-local mutable evidence, not retained revision evidence.
  - #5306 is closed and PR #5331 merged green, but its SOR says `Status: NOT_STARTED` and `Integration state: worktree_only` at lines 17 and 82; its SRP says `findings_status: not_run` and `recommended_outcome: not_run` at lines 58-59.
  - #5307 and #5308 are closed, but both SORs remain `NOT_STARTED`/`worktree_only` and both SRPs remain `not_run`.
  - The Gate 10 umbrella #5240 is closed while its SOR remains `IN_PROGRESS` and its SRP remains `not_run`.
  - Earlier terminal gates also retain stale closeout projections: #5228 says `Closeout state: not_started` and `PR state: not_open`; #5239 says `Closeout state: not_started` and `PR state: not_published`; #5294 says `Closeout state: not_started` and `PR state: not_opened`; their embedded machine facts still classify the PR as open/pending.
  - Fourteen SRPs record `no_findings/pass` in frontmatter while their rendered `Review Results` body still says `Not run yet; implementation has not been bound`, so the human projection contradicts the machine-readable result in the same card.

### P1: Gate 10 umbrella and sunset-child closure occurred without reconciled dependency truth

- File: `docs/architecture/csdlc-v2/gate10d2/ACCELERATED_OPERATOR_DECISION.md`
- Role: docs
- Scenario: A reviewer reconstructs Gate 10 completion from the parent/child issue graph, merged PRs, accelerated approval, and terminal cards.
- Impact: GitHub closure no longer proves that the umbrella acceptance surface was complete or that separately scoped sunset issues received an explicit superseded/closed-no-PR disposition. This weakens the audit trail for the highest-risk deletion step.
- Evidence:
  - Live GitHub truth shows #5240 closed at `2026-07-14T06:44:45Z`, before Gate 10D2 PR #5331 merged at `2026-07-14T12:26:06Z` and before #5307/#5308 closed at approximately `18:18Z`.
  - #5295 closed at `2026-07-14T18:15:25Z`, before #5308 and #5307 closed several minutes later.
  - #5307 and #5308 have no execution PR in their issue timelines; their only linked merged PR is #5320, which prepared child cards/designs. Their local SRP/SOR records remain pre-run and do not record a no-PR, superseded-by-#5306, or accelerated-waiver closeout.
  - The tracked accelerated decision and Gate 10D2 design authorize an exact typed waiver under #5306, and the D2 evidence records importer removal. They do not reconcile the terminal lifecycle state of #5240, #5295, #5307, or #5308.

### P2: Thin operator skills do not document the mandatory generation-resolution step

- File: `csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md`
- Role: docs
- Scenario: An agent follows any of the nine active operator skills directly.
- Impact: The agent is told to invoke a bare `csdlc-*` binary without first resolving the tracked selector and installed provenance, so PATH state can bypass the authority boundary described by both root and nested `AGENTS.md`.
- Evidence:
  - `AGENTS.md:135-138` and `csdlc-v2/AGENTS.md:6-8` require every current route to pass through `csdlc-install resolve` and the dedicated `.adl/bin/csdlc-v2/` installation.
  - Each file under `csdlc-v2/operator/skills/*/SKILL.md` directly says `Invoke csdlc-*` or `Use csdlc-*`; none names `csdlc-install resolve`, selector consumption, or provenance verification.
  - The init skill additionally says `Preserve v1 as default during coexistence`, although current `csdlc-v2/operator/generation-selector.json` selects v2 and `csdlc-v2/operator/coexistence.json` records `v1_sunset: true`.

### P2: The current workspace README contradicts final sunset authority

- File: `csdlc-v2/README.md`
- Role: docs
- Scenario: A maintainer uses the workspace README to understand current generation, deletion, or compatibility-window behavior.
- Impact: The README can lead maintainers to believe v1 is still protected from deletion and that both original windows remain mandatory, despite the reviewed selector and D2 evidence saying the command surface and importer are already sunset.
- Evidence:
  - `csdlc-v2/README.md:44-48` says later cutover never deletes or disables any v1 surface.
  - `csdlc-v2/README.md:50-56` describes D1 as enforcing both mandatory sunset windows and says actual removal belongs to a later issue, without a current D2 status section.
  - Current authority says the opposite: `csdlc-v2/operator/coexistence.json` records `v1_sunset: true`; `docs/architecture/csdlc-v2/gate10d2/ELIGIBILITY_DECISION.json` records the exact waiver, `deletion_executed: true`, and `importer_binary_removed: true`.
  - Historical Gate 10A-C design/evidence remains correctly reviewable as historical evidence; the defect is that the current README does not label its obsolete state or carry the final D2 outcome forward.

### P2: Gate 1 release identity differs across issue, cards, PR, and merge commit

- File: `.adl/v0.91.7/tasks/issue-5228__v0-91-7-csdlc-v2-clean-room-architecture-baseline/sor.md`
- Role: docs
- Scenario: Release evidence groups Gate 1 with the v0.91.7 sprint by issue/card metadata or with v0.92 by PR/commit metadata.
- Impact: Automated or human release accounting can place the architecture baseline in two milestones, undermining the requested issue/PR/commit coverage matrix.
- Evidence:
  - Issue #5228, its label, all six local cards, and its architecture review context identify `v0.91.7`.
  - Merged PR #5231, its head branch, and merge commit `4f53f049a` identify `[v0.92]` / `5228-v0-92`.
  - The issue was closed by that PR without a retained disposition explaining the version mismatch.

## Metadata

- Skill: `repo-review-docs`
- Target: branch `codex/5375-v0-91-7-csdlc-v2-full-sprint-review` at `7c3e1e0e8`
- Date: 2026-07-14
- Artifact: `docs/milestones/v0.91.7/review/csdlc-v2/issue-5375/specialists/DOCS_LIFECYCLE_REVIEW.md`
- Review depth: deep
- Validation mode: inspect-only plus live GitHub metadata comparison

## Reviewed Surfaces

- Documentation objects: root `AGENTS.md`; `csdlc-v2/AGENTS.md`; `csdlc-v2/README.md`; `csdlc-v2/operator/SKILLS.md`; selector/coexistence manifests; all nine operator skills; active prompt registry/templates/schemas; contributor onboarding/default-workflow/playbook docs.
- Gate objects: Gate 1 architecture/budget/recommendation/validation; Gate 2-9 designs, diagrams, validation/soak/decision records; Gate 10A-D4 designs, manifests, decisions, parity/deletion evidence, and historical cutover records.
- Lifecycle objects: SIP, STP, SPP, VPP, SRP, and SOR for issues #5228, #5232-#5240, #5292-#5295, and #5305-#5308; 108/108 named card files were present in the primary checkout.
- Closeout objects: live issue state/timelines for all 18 issues; merged PRs #5231, #5257, #5263, #5268, #5270, #5272, #5274, #5275, #5290, #5298, #5301, #5304, #5316, #5320, and #5331; commit/path history at the reviewed revision.
- Testing-discovery comparison: #5364-#5373 were read and classified as prior testing discoveries. They were not imported as review findings. Where this review independently confirmed overlapping documentation drift, the overlap is labeled above.

## Commands Or Claims Checked

- Checked that forbidden v1 paths named by `csdlc-v2/operator/coexistence.json` are absent.
- Checked active template selection and the literal commands emitted by template set `1.0.3`.
- Checked current selector/install claims against root/nested authority and all operator skills.
- Checked all six card files per issue, SRP result fields, rendered review text, SOR status/integration/PR/closeout fields, ignore/tracking state, and availability in the review worktree.
- Checked issue closure, PR association, merge timestamp, base branch, check conclusions, and merge commit through live GitHub metadata.
- Treated references inside Gate 10A-C evidence as immutable historical observations when clearly scoped as such.

## Validation Performed

- `git status --short --branch` and `git worktree list --porcelain`: confirmed the clean, issue-bound #5375 review worktree and current revision.
- `rg`/`find` scans over `AGENTS.md`, `csdlc-v2/`, `docs/architecture/csdlc-v2/`, active templates, current workflow docs, and all 108 local cards: proved path presence, stale command references, and lifecycle-state contradictions.
- `git check-ignore -v` and `git ls-files` from the primary checkout: proved the reviewed card bundles are ignored and untracked.
- `jq` over Gate 10 selector, coexistence, eligibility, deletion, parity, and final-recheck records: checked machine-readable historical/current authority claims.
- `gh issue view`, issue timeline API, and `gh pr view` for the complete issue/PR set: checked live terminal and merge truth.
- No implementation, test, config, card, or existing-doc validation was run or modified by this docs lane.

## Residual Risk

- The ignored card bundles are machine-local snapshots in the primary checkout; another machine or a later local cleanup may yield different or missing lifecycle evidence.
- This docs lane did not re-execute Rust behavior, CI, provider/network paths, or the local-only proof packet. Code, tests, security, architecture, and dependency specialists own those claims.
- GitHub metadata was current at review time and may change after this artifact is written.
- Review of historical records distinguished descriptive evidence from current instructions by context; a repository-wide archival taxonomy was outside this bounded sprint lane.

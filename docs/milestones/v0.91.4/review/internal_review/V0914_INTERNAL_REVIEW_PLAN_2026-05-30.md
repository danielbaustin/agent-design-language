# v0.91.4 Internal Review Plan

## Status

`planned_not_started`

## Purpose

This document defines the v0.91.4 internal review plan for `WP-16` / `#3366`.
It is an all-on internal review party intended to find code, documentation,
process, evidence, validation, issue-truth, and release-tail problems before
v0.91.4 proceeds to external / third-party review.

This plan does not approve release readiness, external review readiness, or
publication. It defines the review lanes, scope, evidence inputs, stop
boundaries, and expected outputs for the review cycle.

## Review Owner

- Owner issue: `#3366` `[v0.91.4][WP-16][review] Internal review`
- Parent sprint: `#3362` `[v0.91.4][Sprint 4][sprint] Review, Remediation, Planning, And Release`
- Next expected stages:
  - `#3367` external / third-party review
  - `#3368` review findings remediation
  - `#3369` next milestone planning
  - `#3370` next milestone review pass
  - `#3371` release ceremony

## Source-Backed Current State

- Sprint 1, Sprint 2, and Sprint 3 core work are closed.
- Sprint 4 release-tail issues remain open and are the controlling v0.91.4
  closeout path.
- CodeFriend and WildClawBench sidecars are closed as bounded sidecar evidence
  lanes.
- Remaining pre-v0.92 bridge work has moved to v0.91.5, including
  multi-agent/provider/model/public-prompt/demo-readiness/first-birthday
  follow-on work.
- `#3506` created the v0.91.5 bridge split, and setup children `#3507` through
  `#3511` reviewed and repaired the bridge package.
- v0.91.4 must now be reviewed as a release-tail milestone with scoped bridge
  routing, not as a milestone that completes every pre-v0.92 surface.

## Review Goals

The internal review should:

- catch high-severity defects before third-party review
- verify v0.91.4 claims are evidence-bound
- separate completed v0.91.4 work from v0.91.5-routed follow-on work
- verify PVF, provider, model identity, browser/demo, and multi-agent evidence
  is classified truthfully
- verify workflow/process changes follow the ADL `AGENTS.md` contract
- produce a durable findings register and synthesis packet
- create or recommend remediation issues for accepted findings

## Non-Goals

This review must not:

- approve v0.91.4 release readiness
- replace external / third-party review
- close `#3366` without findings disposition
- complete v0.91.5-routed work inside v0.91.4
- claim multi-agent/provider/model/demo/public-prompt work is complete when it
  has been routed to v0.91.5
- treat CodeFriend or WildClawBench sidecars as proof of C-SDLC core completion
- merge, tag, publish, or close release-tail issues by itself

## Durable Review Outputs

Public review records are tracked by default for v0.91.4 forward. See `README.md` in this directory for the public-review-record policy, redaction rules, and local-only control-artifact boundary.

Write the review packet under:

`docs/milestones/v0.91.4/review/internal_review/`

Expected artifacts:

- `V0914_INTERNAL_REVIEW_PLAN_2026-05-30.md`
- `V0914_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-05-30.md`
- `V0914_INTERNAL_REVIEW_SYNTHESIS_2026-05-30.md`
- `V0914_INTERNAL_REVIEW_HANDOFF_2026-05-30.md`

Optional local control artifacts may live under ignored `.adl/reviews/` paths,
but reviewer-facing claims and findings must be summarized in tracked milestone
review docs.

## Primary Evidence Inputs

Milestone planning and release-tail docs:

- `docs/milestones/v0.91.4/README.md`
- `docs/milestones/v0.91.4/WBS_v0.91.4.md`
- `docs/milestones/v0.91.4/SPRINT_v0.91.4.md`
- `docs/milestones/v0.91.4/WP_ISSUE_WAVE_v0.91.4.yaml`
- `docs/milestones/v0.91.4/FEATURE_PROOF_COVERAGE_v0.91.4.md`
- `docs/milestones/v0.91.4/DEMO_MATRIX_v0.91.4.md`
- `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md`
- `docs/milestones/v0.91.4/MILESTONE_CHECKLIST_v0.91.4.md`
- `docs/milestones/v0.91.4/RELEASE_PLAN_v0.91.4.md`
- `docs/milestones/v0.91.4/RELEASE_NOTES_v0.91.4.md`
- `docs/milestones/v0.91.4/NEXT_MILESTONE_HANDOFF_v0.91.4.md`

Core feature docs:

- `docs/milestones/v0.91.4/features/`
- `docs/cognitive-sdlc/`
- `docs/tooling/card-lifecycle.md`
- `docs/tooling/structured-prompt-contracts.md`

Review and proof packets:

- `docs/milestones/v0.91.4/review/merge_readiness/`
- `docs/milestones/v0.91.4/review/obsmem_transition_memory/`
- `docs/milestones/v0.91.4/review/software_development_polis/`
- `docs/milestones/v0.91.4/review/multi_agent_workcell/`
- `docs/milestones/v0.91.4/review/provider_substrate_reconciliation/`
- `docs/milestones/v0.91.4/review/provider_communication_substrate/`
- `docs/milestones/v0.91.4/review/browser_automation/`
- `docs/milestones/v0.91.4/review/demo_showcase/`

Issue and PR truth:

- v0.91.4 issues `#3346` through `#3506` where applicable
- v0.91.5 bridge issues `#3507` through `#3511` for routing truth only
- merged PRs associated with v0.91.4 issue work
- open release-tail issues `#3362` through `#3371`

## Review Lanes

### 1. Workflow And C-SDLC Process Review

Focus:

- `AGENTS.md` compliance
- conductor use for issue lifecycle stages
- editor-only card mutation discipline
- worktree/branch/PR/closeout discipline
- no hand-rolled card or prompt drift
- `SIP -> STP -> SPP -> SRP -> SOR` lifecycle truth
- sprint conductor closeout truth

Questions:

- Can issue state advance without required cards or closeout truth?
- Can sprint state close over stale child issue truth?
- Are review and output records truth surfaces rather than summaries?
- Are public/tracked workflow records sufficient for reviewers?

### 2. Code Correctness Review

Focus:

- Rust provider substrate and adapters
- PVF runner and lane status aggregation
- conductor, doctor, editor, and closeout behavior
- model identity integration
- process-drift regression fixtures
- maintainability splits in large modules

Questions:

- Are behavioral changes tested at the right level?
- Are compatibility paths explicit and bounded?
- Are failure modes fail-closed rather than silent?
- Are local and hosted provider paths separated cleanly?

### 3. Test And PVF Review

Focus:

- PVF lane taxonomy and manifest schema
- docs-only PR lane behavior
- slow-proof / runtime-v2 lane separation
- coverage and release-gate policy
- pre-PR validation evidence reuse
- aggregate status semantics

Questions:

- Can failed, blocked, deferred, or pending lanes be hidden behind aggregate
  success?
- Does docs-only `adl-coverage` skipping remain visible as a proof category?
- Does release evidence distinguish PR-fast checks from release-gate checks?
- Does pre-PR validation reuse require exact commit/tree equivalence and fail
  closed when evidence is stale or absent?

### 4. Documentation And Release-Truth Review

Focus:

- milestone planning docs
- feature docs
- release-tail docs
- v0.91.5 bridge split truth
- v0.92 handoff truth
- stale issue numbers, stale status words, or mismatched routing

Questions:

- Do docs accurately reflect what is complete, blocked, routed, or deferred?
- Are sidecars clearly sidecars?
- Are v0.91.5-routed items no longer represented as v0.91.4 blockers?
- Are release notes and release plans claim-safe?

### 5. Evidence And Demo Review

Focus:

- Starharvest browser proof
- ADL Creative Room proof
- browser automation runbook
- Software Development Polis proof packet
- merge-readiness proof packet
- ObsMem transition memory packet
- multi-agent workcell evidence
- CodeFriend sidecar evidence
- WildClawBench sidecar evidence

Questions:

- Is each demo classified as proving, non-proving, skipped, blocked, or
  historical?
- Are proof packets replayable enough for reviewers?
- Are sidecar outcomes separated from core C-SDLC proof?
- Are browser/demo runbooks usable without hidden local knowledge?

### 6. Provider And Runtime Review

Focus:

- shared provider communication substrate
- Rust hosted provider adapter
- Ollama provider adapter
- tail-friendly provider logs
- ADL/UTS provider reuse boundary
- model identity and execution identity

Questions:

- Are hosted and local failures normalized consistently?
- Are secrets and API keys redacted from logs and artifacts?
- Do logs support tailing and debugging connection failures?
- Does model identity distinguish provider-asserted, tag-only, pinned, ad-hoc,
  and unknown identity strength correctly?

### 7. Security And Publication-Safety Review

Focus:

- secret handling
- local path leakage
- public documentation overclaims
- CodeFriend public surface and AWS/DNS evidence
- benchmark/provider evidence redaction
- review packet publication boundaries

Questions:

- Are any keys, local absolute paths, machine names, or private payloads exposed
  in tracked artifacts?
- Are public claims backed by source evidence?
- Are private/product sidecar claims bounded?
- Are third-party review packets safe to share?

### 8. Dependency And CI Review

Focus:

- GitHub workflows
- PVF CI lanes
- toolchain assumptions
- docs-only CI behavior
- slow-proof lane wiring
- nightly coverage issue automation removal

Questions:

- Are CI pass/skip states interpreted correctly?
- Are broad test cycles required only where justified?
- Are release-gate validations still present after PVF changes?
- Are dependency/tool assumptions documented for reviewers?

### 9. Issue And PR Truth Review

Focus:

- issue state across v0.91.4 and bridge-routed v0.91.5 work
- PR merge state and check results
- milestone labels and titles
- closeout comments and local card truth where available

Questions:

- Do open issues correspond to actual remaining release-tail work?
- Are closed issues truly closed with merged PRs or explicit no-op routing?
- Are v0.91.5 labels applied to moved work?
- Is any issue described as active/complete incorrectly in docs?

## Procedure

### Phase 1: Snapshot

Capture:

- `git status`
- open PR list
- open v0.91.4 issue list
- open v0.91.5 bridge issue list
- merged PR list for v0.91.4 issue range
- current milestone review/evidence tree

Output:

- source snapshot section in the findings register

### Phase 2: Packet Build

Build a review packet that maps:

- issue -> PR -> changed files -> proof/evidence docs
- milestone docs -> feature docs -> review/proof packets
- v0.91.4 release-tail work -> v0.91.5 routed work

Output:

- review source map inside the synthesis packet

### Phase 3: Specialist Reviews

Run specialist review lanes over bounded surfaces. Each lane must produce:

- findings first
- severity labels
- exact file/line or issue/PR evidence
- open questions and assumptions
- non-claims
- recommended route

### Phase 4: Synthesis

Deduplicate findings and classify each as:

- v0.91.4 release-tail blocker
- v0.91.4 pre-external-review blocker
- v0.91.4 remediation issue
- v0.91.5 follow-on
- backlog / non-blocking improvement
- no-op / already fixed

### Phase 5: Remediation Planning

Convert accepted findings into grouped issue candidates using conductor and
versioned templates. Do not hide separate problems inside unrelated fixes.

Output:

- finding-to-issue mapping in the findings register
- remediation recommendations for `#3368`

### Phase 6: Handoff

Prepare a handoff for WP-17 external / third-party review only after:

- findings register exists
- P0/P1 findings are fixed or explicitly routed
- remaining P2/P3 findings have owners or accepted deferrals
- release-tail non-claims are clear
- v0.91.5-routed scope is not represented as v0.91.4 completion

## Severity Policy

- `P0`: release or review cannot proceed; severe correctness, safety, secret,
  data loss, or process-integrity issue.
- `P1`: must fix or route before external review; material correctness,
  validation, lifecycle, or evidence-truth issue.
- `P2`: should fix before release or explicitly route; meaningful doc/code/test
  drift or reviewer-confusing evidence problem.
- `P3`: cleanup, polish, or follow-on improvement that should not block review
  unless it accumulates into systemic drift.

## Required Review Checks

At minimum, the internal review should check:

- no open PRs with failed checks are hidden from the packet
- all open v0.91.4 issues are expected release-tail issues or explicitly routed
- all moved v0.91.5 issues have v0.91.5 labels/titles
- release-tail docs do not overclaim completion
- PVF does not turn skipped docs-only coverage into release proof
- slow-proof/runtime lanes remain visible
- sidecar evidence is not core C-SDLC proof
- provider logs do not expose secrets
- benchmark/demo evidence does not overclaim general results
- tracked review evidence does not depend on private temp paths
- public prompt-record transition is routed to v0.91.5, not implied complete

## Expected Exit Criteria

`#3366` can close only when:

- review plan has been executed or intentionally superseded
- findings register is tracked
- synthesis report is tracked
- P0/P1 findings are fixed or routed
- P2/P3 findings are grouped for remediation/follow-up
- `#3368` has enough detail to execute remediation
- WP-17 handoff can be prepared without hiding caveats

## Residual Risks To Track

- v0.91.4 has had major late changes in PVF, provider substrate, browser
  automation, demos, and bridge routing; review must assume stale docs are
  likely until checked.
- Some workflow/card truth may remain in ignored `.adl/` paths; tracked review
  docs must not cite local-only paths as durable proof unless the packet says
  so explicitly.
- v0.91.5 bridge routing is fresh and should be checked carefully so reviewers
  do not mistake moved work for failed v0.91.4 work.
- CI checks may be green while release-gate validation remains incomplete; PVF
  lane semantics must make that distinction visible.

## Validation For This Plan

Not run. This document is a planning artifact. The review execution issue
should perform the snapshot, review packet build, and specialist lane checks
before producing findings.

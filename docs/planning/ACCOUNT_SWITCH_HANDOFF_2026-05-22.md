# Cross-Account Session Handoff - 2026-05-22

Historical note: this packet captures live routing and milestone assumptions
from 2026-05-22 during the `v0.91.4` wave. Treat milestone-specific guidance
here as time-bound context unless a newer planning or release document
explicitly supersedes it.

## Purpose

This packet exists because the current account is low on credits and future ADL
work may continue from another account or session later today.

It is a continuity document, not a substitute for issue cards, sprint closeout,
or release approval. Future agents should use this as the first orientation
surface, then verify live issue, PR, and repository state before acting.

## First Rules For The Next Session

Read `AGENTS.md` first.

Then follow these rules without exception:

- Use `workflow-conductor` for every issue and lifecycle stage.
- Edit cards only with the editor skills: `sip-editor`, `stp-editor`,
  `spp-editor`, `srp-editor`, and `sor-editor`.
- Do tracked issue work only in a bound worktree on a specific branch.
- Do not work on `main` for tracked issue execution.
- Run the smallest proving validation for the touched surface.
- Do not run broad test suites for docs-only issues.
- Run bounded review before PR publication and fix actionable findings.
- Perform closeout after merge or intentional issue closure.
- Keep `SIP -> STP -> SPP -> SRP -> SOR` as the canonical card lifecycle.

## Current Critical Path

As of 2026-05-22, the immediate project priority is to close v0.91.3 cleanly
without losing the C-SDLC demo mini-sprint state.

The most important live items are:

- `#3224` / PR `#3257`: C-SDLC demo showcase packaging.
- `#3219`: C-SDLC demo mini-sprint umbrella.
- `#3231`: Sprint 4 review, remediation, planning, and release umbrella.
- `#3226` through `#3230`: v0.91.3 closeout-tail work packages.
- `#3211`: v0.91.3 release ceremony.

Observed PR state for `#3257` on 2026-05-22:

- PR: `https://github.com/danielbaustin/agent-design-language/pull/3257`
- State: open draft
- Base: `main`
- Head: `codex/3224-v0-91-3-demo-wp-05-docs-package-c-sdlc-demo-index-and-review-surface`
- Mergeability: mergeable
- Checks observed: `adl-ci` success and `adl-coverage` success

Future sessions should re-check this state before making decisions.

## v0.91.3 State

v0.91.3 is the first C-SDLC implementation milestone. Its goal is to prove one
bounded, reviewable Cognitive State Transition using:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

The milestone is not supposed to claim that all future ADL development has
already moved onto C-SDLC. That is v0.91.4 work.

The v0.91.3 package says:

- v0.91.3 proves the first slice.
- `SPP` is the issue-local operative execution plan.
- workflow records should move toward tracked repo truth.
- GitHub issue, PR, CI, branch, human review, and closeout discipline must be
  preserved.
- trace and proof references should be ready to become signed trace bundles in
  v0.91.4.

Primary source files:

- `docs/milestones/v0.91.3/README.md`
- `docs/milestones/v0.91.3/SPRINT_v0.91.3.md`
- `docs/milestones/v0.91.3/DEMO_MATRIX_v0.91.3.md`
- `docs/milestones/v0.91.3/WP_ISSUE_WAVE_v0.91.3.yaml`

## v0.91.4 State

v0.91.4 is planned as the C-SDLC completion and hardening milestone.

Its job is to make C-SDLC operational enough to become ADL's normal
software-development path:

- stricter lifecycle validators
- reliable conductor and editor routing
- tracked durable workflow state
- minimal signed trace proof
- repeatable evidence convergence
- sprint closeout that cannot complete while child issue closeout is stale
- ObsMem ingestion from tracked evidence, not local-only lore
- repeated five-minute-sprint metrics

The documented durable workflow namespace is:

```text
docs/milestones/v0.91.4/review/evidence/csdlc/
```

Primary source files:

- `docs/milestones/v0.91.4/README.md`
- `docs/milestones/v0.91.4/WBS_v0.91.4.md`
- `docs/milestones/v0.91.4/SPRINT_v0.91.4.md`
- `docs/milestones/v0.91.4/WP_ISSUE_WAVE_v0.91.4.yaml`
- `docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`

## C-SDLC Demo Mini-Sprint

The C-SDLC demo mini-sprint produced real artifacts, but it is not closeout-clean
yet.

Current issue/PR map:

- `#3219`: C-SDLC demo mini-sprint umbrella, still open.
- `#3220`: demo proof contract, closed; PR `#3251` merged.
- `#3221`: Starharvest HTML game, closed; PR `#3252` merged.
- `#3222`: five-minute sprint console, closed; PR `#3253` merged.
- `#3223`: Podcast Studio v2, closed; PR `#3256` merged.
- `#3224`: showcase packaging, still open; PR `#3257` open draft.

Focused local validations previously passed for the demo packets:

- `bash adl/tools/test_csdlc_demo_proof_contract_packet.sh`
- `bash adl/tools/test_five_minute_html_game_packet.sh`
- `bash adl/tools/test_five_minute_sprint_console_packet.sh`
- `bash adl/tools/test_podcast_studio_v2_packet.sh`

Known findings that still matter:

- The mini-sprint cannot be called closeout-clean while `#3224` and `#3219`
  remain open.
- The local sprint review state file under `.adl/reviews/` recorded a passed
  gate with empty checked issue/PR lists. Treat that as suspect local state, not
  release proof.
- Some browser-demo instructions rely on `--print-url` commands even though the
  scripts print a URL and exit; future docs should not imply the server is
  running unless a server command is also supplied.
- `#3224` cards may need editor-skill normalization before closeout if their
  `SPP`, `SRP`, or `SOR` no longer match the live PR state.
- The showcase should avoid saying the mini-sprint is complete until the open
  packaging PR and umbrella closeout are done.

Recommended next actions:

1. Re-check PR `#3257` live state.
2. If the PR is still open, use the `#3224` worktree and editor skills to fix
   any card or demo-doc truth drift.
3. Run only focused demo/docs validations.
4. Merge `#3257` only after review and green checks.
5. Close out `#3224`.
6. Use `sprint-conductor` and `pr-closeout` to close `#3219` with a truthful
   state artifact.

## CodeFriend

CodeFriend planning is tracked, but CodeFriend execution is not part of the
current v0.91.3 closeout.

Current planning home:

- `docs/planning/codefriend/`
- `docs/planning/codefriend/CODEFRIEND_PRE_ALPHA_REPO_AND_S3_WELCOME_MINI_SPRINT.md`

Important boundary:

- v0.91.3 scope is planning only.
- The CodeFriend pre-alpha site mini-sprint is planned for v0.91.4.
- The eventual full CodeFriend alpha milestone is expected later, likely in a
  v0.93.x product milestone.

The v0.91.4 pre-alpha site plan should establish:

- a private CodeFriend product/site repository
- a minimal welcome page
- S3 asset hosting in `us-west-2`
- CloudFront HTTPS delivery
- ACM certificate handling in `us-east-1`
- Route 53 DNS for `codefriend.ai` and `www.codefriend.ai`
- deployment, rollback, verification, and publication-safety docs

Do not create or execute the CodeFriend v0.91.4 issue wave during v0.91.3
unless the operator explicitly changes the milestone scope.

## UTS

UTS is treated as done for current ADL scheduling purposes unless the operator
explicitly reopens it.

Important boundary:

- UTS core work lives in the standalone `universal-tool-schema` repository.
- ADL should not re-mix standalone UTS core with ACC governed execution.
- ADL may retain UTS+ACC companion evidence, but UTS examples and tests should
  work standalone.

If exact UTS status is needed, inspect the standalone repository and its issues
directly rather than relying on chat memory.

## Known Local Residue And Cautions

Do not confuse local residue with active work:

- `.worktrees/codefriend-prealpha-s3-plan` is an older ad hoc CodeFriend draft
  worktree and should not be treated as the active issue path.
- Many old worktrees exist. Do not prune or remove worktrees broadly without an
  explicit operator request.
- `.adl/reviews/sprint-3219-state.json` is suspect as proof because it records
  empty issue/PR truth-check lists.
- Local `.adl/` files are useful workflow state, but public C-SDLC truth must
  be tracked when the milestone requires auditable state.

## First Actions For The Next Account

1. Read `AGENTS.md`.
2. Run `git status --short --branch` in the primary checkout.
3. Verify live state for issue `#3224`, PR `#3257`, and umbrella `#3219`.
4. If `#3257` is still open, continue there before starting unrelated work.
5. If `#3257` is merged, perform issue and sprint closeout before moving to
   Sprint 4 tail work.
6. Continue v0.91.3 closeout in issue order, using `#3231` as the Sprint 4
   umbrella.
7. Keep CodeFriend execution parked for v0.91.4 unless the operator explicitly
   changes scope.
8. Treat this packet as orientation only; issue cards and live GitHub state are
   authoritative.

## Non-Claims

This packet does not claim:

- v0.91.3 is ready to close.
- the C-SDLC demo mini-sprint is fully closed out.
- PR `#3257` is safe to merge without final review.
- v0.91.4 execution has started.
- CodeFriend infrastructure has been created.
- UTS status can be changed from ADL without inspecting the standalone repo.

## Validation Expectations For This Packet

This handoff packet should be validated with focused docs checks only:

- referenced tracked files exist where applicable
- no obvious secret, credential, or host-path leakage
- Markdown/link sanity
- `git diff --check`

Broad code tests are intentionally out of scope for this docs-only continuity
issue.

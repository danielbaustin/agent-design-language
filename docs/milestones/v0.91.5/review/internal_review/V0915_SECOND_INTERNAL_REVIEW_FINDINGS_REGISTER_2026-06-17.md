# v0.91.5 Second Internal Review Findings Register

Date: `2026-06-17`
Issue: `#3923`
Parent review issue: `#3576`
Review status: `findings_recorded`

## Scope

This second-pass internal review checked the live `v0.91.5` milestone after the
first-pass remediation wave and after PR `#3933` merged.

Reviewed surfaces:

- first-pass review register and remediation queue
- Sprint 4 release-tail docs
- logging, tooling, Rust-refactor, provider, review-provider, multi-agent, and
  evidence packets
- ADL GitHub/octocrab validation code paths
- provider/review-provider contract code touched in `v0.91.5`
- current live issue state for key release-tail and remediation issues

## Findings

### P1: PR validation still reports merged green PRs as cancelled when old cancelled check runs remain in the rollup

`#3891` was supposed to fix the merged-green validation truth problem, but the
current ADL validation path still fails on the same class of state. Running:

```bash
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh validation 3933 --json
```

against merged PR `#3933` returned:

- `pr_state: "MERGED"`
- latest `adl-ci` check: `SUCCESS`
- latest `adl-coverage` check: `SUCCESS`
- no failed checks
- no pending checks
- overall `disposition: "cancelled"`
- process error: `validation: PR #3933 is cancelled`

The code classifies any cancelled check in the rollup as globally cancelled
before considering whether newer checks for the same workflow succeeded:

- `adl/src/cli/pr_cmd/github/transport.rs:738`
- `adl/src/cli/pr_cmd/github/transport.rs:743`
- `adl/src/cli/pr_cmd/github/transport.rs:483`

Impact:

- A merged PR with successful latest required checks can still fail local
  validation.
- Review/closeout automation can keep re-opening an already-fixed-seeming
  failure class.
- This directly contradicts the first-pass remediation route for `#3891`, which
  was to treat merged green PRs as successful instead of misclassifying them as
  cancelled.

Recommended route:

- Reopen or create a narrow follow-up for PR validation rollup normalization.
- The fix should group check runs by workflow/check identity and evaluate the
  latest relevant run, or otherwise ignore superseded cancelled runs when newer
  required runs succeeded on the merged head.

### P1: `pr finish` can commit stale SOR truth because it stages before later SOR mutation

The current `pr finish` path stages selected paths before running finish
validation and before writing docs-only validation evidence back into the SOR.
It then commits without re-staging the SOR after those mutations.

Evidence:

- `adl/src/cli/pr_cmd/finish_support.rs:94` normalizes SOR enum aliases.
- `adl/src/cli/pr_cmd/finish_support.rs:96` stages the selected paths.
- `adl/src/cli/pr_cmd/finish_support.rs:117` runs finish validation.
- `adl/src/cli/pr_cmd/finish_support.rs:119` records docs-only validation
  evidence into the output card after staging.
- `adl/src/cli/pr_cmd/finish_support.rs:123` syncs completed output surfaces
  after staging.
- `adl/src/cli/pr_cmd/finish_support.rs:176` commits with `git commit -m`
  without an obvious re-stage of post-validation SOR changes.
- `adl/src/cli/pr_cmd/finish_support.rs:565` shows
  `record_docs_only_validation_evidence_for_finish` mutating the output card.

Impact:

- The local worktree can contain the correct SOR truth while the PR commit omits
  it.
- This explains the repeated class of "merged work but stale cards" problems
  seen during v0.91.5 closeout.
- External reviewers can reasonably distrust closeout cards if the publication
  path itself can leave generated truth unstaged.

Recommended route:

- Fix `pr finish` so all SOR/output mutations happen before staging, or re-stage
  mutated lifecycle/output surfaces immediately before commit.
- Add a focused regression proving docs-only finish evidence written by finish
  is included in the created commit.

### P1: Sprint/release-tail docs still conflict with live issue truth after WP-15

Several tracked milestone docs still describe `#3899` as active or queued even
though live issue state reports `#3899` closed, and still describe `#3579` as
seeded/open even though live issue state reports `#3579` closed.

Evidence:

- `docs/milestones/v0.91.5/SPRINT_v0.91.5.md:39` says first internal-review
  remediation is active under `#3899`.
- `docs/milestones/v0.91.5/SPRINT_v0.91.5.md:48` says closeout-tail execution
  should resume through queued `#3899`.
- `docs/milestones/v0.91.5/SPRINT_v0.91.5.md:105` still marks `#3579` as
  `seeded`.
- `docs/milestones/v0.91.5/WBS_v0.91.5.md:101` says `#3899` is the queued
  first internal-review remediation umbrella.
- `docs/milestones/v0.91.5/WBS_v0.91.5.md:141` says the first remediation tranche
  is currently staged through `#3899`.
- `docs/milestones/v0.91.5/QUALITY_GATE_v0.91.5.md:14` says WP-15 `#3579`
  remains open.
- `docs/milestones/v0.91.5/review/internal_review/V0915_WP15_INPUT_FROM_GAP_REVIEW_2026-06-17.md:61`
  lists the open release-tail issues but omits active second-pass review issue
  `#3923`, while other review packets treat `#3923` as active downstream work.
- Live ADL issue inspection in this review showed `#3899` closed and `#3579`
  closed.

Impact:

- External reviewers will see conflicting release-tail truth between the
  fresher README/Sprint 4 issue body and older Sprint/WBS/quality-gate docs.
- This weakens the claim that WP-15 made the milestone understandable without
  chat reconstruction.
- It can mis-sequence WP-16/WP-17/WP-18 work by making an already-closed queue
  look like an active prerequisite.

Recommended route:

- Create a focused docs-truth remediation issue before external review.
- Normalize `SPRINT_v0.91.5.md`, `WBS_v0.91.5.md`, and
  `QUALITY_GATE_v0.91.5.md` to the same release-tail state already reflected in
  the milestone README and Sprint 4 issue body.

### P2: Quality-gate blocker evidence cites ignored local `.adl` card paths that are absent from the review worktree

The quality-gate packet records stale closed-issue card truth by citing local
`.adl` paths:

- `docs/milestones/v0.91.5/review/internal_review/V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md:25`
- `docs/milestones/v0.91.5/review/internal_review/V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md:26`
- `docs/milestones/v0.91.5/review/internal_review/V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md:27`
- `docs/milestones/v0.91.5/review/internal_review/V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md:35`
- `docs/milestones/v0.91.5/review/internal_review/V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md:64`

But in the current `#3923` worktree, those cited files are not present. The only
local issue bundle under `.adl/v0.91.5/tasks/` is the current `#3923` bundle.

Impact:

- The blocker may be true, but its cited evidence is not durable or reviewable
  from the tracked repository state.
- External reviewers cannot inspect the exact stale SOR evidence without access
  to the original ignored local state.
- The quality gate is using a non-portable proof surface for a release-blocking
  claim.

Recommended route:

- Preserve the relevant stale-card excerpts in a tracked review packet, or rerun
  the closed-issue/card truth audit through a repo-native tool that emits
  tracked, redacted evidence.
- Do not rely on ignored `.adl` paths as the only support for release-tail
  blockers.

### P2: First-pass remediation queue/register packets are stale against the live closed queue

The retained first-pass remediation queue still says the queue is active and
that `#3896` / `#3899` remain open, even though live issue state now reports
the queue complete.

Evidence:

- `docs/milestones/v0.91.5/review/internal_review/V0915_FIRST_INTERNAL_REVIEW_REMEDIATION_QUEUE_2026-06-16.md:9`
  says `Queue status: active_execution`.
- `docs/milestones/v0.91.5/review/internal_review/V0915_FIRST_INTERNAL_REVIEW_REMEDIATION_QUEUE_2026-06-16.md:28`
  says `#3896` remains active with green draft PR `#3907`.
- `docs/milestones/v0.91.5/review/internal_review/V0915_FIRST_INTERNAL_REVIEW_REMEDIATION_QUEUE_2026-06-16.md:32`
  says `#3899` remains the open execution umbrella.
- `docs/milestones/v0.91.5/review/internal_review/V0915_FIRST_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-16.md:20`
  says `#3896` is the remaining active tooling-remediation child.
- Live ADL issue inspection in this review showed `#3899` closed.

Impact:

- A reviewer following the internal-review packet will think the first-pass
  remediation queue is still active, while the milestone README says it is
  closed.
- This makes second-pass review harder to trust because the retained input
  packet no longer reflects the state it is supposed to anchor.

Recommended route:

- Either freeze those files explicitly as historical snapshots and add a current
  "second-pass baseline" packet, or update them to record the closed queue state.
- Prefer a current baseline packet so reviewers do not have to infer which
  source is authoritative.

### P3: Rust refactor closeout still preserves manual `gh pr view` commands as validation provenance

The Rust refactor closeout now labels the raw `gh` commands as historical/manual
provenance, which is better than presenting them as the preferred path. However,
they still appear in the final validation block with `Result: passed`.

Evidence:

- `docs/milestones/v0.91.5/RUST_REFACTOR_MINI_SPRINT_CLOSEOUT_3751.md:136`
- `docs/milestones/v0.91.5/RUST_REFACTOR_MINI_SPRINT_CLOSEOUT_3751.md:165`
- `docs/milestones/v0.91.5/RUST_REFACTOR_MINI_SPRINT_CLOSEOUT_3751.md:173`

Impact:

- This is not a release blocker because the packet is explicit that the commands
  are historical/manual.
- It still creates a small external-review distraction after the milestone has
  repeatedly stated that GitHub workflow truth should move onto ADL/octocrab
  surfaces.

Recommended route:

- Leave as-is if time is tight, but route a future evidence-normalization cleanup
  to replace manual GitHub provenance with ADL-native issue/PR inspection output
  when feasible.

### P1: OpenRouter matrix evidence stores raw prompt text in tracked review artifacts

The v0.91.5 observability contracts say reviewer/public projections must not
expose raw prompts, but the OpenRouter matrix proof packet tracks lane request
JSON that includes full prompt input text.

Evidence:

- `docs/milestones/v0.91.5/SHARED_OBSERVABILITY_AND_OTEL_CONTRACT_3705.md:45`
- `docs/milestones/v0.91.5/OBSERVATORY_LOG_CONSUMPTION_CONTRACT_3710.md:40`
- `docs/milestones/v0.91.5/OBSERVATORY_LOG_CONSUMPTION_CONTRACT_3710.md:193`
- `docs/milestones/v0.91.5/review/openrouter_matrix/lane_requests/planner_openrouter_deepseek_v4_flash.json:28`
- `docs/milestones/v0.91.5/review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md:15`
- `docs/milestones/v0.91.5/review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md:17`
- `adl/tools/run_v0915_openrouter_matrix.py:238`

Impact:

- This is a publication/reviewer-facing redaction breach, not merely an
  internal debug convenience.
- It conflicts with the milestone's own observability/redaction contract and
  makes the evidence packet harder to publish safely.

Recommended route:

- Stop writing raw prompt text into tracked lane request artifacts.
- Preserve a prompt digest, prompt contract reference, task/lane identifiers,
  and redacted excerpt if needed.
- Regenerate or redact the existing OpenRouter matrix request artifacts before
  external review.

### P2: OpenRouter matrix evidence stores full provider output where excerpt-only evidence would suffice

The provider schema allows excerpt-only success records, but the OpenRouter
matrix adapter and validator preserve full provider output in machine-readable
result JSON.

Evidence:

- `adl/src/provider_communication.rs:620`
- `adl/src/provider_adapter.rs:104`
- `adl/src/provider_adapter.rs:105`
- `docs/milestones/v0.91.5/review/openrouter_matrix/lane_results/planner_openrouter_deepseek_v4_flash.json:33`
- `adl/tools/validate_v0915_openrouter_matrix.py:124`
- `adl/tools/run_v0915_openrouter_matrix.py:382`
- `adl/tools/run_v0915_openrouter_matrix.py:386`

Impact:

- This widens the durable evidence surface unnecessarily.
- It is less severe than raw prompt exposure because model output is already
  expected in some review summaries, but the machine-readable proof path should
  still prefer bounded excerpts and digests.

Recommended route:

- Keep human-facing lane output summaries where needed.
- Change machine-readable result artifacts to store output excerpts, output
  digests, status, and provenance rather than full raw model text.

### P3: Private LAN endpoint fixtures remain an active non-portable evidence reintroduction path

The current reviewed v0.91.5 packets did not show this LAN endpoint leaking into
the public proof packet, but nearby provider/demo test surfaces still hard-code
and assert a literal private LAN Ollama endpoint.

Evidence:

- `docs/milestones/v0.91.5/review/remote_gemma_watcher/REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md:30`
- `adl/tools/test_demo_codex_ollama_semantic_fallback.sh:13`
- `adl/tools/test_demo_codex_ollama_semantic_fallback.sh:42`
- `adl/src/provider_substrate.rs:673`

Impact:

- This is not a current packet blocker.
- It is a portability hygiene risk and should be routed if future evidence
  generation consumes those fixtures directly.

Recommended route:

- Use named test endpoints or sanitized fixture aliases in durable artifacts.
- Keep literal LAN endpoints local-only or explicitly redacted before review
  publication.

## Fixed Or Acceptable Since First-Pass Review

- DeepSeek capability truth is now conservative: `deepseek` and `openrouter`
  HTTP providers do not advertise native tool-calling by default in
  `adl/src/provider_substrate.rs`.
- `validate_review_provider_result` now transitively validates the embedded
  provider result and rejects provider-failed results with passing review status.
- The milestone README now reflects the closed `#3899` state and current
  second-pass review frontier.
- Logging and observability packets preserve strong non-claims around OTEL,
  repo-wide JSON cleanliness, and exhaustive provider/runtime correlation.
- Provider/model and multi-agent evidence packets are cautious about blocked
  lanes, live-provider scope, and single-agent fallback.
- Toolkit and tools remediation closeout packets are materially better than the
  earlier sprint-state drift.
- Markdown AST editing now fail-closes on lifecycle-card paths, preserving the
  rule that lifecycle cards should go through card/editor/template paths.

## Residual Risk Summary

The milestone is substantially improved, but not yet clean enough for external
review without remediation. The main risk is not missing feature work; it is
truth drift between live issue/PR state, retained review packets, and milestone
control docs.

The most important pre-external-review fixes are:

1. fix `pr validation` latest-check/superseded-cancelled behavior;
2. fix `pr finish` staging order so SOR/output truth written by finish is
   included in the PR commit;
3. normalize Sprint/WBS/quality-gate release-tail docs after closed `#3899` and
   closed `#3579`;
4. make the closed-issue/card truth blocker evidence durable instead of relying
   only on ignored local `.adl` paths;
5. remove raw prompt text from tracked OpenRouter matrix request artifacts;
6. refresh or supersede stale first-pass remediation queue/register packets.

## Validation Performed

Focused review validation used:

```bash
find docs/milestones/v0.91.5/review -maxdepth 4 -type f | sort
find docs/milestones/v0.91.5 -maxdepth 2 -type f | sort
rg -n "TODO|TBD|NOT_STARTED|not_run|worktree_only|/Users/|/private/tmp|gh |Sprint 1|second internal|first internal|blocked|failed" docs/milestones/v0.91.5 .adl/v0.91.5 -g '*.md'
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3899 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3703 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3845 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3579 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3576 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3923 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3574 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3580 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh validation 3933 --json
test -f .adl/v0.91.5/tasks/issue-3891__v0-91-5-tools-validation-treat-merged-green-prs-as-successful-in-pr-validation/sor.md
test -f .adl/v0.91.5/tasks/issue-3898__allow-intentional-section-local-removals-in-replace-section/sor.md
```

Additional review lanes used:

- bounded docs/release-truth subagent review;
- bounded code/control-plane subagent review;
- bounded tests/PVF proof-surface subagent review;
- bounded security/redaction/evidence-boundary subagent review;
- deterministic review-readiness scan using `inspect_review_readiness.py`,
  retained as local ignored helper output and summarized here rather than
  treated as standalone release proof.

No broad Rust test suite was run for this review issue. The review uses source
inspection, tracked packet inspection, live ADL issue inspection, and one
focused PR-validation reproduction.

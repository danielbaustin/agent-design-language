# v0.91.6 Internal Review Findings Register

Date: `2026-06-27`
Owner issue: `#4582`
Review stage: `WP-14A internal review and pre-v0.92 burn-down`
Status: `findings_recorded_for_release_tail_remediation`

## Scope

This findings register reviews the current `v0.91.6` release-tail state after
WP-11 `#3976`, WP-12 `#3977`, and WP-13 `#3978` have merged. It consumes closed
`#3979` as retained planning/source evidence only. It does not perform external
review, remediation, final preflight, next-milestone planning, or release
ceremony.

Primary evidence surfaces:

- `docs/milestones/v0.91.6/review/internal_review/V0916_INTERNAL_REVIEW_PLAN_2026-06-23.md`
- `docs/milestones/v0.91.6/review/V0916_WP13_DOCS_REVIEW_ALIGNMENT_3978.md`
- `docs/milestones/v0.91.6/MILESTONE_CHECKLIST_v0.91.6.md`
- `docs/milestones/v0.91.6/RELEASE_PLAN_v0.91.6.md`
- `docs/milestones/v0.91.6/README.md`
- `docs/milestones/v0.91.6/FEATURE_DOCS_v0.91.6.md`
- `docs/milestones/v0.91.6/DEMO_MATRIX_v0.91.6.md`
- `docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md`
- `docs/milestones/v0.91.6/OPERATIONAL_COMPLETION_GATE_v0.91.6.md`
- `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md`
- `docs/milestones/v0.91.6/review/V0916_POST_MATRIX_SINGLETON_REVIEW_4502.md`
- `docs/milestones/v0.91.6/review/V0916_CSDLC_ADOPTION_AUDIT_4434.md`
- `docs/milestones/v0.91.6/review/internal_review/V0916_FULL_CODE_REVIEW_2026-06-27.md`
- repo-native live issue search for open `version:v0.91.6` issues
- focused code review over PR lifecycle, SOR/SRP fact emission, runtime AWS signal, prompt/card, GitHub transport, and module-size surfaces

## Findings

### P1: Release-tail docs still carry stale WP-13 state after WP-13 merged

Expected: after WP-13 merged, reviewer-facing release-tail docs should identify
WP-14A `#4582` as the active frontier and should not keep WP-13 `#3978` as the
current/pending docs alignment step.

Observed: several surfaces still describe WP-13 as current, unchecked, or
pre-publication:

- `docs/milestones/v0.91.6/README.md` says WP-13 is the current docs/review
  alignment pass.
- `docs/milestones/v0.91.6/MILESTONE_CHECKLIST_v0.91.6.md` still leaves WP-13
  docs/review-surface alignment unchecked and says current owner is `#3978`.
- `docs/milestones/v0.91.6/RELEASE_PLAN_v0.91.6.md` still leaves WP-13 unchecked
  and says current owner is `#3978`.
- `docs/milestones/v0.91.6/review/V0916_WP13_DOCS_REVIEW_ALIGNMENT_3978.md`
  still says `Status: ready_for_pr_after_bounded_review` even though the packet
  has now landed.

Impact: external review would see contradictory release-tail state immediately
before consuming the internal-review packet. This should block external-review
handoff until fixed or explicitly routed through WP-16 remediation/preflight.

Recommended route: `#3981` remediation/final preflight, or a narrow WP-14A
truth-normalization follow-up if the operator wants it fixed before external
review handoff.

### P1: Pre-v0.92 activation cannot proceed from v0.91.6 evidence alone

Expected: every activation-relevant bridge surface should be classified as
complete, owned by an open v0.91.6 issue, routed to v0.91.7, a v0.92 blocker,
non-activation companion work, or deferred beyond v0.92 with rationale.

Observed: the evidence base is now strong, but the milestone still contains many
explicit non-claims and open gates:

- `docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md` states
  Soak #1 is only the walking-skeleton proof and Soak #2/#3 remain v0.91.7 gates
  before runtime-coherence or activation claims.
- `docs/milestones/v0.91.6/OPERATIONAL_COMPLETION_GATE_v0.91.6.md` requires
  `integrated_proven` before product/runtime surfaces count as operationally
  complete.
- `docs/milestones/v0.91.6/MILESTONE_CHECKLIST_v0.91.6.md` still leaves broad
  runtime/product completion-class, v0.92 activation, security, ACIP/A2A,
  Gemma/model reliability, AEE/Memory/ACP, and runtime coherence rows open.

Impact: v0.91.6 can proceed through review/remediation as a bridge milestone,
but cannot open v0.92 activation or birthday claims until the burn-down checklist
is consumed by WP-16/WP-17 and the named v0.91.7/runtime gates are carried
forward.

Recommended route: consume
`V0916_PRE_V092_BURN_DOWN_CHECKLIST_2026-06-27.md` in `#3981` and `#3982`; do
not advance activation claims in `#3984` ceremony.

### P1: `pr finish` can erase numbered SRP findings from machine-readable SOR facts

Expected: when an SRP records findings, `pr finish` should preserve those
findings in the emitted `sor_facts.review.findings` block regardless of Markdown
list style.

Observed: `parse_sor_review_evidence` reads findings through
`bullet_lines_from_markdown_section` at `adl/src/cli/pr_cmd/finish_support.rs:1003`,
but that helper only accepts lines with a `- ` prefix at
`adl/src/cli/pr_cmd/finish_support.rs:1063`. The WP-14A SRP used numbered
findings, so the current SOR emitted `findings_status: findings_present` while
`findings` became `not_recorded` at
`.adl/v0.91.6/tasks/issue-4582__v0-91-6-wp-14a-review-complete-internal-review-and-pre-v0-92-burn-down-checklist/sor.md:153`.

Impact: release-tail automation can claim findings exist while losing the actual
finding text in the machine-readable record.

Recommended route: fix the parser to support numbered Markdown list items and
add regression coverage using an SRP with numbered findings.

### P2: Repo-native PR inventory is still incomplete for internal-review use

Expected: the internal-review plan requires known open/dirty PRs to be listed
without relying on raw `gh`, because current workflow policy prefers repo-native
ADL/pr.sh and Octocrab-backed paths.

Observed: `pr.sh` supports issue search/view/create/comment/edit/close,
validation, watch, and closing-linkage, but it does not expose a supported
`pr list` command. The command dispatch in `adl/src/cli/pr_cmd.rs:210` has no PR
inventory subcommand, and the wrapper help in `adl/tools/pr.sh:18` confirms no
PR-list surface is advertised.

Impact: internal review can enumerate live issues and per-issue watcher state,
but cannot yet produce a fully repo-native open-PR inventory from the same
workflow surface. That forces either manual reviewer inspection, a forbidden raw
`gh` fallback, or incomplete PR-state evidence.

Recommended route: create a v0.91.6 or v0.91.7 tooling follow-up to add typed
repo-native PR list/search inventory, or explicitly document the approved
interim evidence path for external review.

### P2: Milestone checklist still mixes old forward-checklist residue with current review truth

Expected: by WP-14A, checklist rows should distinguish incomplete work from
completed/routed surfaces so downstream release-tail issues do not have to
reverse-engineer status from older planning rows.

Observed:

- `docs/milestones/v0.91.6/FEATURE_DOCS_v0.91.6.md` says the Observatory/Unity
  bridge doc plus retained WP-09 closeout proof exist, but
  `MILESTONE_CHECKLIST_v0.91.6.md` still leaves "Observatory/Unity consumption
  classification completed or routed" unchecked.
- `MILESTONE_CHECKLIST_v0.91.6.md` still leaves `#3802`-`#3805` unchecked even
  though later tooling/proof-loop reliability work landed through many newer
  issues and review packets.
- Several scope-integrity rows remain open without owner routes in the checklist
  itself, even though the evidence tree has owner packets for many of them.

Impact: the checklist remains useful as a release gate, but not as a clean
current-state ledger. External reviewers may read unchecked rows as missing work
rather than as rows requiring WP-16/WP-17 classification.

Recommended route: `#3981` should update checklist rows to `complete`, `routed`,
`blocked`, or `deferred` rather than leaving mixed-era unchecked rows.

### P2: C-SDLC adoption remains operational but not fully fail-closed by default

Expected: after the C-SDLC tooling work, issue execution should be predictable:
complete cards before execution, issue-bound goals, VPP/PVF lane assignment,
watcher ownership, SOR fact capture, repo-native GitHub operations, and closeout
should be enforced rather than merely available.

Observed: `docs/milestones/v0.91.6/review/V0916_CSDLC_ADOPTION_AUDIT_4434.md`
records zero items as `enforced`, ten as `operational`, eleven as `routed`, three
as `deferred`, and one as `blocked`. It specifically records several important
surfaces as `default-but-not-enforced`, including issue-bound session goals,
session-ledger claims, full card lifecycle, VPP/PVF facts, pre-PR review gates,
watcher/shepherd state, finish-time fact reuse, and SOR fact capture.

Impact: the process is much stronger, but external review should not treat the
C-SDLC control plane as fully autonomous or fail-closed everywhere yet.

Recommended route: preserve this as a v0.91.7/v0.92 C-SDLC adoption residual and
ensure `#3982` carries it forward explicitly.

### P2: Live AWS runtime heartbeat attempts consume sequence numbers even when no signal is published

Expected: blocked live heartbeat publication should fail closed without mutating
heartbeat cursor state, because no durable signal was emitted.

Observed: `publish_runtime_heartbeat_signal` reserves a sequence before choosing
between mock and live mode at `adl/src/runtime_aws_signal.rs:344`. The live mode
branch then emits a failure and returns `Blocked` at
`adl/src/runtime_aws_signal.rs:404`. The reservation writes the cursor at
`adl/src/runtime_aws_signal.rs:847`. Existing tests assert that live mode blocks
at `adl/src/runtime_aws_signal.rs:1160`, but they do not assert that the cursor
remains absent or unchanged.

Impact: a misconfigured or intentionally blocked live runtime can advance local
heartbeat sequence state without publishing a heartbeat.

Recommended route: reserve heartbeat sequence only after the chosen transport is
known to be publishable, or roll back/avoid cursor writes for blocked live mode;
add a regression test asserting blocked live mode does not create or advance the
cursor.

### P3: The internal-review plan has minor duplicate input noise

Expected: the internal-review plan should be clean enough for downstream review
handoff.

Observed: `docs/milestones/v0.91.6/review/internal_review/V0916_INTERNAL_REVIEW_PLAN_2026-06-23.md`
lists `docs/milestones/v0.91.6/DEMO_MATRIX_v0.91.6.md` twice in the primary
planning/control inputs.

Impact: harmless, but it is a small signal of review-packet polish debt.

Recommended route: fix opportunistically in WP-16 docs cleanup or leave as a
non-blocking P3.

### P3: Several control-plane modules are too large for cheap, reliable review

Expected: high-churn workflow-control files should stay small enough that review,
coverage routing, and local reasoning remain cheap.

Observed: code inventory found `adl/src/cli/pr_cmd/finish_support.rs` at roughly
`6167` lines, `adl/src/resilience.rs` at roughly `5225` lines, and
`adl/src/csdlc_prompt_editor.rs` at roughly `3875` lines. The largest finish-path
test file is roughly `7993` lines.

Impact: this is not an immediate behavior bug, but it increases regression risk
and review latency. The repeated finish/SOR/card-truth issues in this milestone
cluster around exactly these dense control-plane surfaces.

Recommended route: route a v0.91.7 refactoring/design issue to split the highest
churn control-plane files along stable seams.

## Positive Review Results

- The retained evidence tree is now substantial and reviewable, with sprint,
  singleton, provider, runtime, security, public-record, build-throughput,
  validation, observability, ADR, and C-SDLC adoption packets present under
  `docs/milestones/v0.91.6/review/`.
- Live open `version:v0.91.6` issues are concentrated in the expected release
  tail: umbrella `#4604`, WP-14A `#4582`, and WP-15 through WP-19 `#3980`-
  `#3984`.
- WP-11 `#3976`, WP-12 `#3977`, and WP-13 `#3978` are closed/merged before this
  internal review executes.
- The operational completion gate gives reviewers a strong vocabulary for
  separating prerequisite proof from integrated runtime/product completion.
- Runtime evidence is unusually disciplined: Soak #1 is explicitly a walking
  skeleton, not a full runtime coherence claim.
- The full code review pass found concrete code issues while confirming several
  important workflow surfaces are materially stronger than earlier milestone
  state.

## Non-Claims

- This register does not approve external review readiness by itself.
- This register does not claim v0.92 activation readiness.
- This register does not remediate accepted findings.
- This register does not claim every code path changed in v0.91.6 was fully
  re-reviewed in WP-14A; it records bounded code/tooling risks and routes the
  remaining release-tail review path.

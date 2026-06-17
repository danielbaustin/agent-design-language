# v0.91.5 Quality Gate

## Status

`blocked`

## Current Phase

WP-14 was applied on 2026-06-17 at the start of Sprint 4. The milestone is in
the review/remediation/release tail and is not release-ready.

- Sprint 4 umbrella `#3574` remains open.
- WP-14 quality gate `#3575` is the active child issue.
- WP-15 `#3579`, WP-16 `#3576`, WP-17 `#3580`, WP-18 `#3577`, WP-19 `#3581`,
  and WP-20 `#3578` all remain open.
- Second-pass internal review `#3923` is also open, so this gate must treat
  review readiness as a handoff surface rather than a completed fact.

## Gate Outcome

The quality gate is blocked for release, but successful as a truthful control
surface.

- Passed:
  - reusable checklist from `#3531` is now applied
  - Rust module size check was run through the live tracker command
  - Sprint 4 ordering and release-tail blockers are explicit
  - an internal-review plan now exists for the second-pass review
- Deferred or follow-on:
  - no dedicated v0.91.5 test-coverage gap-analysis packet exists yet
  - no dedicated test-runtime regression comparison packet exists yet
  - ADR readiness remains `not_applicable` for this docs-only gate pass
- Blockers:
  - Sprint 4 release-tail issues remain open
  - sampled closeout truth is not uniformly clean; recent closed issue `#3891`
    still has a stale local SOR that reports `Integration state: worktree_only`
    despite the issue being closed
  - multiple v0.91.5 milestone docs still carry stale `active_wp_01_opening`
    or opening-era status language and must be normalized in WP-15

## Evidence Packets

- Checklist application packet:
  [V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md](review/internal_review/V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md)
- Internal-review readiness plan:
  [V0915_SECOND_PASS_INTERNAL_REVIEW_PLAN_2026-06-17.md](review/internal_review/V0915_SECOND_PASS_INTERNAL_REVIEW_PLAN_2026-06-17.md)

## Required Validation

The milestone must run focused validation for:

- planning-template structure for canonical planning docs;
- YAML parsing for the issue wave;
- GitHub issue label/title routing for moved bridge issues;
- public prompt packet export, validation, and redaction gates;
- `.adl` archive/deletion review-before-delete disposition;
- provider/model matrix smoke and role-probe evidence;
- multi-agent workcell proof or explicit blocker;
- single-agent versus multi-agent overhead comparison;
- demo readiness and Unity Observatory routing;
- v0.92 activation map completeness;
- AEE completion-tranche routing and proof expectations;
- `#3377` first-birthday readiness packet.

WP-14 itself is docs/control-plane work, so this pass used focused
documentation, issue-state, and tracker evidence rather than broad runtime or
coverage theater.

## Reusable Quality-Gate Checklist

Future coverage / quality-gate WPs should use this checklist as the default
control-tower pattern. The goal is not to run every expensive command by
reflex. The goal is to make release-quality truth visible before docs/adoption
review, internal review, and release packaging depend on it.

Each row should be recorded as `passed`, `blocked`, `deferred`, `not_applicable`,
or `follow_on`, with owner/rationale whenever it is not `passed`.

| Check | Required activity | Evidence / output |
| --- | --- | --- |
| Test coverage gap analysis | Run or review the focused coverage/test-review skill path for changed or high-risk surfaces. | Coverage gap notes, missing-test findings, or explicit no-op rationale. |
| Rust module size tracker | Check `.adl/reports/manual/rust_module_watch_list.md` when present and record module-growth hotspots. | Tracker summary, hotspots, split candidates, or note that the manual tracker was unavailable. |
| Issue closeout truth | Sample or mechanically check recent issue closeouts for merged/closed PR truth, SOR integration state, and follow-on routing. | Closeout audit notes and blocker dispositions. |
| Internal review readiness | Confirm the internal review plan exists, has scope, lanes, source surfaces, and reviewer handoff expectations. | Internal-review plan readiness note. |
| PVF lane health | Confirm docs-only, runtime fast/default, slow-proof, authoritative coverage, and release-gate lanes remain distinct. | PVF lane status showing `passed`, `pending`, `deferred`, `blocked`, or `failed`, never hidden. |
| Changed-file risk review | Identify high-risk changed surfaces since the last quality gate, especially runtime, control plane, prompt templates, validators, CI, and release policy. | Changed-surface risk list and validation mapping. |
| Test runtime regression | Compare slowest test families and lane runtimes against recent baseline evidence. | Slow-test creep notes, long-tail blockers, or release-gate routing. |
| Prompt/card lifecycle audit | Check recent SIP/STP/SPP/SRP/SOR bundles for design-time readiness, review truth, and SOR closeout truth. | Card lifecycle audit result and editor-skill follow-ons if needed. |
| PR stack/base hygiene | Verify open PRs use intended bases, have no hidden conflicts, and do not block release-tail ordering. | PR topology and blocker summary. |
| Docs truth scan | Check README, milestone docs, feature list, release notes, checklist, issue wave, and version surfaces for stale claims. | Docs-truth findings or clean result. |
| ADR / decision readiness | Confirm new architectural decisions have accepted ADRs, candidate ADRs, or explicit routing. | ADR readiness table or no-op rationale. |
| Demo/proof artifacts | Confirm required demos and proof packets are present, replayable enough for review, and honestly classified. | Demo/proof status table. |
| Security/privacy/redaction | Check public prompt records, traces, review packets, and exported evidence for private-state, host-path, or secret leakage. | Redaction/security notes. |
| Follow-on issue hygiene | Confirm every known gap is fixed, explicitly deferred, or tracked in the right milestone. | Follow-on routing table with owners. |

## Checklist Application Rules

- Do not treat skipped broad validation as success. Record why it was skipped
  and which focused proof replaced it.
- Do not turn docs-only quality-gate work into a broad Rust test cycle unless
  runtime, CI, coverage policy, or validation tooling changed.
- Do not require `.adl/reports/manual/rust_module_watch_list.md` to be tracked;
  consume it as a manual tracker when available.
- Keep blockers separate from follow-on improvements. A blocked quality gate is
  a successful truthful outcome when blockers are real.
- Feed unresolved blockers into remediation or final preflight rather than
  hiding them in release notes.

## Blockers

The milestone is blocked if:

- multi-agent execution is claimed without role, shard, provider, review, and
  closeout evidence;
- OpenRouter/provider matrix work hides skipped or blocked lanes;
- public prompt packets can leak local/private state;
- `.adl` cleanup deletes historical material without review;
- v0.92 activation surfaces remain undocumented;
- AEE completion remains implicit in v0.95 convergence instead of being routed
  through `#3534` and v0.92 activation planning;
- v0.92 docs depend on direct v0.91.4 closeout rather than v0.91.5 closeout and
  `#3377`.
- the reusable checklist above is not applied, or its blocked/deferred rows are
  not routed before release closeout.
- portable ADL adapter planning is relied on for external repository evidence
  without a tracked contract, template, or explicit follow-on route.
- recent closed issue records overclaim closeout truth or remain stale without
  explicit remediation routing.
- milestone docs still present WP-01-opening status claims after Sprint 4 has
  begun.

## Release Gate

Release can proceed only when every blocker is fixed, explicitly deferred with
owner/rationale, or converted into a v0.92 WP-01 prerequisite.

The current phase is `blocked_for_release_tail`, not `release_ready`.

# v0.91.5 Quality Gate

## Status

Planned quality gate.

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
- `#3377` first-birthday readiness packet.

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
- v0.92 docs depend on direct v0.91.4 closeout rather than v0.91.5 closeout and
  `#3377`.
- the reusable checklist above is not applied, or its blocked/deferred rows are
  not routed before release closeout.

## Release Gate

Release can proceed only when every blocker is fixed, explicitly deferred with
owner/rationale, or converted into a v0.92 WP-01 prerequisite.

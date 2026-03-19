# ADL Work Breakdown Structure — v0.85

## Metadata
- Milestone: `v0.85`
- Version: `0.85`
- Date: `2026-03-19`
- Owner: `Daniel Austin / Agent Logic`

## Role Of This Document

This WBS is the canonical execution plan for the remainder of v0.85.

Use it to answer:
- what work remains
- what artifact or proof surface each work unit must produce
- which issue owns that work
- what order the remaining work should happen in

Do not treat the mid-flight review document as the execution plan. [MIDFLIGHT_REVIEW_ISSUES.md](/Users/daniel/git/agent-design-language/.adl/docs/v085planning/MIDFLIGHT_REVIEW_ISSUES.md) is now a diagnostic and closure tracker. Execution should flow through this WBS.

## Current Milestone State

What is already landed:
- `WP-02` deterministic queue, checkpoint, and steering substrate
- `WP-03` cluster / distributed execution groundwork
- `WP-04` Prompt Spec completeness for editors
- `WP-05` first authoring/editor surfaces
- `WP-06` editing and review tooling surfaces
- `WP-07` dependable execution runtime surfaces
- `WP-08` verifiable inference runtime surfaces

What remains active:
- `WP-01` milestone alignment / close-the-gap work under `#886`
- `WP-09` through `WP-25`
- the explicit blocking alignment tranche listed in Section 1

Important rule:
- every work package or alignment sub-task must end in a concrete artifact, code path, validated doc, tool, demo, or review record
- no work unit should be satisfiable by description alone unless it is explicitly a bounded alignment deliverable in Section 1

## Working Rules

- Canonical tracker mapping follows [MILESTONE_ISSUE_RECONCILIATION_v0.85.md](/Users/daniel/git/agent-design-language/docs/milestones/v0.85/MILESTONE_ISSUE_RECONCILIATION_v0.85.md).
- Public record architecture follows [STRUCTURED_PROMPT_ARCHITECTURE.md](/Users/daniel/git/agent-design-language/docs/milestones/v0.85/STRUCTURED_PROMPT_ARCHITECTURE.md).
- Every active execution issue should have:
  - a Structured Task Prompt
  - a Structured Implementation Prompt
  - a Structured Output Record
- Alignment tasks must also produce concrete deliverables:
  - canonical docs
  - reference repairs
  - issue-graph cleanup
  - validation or proof-surface rules

## Remaining Execution Order

1. Close the blocking alignment tranche in Section 1, especially cognitive authority and the WBS/scope rewrite.
2. Continue core execution from `WP-09` through `WP-17`.
3. Execute integration/demo work in Section 3.
4. Finish review/release work in Section 4.

This order is intentional:
- the repo is already ahead of the old planning model
- the remaining alignment work exists to keep execution coherent, not to replace execution
- the WBS should now lead execution sequencing while the mid-flight review doc tracks closure progress

---

## Section 1 — Blocking Alignment Tranche

These items remain execution-blocking because they affect whether the rest of the milestone stays conceptually coherent and reviewable.

| Unit | Status | Canonical issue / anchor | Concrete deliverable | Proof / validation surface |
|---|---|---|---|---|
| WP-01 | ACTIVE | `#886` | updated milestone docs, issue graph, and close-the-gap planning set | docs no longer materially contradict issue graph or live execution reality |
| A1 | OPEN | `#886` follow-on | one authoritative cognitive loop document | exactly one canonical cognitive-loop authority remains |
| A2 | OPEN | `#886` follow-on | bounded affect terminology normalized across docs | no mixed emotion/affect terminology in canonical docs |
| A3 | OPEN | `#886` follow-on | stable cognitive stack/layer model | no fractional or conflicting layer numbering remains |
| A4 | OPEN | `#886` follow-on | unified instinct semantics wording | instinct is described consistently as bounded pressure/fast-prior behavior |
| A5 | OPEN | `#886` follow-on | repaired cross-document references | no dead or stale canonical references remain |
| A6 | OPEN | `#886` follow-on | reduced duplicate cognitive model content | no duplicate “authoritative” cognitive sections remain |
| B3 | PARTIAL | `#886` plus landed authoring/runtime work | milestone-wide artifact and proof-surface discipline | every active WP/issue requires a concrete artifact plus validation/proof path |
| B4 | PARTIAL | `#886` plus landed terminology/tooling work | terminology harmonization across docs, templates, task bundles, and public records | STP/SIP/SOR terminology is primary and lingering `card` vocabulary is cleanup-only |
| D1 | ACTIVE | `#927` | rewritten execution-facing WBS | WBS becomes the clearest single place to understand remaining work |
| D2 | ACTIVE | `#927` | reduced v0.85-critical scope framing inside the WBS/sprint plan | WBS and sprint plan reflect the narrower critical path already emerging in the repo |

Notes:
- `B3` is one of the highest-value remaining partial items because it determines whether later execution remains artifact- and proof-bound.
- `B4` now includes template naming, task-bundle naming, public artifact exposure, and lingering `card` vocabulary in tooling/records, not just doc wording.
- `D1` and `D2` are not speculative planning work. They are the doc surfaces catching up to the execution model already visible in the repo.

---

## Section 2 — Core Execution

These are the primary feature/runtime/tooling work packages. Closed items are retained here so the WBS shows real milestone progress rather than a hypothetical plan.

| WP | Status | Canonical issue | Concrete deliverable | Validation / proof surface | Dependencies |
|---|---|---|---|---|---|
| WP-02 Deterministic queue, checkpoint, and steering substrate | LANDED | `#674` | deterministic queue/checkpoint/resume + steering substrate | runtime behavior, replay-safe steering semantics, tests | WP-01 |
| WP-03 Cluster / distributed execution groundwork | LANDED | `#868` | bounded cluster/distributed groundwork | stable lease/ownership behavior, tests/artifacts | WP-01, WP-02 |
| WP-04 Prompt Spec completeness for editors | LANDED | `#716` | stronger prompt/spec contracts and authoring validation | prompt lint/validation surfaces and updated docs | WP-01 |
| WP-05 First authoring/editor surfaces | LANDED | `#870` | first real editor surface | in-repo editor artifact and reviewable usage surface | WP-01, WP-04 |
| WP-06 Editing and review tooling surfaces | LANDED | `#871` | reusable review/helper tooling | deterministic review helper surface and stable fixture/proof path | WP-04, WP-05 |
| WP-07 Dependable execution runtime surfaces | LANDED | `#872` | explicit dependable-execution runtime/artifact behavior | completed-phase output validation, refusal path, proof surface | WP-02, WP-03 |
| WP-08 Verifiable inference runtime surfaces | LANDED | `#873` | provenance/evidence-linked review outputs | deterministic provenance checker and good/bad fixtures | WP-02, WP-06, WP-07 |
| WP-09 Adaptive Execution Engine bounded progress | OPEN | `#874` | bounded AEE runtime progress with inspectable hooks/artifacts | tests, traces, or demoable strategy-loop behavior | WP-02, WP-07, WP-08 |
| WP-10 Deterministic hypothesis generation engine | OPEN | `#748` | deterministic Gödel hypothesis engine | code/tests/artifacts showing inspectable deterministic hypothesis generation | WP-09 |
| WP-11 Policy-learning and adaptive Godel loop | OPEN | `#749` | bounded policy-learning/adaptive loop | traces/tests/artifacts showing explicit adaptation behavior | WP-09, WP-10 |
| WP-12 Experiment prioritization and strategy confidence | OPEN | `#750` | prioritization and strategy-confidence surfaces | inspectable confidence/prioritization outputs and validation path | WP-10, WP-11 |
| WP-13 Cross-workflow learning and recursive improvement | OPEN | `#751` | bounded cross-workflow learning surfaces | explicit artifacts or demos of recursive-improvement behavior | WP-10, WP-11, WP-12 |
| WP-14 Promotion and eval-report artifact loop | OPEN | `#752` | promotion/eval artifact loop | emitted promotion/evaluation artifacts and validation path | WP-10 through WP-13 |
| WP-15 Affect engine core | OPEN | `#875` | minimal working affect engine | code/traces/state model with explicit update rules and proof path | WP-09, WP-10 through WP-14 |
| WP-16 Reasoning graph and affect integration | OPEN | `#876` | affect-linked reasoning graph surfaces | artifact/schema/example set proving the integration exists | WP-15 |
| WP-17 Affect-plus-Godel vertical slice | OPEN | `#877` | runnable affect-plus-Godel slice | bounded runnable demo showing affect changes reasoning/hypothesis behavior | WP-10 through WP-16 |

Core execution rule:
- every open item in this section must produce code, a tool, a runnable demo, an emitted artifact set, or a validated runtime behavior
- docs-only completion is not acceptable here unless explicitly stated in the issue and backed by a concrete validation reason

---

## Section 3 — Integration and Demos

These items prove the milestone as a coherent system rather than as isolated features.

| WP | Status | Canonical issue | Concrete deliverable | Validation / proof surface | Dependencies |
|---|---|---|---|---|---|
| WP-18 Demo program for v0.85 features | OPEN | `#878` with bounded-demo rule from `#743` | demo matrix/playbook and runnable milestone demos | runnable demos proving steering/queueing, authoring/review flow, and affect-plus-Godel behavior | WP-02 through WP-17 |

Integration rule:
- demos are evidence, not optional packaging
- each major milestone claim should have a bounded runnable or replayable proof surface where practical

---

## Section 4 — Review and Release

These items close the milestone with explicit review, release, and handoff evidence.

| WP | Status | Canonical issue | Concrete deliverable | Validation / proof surface | Dependencies |
|---|---|---|---|---|---|
| WP-19 Coverage / quality gate | OPEN | `#879` | release-quality evidence and documented exceptions if needed | coverage/test status plus explicit rationale for any exceptions | WP-02 through WP-18 |
| WP-20 Documentation consistency pass | OPEN | `#880` | milestone docs and canonical issue bodies made internally consistent | review pass showing docs/issues no longer materially contradict execution reality | WP-01 through WP-19 |
| WP-21 Internal review | OPEN | `#901` | internal review record | recorded findings and action items | WP-18 through WP-20 |
| WP-22 External review | OPEN | `#902` | external review record | recorded external findings and action items | WP-18 through WP-21 |
| WP-23 Review findings remediation | OPEN | `#903` | remediations or explicit deferrals | changed artifacts plus tracked deferrals/ownership | WP-21, WP-22 |
| WP-24 Release ceremony | OPEN | `#881` | release notes, validation evidence, tag/ceremony completion | auditable release record | WP-19, WP-20, WP-23 |
| WP-25 Next milestone planning | OPEN | `#882` | next milestone planning package | explicit next-milestone planning materials before v0.85 closure | WP-24 |

---

## Current Sprint Interpretation

The old four-sprint model is still useful as a phasing shorthand, but current execution should be understood like this:

- Sprint 1 foundation work: largely landed (`WP-02` through `WP-04`), with `WP-01` still carrying remaining alignment ownership
- Sprint 2 authoring/runtime tooling work: largely landed (`WP-05` through `WP-08`)
- current active execution queue: `WP-09` through `WP-17`, while the blocking alignment tranche in Section 1 is closed in parallel
- closeout queue: `WP-18` through `WP-25`

If this WBS and the issue graph disagree, treat the disagreement as a defect to fix immediately.

## Acceptance Criteria For This WBS

This WBS is in a good state when:
- it is the clearest single place to understand remaining v0.85 work
- every remaining unit has a concrete deliverable and a proof/validation surface
- the issue graph matches the WBS
- the sprint plan does not materially contradict the WBS
- the diagnostic mid-flight review doc is no longer required to understand what to execute next

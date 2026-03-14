# Work Breakdown Structure (WBS) — v0.8

## Metadata
- Milestone: `v0.8`
- Version: `0.8`
- Date: `2026-03-07`
- Owner: `Daniel Austin / Agent Logic`

## How To Use
- Break work into independently-mergeable issues.
- Keep each item measurable and testable.
- Include deliverables + dependencies + issue links.
- `WP-01` is **always** the milestone **design pass** (canonical docs + WBS + decisions + sprint plan + checklist).
- Reserve the final WPs for the release tail in this order: `WP-13` demos, `WP-14` quality/coverage gate, `WP-15` docs freeze + review convergence, `WP-16` release ceremony.
- For v0.8, the release tail must also include an explicit **3rd party review step** between the documentation freeze and the release ceremony.

## WBS Summary
v0.8 is the milestone where ADL moves from deterministic workflow substrate into **controlled experimentation and authoring**. The work packages are organized into four logical phases:

1. **Milestone design + schema spine** — finalize the design and define the core Gödel experiment artifacts.
2. **Experiment runtime + memory integration** — make the schemas usable by deterministic experiment workflows and ObsMem.
3. **Authoring surfaces + flagship demo** — make structured cards/prompts actionable and demonstrate the system with the Rust transpiler demo scaffold.
4. **Release tail** — demo matrix, quality gate, docs freeze, 3rd party review, and release ceremony.

The milestone must remain focused on a narrow backbone:

- ExperimentRecord / Mutation / EvaluationPlan / Evidence surfaces
- deterministic card → prompt → execution → review flow
- one flagship Rust transpiler demo
- release discipline compatible with v0.75’s deterministic standards

## Work Packages
| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | Design pass (milestone docs + planning) | Finalize canonical v0.8 docs: design, WBS, sprint plan, checklist, decisions, release notes/plan stubs, and align scope with v0.75 outcomes. | Complete the v0.8 planning doc set under `docs/milestones/v0.8/`. | v0.75 design complete | TBD |
| WP-02 | ExperimentRecord schema v1 | Define the stable experiment artifact for baseline/variant comparisons, decisions, and evidence references. | `ExperimentRecord` schema, examples, parser/serializer, tests. | WP-01 | #609 |
| WP-03 | Canonical Evidence View | Implement deterministic evidence canonicalization for traces, failure codes, artifact hashes, and verification results. | Canonical evidence module + tests + docs. | WP-01 | #610 |
| WP-04 | Mutation format v1 | Define bounded, policy-gated mutation records used by Gödel experiments. | Mutation schema + validation rules + examples. | WP-01 | #611 |
| WP-05 | EvaluationPlan v1 | Define deterministic evaluation plans and wire them to verification hooks. | EvaluationPlan schema/executor + tests. | WP-02, WP-03, WP-04 | #612 |
| WP-06 | Gödel experiment workflow template | Create the first ADL workflow template for baseline → mutation → verification → evidence → decision. | Canonical experiment workflow template + example artifacts. | WP-02, WP-03, WP-04, WP-05 | #613 |
| WP-07 | ObsMem indexing for run summaries + experiment records | Extend ObsMem to index run summaries and ExperimentRecords in a deterministic, privacy-safe way. | Indexing pipeline + query surfaces + tests. | WP-02, WP-03, WP-06 | #614 |
| WP-08 | ToolResult contract hardening | Harden ToolResult metadata and success/error semantics to support evidence, repair loops, and deterministic review surfaces. | Updated ToolResult contract + tests + docs. | WP-03 | #618 |
| WP-09 | Authoring surfaces v1 (structured cards / prompts) | Establish structured cards as first-class execution contracts and align prompt-generation surfaces. | Authoring surface spec + updated card/prompt docs + initial automation hooks. | WP-01, v0.75 DX work (#629, #630, #633, #634) | #517 |
| WP-10 | Prompt automation + reviewer-ready execution flow | Convert structured card surfaces into deterministic prompt-generation and reviewer-compatible validation flow. | Card parser / prompt generator / reviewer-facing output integration. | WP-09, WP-08 | TBD |
| WP-11 | Rust transpiler fixture + workflow scaffold | Create the minimal fixture and deterministic mapping scaffold for the Rust transpiler demo surface. | Workflow fixture + transpiler scaffold + checked-in runtime skeleton artifacts. | WP-06, WP-09 | TBD |
| WP-12 | Rust transpiler verification + bounded evidence reporting | Add deterministic verification output and bounded adaptive-execution evidence fields to the transpiler demo scaffold. | Runnable mapping-check demo + verification artifact + scope docs. | WP-05, WP-08, WP-10, WP-11 | TBD |
| WP-13 | Demo matrix + integration demos | Integrate the flagship demos into a canonical demo matrix: Gödel experiment demo, Rust transpiler demo, and any required supporting demos. | Demo matrix doc, runnable commands, validated artifacts, milestone demo surfaces. | WP-06, WP-07, WP-10, WP-12 | TBD |
| WP-14 | Coverage / quality gate (ratchet + exclusions) | Establish or extend milestone quality gates so the new v0.8 surfaces meet deterministic coverage and validation expectations. | Coverage/quality policy, gate scripts/CI updates, documented exclusions. | WP-08 through WP-13 | TBD |
| WP-15 | Documentation pass + review convergence (docs freeze before review) | Finalize and freeze the v0.8 docs package, align all commands/examples, then prepare for formal review. Documentation must be frozen before review begins. | Frozen v0.8 milestone docs package + review handoff artifact. | WP-13, WP-14 | TBD |
| WP-16 | Release ceremony (after 3rd party review) | Perform final validation, ensure 3rd party review findings are resolved or deferred explicitly, tag the release, and publish notes. | Final validation artifact, release tag, release notes, cleanup. | WP-15 + explicit 3rd party review step + review fixes | TBD |

## Sequencing
- Phase 1: **Milestone design + schema spine** — WP-01 through WP-05.
- Phase 2: **Experiment runtime + memory integration** — WP-06 through WP-08.
- Phase 3: **Authoring surfaces + flagship demo** — WP-09 through WP-12.
- Phase 4: **Release tail** — WP-13 through WP-16, with the required sequence:
  - demos complete
  - quality gate complete
  - docs pass complete and frozen
  - **3rd party review step executed**
  - review findings fixed or explicitly deferred
  - release ceremony

## Acceptance Mapping
- WP-01 (Design pass) -> v0.8 scope, risks, and validation plan are explicit and consistent across milestone docs.
- WP-02 -> ExperimentRecord schema exists, is versioned, and has at least one example and round-trip validation path.
- WP-03 -> Canonical Evidence View exists and excludes volatile fields deterministically.
- WP-04 -> Mutation format is bounded, policy-gated, and machine-readable.
- WP-05 -> Evaluation plans can run deterministic checks and emit reproducible results.
- WP-06 -> Gödel experiments can be expressed as ordinary ADL workflows with explicit evidence and decisions.
- WP-07 -> ObsMem can index run summaries and ExperimentRecords deterministically and privacy-safely.
- WP-08 -> ToolResult surfaces are strong enough for experiment evidence and repair loops.
- WP-09 -> Structured cards/prompts act as first-class execution contracts.
- WP-10 -> Card → prompt → execution → reviewer flow is deterministic and machine-friendly.
- WP-11 -> Rust transpiler fixture and workflow scaffold exist and are runnable.
- WP-12 -> Rust transpiler demo scaffold demonstrates deterministic mapping verification and bounded evidence reporting without claiming a full migration engine.
- WP-13 (Demos) -> Canonical demo matrix exists and flagship demos run reproducibly.
- WP-14 (Quality gate) -> Milestone quality gate is documented, enforced, and reviewable.
- WP-15 (Docs/review) -> v0.8 docs are complete, aligned, and frozen before review begins.
- WP-16 (Release ceremony) -> 3rd party review is complete, findings are handled, and release artifacts are published cleanly.

## Exit Criteria
- Every in-scope requirement maps to at least one WBS item.
- Every WBS item has an owner, issue reference (or explicit TBD), and concrete deliverable.
- Dependency order is explicit enough to execute deterministically.
- The 3rd party review step is included explicitly between docs freeze and release ceremony.
- The Rust transpiler demo is clearly assigned to v0.8, not silently pulled backward into v0.75.

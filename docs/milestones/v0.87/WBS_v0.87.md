# Work Breakdown Structure (WBS): v0.87

## Metadata
- Milestone: `v0.87`
- Version: `0.87`
- Date: `2026`
- Owner: `adl`

## WBS Summary
`v0.87` is the milestone where ADL consolidates the bounded cognitive system from `v0.86` into a coherent, deterministic, and externally credible substrate. The work is organized around four core implementation bands—trace, provider portability, shared memory, and operational/control-plane stability—followed by the standard demo, quality, docs/review, release tail, and next-milestone planning handoff.

This milestone is intentionally substrate-heavy. It should improve:
- cross-surface coherence (`contracts -> execution -> trace -> review -> docs`)
- provider/model correctness and portability
- shared observable memory foundations
- deterministic operational workflows and PR tooling
- reviewer-facing proof and demo surfaces

The WBS below preserves mergeable slices, explicit dependencies, and a clean release tail.

## Work Packages
| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | Design pass (milestone docs + planning) | Fill and align canonical `v0.87` milestone docs: vision, design, WBS, sprint, checklist, demo matrix, release plan, release notes, decisions, and feature-doc index. | Canonical `docs/milestones/v0.87/` doc set aligned to roadmap and substrate scope. | none | #1292 |
| WP-02 | Trace v1 schema + event model | Define and implement the first authoritative trace substrate for `v0.87`, including stable event naming, canonical event schema, and the minimum event set needed for reconstruction and review. | Trace v1 schema, event taxonomy, and implementation surfaces for stable structured events. | WP-01 | #1293 |
| WP-03 | Trace instrumentation + artifact linkage | Instrument the runtime and adjacent tooling so major decisions and outputs emit trace events and trace links cleanly to artifacts/proof surfaces. | Runtime trace emission aligned to actual execution, with artifact ↔ trace linkage. | WP-02 | #1294 |
| WP-04 | Provider / transport substrate v1 | Redesign provider handling around explicit vendor / transport / model separation, stable `model_ref`, provider-model mapping, and adapter boundaries for common providers. | Provider/transport substrate v1 with deterministic configuration and extensible adapter model. | WP-01 | #1295 |
| WP-05 | Provider portability + config compatibility | Thread the new provider substrate through real config surfaces, preserve compatibility where needed, and ensure agents can target common providers without brittle provider-native strings in core authoring surfaces. | Working provider portability path plus backward-compatible profile expansion where required. | WP-04 | #1296 |
| WP-06 | Shared ObsMem foundation | Establish the first shared-memory substrate across runs/surfaces with bounded indexing, retrieval, and storage discipline. This is a foundation layer, not full social memory. | Shared ObsMem base interfaces and implementation surface for cross-run/shared retrieval. | WP-01 | #1297 |
| WP-07 | Trace ↔ memory coherence | Align shared-memory entries with trace and execution truth so retrieval and persisted context can be explained, reviewed, and tied back to actual events. | Trace-aware shared-memory behavior and documented coherence rules. | WP-03, WP-06 | #1298 |
| WP-08 | Operational skills substrate | Implement the first operational skill substrate, including bounded invocation, common output shape, and at least the first real workflow skills (for example preflight/review-oriented surfaces). | Operational skills v1 with deterministic invocation and structured outputs. | WP-01, WP-03 | #1299 |
| WP-09 | PR tooling / control-plane consolidation | Continue moving workflow ownership into the canonical control plane so PR/worktree/card behavior is less fragile, less shell-dependent, and more deterministic. | Strengthened Rust-/control-plane-owned workflow surfaces with thinner shell wrappers. | WP-01 | #1300 |
| WP-10 | Tooling hardening + workflow stability | Harden day-to-day developer workflow around worktrees, repo-root handling, validation boundaries, and automation expectations so the substrate is credible in practice. | Stable operational workflow surfaces for daily use and milestone closeout. | WP-08, WP-09 | #1301 |
| WP-11 | Review-surface formalization | Convert review into a canonical structured output surface: findings, impact, triggers, evidence, fix direction, system-level assessment, and action plan. | Standardized review/verification output contract and at least one real implementation surface using it. | WP-03, WP-08 | #1302 |
| WP-12 | Documentation canonicalization + feature index | Align milestone docs and feature-doc index with implemented `v0.87` behavior, dependencies, and proof surfaces without inflating scope. | Canonical `v0.87` docs and feature-doc map that truthfully reflect implementation. | WP-02, WP-05, WP-07, WP-11 | #1345 |
| WP-13 | Demo matrix + integration demos | Define and implement the milestone’s primary proof surfaces, including substrate demos for trace/provider/shared-memory/skills and the planned PR Demo preparation surfaces for later milestones. | `v0.87` demo matrix plus runnable integration demos with clear proof claims. | WP-03, WP-05, WP-07, WP-08, WP-11 | #1346 |
| WP-14 | Coverage / quality gate (ratchet + exclusions) | Enforce truthful quality posture for the milestone, including tests, validators, coverage/ratchet posture, and concrete command surfaces for auditing substrate correctness. | Final `v0.87` quality/coverage gate record with explicit exceptions if any. | WP-02 through WP-13 | #1347 |
| WP-15 | Docs + review pass (repo-wide alignment) | Converge milestone docs, proof surfaces, review artifacts, and entry-point docs so an internal/external reviewer can understand the implemented `v0.87` substrate truthfully. | Reviewed and aligned docs/review surface package for `v0.87`. | WP-12, WP-13, WP-14 | #1348 |
| WP-15A | 3rd-party review | Conduct external / 3rd-party review of the `v0.87` milestone, capture findings, and ensure all issues are either resolved or explicitly dispositioned before release closeout. | External review findings and disposition record for milestone closeout. | WP-15 | #1349 |
| WP-16 | Release ceremony (final validation + tag + notes + cleanup) | Perform final release-tail work for `v0.87`: validation evidence, checklist/release-note alignment, closeout record, and clean handoff into the next roadmap slice. | `v0.87` release-closeout package with final validation and milestone handoff. | WP-15 | #1350 |
| WP-17 | Next milestone planning (`v0.87.1`) | Prepare the canonical tracked planning package for `v0.87.1` before `v0.87` closes, including the next milestone shell, sprint/WBS framing, and the initial docs needed for runtime-completion work to begin from a coherent public surface. | Canonical `v0.87.1` milestone shell and planning package ready before `v0.87` closeout. | WP-15, WP-16 | #1354 |
| WP-18 | Review findings remediation | Resolve accepted internal and 3rd-party review findings, or record bounded explicit deferrals with owners, before final release closeout. | Truthful remediation/disposition record for accepted review findings ahead of milestone closeout. | WP-15, WP-15A | #1414 |

## Sequencing
- Phase 1: Canonical planning + substrate definition (`WP-01` through `WP-04`)
- Phase 2: Shared substrate implementation and workflow hardening (`WP-05` through `WP-11`)
- Phase 3: Canonical docs, demos, quality gate, review alignment, remediation, release tail, and next-milestone handoff (`WP-12` through `WP-18`)

## Acceptance Mapping
- WP-01 (Design pass) -> Canonical milestone docs are filled, internally consistent, and aligned to the roadmap-defined `v0.87` substrate.
- WP-02 -> Trace v1 has an explicit schema, stable event vocabulary, and a bounded but authoritative event model.
- WP-03 -> Trace events are emitted from real execution/control points and link coherently to artifacts/proof surfaces.
- WP-04 -> Provider/transport substrate v1 is explicit, deterministic, and no longer conflates vendor, transport, and model identity.
- WP-05 -> Common providers can be targeted through the new substrate without brittle core-surface dependence on provider-native strings.
- WP-06 -> Shared ObsMem foundation exists as a real shared-memory layer, not just isolated local memory behavior.
- WP-07 -> Shared-memory behavior is explainable through trace/execution history and does not drift silently from runtime truth.
- WP-08 -> Operational skills have a real invocation substrate and structured outputs, with at least initial workflow-oriented skills implemented.
- WP-09 -> PR/control-plane behavior is more centralized, deterministic, and less shell-fragile than before.
- WP-10 -> Daily workflow and milestone-closeout tooling are reproducible, worktree-safe, and minimally surprising.
- WP-11 -> Review outputs are structured, evidence-bearing, and suitable for real internal/external review surfaces.
- WP-12 -> Canonical docs and feature index truthfully describe the implemented `v0.87` substrate and its proof surfaces.
- WP-13 (Demos) -> `v0.87` has clear integration demos proving trace, provider portability, shared memory, and operational substrate behavior.
- WP-14 (Quality gate) -> The milestone has a truthful, auditable quality/coverage posture with explicit command surfaces and justified exceptions.
- WP-15 (Docs/review) -> Docs, proof surfaces, and review artifacts converge into a reviewer-legible, contradiction-free package.
- WP-15A (3rd-party review) -> External review findings are captured and every finding is either resolved or explicitly dispositioned before release closeout.
- WP-16 (Release ceremony) -> Final validation, release-tail docs, and milestone handoff are explicit, truthful, and auditable.
- WP-17 (Next milestone planning) -> `v0.87.1` canonical milestone docs exist before `v0.87` closeout and provide a coherent tracked starting point for runtime-completion work.
- WP-18 (Review remediation) -> Accepted review findings are either remediated or explicitly deferred with ownership before milestone closeout.

## Exit Criteria
- Every in-scope `v0.87` requirement maps to at least one WBS item.
- Every WBS item has a concrete deliverable and explicit dependency order.
- The four major substrate bands—trace, provider, shared memory, and operational/control-plane stability—are all represented directly in the WBS.
- The release tail remains bounded and downstream of implementation truth.
- The next milestone planning package exists before `v0.87` is considered fully closed.

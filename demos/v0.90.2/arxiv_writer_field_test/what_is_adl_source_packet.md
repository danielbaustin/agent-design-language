# Source Packet: What Is ADL?

## Metadata

- Packet: `what_is_adl_source_packet`
- Intended paper: `What Is ADL?`
- Issue: `#2272`
- Packet status: review input
- Publication status: not submitted and not publication-ready

## Core Problem Statement

ADL needs a crisp technical front-door paper that explains what the project is
now: a deterministic agent-runtime and orchestration platform with a Rust
runtime, explicit workflow contracts, trace artifacts, review surfaces, bounded
demo proof, and a milestone-driven path toward richer runtime and governance
substrates.

## Contribution Claim

The paper may claim that ADL is currently a repository-backed engineering
system for inspectable agent workflows, not merely a prompt pattern or planning
notation.

This claim is supported by the tracked repository overview, feature list,
milestone packages, demo surfaces, and runtime documentation. It must be framed
as project/system contribution, not as externally benchmarked superiority.

## Source Evidence

### `README.md`

Supported facts:

- ADL is described as a deterministic orchestration system for AI workflows.
- The repo includes a language, Rust runtime, CLI, review surfaces, and
  milestone proof packages.
- ADL emphasizes explicit contracts, bounded runtime behavior, durable
  artifacts, and repository-visible proof.
- Current user-facing entrypoints include runtime demos, milestone proof
  packages, and review surfaces.

### `docs/planning/ADL_FEATURE_LIST.md`

Supported facts:

- The feature list treats ADL as a deterministic agent-runtime and
  orchestration platform.
- Implemented baselines include deterministic workflow execution, Rust runtime
  and CLI, trace/artifact review surfaces, operational skills, task-bundle
  workflow, review/validation surfaces, bounded Godel-style experimentation,
  cognitive proof paths, and milestone demo packages.
- Runtime v2, CSM Observatory, CodeBuddy, Aptitude Atlas, identity, governance,
  and economics are explicitly separated into active, planned, or deferred
  bands.

### `docs/milestones/v0.90/README.md`

Supported facts:

- v0.90 made ADL capable of supervising long-lived agents with bounded cycles,
  durable continuity handles, operator safety controls, and concrete demo
  evidence.
- v0.90 explicitly did not claim the full identity/capability substrate.
- v0.90 included stock-league demo evidence, milestone compression,
  repo-visibility, coverage, and review-tail work.

### `docs/milestones/v0.90.1/README.md`

Supported facts:

- v0.90.1 completed the Runtime v2 foundation prototype line.
- Its scope included kernel services, provisional citizen records, snapshots,
  manifold links, invariant violation artifacts, operator controls, and CSM
  Observatory surfaces.
- Its out-of-scope list explicitly excludes first true Godel-agent birthday,
  full moral/emotional civilization, full identity/capability rebinding, full
  cross-polis migration, and full red/blue/purple security ecology.

### `demos/v0.89.1/arxiv_manuscript_workflow_demo.md`

Supported facts:

- ADL already has a bounded arXiv manuscript workflow packet for the initial
  three-paper program.
- The three-paper slate is `What Is ADL?`, `Godel Agents and ADL`, and
  `Cognitive Spacetime Manifold`.
- That earlier demo proves source-packet, role/order, review-gate, and
  manuscript-status shape without claiming final arXiv submission.

### Local backlog evidence

Source: `.adl/docs/TBD/publication/ARXIV_PAPER_PROGRAM_PLAN.md` from the local
control-plane backlog.

Supported planning facts:

- The recommended first paper is `What Is ADL?`.
- The paper should explain ADL as it actually is now.
- The paper should separate current runtime surfaces from roadmap claims.
- Paper Sonata may later assist manuscript assembly, but this issue is not the
  Paper Sonata expansion issue.

This local backlog source should not be cited as public release truth. It is
used here only to justify issue scope and source-packet selection.

## Allowed Paper Claims

| Claim | Status | Evidence |
| --- | --- | --- |
| ADL is a deterministic orchestration system with a Rust runtime and CLI. | SUPPORTED | `README.md`; `docs/planning/ADL_FEATURE_LIST.md` |
| ADL uses explicit workflow contracts, trace artifacts, and review surfaces. | SUPPORTED | `README.md`; `docs/planning/ADL_FEATURE_LIST.md` |
| ADL has milestone proof packages and bounded demos. | SUPPORTED | `README.md`; `demos/README.md`; milestone docs |
| ADL has implemented baseline operational skills and task-bundle workflows. | SUPPORTED | `docs/planning/ADL_FEATURE_LIST.md` |
| Runtime v2 foundation work is real but bounded. | SUPPORTED | `docs/milestones/v0.90.1/README.md` |
| ADL has proven external benchmark superiority over other agent frameworks. | REMOVE_OR_WEAKEN | No source packet evidence |
| ADL has completed first true identity-bearing citizen birth. | REMOVE_OR_WEAKEN | Explicitly out of scope in v0.90.1 |
| ADL eliminates hallucination or execution error. | REMOVE_OR_WEAKEN | No source packet evidence |

## Citation And Evidence Gaps

- External related work on workflow engines, agent frameworks, orchestration
  systems, and traceability is needed before submission.
- External citations for deterministic replay, provenance, and software
  engineering reviewability are needed before submission.
- Comparative claims against other systems require a separate sourced related
  work pass.
- Empirical reliability or productivity claims require experiments not present
  in this packet.

## Forbidden Claims

- Do not claim arXiv submission.
- Do not claim peer review or acceptance.
- Do not claim the first true Godel-agent birthday has happened.
- Do not claim full identity, governance, economics, or moral/emotional
  civilization layers are complete.
- Do not claim benchmark wins or industry adoption.

## Target Audience

Technical readers who need a formal but approachable entry point into ADL:

- AI workflow and agent-system engineers
- reviewers evaluating whether ADL is more than prompt choreography
- researchers interested in inspectable, contract-driven agent runtimes

## Recommended Section Order

1. Introduction: why inspectable agent workflows need explicit runtime truth
2. ADL as a contract-first orchestration system
3. Runtime model: deterministic plans, bounded execution, traces, and artifacts
4. Review and proof surfaces: demos, milestone packages, and issue lifecycle
5. Implemented baseline and active boundaries
6. Limitations and future work

## Boundary

This packet is evidence for drafting. It is not a final paper, a submission
package, or author-approved publication text.

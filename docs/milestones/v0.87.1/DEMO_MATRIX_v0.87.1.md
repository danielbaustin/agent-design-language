# Demo Matrix - v0.87.1

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Date: `2026-04-06`
- Owner: `Daniel Austin`
- Related issues / work packages: `#1354`, `WP-13`

## Purpose
Define the canonical milestone demo program: which bounded demos exist, which milestone claims they prove, how to run them, and what artifacts or proof surfaces reviewers should inspect.

## Status

This is the planned proof program for a large runtime-completion milestone. The runtime demo set is expected to include roughly a dozen bounded demos spanning execution environment, lifecycle, trace alignment, resilience, operator surfaces, and reviewer entry artifacts.

## How To Use
- Use this document for runnable milestone evidence, not for broad feature brainstorming.
- Keep demo rows and per-demo sections aligned so a reviewer can move from summary -> execution -> proof surface without reconstructing context by hand.
- Prefer bounded, replayable, copy/paste-friendly commands over aspirational demo descriptions.
- If a milestone claim cannot yet be shown through a runnable demo, say so explicitly and record the substitute proof surface.
- Keep names stable across milestones where practical so comparisons remain easy.
- If a section is not relevant, include a one-line rationale instead of deleting it.

## Scope

In scope for `v0.87.1`:
- runtime environment demos
- lifecycle and execution-boundary demos
- trace-aligned runtime execution demos
- resilience, restartability, and failure-handling demos
- operator/review-surface demos
- mapping between milestone claims and bounded demo surfaces

Out of scope for `v0.87.1`:
- broad speculative demos not tied to runtime-completion claims
- later cognitive demos intended for `v0.88+`

## Runtime Preconditions

Working directory:

```bash
cd agent-design-language
```

Baseline repository/runtime validation:

```bash
cargo build
```

Additional environment / fixture requirements:
- local Rust toolchain installed
- demo scripts should be runnable from the repository root
- each demo should write stable artifacts or review surfaces under documented locations
- demos should prefer local or mocked providers unless a specific external dependency is part of the milestone claim

## Related Docs
- Design contract: `docs/milestones/v0.87.1/DESIGN_v0.87.1.md`
- WBS / milestone mapping: `docs/milestones/v0.87.1/WBS_v0.87.1.md`
- Sprint / execution plan: `docs/milestones/v0.87.1/SPRINT_v0.87.1.md`
- Release / checklist context: `docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md`
- Other proof-surface docs: Trace v1 artifacts and runtime outputs from v0.87

## Provider Family Demo / Test Issue Map

These family-level issues seed the provider proof surfaces that later demo work can implement:

| Provider family | Scope | Issue |
|---|---|---|
| local Ollama | bounded local provider demo plus acceptance coverage for `ollama` / `local_ollama` | `#1485` |
| bounded HTTP | bounded generic remote HTTP demo plus acceptance coverage for `http` / `http_remote` | `#1486` |
| mock | no-network mock provider demo plus acceptance coverage | `#1487` |
| ChatGPT | `chatgpt:` family demo plus acceptance coverage using the current setup flow | `#1488` |

## Demo Coverage Summary

Use this table as the fast review surface for milestone coverage.

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
|---|---|---|---|---|---|---|---|
| D1 | Runtime Environment Bring-Up | `WP-02` runtime environment completion | `adl/tools/demo_v0871_runtime_environment.sh` | `.adl/runtime_environment.json` plus a bounded `.adl/runs/<run_id>/` artifact set | Runtime environment initializes cleanly with documented contracts | Stable env inputs should preserve artifact shape | PLANNED |
| D2 | Lifecycle Phases And Boundaries | `WP-03` execution boundaries and lifecycle | `adl/tools/demo_v0871_lifecycle.sh` | lifecycle phase trace / summary | `init -> execute -> complete/teardown` is explicit and reviewable | Fixed scenario should preserve lifecycle phase ordering | PLANNED |
| D3 | Trace-Aligned Runtime Execution | `WP-04` trace-aligned runtime execution | `adl/tools/demo_v0871_trace_runtime.sh` | `logs/trace_v1.json`, `run_summary.json`, and trace bundle export surfaces | Runtime actions map coherently to trace events, linked artifacts, and replay bundle outputs | Replay should preserve execution-to-trace shape | PLANNED |
| D4 | Local Failure Handling | `WP-05` local runtime resilience | `adl/tools/demo_v0871_resilience_failure.sh` | `run_status.json`, `run_summary.json`, and `logs/trace_v1.json` | Failure is bounded, explained, and leaves inspectable artifacts | Same induced failure should preserve failure classification | PLANNED |
| D4A | Shepherd Preservation And Recovery | `WP-05`, `WP-07` Shepherd preservation + continuity discipline | `adl/tools/demo_v0871_shepherd_recovery.sh` | `run_status.json`, `pause_state.json`, and `logs/trace_v1.json` | Interrupted work is preserved, resumed, or dispositioned under explicit runtime rules | Fixed interruption scenario should preserve preservation and recovery classification | PLANNED |
| D5 | Restartability And Recovery | `WP-05`, `WP-07` resilience + state discipline | `adl/tools/demo_v0871_restartability.sh` | restart/recovery artifact set | Bounded run can resume or restart under documented rules | Restart behavior should remain stable under fixed state | PLANNED |
| D6 | Operator Invocation Surface | `WP-06` operator surfaces | `bash adl/tools/demo_v0871_operator_surface.sh` | `artifacts/v0871/operator_surface/runtime/runs/v0-4-demo-deterministic-replay/run_summary.json` | Operator entrypoints are clear, stable, and reviewer-usable, with one canonical runtime-root proof set | Same command contract should preserve invocation shape and artifact naming | READY |
| D7 | Runtime State / Persistence Discipline | `WP-07` state / persistence discipline | `bash adl/tools/demo_v0871_runtime_state.sh` | `artifacts/v0871/runtime_state/runtime/runs/v0-6-hitl-pause-demo/run_status.json` | State is inspectable, bounded, and cleaned up deterministically across paused and completed runs | Stable inputs should preserve persistence classification, cleanup disposition, and state artifact inventory | READY |
| D8 | Review Surface Walkthrough | `WP-08` runtime review surfaces | `bash adl/tools/demo_v0871_review_surface.sh` | `artifacts/v0871/review_surface/demo_manifest.json` and `artifacts/v0871/review_surface/README.md` | Reviewer can locate primary D6 and D7 proof surfaces from one entrypoint | Manifest layout, reviewer guidance, and package ordering remain stable | READY |
| D9 | Integrated Runtime Path | `WP-02` through `WP-08` integrated runtime completion | `adl/tools/demo_v0871_integrated_runtime.sh` | integrated runtime artifact set | One run demonstrates the authoritative runtime path end-to-end | Replay judged by control-path and artifact-shape stability | PLANNED |
| D10 | Docs-To-Runtime Consistency Check | `WP-09`, `WP-15` docs/review convergence | `adl/tools/demo_v0871_docs_review.sh` | reviewer entry surfaces | Reviewer can move from docs to runtime proof without contradiction | Navigation and proof mapping should remain stable | PLANNED |
| D11 | Quality Gate Walkthrough | `WP-14` quality gate | `adl/tools/demo_v0871_quality_gate.sh` | quality-gate record | Tests, validators, and coverage posture are reviewable in one place | Same repo state should preserve gate outcome | PLANNED |
| D12 | Release Review Package | `WP-16` through `WP-20` review/remediation/planning/release tail | `adl/tools/demo_v0871_release_review_package.sh` | release review package | Review, remediation, planning, and release artifacts are coherent and navigable | Package layout and key entrypoints remain stable | PLANNED |
| D13 | Claude + ChatGPT Tea Discussion | bounded multi-agent runtime discussion proof | `bash adl/tools/demo_v0871_multi_agent_discussion.sh` | `artifacts/v0871/multi_agent_discussion/transcript.md` | Reviewer can inspect five explicit turns, two named agents, and the paired runtime trace/summaries | Fixed shim outputs should preserve transcript shape and turn ordering | READY |

Status guidance:
- `PLANNED` = intended but not yet validated
- `READY` = runnable and locally validated
- `BLOCKED` = known dependency or missing proof surface
- `LANDED` = milestone evidence exists and is ready for review

## Coverage Rules
- Every major milestone claim should map to a runnable demo or an explicit alternate proof surface.
- Every demo should name one primary proof surface that a reviewer can inspect directly.
- Commands should be copy/paste-ready and should not require private local state.
- Success signals should say what to check, not just “command exits 0”.
- Determinism / replay notes should explain how stability is judged.

## Demo -> Feature Mapping
- `D1` -> `ADL_RUNTIME_ENVIRONMENT.md`, `ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md`
- `D2` -> `AGENT_LIFECYCLE.md`, `EXECUTION_BOUNDARIES.md`
- `D3` -> `ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md`, `AGENT_LIFECYCLE.md`, Trace v1 artifact and replay-bundle surfaces
- `D4` -> `LOCAL_RUNTIME_RESILIENCE.md`
- `D4A` -> `SHEPHERD_RUNTIME_MODEL.md`, `LOCAL_RUNTIME_RESILIENCE.md`
- `D5` -> `LOCAL_RUNTIME_RESILIENCE.md`, `SHEPHERD_RUNTIME_MODEL.md`, `AGENT_LIFECYCLE.md`
- `D6` -> `ADL_RUNTIME_ENVIRONMENT.md`, `EXECUTION_BOUNDARIES.md`
- `D7` -> `AGENT_LIFECYCLE.md`, `SHEPHERD_RUNTIME_MODEL.md`
- `D8` -> `ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md`, `SHEPHERD_RUNTIME_MODEL.md`
- `D9` -> all promoted `v0.87.1` runtime feature docs
- `D10` -> `FEATURE_DOCS_v0.87.1.md` and all promoted `v0.87.1` runtime feature docs
- `D11` -> milestone review and validation surfaces derived from the promoted runtime feature set
- `D12` -> review, remediation, planning, and release surfaces for the runtime milestone
- `D13` -> bounded multi-agent runtime demo evidence for later conversation/runtime follow-on work

## Demo Details

Per-demo detail sections will be filled as the runtime milestone opens. This matrix already defines the bounded demo inventory the milestone is expected to implement and review.

### D8) Review Surface Walkthrough

Description:
- Provides one obvious reviewer entry point for the milestone runtime proof set.
- Bundles the canonical D6 and D7 proof surfaces and points to the primary runtime artifacts.

Commands to run:

```bash
bash adl/tools/demo_v0871_review_surface.sh
adl tooling review-runtime-surface --review-root artifacts/v0871/review_surface
```

Expected artifacts:
- `artifacts/v0871/review_surface/demo_manifest.json`
- `artifacts/v0871/review_surface/README.md`
- `artifacts/v0871/review_surface/index.txt`
- `artifacts/v0871/operator_surface/runtime/runs/v0-4-demo-deterministic-replay/run_summary.json`
- `artifacts/v0871/runtime_state/runtime/runs/v0-6-hitl-pause-demo/run_status.json`

Primary proof surface:
- `artifacts/v0871/review_surface/demo_manifest.json`

Secondary proof surfaces:
- `artifacts/v0871/review_surface/README.md`
- `artifacts/v0871/operator_surface/README.md`
- `artifacts/v0871/runtime_state/README.md`

Expected success signals:
- One command gives the reviewer a stable starting point.
- The manifest points to the D6 and D7 primary proof artifacts.
- The README explicitly tells the reviewer to inspect D6 first.

Determinism / replay notes:
- The manifest layout, reviewer guidance, and referenced artifact names must remain stable.
- This is a review-oriented proof surface; it depends on the bounded correctness of D6 and D7 rather than replacing their subsystem validations.

## Cross-Demo Validation

Required baseline validation:

```bash
cargo build
```

Cross-demo checks:
- the integrated runtime path must be consistent with the specialized demo rows
- reviewer entry surfaces must point to real demo proof roots
- the runtime demo set should remain bounded, deterministic, and reviewable

Failure policy:
- if the runtime demos do not prove the milestone claims truthfully, the milestone cannot be considered complete

## Determinism Evidence

Evidence directory / run root:
- runtime demo artifact roots will be defined per demo as implementation lands
- the canonical runtime root and runs root are established by `adl::runtime_environment::RuntimeEnvironment`

Repeatability approach:
- runtime control-path shape, artifact naming, and reviewer entry surfaces should remain stable for fixed inputs

Normalization rules:
- none required

Observed results summary:
- planned; to be filled with real demo outcomes as `v0.87.1` lands

## Reviewer Sign-Off Surface

For each demo, the reviewer should be able to answer:
- What milestone claim does this demo prove?
- Which command should be run first?
- Which artifact or trace is the primary proof surface?
- What deterministic or replay guarantee is being claimed?
- What caveats or substitutions apply?

Review owners:
- Daniel Austin
- 3rd party reviewer

Review status:
- pending

## Notes
- `v0.87.1` is expected to ship a substantial runtime demo program rather than a placeholder matrix.

## Exit Criteria
- The milestone’s major claims are mapped to bounded demos or explicit alternate proof surfaces.
- Each demo has runnable commands, expected artifacts, and a clear success signal.
- Determinism / replay expectations are explicit where required.
- A reviewer can inspect the matrix and locate the primary proof surface for each demo without extra reconstruction work.

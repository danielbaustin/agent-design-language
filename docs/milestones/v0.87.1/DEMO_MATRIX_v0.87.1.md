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

This is the canonical bounded proof program for the runtime-completion milestone. The runtime demo set now includes runnable bounded demos spanning execution environment, lifecycle, trace alignment, resilience, operator surfaces, review-tail packaging, and reviewer entry artifacts.

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
- Shared provider proof governance: `docs/tooling/PROVIDER_DEMO_SURFACES.md`
- Other proof-surface docs: Trace v1 artifacts and runtime outputs from v0.87

## Demo Issue Inventory

Use this inventory as the canonical reviewer map for demo work completed during `v0.87.1`. If a demo issue is not listed here or linked from a row below, reviewers should treat that as a documentation bug.

| Issue | Demo / proof surface | Matrix location | Status |
|---|---|---|---|
| `#1467` | Codex CLI + Ollama operational-skills demo | Adjacent operational-skills demo; see `demos/v0.87.1/codex_ollama_operational_skills_demo.md` and `bash adl/tools/demo_codex_ollama_operational_skills.sh` | READY |
| `#1468` | provider demo/test issue and card set | Provider family map planning lineage | READY |
| `#1485` | local Ollama provider demo + acceptance test | Provider family map, D0 suite provider package | READY |
| `#1486` | bounded HTTP provider demo + acceptance test | Provider family map, D0 suite provider package | READY |
| `#1487` | mock provider demo + acceptance test | Provider family map, D0 suite provider package | READY |
| `#1488` | ChatGPT provider demo + acceptance test | Provider family map, D0 suite provider package | READY |
| `#1490` | Claude + ChatGPT multi-agent discussion demo | D13 | READY |
| `#1491` | bounded Claude + ChatGPT discussion workflow demo | D13 | READY |
| `#1500` | first-class Claude provider-family parity with ChatGPT profiles | D13 and D13L provider-family support | READY |
| `#1501` | conversation-native multi-agent turn primitives | D13 and D13L turn metadata / runtime primitive support | READY |
| `#1502` | transcript artifact contract for multi-agent discussion demos | D13 and D13L transcript contract / validator support | READY |
| `#1507` | shared provider demo harness | Provider family map shared wrapper helper | READY |
| `#1508` | provider demo proof-surface governance | Provider family map shared doc ownership | READY |
| `#1509` | mock provider family runnable in provider substrate | Provider family map mock row, D0 suite provider package | READY |
| `#1518` | scattered run artifact inventory and consolidation | Trace/archive review context for generated demo outputs | READY |
| `#1519` | trace run manifest and provenance capture | D6, D13, D13L runtime proof surfaces include manifest/provenance context | READY |
| `#1520` | canonical trace archive routing for demo/provider traces | Provider wrappers and runtime demos print durable archive roots | READY |
| `#1521` | milestone-organized run discovery and export | Review/export support for archived v0.87.1 demo runs | READY |
| `#1533` | real ChatGPT + Claude multi-agent provider demo | D13L live-provider companion proof | READY_WITH_OPERATOR_CREDENTIALS |

## Provider Family Demo / Test Issue Map

These family-level issues seed the provider proof surfaces that later demo work can implement:

Shared doc ownership:
- use `docs/tooling/PROVIDER_DEMO_SURFACES.md` for shared provider demo proof-surface rules (`#1508`)
- keep family-specific run instructions and proof caveats in the family wrapper outputs, not in this matrix

Shared wrapper helper:
- `adl/tools/provider_demo_common.sh` owns the small common README/proof-surface scaffolding used by provider-family demo wrappers (`#1507`).

Supporting provider infrastructure:
- ChatGPT provider profiles are tracked by `#1469`.
- Operator-entered provider credential setup is tracked by `#1474`.
- HTTPS-only remote transport discipline is tracked by `#1477`.

| Provider family | Scope | Issue |
|---|---|---|
| local Ollama | bounded local provider demo plus acceptance coverage for `ollama` / `local_ollama`; canonical command `bash adl/tools/demo_v0871_provider_local_ollama.sh` | `#1485` |
| bounded HTTP | bounded generic remote HTTP demo plus acceptance coverage for `http` / `http_remote`; canonical command `bash adl/tools/demo_v0871_provider_http.sh` | `#1486` |
| mock | no-network mock provider demo plus acceptance coverage and provider-substrate runnable proof; canonical command `bash adl/tools/demo_v0871_provider_mock.sh` | `#1487`, `#1509` |
| ChatGPT | `chatgpt:` family demo plus acceptance coverage using the current setup flow; canonical command `bash adl/tools/demo_v0871_provider_chatgpt.sh` | `#1488` |
| Claude | provider-family parity used by the bounded and live multi-agent discussion proofs; canonical live command `bash adl/tools/demo_v0871_real_multi_agent_discussion.sh` | `#1500`, `#1533` |

## Demo Coverage Summary

Use this table as the fast review surface for milestone coverage.

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
|---|---|---|---|---|---|---|---|
| D0 | Milestone Demo Suite | `WP-13` demo matrix + integrated proof entrypoint | `bash adl/tools/demo_v0871_suite.sh` | `artifacts/v0871/suite/demo_manifest.json` | One command runs the bounded runtime, provider-family, operator/review-surface, review-tail, and bounded multi-agent proof surfaces | Suite manifest ordering is stable across bounded local and mock inputs; live-provider proof remains D13L | READY |
| D1 | Runtime Environment Bring-Up | `WP-02` runtime environment completion | `bash adl/tools/demo_v0871_runtime_environment.sh` | `artifacts/v0871/runtime_environment/runtime/runtime_environment.json` | Runtime environment initializes cleanly with documented contracts and emits the canonical deterministic replay run root under the declared runtime root | Stable env inputs preserve runtime-environment artifact shape and bounded run-root structure | READY |
| D2 | Lifecycle Phases And Boundaries | `WP-03` execution boundaries and lifecycle | `bash adl/tools/demo_v0871_lifecycle.sh` | `artifacts/v0871/lifecycle/lifecycle_summary.json` | `init -> runtime_init -> execute -> complete -> run_completion -> teardown` is explicit and reviewable from one bounded summary | Fixed scenario preserves lifecycle phase ordering and boundary classification | READY |
| D3 | Trace-Aligned Runtime Execution | `WP-04` trace-aligned runtime execution | `bash adl/tools/demo_v0871_trace_runtime.sh` | `artifacts/v0871/trace_runtime/trace_bundle_manifest.json` | Runtime actions map coherently to emitted trace events, run summary, and archived trace-bundle surfaces | Replay preserves execution-to-trace shape and archive bundle structure for the bounded run | READY |
| D4 | Local Failure Handling | `WP-05` local runtime resilience | `bash adl/tools/demo_v0871_resilience_failure.sh` | `artifacts/v0871/resilience_failure/failure_summary.json` | Failure is bounded, explained, and leaves inspectable failure classification plus trace artifacts | Same induced failure preserves failure classification and review disposition | READY |
| D4A | Shepherd Preservation And Recovery | `WP-05`, `WP-07` Shepherd preservation + continuity discipline | `bash adl/tools/demo_v0871_shepherd_recovery.sh` | `artifacts/v0871/shepherd_recovery/shepherd_recovery_summary.json` | Interrupted work is preserved and classified for resume under explicit runtime rules | Fixed interruption scenario preserves preservation and recovery classification | READY |
| D5 | Restartability And Recovery | `WP-05`, `WP-07` resilience + state discipline | `bash adl/tools/demo_v0871_restartability.sh` | `artifacts/v0871/restartability/restartability_summary.json` | Bounded run can resume or restart under documented guardrails | Restart behavior remains stable under fixed paused-state inputs and resume guards | READY |
| D6 | Operator Invocation Surface | `WP-06` operator surfaces | `bash adl/tools/demo_v0871_operator_surface.sh` | `artifacts/v0871/operator_surface/runtime/runs/v0-4-demo-deterministic-replay/run_summary.json` plus `run_manifest.json` | Operator entrypoints are clear, stable, and reviewer-usable, with one canonical runtime-root proof set | Same command contract should preserve invocation shape and artifact naming | READY |
| D7 | Runtime State / Persistence Discipline | `WP-07` state / persistence discipline | `bash adl/tools/demo_v0871_runtime_state.sh` | `artifacts/v0871/runtime_state/runtime/runs/v0-6-hitl-pause-demo/run_status.json` | State is inspectable, bounded, and cleaned up deterministically across paused and completed runs | Stable inputs should preserve persistence classification, cleanup disposition, and state artifact inventory | READY |
| D8 | Review Surface Walkthrough | `WP-08` runtime review surfaces | `bash adl/tools/demo_v0871_review_surface.sh` | `artifacts/v0871/review_surface/demo_manifest.json` and `artifacts/v0871/review_surface/README.md` | Reviewer can locate primary D6 and D7 proof surfaces from one entrypoint | Manifest layout, reviewer guidance, and package ordering remain stable | READY |
| D9 | Integrated Runtime Path | `WP-02` through `WP-08` integrated runtime completion | `bash adl/tools/demo_v0871_integrated_runtime.sh` | `artifacts/v0871/integrated_runtime/demo_manifest.json` | One run demonstrates the authoritative bounded runtime path from D1 through D8 end-to-end | Replay is judged by package ordering and proof-surface shape stability across the integrated manifest | READY |
| D10 | Docs-To-Runtime Consistency Check | `WP-09`, `WP-15` docs/review convergence | `bash adl/tools/demo_v0871_docs_review.sh` | `artifacts/v0871/docs_review/docs_review_manifest.json` | Reviewer can move from promoted runtime docs to integrated proof surfaces without contradiction | Navigation and proof mapping remain stable for the bounded docs-to-proof map | READY |
| D11 | Quality Gate Walkthrough | `WP-14` quality gate | `bash adl/tools/demo_v0871_quality_gate.sh` | `artifacts/v0871/quality_gate/quality_gate_record.json` | Tests, validators, and bounded demo checks are reviewable in one place with per-check logs | Same repo state should preserve gate outcome and log inventory | READY |
| D12 | Release Review Package | `WP-16` through `WP-20` review/remediation/planning/release tail | `bash adl/tools/demo_v0871_release_review_package.sh` | `artifacts/v0871/release_review_package/release_review_package_manifest.json` | Review, remediation, planning, and release artifacts are coherent and navigable from one package root | Package layout and key entrypoints remain stable for the bounded release-review surface | READY |
| D13 | Claude + ChatGPT Tea Discussion | bounded multi-agent runtime discussion proof (`#1490`, `#1491`, `#1501`, `#1502`) | `bash adl/tools/demo_v0871_multi_agent_discussion.sh` | `artifacts/v0871/multi_agent_discussion/transcript.md` | Reviewer can inspect five explicit turns, two named agents, runtime turn metadata, the transcript contract, and the paired runtime trace/summaries | Fixed shim outputs should preserve transcript shape and turn ordering | READY |
| D13L | Live Claude + ChatGPT Tea Discussion | live-provider companion proof for D13 (`#1500`, `#1501`, `#1502`, `#1533`) | `bash adl/tools/demo_v0871_real_multi_agent_discussion.sh` | `artifacts/v0871/real_multi_agent_discussion/provider_invocations.json` plus `transcript.md` | Reviewer can inspect real OpenAI and Anthropic invocation metadata, five explicit turns, runtime turn metadata, and transcript contract proof without secret leakage when valid operator credentials and provider account credit are available; a no-credential test skip is a bounded non-proving disposition, not live-provider proof | Live model text is non-deterministic; runtime artifact shape, turn ordering, accepted contract shape, and non-secret invocation metadata remain stable | READY_WITH_OPERATOR_CREDENTIALS |

Provider demo wrappers also archive successful bounded runtime roots into `.adl/trace-archive/milestones/v0.87.1/runs/<run_id>/` and print that canonical archive location (`#1520`, `#1521`). The original `artifacts/v0871/.../runtime/runs/<run_id>/` proof surfaces remain the immediate demo outputs; the archive is the durable local trace index for later review/export.

Status guidance:
- `PLANNED` = intended but not yet validated
- `READY` = runnable and locally validated
- `READY_WITH_OPERATOR_CREDENTIALS` = runnable with operator-managed live-provider credentials and active provider account access; missing-credential skips remain non-proving and do not satisfy the live-provider proof claim; not required for CI-safe demo-suite validation
- `BLOCKED` = known dependency or missing proof surface
- `LANDED` = milestone evidence exists and is ready for review

## Coverage Rules
- Every major milestone claim should map to a runnable demo or an explicit alternate proof surface.
- Demo coverage should be reviewed against the WBS Acceptance Mapping before internal review.
- Every demo should name one primary proof surface that a reviewer can inspect directly.
- Commands should be copy/paste-ready and should not require private local state.
- Success signals should say what to check, not just “command exits 0”.
- Determinism / replay notes should explain how stability is judged.

## Demo -> Feature Mapping
- `D0` -> canonical `v0.87.1` demo-suite entrypoint for currently implemented proof surfaces
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
- `D13` -> bounded multi-agent runtime demo evidence for later conversation/runtime follow-on work (`#1490`, `#1491`, `#1501`, `#1502`)
- `D13L` -> live provider evidence that the bounded D13 shape can call real OpenAI and Anthropic models through the current ADL HTTP completion boundary (`#1533`)

## Demo Details

Per-demo detail sections below describe the runnable bounded proof surfaces for the milestone. Live-provider surfaces remain explicitly scoped where operator-managed credentials are required.

### D0) Milestone Demo Suite

Description:
- Provides the canonical WP-13 entrypoint for the bounded `v0.87.1` proof surfaces.
- Runs the bounded runtime rows D1-D5 and D9-D12, the provider-family demos, the runtime review walkthrough, and the bounded multi-agent discussion demo.
- Deliberately excludes the live D13L provider demo from the default suite because it depends on operator-managed OpenAI and Anthropic credentials.

Commands to run:

```bash
bash adl/tools/demo_v0871_suite.sh
```

Expected artifacts:
- `artifacts/v0871/suite/demo_manifest.json`
- `artifacts/v0871/suite/README.md`
- `artifacts/v0871/suite/index.txt`
- bounded runtime proof roots under `artifacts/v0871/suite/runtime_environment`, `lifecycle`, `trace_runtime`, `resilience_failure`, `shepherd_recovery`, `restartability`
- integrated/review-tail proof roots under `artifacts/v0871/suite/integrated_runtime`, `docs_review`, `quality_gate`, `release_review_package`
- provider proof roots under `artifacts/v0871/suite/provider_*`
- review proof roots under `artifacts/v0871/suite/review_surface`
- multi-agent proof roots under `artifacts/v0871/suite/multi_agent_discussion`
- durable trace/archive copies under `.adl/trace-archive/milestones/v0.87.1/runs/<run_id>/` when the underlying demo wrappers archive runtime roots

Primary proof surface:
- `artifacts/v0871/suite/demo_manifest.json`

Secondary proof surfaces:
- `artifacts/v0871/suite/README.md`
- `artifacts/v0871/suite/index.txt`
- `artifacts/v0871/suite/review_surface/demo_manifest.json`
- `artifacts/v0871/suite/multi_agent_discussion/transcript.md`

Expected success signals:
- The suite exits successfully.
- The manifest includes D1-D5, D8-D13, and the provider-family packages.
- D13L is discoverable from this matrix and `demos/v0.87.1/real_chatgpt_claude_multi_agent_discussion_demo.md`, but is not required for the CI-safe D0 suite.
- A skipped no-credential validation run for D13L is acceptable for bounded local ergonomics, but reviewers should treat it as an explicit non-proof disposition until a credentialed pass produces the live-provider artifacts named in the D13L row.
- The demo issue inventory above names every `v0.87.1` demo/proof issue so reviewers can find demos that are not part of the D0 runtime suite.

Determinism / replay notes:
- The suite uses bounded local provider shims, mock providers, and controlled repo-local runtime inputs.
- Proof-surface ordering in the manifest and index is stable.
- The suite does not claim full byte-for-byte replay for all generated artifacts.
- Live-provider output is intentionally outside the D0 determinism claim and is scoped by the D13L row.

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
- `bash adl/tools/test_demo_v0871_suite.sh` should pass for the canonical WP-13 suite
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
- D0 is locally validated by `bash adl/tools/test_demo_v0871_suite.sh`.
- D1-D5, D6, D7, D8, D9-D13, and the provider-family proof roots are included in the current suite or its integrated review surfaces.
- D13L remains the only live-provider companion row outside the default bounded suite.

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

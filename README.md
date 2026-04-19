# Agent Design Language (ADL)

Agent Design Language (ADL) is now more than a schema or notation layer. It is a deterministic orchestration system for AI workflows: a language, a reference Rust runtime, a CLI, review surfaces, and milestone proof packages that make agent execution inspectable and falsifiable.

ADL is built for teams that want AI systems to survive code review, ops review, and postmortem analysis. It turns orchestration into an engineering surface with explicit contracts, bounded runtime behavior, durable artifacts, and repository-visible proof instead of prompt theater.

ADL still starts from structured artifacts:
- providers
- tools
- agents
- tasks
- workflows
- runs

But those artifacts are not the whole story. In the current repository, they are schema-validated, compiled into a deterministic execution plan, and executed by a real runtime with explicit semantics for concurrency, failure handling, retries, signing, tracing, remote execution boundaries, and artifact emission. Every run leaves behind stable review surfaces under `.adl/`, so execution can be inspected, replayed, and reviewed with confidence.

[![adl-ci (main)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml)
[![coverage](https://codecov.io/gh/danielbaustin/agent-design-language/graph/badge.svg?branch=main)](https://app.codecov.io/gh/danielbaustin/agent-design-language/tree/main)
![Milestone](https://img.shields.io/badge/milestone-v0.90%20released-blue)

Today, ADL includes:
- a reference Rust runtime and CLI for deterministic workflow execution
- explicit planning and execution semantics rather than hidden orchestration logic
- bounded concurrency, retries, failure policy, and signing/verification surfaces
- trace, run-manifest, and artifact emission surfaces for replay and review
- provider and remote-execution boundaries that preserve local scheduler control
- milestone-specific demo and review packages that make claims inspectable

## Start Here

### If you want a fast first run

Print a deterministic plan from a minimal example:

```bash
cargo run -q --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-87-1-minimal-runtime-demo.adl.yaml --print-plan
```

Actually run the same minimal example and emit trace/artifact output:

```bash
cargo run -q --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-87-1-minimal-runtime-demo.adl.yaml --run --trace --allow-unsigned
```

### If you want the latest completed milestone proof package

Run the v0.90 readiness and proof surfaces from the milestone package:

```bash
python3 adl/tools/check_v090_milestone_state.py
```

This is the best top-level entrypoint if you want to see the milestone-compression drift check and proof graph for the latest completed milestone.

### If you want the flagship v0.90 demo

Run the long-lived stock-league package, the flagship bounded recurring-agent demo carried by the v0.90 release:

```bash
bash adl/tools/demo_v0891_long_lived_stock_league.sh
```

### If you want the previous runtime milestone package

```bash
bash adl/tools/demo_v0871_suite.sh
```

## Why ADL

ADL focuses on making agent systems reliable, inspectable, and usable in real engineering workflows.

ADL is built for readers and builders who care about:
- deterministic orchestration with clear runtime behavior
- explicit workflow contracts and structured execution surfaces
- stable proof surfaces that support review and debugging
- bounded, inspectable agent behavior
- local and enterprise-ready control over execution behavior

ADL is not trying to be a vague "agent framework." Its center of gravity is execution truth: what runs, what artifacts are emitted, what reviewers can inspect, and what the repository can prove today.

## What ADL Provides

ADL currently provides:
- a Rust runtime and CLI for deterministic workflow execution
- structured workflow, task, and provider definitions
- deterministic planning and execution semantics
- bounded concurrency, retries, and failure policies
- signing and verification surfaces for safer execution
- remote-execution wiring without giving up local scheduler control
- trace events, run manifests, and replay-friendly artifact roots
- bounded scientific / Gödel-style execution loops with reviewable artifacts
- reviewer-facing milestone proof packages and runnable demos that are meant to be inspectable and falsifiable

## Quick Start

For the most common entrypoints, use the `Start Here` section above.

Other useful entrypoints:
- previous runtime-completion milestone package: `bash adl/tools/demo_v0871_suite.sh`
- completed substrate milestone package: `bash adl/tools/demo_v087_suite.sh`
- bounded local operational-skills demo: `bash adl/tools/demo_codex_ollama_operational_skills.sh --dry-run`
- bounded multi-agent discussion demo: `bash adl/tools/demo_v0871_multi_agent_discussion.sh`

## Current Status

- Active milestone: **v0.90.1 planning is ready to start**
- Current release state: **v0.90 final release package ready for tag/release ceremony**
- Most recently completed milestone: **v0.90**
- Current crate version: **0.90.0**
- Version note: **v0.90 implementation, coverage, Rust refactoring, internal review, third-party review, accepted remediation, next-milestone planning, and release ceremony preparation are complete; the release ceremony script creates the tag and GitHub release after merge**
- Previous completed milestone package: **v0.89.1**
- Previous completed milestone: **v0.89**
- Project changelog: `CHANGELOG.md`

ADL is in active development. This repository contains both implemented runtime surfaces and milestone/spec/planning documents. Read the milestone docs as bounded engineering records: they distinguish what has shipped, what is under active review or closeout, what is demoable, and what is still planned.

## Current Milestone

v0.90.1 is the next milestone package and is ready to start after the v0.90 tag/release ceremony. Its tracked planning package lives under `docs/milestones/v0.90.1/`.

v0.90 is the just-completed long-lived-agent runtime milestone. It carries ADL from bounded single-run proof surfaces into supervised recurring cycles with durable artifacts, pre-identity continuity handles, operator controls, demo proof, milestone compression, repo visibility, explicit Rust refactoring, and a measured coverage ratchet.

The implementation wave landed through the long-lived runtime, stock-league demo, demo-extension, compression, repo-visibility, coverage, Rust-refactoring, docs, and internal-review surfaces. The release tail completed third-party review, accepted ADR remediation, next-milestone planning, and release ceremony preparation. The release ceremony script performs the remaining tag/GitHub-release operation after this final package merges.

Best current v0.90 entrypoints:
- milestone docs: `docs/milestones/v0.90/README.md`
- demo matrix: `docs/milestones/v0.90/DEMO_MATRIX_v0.90.md`
- pre-third-party readiness report: `docs/milestones/v0.90/V090_PRE_THIRD_PARTY_READINESS_REPORT.md`
- milestone compression packet: `docs/milestones/v0.90/milestone_compression/README.md`
- repo visibility packet: `docs/milestones/v0.90/repo_visibility/README.md`
- stock-league demo docs: `demos/v0.90/long_lived_stock_league_demo.md`

## Recent Milestones

### v0.90 - Long-Lived-Agent Runtime Milestone

v0.90 is the most recently completed milestone package. It landed the bounded long-lived-agent runtime slice, stock-league proof package, release-discipline sidecars, coverage ratchet, Rust refactoring pass, internal and third-party review, ADR 0011 remediation, and v0.90.1 planning handoff.

Key features:
- supervised recurring cycles with heartbeat and lease status
- durable cycle artifacts and explicit continuity handles
- operator inspection, stop, and guardrail controls
- stock-league long-lived-agent proof package plus bounded demo extensions
- milestone compression and repo visibility proof packets
- completed release review, accepted remediation, next planning, and release ceremony preparation

### v0.89.1 - Adversarial Runtime and Review-Tail Milestone

v0.89.1 is the previous completed milestone. The adversarial/runtime implementation and proof surfaces landed, third-party review and review remediation are closed, and the v0.90 planning package handed off into the now-completed v0.90 wave.

Key features:
- adversarial runtime model and red/blue/purple execution architecture
- exploit artifact schema, replay manifest, continuous verification, and self-attack proof surfaces
- provider-proof packaging, proof-entry-point integration, and quality-gate review surfaces
- five-agent Hey Jude MIDI demo and bounded arXiv manuscript workflow packet
- completed third-party review, review remediation, next-milestone planning, and release ceremony

### v0.89 - Completed Governed Adaptive Execution Milestone

v0.89 is the completed governed-adaptation milestone. The core execution wave landed through `WP-13`, and its release-tail work handed off the adversarial/runtime carry-forward into `v0.89.1`.

Key features:
- AEE 1.0 convergence and bounded stop-family proof surfaces
- Freedom Gate v2, decision/action mediation, and governed skill execution contracts
- experiment records, ObsMem evidence/ranking, and security/trust planning carried into one canonical package
- bounded provider-participation demos plus an integrated `v0.89` reviewer surface
- completed handoff into the `v0.89.1` adversarial/runtime follow-on

### v0.88 - Temporal / Chronosense + Instinct Review-Tail Milestone

v0.88 is the prior temporal / chronosense and instinct milestone package. Its implementation wave completed through `WP-13`, and its review-tail work set up the `v0.89` governance band.

Key features:
- promoted temporal / chronosense and instinct / bounded-agency feature-doc package
- bounded proof surfaces for temporal review, PHI metrics, instinct review, and the integrated `v0.88` review surface
- Paper Sonata as the flagship bounded public-facing demo
- deep-agents comparative proof as a supporting reviewer-facing proof row
- completed handoff into internal review, 3rd-party review, remediation, next-milestone planning, and release ceremony

### v0.87.1 - Runtime Completion and Reviewer-Facing Proof Package

v0.87.1 is the previous runtime-completion milestone. The implementation and bounded demo program landed on `main`, and it now serves as the prior runtime proof package that `v0.88` and `v0.89` build on.

Key features:
- runtime environment, lifecycle, execution-boundary, and resilience surfaces promoted into one canonical milestone package
- bounded demo suite and reviewer walkthrough package for runtime, provider, quality-gate, and release-tail proof surfaces
- explicit trace/run-manifest/archive surfaces for review and export
- credential-gated live-provider companion proof kept explicit as non-CI reviewer evidence rather than implied as default proof
- active handoff into internal review, external / 3rd-party review, remediation, and release

### v0.87 - Substrate Convergence and Reviewer-Facing Milestone Truth

v0.87 completed the turn from the bounded cognitive proof in v0.86 into a coherent, deterministic, and reviewer-legible substrate milestone.

Key features:
- trace, provider, shared-memory, skills, and control-plane work aligned under one canonical milestone spine
- promoted feature docs and milestone docs reconciled against the real implementation and issue sequence
- bounded demo and reviewer proof surfaces for trace, provider portability, shared ObsMem, skills, and control-plane behavior
- completed Sprint 3 release-tail work for documentation convergence, review, quality gate, and release closeout
- explicit handoff into `v0.87.1` for the runtime-completion milestone that set up the current `v0.88` follow-on

### v0.86 - Bounded Cognitive System and Reviewable Proof Surfaces

v0.86 established ADL's first working bounded cognitive system on `main`.

Key features:
- one canonical bounded cognitive path:
  `signals -> candidate selection -> arbitration -> reasoning -> bounded execution -> evaluation -> reframing -> memory participation -> Freedom Gate`
- canonical runtime artifacts for the bounded cognitive path and related proof surfaces
- local demo and review surfaces for the integrated milestone proof set
- Sprint 7 quality-gate work with local `fmt`, `clippy`, `test`, coverage, and demo-validation proof
- docs, release-tail surfaces, and reviewer entry points aligned toward milestone truth

### v0.85 - Authoring Truth and Demo Proof Surfaces

v0.85 focused on bringing the authoring model, demos, and runtime behavior into a coherent and reliable whole.

Key features:
- clarified authoring lifecycle (`pr init`, `pr start`, `pr run`, `pr finish`)
- bounded editor-command adapter aligned to the control plane
- end-to-end demo and regression proof surfaces for authoring workflows
- worktree hygiene and queue-mechanics cleanup
- Rust maintainability improvements (module refactors, test restructuring, guardrails)

### v0.8 - Bounded Godel Runtime and Artifact-Centered Review

v0.8 extended ADL into bounded reflective execution with structured artifacts and strong inspection surfaces.

Key features:
- bounded Godel-style scientific loop integrated into runtime
- canonical artifact emission for mutation, evaluation, and experiment records
- CLI surfaces for running and inspecting reasoning workflows
- ObsMem-backed indexing and retrieval-assisted review flows
- runnable demo and evaluation surfaces for hypothesis-driven execution

### v0.7 - Deterministic Runtime Foundation

v0.7 established the deterministic execution model that underpins the ADL runtime.

Key features:
- ExecutionPlan-driven runtime
- deterministic fork/join and concurrency semantics
- bounded parallelism and explicit retry/failure policies
- replay-oriented traces and graph export tooling
- signing and verification surfaces for execution integrity

## Demos and Proof Surfaces

ADL includes both ordinary demos and heavyweight reviewer or release proof packages.

Start here:
- canonical user-facing demo index: `demos/README.md`
- current milestone quality-gate package: `bash adl/tools/demo_v0891_quality_gate.sh`
- current milestone integration-demo package: `bash adl/tools/demo_v0891_wp13_demo_integration.sh`

Important supporting demo/readiness docs:
- `docs/tooling/editor/README.md`
- `docs/tooling/editor/five_command_demo.md`
- `docs/tooling/editor/five_command_regression_suite.md`

Use this split when choosing an entrypoint:
- ordinary demos are bounded runnable proofs intended for demo sweeps and first-run exploration
- reviewer packages combine multiple proof rows into one heavier review surface
- quality-gate and release-review packages are heavyweight release-tail proofs, not ordinary demos

For milestone-specific context:
- `docs/milestones/v0.89/DEMO_MATRIX_v0.89.md`
- `docs/milestones/v0.88/DEMO_MATRIX_v0.88.md`
- `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`
- `docs/milestones/v0.86/DEMO_MATRIX_v0.86.md`
- `docs/milestones/v0.7/DEMOS_v0.7.md`
- `docs/milestones/v0.8/DEMOS_V0.8.md`
- `docs/milestones/v0.85/DEMO_MATRIX_v0.85.md`

## Repository Layout

- `adl/`: Rust reference runtime and CLI
- `adl/examples/`: runnable workflow fixtures used by the runtime and tests
- `adl-spec/`: language-level specification docs
- `demos/`: canonical user-facing demo index, runbooks, and proof surfaces
- `docs/`: contributor workflow, roadmap, tooling, and milestone docs
- `docs/adr/`: architecture decision records
- `.adl/`: cards, reports, run artifacts, and related authoring surfaces

## Default Workflow

The default contributor workflow is documented as a bounded authoring cycle.

Start here:
- `docs/default_workflow.md`
- `docs/tooling/adl_pr_cycle_skill.md`
- `adl/tools/README.md`


## License

Apache-2.0

## Security

- Security policy: `SECURITY.md`
- Threat model: `docs/security/THREAT_MODEL_v0.7.md`

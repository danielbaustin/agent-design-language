# Agent Design Language (ADL)

Agent Design Language (ADL) is a deterministic, contract-driven orchestration language for AI systems. It is designed for teams that want AI workflows to be reviewable, testable, reproducible, and auditable, with clear execution semantics and transparent runtime behavior.

ADL lets you define the core pieces of an AI system as structured artifacts:
- providers
- tools
- agents
- tasks
- workflows
- runs

Those artifacts are schema-validated, compiled into a deterministic execution plan, and executed with explicit semantics for concurrency, failure handling, retries, signing, and artifact emission. Every run leaves behind stable review surfaces under `.adl/` so execution can be inspected, replayed, and reviewed with confidence.

[![adl-ci (main)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml)
[![coverage](https://codecov.io/gh/danielbaustin/agent-design-language/graph/badge.svg?branch=main)](https://app.codecov.io/gh/danielbaustin/agent-design-language/tree/main)
![Milestone](https://img.shields.io/badge/milestone-v0.87.1%20active-blue)

## Why ADL

ADL focuses on making agent systems reliable, inspectable, and suitable for real engineering workflows.

ADL is built for readers and builders who care about:
- deterministic orchestration with clear runtime behavior
- explicit workflow contracts and structured execution surfaces
- stable proof surfaces that support review and debugging
- bounded, inspectable agent behavior
- local and enterprise-ready control over execution behavior

If you want AI systems that can survive code review, operations review, and postmortem analysis, ADL is aimed at you.

## What ADL Provides

ADL currently provides:
- a Rust runtime and CLI for deterministic workflow execution
- structured workflow, task, and provider definitions
- deterministic planning and execution semantics
- bounded concurrency, retries, and failure policies
- signing and verification surfaces for safer execution
- remote-execution wiring without giving up local scheduler control
- bounded scientific / Gödel-style execution loops with reviewable artifacts
- demo and proof surfaces that are meant to be runnable, inspectable, and falsifiable

## Quick Start

From repo root:

```bash
cargo run -q --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-3-fork-join-seq-run.adl.yaml --print-plan
```

This prints a deterministic fork/join execution plan with no provider runtime setup.

A second quick check:

```bash
cargo run -q --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-3-on-error-retry.adl.yaml --print-plan
```

## Current Status

- Current active milestone: **v0.87.1**
- Recent stable milestone: **v0.8**
- Most recently completed milestone: **v0.87**
- Previous closure milestone: **v0.86**
- Project changelog: `CHANGELOG.md`

ADL is in active development. The repository contains both implemented runtime surfaces and milestone/spec/planning documents. The milestone docs should be read as bounded engineering records: they distinguish what has shipped, what is under active review/closeout, what is demoable, and what is still planned.

## Recent Milestones

### v0.87.1 - Runtime Completion and Reviewer-Facing Proof Package

v0.87.1 is the current active milestone. The implementation and bounded demo program are in place, and the remaining work is the docs/review/quality/release tail that makes the runtime package reviewable without oral reconstruction.

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
- explicit handoff into `v0.87.1` for the runtime-completion milestone that now sits in the review/release tail

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

ADL includes both user-facing demos and milestone-specific proof surfaces.

Start here:
- `demos/README.md`

Important supporting demo/readiness docs:
- `docs/tooling/editor/README.md`
- `docs/tooling/editor/five_command_demo.md`
- `docs/tooling/editor/five_command_regression_suite.md`

For milestone-specific context:
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

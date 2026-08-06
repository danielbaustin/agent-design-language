# Tooling Documentation

This directory is the main entrypoint for ADL tooling guides, prompt-spec references, reviewer surfaces, editor-related proof surfaces, and maintainability utilities.

The goal of this directory is to make ADL’s tooling surfaces understandable and navigable without forcing the reader to learn the entire internal workflow system first.

## Start Here

- Prompt-spec and structured prompt surfaces: `prompt-spec.md`
- Canonical issue-card lifecycle: `card-lifecycle.md`
- Structured prompt contracts: `structured-prompt-contracts.md`
- Session coordination and root checkout policy:
  `SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md`
- Claim-free C-SDLC v2 issue creation and binding:
  `C_SDLC_V2_ISSUE_CREATION_AND_BINDING_RUNBOOK.md`
- Default contributor workflow: `../default_workflow.md`
- Editor and authoring proof surfaces: `editor/README.md`
- Root project overview: `../README.md`

## Remote Build Start Here

Remote validation and CodeFriend-scale build sessions should read these docs
before launching paid or remote work:

- [Validation Platform Routing](VALIDATION_PLATFORM_ROUTING.md) - scheduler
  routing truth for local, Nessus, AWS Spot, CodeBuild, and Wuji.
- [Remote Build How-To](REMOTE_BUILD_HOW_TO.md) - reusable operator playbook
  for safe dry-runs, live runs, cache proof, and result recording.
- [ADL Builder Image](ADL_BUILDER_IMAGE.md) - shared builder-image contract,
  including the requirement to use a pre-published image instead of rebuilding
  it inside each validation run.
- [AWS CodeFriend Build Lane](AWS_CODEFRIEND_BUILD_LANE.md) - CodeBuild setup,
  wrapper usage, cache posture, account checks, and live-run guardrails.
- [AWS Spot Remote Validation Lane](AWS_SPOT_REMOTE_VALIDATION_LANE.md) - Spot
  wrapper usage, warm EBS cache posture, SSH/debug affordances, and cleanup
  expectations.
- [Nessus Validation Manager Lane](NESSUS_VALIDATION_MANAGER_LANE.md) - Nessus
  remote validation-manager wrapper and artifact contract.
- [Build Platform Benchmarks](BUILD_PLATFORM_BENCHMARKS.md) - current retained
  timing rows and the accepted proof posture for each platform.

Operational defaults:

- Use the Agent Logic AWS profile, `agent-logic-admin`, for ADL AWS work.
- Keep AWS Spot and CodeBuild as explicit operator-triggered paths; do not wire
  paid lanes into ordinary PR or push CI by accident.
- Keep cache posture visible in proof: Spot should show retained warm EBS cache,
  CodeBuild should show the fixed builder image plus stable target/cache setup,
  and Nessus should distinguish cold image-backed from warm target-cache rows.
- Do not claim Wuji image-backed parity until an ARM64 or multi-arch builder
  image exists.

## Core Tooling Areas

### Prompt and Card Surfaces

These docs describe the structured prompt surfaces used to shape issues, issue-card lifecycle state, outcome records, and deterministic reviewer flows.

- [Prompt Spec](prompt-spec.md)
- [ADL Card Lifecycle](card-lifecycle.md)
- [Active Card Lifecycle Migration Readiness](active-card-lifecycle-migration-readiness-v0.91.2.md)
- [Structured Prompt Contracts](structured-prompt-contracts.md)
- [Structured Prompt Validator Binary Resolution](structured-prompt-validator-binary-resolution.md)
- [SRP, SOR, And ObsMem Handoff Model](srp-sor-obsmem-handoff-v0.91.2.md)
- [Prompt/Reviewer Surface Mapping](prompt-review-surface-mapping.md)
- [Prompt Spec Protocol Bindings](prompt-spec.md#protocol-bindings)
- [Issue Prompt Templates](issue-prompts/README.md)

### Portable ADL Project Surfaces

These docs and templates describe how external repositories can declare ADL
C-SDLC workflow policy without copying the full ADL toolchain.

- [Portable ADL Project Adapter Contract](PORTABLE_ADL_PROJECT_ADAPTER_CONTRACT_v0.91.5.md)
- [Portable ADL Project Doctor Plan](PORTABLE_ADL_PROJECT_DOCTOR_PLAN_v0.91.5.md)
- [Portable ADL Adapter Follow-Ons](PORTABLE_ADL_PROJECT_ADAPTER_FOLLOW_ONS_v0.91.5.md)
- [Portable ADL Templates](../templates/portable-adl/README.md)

### Reviewer and Validation Surfaces

These docs describe bounded reviewer behavior, deterministic output formats, and provenance/review validation surfaces.

- [Reviewer Surface](reviewer-surface.md)
- [Reviewer Output Provenance](reviewer-provenance.md)
- [Card Reviewer GPT Instructions](card-reviewer-gpt.md)
- [Deterministic Review Output Format](card-review-output-format.md)

Stable reviewer regression fixture:
- `docs/tooling/examples/reviewer-regression/issue-661/input_661.md`
- `docs/tooling/examples/reviewer-regression/issue-661/output_661.md`
- `docs/tooling/examples/reviewer-regression/issue-661/expected_review_output_661.yaml`

### Editor and Authoring Surfaces

These docs describe the bounded editor and authoring surfaces used in the v0.85 authoring/control-plane work.

- [Task Bundle Editor](editor/README.md)
- [Milestone Dashboard](milestone-dashboard/README.md)
- `editor/five_command_demo.md`
- `editor/five_command_regression_suite.md`

### Worktree and Maintainability Surfaces

These docs describe worktree governance, large-module tracking, and related maintenance guidance.

- [Worktree Governance](worktree_governance.md)
- [Session Coordination And Root Checkout Policy](SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md)
- Rust module size reports are local operational artifacts under `.adl/reports/manual/`; regenerate them with `./adl/tools/report_large_rust_modules.sh`
- [WP Issue-Wave Generation](WP_ISSUE_WAVE_GENERATION.md)
- [Historical Public Task Records](../records/README.md)

## Tooling Scripts and Utilities

Important repo-local tooling surfaces include:

- `csdlc-doctor` — typed PR readiness and drift diagnostics
- `csdlc-issue create` and `csdlc-bind` — claim-free typed issue creation and
  Git-topology binding; see the
  [creation and binding runbook](C_SDLC_V2_ISSUE_CREATION_AND_BINDING_RUNBOOK.md)
- `csdlc-validate`, `csdlc-review`, and `csdlc-publish` — typed finalization,
  exact-revision review, and publication path
- `adl-lint-prompt-spec` — direct Prompt Spec lint and validation binary
- `adl-prompt-template` — direct prompt-card editor and renderer binary
- `adl-validate-structured-prompt` — direct structured prompt contract validator
- `bash adl/tools/run_aws_codefriend_build_lane.sh` — manual GitHub Actions plus AWS CodeBuild lane wrapper for CodeFriend build orchestration; see [AWS CodeFriend Build Lane](AWS_CODEFRIEND_BUILD_LANE.md)
- CI log archival is not exposed through the removed tooling multiplexer; see
  [CI Log Archive To S3](CI_LOG_ARCHIVE_S3.md) for historical context.
- `adl/tools/validation_manager.py --run` — emit durable `adl.build_action_log.v1` packets for local validation actions; see [Build Action Logs](BUILD_ACTION_LOGS.md)
- `csdlc-validate` — current typed C-SDLC v2 lifecycle/card validation; see [structured prompt validation boundary](structured-prompt-validator-binary-resolution.md)
- current direct owner binaries and typed skills own planning, provenance, and
  review validation; the removed `adl tooling` multiplexer is not an
  operational route.
- `bash adl/tools/demo_v0871_operator_surface.sh` — canonical `v0.87.1` operator-surface demo for runtime bring-up and proof-surface inspection
- `bash adl/tools/demo_v0871_review_surface.sh` — canonical `v0.87.1` reviewer walkthrough package across operator and runtime-state proof roots
- `bash adl/tools/run_validation_manager_nessus_lane.sh` — validation-manager
  wrapper that consumes one eligible local lane and routes it to the Nessus
  remote runner; see [Nessus Validation Manager Lane](NESSUS_VALIDATION_MANAGER_LANE.md)
- `bash adl/tools/setup_adl_builder_image.sh` — shared validation toolchain
  image setup for CodeBuild, AWS Spot EC2, Nessus, and local runners; see
  [ADL Builder Image](ADL_BUILDER_IMAGE.md)
- `bash adl/tools/import_adl_builder_image_from_s3_to_ecr.sh` — AWS-side
  importer that loads a builder-image tar from S3 and pushes it to ECR through
  a privileged, purpose-specific CodeBuild project
- `bash adl/tools/validation_manager.sh --platform-routing` — first-class
  local, Nessus, AWS Spot, CodeBuild, and wuji routing decisions for scheduler
  consumption without launching paid cloud resources; see
  [Validation Platform Routing](VALIDATION_PLATFORM_ROUTING.md)
- `bash adl/tools/run_aws_spot_remote_validation_lane.sh` — AWS Spot EC2
  remote validation wrapper that checks the `agent-logic-admin` account against
  retained Agent Logic proof before launch and reuses the retained warm EBS
  cache volume by default; see
  [AWS Spot Remote Validation Lane](AWS_SPOT_REMOTE_VALIDATION_LANE.md)
- `bash adl/tools/run_build_platform_benchmark.sh` — shared Wuji, Nessus, AWS
  Spot, and CodeBuild timing workload; see
  [Build Platform Benchmarks](BUILD_PLATFORM_BENCHMARKS.md)
- lifecycle authority is the independent binary set under `.adl/bin/csdlc-v2/`
- `adl/tools/report_large_rust_modules.sh` — non-blocking Rust source-and-test module size report; by default it scans both `adl/src` and `adl/tests`, and current snapshots should live under `.adl/reports/manual/` instead of tracked repo docs
- `adl/tools/sync_task_bundle_prompts.sh` — refresh canonical local task-bundle prompt layout from compatibility paths

Historical evidence may mention removed v1 aliases, but active tests and docs
must not expose them as executable control-plane routes.

## Current Status

- Current closure milestone: **v0.87**
- Next active milestone: **v0.87.1**
- Role of this directory: tooling/reference entrypoint for prompt, reviewer, editor, and maintenance surfaces

## Runtime Operator Surface

For `v0.87.1`, the bounded runtime operator contract is:
- one canonical runtime invocation path via `adl-runtime run <adl-file> ...`
- one canonical runtime-root marker at `runtime_environment.json`
- one canonical per-run inspection set rooted at `run_summary.json`, `run_status.json`, and `logs/trace_v1.json`

Use `bash adl/tools/demo_v0871_operator_surface.sh` as the smallest repo-local proof of that operator surface.

## Runtime Review Surface

For `v0.87.1`, the bounded runtime reviewer contract is:
- one canonical walkthrough command via `bash adl/tools/demo_v0871_review_surface.sh`
- one canonical package manifest at `artifacts/v0871/review_surface/demo_manifest.json`
- one canonical reviewer guide at `artifacts/v0871/review_surface/README.md`
- one stable package ordering rooted in D6 operator proof and D7 runtime-state proof

The historical review package requires a current direct validator before it can
be advertised as an executable proof route.

## Notes

Tooling docs should be read as bounded engineering references. They describe the surfaces that support ADL authoring, review, and maintenance without claiming that every internal helper is equally important to every reader.

## Current Retirement Boundary

The active `adl/tools` surface intentionally excludes a small set of retired legacy
residue that no longer supports the live PR workflow, current demos, or active
regression tests. The `v0.87` bounded cleanup retired:

- `BURST_PLAYBOOK.md`
- `REPORT_SCHEMA.md`
- `default.rules.profiles.example`
- `demo_v0_4.sh`
- `pr_smoke.sh`

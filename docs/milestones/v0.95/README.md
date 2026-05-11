# v0.95 Milestone README

## Status

Forward planning. `v0.95` is not yet an active implementation milestone and
has no final issue wave.

## Purpose

`v0.95` is the planned MVP convergence and feature-freeze milestone.

It should give ADL one coherent launch-shape platform story by finishing:

- HTML milestone dashboard and compression reporting
- CSM Shepherd and Gemma evaluation/training closure
- Aptitude Atlas as the bounded model-evaluation platform
- distributed-substrate integration
- polished demo catalog and MVP walkthrough
- control-plane/tooling hardening
- web-based code-editor integration
- the explicit MVP editor decision boundary, including Zed

## Milestone Role

`v0.95` should establish:

- converged milestone, dashboard, and compression-reporting surfaces for review
- a bounded evaluator/training lane for Shepherd/Gemma and adjacent model work
- one coherent model-evaluation platform story through Aptitude Atlas
- converged user-facing and reviewer-facing walkthrough surfaces
- explicit integration closure across the platform bands already scheduled
- final high-value control-plane migration and hardening work
- a truth-preserving MVP boundary for editor capability, web editing, and
  optional Zed carry-in

`v0.95` should not silently admit new architectural domains after the
`v0.94`/`v0.94.1` closure bands.

## Scope Summary

### In scope

- HTML milestone dashboard and compression reporting
- CSM Shepherd and Gemma training/evaluation follow-on
- Aptitude Atlas model-evaluation platform closure
- distributed execution integration closure
- demo catalog and polished MVP walkthrough
- control-plane Rust migration and tooling hardening
- web-based code-editor integration baseline
- bounded Zed integration decision or explicit deferral
- MVP feature-freeze and 1.0 scope-boundary preparation

### Out of scope

- silent post-`v0.95` deferral of must-have roadmap bands
- introducing new greenfield platform domains
- pretending optional editor-host preference is already a must-have requirement

## Document Map

- Vision: [VISION_v0.95.md](VISION_v0.95.md)
- Design: [DESIGN_v0.95.md](DESIGN_v0.95.md)
- WBS: [WBS_v0.95.md](WBS_v0.95.md)
- Sprint plan: [SPRINT_v0.95.md](SPRINT_v0.95.md)
- Decisions: [DECISIONS_v0.95.md](DECISIONS_v0.95.md)
- Demo matrix: [DEMO_MATRIX_v0.95.md](DEMO_MATRIX_v0.95.md)
- Milestone checklist: [MILESTONE_CHECKLIST_v0.95.md](MILESTONE_CHECKLIST_v0.95.md)
- Release plan: [RELEASE_PLAN_v0.95.md](RELEASE_PLAN_v0.95.md)
- Release notes: [RELEASE_NOTES_v0.95.md](RELEASE_NOTES_v0.95.md)
- Feature plans: [features/README.md](features/README.md)
- HTML dashboard and compression reporting:
  [HTML_MILESTONE_DASHBOARD_AND_COMPRESSION_REPORTING_v0.95.md](features/HTML_MILESTONE_DASHBOARD_AND_COMPRESSION_REPORTING_v0.95.md)
- Shepherd/Gemma training path:
  [CSM_SHEPHERD_AND_GEMMA_TRAINING_PATH_v0.95.md](features/CSM_SHEPHERD_AND_GEMMA_TRAINING_PATH_v0.95.md)
- Aptitude Atlas evaluation platform:
  [APTITUDE_ATLAS_MODEL_EVALUATION_PLATFORM_v0.95.md](features/APTITUDE_ATLAS_MODEL_EVALUATION_PLATFORM_v0.95.md)
- Distributed execution integration:
  [DISTRIBUTED_EXECUTION_INTEGRATION_v0.95.md](features/DISTRIBUTED_EXECUTION_INTEGRATION_v0.95.md)
- Demo catalog and MVP walkthrough:
  [DEMO_CATALOG_AND_MVP_WALKTHROUGH_v0.95.md](features/DEMO_CATALOG_AND_MVP_WALKTHROUGH_v0.95.md)
- Control-plane Rust migration and tooling hardening:
  [CONTROL_PLANE_RUST_MIGRATION_AND_TOOLING_HARDENING_v0.95.md](features/CONTROL_PLANE_RUST_MIGRATION_AND_TOOLING_HARDENING_v0.95.md)
- Web-based code editor integration:
  [WEB_BASED_CODE_EDITOR_INTEGRATION_v0.95.md](features/WEB_BASED_CODE_EDITOR_INTEGRATION_v0.95.md)
- Zed integration:
  [ZED_INTEGRATION_v0.95.md](features/ZED_INTEGRATION_v0.95.md)

## Dependency Boundary

`v0.95` depends on:

- earlier implemented baseline work through `v0.91.2`
- `v0.92` identity/birthday completion
- `v0.93` governance and enterprise-security completion
- `v0.94` secure-execution and reasoning/provenance closure
- `v0.94.1` payments/economic follow-on closure where those surfaces are
  declared part of MVP

## Success Criteria

- every MVP-scoped feature row has a canonical tracked home
- the dashboard/compression and evaluation-platform surfaces are explicit rather
  than implied by older backlog language
- distributed execution remains bounded and reviewable
- the demo catalog and walkthrough tell one coherent platform story
- the control plane is hardened enough for the MVP review posture
- the editor story is explicit across web baseline and Zed decision boundary

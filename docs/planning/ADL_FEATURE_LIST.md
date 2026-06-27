# ADL Feature List

## Purpose

This document is the canonical ADL capability overview.

It answers four practical questions:

- what ADL already does today
- what is active in the current milestone
- which major platform bands are planned next
- how each feature is expected to reach an explicit completion state

The tone should be strong because ADL has already become a substantial system.
The status language should remain strict: we only call something implemented
when the repo contains a real bounded runtime surface, proof surface, or
reviewable artifact set for it.

## What ADL Is Now

ADL is no longer just a language idea or schema set.

ADL is now a deterministic agent-runtime and orchestration platform with:
- a Rust reference runtime and CLI
- explicit workflow, task, agent, tool, and provider artifacts
- deterministic planning and bounded execution semantics
- trace, artifact, and review surfaces for post-run inspection
- bounded demos and milestone proof packages
- structured authoring and control-plane workflows for repo-scale execution

In short: ADL has become an engineering system for building AI workflows that
can survive code review, ops review, and postmortem analysis.

## Status Legend

The status column describes current repo maturity, not final completion. A
feature can have a real implemented baseline and still need later subsystem,
milestone-integration, or MVP-completion work.

- **Implemented**: materially present on `main` with working code, artifacts,
  docs, and/or demo or review surfaces. This does not automatically mean the
  feature is complete for every milestone or for the MVP.
- **Implemented baseline**: a real, usable slice exists. Baseline means "there
  is something reviewable to build from," not "the feature is done."
- **Active milestone**: materially present and under active closeout/review in
  the current milestone band.
- **Partially implemented**: meaningful enabling surfaces exist, but the
  capability is not yet complete enough to count as a finished subsystem or
  platform band.
- **Planned**: primarily a planned milestone/feature band today.
- **MVP-scoped**: not complete yet, but explicitly assigned to the MVP path.
  By `v0.95`, every in-roadmap feature should at least reach
  `baseline_exists` unless it is explicitly out of scope.
- **Post-MVP / post-v0.95 completion**: baseline is expected by `v0.95`, but
  subsystem-complete or product-complete work is intentionally later. This is
  valid only when the feature has an explicit rationale and non-goal boundary.
- **Out of scope**: deliberately excluded from the current roadmap, with a
  recorded reason.

Every feature row must eventually name an explicit baseline target, completion
target, or out-of-scope disposition. "Deepen later" and "future" are not valid
by themselves. Post-v0.95 completion is allowed, but the feature should still
have a v0.95 baseline if it remains in the MVP roadmap.

MVP rule: `v0.95` is the MVP convergence milestone, not the universal
subsystem-completion milestone. By `v0.95`, every feature that remains in the
MVP roadmap should at least have a reviewable baseline, proof posture, and
explicit post-MVP disposition if it is not fully complete.

## Completion State Model

Use these completion states when interpreting or updating feature rows.

| Completion state | Meaning | Required evidence |
| --- | --- | --- |
| `baseline_exists` | A bounded slice exists and is reviewable. | Code, docs, demo, fixture, proof packet, or review artifact exists on `main`. |
| `subsystem_complete` | The feature has explicit done criteria and all required subsystem behavior is implemented or explicitly out of scope. | Done criteria, owner issue/PR evidence, validation, review, and residual-risk disposition. |
| `integrated_for_milestone` | The feature is usable for a named milestone story. | Milestone docs, issue wave, proof surface, and release-tail evidence agree. |
| `mvp_complete` | No known MVP-blocking work remains for the feature. | MVP acceptance criteria, validation, demo/proof, review sign-off, and release evidence. |
| `partial_or_blocked` | Some surfaces exist but completion depends on unresolved work or a blocker. | Blocker, owner, target milestone, and next proof step are recorded. |
| `post_mvp_completion` | Baseline is expected by `v0.95`, but subsystem/product completion is later. | v0.95 baseline proof, explicit non-MVP rationale, boundary, and optional future target. |
| `out_of_scope` | ADL deliberately will not pursue this work in the current roadmap. | Decision note or issue disposition explaining why. |

Moving a feature from `baseline_exists` to any completion state requires four
things: clear done criteria, an owner milestone or post-MVP disposition, a
proof surface, and residual-risk routing. A baseline row without those four
things is not complete; it is only a starting point future agents can rely on.

## Enterprise Security Organization Boundary

Enterprise-security features are part of the roadmap, but they should not make
the core ADL runtime harder to understand, build, or review. Treat enterprise
security as a separable capability band rather than as ambient mainline
complexity.

Recommended organization rule:

- Keep core runtime, C-SDLC, prompt records, provider substrate, and ordinary
  validation paths in the mainline code/docs surface.
- Put enterprise security features behind explicit modules, feature docs,
  schemas, adapters, fixtures, tests, and proof packets.
- Prefer names and paths that make the boundary obvious, such as
  `enterprise_security`, `adversarial_runtime`, `signed_trace`, `policy`,
  `trust`, or milestone feature docs that point to those modules.
- Do not let enterprise-only threat models, deployment assumptions, or audit
  requirements become hidden prerequisites for normal local ADL development.
- Require every enterprise-security feature to declare whether its `v0.95`
  requirement is baseline, subsystem complete, MVP complete, or post-MVP
  completion.

Dedicated planning issue `#3538` owns the architecture decision before large
code moves. Its tracked planning packet is
`docs/milestones/v0.91.5/features/ENTERPRISE_SECURITY_ORGANIZATION_BOUNDARY_v0.91.5.md`.
That packet inventories the current security/trust/policy/signing/adversarial/
isolation/ACIP/governed-execution surfaces and recommends staged module/proof
separation before any crate extraction. The organizing principle is clear now:
separate the enterprise security band without orphaning the proof and policy
contracts it depends on.

## Current Repo Status

Current roadmap planning truth as of 2026-06-27:
- the feature-doc production wave is tracked as issue `#3779` with child
  issues `#3778`, `#3780`, `#3781`, and `#3782`
- `v0.91.6` is the first pre-v0.92 bridge/readiness tranche and is currently
  in release-tail execution after WP-12
- `v0.91.7` is now planned as the second required pre-v0.92 bridge tranche
- `v0.92` remains the first true identity, continuity, and birthday milestone
- `v0.95` remains MVP convergence and packaging, not first implementation of
  major cognitive or product systems

Release and issue closeout truth still lives in the milestone release records,
issue cards, PRs, and review packets. This feature list records roadmap
placement and capability state; it does not close or approve milestones.

Recent completed/planned milestone reading:
- `v0.91.4` completed the C-SDLC default-operation hardening band
- `v0.91.5` carried bridge/tooling/provider/public-record pressure toward the
  pre-v0.92 path
- `v0.91.6` and `v0.91.7` are the bridge/readiness tranches before v0.92
  activation; v0.91.6 has landed WP-11 demo/proof convergence and WP-12
  quality gate, with internal review `#4582` next
- most recently completed tooling/workflow-pressure milestone package: `v0.91.2`
- most recently completed inhabited-runtime milestone package: `v0.91.1`
- most recently completed moral-governance milestone package: `v0.91`
- most recently completed governed-tools milestone package: `v0.90.5`
- most recently completed bounded economics milestone package: `v0.90.4`
- most recently completed citizen-state substrate milestone package: `v0.90.3`
- most recently completed Runtime v2 foundation milestone package: `v0.90.1`
- most recently completed long-lived runtime milestone package: `v0.90`
- most recently completed adversarial-runtime milestone package before that: `v0.89.1`
- most recently completed governed-adaptation milestone package before that: `v0.89`

That means the feature story should be read this way:
- `v0.7` through `v0.89.1` provide the implemented platform baseline
- `v0.90` adds the implemented long-lived runtime, inspection, demo,
  compression, repo-visibility, and refactoring band
- `v0.90.1` adds the implemented Runtime v2 foundation and read-only CSM
  Observatory band
- `v0.90.2` added the implemented first bounded CSM run and Runtime v2
  hardening band
- `v0.90.3` is the completed citizen-state substrate band: private-state
  authority, signed envelopes, local sealing, lineage, witnesses, standing,
  access control, redacted projections, challenge/appeal, and the inhabited
  Observatory demo
- `v0.90.4` is the completed bounded citizen-economics and contract-market band
- `v0.90.5` is the completed Governed Tools v1.0 and first Comms / ACIP tranche
- `v0.91` is the completed moral-governance, cognitive-being, structured planning /
  SRP, and secure Agent Comms milestone
- `v0.91.1` is the completed inhabited-runtime readiness band
- `v0.91.2` is the completed tooling, evaluation, productization, publication,
  and workflow pressure-release band
- `v0.91.3` is the completed first Cognitive SDLC implementation slice
- `v0.91.4` is the completed Cognitive SDLC default-operation hardening band
- `v0.91.5` through `v0.91.7` are the pre-v0.92 bridge, review, logging,
  tooling, reliability, security, resilience, Curiosity, Constructability, and
  reasoning-graph readiness path
- `v0.92` through `v0.95` are the planned identity, governance, secure
  execution, product-proof, and MVP-convergence bands

## ADL at a Glance

ADL already provides a serious platform baseline:
- deterministic workflow execution
- a real Rust runtime and CLI
- bounded concurrency, retry, and failure policy
- signing, verification, and trust surfaces
- trace, artifact, and replay-oriented reviewability
- provider and transport substrate
- structured authoring and task-bundle workflow
- review and validation contracts
- bounded Godel-style experimentation
- ObsMem indexing, retrieval, and shared-memory substrate
- bounded cognitive and agency-oriented proof paths
- operational skills and control-plane workflow substrate
- Godel agents as the long-running self-reference and adaptation arc
- Godel-Hadamard-Bayes (GHB) as the cognitive compression and reasoning-control
  algorithm lineage
- reviewer-facing milestone packages and demo matrices

## Feature Status Matrix

| Feature band | Status now | Current proof surface | Completion target |
| --- | --- | --- | --- |
| Deterministic workflow execution | Implemented | runtime/CLI, examples, milestone docs | Completed baseline by `v0.8` |
| ExecutionPlan runtime | Implemented | Rust runtime and plan execution | Completed baseline by `v0.8` |
| Sequential + fork/join coordination | Implemented | examples, tests, demo docs | Completed baseline by `v0.8` |
| Bounded concurrency and retry/failure controls | Implemented | runtime semantics, tests, `v0.7` docs, and `v0.8` crosswalk evidence | Completed baseline by `v0.8` |
| Run artifacts and replay-oriented inspection | Implemented baseline | run artifacts, trace/review docs, milestone demos | Completed baseline in `v0.90`; inhabited-runtime integration complete in `v0.91.1` |
| Signing, verification, and trust policy | Implemented baseline | signing/verification surfaces, trust docs | Completed baseline in `v0.90`; enterprise hardening complete in `v0.93` |
| Provider and transport substrate | Implemented baseline | provider docs, HTTP/local provider surfaces, reviewer package | Runtime/provider completion target: `v0.92` |
| Remote execution baseline | Implemented baseline | bounded remote execution surfaces and docs | Runtime/security completion target: `v0.93` |
| Human-in-the-loop pause/resume | Implemented baseline; AEE-required subset pulled forward | runtime/control surfaces and review docs | AEE-required queue/wake/handoff subset targeted for `v0.92`; broad operator UX polish by `v0.95` |
| Structured authoring model | Implemented baseline | SIP/STP/SPP/SRP/SOR contracts and prompt tooling | MVP completion target: `v0.95` |
| Structured planning and Structured Review Prompt workflow | Implemented baseline | `v0.91` SPP/SRP feature docs, readiness records, validation tooling, and issue-bundle workflow surfaces; historical records may still mention the older Structured Review Policy wording | Completed baseline by `v0.91.0`; validator/closeout hardening continues in `v0.91.1` |
| Control-plane lifecycle | Implemented baseline; AEE-required truth surfaces pulled forward | `pr init/create/start/run/finish`, doctor, janitor, closeout surfaces | AEE/control-path truth subset targeted for `v0.91.5`/`v0.92`; broad hardening by `v0.95` |
| Editor and command-adapter surfaces | Implemented baseline | editor docs, demos, bounded command adapters | MVP completion target: `v0.95` |
| Review and validation surfaces | Implemented baseline | reviewer contracts, validation tools, review packages | MVP completion target: `v0.95` |
| Task-bundle workflow | Implemented baseline | issue/task bundles and public execution records | MVP completion target: `v0.95` |
| Agency, cognitive loop, and cognitive stack | Implemented baseline | `v0.86` agency/cognition feature package, demos, and review artifacts | Cognitive-being completion target: `v0.91`; identity/governance integration complete by `v0.93` |
| Fast/slow thinking and cognitive arbitration | Implemented baseline | `v0.86` feature docs and bounded proof package | Moral-cognition completion target: `v0.91`; MVP integration complete by `v0.95` |
| Bounded Godel loop | Implemented baseline | `v0.8` runtime artifacts, demos, experiment surfaces, `v0.89` experiment package | Godel-agent completion target: `v0.92` |
| Godel agents and Godel-Hadamard-Bayes algorithm | Partially implemented | `v0.8` bounded Godel loop, `v0.89` Godel experiment system, GHB execution/state-space-compression idea docs, reasoning-pattern substrate, ObsMem, trace, cognitive proof surfaces, and `docs/milestones/v0.91.7/features/GODEL_MECHANICS_BRIDGE_v0.91.7.md` | GHB runtime/story explicit in `v0.91.1`; paper packet in `v0.91.2`; first true Godel-agent birthday complete in `v0.92` |
| ObsMem indexing, retrieval, and evidence-aware ranking | Implemented baseline | `v0.8` / `v0.87` proof surfaces plus `v0.89` D6 retrieval/ranking proof | Memory/identity completion target: `v0.92` |
| Shared ObsMem foundation | Implemented baseline | `v0.87` shared-memory docs and proof surfaces | Identity/continuity completion target: `v0.92` |
| Trace validation, trace review, and trace-to-memory ingestion | Implemented baseline | `v0.87` trace schema/emission/artifact/review package and trace-ObsMem docs | Signed/queryable trace completion target: `v0.94` |
| Bounded cognitive path | Implemented baseline | `v0.86` cognitive demo/artifact package | Cognitive-being completion target: `v0.91`; MVP integration complete by `v0.95` |
| Freedom Gate baseline | Implemented baseline | `v0.86` bounded cognitive proof path | Completed baseline by `v0.86` |
| Freedom Gate v2 | Implemented baseline | `v0.89` judgment-boundary and gate proof surfaces | Governance completion target: `v0.93` |
| Trace substrate | Implemented baseline | `v0.87` trace docs and reviewer-facing proof surfaces | Runtime v2 integration complete in `v0.91.1`; signed/queryable trace complete by `v0.94` |
| Operational skills substrate | Implemented baseline | `v0.87` skills/control-plane docs and operational demos | MVP completion target: `v0.95` |
| Runtime environment and lifecycle completion | Implemented baseline | `v0.87.1` runtime docs, demos, and review package | Agent lifecycle completion target: `v0.91.1` |
| Execution boundaries and capability-aware local execution | Implemented baseline | `v0.87.1` runtime, local-model, and capability-aware execution docs/demos | Capability/evaluator completion target: `v0.91.2` |
| Local runtime resilience and Shepherd preservation | Implemented baseline | `v0.87.1` resilience and preservation docs/demos | Runtime resilience completion target: `v0.91.2` |
| Chronosense / temporal substrate | Implemented baseline | `v0.88` feature package and review surfaces | Identity/governance integration complete by `v0.93` |
| Temporal query, retrieval, identity semantics, and continuity hooks | Implemented baseline | `v0.88` temporal schema, retrieval, and continuity/identity feature docs | Identity completion target: `v0.92` |
| Commitments, deadlines, and bounded temporal causality | Implemented baseline | `v0.88` feature docs and reviewer package | Governance completion target: `v0.93` |
| Cost model, accounting primitives, and bounded economics hooks | Implemented baseline | `v0.88` cost-model feature docs and planning surfaces plus `docs/milestones/v0.91.7/features/ECONOMICS_CONTEXT_DECISION_v0.91.7.md` | Bounded economics baseline complete in `v0.90.4`; MVP integration complete by `v0.95` |
| PHI-style integration metrics | Implemented baseline | `v0.88` feature docs and review surfaces | Evaluation completion target: `v0.91.2` |
| Instinct and bounded agency | Implemented baseline | `v0.88` feature docs, instinct review surface, Paper Sonata | Cognitive-being completion target: `v0.91`; governance integration complete by `v0.93` |
| Paper Sonata public-facing proof surface | Implemented baseline | `demo_v088_paper_sonata.sh` and milestone docs | Publication/demo catalog completion target: `v0.95` |
| Deep-agents comparative proof | Implemented baseline | `demo_v088_deep_agents_comparative_proof.sh` and `v0.89` follow-on demo docs | Publication/demo catalog completion target: `v0.95` |
| AEE 1.0 convergence | Implemented baseline; subsystem closure explicitly pulled forward | `v0.89` `control_path/convergence.json`, D1 proof row, feature doc, `docs/milestones/v0.91.5/features/AEE_COMPLETION_TRANCHE_v0.91.5.md`, `docs/milestones/v0.91.5/features/V092_ACTIVATION_READINESS_v0.91.5.md`, and `docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md` | Runtime-inhabitant integration complete in `v0.91.1`; closure tranche defined in `v0.91.5`; subsystem proof targeted for `v0.92`; MVP polish only by `v0.95` |
| Decision, action, and skill-governance surfaces | Implemented baseline | `v0.89` and `v0.89.1` decision/action/skill docs, runtime/proof surfaces | Governance completion target: `v0.93` |
| Delegation, refusal, and coordination contracts | Implemented baseline | `v0.89.1` operational-skill and coordination package | Governance completion target: `v0.93` |
| Provider-extension packaging and safe extension boundaries | Implemented baseline | `v0.89.1` provider-extension package and proof surfaces | Provider/security completion target: `v0.93` |
| Security, posture, and trust-under-adversary package | Implemented baseline | `v0.89` security posture package plus `v0.89.1` adversarial-runtime proof surfaces | Enterprise security completion target: `v0.93` |
| Adversarial runtime, exploit/replay, and self-attack band | Implemented baseline | `v0.89.1` issue wave and feature package | Enterprise security completion target: `v0.93` |
| Demo proof entry points and quality gate | Implemented baseline | `v0.89.1` demo matrix, proof-entry work, quality gate, and release review surfaces | MVP demo/review completion target: `v0.95` |
| Five-agent Hey Jude MIDI demo | Implemented baseline | `v0.89.1` planning/proof surfaces and demo package | Demo catalog completion target: `v0.95` |
| arXiv paper writer and three-paper program | Implemented baseline | `v0.89.1` skills/publication package | Publication packet completion target: `v0.91.2`; MVP publication lane complete by `v0.95` |
| Long-lived supervisor, heartbeat, and cycle artifacts | Implemented baseline | `v0.90` feature contracts, runtime surfaces, and stock-league demo package | Inhabited-runtime completion target: `v0.91.1` |
| Stock-league long-lived demo family | Implemented baseline | `v0.90` stock-league scaffold, recurring run, and proof artifacts | Demo catalog completion target: `v0.95` |
| Minimal status/inspection boundary | Implemented baseline | `v0.90` trace/status issue, CLI/report surfaces, and review gate | Observatory/runtime completion target: `v0.91.1` |
| CodeFriend review showcase and architecture-document generation | Implemented baseline | `v0.90` repo-review, diagram, product-report, and architecture-doc skill/demo package | Productization completion target: `v0.91.2`; MVP completion target: `v0.95` |
| Coverage ratchet, test tracker, and quality tracking | Implemented baseline | `v0.90` coverage/test tracker updates and quality-gate docs | Runtime/test-cycle recovery target: `v0.91.2`; MVP quality target: `v0.95` |
| Rust refactoring tracker and evidence-driven maintenance | Implemented baseline | `v0.90` refactoring tracker, ADR remediation, and follow-on maintenance planning | Rustdoc/tooling cleanup target: `v0.91.2`; MVP hardening target: `v0.95` |
| Milestone compression and repo visibility prototypes | Implemented baseline | `v0.90` compression and repo-visibility docs/proofs | Repo-visibility follow-on target: `v0.91.2`; fuller repo-cognition convergence target: `v0.95` |
| HTML milestone dashboard and compression reporting | MVP-scoped | `docs/milestones/v0.95/features/HTML_MILESTONE_DASHBOARD_AND_COMPRESSION_REPORTING_v0.95.md` plus the milestone-dashboard tooling baseline | Dashboard/compression completion target: `v0.95` |
| Runtime v2 foundation prototype | Implemented baseline | `v0.90.1` feature contracts, Runtime v2 WPs, integrated demo, and proof packet | Foundation complete; hardened by `v0.90.2` |
| CSM Observatory visibility and operator-report surfaces | Implemented baseline | visibility packet, static console, operator report, CLI bundle, command packet design, v0.90.2 operator report integration, v0.90.3 redacted projections, multimode UI architecture, inhabited flagship demo, `docs/milestones/v0.91.5/features/DEMO_AND_UNITY_OBSERVATORY_READINESS_v0.91.5.md`, and `docs/milestones/v0.91.6/features/OBSERVATORY_UNITY_CONSUMPTION_CLASSIFICATION_v0.91.6.md` | Active agent surface completion target: `v0.91.1`; Observatory/Unity consumption bridge target: `v0.91.6` |
| Runtime v2 hardening, recovery, quarantine, and expanded invariants | Implemented baseline | `v0.90.2` implementation docs, proof packets, tests, and demo matrix | Completed baseline by `v0.90.2` |
| First bounded CSM run | Implemented baseline | `v0.90.2` integrated first-run demo, feature-proof coverage, CSM run packet, Observatory report | Completed baseline by `v0.90.2` |
| Third-party review and review-quality gates | Implemented baseline | v0.90.1 WP-15A, v0.90.2 review-tail planning, review handoff packets, finding disposition | Release-tail completion target: every milestone through `v0.95` |
| ANRM / shepherd-model experiments | Partially implemented | v0.90.1 ANRM Gemma scaffold comparison and ten-trial results | Placement complete in `v0.91.1`; evaluator/training path complete by `v0.95` |
| CSM Shepherd model and Gemma training path | MVP-scoped | `docs/milestones/v0.95/features/CSM_SHEPHERD_AND_GEMMA_TRAINING_PATH_v0.95.md` plus ANRM comparison/trace-dataset foundations | Placement and trace dataset complete in `v0.91.1`; training/evaluator MVP complete by `v0.95` |
| Capability-testing evidence and Aptitude Atlas boundary | Post-MVP Aptitude Atlas productization; v0.95 evidence consumption only | capability/aptitude harness baseline and `docs/milestones/v0.95/features/APTITUDE_ATLAS_MODEL_EVALUATION_PLATFORM_v0.95.md` | Harness baseline complete in `v0.91.1`; `v0.95` consumes capability-testing evidence only; Aptitude Atlas product/baseline productization is post-v0.95 |
| Governed tool calls and capability contracts | Implemented baseline | `docs/milestones/v0.90.5` Governed Tools v1.0 planning, Universal Tool Schema, ADL Capability Contract, and tool-to-capability compiler design | Completed baseline; benchmark and conformance expansion in `v0.91.2` |
| Cognitive Compression Cost instrumentation | Implemented first pass | CCC v0 fixture extractor, generated comparison report, validation command, metric draft, and milestone-compression planning | Trace-backed metric completion target: `v0.91.1`; reporting completion target: `v0.95` |
| Web-based code editor integration | MVP-scoped | `docs/milestones/v0.95/features/WEB_BASED_CODE_EDITOR_INTEGRATION_v0.95.md` plus the HTA editor-planning baseline | Editor/operator completion target: `v0.95` |
| Reasoning graph baseline | MVP-scoped | `docs/milestones/v0.94/features/REASONING_GRAPH_BASELINE_v0.94.md` | Reasoning/provenance completion target: `v0.94` |
| Signed trace and trace query | MVP-scoped | `docs/milestones/v0.94/features/SIGNED_TRACE_AND_TRACE_QUERY_v0.94.md` | Reasoning/provenance completion target: `v0.94` |
| Wellbeing, affect, kindness, moral cognition, humor | Implemented baseline | `v0.91` feature docs, Runtime v2 proof surfaces, demo matrix, feature-proof coverage, release evidence, ADR 0016, and `docs/milestones/v0.91.7/features/AFFECT_HAPPINESS_BRIDGE_v0.91.7.md` | Completed `v0.91.0` baseline; consumed by `v0.91.1` inhabited-runtime work |
| Secure Agent Communication and Invocation Protocol | Implemented baseline plus active hardening | v0.90.5 ACIP planning plus v0.91 secure local Agent Comms, A2A boundary docs, proof coverage, and ADR 0017 | `v0.91.0` substrate complete; local hardening baseline completes in `v0.91.1` |
| Inhabited runtime readiness | Implemented baseline | `docs/milestones/v0.91.1` WBS, sprint plan, issue-wave YAML, readiness docs, demo matrix, feature index, and completed WP issue wave | Completed baseline by `v0.91.1` |
| Runtime/polis architecture alignment | Implemented baseline | `docs/milestones/v0.91.1/features/RUNTIME_POLIS_ARCHITECTURE.md` plus landed docs/runtime alignment from closed `WP-02` / `#2824` | Completed baseline by `v0.91.1` |
| Agent lifecycle state model | Implemented baseline | `docs/milestones/v0.91.1/features/AGENT_LIFECYCLE_STATE_MODEL.md` plus landed runtime/tests from closed `WP-03` / `#2825` | Completed baseline by `v0.91.1` |
| CSM Observatory active agent runtime | Implemented baseline | `docs/milestones/v0.91.1/features/CSM_OBSERVATORY_ACTIVE_SURFACE.md` plus landed runtime/tests from closed `WP-04` / `#2826` | Completed baseline by `v0.91.1` |
| Citizen standing and citizen state follow-on | Implemented baseline | `docs/milestones/v0.91.1/features/CITIZEN_STANDING_MODEL.md`, `docs/milestones/v0.91.1/features/CITIZEN_STATE_SUBSTRATE.md`, and landed runtime/fixture proof from closed `WP-05` / `#2827` and `WP-06` / `#2828` | Completed baseline by `v0.91.1` |
| Memory, Theory of Mind, capability testing, intelligence metrics, governed learning, and ANRM/Gemma | Implemented baseline | `docs/milestones/v0.91.1/features/README.md` plus landed proof surfaces from closed `WP-07` through `WP-12` / `#2829` through `#2834` | Completed baseline by `v0.91.1` |
| ACIP hardening and local encryption boundary | Implemented baseline | `docs/milestones/v0.91.1/features/ACIP_HARDENING.md` plus landed hardening/conformance work from closed `WP-13` / `#2835` | Completed local hardening baseline by `v0.91.1` |
| A2A adapter boundary | Implemented baseline | `docs/milestones/v0.91.1/features/A2A_ADAPTER_BOUNDARY.md` and closed `WP-14` / `#2836` | Completed baseline by `v0.91.1` |
| Runtime inhabitant proof | Implemented baseline | `docs/milestones/v0.91.1/features/RUNTIME_INHABITANT_PROOF.md` and closed `WP-15` / `#2837` | Completed baseline by `v0.91.1` |
| UTS + ACC multi-model benchmark and provider-native tool-call comparison | Implemented baseline | `docs/milestones/v0.91.2/features/UTS_ACC_MULTI_MODEL_BENCHMARK.md` plus `docs/milestones/v0.91.2/review/uts_benchmark_evidence/` | `v0.91.2` ADL benchmark baseline complete; standalone UTS repo migration continues under `#3107` |
| Runtime/test-cycle recovery and coverage ergonomics | Implemented baseline | `docs/milestones/v0.91.2/features/RUNTIME_TEST_CYCLE_RECOVERY.md`, runtime recovery reports, coverage ergonomics report, and CI runtime budget docs | `v0.91.2` baseline complete with follow-on observability/hardening continuing as needed |
| CodeFriend repo-review product layer | Implemented baseline | `docs/milestones/v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md` plus existing review-product proof baseline | `v0.91.2` productization lane complete; MVP polish continues by `v0.95` |
| Review heuristics and reviewer demo lane | Implemented baseline | `docs/milestones/v0.91.2/features/REVIEW_HEURISTICS_AND_DEMOS.md` | `v0.91.2` review/demo baseline complete |
| Google Workspace CMS bridge and Rust-native adapter boundary | Implemented baseline plus active hardening | `docs/milestones/v0.91.2/features/GOOGLE_WORKSPACE_CMS_BRIDGE.md` | Completed bounded baseline in `v0.91.2`; live hardening and future project operational use continue as follow-on work |
| Automated repository modernization and external refactoring integration | Implemented bounded packet | `docs/milestones/v0.91.2/features/CODE_MODERNIZATION_DEMO.md` plus existing Moderne/OpenRewrite and code-modernization planning docs | `v0.91.2` bounded Moderne/OpenRewrite LST demo lane complete |
| Generic speculative decoding runtime acceleration | Implemented evaluation packet | `docs/milestones/v0.91.2/features/SPECULATIVE_DECODING_PROTOTYPE.md`, `docs/milestones/v0.91.2/review/speculative_decoding/speculative_decoding_prototype_packet.md`, and `docs/adr/0022-speculative-decoding-deterministic-commit-boundary.md` | `v0.91.2` bounded evaluation complete; productization deferred pending real backend follow-on |
| Repo visibility follow-on | Implemented baseline | `docs/milestones/v0.91.2/features/REPO_VISIBILITY_FOLLOW_ON.md` plus the `v0.90` repo-visibility baseline | `v0.91.2` follow-on packet complete |
| Publication packet program and GHB paper lane | Implemented packet | `docs/milestones/v0.91.2/features/PUBLICATION_PROGRAM.md` plus GHB paper lineage docs | `v0.91.2` publication-planning packet complete; no publication approval claimed |
| General-intelligence paper packet | Implemented packet | `docs/milestones/v0.91.2/features/GENERAL_INTELLIGENCE_PAPER_PACKET.md` | `v0.91.2` paper packet complete with canonical paper repo as source of manuscript truth |
| Rustdoc/doc cleanup | Implemented baseline | `docs/milestones/v0.91.2/features/RUSTDOC_DOC_CLEANUP.md` | `v0.91.2` cleanup baseline complete |
| Workflow guardrails | Implemented baseline | `docs/milestones/v0.91.2/features/WORKFLOW_GUARDRAILS.md` | `v0.91.2` guardrail baseline complete |
| Cognitive SDLC first slice and transition manifest | Implemented baseline | `docs/milestones/v0.91.3/features/COGNITIVE_SDLC_FIRST_SLICE.md`, `docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md`, `docs/milestones/v0.91.3/features/CARD_LIFECYCLE_INTEGRATION.md`, `docs/milestones/v0.91.3/features/TRANSITION_DAG_AND_SHARD_COORDINATION.md`, `docs/milestones/v0.91.3/features/EVIDENCE_BUNDLE_AND_REVIEW_SYNTHESIS.md`, `docs/milestones/v0.91.3/features/GOVERNED_MERGE_READINESS_GATE.md`, `docs/milestones/v0.91.3/features/SRP_SOR_OBSMEM_HANDOFF.md`, and `docs/milestones/v0.91.3/features/FIVE_MINUTE_SPRINT_FIRST_PROOF.md` | `v0.91.3` proved one bounded Cognitive State Transition; `v0.91.4` hardens repeatable/default operation |
| Cognitive SDLC default operation and five-minute-sprint repeatability | Implemented baseline | `docs/milestones/v0.91.4/features/COGNITIVE_SDLC_DEFAULT_OPERATION.md`, `docs/milestones/v0.91.4/features/CSDL_VALIDATION_AND_ROUTING_HARDENING.md`, `docs/milestones/v0.91.4/features/SOFTWARE_DEVELOPMENT_POLIS_AND_ACTOR_STANDING.md`, `docs/milestones/v0.91.4/features/SHARD_OWNERSHIP_AND_INTERFACE_FREEZE.md`, `docs/milestones/v0.91.4/features/EVIDENCE_CONVERGENCE_REVIEW_SYNTHESIS_AND_SIGNED_TRACE.md`, `docs/milestones/v0.91.4/features/MERGE_READINESS_AND_PR_GATE_HARDENING.md`, `docs/milestones/v0.91.4/features/OBSMEM_TRANSITION_MEMORY_INTEGRATION.md`, `docs/milestones/v0.91.4/features/SPRINT_CONDUCTOR_DEFAULT_CSDL_LANE.md`, `docs/milestones/v0.91.4/features/FIVE_MINUTE_SPRINT_REPEATABILITY.md`, `docs/milestones/v0.91.4/features/ACTIVE_ISSUE_MIGRATION_POLICY.md`, `docs/milestones/v0.91.4/features/PROCESS_DRIFT_REGRESSION_FIXTURES.md`, and `docs/planning/C_SDLC_PROMPT_TEMPLATE_EDITOR_TRANSITION_PLAN.md` | `v0.91.4` completes the C-SDLC rollout and makes it the default software-development path, including validation-tail/proof-latency handling and Parallel Validation Fabric planning so sprint speed does not hide long blocking proof cycles; tracked prompt-record migration consumes the versioned `docs/templates/prompts/1.0.0/` substrate, Rust-owned field validation, browser-assisted human review, and the transition plan created in `v0.91.3` |
| Logging, observability, and tooling proof-loop readiness | Planned bridge work | local workflow-tooling and issue-draft source planning plus `docs/milestones/v0.91.5/CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md` and `docs/milestones/v0.91.6/features/TOOLING_PROOF_LOOP_RELIABILITY_v0.91.6.md` | `v0.91.6` first bridge tranche; must preserve broad release confidence while improving bounded PR/tooling cycle time |
| Resilience, citizen persistence, and operational sleep/wake | Planned required pre-v0.92 bridge | local resilience source planning, `LB-102`, `docs/milestones/v0.91.5/features/AEE_COMPLETION_TRANCHE_v0.91.5.md`, and `docs/milestones/v0.91.6/features/RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md` | `v0.91.6` completion-bound bridge: retry/fault classification, provider/tool/workflow resilience, health persistence, checkpoint/restore, sleep/wake, hibernation, simulation, in-transit custody, migration, replay, and continuity proof |
| Public prompt records export, redaction, validation, and indexing | Planned required pre-v0.92 bridge | local prompt-card authoring surface, public prompt-record export work, `docs/milestones/v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md`, and `docs/milestones/v0.91.6/features/PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md` | `v0.91.6` bridge; local cards remain editable while reviewed public export/redaction/indexing boundaries are made explicit |
| Provider/model reliability and multi-agent readiness | Planned required pre-v0.92 bridge | Sprint 2/remediation proof packets, ANRM/provider planning, Gemma/OpenRouter/local/remote model evidence, local multi-agent demo planning, `docs/milestones/v0.91.5/features/PROVIDER_MODEL_MATRIX_v0.91.5.md`, `docs/milestones/v0.91.5/features/MULTI_AGENT_C_SDLC_OPERATION_v0.91.5.md`, and `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md` | `v0.91.6` bridge; reliability proof must be separated from training or ANRM product claims |
| Security bridge readiness and Continuous Adversarial Verification | Planned required pre-v0.92 bridge | local security planning, `LB-011`, ACIP access-rule docs, citizen-state security docs, moral anti-harm/Freedom Gate validation sources, `docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SCHEDULING_v0.91.5.md`, `docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SOURCE_PACKET_v0.91.5.md`, `docs/milestones/v0.91.5/features/ENTERPRISE_SECURITY_ORGANIZATION_BOUNDARY_v0.91.5.md`, `docs/milestones/v0.91.6/features/SECURITY_BRIDGE_AND_CAV_v0.91.6.md`, and `docs/milestones/v0.91.7/features/SECURITY_RESIDUAL_READINESS_v0.91.7.md` | `v0.91.6`/`v0.91.7` bridge; threat-model refresh, CAV plan, public-record redaction/security checks, provider/model trust expectations, ACIP access/security handoff, and adversarial/malformed-output expectations must be explicit before v0.92 |
| Curiosity Engine and Discovery Substrate | Planned required pre-v0.92 bridge | local curiosity planning, `LB-104`, hypothesis/reasoning graph sources, constructability source material, and `docs/milestones/v0.91.7/features/CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md` | `v0.91.7` unless a smaller slice is pulled into `v0.91.6`; must prove at least one governed discovery cycle before v0.92 activation |
| Constructability Gate for shared ADL reality | Planned required pre-v0.92 bridge | local constructability planning, `LB-097`, related trace/external-anchor planning, and `docs/milestones/v0.91.7/features/CONSTRUCTABILITY_GATE_v0.91.7.md` | `v0.91.7` unless pulled forward; construction-event schema, external-anchor schema, admissibility validator, shared-reality boundary, and proof path before v0.92 |
| Reasoning graph, loop runtime, and `adl.skill.v1` bridge | Planned required pre-v0.92 bridge | local reasoning-graph planning, `LB-101`, `docs/milestones/v0.91.7/features/REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md`, and `docs/milestones/v0.94/features/REASONING_GRAPH_BASELINE_v0.94.md` | `v0.91.7` bridge contract before v0.92; deeper reasoning-graph convergence remains `v0.94` |
| ACP / cognitive profiles runtime surface | Planned | `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md` | `v0.92` |
| ACIP binary schema and WebSocket carrier | Planned | `docs/milestones/v0.91.6/features/ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md`, `docs/milestones/v0.91.7/features/ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md`, and `docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md` | `v0.92` binary/schema carrier; `v0.93` security; `v0.94` signed trace |
| Identity, stable name, and continuity substrate | Planned | `docs/milestones/v0.91.6/features/IDENTITY_CONTINUITY_CAPABILITY_SELECTOR_BRIDGE_v0.91.6.md`, `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md`, and `docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md` | `v0.92` |
| Memory grounding, capability envelope, and birth witnesses/receipt | Planned | `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md` | `v0.92` |
| Memory Palace navigable context topology | Planned v0.92 bridge under development | `docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md` plus context-problem planning | v0.92 bridge route exists; smallest implementation/proof slice must distinguish ObsMem, palace topology, working set, and context cache |
| First true Gödel-agent birthday | Planned | `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md` and `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md` | `v0.92` |
| Constitutional citizenship, rights/duties, and governance review | Planned | `docs/milestones/v0.93/features/CITIZENSHIP_RIGHTS_DUTIES_AND_SOCIAL_CONTRACT_v0.93.md` plus the `v0.93` citizenship/governance allocation plan | `v0.93` |
| Bounded Theory of Mind, relationship, reputation, and shared social memory boundary | Planned | `docs/milestones/v0.93/features/THEORY_OF_MIND_AND_SOCIAL_COGNITION_v0.93.md` and `docs/milestones/v0.93/features/SOCIAL_RELATIONSHIP_REPUTATION_AND_SHARED_MEMORY_v0.93.md` | `v0.93` |
| Delegation, upstream delegation, IAM, standing transition, and challenge/appeal governance | Planned | `docs/milestones/v0.93/features/DELEGATION_IAM_STANDING_AND_APPEAL_GOVERNANCE_v0.93.md` plus local upstream-delegation source planning | `v0.93` |
| Guilds and collective organization | MVP-scoped planned governance surface | `docs/milestones/v0.93/features/GUILDS_AND_COLLECTIVE_ORGANIZATION_v0.93.md` | v0.93 feature doc with v0.95 MVP consumption; must define identity, membership, authority, delegation, shared resources/capabilities, isolation, privacy, trace, and review/challenge boundaries |
| Enterprise security for the ADL polis | Planned | `docs/milestones/v0.93/features/ENTERPRISE_SECURITY_v0.93.md`, `docs/milestones/v0.93/features/SECURITY_WP_S1_ZERO_TRUST_ARCHITECTURE_v0.93.md`, `docs/milestones/v0.93/features/SECURITY_WP_S2_POLICY_ENFORCEMENT_AUTHORIZATION_v0.93.md`, `docs/milestones/v0.93/features/SECURITY_WP_S3_SECRETS_KEYS_CRYPTOGRAPHIC_TRUST_v0.93.md`, `docs/milestones/v0.93/features/SECURITY_WP_S4_AUDIT_COMPLIANCE_INCIDENT_EVIDENCE_v0.93.md`, `docs/milestones/v0.93/features/SECURITY_WP_S5_ISOLATION_DATA_GOVERNANCE_PRIVACY_v0.93.md`, and `docs/milestones/v0.93/features/SECURITY_WP_S6_SECURITY_OPERATIONS_ADVERSARIAL_PROVENANCE_v0.93.md` | `v0.93` |
| Secure execution, policy, identity/auth, isolation, and provider-trust convergence | Planned | `docs/milestones/v0.94/features/SECURE_EXECUTION_AND_TRUST_CONVERGENCE_v0.94.md` and the tracked `v0.94` milestone package | `v0.94`; enterprise-security repo/module separation must be explicitly planned before large code movement |
| Mental time travel / temporal self-projection | Planned | `docs/milestones/v0.94/features/MENTAL_TIME_TRAVEL_v0.94.md` plus the `MTT-v1` source note | `v0.94` |
| Payments, settlement, economic agency, and `x402` / Lightning adapters | Planned | `docs/milestones/v0.94.1/features/PAYMENTS_SETTLEMENT_AND_X402_v0.94.1.md` and the tracked `v0.94.1` milestone package | `v0.94.1` |
| Bounded contract-market and resource-stewardship bridge | Implemented baseline | `docs/milestones/v0.90.4` contract-market docs, proof coverage, and demo matrix | Completed bounded baseline by `v0.90.4` |
| Distributed execution integration | Partially implemented; AEE/multi-agent boundary subset pulled forward | `docs/milestones/v0.95/features/DISTRIBUTED_EXECUTION_INTEGRATION_v0.95.md`, ADR 0003, cluster-execution groundwork docs, and `docs/milestones/v0.91.5/features/AEE_COMPLETION_TRANCHE_v0.91.5.md` | AEE/multi-agent boundary proof targeted for `v0.91.5`/`v0.92`; broad distributed integration by `v0.95` |
| CodeFriend v1 and portable adapter v2 | MVP-scoped planned product proof | `docs/milestones/v0.95/features/CODEFRIEND_V1_PORTABLE_ADAPTER_V2_PROOF_v0.95.md` plus tracked CodeFriend planning | CodeFriend v1 plus adapter v2 must land after v0.92 and before v0.95; broad product UX/accounts/billing/report UX remain post-v0.95 |
| Capability-testing evidence consumption / Aptitude Atlas boundary | MVP-scoped evidence consumption; productization post-v0.95 | capability-testing harness docs, local capability-testing planning, and `LB-046` | v0.95 consumes capability-testing evidence only; Aptitude Atlas product/baseline productization is post-v0.95 |
| Demo catalog and polished MVP walkthrough | Partially implemented | `docs/milestones/v0.95/features/DEMO_CATALOG_AND_MVP_WALKTHROUGH_v0.95.md` plus milestone demo matrices and reviewer packages | `v0.95` |
| Control-plane Rust migration / tooling hardening | Partially implemented | `docs/milestones/v0.95/features/CONTROL_PLANE_RUST_MIGRATION_AND_TOOLING_HARDENING_v0.95.md` and Python-elimination/tooling-hardening planning | `v0.95` |
| Zed integration | MVP-scoped decision | `docs/milestones/v0.95/features/ZED_INTEGRATION_v0.95.md` plus the v0.85 MVP-boundary/editor-planning notes | Complete the adapter or explicitly remove it from MVP scope by `v0.95` |

## Milestone Coverage Crosswalk

This crosswalk keeps the feature list connected to the milestone story. It is
not a release note replacement; it is the map of which capability families ADL
has already landed, is landing now, or has explicitly placed on the path to the
`v0.95` MVP.

| Milestone band | Capability families covered here |
| --- | --- |
| `v0.7` through `v0.8` | Deterministic workflow execution, execution plans, bounded concurrency, replayable artifacts, signing/verification, provider/transport foundations, bounded Godel-style experimentation, the first bounded Godel loop, and early ObsMem indexing/retrieval. |
| `v0.85` | Runtime/product positioning, demo discipline, and pre-cognitive milestone scaffolding that later milestones consume rather than repeat. |
| `v0.86` | Agency, bounded cognitive system, cognitive loop/stack, fast/slow thinking, arbitration, Freedom Gate baseline, and local cognitive proof demos. |
| `v0.87` | Trace schema/emission/artifacts, trace validation, trace review, trace-to-ObsMem ingestion, shared ObsMem, provider substrate, operational skills, and PR/control-plane workflow surfaces. |
| `v0.87.1` | Runtime environment completion, agent lifecycle, execution boundaries, capability-aware local model execution, local runtime resilience, and Shepherd preservation. |
| `v0.88` | Chronosense, temporal schema, temporal retrieval/query, identity/continuity semantics, commitments/deadlines, bounded temporal causality, PHI metrics, cost-model hooks, instinct, bounded agency, Paper Sonata, and deep-agents comparative proof. |
| `v0.89` | AEE 1.0 convergence, Freedom Gate v2, decision/action mediation, skill governance, Godel experiment system, GHB algorithm/execution/state-space-compression backgrounders, ObsMem evidence ranking, security posture, threat/trust surfaces, ADL Constitution/reasonableness/learning backgrounders, and governed-adaptation proof. |
| `v0.89.1` | Adversarial runtime, red/blue proof surfaces, exploit artifacts, replay manifests, continuous verification, self-attack, operational skills, skill composition, delegation/refusal/coordination, provider-extension packaging, demo proof entry points, five-agent Hey Jude, arXiv writing workflow, and quality gate. |
| `v0.90` | Long-lived supervisor, heartbeat, cycle manifests, artifact contracts, continuity handles, operator safety, status/inspection boundary, stock-league demos, repo visibility, milestone compression, CodeFriend predecessor showcase, architecture-document generation, coverage ratchet, Rust refactoring tracker, ADR remediation, internal review, and third-party review closeout. |
| `v0.90.1` | Runtime v2 foundation, manifold/snapshot contracts, kernel/control-plane boundaries, provisional citizens, invariant/security-boundary proof, CSM Observatory visibility packet, static console, operator report, CLI bundle, command-packet design, ANRM shepherd experiments, third-party review as WP-15A, Aptitude Atlas planning, and CodeFriend product-lane planning. |
| `v0.90.2` | Runtime v2 hardening, expanded invariants, violation artifacts, recovery/quarantine, operator review surfaces, stronger security-boundary evidence, CSM Observatory integration, and first meaningful CSM-run preparation. |
| `v0.90.3` | Citizen state security, standing, canonical private-state authority, signed envelopes, local sealing, append-only lineage, continuity witnesses/receipts, anti-equivocation, sanctuary/quarantine, access control, redacted projections, challenge/appeal/threat-model evidence, inhabited Observatory flagship demo, and forward planning for later governance prerequisites. |
| `v0.90.4` | Bounded contract-market and resource-stewardship bridge, with explicit deferral of payment rails, legal/billing, inter-polis economics, and governed-tool authority. |
| `v0.90.5` | Governed Tools v1.0: Universal Tool Schema, ADL Capability Contract, capability-to-tool binding, policy enforcement, audit, privacy, and model compatibility proof. |
| `v0.91` | Moral governance, wellbeing and happiness, affect, kindness, humor, moral cognition, structured planning / SRP, secure local Agent Comms substrate, A2A boundary planning, cognitive-being flagship demo, review/remediation, next-milestone handoff, and release ceremony. |
| `v0.91.1` | Inhabited-runtime readiness: Runtime v2/polis alignment, agent lifecycle states, CSM Observatory active surfaces, citizen standing/state, memory/identity architecture, Theory of Mind, capability/aptitude testing, intelligence metrics, governed learning, ANRM/Gemma placement, ACIP/A2A hardening, GHB-adjacent runtime evidence, and observatory-visible agent runtime proof. |
| `v0.91.2` | Tooling/evaluation/productization/publication pressure release: UTS + ACC multi-model benchmark, provider-native tool-call comparison, runtime/test-cycle recovery, coverage ergonomics, CodeFriend productization, Google Workspace CMS bridge, modernization demo, publication packets including GHB/general-intelligence source packets, rustdoc/doc cleanup, and workflow guardrails. |
| `v0.91.3` | Completed first Cognitive SDLC implementation slice: one bounded Cognitive State Transition, issue-local operative `SPP`, transition manifest, transition DAG, evidence bundle, governed merge-readiness gate, SRP/SOR memory handoff, and first five-minute-sprint proof surface. |
| `v0.91.4` | Completed Cognitive SDLC default-operation and hardening milestone: default-operation lifecycle, validator/doctor/conductor/editor alignment, Software Development Polis actor standing, shard ownership, tracked durable workflow records, signed trace proof, ObsMem handoff, repeatability metrics, validation-tail/proof-latency handling, Parallel Validation Fabric planning, and active-issue migration policy. |
| `v0.91.5` | Bridge/review/tooling/provider/public-record pressure toward v0.92: AEE completion tranche, prompt-template workflow integration, multi-agent and provider reliability proof, public C-SDLC prompt records, and logging/observability follow-on setup. |
| `v0.91.6` | First required pre-v0.92 bridge tranche: resilience/citizen persistence, logging/tooling proof-loop fixes, public prompt records, provider/model reliability, first ACIP/A2A/security decisions, and feature-doc issue-wave setup. |
| `v0.91.7` | Second required pre-v0.92 bridge tranche: Curiosity Engine, Constructability Gate, reasoning graph / loop / `adl.skill.v1` bridge, residual security readiness, and residual ACIP/A2A/protobuf/JSON projection decisions. |
| `v0.92` | Identity-bearing agent substrate, stable names, cognitive profiles, model/provider capability contracts, continuity across runs, memory grounding, Memory Palace bridge slice, witnesses, receipts, and the first true Gödel-agent birthday. |
| `v0.93` | Constitutional citizenship, rights/duties, social contract, delegation/upstream delegation/IAM, standing/challenge/appeal governance, guilds, relationship model, reputation/trust, shared social memory, ToM/social cognition, security governance, and enterprise security for the ADL polis. |
| `v0.94` | Secure execution, policy-engine and identity/auth convergence, provider trust and isolation, secrets/data governance, signed/queryable trace and reasoning/provenance closure, bounded mental time travel / temporal self-projection, and cross-band convergence before MVP freeze. |
| `v0.94.1` | Payments, settlement, accounting/ledger/economic trace, economic agency, and `x402` / Lightning adapter follow-on work. |
| `v0.95` | MVP convergence, dashboard/compression reporting, Shepherd/Gemma evidence, capability-testing evidence consumption, CodeFriend v1/adapter v2 proof packaging, distributed-substrate integration, polished demo catalog, coherent reviewer/customer walkthrough, control-plane/tooling hardening and Rust refactoring, web-editor baseline, explicit Zed/logistic-split decision boundaries, post-v0.95 disposition map, feature freeze, and the 1.0 scope boundary. |

## Implemented Platform Highlights

### Deterministic Runtime and Execution Semantics

ADL already executes workflows as explicit, deterministic plans rather than
fragile prompt chains. That gives the project its core credibility: readers can
inspect what will run, reason about ordering and failure behavior, and trust
that the runtime is not improvising hidden orchestration logic.

### Real Rust Runtime and CLI

The Rust runtime is not a placeholder. ADL already has a reference runtime and
CLI capable of plan printing, execution, tracing, signing, verification,
artifact emission, and bounded remote/provider interaction. That is the
difference between "a language idea" and "a platform you can actually run."

### Bounded Workflow Coordination

ADL already supports the coordination patterns serious orchestration needs:
sequential execution, fork/join structure, bounded concurrency, retries, and
failure policy. This makes the system useful for real engineering workflows,
not just single-prompt demos.

### Reviewable Artifacts and Proof Surfaces

Every important ADL milestone has pushed toward one principle: execution should
leave behind durable proof surfaces. Trace artifacts, run records, milestone
demo matrices, review handoffs, and local review packages make the platform
inspectable after the fact rather than dependent on oral reconstruction.

### Signing, Verification, and Trust Boundaries

ADL already includes signing and verification surfaces and treats trust as part
of the runtime story. That baseline becomes richer later, but it is already a
real part of the system, not an aspirational security note.

### Provider, Remote, and Transport Substrate

ADL already has real provider/transport structure, including bounded remote
execution and local/provider proof paths. This matters because it establishes
the platform boundary between orchestration logic and execution backends.

### Structured Authoring and Control Plane

The repo now has a real control-plane lifecycle around issue creation,
bootstrap, plan review, run binding, validation, review, and closeout.
SIP/STP/SPP/SRP/SOR records, doctor checks, janitoring, and bounded PR tooling
give ADL a strong authoring and execution spine instead of relying on vague
contributor habit.

### Operational Skills as System Intelligence

Operational skills are now part of ADL's real platform story. They reduce
error, improve determinism, and turn repeated repo operations into bounded,
reviewable execution surfaces rather than free-form prompting.

### Bounded Godel, ObsMem, and Cognitive Substrate

ADL already has real bounded reflective execution, memory participation, and
cognitive proof surfaces:
- `v0.8` established bounded Godel-style experimentation and canonical artifacts
- `v0.86` established the first working bounded cognitive-system proof package
- `v0.87` strengthened trace/provider/shared-memory/skills substrate

These are not disconnected demos. Together they form the core of ADL's claim
that bounded adaptive systems can be both powerful and reviewable.

### Godel Agents and the GHB Algorithm

Godel agents and the Godel-Hadamard-Bayes algorithm are not decorative research
terms in ADL. They are the central reasoning-and-adaptation arc tying together
Godel experiments, reasoning patterns, trace, ObsMem, cognitive compression,
governed learning, identity, and the later birthday milestone.

The current truth boundary is important:
- ADL has an implemented bounded Godel loop and Godel experiment system.
- ADL has GHB execution and state-space-compression design documents that
  explain how Godel, Hadamard-style exploration, and Bayesian evidence updates
  fit together.
- ADL does not yet claim a complete GHB runtime or the first true Godel-agent
  birthday.

The strategic direction is still clear. GHB is ADL's candidate control loop for
how agents expand problem structure, explore alternatives, compress evidence
back into reusable cognitive state, and improve without escaping deterministic
trace, memory, governance, and review surfaces.

## Recently Completed Milestone Bands

### v0.88 - Temporal and Bounded Agency Substrate

`v0.88` is complete as a materially landed milestone package. It added two
major bounded bands:
- temporal / chronosense substrate
- instinct / bounded agency substrate

High-signal `v0.88` achievements include:
- promoted temporal schema, continuity/identity semantics, temporal retrieval,
  commitments/deadlines, bounded temporal causality, PHI metrics, instinct, and
  instinct runtime influence into one canonical feature package
- reviewer-facing proof surfaces for temporal review, PHI review, instinct
  review, and the integrated `v0.88` review surface
- `Paper Sonata` as the flagship public-facing bounded demo
- deep-agents comparative proof as a supporting reviewer-facing row
- a full internal repo-code-review pass completed before 3rd-party review

So the truthful `v0.88` story is:
- core implementation: landed
- review/remediation/closeout: completed through the milestone closeout flow
- milestone value: already very real

### v0.89 - Governed Adaptive Execution

`v0.89` turned governed adaptation into a first-class platform package:
- AEE 1.0 convergence
- Freedom Gate v2
- explicit decision and action mediation surfaces
- skill execution contracts
- security, trust, and posture surfaces serious enough to support adversarial work

The bounded v0.89 story is implemented baseline, not universal completion of
every future adaptive-system idea. Later milestones should consume and deepen
these surfaces rather than restating them as unbuilt from scratch.

### v0.90 - Long-Lived Runtime, Inspection, and Milestone Compression

`v0.90` is complete. It deepened ADL from bounded governed execution into
practical long-lived runtime supervision:
- supervisor and heartbeat behavior
- cycle manifest and artifact contracts
- continuity handles without claiming full identity substrate
- operator stop and guardrail controls
- status and inspection boundaries
- stock-league long-lived demo evidence
- milestone compression and repo visibility prototypes
- evidence-driven Rust refactoring
- internal and third-party release review gates

The truthful v0.90 story is implemented baseline, with Runtime v2 consuming
those surfaces rather than replacing them.

## Current Planning Focus: v0.91.6 / v0.91.7 Bridge To v0.92

`v0.91.4` is no longer the active planning center in this feature list. Its
role is the completed Cognitive SDLC default-operation hardening band that the
current workflow now consumes.

The current planning focus is the pre-v0.92 bridge:

- finish the `v0.91.6` first bridge tranche without overloading it
- plan `v0.91.7` as the second required bridge tranche
- keep v0.92 as the first true identity/continuity/birthday milestone
- keep v0.95 as convergence and packaging, not first implementation of major
  cognitive systems

The completed v0.91.4 bands were:
- default `SIP -> STP -> SPP -> SRP -> SOR` lifecycle operation
- validator, doctor, conductor, sprint-conductor, and editor alignment
- Software Development Polis actor standing and shard ownership rules
- evidence convergence, review synthesis, signed trace, and ObsMem handoff
- merge-readiness and PR gate hardening
- five-minute-sprint repeatability
- validation-tail and Parallel Validation Fabric work
- active-issue migration policy and process-drift regression fixtures

`v0.91.3` remains the completed first-slice milestone that v0.91.4 consumes.

### v0.91.5 - Bridge, Provider, Prompt-Record, And Tooling Pressure

`v0.91.5` carries bridge pressure between the C-SDLC default-operation band and
the pre-v0.92 readiness path. Its feature-list role is not to absorb v0.92, but
to make the handoff reviewable:

- AEE completion tranche and activation-readiness inputs
- prompt-template workflow integration
- public C-SDLC prompt-record transition and export posture
- multi-agent workcell and provider/model reliability evidence
- shared observability/logging contract and follow-on logging mini-sprint
- bridge routing for work that must not be hidden inside v0.92 WP-01

### v0.91.6 - First Required Pre-v0.92 Bridge Tranche

`v0.91.6` should finish the load-bearing bridge work already in motion:

- resilience, citizen persistence, and operational sleep/wake
- logging/tooling proof-loop fixes
- public prompt records export, redaction, validation, and indexing
- provider/model reliability and multi-agent readiness
- first ACIP/A2A/security decisions
- feature-doc production issue-wave setup

It should not pretend to complete every required pre-v0.92 surface if doing so
would hide unfinished bridge work.

### v0.91.7 - Second Required Pre-v0.92 Bridge Tranche

`v0.91.7` is now planned as a narrow second bridge tranche before v0.92:

- Curiosity Engine / Discovery Substrate
- Constructability Gate
- reasoning graph, loop runtime, and `adl.skill.v1` bridge
- residual security bridge readiness
- residual ACIP/A2A/protobuf/JSON projection decisions

This is not a general spillover milestone. It exists so v0.92 can consume
truthful bridge outputs rather than rediscover unresolved scope during WP-01.

## Current And Upcoming Capability Bands

### v0.90.3 - Citizen State, Standing, And Private-State Substrate

`v0.90.3` is complete. Its implementation and review surfaces landed:
- private citizen-state format
- signed envelopes and continuity witnesses
- append-only lineage and anti-equivocation rules
- sanctuary/quarantine behavior
- redacted Observatory projections
- access-control semantics for who may inspect, project, wake, migrate, or
  challenge citizen state

Its release-tail work is complete; later milestones consume these surfaces
instead of reopening the band.

### v0.90.4 - Citizen Economics And Contract Markets

`v0.90.4` is the completed bounded citizen-economics and contract-market band:
- resource stewardship
- contract-market shape
- accounting and allocation evidence
- citizen-safe economic boundaries
- non-payment proof surfaces before any payment rail claims

It consumed the v0.90.3 economics placement bridge without hiding economics
inside citizen-state security work.

### v0.90.5 - Governed Tools v1.0

`v0.90.5` is the completed Governed Tools v1.0 band:
- Universal Tool Schema
- ADL Capability Contract
- capability-to-tool binding
- authority, privacy, and audit semantics
- denial and misuse proof surfaces
- model compatibility testing

Tools are first-class ADL primitives, but the current industry pattern is too
unsafe to treat as a production governance model. This milestone made tool
calls policy-mediated, inspectable, and fail-closed at the completed baseline.

### v0.91 - Affect and Moral Cognition

`v0.91` is where ADL's cognitive architecture became more emotionally and
normatively legible on the released `v0.91.0` line:
- affect
- kindness
- humor/absurdity
- moral cognition and related evaluation surfaces
- Freedom Gate moral event records
- moral trace schema, validation, outcome linkage, metrics, and trajectory
  review
- anti-harm trajectory constraints and bounded harm-prevention proof surfaces
- wellbeing metrics as a second-half diagnostic report after trace,
  validation, outcome-linkage, and trajectory-review foundations
- moral resources and wellbeing links that remain evidence-grounded rather than
  scalar, reward-channel, or rhetorical

This milestone does not claim full constitutional citizenship, final social
contract, production moral agency, or the first true Gödel-agent birthday.

### v0.91.1 - Inhabited Runtime Readiness

`v0.91.1` is the completed bridge between moral/cognitive-being work and the
identity/birthday band. Its job is to make the CSM ready for inhabitant-shaped
runtime proof:
- Runtime v2 and polis architecture alignment
- agent lifecycle states for active, quiescent, suspended, dormant,
  simulation, in-transit, bootstrap, shutdown, and forced-suspension regimes
- CSM Observatory active packet and projection surfaces
- citizen standing and citizen state follow-on implementation
- memory/identity architecture without birthday claims
- Theory of Mind foundation
- capability and aptitude testing foundation
- intelligence metric architecture
- governed learning substrate
- ANRM/Gemma placement, trace extractor, and dataset mapping
- ACIP conformance, local encryption hardening, and A2A adapter boundary
- ACIP reception/invocation eligibility by lifecycle state
- GHB-adjacent evidence from intelligence metrics, governed learning, and
  memory/identity architecture
- runtime inhabitant integration and observatory-visible agent flagship demo

This milestone should not claim the first true birthday, external federation,
legal personhood, or production identity continuity. It prepares the evidence
v0.92 needs.

### v0.91.2 - Tooling, Evaluation, Productization, Publication, And Workflow Pressure Release

`v0.91.2` is the completed pressure-release milestone for work that should not
have overloaded v0.91 or v0.91.1:
- UTS + ACC multi-model benchmark harness
- provider-native tool/function-call comparison
- runtime/test-cycle recovery
- changed-source coverage ergonomics
- CodeFriend review-packet productization and review-skill/demo work
- Google Workspace CMS bridge and Rust-native adapter boundary
- bounded code-modernization demo and review policy
- publication program and general-intelligence paper packet
- Godel agents and GHB source-packet lane, including the planned
  Godel-Hadamard-Bayes paper packet
- rustdoc/doc cleanup
- workflow guardrails for main writes, hung watchers, safe reports, and card
  drift

This milestone reduced operational drag before the C-SDLC first-slice and
default-operation milestones, then the v0.92 identity band and v0.93
constitutional/social-governance band.

### v0.92 - Identity, Capability, and Continuity

`v0.92` is the bridge from bounded cognitive behavior to identity-bearing
agents:
- first-class identity
- ACP / cognitive profiles runtime surface
- provider/model capability contracts
- stable names
- continuity hooks across runs
- Memory Palace bridge slice for navigable long-running context, after the
  context-problem note is reviewed
- first true Gödel-agent birthday
- witnessed memory grounding
- birth record, witness set, receipt, and reviewer packet
- negative cases proving startup, wake, snapshot, admission, and copied state
  are not birth

The birthday is the special center of gravity for this milestone. It should be
reviewable as an event, not asserted as ceremony or inferred from ordinary
runtime lifecycle mechanics.

### v0.93 - Governance, Delegation, Upstream Delegation, IAM, and Social Contract

`v0.93` is expected to turn identity substrate into accountable governance:
- IAM
- delegation
- policy and constitutional surfaces
- rights/duties and social contract surfaces
- guilds and collective organization
- social cognition, relationship, reputation, and shared social memory
- enterprise-security foundations:
  zero-trust architecture, policy enforcement, secrets/key lifecycle,
  audit/compliance evidence, isolation/data governance, and security operations

### Economics And Payment Substrate

`v0.90.4` completed the bounded citizen-economics and contract-market
substrate that was planned for that milestone. Any payment adapters,
settlement rails, Lightning / x402 experiments, or cross-polis economic
integration must be explicitly scheduled before `v0.95` if they are promoted
into MVP scope; otherwise they remain non-claims, not hidden backlog.

### v0.94 - Integration and Dependency Closure

`v0.94` should close the remaining cross-cutting dependency gaps:
- distributed-substrate integration
- cross-band convergence
- MVP dependency cleanup
- reasoning graph and signed/queryable trace completion
- secure execution, policy-engine, identity/auth, isolation, secrets/data
  governance, and provider-trust convergence

### v0.95 - MVP Baseline Convergence and Feature Freeze

`v0.95` is the planned MVP convergence point. It should ensure every
in-roadmap feature has at least a reviewable baseline and clear proof posture;
it does not have to make every subsystem product-complete. Features that remain
unfinished after the MVP must have explicit post-MVP disposition instead of
implicit backlog language.

The v0.95 band includes:
- polished demo catalog
- coherent MVP walkthrough
- control-plane/tooling hardening
- Rust refactoring that reduces change-specific test burden
- CodeFriend v1 and portable adapter v2 proof packaging
- capability-testing evidence consumption without Aptitude Atlas productization
- guilds consumed as an MVP-scoped governance surface
- post-v0.95 disposition map
- feature freeze and `1.0` scope boundary

### Product Lanes: CodeFriend and Aptitude Atlas

Two ADL-powered product directions are now recognized and explicitly scheduled
around the v0.95 path without distorting the core Runtime v2 milestone path:

- CodeFriend: repo-wide code and architecture review, diagrams, tests,
  remediation plans, and product-grade reports powered by ADL review skills.
  CodeFriend v1 and portable adapter v2 must land after v0.92 and before
  v0.95 so MVP convergence can consume real external-repo proof. Broader
  customer UX, accounts, billing, repo connection UI, and report UX are
  post-v0.95 product work.
- Aptitude Atlas: model capability and aptitude assessment with leaderboard-
  style public reporting, deeper than one-off benchmark scores. v0.95 consumes
  capability-testing evidence only; Aptitude Atlas productization and baseline
  product work are post-v0.95.

Both are strategically important, but neither should distort the pre-v0.92
bridge or the v0.92 birthday milestone.

## MVP-Scoped Decision

### Zed Integration

Zed integration is recognized as useful but not yet proven as required. By
`v0.95`, ADL must either ship a bounded editor/operator integration surface for
it or explicitly remove it from MVP scope. It is no longer allowed to sit as
open-ended post-`v0.95` ambiguity in the feature list.

## Summary

ADL already has a substantial platform:
- deterministic execution
- a real Rust runtime
- bounded orchestration semantics
- trust and verification surfaces
- reviewable traces and artifacts
- provider and transport substrate
- structured authoring and control-plane workflow
- operational skills
- bounded Godel, ObsMem, and cognitive proof paths
- Godel agents and the Godel-Hadamard-Bayes algorithm as the core
  self-reference, exploration, evidence-update, and cognitive-compression arc
- completed temporal, bounded-agency, and governed-adaptation milestone work
- completed adversarial-runtime, publication-skill, and long-lived runtime
  milestone work
- completed inhabited-runtime readiness work for v0.91.1

What remains through `v0.95` is not random feature accumulation. It is a
deliberate convergence path:
- preserve v0.91.1 inhabited-runtime work as implemented slices, not
  planning-only foundations
- complete v0.91.3/v0.91.4 C-SDLC first-slice and default-operation work
- complete v0.92 identity/birthday work and v0.93 governance/security work
- close v0.94 integration gaps before the v0.95 MVP freeze
- ensure every in-roadmap feature has a baseline, proof posture, and completion
  or post-MVP disposition
- close the MVP as a serious, reviewable agent-runtime platform

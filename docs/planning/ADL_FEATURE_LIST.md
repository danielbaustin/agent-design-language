# ADL Feature List

## Purpose

This document is the canonical ADL capability overview.

It answers four practical questions:

- what ADL already does today
- what is active in the current milestone
- which major platform bands are planned next
- how the project is expected to converge by `v0.95`

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

- **Implemented**: materially present on `main` with working code, artifacts,
  docs, and/or demo or review surfaces.
- **Implemented baseline**: already real and usable, with later milestones
  deepening, integrating, or hardening it.
- **Active milestone**: materially present and under active closeout/review in
  the current milestone band.
- **Partially implemented**: meaningful enabling surfaces exist, but the
  capability is not yet complete enough to count as a finished platform band.
- **Planned**: primarily a planned milestone/feature band today.
- **MVP-scoped**: not complete yet, but explicitly assigned to a milestone no
  later than `v0.95`.

Every feature row must name the milestone where the feature is complete enough
for the `v0.95` MVP. "Deepen later", "future", and "post-v0.95" are not valid
completion targets in this list.

## Current Repo Status

The current repo truth is:
- active milestone: `v0.91.1`
- current crate version on the active release line: `0.91.1`
- current milestone state: `v0.91` is released and `v0.91.1` has started with
  the full `WP-01` / `#2823` through `WP-24` / `#2846` issue wave open
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
- `v0.91.1` is the active inhabited-runtime readiness band
- `v0.91.2` is the planned tooling, evaluation, productization, publication,
  and workflow pressure-release band
- `v0.92` through `v0.95` are the later planned identity, governance,
  publication, integration, and MVP-convergence bands

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
| Human-in-the-loop pause/resume | Implemented baseline | runtime/control surfaces and review docs | MVP completion target: `v0.95` |
| Structured authoring model | Implemented baseline | STP/SIP/SOR contracts and prompt tooling | MVP completion target: `v0.95` |
| Structured planning and Structured Review Policy workflow | Implemented baseline | `v0.91` SPP/SRP feature docs, readiness records, validation tooling, and issue-bundle workflow surfaces | Completed baseline by `v0.91.0`; validator/closeout hardening continues in `v0.91.1` |
| Control-plane lifecycle | Implemented baseline | `pr init/create/start/run/finish`, doctor, janitor, closeout surfaces | MVP completion target: `v0.95` |
| Editor and command-adapter surfaces | Implemented baseline | editor docs, demos, bounded command adapters | MVP completion target: `v0.95` |
| Review and validation surfaces | Implemented baseline | reviewer contracts, validation tools, review packages | MVP completion target: `v0.95` |
| Task-bundle workflow | Implemented baseline | issue/task bundles and public execution records | MVP completion target: `v0.95` |
| Agency, cognitive loop, and cognitive stack | Implemented baseline | `v0.86` agency/cognition feature package, demos, and review artifacts | Cognitive-being completion target: `v0.91`; identity/governance integration complete by `v0.93` |
| Fast/slow thinking and cognitive arbitration | Implemented baseline | `v0.86` feature docs and bounded proof package | Moral-cognition completion target: `v0.91`; MVP integration complete by `v0.95` |
| Bounded Godel loop | Implemented baseline | `v0.8` runtime artifacts, demos, experiment surfaces, `v0.89` experiment package | Godel-agent completion target: `v0.92` |
| Godel agents and Godel-Hadamard-Bayes algorithm | Partially implemented | `v0.8` bounded Godel loop, `v0.89` Godel experiment system, GHB execution/state-space-compression idea docs, reasoning-pattern substrate, ObsMem, trace, and cognitive proof surfaces | GHB runtime/story explicit in `v0.91.1`; paper packet in `v0.91.2`; first true Godel-agent birthday complete in `v0.92` |
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
| Cost model, accounting primitives, and bounded economics hooks | Implemented baseline | `v0.88` cost-model feature docs and planning surfaces | Bounded economics baseline complete in `v0.90.4`; MVP integration complete by `v0.95` |
| PHI-style integration metrics | Implemented baseline | `v0.88` feature docs and review surfaces | Evaluation completion target: `v0.91.2` |
| Instinct and bounded agency | Implemented baseline | `v0.88` feature docs, instinct review surface, Paper Sonata | Cognitive-being completion target: `v0.91`; governance integration complete by `v0.93` |
| Paper Sonata public-facing proof surface | Implemented baseline | `demo_v088_paper_sonata.sh` and milestone docs | Publication/demo catalog completion target: `v0.95` |
| Deep-agents comparative proof | Implemented baseline | `demo_v088_deep_agents_comparative_proof.sh` and `v0.89` follow-on demo docs | Publication/demo catalog completion target: `v0.95` |
| AEE 1.0 convergence | Implemented baseline | `v0.89` `control_path/convergence.json`, D1 proof row, feature doc | Runtime-inhabitant integration complete in `v0.91.1`; MVP integration complete by `v0.95` |
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
| CodeBuddy review showcase and architecture-document generation | Implemented baseline | `v0.90` repo-review, diagram, product-report, and architecture-doc skill/demo package | Productization completion target: `v0.91.2`; MVP completion target: `v0.95` |
| Coverage ratchet, test tracker, and quality tracking | Implemented baseline | `v0.90` coverage/test tracker updates and quality-gate docs | Runtime/test-cycle recovery target: `v0.91.2`; MVP quality target: `v0.95` |
| Rust refactoring tracker and evidence-driven maintenance | Implemented baseline | `v0.90` refactoring tracker, ADR remediation, and follow-on maintenance planning | Rustdoc/tooling cleanup target: `v0.91.2`; MVP hardening target: `v0.95` |
| Milestone compression and repo visibility prototypes | Implemented baseline | `v0.90` compression and repo-visibility docs/proofs | Repo-visibility follow-on target: `v0.91.2`; fuller repo-cognition convergence target: `v0.95` |
| HTML milestone dashboard and compression reporting | MVP-scoped | backlog item and milestone-compression planning surfaces | Dashboard/compression completion target: `v0.95` |
| Runtime v2 foundation prototype | Implemented baseline | `v0.90.1` feature contracts, Runtime v2 WPs, integrated demo, and proof packet | Foundation complete; hardened by `v0.90.2` |
| CSM Observatory visibility and operator-report surfaces | Implemented baseline | visibility packet, static console, operator report, CLI bundle, command packet design, v0.90.2 operator report integration, v0.90.3 redacted projections, multimode UI architecture, and inhabited flagship demo | Active agent surface completion target: `v0.91.1` |
| Runtime v2 hardening, recovery, quarantine, and expanded invariants | Implemented baseline | `v0.90.2` implementation docs, proof packets, tests, and demo matrix | Completed baseline by `v0.90.2` |
| First bounded CSM run | Implemented baseline | `v0.90.2` integrated first-run demo, feature-proof coverage, CSM run packet, Observatory report | Completed baseline by `v0.90.2` |
| Third-party review and review-quality gates | Implemented baseline | v0.90.1 WP-15A, v0.90.2 review-tail planning, review handoff packets, finding disposition | Release-tail completion target: every milestone through `v0.95` |
| ANRM / shepherd-model experiments | Partially implemented | v0.90.1 ANRM Gemma scaffold comparison and ten-trial results | Placement complete in `v0.91.1`; evaluator/training path complete by `v0.95` |
| CSM Shepherd model and Gemma training path | MVP-scoped | ANRM comparison results, trace-dataset planning, and evaluator/scaffold backlog | Placement and trace dataset complete in `v0.91.1`; training/evaluator MVP complete by `v0.95` |
| Aptitude Atlas model-evaluation platform | MVP-scoped | capability/aptitude planning and product backlog | Harness complete in `v0.91.1`; productized evaluation complete by `v0.95` |
| Governed tool calls and capability contracts | Implemented baseline | `docs/milestones/v0.90.5` Governed Tools v1.0 planning, Universal Tool Schema, ADL Capability Contract, and tool-to-capability compiler design | Completed baseline; benchmark and conformance expansion in `v0.91.2` |
| Cognitive Compression Cost instrumentation | Implemented first pass | CCC v0 fixture extractor, generated comparison report, validation command, metric draft, and milestone-compression planning | Trace-backed metric completion target: `v0.91.1`; reporting completion target: `v0.95` |
| Web-based code editor integration | MVP-scoped | editor backlog issue and skills-wiring planning | Editor/operator completion target: `v0.95` |
| Reasoning graph baseline | MVP-scoped | planning/schema/proof surfaces | Reasoning/provenance completion target: `v0.94` |
| Signed trace and trace query | MVP-scoped | roadmap and planning docs | Reasoning/provenance completion target: `v0.94` |
| Wellbeing, affect, kindness, moral cognition, humor | Implemented baseline | `v0.91` feature docs, Runtime v2 proof surfaces, demo matrix, feature-proof coverage, release evidence, and ADR 0016 | Completed `v0.91.0` baseline; consumed by `v0.91.1` inhabited-runtime work |
| Secure Agent Communication and Invocation Protocol | Implemented baseline plus active hardening | v0.90.5 ACIP planning plus v0.91 secure local Agent Comms, A2A boundary docs, proof coverage, and ADR 0017 | `v0.91.0` substrate complete; local hardening baseline completes in `v0.91.1` |
| Inhabited runtime readiness | Active milestone | `docs/milestones/v0.91.1` WBS, sprint plan, issue-wave YAML, readiness docs, demo matrix, feature index, and WP-01 issue #2823 | `v0.91.1` |
| Runtime/polis architecture alignment | Implemented baseline | `docs/milestones/v0.91.1/features/RUNTIME_POLIS_ARCHITECTURE.md` plus landed docs/runtime alignment from closed `WP-02` / `#2824` | Completed baseline by `v0.91.1` |
| Agent lifecycle state model | Implemented baseline | `docs/milestones/v0.91.1/features/AGENT_LIFECYCLE_STATE_MODEL.md` plus landed runtime/tests from closed `WP-03` / `#2825` | Completed baseline by `v0.91.1` |
| CSM Observatory active agent runtime | Implemented baseline | `docs/milestones/v0.91.1/features/CSM_OBSERVATORY_ACTIVE_SURFACE.md` plus landed runtime/tests from closed `WP-04` / `#2826` | Completed baseline by `v0.91.1` |
| Citizen standing and citizen state follow-on | Implemented baseline | `docs/milestones/v0.91.1/features/CITIZEN_STANDING_MODEL.md`, `docs/milestones/v0.91.1/features/CITIZEN_STATE_SUBSTRATE.md`, and landed runtime/fixture proof from closed `WP-05` / `#2827` and `WP-06` / `#2828` | Completed baseline by `v0.91.1` |
| Memory, Theory of Mind, capability testing, intelligence metrics, governed learning, and ANRM/Gemma | Implemented baseline | `docs/milestones/v0.91.1/features/README.md` plus landed proof surfaces from closed `WP-07` through `WP-12` / `#2829` through `#2834` | Completed baseline by `v0.91.1` |
| ACIP hardening and local encryption boundary | Implemented baseline | `docs/milestones/v0.91.1/features/ACIP_HARDENING.md` plus landed hardening/conformance work from closed `WP-13` / `#2835` | Completed local hardening baseline by `v0.91.1` |
| A2A adapter boundary | Planned | `docs/milestones/v0.91.1/features/A2A_ADAPTER_BOUNDARY.md` and open `WP-14` / `#2836` | `v0.91.1` |
| Runtime inhabitant proof | Planned | `docs/milestones/v0.91.1/features/RUNTIME_INHABITANT_PROOF.md` and open `WP-15` / `#2837` | `v0.91.1` |
| UTS + ACC multi-model benchmark and provider-native tool-call comparison | Planned | `docs/milestones/v0.91.2/features/UTS_ACC_MULTI_MODEL_BENCHMARK.md` | `v0.91.2` |
| Runtime/test-cycle recovery and coverage ergonomics | Planned | `docs/milestones/v0.91.2/features/RUNTIME_TEST_CYCLE_RECOVERY.md` | `v0.91.2` |
| CodeBuddy repo-review product layer | Planned | `docs/milestones/v0.91.2/features/CODEBUDDY_PRODUCTIZATION.md` plus existing CodeBuddy proof baseline | `v0.91.2` productization lane |
| Review heuristics and reviewer demo lane | Planned | `docs/milestones/v0.91.2/features/REVIEW_HEURISTICS_AND_DEMOS.md` | `v0.91.2` |
| Google Workspace CMS bridge and Rust-native adapter boundary | Planned | `docs/milestones/v0.91.2/features/GOOGLE_WORKSPACE_CMS_BRIDGE.md` | `v0.91.2` |
| Automated repository modernization and external refactoring integration | Planned | `docs/milestones/v0.91.2/features/CODE_MODERNIZATION_DEMO.md` plus existing Moderne/OpenRewrite and code-modernization planning docs | `v0.91.2` bounded Moderne/OpenRewrite LST demo lane |
| Generic speculative decoding runtime acceleration | Planned | `.adl/docs/TBD/ADL_AND_GENERIC_SPECULATIVE_DECODING.md`, `.adl/docs/TBD/ADL_AND_SPECULATIVE_CODING_REPLAY.md`, and `docs/milestones/v0.91.2/features/SPECULATIVE_DECODING_PROTOTYPE.md` | `v0.91.2` |
| Repo visibility follow-on | Planned | `docs/milestones/v0.91.2/features/REPO_VISIBILITY_FOLLOW_ON.md` plus the `v0.90` repo-visibility baseline | `v0.91.2` |
| Publication packet program and GHB paper lane | Planned | `docs/milestones/v0.91.2/features/PUBLICATION_PROGRAM.md` plus GHB paper lineage docs | `v0.91.2` |
| General-intelligence paper packet | Planned | `docs/milestones/v0.91.2/features/GENERAL_INTELLIGENCE_PAPER_PACKET.md` | `v0.91.2` |
| Rustdoc/doc cleanup | Planned | `docs/milestones/v0.91.2/features/RUSTDOC_DOC_CLEANUP.md` | `v0.91.2` |
| Workflow guardrails | Planned | `docs/milestones/v0.91.2/features/WORKFLOW_GUARDRAILS.md` | `v0.91.2` |
| ACP / cognitive profiles runtime surface | Planned | `v0.92` identity/birthday allocation plan and cognitive-profile planning source | `v0.92` |
| Identity, stable name, and continuity substrate | Planned | `v0.92` identity, continuity, and birthday allocation plan | `v0.92` |
| Memory grounding, capability envelope, and birth witnesses/receipt | Planned | `v0.92` identity/birthday planning docs and Runtime v2 birthday boundary roadmap | `v0.92` |
| First true Gödel-agent birthday | Planned | `v0.92` identity, continuity, and birthday allocation plan plus Runtime v2 birthday boundary roadmap | `v0.92` |
| Constitutional citizenship, rights/duties, and governance review | Planned | `docs/milestones/v0.93/features/CITIZENSHIP_RIGHTS_DUTIES_AND_SOCIAL_CONTRACT_v0.93.md` plus the `v0.93` citizenship/governance allocation plan | `v0.93` |
| Bounded Theory of Mind, relationship, reputation, and shared social memory boundary | Planned | `docs/milestones/v0.93/features/THEORY_OF_MIND_AND_SOCIAL_COGNITION_v0.93.md` and `docs/milestones/v0.93/features/SOCIAL_RELATIONSHIP_REPUTATION_AND_SHARED_MEMORY_v0.93.md` | `v0.93` |
| Delegation, IAM, standing transition, and challenge/appeal governance | Planned | `v0.93` constitutional citizenship and polis-governance allocation plan | `v0.93` |
| Enterprise security for the ADL polis | Planned | `docs/milestones/v0.93/features/ENTERPRISE_SECURITY_v0.93.md` and the `v0.93` zero-trust/security work breakdown | `v0.93` |
| Secure execution, policy, identity/auth, isolation, and provider-trust convergence | Planned | `docs/milestones/v0.94/features/SECURE_EXECUTION_AND_TRUST_CONVERGENCE_v0.94.md` and the tracked `v0.94` milestone package | `v0.94` |
| Mental time travel / temporal self-projection | Planned | `docs/milestones/v0.94/features/MENTAL_TIME_TRAVEL_v0.94.md` plus the `MTT-v1` source note | `v0.94` |
| Payments, settlement, economic agency, and `x402` / Lightning adapters | Planned | `docs/milestones/v0.94.1/features/PAYMENTS_SETTLEMENT_AND_X402_v0.94.1.md` and the tracked `v0.94.1` milestone package | `v0.94.1` |
| Bounded contract-market and resource-stewardship bridge | Implemented baseline | `docs/milestones/v0.90.4` contract-market docs, proof coverage, and demo matrix | Completed bounded baseline by `v0.90.4` |
| Distributed execution integration | Partially implemented | cluster groundwork plus planning docs | Integration completion target: `v0.94`; MVP hardening target: `v0.95` |
| Demo catalog and polished MVP walkthrough | Partially implemented | milestone demo matrices and reviewer packages | `v0.95` |
| Control-plane Rust migration / tooling hardening | Partially implemented | mixed Rust/shell control plane and active tooling hardening | `v0.95` |
| Zed integration | MVP-scoped decision | planning docs only | Complete the adapter or explicitly remove it from MVP scope by `v0.95` |

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
| `v0.90` | Long-lived supervisor, heartbeat, cycle manifests, artifact contracts, continuity handles, operator safety, status/inspection boundary, stock-league demos, repo visibility, milestone compression, CodeBuddy showcase, architecture-document generation, coverage ratchet, Rust refactoring tracker, ADR remediation, internal review, and third-party review closeout. |
| `v0.90.1` | Runtime v2 foundation, manifold/snapshot contracts, kernel/control-plane boundaries, provisional citizens, invariant/security-boundary proof, CSM Observatory visibility packet, static console, operator report, CLI bundle, command-packet design, ANRM shepherd experiments, third-party review as WP-15A, Aptitude Atlas planning, and CodeBuddy product-lane planning. |
| `v0.90.2` | Runtime v2 hardening, expanded invariants, violation artifacts, recovery/quarantine, operator review surfaces, stronger security-boundary evidence, CSM Observatory integration, and first meaningful CSM-run preparation. |
| `v0.90.3` | Citizen state security, standing, canonical private-state authority, signed envelopes, local sealing, append-only lineage, continuity witnesses/receipts, anti-equivocation, sanctuary/quarantine, access control, redacted projections, challenge/appeal/threat-model evidence, inhabited Observatory flagship demo, and forward planning for later governance prerequisites. |
| `v0.90.4` | Bounded contract-market and resource-stewardship bridge, with explicit deferral of payment rails, legal/billing, inter-polis economics, and governed-tool authority. |
| `v0.90.5` | Governed Tools v1.0: Universal Tool Schema, ADL Capability Contract, capability-to-tool binding, policy enforcement, audit, privacy, and model compatibility proof. |
| `v0.91` | Moral governance, wellbeing and happiness, affect, kindness, humor, moral cognition, structured planning / SRP, secure local Agent Comms substrate, A2A boundary planning, cognitive-being flagship demo, review/remediation, next-milestone handoff, and release ceremony. |
| `v0.91.1` | Inhabited-runtime readiness: Runtime v2/polis alignment, agent lifecycle states, CSM Observatory active surfaces, citizen standing/state, memory/identity architecture, Theory of Mind, capability/aptitude testing, intelligence metrics, governed learning, ANRM/Gemma placement, ACIP/A2A hardening, GHB-adjacent runtime evidence, and observatory-visible agent runtime proof. |
| `v0.91.2` | Tooling/evaluation/productization/publication pressure release: UTS + ACC multi-model benchmark, provider-native tool-call comparison, runtime/test-cycle recovery, coverage ergonomics, CodeBuddy productization, Google Workspace CMS bridge, modernization demo, publication packets including GHB/general-intelligence source packets, rustdoc/doc cleanup, and workflow guardrails. |
| `v0.92` | Identity-bearing agent substrate, stable names, cognitive profiles, model/provider capability contracts, continuity across runs, memory grounding, witnesses, receipts, and the first true Gödel-agent birthday. |
| `v0.93` | Constitutional citizenship, rights/duties, social contract, delegation/IAM, standing/challenge/appeal governance, relationship model, reputation/trust, shared social memory, ToM/social cognition, and enterprise security for the ADL polis. |
| `v0.94` | Secure execution, policy-engine and identity/auth convergence, provider trust and isolation, secrets/data governance, signed/queryable trace and reasoning/provenance closure, bounded mental time travel / temporal self-projection, and cross-band convergence before MVP freeze. |
| `v0.94.1` | Payments, settlement, accounting/ledger/economic trace, economic agency, and `x402` / Lightning adapter follow-on work. |
| `v0.95` | MVP convergence, distributed-substrate integration, polished demo catalog, coherent reviewer/customer walkthrough, control-plane/tooling hardening, feature freeze, and the 1.0 scope boundary. |

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
bootstrap, run binding, validation, and closeout. STP/SIP/SOR records, doctor
checks, janitoring, and bounded PR tooling give ADL a strong authoring and
execution spine instead of relying on vague contributor habit.

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

## Current Active Milestone: v0.91.1

`v0.91.1` is the active milestone. Its job is to turn the inhabited-runtime
readiness plan into concrete Runtime v2 / polis, lifecycle, state, memory,
Theory of Mind, capability, intelligence, learning, ACIP/A2A hardening, and
Observatory-visible agent proof surfaces.

The current active bands are:
- Runtime v2 and polis architecture alignment
- agent lifecycle state and ACIP eligibility
- CSM Observatory active packets and projections
- citizen standing and citizen state follow-on implementation
- memory/identity architecture without birthday claims
- Theory of Mind foundations
- capability and aptitude testing
- intelligence metric architecture and governed learning
- ANRM/Gemma placement and trace-dataset architecture
- ACIP conformance, local encryption hardening, and A2A adapter boundary
- runtime inhabitant integration and observatory-visible agent flagship demo

The first true Gödel-agent birthday remains a later milestone event. `v0.91.1`
prepares the runtime evidence that v0.92 needs; it does not claim full
identity, production citizenship, legal personhood, external federation, or
complete constitutional authority.

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

`v0.91.1` is the active bridge between moral/cognitive-being work and the
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

`v0.91.2` is the planned pressure-release milestone for work that should not
overload v0.91 or v0.91.1:
- UTS + ACC multi-model benchmark harness
- provider-native tool/function-call comparison
- runtime/test-cycle recovery
- changed-source coverage ergonomics
- CodeBuddy review-packet productization and review-skill/demo work
- Google Workspace CMS bridge and Rust-native adapter boundary
- bounded code-modernization demo and review policy
- publication program and general-intelligence paper packet
- Godel agents and GHB source-packet lane, including the planned
  Godel-Hadamard-Bayes paper packet
- rustdoc/doc cleanup
- workflow guardrails for main writes, hung watchers, safe reports, and card
  drift

This milestone should reduce operational drag before the v0.92 identity band
and v0.93 constitutional/social-governance band.

### v0.92 - Identity, Capability, and Continuity

`v0.92` is the bridge from bounded cognitive behavior to identity-bearing
agents:
- first-class identity
- ACP / cognitive profiles runtime surface
- provider/model capability contracts
- stable names
- continuity hooks across runs
- first true Gödel-agent birthday
- witnessed memory grounding
- birth record, witness set, receipt, and reviewer packet
- negative cases proving startup, wake, snapshot, admission, and copied state
  are not birth

The birthday is the special center of gravity for this milestone. It should be
reviewable as an event, not asserted as ceremony or inferred from ordinary
runtime lifecycle mechanics.

### v0.93 - Governance, Delegation, IAM, and Social Contract

`v0.93` is expected to turn identity substrate into accountable governance:
- IAM
- delegation
- policy and constitutional surfaces
- rights/duties and social contract surfaces
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

### v0.95 - MVP Convergence and Feature Freeze

`v0.95` is the planned convergence point:
- polished demo catalog
- coherent MVP walkthrough
- control-plane/tooling hardening
- feature freeze and `1.0` scope boundary

### Product Lanes: CodeBuddy and Aptitude Atlas

Two ADL-powered product directions are now recognized and explicitly scheduled
inside the v0.95 path without distorting the core Runtime v2 milestone path:

- CodeBuddy: repo-wide code and architecture review, diagrams, tests,
  remediation plans, and product-grade reports powered by ADL review skills.
- Aptitude Atlas: model capability and aptitude assessment with leaderboard-
  style public reporting, deeper than one-off benchmark scores.

Both are strategically important. CodeBuddy productization belongs to
`v0.91.2` with MVP polish by `v0.95`; Aptitude Atlas begins with the v0.91.1
capability harness and reaches productized MVP shape by `v0.95`.

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
- active inhabited-runtime readiness work for v0.91.1

What remains through `v0.95` is not random feature accumulation. It is a
deliberate convergence path:
- complete v0.91.1 inhabited-runtime work as implemented slices, not
  planning-only foundations
- complete v0.91.2 tooling, evaluation, publication, and workflow-pressure
  release work
- complete v0.92 identity/birthday work and v0.93 governance/security work
- close v0.94 integration gaps before the v0.95 MVP freeze
- close the MVP as a serious, reviewable agent-runtime platform

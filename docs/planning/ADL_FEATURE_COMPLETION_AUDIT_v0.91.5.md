# ADL Feature Completion Audit v0.91.5

## Status

Draft audit for issue `#3526`.

## Purpose

This audit reconciles the canonical ADL feature list with milestone planning
truth. It answers:

- which feature bands are already baseline-complete
- which feature bands still have planned completion targets
- where the roadmap gives a milestone target but no calendar date
- where a feature is too implicit or too late
- whether the Adaptive Execution Engine (AEE) needs an explicit pull-forward
  tranche before `v0.95`

## Source Baseline

- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/explainers/AEE.md`
- `docs/milestones/v0.8/ADAPTIVE_EXECUTION_ENGINE.md`
- `docs/milestones/v0.8/BOUNDED_AEE_V1_SCOPE_V0.8.md`
- `docs/milestones/v0.89/features/AEE_CONVERGENCE_MODEL.md`
- `docs/milestones/v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md`
- `docs/milestones/v0.91.5/WBS_v0.91.5.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/milestones/v0.93/WBS_v0.93.md`
- `docs/milestones/v0.94/WBS_v0.94.md`
- `docs/milestones/v0.94.1/WBS_v0.94.1.md`
- `docs/milestones/v0.95/WBS_v0.95.md`

## Date Policy

The current roadmap usually names milestone completion targets, not exact
calendar dates. This audit therefore uses:

- `Completed by <milestone>` when the feature-list target names a completed
  baseline.
- `Target: <milestone>; calendar date TBD` when the work is planned.
- `Every milestone through <milestone>` when the feature is a recurring release
  gate.

No exact calendar release date is invented here.

## Key Findings

1. The canonical feature list contains 110 feature rows.
2. Every row has a milestone target or milestone-bound target language.
3. Many rows are baseline-complete but still carry later integration targets;
   those should be read as "baseline done, integration not done."
4. Adaptive Execution Engine (AEE) is the clearest roadmap-truth gap. The
   feature list says AEE 1.0 has an implemented baseline, but full MVP
   integration is deferred to `v0.95`.
5. If AEE is meant to feel like a completed subsystem before MVP convergence, it
   needs an explicit completion tranche before `v0.95`.
6. Several `v0.95` rows should be split into earlier prerequisite tranches so
   the MVP milestone is convergence and polish, not first subsystem closure.
7. `v0.91.5` should be treated as a real completion target for bridge work:
   multi-agent stabilization, provider/model breadth, public prompt records,
   v0.92 activation readiness, and AEE closure-definition work.

## Adaptive Execution Engine (AEE) Assessment

### Current Truth

| AEE surface | Evidence | Audit result |
| --- | --- | --- |
| Bounded retry/recovery substrate | `docs/milestones/v0.8/ADAPTIVE_EXECUTION_ENGINE.md`, `docs/milestones/v0.8/BOUNDED_AEE_V1_SCOPE_V0.8.md` | Baseline implemented, but explicitly not full adaptive autonomy. |
| Convergence model | `docs/milestones/v0.89/features/AEE_CONVERGENCE_MODEL.md` | Landed bounded convergence contract: progress signals, stop conditions, strategy-change visibility, and convergence records. |
| Runtime-inhabitant integration | `docs/planning/ADL_FEATURE_LIST.md` row "AEE 1.0 convergence" | Claimed complete in `v0.91.1`, but as integration language, not subsystem closure. |
| MVP integration | `docs/planning/ADL_FEATURE_LIST.md` row "AEE 1.0 convergence" | Deferred to `v0.95`, too broad if AEE is expected to be done as a named subsystem. |

### AEE Done Criteria Needed

AEE should not be considered complete merely because the MVP convergence package
eventually feels coherent. A named AEE completion tranche should define and
prove:

- steering semantics
- queueing, wake, and handoff semantics
- distributed execution boundary
- control-path truth and reviewability
- policy and budget stop conditions
- trace/replay proof for continue, stop, strategy-change, and handoff outcomes
- demo or proof surface for end-to-end AEE behavior
- explicit non-goals, especially no unconstrained self-modification or hidden
  adaptive state

### Recommendation

Pull AEE subsystem completion forward into `v0.91.5` planning or `v0.92`
activation readiness, depending on scope pressure:

- Minimum safe move: add an AEE completion tranche to `v0.91.5` as planning and
  closure-definition work, then schedule implementation/proof in `v0.92`.
- Stronger move: make AEE closure a `v0.91.5` bridge work package if it is a
  precondition for multi-agent, provider/model, or first-birthday readiness.
- Leave only polish, demo catalog placement, and MVP walkthrough integration in
  `v0.95`.

## Pull-Forward Candidates

| Feature | Current target | Why it needs pull-forward | Recommended routing |
| --- | --- | --- | --- |
| AEE 1.0 convergence | MVP integration by `v0.95` | Too implicit; subsystem closure is diffused across distributed execution, control plane, proof/demo convergence, and release-tail review. | Pull forward now: `#3534` should define the AEE completion tranche in `v0.91.5`, then schedule implementation/proof no later than `v0.92` unless operator review routes a bounded subset to `v0.91.5`. |
| Human-in-the-loop pause/resume | MVP target `v0.95` | AEE queue/wake/handoff semantics should not wait for generic MVP convergence if they are needed for first-birthday or multi-agent proofs. | Pull the AEE-required subset into the AEE closure tranche. Broad UX polish may remain `v0.95`, but queue/wake/handoff semantics should target `v0.92`. |
| Control-plane lifecycle | MVP target `v0.95` | AEE control-path truth depends on lifecycle and reviewability. | Pull the AEE-required control-path subset into `v0.91.5`/`v0.92`; leave broad Rust migration and polish to `v0.95`. |
| Distributed execution integration | `v0.95` | Multi-agent and AEE distributed boundaries need earlier proof to make `v0.92` activation credible. | Pull a bounded AEE/multi-agent distributed-boundary proof into `v0.91.5` or `v0.92`; leave broad distributed execution integration in `v0.95`. |
| Multi-agent stabilization | Bridge work not represented as a feature-list completion target | It is now core C-SDLC operational hardening, not optional side work. | Treat as `v0.91.5` completion work with proof of role/delegation usefulness before `v0.92` opens. |
| Provider/model matrix and OpenRouter breadth | Bridge work not represented as a feature-list completion target | Multi-agent role selection and aptitude testing need model breadth before first-birthday activation. | Treat as `v0.91.5` completion work, with hosted/local/remote/OpenRouter evidence surfaces. |
| Public C-SDLC prompt records | Bridge work not represented as a feature-list completion target | Durable public prompt records are part of making C-SDLC observable rather than local habit. | Treat as `v0.91.5` completion work before `v0.92` activation. |
| v0.92 activation readiness map | Bridge work not represented as a feature-list completion target | v0.92 brings many dormant surfaces alive; surprises here would destabilize the birthday milestone. | Treat as `v0.91.5` completion work and use `#3377` as final go/no-go readiness. |
| Generic speculative decoding productization | Productization deferred pending backend | It is the only row that is not cleanly scheduled as a productization path. | Add a decision boundary before `v0.95`: either identify a viable backend/productization lane or explicitly defer/remove it from MVP scope. |

## Pull-Forward Decisions

The audit recommends the following scheduling policy:

| Decision | Result |
| --- | --- |
| Do not leave AEE completion buried in `v0.95`. | `v0.91.5` defines the completion tranche; `v0.92` should carry implementation/proof unless the tranche proves a smaller `v0.91.5` closure is safe. |
| Do not pull all of `v0.95` forward. | Pull only prerequisite subsets needed for AEE, multi-agent, provider breadth, public prompt records, and v0.92 activation readiness. |
| Treat `v0.91.5` as a real bridge-completion milestone. | Add explicit completion expectations for multi-agent stabilization, provider/model matrix, public prompt records, and activation readiness. |
| Keep `v0.95` as convergence/polish. | Leave broad demo catalog polish, broad distributed execution, broad control-plane Rust migration, and product UX convergence in `v0.95` unless needed earlier by AEE or v0.92 activation. |
| Resolve unscheduled speculative decoding productization. | Add a pre-`v0.95` decision gate rather than leaving productization open-ended. |

## Completion Table

The table below represents every feature row in
`docs/planning/ADL_FEATURE_LIST.md`. "Completion date" is milestone-based unless
the source docs provide an exact calendar date.

| # | Feature | Status now | Proof surface summary | Completion target/date | Audit disposition |
| --- | --- | --- | --- | --- | --- |
| 1 | Deterministic workflow execution | Implemented | Runtime/CLI, examples, milestone docs | Completed by `v0.8` | Baseline complete. |
| 2 | ExecutionPlan runtime | Implemented | Rust runtime and plan execution | Completed by `v0.8` | Baseline complete. |
| 3 | Sequential + fork/join coordination | Implemented | Examples, tests, demo docs | Completed by `v0.8` | Baseline complete. |
| 4 | Bounded concurrency and retry/failure controls | Implemented | Runtime semantics, tests, `v0.7` docs, `v0.8` crosswalk | Completed by `v0.8` | Baseline complete. |
| 5 | Run artifacts and replay-oriented inspection | Implemented baseline | Run artifacts, trace/review docs, milestone demos | Baseline `v0.90`; integration `v0.91.1` | Complete with later integration already recorded. |
| 6 | Signing, verification, and trust policy | Implemented baseline | Signing/verification surfaces, trust docs | Baseline `v0.90`; enterprise hardening `v0.93` | Baseline complete; hardening scheduled. |
| 7 | Provider and transport substrate | Implemented baseline | Provider docs, HTTP/local provider surfaces, reviewer package | Target `v0.92`; calendar TBD | Scheduled. |
| 8 | Remote execution baseline | Implemented baseline | Bounded remote execution surfaces and docs | Target `v0.93`; calendar TBD | Scheduled. |
| 9 | Human-in-the-loop pause/resume | Implemented baseline | Runtime/control surfaces and review docs | AEE-required subset target `v0.92`; broad UX/MVP polish `v0.95`; calendar TBD | Pull forward the AEE queue/wake/handoff subset; leave broad polish scheduled for `v0.95`. |
| 10 | Structured authoring model | Implemented baseline | SIP/STP/SPP/SRP/SOR contracts and prompt tooling | Target `v0.95`; calendar TBD | Baseline complete; MVP integration scheduled. |
| 11 | Structured planning and Structured Review Prompt workflow | Implemented baseline | `v0.91` SPP/SRP docs, readiness records, validation tooling, issue bundles | Baseline `v0.91.0`; hardening `v0.91.1` | Baseline complete. |
| 12 | Control-plane lifecycle | Implemented baseline | `pr init/create/start/run/finish`, doctor, janitor, closeout | AEE-required subset target `v0.91.5`/`v0.92`; broad hardening `v0.95`; calendar TBD | Pull forward the AEE control-path/reviewability subset; leave broad hardening scheduled. |
| 13 | Editor and command-adapter surfaces | Implemented baseline | Editor docs, demos, bounded command adapters | Target `v0.95`; calendar TBD | Scheduled. |
| 14 | Review and validation surfaces | Implemented baseline | Reviewer contracts, validation tools, review packages | Target `v0.95`; calendar TBD | Scheduled. |
| 15 | Task-bundle workflow | Implemented baseline | Issue/task bundles and public execution records | Target `v0.95`; calendar TBD | Scheduled. |
| 16 | Agency, cognitive loop, and cognitive stack | Implemented baseline | `v0.86` agency/cognition package, demos, review artifacts | Cognitive-being `v0.91`; identity/governance `v0.93` | Baseline complete; integration scheduled. |
| 17 | Fast/slow thinking and cognitive arbitration | Implemented baseline | `v0.86` feature docs and proof package | Moral cognition `v0.91`; MVP `v0.95` | Baseline complete; MVP integration scheduled. |
| 18 | Bounded Godel loop | Implemented baseline | `v0.8` artifacts, demos, experiment surfaces, `v0.89` package | Target `v0.92`; calendar TBD | Scheduled for birthday milestone. |
| 19 | Godel agents and Godel-Hadamard-Bayes algorithm | Partially implemented | Bounded Godel loop, `v0.89` experiments, GHB docs, ObsMem, trace, cognitive proof | Runtime/story `v0.91.1`; paper `v0.91.2`; birthday `v0.92` | Scheduled for `v0.92` closure. |
| 20 | ObsMem indexing, retrieval, and evidence-aware ranking | Implemented baseline | `v0.8`/`v0.87` proof plus `v0.89` D6 proof | Target `v0.92`; calendar TBD | Scheduled. |
| 21 | Shared ObsMem foundation | Implemented baseline | `v0.87` shared-memory docs/proof | Target `v0.92`; calendar TBD | Scheduled. |
| 22 | Trace validation, trace review, and trace-to-memory ingestion | Implemented baseline | `v0.87` trace package and trace-ObsMem docs | Target `v0.94`; calendar TBD | Scheduled. |
| 23 | Bounded cognitive path | Implemented baseline | `v0.86` cognitive demo/artifact package | Cognitive-being `v0.91`; MVP `v0.95` | Baseline complete; MVP integration scheduled. |
| 24 | Freedom Gate baseline | Implemented baseline | `v0.86` bounded cognitive proof path | Completed by `v0.86` | Baseline complete. |
| 25 | Freedom Gate v2 | Implemented baseline | `v0.89` judgment-boundary and gate proof | Target `v0.93`; calendar TBD | Scheduled. |
| 26 | Trace substrate | Implemented baseline | `v0.87` trace docs and proof surfaces | Runtime v2 `v0.91.1`; signed/queryable `v0.94` | Baseline complete; closure scheduled. |
| 27 | Operational skills substrate | Implemented baseline | `v0.87` skills/control-plane docs and demos | Target `v0.95`; calendar TBD | Scheduled. |
| 28 | Runtime environment and lifecycle completion | Implemented baseline | `v0.87.1` runtime docs, demos, review package | Target `v0.91.1` | Complete. |
| 29 | Execution boundaries and capability-aware local execution | Implemented baseline | `v0.87.1` runtime/local-model docs and demos | Target `v0.91.2` | Complete. |
| 30 | Local runtime resilience and Shepherd preservation | Implemented baseline | `v0.87.1` resilience and preservation docs/demos | Target `v0.91.2` | Complete. |
| 31 | Chronosense / temporal substrate | Implemented baseline | `v0.88` feature package and review surfaces | Target `v0.93`; calendar TBD | Scheduled. |
| 32 | Temporal query, retrieval, identity semantics, and continuity hooks | Implemented baseline | `v0.88` temporal schema/retrieval/continuity docs | Target `v0.92`; calendar TBD | Scheduled. |
| 33 | Commitments, deadlines, and bounded temporal causality | Implemented baseline | `v0.88` feature docs and review package | Target `v0.93`; calendar TBD | Scheduled. |
| 34 | Cost model, accounting primitives, and bounded economics hooks | Implemented baseline | `v0.88` cost-model docs and planning | Baseline `v0.90.4`; MVP `v0.95` | Baseline complete; MVP integration scheduled. |
| 35 | PHI-style integration metrics | Implemented baseline | `v0.88` feature docs and review surfaces | Target `v0.91.2` | Complete. |
| 36 | Instinct and bounded agency | Implemented baseline | `v0.88` feature docs, instinct review, Paper Sonata | Cognitive-being `v0.91`; governance `v0.93` | Baseline complete; governance scheduled. |
| 37 | Paper Sonata public-facing proof surface | Implemented baseline | `demo_v088_paper_sonata.sh` and milestone docs | Target `v0.95`; calendar TBD | Scheduled for demo catalog. |
| 38 | Deep-agents comparative proof | Implemented baseline | `demo_v088_deep_agents_comparative_proof.sh`, `v0.89` demo docs | Target `v0.95`; calendar TBD | Scheduled for demo catalog. |
| 39 | AEE 1.0 convergence | Implemented baseline | `v0.89` convergence contract names control-path convergence output, D1 proof row, feature doc | Runtime integration `v0.91.1`; closure-definition `v0.91.5`; subsystem implementation/proof target `v0.92`; MVP polish `v0.95` | Pulled forward: no longer acceptable to leave AEE subsystem closure only inside broad `v0.95` convergence. |
| 40 | Decision, action, and skill-governance surfaces | Implemented baseline | `v0.89`/`v0.89.1` decision/action/skill docs and proof | Target `v0.93`; calendar TBD | Scheduled. |
| 41 | Delegation, refusal, and coordination contracts | Implemented baseline | `v0.89.1` operational-skill and coordination package | Target `v0.93`; calendar TBD | Scheduled. |
| 42 | Provider-extension packaging and safe extension boundaries | Implemented baseline | `v0.89.1` provider-extension package and proof | Target `v0.93`; calendar TBD | Scheduled. |
| 43 | Security, posture, and trust-under-adversary package | Implemented baseline | `v0.89` security package and `v0.89.1` adversarial proof | Target `v0.93`; calendar TBD | Scheduled. |
| 44 | Adversarial runtime, exploit/replay, and self-attack band | Implemented baseline | `v0.89.1` issue wave and feature package | Target `v0.93`; calendar TBD | Scheduled. |
| 45 | Demo proof entry points and quality gate | Implemented baseline | `v0.89.1` demo matrix, proof-entry, quality gate, release review | Target `v0.95`; calendar TBD | Scheduled for MVP demo/review. |
| 46 | Five-agent Hey Jude MIDI demo | Implemented baseline | `v0.89.1` planning/proof and demo package | Target `v0.95`; calendar TBD | Scheduled for demo catalog. |
| 47 | arXiv paper writer and three-paper program | Implemented baseline | `v0.89.1` skills/publication package | Packet `v0.91.2`; MVP lane `v0.95` | Packet complete; MVP lane scheduled. |
| 48 | Long-lived supervisor, heartbeat, and cycle artifacts | Implemented baseline | `v0.90` feature contracts, runtime surfaces, stock-league demo | Target `v0.91.1` | Complete. |
| 49 | Stock-league long-lived demo family | Implemented baseline | `v0.90` stock-league scaffold, recurring run, proof artifacts | Target `v0.95`; calendar TBD | Scheduled for demo catalog. |
| 50 | Minimal status/inspection boundary | Implemented baseline | `v0.90` trace/status issue, CLI/report, review gate | Target `v0.91.1` | Complete. |
| 51 | CodeFriend review showcase and architecture-document generation | Implemented baseline | `v0.90` repo-review, diagram, report, architecture-doc skill/demo package | Productization `v0.91.2`; MVP `v0.95` | Productization complete; MVP polish scheduled. |
| 52 | Coverage ratchet, test tracker, and quality tracking | Implemented baseline | `v0.90` coverage/test tracker and quality-gate docs | Recovery `v0.91.2`; MVP `v0.95` | Recovery complete; MVP quality scheduled. |
| 53 | Rust refactoring tracker and evidence-driven maintenance | Implemented baseline | `v0.90` refactoring tracker, ADR remediation, maintenance planning | Cleanup `v0.91.2`; MVP `v0.95` | Cleanup complete; MVP hardening scheduled. |
| 54 | Milestone compression and repo visibility prototypes | Implemented baseline | `v0.90` compression and repo-visibility docs/proofs | Follow-on `v0.91.2`; convergence `v0.95` | Follow-on complete; MVP convergence scheduled. |
| 55 | HTML milestone dashboard and compression reporting | MVP-scoped | `v0.95` feature doc and milestone-dashboard tooling baseline | Target `v0.95`; calendar TBD | Scheduled. |
| 56 | Runtime v2 foundation prototype | Implemented baseline | `v0.90.1` contracts, Runtime v2 WPs, demo, proof packet | Foundation complete; hardened by `v0.90.2` | Complete. |
| 57 | CSM Observatory visibility and operator-report surfaces | Implemented baseline | Visibility packet, static console, operator report, CLI bundle, reports and demos | Target `v0.91.1` | Complete baseline. |
| 58 | Runtime v2 hardening, recovery, quarantine, and expanded invariants | Implemented baseline | `v0.90.2` docs, proof packets, tests, demo matrix | Completed by `v0.90.2` | Complete. |
| 59 | First bounded CSM run | Implemented baseline | `v0.90.2` first-run demo, proof coverage, CSM packet, Observatory report | Completed by `v0.90.2` | Complete. |
| 60 | Third-party review and review-quality gates | Implemented baseline | Review handoff packets and finding disposition | Every milestone through `v0.95` | Recurring gate, not one-time complete. |
| 61 | ANRM / shepherd-model experiments | Partially implemented | v0.90.1 ANRM Gemma scaffold comparison and ten-trial results | Placement `v0.91.1`; evaluator/training `v0.95` | Placement complete; evaluator scheduled. |
| 62 | CSM Shepherd model and Gemma training path | MVP-scoped | `v0.95` feature doc plus ANRM/trace dataset foundations | Placement `v0.91.1`; MVP `v0.95` | Scheduled. |
| 63 | Aptitude Atlas model-evaluation platform | MVP-scoped | `v0.95` feature doc plus capability/aptitude harness baseline | Harness `v0.91.1`; productized `v0.95` | Scheduled. |
| 64 | Governed tool calls and capability contracts | Implemented baseline | Governed Tools v1.0, UTS, ACC, compiler design | Baseline complete; expansion `v0.91.2` | Complete baseline. |
| 65 | Cognitive Compression Cost instrumentation | Implemented first pass | CCC fixture extractor, comparison report, validation, metric draft | Metric `v0.91.1`; reporting `v0.95` | First pass complete; reporting scheduled. |
| 66 | Web-based code editor integration | MVP-scoped | `v0.95` feature doc plus HTA editor-planning baseline | Target `v0.95`; calendar TBD | Scheduled. |
| 67 | Reasoning graph baseline | MVP-scoped | `v0.94` feature doc | Target `v0.94`; calendar TBD | Scheduled. |
| 68 | Signed trace and trace query | MVP-scoped | `v0.94` feature doc | Target `v0.94`; calendar TBD | Scheduled. |
| 69 | Wellbeing, affect, kindness, moral cognition, humor | Implemented baseline | `v0.91` feature docs, Runtime v2 proof, demo matrix, release evidence, ADR 0016 | Baseline `v0.91.0`; consumed by `v0.91.1` | Complete baseline; activation testing still relevant for `v0.92`. |
| 70 | Secure Agent Communication and Invocation Protocol | Implemented baseline plus active hardening | ACIP planning, local Agent Comms, A2A docs, proof coverage, ADR 0017 | Substrate `v0.91.0`; hardening `v0.91.1` | Complete local baseline; later carrier/security scheduled. |
| 71 | Inhabited runtime readiness | Implemented baseline | `v0.91.1` WBS, issue wave, readiness docs, demo matrix, feature index | Completed by `v0.91.1` | Complete. |
| 72 | Runtime/polis architecture alignment | Implemented baseline | `v0.91.1` feature doc and closed WP-02/#2824 | Completed by `v0.91.1` | Complete. |
| 73 | Agent lifecycle state model | Implemented baseline | `v0.91.1` feature doc and closed WP-03/#2825 | Completed by `v0.91.1` | Complete. |
| 74 | CSM Observatory active agent runtime | Implemented baseline | `v0.91.1` feature doc and closed WP-04/#2826 | Completed by `v0.91.1` | Complete. |
| 75 | Citizen standing and citizen state follow-on | Implemented baseline | `v0.91.1` feature docs and closed WP-05/#2827, WP-06/#2828 | Completed by `v0.91.1` | Complete. |
| 76 | Memory, Theory of Mind, capability testing, intelligence metrics, governed learning, and ANRM/Gemma | Implemented baseline | `v0.91.1` feature README and closed WP-07 through WP-12 | Completed by `v0.91.1` | Complete baseline; v0.92 activation map still needs testing. |
| 77 | ACIP hardening and local encryption boundary | Implemented baseline | `v0.91.1` ACIP hardening and closed WP-13/#2835 | Completed by `v0.91.1` | Complete local baseline. |
| 78 | A2A adapter boundary | Implemented baseline | `v0.91.1` A2A boundary and closed WP-14/#2836 | Completed by `v0.91.1` | Complete baseline. |
| 79 | Runtime inhabitant proof | Implemented baseline | `v0.91.1` runtime inhabitant proof and closed WP-15/#2837 | Completed by `v0.91.1` | Complete baseline. |
| 80 | UTS + ACC multi-model benchmark and provider-native tool-call comparison | Implemented baseline | `v0.91.2` benchmark feature and evidence package | ADL baseline `v0.91.2`; UTS repo migration continues | ADL complete; external migration tracked separately. |
| 81 | Runtime/test-cycle recovery and coverage ergonomics | Implemented baseline | `v0.91.2` recovery docs/reports and CI budget docs | Baseline `v0.91.2`; follow-on as needed | Complete baseline. |
| 82 | CodeFriend repo-review product layer | Implemented baseline | `v0.91.2` CodeFriend productization plus review-product baseline | Productization `v0.91.2`; MVP `v0.95` | Productization complete; MVP polish scheduled. |
| 83 | Review heuristics and reviewer demo lane | Implemented baseline | `v0.91.2` feature doc | Baseline `v0.91.2` | Complete baseline. |
| 84 | Google Workspace CMS bridge and Rust-native adapter boundary | Implemented baseline plus active hardening | `v0.91.2` GWS CMS bridge | Bounded baseline `v0.91.2`; operational hardening ongoing | Complete baseline; ongoing hardening. |
| 85 | Automated repository modernization and external refactoring integration | Implemented bounded packet | `v0.91.2` code modernization demo | Bounded packet `v0.91.2` | Complete bounded packet. |
| 86 | Generic speculative decoding runtime acceleration | Implemented evaluation packet | TBD docs plus `v0.91.2` prototype feature | Evaluation `v0.91.2`; decision gate before `v0.95`; productization only if backend is viable | Productization is not yet scheduled; add a pre-`v0.95` decision gate so it is either scheduled, explicitly deferred, or removed from MVP scope. |
| 87 | Repo visibility follow-on | Implemented baseline | `v0.91.2` follow-on plus `v0.90` baseline | Follow-on `v0.91.2` | Complete baseline. |
| 88 | Publication packet program and GHB paper lane | Implemented packet | `v0.91.2` publication program and GHB lineage docs | Packet `v0.91.2`; no publication approval | Complete packet. |
| 89 | General-intelligence paper packet | Implemented packet | `v0.91.2` paper packet | Packet `v0.91.2` | Complete packet. |
| 90 | Rustdoc/doc cleanup | Implemented baseline | `v0.91.2` cleanup feature | Baseline `v0.91.2` | Complete baseline. |
| 91 | Workflow guardrails | Implemented baseline | `v0.91.2` guardrail feature | Baseline `v0.91.2` | Complete baseline. |
| 92 | Cognitive SDLC first slice and transition manifest | Implemented baseline | `v0.91.3` C-SDLC feature packet | First slice `v0.91.3`; hardening `v0.91.4` | First slice complete; hardening active. |
| 93 | Cognitive SDLC default operation and five-minute-sprint repeatability | Active milestone | `v0.91.4` C-SDLC feature docs and prompt-template transition plan | Target `v0.91.4`; release in progress | Active closeout; not complete until release tail finishes. |
| 94 | ACP / cognitive profiles runtime surface | Planned | `v0.92` feature doc | Target `v0.92`; calendar TBD | Scheduled. |
| 95 | ACIP binary schema and WebSocket carrier | Planned | `v0.92` feature doc | Schema/carrier `v0.92`; security `v0.93`; signed trace `v0.94` | Scheduled in staged tranches. |
| 96 | Identity, stable name, and continuity substrate | Planned | `v0.92` feature doc | Target `v0.92`; calendar TBD | Scheduled. |
| 97 | Memory grounding, capability envelope, and birth witnesses/receipt | Planned | `v0.92` feature doc | Target `v0.92`; calendar TBD | Scheduled. |
| 98 | First true Gödel-agent birthday | Planned | `v0.92` feature doc | Target `v0.92`; calendar TBD | Scheduled. |
| 99 | Constitutional citizenship, rights/duties, and governance review | Planned | `v0.93` feature doc and allocation plan | Target `v0.93`; calendar TBD | Scheduled. |
| 100 | Bounded Theory of Mind, relationship, reputation, and shared social memory boundary | Planned | `v0.93` ToM/social memory feature docs | Target `v0.93`; calendar TBD | Scheduled. |
| 101 | Delegation, upstream delegation, IAM, standing transition, and challenge/appeal governance | Planned | `v0.93` delegation/IAM feature doc plus upstream delegation note | Target `v0.93`; calendar TBD | Scheduled. |
| 102 | Enterprise security for the ADL polis | Planned | `v0.93` enterprise security docs and work breakdown | Target `v0.93`; calendar TBD | Scheduled. |
| 103 | Secure execution, policy, identity/auth, isolation, and provider-trust convergence | Planned | `v0.94` secure execution/trust feature doc | Target `v0.94`; calendar TBD | Scheduled. |
| 104 | Mental time travel / temporal self-projection | Planned | `v0.94` MTT feature doc plus source note | Target `v0.94`; calendar TBD | Scheduled. |
| 105 | Payments, settlement, economic agency, and `x402` / Lightning adapters | Planned | `v0.94.1` payments/settlement feature doc | Target `v0.94.1`; calendar TBD | Scheduled. |
| 106 | Bounded contract-market and resource-stewardship bridge | Implemented baseline | `v0.90.4` contract-market docs, proof coverage, demo matrix | Completed by `v0.90.4` | Complete bounded baseline. |
| 107 | Distributed execution integration | Partially implemented | `v0.95` feature doc, ADR 0003, cluster-execution groundwork | AEE/multi-agent boundary proof target `v0.91.5`/`v0.92`; broad integration `v0.95`; calendar TBD | Pull forward bounded boundary proof if required by multi-agent or AEE; broad integration remains scheduled for `v0.95`. |
| 108 | Demo catalog and polished MVP walkthrough | Partially implemented | `v0.95` feature doc plus demo matrices/reviewer packages | Target `v0.95`; calendar TBD | Scheduled. |
| 109 | Control-plane Rust migration / tooling hardening | Partially implemented | `v0.95` feature doc and Python-elimination/tooling plans | Target `v0.95`; calendar TBD | Scheduled. |
| 110 | Zed integration | MVP-scoped decision | `v0.95` feature doc plus v0.85 editor-planning notes | Decide adapter or removal by `v0.95`; calendar TBD | Scheduled decision boundary. |

## Milestone Completion Summary

| Milestone target | Feature rows | Audit summary |
| --- | --- | --- |
| Completed by `v0.8` or earlier | 1-4, 24 | Baseline deterministic execution and bounded AEE-era primitives are complete. |
| Completed by `v0.86`-`v0.91.2` | 5, 11, 16, 18-21, 23, 26, 28-36, 39, 47-54, 56-65, 69-91, 106 | Many rows are baseline-complete but several retain later integration targets. |
| Active `v0.91.4` | 93 | C-SDLC default-operation closeout is active, not complete until Sprint 4/release tail closes. |
| Bridge `v0.91.5` | new pull-forward targets: AEE closure definition, multi-agent stabilization, provider/model matrix, public prompt records, and v0.92 activation readiness | v0.91.5 should now be treated as a real bridge-completion milestone, not just preparation. |
| Planned `v0.92` | 7, 9 subset, 12 subset, 18-21, 32, 39, 94-98, 107 subset | Identity/birthday/ACP/ACIP, memory grounding, AEE implementation/proof, and bounded distributed/control-path subsets are scheduled. |
| Planned `v0.93` | 6, 8, 16, 25, 31, 33, 36, 40-44, 95, 99-102 | Governance, delegation, security, and enterprise trust are scheduled. |
| Planned `v0.94` | 22, 26, 67-68, 95, 103-104 | Secure execution, signed/queryable trace, reasoning/provenance, and MTT are scheduled. |
| Planned `v0.94.1` | 105 | Payments and economic rails are intentionally separated. |
| Planned `v0.95` | 9-10, 12-15, 17, 23, 27, 34, 37-38, 45-46, 49, 51-55, 61-63, 65-66, 86 decision/productization boundary, 107-110 | MVP convergence remains broad, but AEE subsystem closure and its prerequisite subsets should no longer wait until `v0.95`. |

## Gaps And Follow-On Recommendations

| Severity | Gap | Recommendation |
| --- | --- | --- |
| P1 | AEE completion was too implicit and too late if it is expected to be a finished subsystem before MVP convergence. | Pull forward immediately: use `#3534` to define the tranche in `v0.91.5`, then schedule implementation/proof for `v0.92` unless the tranche proves a smaller `v0.91.5` closure is safe. |
| P2 | Calendar dates are not represented in the feature list or forward milestone docs. | Keep milestone targets as authoritative for now; add exact dates only when release planning owns them. |
| P2 | Several baseline-complete rows also have future integration targets, which can make "done" feel ambiguous. | Split "baseline complete" from "integration complete" in future feature-list revisions, with explicit pulled-forward subset targets where needed. |
| P2 | `v0.91.5` is important bridge work but is not represented as a completion target in the feature list. | Treat `v0.91.5` as a completion target for multi-agent stabilization, public prompt records, provider/model matrix, v0.92 activation readiness, and AEE closure-definition work. |
| P2 | Generic speculative decoding productization remains open-ended. | Add a decision gate before `v0.95`: schedule productization if a backend is viable, otherwise explicitly defer or remove it from MVP scope. |
| P3 | The feature list is very broad and hard to review as one table. | Add a generated/maintained completion dashboard or split by milestone band after this audit is reviewed. |

## Proposed AEE Follow-On Issue Shape

Title:

```text
[v0.91.5][planning] Define AEE subsystem completion tranche and pull-forward plan
```

Goal:

Define the exact system-level done criteria for AEE and decide whether
implementation/proof lands in `v0.91.5`, `v0.92`, or another pre-`v0.95`
milestone.

Acceptance:

- AEE closure criteria are explicit and testable.
- The tranche separates baseline retry/convergence from full subsystem closure.
- The work identifies dependencies on pause/resume, queue/wake/handoff,
  distributed execution, control-plane truth, trace/replay, and demo proof.
- The roadmap no longer implies AEE is "done when MVP feels coherent."

## Non-Claims

- This audit does not implement AEE.
- This audit does not approve a new milestone scope by itself.
- This audit does not assign exact calendar release dates.
- This audit does not claim that all future milestone plans are complete.

## Validation

Validation completed for this audit:

- Count feature rows in `docs/planning/ADL_FEATURE_LIST.md`.
- Confirm this audit represents all 110 rows.
- Confirm AEE source docs exist.
- Confirm pulled-forward rows are explicitly represented in this audit.
- Run markdown/path checks for touched docs.

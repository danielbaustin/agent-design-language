# Demo Matrix - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Date: `2026-04-14`
- Owner: `Daniel Austin`
- Related issues / work packages: `WP-02` - `WP-15`

## Purpose

Define the canonical `v0.89.1` demo and proof program.

This matrix names the milestone claims, landed proof surfaces, remaining integration rows, and expected reviewer signals now that the issue wave is active. It is designed to make release-tail implementation and review mechanical rather than forcing the proof story to be rediscovered during execution.

## Scope

In scope for `v0.89.1`:
- adversarial runtime behavior
- exploit artifacts and replay evidence
- continuous verification and self-attack patterns
- flagship adversarial demo and governed execution substrate
- bounded manuscript/publication workflow for the initial three-paper arXiv program

Out of scope for `v0.89.1`:
- reopening the settled `v0.89` core package
- later identity, moral-governance, or constitutional bands
- pretending under-authored local notes already count as proof

## Demo Taxonomy

Use these categories consistently during `v0.89.1`:

- Ordinary demos:
  bounded runnable proof rows intended for milestone demo sweeps.
- Heavyweight proof packages:
  reviewer, quality-gate, or release-tail surfaces that may be canonical proof but should not be treated like quick demos.

For `v0.89.1`, rows `D1` through `D6` plus `D8` are expected to behave like ordinary or bounded-integration proof rows.
Rows `D7`, `D9`, and `D10` are heavier reviewer-facing packages and may remain artifact or document driven even when complete.

## Runtime Preconditions

Working directory:

```bash
cd adl
```

Execution assumptions:

```bash
Prefer bounded demo targets, replay fixtures, and local/provider shims where possible.
Do not require private credentials for the main v0.89.1 proof rows unless a row is explicitly marked live-only.
```

Additional environment / fixture requirements:
- use safe bounded demo targets rather than uncontrolled live infrastructure
- keep posture, target scope, and mitigation authority explicit in every proof row
- prefer replayable or reviewer-legible packets over unconstrained live attack theatrics

## Related Docs
- Vision: `VISION_v0.89.1.md`
- Design contract: `DESIGN_v0.89.1.md`
- WBS / milestone mapping: `WBS_v0.89.1.md`
- Sprint / execution plan: `SPRINT_v0.89.1.md`
- Release / checklist context: `MILESTONE_CHECKLIST_v0.89.1.md`
- Feature index: `FEATURE_DOCS_v0.89.1.md`
- Quality gate: `QUALITY_GATE_v0.89.1.md`
- Docs review: `DOCS_REVIEW_v0.89.1.md`

## Demo Coverage Summary

| Demo ID | Demo title | Milestone claim / WP proved | Entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
|---|---|---|---|---|---|---|---|
| D1 | Adversarial runtime walkthrough | `WP-02` - `WP-04` contested runtime, role architecture, and execution runner | `adl identity adversarial-runtime --out .adl/state/adversarial_runtime_model_v1.json`, `adl identity red-blue-architecture --out .adl/state/red_blue_agent_architecture_v1.json`, and `adl identity adversarial-runner --out .adl/state/adversarial_execution_runner_v1.json` | adversarial runtime, red/blue architecture, and runner contract packets under `.adl/state/` | reviewer can see posture, target, roles, and bounded execution stages end to end | contract packets are deterministic; runner execution stays bounded by the declared contract | LANDED |
| D2 | Exploit artifact and replay proof | `WP-05` exploit artifact family and replay manifest | `adl identity exploit-replay --out .adl/state/exploit_artifact_replay_v1.json` | `.adl/state/exploit_artifact_replay_v1.json` | reviewer can inspect exploit hypothesis, evidence, replay mode, and expected outcome without narrative reconstruction | replay contract declares deterministic, bounded-variance, or best-effort mode explicitly | LANDED |
| D3 | Continuous verification loop | `WP-06` continuous verification and exploit generation | `adl identity continuous-verification --out .adl/state/continuous_verification_self_attack_v1.json` | continuous verification contract artifact with lifecycle, cadence, replay, mitigation, and promotion rules | reviewer can see repeated falsification pressure as a governed execution pattern rather than ad hoc red-teaming | repeated bounded inputs should preserve lifecycle shape and proof packet structure | LANDED |
| D4 | Self-attack scenario packet | `WP-06` self-attacking systems as architecture rather than rhetoric | `adl identity continuous-verification --out .adl/state/continuous_verification_self_attack_v1.json` | self-attack contract artifact with bounded layers, target/posture policy, and evidence/replay rules | reviewer can see the system's self-attack pattern before externalization and inspect the required evidence chain | scenario should remain posture-bounded and replay-legible | LANDED |
| D5 | Flagship adversarial demo | `WP-07` full exploit -> replay -> mitigation -> promotion loop | `adl demo demo-h-v0891-adversarial-self-attack --run --trace --out .adl/reports/adversarial-demo --no-open` | `.adl/reports/adversarial-demo/demo-h-v0891-adversarial-self-attack/review_packet.json` | reviewer can answer what was attacked, how it was reproduced, what mitigation was applied, and whether replay post-fix succeeded | deterministic local replay compares the same request before and after mitigation | LANDED |
| D6 | Operational skills substrate integration | `WP-08` - `WP-09` operational skills, composition, and bounded governance follow-through | `adl identity operational-skills --out .adl/state/operational_skills_substrate_v1.json`, `adl identity skill-composition --out .adl/state/skill_composition_model_v1.json`, and `adl identity delegation-refusal-coordination --out .adl/state/delegation_refusal_coordination_v1.json` | substrate/composition/governance contract packets | reviewer can see that adversarial work runs through explicit skill/composition surfaces with bounded delegation, refusal, approval-gate, and coordination outcomes | orchestration structure and governance outcome taxonomy should be deterministic even if node outputs remain stochastic | LANDED |
| D7 | Reviewer-facing security proof package | `WP-10` - `WP-13` packaging convergence, milestone convergence, and integration demos | `adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json`, `adl identity demo-proof-entry-points --out .adl/state/demo_proof_entry_points_v1.json`, and `bash adl/tools/demo_v0891_wp13_demo_integration.sh` | provider-extension packaging packet + demo proof entry-point packet + WP-13 integration manifest | reviewer can inspect milestone claims, carry-forward boundaries, D8, and D9 as one coherent package | identity packets and the WP-13 integration packet are deterministic; heavyweight child demos remain replayable through their own test commands | LANDED |
| D8 | Five-Agent Hey Jude MIDI demo | `WP-08` - `WP-10`, `WP-13` cross-provider coordination, human-in-the-loop orchestration, and integration delight surface | `bash adl/tools/demo_v0891_five_agent_hey_jude.sh` | `artifacts/v0891/five_agent_hey_jude/performance_manifest.json` plus MIDI and participation artifacts | reviewer can see one human plus four providers coordinating on one ADL runtime with explicit orchestration boundaries | fixture-backed MVAVE Chocolate events preserve cue order and artifact shape | LANDED |
| D9 | ArXiv manuscript workflow packet | `WP-08`, `WP-13` bounded `arxiv-paper-writer` skill plus the initial three-paper publication program | `bash adl/tools/demo_v0891_arxiv_manuscript_workflow.sh` | `artifacts/v0891/arxiv_manuscript_workflow/demo_manifest.json` plus `manuscript_status/three_paper_status.json` | reviewer can see the bounded manuscript workflow packet for What Is ADL?, Gödel Agents and ADL, and Cognitive Spacetime Manifold without losing claim discipline or hiding the WP-08/WP-13 boundary | packet generation is deterministic; bounded source packets preserve role order, section structure, packet shape, and no-submission boundary | LANDED |
| D10 | Quality gate walkthrough | `WP-14` coverage/quality gate, coverage posture, proof-package aggregation, and maintainability watch posture | `bash adl/tools/demo_v0891_quality_gate.sh` | `artifacts/v0891/quality_gate/quality_gate_record.json` plus per-check logs | reviewer can see the local quality suite, coverage policy, D7/D8/D9 proof-package checks, and large-module watch output in one place | command uses deterministic local checks and fixture-backed proof demos; it does not replace CI or the CI-only PR closing-linkage guardrail | LANDED |

Status guidance:
- `PLANNED` = intended but not yet validated
- `READY` = runnable and locally validated
- `PARTIAL` = one bounded proof surface has landed while broader downstream package work remains
- `BLOCKED` = known dependency or missing proof surface
- `LANDED` = milestone evidence exists and is ready for review

Current planning truth:
- the `v0.89.1` issue wave is open
- rows `D1` through `D6` are landed from `WP-02` - `WP-09`
- row `D7` is landed through the `WP-10` / `WP-11` proof packets, `WP-12` convergence, and the `WP-13` integration packet
- row `D8` is landed as a bounded five-agent Hey Jude MIDI integration demo
- row `D9` is landed as a bounded manuscript workflow packet with final arXiv submission still out of scope
- row `D10` is the `WP-14` quality-gate aggregation surface; it is a heavyweight proof package, not a quick demo sweep
- `WP-15` adds the docs-review convergence record that reviewers should read alongside this matrix
- this matrix is a convergence surface for review-tail execution, not permission to claim later demo work before it exists

Heavyweight proof-package rule:
- if a proof surface mainly exists to bundle review, release, or quality-gate evidence, classify it as a heavyweight proof package even if it is script-driven
- do not silently fold heavyweight proof packages into ordinary demo sweeps without saying so explicitly

## Coverage Rules
- every major milestone claim should map to a runnable demo or explicit alternate proof surface
- every row should name one primary proof surface that a reviewer can inspect directly
- entry points should become copy/paste-ready once the official issue wave lands
- success signals should describe what to inspect, not only process exit codes

## Demo Details

### D10) Quality gate walkthrough

Description:
- aggregate the `v0.89.1` local quality command suite into one reviewer-facing packet
- prove that coverage policy, proof-package checks, and maintainability-watch output are visible together

Milestone claims / work packages covered:
- `WP-14`

Entry point:

```bash
bash adl/tools/demo_v0891_quality_gate.sh
```

Expected artifacts:
- `artifacts/v0891/quality_gate/quality_gate_record.json`
- per-check logs under `artifacts/v0891/quality_gate/`
- `artifacts/v0891/quality_gate/README.md`

Primary proof surface:
- `quality_gate_record.json`

Expected success signals:
- reviewer can see the local quality suite and proof-package checks in one manifest
- coverage posture is explicit and has no active per-file exclusion regex
- large-module reporting remains visible but non-blocking

Validation:

```bash
bash adl/tools/test_demo_v0891_quality_gate.sh
```

Known limits / caveats:
- this row does not replace CI
- the PR closing-linkage guardrail remains CI-only because it depends on pull-request event context
- later `WP-16` through `WP-20` still own review, remediation, planning, and release ceremony

---

### D1) Adversarial runtime walkthrough

Description:
- demonstrate continuous adversarial pressure as a first-class runtime condition
- make red / blue / purple roles, posture, and stage order explicit

Milestone claims / work packages covered:
- `WP-02`
- `WP-03`
- `WP-04`

Entry point:

```bash
adl identity adversarial-runtime --out .adl/state/adversarial_runtime_model_v1.json
adl identity red-blue-architecture --out .adl/state/red_blue_agent_architecture_v1.json
adl identity adversarial-runner --out .adl/state/adversarial_execution_runner_v1.json
```

Expected artifacts:
- adversarial runtime contract packet
- red / blue agent architecture contract packet
- bounded adversarial execution runner contract packet
- posture declaration and role-attribution evidence in the contract surfaces

Primary proof surface:
- `.adl/state/adversarial_runtime_model_v1.json`
- `.adl/state/red_blue_agent_architecture_v1.json`
- `.adl/state/adversarial_execution_runner_v1.json`

Expected success signals:
- reviewer can see the declared target, posture, goal, and bounded limit
- reviewer can identify red, blue, and purple role contributions in the resulting trace
- the execution loop is legible as architecture, not narrative

Known limits / caveats:
- this row is about runtime architecture and runner contract scaffolding; full mitigation and regression promotion remain the flagship D5 demo path

---

### D2) Exploit artifact and replay proof

Description:
- demonstrate that exploit knowledge becomes structured artifacts rather than prose
- show that replay mode, expected outcome, and preconditions are explicit

Milestone claims / work packages covered:
- `WP-05`

Entry point:

```bash
adl identity exploit-replay --out .adl/state/exploit_artifact_replay_v1.json
```

Expected artifacts:
- exploit hypothesis artifact
- exploit evidence artifact
- exploit classification artifact
- adversarial replay manifest

Primary proof surface:
- `.adl/state/exploit_artifact_replay_v1.json`

Expected success signals:
- reviewer can inspect exploit family, preconditions, unsafe outcome, replay mode, and success criteria directly
- mitigation and regression follow-on are linkable rather than implied

Known limits / caveats:
- this row proves artifact and replay contract quality, not full end-to-end adversarial demo closure

---

### D3) Continuous verification loop

Description:
- show continuous verification and exploit generation as a governed execution pattern

Milestone claims / work packages covered:
- `WP-06`

Planned entry point:

```bash
adl identity continuous-verification --out .adl/state/continuous_verification_self_attack_v1.json
```

Expected artifacts:
- continuous verification contract artifact
- lifecycle and cadence rules
- replay, mitigation, and promotion linkage rules

Primary proof surface:
- `.adl/state/continuous_verification_self_attack_v1.json`

Expected success signals:
- reviewer can see surface selection, exploit hypothesis generation, bounded attempt, evidence capture, and promotion linkage
- continuous verification reads like a real lifecycle, not a slogan

Known limits / caveats:
- this row should stay governed and posture-bounded rather than drifting into unconstrained offensive capability
- this row proves the contract/proof surface; the flagship executable demo remains `WP-07`

---

### D4) Self-attack scenario packet

Description:
- show the self-attacking principle as a bounded scenario with explicit evidence

Milestone claims / work packages covered:
- `WP-06`

Planned entry point:

```bash
adl identity continuous-verification --out .adl/state/continuous_verification_self_attack_v1.json
```

Expected artifacts:
- self-attack contract artifact
- bounded self-attack layer rules
- posture, target-scope, evidence, replay, and learning-promotion requirements

Primary proof surface:
- `.adl/state/continuous_verification_self_attack_v1.json`

Expected success signals:
- reviewer can see the system attack itself before others do
- the result is bounded, attributable, and evidence-bearing

Known limits / caveats:
- this row should not be mistaken for open-ended autonomous offense; it is a governed verification pattern
- this row proves the bounded self-attack contract; concrete scenario execution remains `WP-07`

---

### D5) Flagship adversarial demo

Description:
- prove the full exploit -> replay -> mitigation -> regression-promotion loop

Milestone claims / work packages covered:
- `WP-07`

Planned entry point:

```bash
adl demo demo-h-v0891-adversarial-self-attack --run --trace --out .adl/reports/adversarial-demo --no-open
```

Expected artifacts:
- `target/target.json`
- `target/security_posture.json`
- `hypothesis.json`
- `evidence.json`
- `classification.json`
- `replay_manifest.json`
- `replay_pre_fix/result.json`
- `mitigation.json`
- `replay_post_fix/result.json`
- `promotion.json`
- `review_packet.json`
- `trace.jsonl`

Primary proof surface:
- `.adl/reports/adversarial-demo/demo-h-v0891-adversarial-self-attack/review_packet.json`

Expected success signals:
- reviewer can answer what was found, how it was exploited, how replay worked, what changed after mitigation, and what was promoted
- the packet closes the loop without hidden reasoning steps
- pre-mitigation replay reaches the unsafe state and post-mitigation replay denies the same request

Known limits / caveats:
- this row is the flagship proof row and stays bounded to a safe, local, deterministic target

### D6) Operational skills substrate integration

Description:
- show that the adversarial band runs through explicit skills and compositions rather than informal orchestration

Milestone claims / work packages covered:
- `WP-08`
- `WP-09`

Entry point:

```bash
adl identity operational-skills --out .adl/state/operational_skills_substrate_v1.json
adl identity skill-composition --out .adl/state/skill_composition_model_v1.json
adl identity delegation-refusal-coordination --out .adl/state/delegation_refusal_coordination_v1.json
```

Expected artifacts:
- operational skills substrate packet
- skill composition packet
- delegation / refusal / coordination governance packet
- bounded policy-outcome mapping for allowed delegation, governed refusal, and approval gates
- coordination and bounded-dissent review rules

Primary proof surface:
- substrate/composition/governance contract packets under `.adl/state/`

Expected success signals:
- reviewer can see explicit invocation boundaries, composition primitives, and governed coordination outcomes
- the adversarial milestone is anchored in runtime execution substrate rather than ad hoc scripts
- refusal remains distinguishable from generic failure, approval gates stop before authority, and delegation preserves constraints

Known limits / caveats:
- the draft delegation/refusal and negotiation notes remain supporting inputs rather than promoted tracked feature docs
- this contract does not implement final constitutional negotiation, social reputation, or provider-security extension

---

### D7) Reviewer-facing security proof package

Description:
- bundle the adversarial/runtime proof story into a reviewer-facing milestone packet

Milestone claims / work packages covered:
- `WP-10`
- `WP-11`
- `WP-12`
- `WP-13`

Entry point:

```bash
adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json
adl identity demo-proof-entry-points --out .adl/state/demo_proof_entry_points_v1.json
bash adl/tools/demo_v0891_wp13_demo_integration.sh
```

Expected artifacts:
- provider extension packaging contract packet
- demo proof entry-point packet
- WP-13 demo integration manifest
- reviewer-facing adversarial/replay/trust packet index
- milestone convergence and carry-forward note

Primary proof surface:
- `.adl/state/provider_extension_packaging_v1.json`
- `.adl/state/demo_proof_entry_points_v1.json`
- `artifacts/v0891/wp13_demo_integration/integration_manifest.json`

Expected success signals:
- reviewer can inspect the milestone as one coherent adversarial/runtime package
- boundary with `v0.89` remains explicit and defensible
- provider-security extension work is visible as deferred rather than silently promoted
- D8 and D9 have runnable/testable proof entries rather than planned placeholders

Known limits / caveats:
- this is a heavyweight proof package and should not be confused with a quick demo row
- `WP-12` convergence has landed; `WP-13` consumes that convergence rather than closing or replacing WP-12
- `WP-10` does not implement provider attestation, trust scoring, network posture enforcement, secret lifecycle enforcement, provider sandboxing, or external provider-security demos

---

### D8) Five-Agent Hey Jude MIDI demo

Description:
- prove that the adversarial/runtime band can also support a high-delight human-in-the-loop coordination surface
- show one human plus four providers participating on one ADL runtime through a MIDI control surface

Milestone claims / work packages covered:
- `WP-08`
- `WP-09`
- `WP-10`
- `WP-13`

Entry point:

```bash
bash adl/tools/demo_v0891_five_agent_hey_jude.sh
```

Expected artifacts:
- `artifacts/v0891/five_agent_hey_jude/performance_manifest.json`
- `artifacts/v0891/five_agent_hey_jude/cast.json`
- `artifacts/v0891/five_agent_hey_jude/section_plan.json`
- `artifacts/v0891/five_agent_hey_jude/cue_timeline.json`
- `artifacts/v0891/five_agent_hey_jude/transcript.md`
- `artifacts/v0891/five_agent_hey_jude/midi_event_log.json`
- `artifacts/v0891/five_agent_hey_jude/provider_participation_summary.json`
- `artifacts/v0891/five_agent_hey_jude/runtime/runs/v0-89-1-five-agent-hey-jude-midi-demo/run_summary.json`

Primary proof surface:
- five-agent Hey Jude demo packet under `artifacts/v0891/five_agent_hey_jude/`

Expected success signals:
- reviewer can see human-in-the-loop orchestration rather than passive provider fan-out
- cross-provider participation is explicit and bounded
- the demo is charming without becoming structurally vague

Validation:

```bash
bash adl/tools/test_demo_v0891_five_agent_hey_jude.sh
```

Source planning inputs:
- `docs/planning/NEXT_MILESTONE_DEMO_CANDIDATES.md`
- bounded demo doc: `demos/v0.89.1/five_agent_hey_jude_midi_demo.md`

Known limits / caveats:
- this row is a flagship delight/integration surface, not the core exploit/replay proof row
- it should remain bounded and reviewer-legible rather than turning into an open-ended performance artifact
- it uses section cues rather than storing a full lyric sheet in tracked repo artifacts

---

### D9) ArXiv manuscript workflow packet

Description:
- prove that `v0.89.1` can use a bounded manuscript workflow to support serious technical writing about ADL itself
- make the writer skill, source packet, role decomposition, and three-paper outputs reviewer-legible

Milestone claims / work packages covered:
- `WP-08`
- `WP-13`

Entry point:

```bash
bash adl/tools/demo_v0891_arxiv_manuscript_workflow.sh
```

Expected artifacts:
- `artifacts/v0891/arxiv_manuscript_workflow/demo_manifest.json`
- `artifacts/v0891/arxiv_manuscript_workflow/writer_skill_packet/writer_skill_status.json`
- `artifacts/v0891/arxiv_manuscript_workflow/writer_skill_packet/workflow_contract.md`
- `artifacts/v0891/arxiv_manuscript_workflow/source_packets/source_packet_manifest.json`
- `artifacts/v0891/arxiv_manuscript_workflow/manuscript_status/three_paper_status.json`
- `artifacts/v0891/arxiv_manuscript_workflow/review/review_gates.json`
- `artifacts/v0891/arxiv_manuscript_workflow/review/claim_boundaries.md`
- source and status packets for:
  - What Is ADL?
  - Gödel Agents and ADL
  - Cognitive Spacetime Manifold

Primary proof surface:
- reviewer-facing manuscript workflow packet plus three-paper status bundle under `artifacts/v0891/arxiv_manuscript_workflow/`

Expected success signals:
- reviewer can see the role mapping, source packet, section structure, and review packet shape directly
- the paper workflow preserves claim discipline and distinguishes repo truth from future direction
- the milestone can show real manuscript progress without pretending autonomous publication

Validation:

```bash
bash adl/tools/test_demo_v0891_arxiv_manuscript_workflow.sh
```

Known limits / caveats:
- this row is about bounded drafting and review workflow, not automatic submission or unverifiable authorship claims
- WP-08 defines the writer skill and composition boundary; WP-13 lands the three-paper manuscript status packet while leaving final arXiv submission out of scope
- the WP-08 proof hooks are `adl identity operational-skills --out .adl/state/operational_skills_substrate_v1.json` and `adl identity skill-composition --out .adl/state/skill_composition_model_v1.json`

# Demo Matrix - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Date: `2026-04-14`
- Owner: `Daniel Austin`
- Related issues / work packages: `WP-02` - `WP-13`

## Purpose

Define the canonical `v0.89.1` demo and proof program.

This matrix names the milestone claims, the intended proof surfaces, and the expected reviewer signals before the issue wave opens. It is designed to make later implementation and review mechanical rather than forcing the proof story to be rediscovered during execution.

## Scope

In scope for `v0.89.1`:
- adversarial runtime behavior
- exploit artifacts and replay evidence
- continuous verification and self-attack patterns
- flagship adversarial demo and governed execution substrate

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
Row `D7` is a heavier reviewer-facing package and may remain artifact or document driven even when complete.

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

## Demo Coverage Summary

| Demo ID | Demo title | Milestone claim / WP proved | Planned entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
|---|---|---|---|---|---|---|---|
| D1 | Adversarial runtime walkthrough | `WP-02` - `WP-04` contested runtime, role architecture, and execution runner | planned `WP-02` / `WP-04` adversarial runner entry point | adversarial runtime review packet + bounded trace bundle | reviewer can see posture, target, roles, and bounded execution stages end to end | same bounded target and posture should preserve stage order and role attribution | PLANNED |
| D2 | Exploit artifact and replay proof | `WP-05` exploit artifact family and replay manifest | planned `WP-05` schema/replay validation surface | exploit artifact family + replay manifest packet | reviewer can inspect exploit hypothesis, evidence, replay mode, and expected outcome without narrative reconstruction | replay contract should declare deterministic, bounded-variance, or best-effort mode explicitly | PLANNED |
| D3 | Continuous verification loop | `WP-06` continuous verification and exploit generation | planned `WP-06` bounded verification runner | verification trace + exploit-attempt packet + classification/promotion artifacts | reviewer can see repeated falsification pressure as a governed execution pattern rather than ad hoc red-teaming | repeated bounded inputs should preserve lifecycle shape and proof packet structure | PLANNED |
| D4 | Self-attack scenario packet | `WP-06` self-attacking systems as architecture rather than rhetoric | planned `WP-06` self-attack scenario entry point | self-attack review packet with bounded trace/evidence | reviewer can see the system attack itself before externalization and capture the result as explicit evidence | scenario should remain posture-bounded and replay-legible | PLANNED |
| D5 | Flagship adversarial demo | `WP-07` full exploit -> replay -> mitigation -> promotion loop | planned `WP-07` adversarial demo entry point | flagship adversarial demo packet | reviewer can answer what was attacked, how it was reproduced, what mitigation was applied, and whether replay post-fix succeeded | replay should make before/after mitigation comparison explicit | PLANNED |
| D6 | Operational skills substrate integration | `WP-08` - `WP-09` operational skills, composition, and bounded governance follow-through | planned `WP-08` / `WP-09` governed composition entry point | substrate/composition packet + delegation/refusal integration note | reviewer can see that adversarial work runs through explicit skill/composition surfaces instead of ad hoc orchestration | orchestration structure should be deterministic even if node outputs remain stochastic | PLANNED |
| D7 | Reviewer-facing security proof package | `WP-10` - `WP-13` packaging convergence, milestone convergence, and integration demos | planned `WP-10` / `WP-13` review package | reviewer-facing adversarial/replay/trust packet | reviewer can inspect milestone claims, carry-forward boundaries, and proof surfaces as one coherent package | may remain artifact/document driven rather than fully runnable | PLANNED |
| D8 | Five-Agent Hey Jude MIDI demo | `WP-08` - `WP-10`, `WP-13` cross-provider coordination, human-in-the-loop orchestration, and integration delight surface | planned `WP-08` / `WP-13` coordination demo entry point | Hey Jude coordination packet + MIDI control trace + provider participation summary | reviewer can see one human plus four providers coordinating on one ADL runtime with explicit orchestration boundaries | bounded score/input should preserve composition shape, participant roles, and MIDI event ordering where declared | PLANNED |

Status guidance:
- `PLANNED` = intended but not yet validated
- `READY` = runnable and locally validated
- `BLOCKED` = known dependency or missing proof surface
- `LANDED` = milestone evidence exists and is ready for review

Current planning truth:
- all rows remain planned because the `v0.89.1` issue wave has not opened yet
- this matrix is review-ready and intended to make later issue execution faster, not to claim implementation already exists

Heavyweight proof-package rule:
- if a proof surface mainly exists to bundle review, release, or quality-gate evidence, classify it as a heavyweight proof package even if it is script-driven
- do not silently fold heavyweight proof packages into ordinary demo sweeps without saying so explicitly

## Coverage Rules
- every major milestone claim should map to a runnable demo or explicit alternate proof surface
- every row should name one primary proof surface that a reviewer can inspect directly
- entry points should become copy/paste-ready once the official issue wave lands
- success signals should describe what to inspect, not only process exit codes

## Demo Details

### D1) Adversarial runtime walkthrough

Description:
- demonstrate continuous adversarial pressure as a first-class runtime condition
- make red / blue / purple roles, posture, and stage order explicit

Milestone claims / work packages covered:
- `WP-02`
- `WP-03`
- `WP-04`

Planned entry point:

```bash
Defined when the official `WP-02` / `WP-04` issues land.
```

Expected artifacts:
- adversarial runtime review packet
- bounded adversarial execution trace bundle
- posture declaration and role-attribution evidence

Primary proof surface:
- reviewer-facing runtime packet centered on the adversarial execution runner

Expected success signals:
- reviewer can see the declared target, posture, goal, and bounded limit
- reviewer can identify red, blue, and purple role contributions in the resulting trace
- the execution loop is legible as architecture, not narrative

Known limits / caveats:
- this row is about runtime architecture and trace, not yet full mitigation or regression promotion

---

### D2) Exploit artifact and replay proof

Description:
- demonstrate that exploit knowledge becomes structured artifacts rather than prose
- show that replay mode, expected outcome, and preconditions are explicit

Milestone claims / work packages covered:
- `WP-05`

Planned entry point:

```bash
Defined when the official `WP-05` schema/replay issue lands.
```

Expected artifacts:
- exploit hypothesis artifact
- exploit evidence artifact
- exploit classification artifact
- adversarial replay manifest

Primary proof surface:
- exploit artifact family plus replay manifest packet

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
Defined when the official `WP-06` verification issue lands.
```

Expected artifacts:
- bounded verification runner packet
- exploit-attempt evidence bundle
- classification / promotion linkage artifacts

Primary proof surface:
- bounded verification runner packet

Expected success signals:
- reviewer can see surface selection, exploit hypothesis generation, bounded attempt, evidence capture, and promotion linkage
- continuous verification reads like a real lifecycle, not a slogan

Known limits / caveats:
- this row should stay governed and posture-bounded rather than drifting into unconstrained offensive capability

---

### D4) Self-attack scenario packet

Description:
- show the self-attacking principle as a bounded scenario with explicit evidence

Milestone claims / work packages covered:
- `WP-06`

Planned entry point:

```bash
Defined when the official `WP-06` self-attack slice lands.
```

Expected artifacts:
- self-attack scenario packet
- bounded trace/evidence bundle
- posture and target-scope record

Primary proof surface:
- self-attack review packet

Expected success signals:
- reviewer can see the system attack itself before others do
- the result is bounded, attributable, and evidence-bearing

Known limits / caveats:
- this row should not be mistaken for open-ended autonomous offense; it is a governed verification pattern

---

### D5) Flagship adversarial demo

Description:
- prove the full exploit -> replay -> mitigation -> regression-promotion loop

Milestone claims / work packages covered:
- `WP-07`

Planned entry point:

```bash
Defined when the official `WP-07` adversarial demo issue lands.
```

Expected artifacts:
- exploit artifact packet
- replay manifest and replay results
- mitigation linkage artifact
- promotion / regression artifact

Primary proof surface:
- flagship adversarial demo packet

Expected success signals:
- reviewer can answer what was found, how it was exploited, how replay worked, what changed after mitigation, and what was promoted
- the packet closes the loop without hidden reasoning steps

Known limits / caveats:
- this row is the flagship proof row and should stay bounded to a safe, intelligible target

---

### D6) Operational skills substrate integration

Description:
- show that the adversarial band runs through explicit skills and compositions rather than informal orchestration

Milestone claims / work packages covered:
- `WP-08`
- `WP-09`

Planned entry point:

```bash
Defined when the official `WP-08` / `WP-09` issues land.
```

Expected artifacts:
- operational skills substrate packet
- skill composition packet
- delegation / refusal / coordination integration note

Primary proof surface:
- substrate/composition integration packet

Expected success signals:
- reviewer can see explicit invocation boundaries, composition primitives, and governed coordination surfaces
- the adversarial milestone is anchored in runtime execution substrate rather than ad hoc scripts

Known limits / caveats:
- delegation/refusal and negotiation remain supporting inputs unless and until they are integrated truthfully during execution

---

### D7) Reviewer-facing security proof package

Description:
- bundle the adversarial/runtime proof story into a reviewer-facing milestone packet

Milestone claims / work packages covered:
- `WP-10`
- `WP-11`
- `WP-12`
- `WP-13`

Planned entry point:

```bash
Defined when the official convergence/demo issues land.
```

Expected artifacts:
- reviewer-facing adversarial/replay/trust packet
- integration demo summary
- milestone convergence and carry-forward note

Primary proof surface:
- reviewer-facing security proof package

Expected success signals:
- reviewer can inspect the milestone as one coherent adversarial/runtime package
- boundary with `v0.89` remains explicit and defensible

Known limits / caveats:
- this is a heavyweight proof package and should not be confused with a quick demo row

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

Planned entry point:

```bash
Defined when the official coordination/integration demo issues land.
```

Expected artifacts:
- Hey Jude coordination packet
- MIDI control trace or event summary
- provider participation and orchestration packet
- reviewer-facing integration summary

Primary proof surface:
- five-agent Hey Jude demo packet

Expected success signals:
- reviewer can see human-in-the-loop orchestration rather than passive provider fan-out
- cross-provider participation is explicit and bounded
- the demo is charming without becoming structurally vague

Source planning inputs:
- `docs/planning/NEXT_MILESTONE_DEMO_CANDIDATES.md`
- `.adl/docs/TBD/FIVE_AGENT_HEY_JUDE_MIDI_DEMO_PLAN.md`
- `.adl/docs/TBD/FIVE_AGENT_HEY_JUDE_MIDI_DEMO_IMPLEMENTATION_PLAN.md`
- `.adl/docs/TBD/NEXT_MILESTONE_CANDIDATE_FIVE_AGENT_HEY_JUDE_MIDI_DEMO.md`

Known limits / caveats:
- this row is a flagship delight/integration surface, not the core exploit/replay proof row
- it should remain bounded and reviewer-legible rather than turning into an open-ended performance artifact

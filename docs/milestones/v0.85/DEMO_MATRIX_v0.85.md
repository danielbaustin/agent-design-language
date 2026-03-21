# ADL v0.85 Demo Matrix

**Status:** Draft  
**Milestone:** v0.85  
**Purpose:** Canonical demo and proof-surface matrix for milestone review, release readiness, and external legibility.

---

## 1. Purpose

This document maps the major claims of ADL `v0.85` to concrete proof surfaces.

For each significant feature band, it identifies either:
- a **runnable demo**, or
- an explicit **alternate proof surface** when a full runnable demo is not yet the right primary mechanism.

The goal is not to create a vague showcase list. The goal is to make milestone review and release readiness legible.

This matrix should help reviewers answer:
- What does `v0.85` actually demonstrate?
- Which demos are headline milestone demos versus supporting demos?
- What artifact or record proves a claim if no direct demo is run?
- What still remains partial or deferred?

---

## 2. How to Read This Matrix

### Demo Classes

- **Primary demo**: a headline runnable demo that should be shown in milestone review and external explanation.
- **Supporting demo**: a narrower runnable demo that proves a bounded surface but is not itself the headline narrative.
- **Alternate proof surface**: an artifact-driven proof surface used when a runnable demo is not the best primary mechanism.

### Demo Status

- **Ready**: can be used now for milestone review.
- **Partial**: useful, but still rough or dependent on adjacent cleanup.
- **Deferred**: intentionally not a `v0.85` runnable demo surface.

### Reviewer Rule

Every row in this matrix must provide at least one of:
- a runnable command or playbook, or
- an explicit artifact/proof surface that a reviewer can inspect.

---

## 3. Demo Coverage Summary

| Feature band | Class | Status | Primary surface | Proof artifact / review surface |
|---|---|---:|---|---|
| Structured authoring and task artifacts | Primary demo | Ready | Task-bundle / card workflow walkthrough | STP / SIP / SOR bundle and reviewer-visible output records |
| Dependable execution and verifiable inference | Primary demo | Ready | Output-record and proof-surface walkthrough | Strong SORs, validation evidence, artifact-first review surfaces |
| AEE bounded adaptation | Primary demo | Ready | `swarm/tools/demo_aee_bounded_adaptation.sh` | `aee_decision.json`, demo README, output card |
| Affect engine core | Supporting demo | Ready | `swarm/tools/demo_affect_engine.sh` | affect-state artifact, downstream decision change |
| Reasoning graph + affect integration | Supporting demo | Ready | `swarm/tools/demo_reasoning_graph_affect.sh` | reasoning-graph artifact, before/after comparison |
| Affect + Gödel vertical slice | Primary demo | Ready | `swarm/tools/demo_affect_godel_vertical_slice.sh` | vertical-slice artifact proving full causal chain |
| Hypothesis engine | Supporting demo | Ready | `swarm/tools/demo_godel_hypothesis_engine.sh` | structured hypothesis artifact |
| Policy learning | Supporting demo | Ready | `swarm/tools/demo_adaptive_godel_loop.sh` | policy delta artifact and before/after comparison |
| Experiment prioritization | Supporting demo | Ready | `swarm/tools/demo_experiment_prioritization.sh` | ranked artifact with tie-break behavior |
| Cross-workflow learning | Alternate proof surface | Ready | `swarm/tools/demo_cross_workflow_learning.sh` | linked artifact chain across workflows; demos/cross_workflow_learning_demo.md |
| Promotion / evaluation loop | Supporting demo | Ready | `swarm/tools/demo_promotion_eval_loop.sh` | evaluation artifact plus promotion decision artifact |
| Review and output-record rigor | Alternate proof surface | Ready | template + output-card review | issues #918, #941, #948 and strong example SORs |
| Demo/readiness discipline itself | Alternate proof surface | Ready | this document | canonical demo matrix |

---

## 4. Primary Demos

These are the milestone’s headline demonstrations. They should be the first things shown in a milestone review and the first things referenced externally.

### 4.1 Structured Authoring / Task-Bundle Workflow

**Class:** Primary demo  
**Status:** Ready  
**Why it matters:** ADL’s credibility begins with explicit artifacts and explicit lifecycle control. Without this, the rest of the system looks like ordinary prompt orchestration.

**Demo shape:**
- Walk through one real issue bundle showing:
  - STP / issue prompt
  - SIP / input card
  - SOR / output card
- Show how the artifacts differ by role.
- Show one reviewer-ready output record with concrete proof surfaces.

**Primary review surfaces:**
- `.adl/issues/v0.85/bodies/`
- `.adl/cards/`
- `.adl/v0.85/tasks/` where available
- reviewed output cards such as:
  - `#918`
  - `#941`
  - `#958`

**Reviewer should look for:**
- clear separation between intent, execution, and review
- bounded validation/proof surfaces
- truthful integration-state language

**Notes:**
This is partly a walkthrough rather than a single shell script, but it is still a core milestone demo because it explains the substrate.

---

### 4.2 Dependable Execution and Verifiable Inference

**Class:** Primary demo  
**Status:** Ready  
**Why it matters:** This is one of the clearest ADL differentiators. The system must show that it can produce execution records that are more than plausible prose.

**Demo shape:**
- Walk through one strong SOR and its proof surfaces.
- Show artifact-first validation.
- Show how output review is grounded in emitted artifacts, not just narrative claims.

**Primary review surfaces:**
- strong SORs from recent work, especially:
  - `#918`
  - `#941`
  - `#958`
- `docs/tooling/prompt-spec.md`
- `docs/tooling/structured-prompt-contracts.md`

**Reviewer should look for:**
- primary proof surface named explicitly
- validation commands tied to real artifacts
- clear distinction between worktree state and merged state
- no placeholder leakage

**Notes:**
This is best demonstrated through reviewed artifacts rather than a theatrical CLI run.

---

### 4.3 AEE Bounded Adaptation

**Class:** Primary demo  
**Status:** Ready  
**Command / playbook:**
- `swarm/tools/demo_aee_bounded_adaptation.sh`

**Primary supporting docs:**
- `demos/aee-recovery/README.md`

**Expected proof artifacts:**
- emitted `aee_decision.json` or equivalent run artifact
- corresponding SOR and validation evidence

**Why it matters:**
This is the first visible proof that ADL can adapt within bounds rather than behave as a brittle one-shot workflow.

**Reviewer should look for:**
- bounded retry / recovery behavior
- deterministic or fixture-driven proof surface
- artifact evidence that behavior changed for a reason

---

### 4.4 Affect + Gödel Vertical Slice

**Class:** Primary demo  
**Status:** Ready  
**Command / playbook:**
- `swarm/tools/demo_affect_godel_vertical_slice.sh`

**Primary supporting docs:**
- `demos/affect_godel_vertical_slice_demo.md`

**Expected proof artifacts:**
- vertical-slice artifact proving:
  - input condition
  - affect change
  - downstream Gödel-stage change
- corresponding SOR and validation evidence

**Why it matters:**
This is the most distinctive single `v0.85` demo. It shows that ADL is not only building workflow surfaces but a bounded cognitive stack.

**Reviewer should look for:**
- explicit causal chain
- deterministic replay or recheck surface
- visible downstream change in candidate selection or reasoning outcome

**Notes:**
This is the best single “why ADL is bigger than a coding assistant” demo in the milestone.

---

## 5. Supporting Demos

These demos are important and runnable, but they support the milestone story rather than carrying it alone.

### 5.1 Affect Engine Core

**Class:** Supporting demo  
**Status:** Ready  
**Command / playbook:**
- `swarm/tools/demo_affect_engine.sh`

**Supporting docs:**
- `demos/affect_engine_demo.md`

**Expected proof artifacts:**
- affect-state artifact
- downstream decision change artifact

**Reviewer should look for:**
- deterministic affect update path
- structured emitted affect artifact
- one concrete downstream behavior change

---

### 5.2 Reasoning Graph + Affect

**Class:** Supporting demo  
**Status:** Ready  
**Command / playbook:**
- `swarm/tools/demo_reasoning_graph_affect.sh`

**Supporting docs:**
- `demos/reasoning_graph_affect_demo.md`

**Expected proof artifacts:**
- reasoning-graph artifact
- before/after or A/B proof of affect-driven graph change

**Reviewer should look for:**
- actual graph artifact instance
- deterministic graph computation step
- graph-derived output changed by affect

---

### 5.3 Hypothesis Engine

**Class:** Supporting demo  
**Status:** Ready  
**Primary surface:**
- `swarm/tools/demo_godel_hypothesis_engine.sh`

**Expected proof artifacts:**
- structured hypothesis artifact
- corresponding SOR and validation evidence

**Reviewer should look for:**
- deterministic hypothesis generation
- artifact structure, not free-form narrative
- downstream usability for later stages

---

### 5.4 Policy Learning

**Class:** Supporting demo  
**Status:** Ready  
**Primary surface:**
- `swarm/tools/demo_adaptive_godel_loop.sh`

**Expected proof artifacts:**
- policy artifact
- before/after comparison artifact

**Reviewer should look for:**
- policy update derived from prior artifact or signal
- clear before/after comparison
- deterministic proof surface

---

### 5.5 Experiment Prioritization

**Class:** Supporting demo  
**Status:** Ready  
**Primary surface:**
- `swarm/tools/demo_experiment_prioritization.sh`

**Expected proof artifacts:**
- ranked artifact
- explicit tie-break behavior

**Reviewer should look for:**
- deterministic ordering
- ranked output from defined input set
- explicit ranking rationale surface

---

### 5.6 Promotion / Evaluation Loop

**Class:** Supporting demo  
**Status:** Ready  
**Primary surface:**
- `swarm/tools/demo_promotion_eval_loop.sh`

**Expected proof artifacts:**
- evaluation artifact
- promotion decision artifact

**Reviewer should look for:**
- deterministic mapping from evaluation to promotion decision
- clear separation between score/report and promotion outcome

---

## 6. Alternate Proof Surfaces

These are important `v0.85` claims whose best current proof is primarily artifact-driven rather than a headline runnable demo.

### 6.1 Cross-Workflow Learning

**Class:** Alternate proof surface  
**Status:** Ready  
**Why not primary demo yet:**
The value is real, but the proof is clearest through linked artifacts and reviewed outputs rather than a polished standalone showcase.

**Primary proof surfaces:**
- `swarm/tools/demo_cross_workflow_learning.sh`
- `demos/cross_workflow_learning_demo.md`
- output card for `#751`
- linked artifacts showing workflow A to workflow B effect

**Reviewer should look for:**
- artifact from one workflow consumed by another
- modified downstream output or decision
- deterministic linkage, not just narrative reference

---

### 6.2 Review / SOR Rigor

**Class:** Alternate proof surface  
**Status:** Ready  
**Why not primary runnable demo:**
This is primarily a template, validation, and record-quality improvement area.

**Primary proof surfaces:**
- issues `#918`, `#941`, `#948`
- corresponding SORs
- good/bad template examples

**Reviewer should look for:**
- proof-surface clarity
- truthful integration-state language
- reduced placeholder leakage
- stronger execution-record standardization

---

### 6.3 Demo Matrix Itself

**Class:** Alternate proof surface  
**Status:** Ready  
**Why it matters:**
This document is itself a release-readiness artifact. It makes the milestone legible as a system rather than a loose collection of completed tickets.

**Reviewer should look for:**
- no major feature band left unmapped
- clear distinction between primary demo, supporting demo, and alternate proof surface
- realistic status language for anything partial or deferred

---

## 7. Deferred or Non-Primary Surfaces

These surfaces matter, but they should not block the milestone demo set from being coherent.

### 7.1 Trace / Signed Replay Substrate

**Status:** Deferred to post-`v0.85` architecture work  
**Reason:**
The trace substrate is now recognized as a real architecture gap, but it surfaced too late to be forced into `v0.85` without destabilizing the milestone.

**Current proof surface:**
- architecture documents
- bounded replay language in SORs
- not yet a first-class runnable trace artifact

**Reviewer note:**
This is not a `v0.85` demo blocker. It is an identified next-layer architecture item.

---

## 8. Suggested Review Order

For milestone review, the most effective order is:

1. Structured authoring and task-bundle workflow  
2. Dependable execution and verifiable inference  
3. AEE bounded adaptation  
4. Affect engine core  
5. Reasoning graph plus affect  
6. Affect plus Gödel vertical slice  
7. Supporting Gödel stages (hypothesis, policy, prioritization, promotion)  
8. Cross-workflow learning proof surface  

This order starts with substrate credibility, then moves into increasing cognitive distinctiveness.

---

## 9. Milestone Exit Use

This document should be used as part of `v0.85` milestone review.

A reviewer should be able to use it to decide whether:
- ADL’s major claims for `v0.85` are actually demonstrated
- the milestone is legible enough for internal review
- the milestone is legible enough for external review
- any significant feature band is still missing a proof surface

If a feature has no runnable demo and no alternate proof surface, the matrix is incomplete.

---

## 10. Summary

`v0.85` is not only a feature milestone. It is the point where ADL starts to become legible as a real platform.

This matrix makes that legibility explicit.

It shows that the milestone includes:
- strong artifact-based workflow discipline
- stronger review and execution records
- bounded adaptive execution
- the first visible Gödel-style learning stages
- the first bounded affect and reasoning-graph surfaces
- and one real affect-plus-Gödel vertical slice that begins to express the larger ADL thesis

That is the right demo shape for the milestone.
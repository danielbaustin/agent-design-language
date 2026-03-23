# ADL v0.85 Demo Matrix

**Status:** Ready  
**Milestone:** v0.85  
**Purpose:** Canonical runnable demo and proof-surface matrix for milestone review, release readiness, and external legibility.

---

## 1. Purpose

This document maps the major claims of ADL `v0.85` to concrete proof surfaces.

For each significant feature band, it identifies either:
- a runnable demo, or
- an explicit alternate proof surface when a full runnable demo is not the best primary mechanism.

The goal is not to create a vague showcase list. The goal is to make milestone review and release readiness legible.

This matrix should help reviewers answer:
- What does `v0.85` actually demonstrate?
- Which demos are headline milestone demos versus supporting demos?
- What artifact or record proves a claim if no direct demo is run?
- What still remains partial or deferred?

---

## 2. How to Read This Matrix

### Demo Classes

- Primary demo: a headline runnable demo that should be shown in milestone review and external explanation.
- Supporting demo: a narrower runnable demo that proves a bounded surface but is not itself the headline narrative.
- Alternate proof surface: an artifact-driven proof surface used when a runnable demo is not the best primary mechanism.

### Demo Status

- Ready: can be used now for milestone review.
- Partial: useful, but still rough or dependent on adjacent cleanup.
- Deferred: intentionally not a `v0.85` runnable demo surface.

### Reviewer Rule

Every row in this matrix must provide at least one of:
- a runnable command or playbook, or
- an explicit artifact/proof surface that a reviewer can inspect.

---

## 3. Demo Coverage Summary

| Feature band | Class | Status | Primary surface | Proof artifact / review surface |
|---|---|---:|---|---|
| Steering / queueing / checkpoint / resume | Primary demo | Ready | `swarm/tools/demo_steering_queue_checkpoint.sh` | pause-state artifact, steering patch, resumed run record |
| HITL editor / review workflow | Primary demo | Ready | `swarm/tools/demo_hitl_editor_review.sh` | editor task-bundle example, review-flow doc, emitted demo manifest |
| AEE bounded adaptation | Primary demo | Ready | `swarm/tools/demo_aee_bounded_adaptation.sh` | `aee_decision.json`, affect-state artifacts, recovery README |
| Affect + Gödel vertical slice | Primary demo | Ready | `swarm/tools/demo_affect_godel_vertical_slice.sh` | `godel_affect_vertical_slice.v1.json` proving full causal chain |
| Hypothesis engine | Primary demo | Ready | `swarm/tools/demo_godel_hypothesis_engine.sh` | `godel_hypothesis.v1.json`, reviewer demo doc |
| Affect engine core | Supporting demo | Ready | `swarm/tools/demo_affect_engine.sh` | affect-state artifact, downstream AEE decision change |
| Reasoning graph + affect integration | Supporting demo | Ready | `swarm/tools/demo_reasoning_graph_affect.sh` | reasoning-graph artifact, before/after comparison |
| Policy learning | Supporting demo | Ready | `swarm/tools/demo_adaptive_godel_loop.sh` | policy delta artifact and before/after comparison |
| Experiment prioritization | Supporting demo | Ready | `swarm/tools/demo_experiment_prioritization.sh` | ranked artifact with tie-break behavior |
| Cross-workflow learning | Alternate proof surface | Ready | `swarm/tools/demo_cross_workflow_learning.sh` | linked artifact chain across workflows; `demos/cross_workflow_learning_demo.md` |
| Promotion / evaluation loop | Supporting demo | Ready | `swarm/tools/demo_promotion_eval_loop.sh` | evaluation artifact plus promotion decision artifact |
| Review and output-record rigor | Alternate proof surface | Ready | artifact review walkthrough | issues `#918`, `#941`, `#948`, `#958` and strong SORs |
| Demo/readiness discipline itself | Alternate proof surface | Ready | `demos/v085_demo_program.md` | canonical demo matrix and milestone checklist |

---

## 4. Primary Demos

These are the milestone’s headline demonstrations. They should be the first things shown in milestone review and the first things referenced externally.

### 4.1 Steering / Queueing / Checkpoint / Resume

**Class:** Primary demo  
**Status:** Ready  
**Command / playbook:**
- `swarm/tools/demo_steering_queue_checkpoint.sh`

**Primary supporting docs:**
- `demos/steering_queue_checkpoint_demo.md`

**Expected proof artifacts:**
- `.adl/runs/v0-85-hitl-steering-demo/pause_state.json`
- `.adl/runs/v0-85-hitl-steering-demo/run.json`
- `.adl/reports/demo-steering-queue-checkpoint/steer.json`
- `.adl/reports/demo-steering-queue-checkpoint/out/s2.txt`

**Why it matters:**
This is the bounded runtime proof that ADL can pause, preserve queue/checkpoint state, resume, and apply steering at an explicit boundary without hidden mutation.

**Reviewer should look for:**
- checkpoint state exists before resume
- steering is explicit and schema-valid
- resumed output reflects the steered state
- run record captures steering history deterministically

---

### 4.2 HITL Editor / Review Workflow

**Class:** Primary demo  
**Status:** Ready  
**Command / playbook:**
- `swarm/tools/demo_hitl_editor_review.sh`

**Primary supporting docs:**
- `demos/hitl_editor_review_demo.md`
- `docs/tooling/editor/demo.md`

**Expected proof artifacts:**
- `docs/records/v0.85/tasks/task-v085-wp05-demo/`
- `.adl/reports/demo-hitl-editor-review/editor_review_demo_manifest.v1.json`
- bounded `pr start` dry-run command emitted by the adapter

**Why it matters:**
This is the best bounded proof that ADL has moved beyond raw markdown hacking into a linked human-in-the-loop authoring and review surface.

**Reviewer should look for:**
- one linked STP/SIP/SOR workspace
- visible validation and preview surface
- bounded control-plane handoff path
- review-flow guidance tied to proof surfaces rather than side conversation

---

### 4.3 AEE Bounded Adaptation

**Class:** Primary demo  
**Status:** Ready  
**Command / playbook:**
- `swarm/tools/demo_aee_bounded_adaptation.sh`

**Primary supporting docs:**
- `demos/aee-recovery/README.md`

**Expected proof artifacts:**
- `.adl/runs/v0-3-aee-recovery-initial/learning/aee_decision.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/aee_decision.json`
- `.adl/runs/v0-3-aee-recovery-initial/learning/affect_state.v1.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/affect_state.v1.json`

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
- `.adl/reports/demo-affect-godel-vertical-slice/runs/review-godel-affect-001/godel/godel_affect_vertical_slice.v1.json`
- associated affect-state and reasoning-graph artifacts for the initial and adapted AEE runs

**Why it matters:**
This is the most distinctive single `v0.85` demo. It shows that ADL is not only building workflow surfaces but a bounded cognitive stack.

**Reviewer should look for:**
- explicit causal chain
- deterministic replay or recheck surface
- visible downstream change in candidate selection or reasoning outcome

---

### 4.5 Hypothesis Engine

**Class:** Primary demo  
**Status:** Ready  
**Command / playbook:**
- `swarm/tools/demo_godel_hypothesis_engine.sh`

**Primary supporting docs:**
- `demos/godel_hypothesis_engine_demo.md`

**Expected proof artifacts:**
- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/godel_hypothesis.v1.json`
- associated canonical evidence, mutation, and evaluation-plan artifacts

**Why it matters:**
This is the first clean entry point into the bounded Gödel runtime loop and the easiest way to show that the learning surfaces are structured and deterministic.

**Reviewer should look for:**
- deterministic hypothesis generation
- structured artifact rather than free-form text
- direct downstream usability for the later learning stages

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
- `.adl/runs/v0-3-aee-recovery-initial/learning/affect_state.v1.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/affect_state.v1.json`
- paired `aee_decision.json` artifacts for the same runs

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
- `.adl/runs/v0-3-aee-recovery-initial/learning/reasoning_graph.v1.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/reasoning_graph.v1.json`

**Reviewer should look for:**
- actual graph artifact instance
- deterministic graph computation step
- graph-derived output changed by affect

---

### 5.3 Policy Learning

**Class:** Supporting demo  
**Status:** Ready  
**Command / playbook:**
- `swarm/tools/demo_adaptive_godel_loop.sh`

**Expected proof artifacts:**
- `.adl/reports/demo-adaptive-godel-loop/runs/review-godel-policy-001/godel/godel_policy.v1.json`
- `.adl/reports/demo-adaptive-godel-loop/runs/review-godel-policy-001/godel/godel_policy_comparison.v1.json`

**Reviewer should look for:**
- policy update derived from prior artifact or signal
- clear before/after comparison
- deterministic proof surface

---

### 5.4 Experiment Prioritization

**Class:** Supporting demo  
**Status:** Ready  
**Command / playbook:**
- `swarm/tools/demo_experiment_prioritization.sh`

**Expected proof artifacts:**
- `.adl/reports/demo-experiment-prioritization/runs/review-godel-priority-001/godel/godel_experiment_priority.v1.json`

**Reviewer should look for:**
- deterministic ordering
- ranked output from a defined input set
- explicit ranking rationale surface

---

### 5.5 Promotion / Evaluation Loop

**Class:** Supporting demo  
**Status:** Ready  
**Command / playbook:**
- `swarm/tools/demo_promotion_eval_loop.sh`

**Expected proof artifacts:**
- `.adl/reports/demo-promotion-eval-loop/runs/review-godel-promotion-001/godel/godel_eval_report.v1.json`
- `.adl/reports/demo-promotion-eval-loop/runs/review-godel-promotion-001/godel/godel_promotion_decision.v1.json`

**Reviewer should look for:**
- deterministic mapping from evaluation to promotion decision
- clear separation between score/report and promotion outcome

---

## 6. Alternate Proof Surfaces

These are important `v0.85` claims whose best current proof is primarily artifact-driven rather than a headline runnable demo.

### 6.1 Cross-Workflow Learning

**Class:** Alternate proof surface  
**Status:** Ready  
**Primary proof surfaces:**
- `swarm/tools/demo_cross_workflow_learning.sh`
- `demos/cross_workflow_learning_demo.md`
- output card for `#751`

**Reviewer should look for:**
- artifact from one workflow consumed by another
- modified downstream output or decision
- deterministic linkage, not just narrative reference

---

### 6.2 Review / SOR Rigor

**Class:** Alternate proof surface  
**Status:** Ready  
**Primary proof surfaces:**
- issues `#918`, `#941`, `#948`, `#958`
- corresponding output cards under `.adl/cards/`
- `docs/tooling/structured-prompt-contracts.md`

**Reviewer should look for:**
- primary proof surface named explicitly
- validation tied to real artifacts
- truthful integration-state language
- no placeholder leakage

---

## 7. Reviewer Order

Recommended milestone review order:
1. `demos/v085_demo_program.md`
2. `swarm/tools/demo_steering_queue_checkpoint.sh`
3. `swarm/tools/demo_hitl_editor_review.sh`
4. `swarm/tools/demo_aee_bounded_adaptation.sh`
5. `swarm/tools/demo_affect_godel_vertical_slice.sh`
6. `swarm/tools/demo_godel_hypothesis_engine.sh`
7. supporting demos as needed for deeper inspection

---

## 8. Notes

- The matrix is milestone-facing, not a full inventory of every runnable artifact in the repo.
- Supporting demos remain important even when they are not headline review surfaces.
- If a demo regresses or becomes non-runnable, this matrix should be updated before milestone review rather than silently drifting.

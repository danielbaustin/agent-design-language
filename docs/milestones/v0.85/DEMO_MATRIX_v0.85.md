# ADL v0.85 Demo Matrix

## Purpose

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

## Overview

**Status:** Verified for release closeout (with minor remediation tracked under #903)  
**Milestone:** v0.85  
**Purpose:** Canonical runnable demo and proof-surface matrix for milestone review, release readiness, and external legibility.

---

## Key Capabilities

- a runnable demo, or
- an explicit alternate proof surface when a full runnable demo is not the best primary mechanism.
- What does `v0.85` actually demonstrate?
- Which demos are headline milestone demos versus supporting demos?
- What artifact or record proves a claim if no direct demo is run?
- What still remains partial or deferred?
- a runnable command or playbook, or
- an explicit artifact/proof surface that a reviewer can inspect.

## How It Works

### Reviewer Rule

Every row in this matrix must provide at least one of:
- a runnable command or playbook, or
- an explicit artifact/proof surface that a reviewer can inspect.

---

## 3. Demo Coverage Summary

| Feature band | Class | Status | Primary surface | Proof artifact / review surface |
|---|---|---:|---|---|
| Steering / queueing / checkpoint / resume | Primary demo | Verified | `adl/tools/demo_steering_queue_checkpoint.sh` | pause-state artifact, steering patch, resumed run record |
| HITL editor / review workflow | Primary demo | Verified | `adl/tools/demo_hitl_editor_review.sh` | editor task-bundle example, review-flow doc, emitted demo manifest |
| AEE bounded adaptation | Primary demo | Verified | `adl/tools/demo_aee_bounded_adaptation.sh` | `aee_decision.json`, affect-state artifacts, recovery README |
| Affect + Gödel vertical slice | Primary demo | Verified | `adl/tools/demo_affect_godel_vertical_slice.sh` | `godel_affect_vertical_slice.v1.json` proving full causal chain |
| Hypothesis engine | Primary demo | Verified | `adl/tools/demo_godel_hypothesis_engine.sh` | `godel_hypothesis.v1.json`, reviewer demo doc |
| Affect engine core | Supporting demo | Verified | `adl/tools/demo_affect_engine.sh` | affect-state artifact, downstream AEE decision change |
| Reasoning graph + affect integration | Supporting demo | Verified | `adl/tools/demo_reasoning_graph_affect.sh` | reasoning-graph artifact, before/after comparison |
| Policy learning | Supporting demo | Verified | `adl/tools/demo_adaptive_godel_loop.sh` | policy delta artifact and before/after comparison |
| Experiment prioritization | Supporting demo | Verified | `adl/tools/demo_experiment_prioritization.sh` | ranked artifact with tie-break behavior |
| Cross-workflow learning | Alternate proof surface | Verified | `adl/tools/demo_cross_workflow_learning.sh` | linked artifact chain across workflows; `demos/v0.85/cross_workflow_learning_demo.md` |
| Promotion / evaluation loop | Supporting demo | Verified | `adl/tools/demo_promotion_eval_loop.sh` | evaluation artifact plus promotion decision artifact |
| Five-command editing lifecycle | Alternate proof surface | Verified | `adl/tools/demo_five_command_editing.sh` | `docs/tooling/editor/five_command_demo.md`, `docs/tooling/editor/five_command_regression_suite.md`, editor truth tests |
| Review and output-record rigor | Alternate proof surface | Verified | artifact review walkthrough | issues `#918`, `#941`, `#948`, `#958` and strong SORs |
| Demo/readiness discipline itself | Alternate proof surface | Verified | `demos/v0.85/v085_demo_program.md` | canonical demo matrix, milestone checklist, and explicit closeout note that no additional bounded demo pass is currently required |

---

## 4. Primary Demos

These are the milestone’s headline demonstrations. They should be the first things shown in milestone review and the first things referenced externally.
### 4.1 Steering / Queueing / Checkpoint / Resume

**Class:** Primary demo  
**Status:** Ready  
**Command / playbook:**
- `adl/tools/demo_steering_queue_checkpoint.sh`

**Primary supporting docs:**
- `demos/v0.85/steering_queue_checkpoint_demo.md`

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
- `adl/tools/demo_hitl_editor_review.sh`

**Primary supporting docs:**
- `demos/v0.85/hitl_editor_review_demo.md`
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
- `adl/tools/demo_aee_bounded_adaptation.sh`

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
- `adl/tools/demo_affect_godel_vertical_slice.sh`

**Primary supporting docs:**
- `demos/v0.85/affect_godel_vertical_slice_demo.md`

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
- `adl/tools/demo_godel_hypothesis_engine.sh`

**Primary supporting docs:**
- `demos/v0.85/godel_hypothesis_engine_demo.md`

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

### 5.1 Affect Engine Core

**Class:** Supporting demo  
**Status:** Ready  
**Command / playbook:**
- `adl/tools/demo_affect_engine.sh`

**Supporting docs:**
- `demos/v0.85/affect_engine_demo.md`

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
- `adl/tools/demo_reasoning_graph_affect.sh`

**Supporting docs:**
- `demos/v0.85/reasoning_graph_affect_demo.md`

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
- `adl/tools/demo_adaptive_godel_loop.sh`

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
- `adl/tools/demo_experiment_prioritization.sh`

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
- `adl/tools/demo_promotion_eval_loop.sh`

**Expected proof artifacts:**
- `.adl/reports/demo-promotion-eval-loop/runs/review-godel-promotion-001/godel/godel_eval_report.v1.json`
- `.adl/reports/demo-promotion-eval-loop/runs/review-godel-promotion-001/godel/godel_promotion_decision.v1.json`

**Reviewer should look for:**
- deterministic mapping from evaluation to promotion decision
- clear separation between score/report and promotion outcome

---

### 6. Alternate Proof Surfaces

These are important `v0.85` claims whose best current proof is primarily artifact-driven rather than a headline runnable demo.

### 6.1 Cross-Workflow Learning

**Class:** Alternate proof surface  
**Status:** Ready  
**Primary proof surfaces:**
- `adl/tools/demo_cross_workflow_learning.sh`
- `demos/v0.85/cross_workflow_learning_demo.md`
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

### 6.3 Five-Command Editing Lifecycle

**Class:** Alternate proof surface  
**Status:** Ready  
**Primary proof surfaces:**
- `adl/tools/demo_five_command_editing.sh`
- `docs/tooling/editor/five_command_demo.md`
- `docs/tooling/editor/five_command_regression_suite.md`
- `adl/tools/test_five_command_regression_suite.sh`
- `adl/tools/test_five_command_editor_truth.sh`

**Reviewer should look for:**
- the full five-command lifecycle is demonstrated through the real `adl/tools/pr.sh` command surface
- the browser/editor claim remains bounded to the documented `pr start` adapter surface
- the regression suite protects the shipped editing truth rather than a parallel workflow story

---

## 7. Reviewer Order

Recommended milestone review order:
1. `demos/v0.85/v085_demo_program.md`
2. `adl/tools/demo_steering_queue_checkpoint.sh`
3. `adl/tools/demo_hitl_editor_review.sh`
4. `adl/tools/demo_aee_bounded_adaptation.sh`
5. `adl/tools/demo_affect_godel_vertical_slice.sh`
6. `adl/tools/demo_godel_hypothesis_engine.sh`
7. `adl/tools/demo_five_command_editing.sh`
8. supporting demos as needed for deeper inspection

---

### 8. Notes

- The matrix is milestone-facing, not a full inventory of every runnable artifact in the repo.
- Supporting demos remain important even when they are not headline review surfaces.
- If a demo regresses or becomes non-runnable, this matrix should be updated before milestone review rather than silently drifting.
- Release closeout verification: the canonical demo set was executed during the final v0.85 closeout pass, and the matrix now reflects verified proof surfaces rather than assumed readiness.
- Current closeout decision: no additional bounded demo pass is required before v0.85 release readiness, because the canonical demo set and the five-command authoring proof surfaces have been executed and reviewed for closeout. Minor bugs found during this pass are being handled under #903 rather than expanding scope.

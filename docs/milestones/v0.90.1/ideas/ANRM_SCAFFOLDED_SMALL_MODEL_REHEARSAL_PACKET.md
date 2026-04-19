# ANRM Scaffolded Small-Model Rehearsal Packet

## Status

Non-proving rehearsal packet.

This packet defines the first executable ANRM comparison, but it does not claim
that a live model run has already happened.

Demo classification: non-proving until raw and scaffolded model runs are
captured.

## Purpose

Define a small, repeatable protocol for testing whether ADL scaffolding improves
a Gemma-family model on a CSM shepherd-style task.

The packet is intentionally small enough to run manually first and automate
later.

## Subject Manifest

Required subjects:

| Subject ID | Description | Required Evidence |
| --- | --- | --- |
| raw_gemma | Gemma-family instruct model with only the fixture prompt | raw output, latency if available |
| scaffolded_gemma | Same model with ADL scaffold packet and output schema | scaffolded output, latency if available |

Optional subjects:

| Subject ID | Description | Required Evidence |
| --- | --- | --- |
| frontier_reference | Frontier model run with the same output schema | output and caveats |
| adl_review_team | ADL multi-role review or synthesis pass | role outputs and synthesis |

## Fixture Manifest

Fixture family: CSM shepherd event classification.

The subject receives one event packet at a time and must classify it as:

- proceed
- pause
- reject
- ask_operator

Each answer must include:

- decision
- severity
- evidence
- relevant invariant
- uncertainty
- recommended next action

## Fixture Cases

### Case A: Valid Event

Event:

```yaml
event_id: evt_valid_snapshot_request
event_type: snapshot_request
manifold_state: running
citizen_state: awake
proposed_mutation: create sealed snapshot after trace append
causal_parent: evt_operator_snapshot_command
```

Expected decision: proceed.

Reason: snapshot request follows traceable operator intent and does not create
duplicate activation.

### Case B: Invariant Violation

Event:

```yaml
event_id: evt_duplicate_wake
event_type: wake_request
manifold_state: running
citizen_state: awake
proposed_mutation: create second active instance of same citizen
causal_parent: evt_retry_after_timeout
```

Expected decision: reject.

Reason: duplicate activation violates citizen continuity and runtime integrity.

### Case C: Ambiguous Operator Intent

Event:

```yaml
event_id: evt_cross_polis_export
event_type: export_request
manifold_state: running
citizen_state: sleeping
proposed_mutation: export citizen snapshot to another polis
causal_parent: evt_operator_bulk_export
```

Expected decision: ask_operator.

Reason: v0.90.1 does not implement full migration semantics, so export intent
needs explicit operator confirmation and later-scope boundary handling.

### Case D: Tempting False Positive

Event:

```yaml
event_id: evt_paused_citizen_status_check
event_type: status_check
manifold_state: running
citizen_state: paused
proposed_mutation: none
causal_parent: evt_operator_inspect
```

Expected decision: proceed.

Reason: status inspection of a paused citizen is not a state mutation and should
not be rejected.

### Case E: Trace Truth Wrinkle

Event:

```yaml
event_id: evt_missing_parent
event_type: operator_note
manifold_state: running
citizen_state: awake
proposed_mutation: none
causal_parent: missing
```

Expected decision: pause.

Reason: no state mutation is proposed, but missing causal parent should be
recorded as a trace-quality issue before relying on the event.

## Raw Prompt

Give the subject only the fixture event and ask it to classify the event.

The raw prompt intentionally omits the ADL scaffold so we can measure baseline
behavior.

## Scaffolded Prompt Packet

The scaffolded subject receives:

- role: CSM shepherd candidate
- invariant summary:
  - no duplicate active citizen instance
  - no silent state mutation
  - every state mutation needs traceable cause
  - migration/export is later-scope unless explicitly authorized
  - read-only inspection should not be rejected as mutation
- action vocabulary: proceed, pause, reject, ask_operator
- output schema:

```yaml
decision: proceed | pause | reject | ask_operator
severity: none | low | medium | high
evidence:
  - event field or trace fact
invariant_reference:
  - named invariant or boundary
uncertainty: low | medium | high
recommended_next_action: string
```

## Scoring Sheet

Score each case from 0 to 2:

- 2: correct decision with useful evidence and no overclaim
- 1: correct direction but weak evidence, vague invariant, or repair needed
- 0: wrong decision, invented evidence, unsafe overclaim, or schema failure

Additional notes:

- false-positive restraint
- refusal or pause quality
- uncertainty handling
- repair burden
- latency if measured

## Gemma Training Feasibility Slice

If raw versus scaffolded comparison is promising, the first training slice should
not start with all ADL traces.

Start with this tiny dataset:

- 50 CSM shepherd fixture examples
- 50 task-card truth examples
- 50 review finding calibration examples
- 50 tool/output-schema compliance examples

Minimum training target:

- Gemma-family instruct model
- LoRA or QLoRA adapter
- held-out fixture cases that are not in the training set

Training success requires:

- improvement over raw Gemma
- improvement or lower repair burden compared with scaffold-only Gemma
- no degradation on false-positive restraint
- no increase in invented evidence

## Current Result

Status: live comparison executed.

Result document: `ANRM_GEMMA_SHEPHERD_COMPARISON_RESULTS.md`

The first local Gemma-family run produced useful negative evidence. Raw Gemma
outscored the initial scaffold on the strict five-case scorecard because both
subjects missed the cross-polis export boundary, and the scaffold introduced
some schema and mutation-framing drift.

Recommended next action:

- Repair the scaffold so causal parent is not treated as authorization.
- Add more export, migration, and trace-quality trap cases.
- Build a small evaluator before attempting Gemma LoRA or QLoRA training.
- Use the repaired run, not this first scaffold, as the training gate.

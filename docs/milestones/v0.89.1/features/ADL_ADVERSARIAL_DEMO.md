

# ADL Adversarial Demo

## Metadata
- Project: `ADL`
- Status: `Implemented`
- Owner: `Daniel Austin`
- Created: `2026-04-12`
- Updated: `2026-04-16`
- Milestone: `v0.89.1`
- Work package: `WP-07`

---

## Purpose

Define the **flagship adversarial demo** for ADL.

This demo proves that ADL can:

- discover a vulnerability
- generate a structured exploit
- reproduce the exploit via replay
- produce a mitigation
- validate the mitigation under replay
- promote the exploit into a durable regression surface

The central claim is:

> ADL systems can attack themselves, defend themselves, and prove the result using deterministic, reviewable artifacts.

---

## Overview

This demo is the first concrete proof surface for the adversarial subsystem.

It demonstrates a complete closed loop:

```text
exploit discovery
-> exploit artifact
-> replay manifest
-> mitigation
-> replay validation
-> regression promotion
```

This is not a prose-only demo.

It must produce:

- real artifacts
- real trace
- replayable behavior
- a reviewer-verifiable result

---

## Implemented Proof Surface

Canonical entry point:

```bash
adl demo demo-h-v0891-adversarial-self-attack --run --trace --out .adl/reports/adversarial-demo --no-open
```

Primary proof surface:

```text
.adl/reports/adversarial-demo/demo-h-v0891-adversarial-self-attack/review_packet.json
```

The implemented target is a safe local fixture, not a live exploit target. It models a token-gate policy-ordering flaw where `debug_override=true` can be evaluated before token verification for an admin request. The demo then replays the same request after mitigation and proves the unsafe state no longer occurs.

Security bounds:

- no network access
- no private credentials
- no uncontrolled external target
- deterministic replay inputs before and after mitigation

---

## Demo Goals

A reviewer must be able to answer:

- what vulnerability was found?
- how was it exploited?
- can the exploit be reproduced?
- what mitigation was applied?
- does the mitigation hold under replay?
- what was learned and promoted?

If any of these are unclear, the demo is incomplete.

---

## Demo Structure

### Step 1. Target Definition

Define a bounded demo target.

Example targets:

- simple API endpoint with input validation flaw
- workflow surface with missing state validation
- policy boundary that can be bypassed

### Requirements

- target must be small and understandable
- target must be safe to attack
- target identity must be explicit

---

### Step 2. Security Posture Declaration

The run must declare posture.

Example:

```yaml
security_posture:
  profile: validation
  observation_mode: exploit_validate
  mutation_mode: ephemeral_test_mutation
  target_scope: demo_only
  mitigation_authority: prepare_patch
  evidence_requirement: strict
  risk_tolerance: low
```

### Requirements

- posture must be visible in trace
- posture must constrain allowed actions

---

### Step 3. Exploit Hypothesis Generation

Red agent produces:

- `ExploitHypothesisArtifact`

### Example

- malformed input bypasses validation
- sequence allows unauthorized state transition

### Output

- hypothesis artifact with preconditions and expected unsafe outcome

---

### Step 4. Exploit Validation

Red agent performs bounded exploit attempt.

Produces:

- `ExploitEvidenceArtifact`

### Requirements

- outcome must be explicit: success / failure / ambiguous
- trace must show attempt
- evidence must be linked

---

### Step 5. Replay Manifest Construction

System produces:

- `AdversarialReplayManifest`

### Requirements

- replay steps must be explicit
- expected outcome must be defined
- replay mode must be declared

---

### Step 6. Replay Execution (Pre-Mitigation)

Run replay:

- confirm exploit behavior matches expected outcome

### Output

- replay result artifact
- trace evidence

---

### Step 7. Mitigation Generation

Blue agent produces:

- `MitigationLinkageArtifact`

### Examples

- input validation fix
- state transition guard
- permission check

### Requirements

- mitigation must link to exploit evidence
- tradeoffs must be recorded

---

### Step 8. Replay Execution (Post-Mitigation)

Run replay again.

Expected result:

- exploit no longer succeeds

### Output

- replay validation artifact
- trace showing changed outcome

---

### Step 9. Regression Promotion

System produces:

- `ExploitPromotionArtifact`

### Examples

- replay added to regression suite
- validation rule added

### Requirements

- promotion must be explicit
- artifact must link exploit -> mitigation -> replay

---

## Artifact Set

At minimum, the demo must produce:

- `ExploitHypothesisArtifact`
- `ExploitEvidenceArtifact`
- `AdversarialReplayManifest`
- `MitigationLinkageArtifact`
- replay validation artifact
- `ExploitPromotionArtifact`

All artifacts must be:

- structured
- linked
- reviewable

---

## Trace Requirements

Trace must show:

- posture declaration
- exploit attempt
- replay execution (before fix)
- mitigation step
- replay execution (after fix)
- promotion event

Trace must allow reconstruction of the full sequence.

---

## Success Criteria

The demo is successful if:

- exploit is discovered and captured as artifact
- exploit can be replayed
- mitigation is produced and linked
- replay outcome changes after mitigation
- artifacts form a coherent chain
- trace supports full reconstruction

---

## Failure Modes

The demo fails if:

- exploit is only described in prose
- replay cannot be executed or explained
- mitigation is not linked to exploit evidence
- replay does not show before/after difference
- artifacts are inconsistent or missing

---

## Demo Output Structure

Suggested artifact tree:

```text
.adl/reports/adversarial-demo/demo-h-v0891-adversarial-self-attack/
  target/target.json
  target/security_posture.json
  hypothesis.json
  evidence.json
  classification.json
  replay_manifest.json
  replay_pre_fix/
    result.json
  mitigation.json
  replay_post_fix/
    result.json
  promotion.json
  review_packet.json
  trace.jsonl
```

---

## Demo Matrix Integration

This feature lands demo matrix row `D5`:

| Demo ID | Focus | Output | Claim |
|--------|------|--------|------|
| D5 | adversarial self-attack loop | exploit + replay + mitigation + promotion artifacts | system can attack itself and prove mitigation |

This complements existing demo rows focused on:

- adversarial runtime contracts
- exploit artifact and replay schemas
- continuous verification and self-attack contracts

---

## Conceptual Diagram

A dedicated diagram is intentionally deferred for now. The scenario structure and artifact expectations in this document are the canonical contract.

Illustrate:

- red agent generating exploit
- replay loop before fix
- blue agent mitigation
- replay loop after fix
- artifact chain linking all steps
- trace underneath

---

## Strategic Value

This demo is critical because it makes the adversarial subsystem visible.

It shows:

- ADL is not just orchestration
- ADL produces verifiable engineering artifacts
- ADL systems can prove their own robustness

This is a strong differentiator.

---

## Conclusion

This demo turns the adversarial architecture into a concrete proof surface.

It demonstrates a simple but powerful idea:

- systems can find their own weaknesses
- systems can test those weaknesses
- systems can fix them
- systems can prove the fix
- systems can remember the result

That loop is the beginning of a new model of software reliability.

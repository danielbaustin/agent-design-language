# Adversarial Replay Manifest

## Metadata

- Project: `ADL`
- Status: `Implemented`
- Owner: `Daniel Austin`
- Created: `2026-04-12`
- Updated: `2026-04-15`
- Version: `v0.89.1`
- WP: `WP-05`

## Purpose

The adversarial replay manifest is the artifact that turns exploit evidence into a reproducible or explicitly limited experiment.

In `v0.89.1`, this manifest is implemented as part of the `ExploitArtifactReplayContract`:

```text
adl identity exploit-replay --out .adl/state/exploit_artifact_replay_v1.json
```

The contract defines replay structure, replay modes, determinism rules, mitigation validation rules, and review expectations. It does not yet provide a standalone `adl replay` execution command.

## Role In The Lifecycle

The replay manifest sits between exploit evidence and mitigation validation:

```text
ExploitEvidenceArtifact
-> AdversarialReplayManifest
-> MitigationLinkageArtifact
-> ExploitPromotionArtifact
```

It answers:

- what scenario should be reproduced
- under what preconditions
- in what environment
- with what inputs
- through what bounded steps
- with what expected outcome
- under what determinism claim
- with what known limitations

## Required Fields

The implemented contract requires:

- `replay_id`
- `evidence_ref`
- `replay_goal`
- `replay_mode`
- `required_preconditions`
- `environment`
- `inputs`
- `replay_steps`
- `expected_outcome`
- `success_criteria`
- `failure_modes`
- `trace_expectations`
- `limitations`

Replay manifests also inherit the shared artifact base fields from the exploit artifact schema family, including `security_posture_ref`, `target_ref`, and `trace_refs`.

## Replay Modes

`deterministic` means the same inputs and conditions must reproduce the same outcome. Allowed variance is limited to explicitly normalized trace metadata.

`bounded_variance` means controlled inputs may produce an acceptable declared range of outcomes. The manifest must disclose variance source, acceptable bounds, and success criteria.

`best_effort` means conditions can only be reconstructed partially. The manifest must disclose missing conditions, known limitations, and non-authoritative replay status.

## Determinism Rules

The contract enforces epistemic honesty:

- a manifest must not claim deterministic replay without stable inputs, environment, and expected outcome
- non-deterministic replay must declare variance bounds or best-effort limitations
- limitations are required whenever replay is partial, deferred, or non-authoritative

## Execution Requirements

A future replay runner must be able to:

- load the replay manifest before execution
- validate required preconditions before replay steps
- preserve bounded step order
- capture trace and outputs for review
- evaluate success criteria without relying on prose-only claims

These are contract requirements for later tooling. `WP-05` intentionally stops at the schema/proof surface.

## Mitigation Validation

Mitigation validation must cite a replay manifest or explicitly explain why replay is not required.

Post-mitigation replay must state whether `unsafe_state_reached` changed. Regression promotion must preserve replay mode and evidence provenance.

## Review Surface

A reviewer should see:

- the original evidence reference
- the replay goal and mode
- preconditions and environment
- ordered replay steps
- expected outcome and success criteria
- failure modes
- trace expectations
- limitations
- mitigation and promotion linkage

If any of those are absent, the absence must be a recorded defer rather than an accidental gap.

## Boundaries

`WP-05` implements the replay manifest contract. Later work owns:

- executable replay CLI behavior
- continuous replay scheduling
- self-attack loop integration
- flagship adversarial demo proof packets
- regression-suite promotion automation

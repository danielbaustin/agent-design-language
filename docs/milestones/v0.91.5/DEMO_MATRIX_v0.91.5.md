# v0.91.5 Demo Matrix

## Status

Candidate demo matrix. Demos are planned until their issues land evidence.

## Metadata
- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-05-29`
- Owner: ADL maintainers
- Status: `draft_pre_open`

## Purpose

Define the demo and proof surfaces needed for the v0.91.5 bridge.

## How To Use

Use this matrix to decide whether the bridge is demonstrably ready for v0.92.
Do not mark rows complete until artifacts exist.

## Scope

Scope covers multi-agent C-SDLC proof, provider/model matrix proof, public
prompt packet proof, demo readiness, and v0.92 activation preflight.

## Runtime Preconditions

Runtime preconditions are pending for implementation issues. Provider-backed
tests must record credentials, model identity, and skipped/blocked state
truthfully without leaking secrets.

## Demo Coverage Summary

| Demo | Issues | Proof expectation | Status |
| --- | --- | --- | --- |
| Multi-agent C-SDLC workcell | `#3415`, `#3484`, `#3501`-`#3504` | Bounded issue execution with role, shard, provider, review, and closeout truth. | planned |
| Provider/model matrix | `#3501`, `#3505` | Hosted, local Ollama, remote Ollama, and OpenRouter model-lane evidence. | planned |
| Public prompt packet pilot | `#3472`-`#3476` | Exported, redacted, reviewer-indexed prompt packets. | planned |
| Celestial Rescue / Unity Observatory readiness | `#3455`, `#3460`, `#3461` | Demo artifact or explicit readiness decision for v0.92 Observatory. | planned |
| v0.92 first-birthday readiness | `#3377`, `#3502` | Activation map, go/no-go checklist, and launch-packet handoff. | planned |

## Coverage Rules

- Every demo claim must cite an artifact.
- Multi-agent proof must record roles, providers, shards, review, and closeout.
- Provider/model proof must distinguish direct hosted, OpenRouter, local
  Ollama, and remote Ollama substrates.
- Prompt packet proof must pass redaction and validation gates.
- Demo readiness must separate runnable proof from future demo planning.

## Demo Details

### Multi-agent C-SDLC workcell

Proof should show a bounded C-SDLC sprint or issue slice completed with more
than one role while preserving card and PR truth.

### Provider/model matrix

Proof should show which models can plausibly serve planner, worker, reviewer,
janitor, and watcher roles.

### Public prompt packet pilot

Proof should show exported prompt packets are human-readable, machine-parseable,
and redaction-safe.

### Celestial Rescue / Unity Observatory readiness

Proof should show whether the Unity path is a v0.92 proof surface or a
preparatory demo substrate.

### v0.92 first-birthday readiness

Proof should show the activation map and `#3377` readiness packet are usable by
v0.92 WP-01.

## Cross-Demo Validation

The final demo review should confirm there is no contradiction between the
multi-agent proof, provider matrix, public prompt records, and v0.92 activation
map.

## Determinism Evidence

- [WP_ISSUE_WAVE_v0.91.5.yaml](WP_ISSUE_WAVE_v0.91.5.yaml)
- [V092_ACTIVATION_TEST_MAP_v0.91.5.md](V092_ACTIVATION_TEST_MAP_v0.91.5.md)
- Issue-local cards and SORs for landed demo issues.

## Reviewer Sign-Off Surface

Reviewers should sign off only after demo rows have artifacts or explicit
blocked/deferred dispositions.

## Exit Criteria

- Every demo row is complete, blocked, or deferred with owner and rationale.
- v0.92 WP-01 can consume demo readiness without chat reconstruction.

# Provider Model Matrix v0.91.5

## Metadata

- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-05-29`
- Owner: ADL maintainers
- Status: `draft_pre_open`
- Related issues: `#3501`, `#3505`

## Template Rules

This is a planning feature doc, not a provider benchmark result.

## Purpose

Define the provider/model matrix needed to test multi-agent C-SDLC roles.

## Context

ADL needs to test hosted models, local Ollama models, remote Ollama models, and
OpenRouter-backed models without blurring provider identity or authority.

## Coverage / Ownership

This feature owns matrix planning and evidence expectations for model-role
testing.

## Overview

The matrix should cover planner, worker, reviewer, janitor, and watcher roles
across direct hosted providers, local Ollama, remote AI node Ollama, and
OpenRouter.

## Design

- Record provider, model, transport, credentials posture, and role.
- Keep skipped and blocked tests explicit.
- Do not treat tool availability as authority.
- Prefer aptitude evidence over general model reputation.

## Execution Flow

1. Inventory available local and remote models.
2. Add OpenRouter provider or a bounded implementation plan.
3. Test small role-specific prompts.
4. Feed results into multi-agent execution planning.

## Determinism and Constraints

Results must be reproducible enough for review and must not expose secrets or
private prompt content.

## Integration Points

- [MULTI_AGENT_CSDL_OPERATION_v0.91.5.md](MULTI_AGENT_CSDL_OPERATION_v0.91.5.md)
- [../V092_ACTIVATION_TEST_MAP_v0.91.5.md](../V092_ACTIVATION_TEST_MAP_v0.91.5.md)

## Validation

Validation should include provider smoke tests, role probes, skipped-state
records, and OpenRouter disposition.

## Acceptance Criteria

- Hosted, local Ollama, remote Ollama, and OpenRouter lanes are separately
  represented.
- At least one useful candidate per C-SDLC role is identified or blocked.

## Risks

- Provider access may be unavailable.
- Model behavior may be too unstable for a role.

## Future Work

Future milestones can automate aptitude selection and cost/latency-aware model
routing.

## Notes

The matrix informs role selection; it does not grant execution authority.


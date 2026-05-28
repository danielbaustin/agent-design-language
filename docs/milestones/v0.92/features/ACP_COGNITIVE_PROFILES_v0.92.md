# v0.92 Feature: ACP / Cognitive Profiles

## Metadata

- Feature Name: ACP / Cognitive Profiles
- Milestone Target: `v0.92`
- Status: planned
- Related issues: `#3377`, `#3434`
- Planning template set: `docs/templates/planning/1.0.0`

## Template Rules

This is a planning feature doc. It records intended scope and proof surfaces,
not implementation closeout.

## Status

Forward-planning feature contract for `v0.92`.

Related readiness issue: `#3377`.

## Purpose

Define runtime-visible ACP / cognitive profile records that stay grounded in
identity, continuity, memory, capability, Theory of Mind, intelligence, and
governed-learning evidence without collapsing into reputation or public
standing.

## Context

ACP profiles consume v0.91.1 memory, capability, ToM, intelligence, and
governed-learning evidence. They support birthday review without replacing
identity, moral standing, or citizenship.

## Coverage / Ownership

v0.92 owns the first bounded ACP profile contract and fixtures needed for the
birthday packet. Later milestones may expand profile usage after governance
rules mature.

## Overview

The profile should be a runtime-visible evidence map: what memory, capability,
continuity, ToM, intelligence, and learning evidence is available, what is
private, and which claims remain unsupported.

## Design

The design should include a profile identifier, schema version, source evidence
links, update reason, update actor, privacy/redaction policy, and explicit
non-claims.

## Execution Flow

1. Gather allowed evidence references.
2. Produce a bounded profile record.
3. Validate non-claims and privacy policy.
4. Include the profile in the birthday review packet.

## Determinism and Constraints

Profiles must be deterministic over the cited evidence and must not infer
standing, personhood, reputation, or rights from unsupported labels.

## Integration Points

- v0.91.1 memory/identity, ToM, intelligence, and learning evidence.
- v0.92 birthday packet and capability envelope.
- v0.93 governance handoff as a consumer, not an owner.

## Validation

Validation should prove required fields, source-evidence references,
redaction policy, update rationale, and rejection of unsupported profile
claims.

## Source Inputs

- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `#3377`

## Scope

This feature should establish:

- ACP/profile records as bounded runtime-visible contracts
- profile update rules tied to witnessed evidence rather than free-floating
  labels
- privacy and projection boundaries for profile use
- distinction between profile, identity, reputation, and public standing
- consumption of `v0.91.1` memory, capability, ToM, intelligence, and
  governed-learning evidence

## Acceptance Criteria

- ACP/profile schema and fixtures exist.
- Profile claims cite allowed evidence references.
- Unsupported reputation, personhood, and standing claims are rejected.
- Review packet includes profile evidence and non-claims.

## Risks

- Profiles could become horoscope-like labels. Mitigation: require evidence
  links and explicit non-claims.
- Profiles could leak private state. Mitigation: require redaction policy and
  allowed projection boundaries.

## Future Work

Future milestones may connect ACP profiles to governance, reputation, and
cross-polis exchange after v0.93 rules exist.

## Notes

This feature is intentionally narrower than citizenship or reputation.

## Non-goals

- scalar moral verdicts
- reputation replacement
- public standing by profile inference alone
- unaudited private-state exposure

## Completion Target

`v0.92`

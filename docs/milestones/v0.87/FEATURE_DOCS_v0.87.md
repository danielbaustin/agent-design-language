# Feature Document Template

## Metadata
- Feature Name: `{{feature_name}}`
- Milestone Target: `{{milestone}}`
- Status: `{{status}}` (draft | planned | in-progress | complete)
- Owner: `{{owner}}`
- Doc Role: `{{doc_role}}` (primary | supporting)
- Supporting Docs: `{{supporting_docs}}` (list of related feature docs)
- Feature Types: `{{feature_types}}` (list: runtime | artifact | policy | architecture)
- Proof Modes: `{{proof_modes}}` (list: demo | tests | schema | replay | review)

## Template Rules

- Every section must be completed or explicitly marked `N/A` with a brief justification.
- Sections such as Execution Flow, Demo, Tests, or Schema Validation may be `N/A` for non-runtime or non-artifact features, but must state why.

## Purpose

Describe what this feature is, why it exists, and what problem it solves.

## Context

- Related milestone: `{{milestone}}`
- Related issues: `{{issues}}`
- Dependencies: `{{dependencies}}`

Explain how this feature fits into the broader ADL architecture and roadmap.

## Coverage / Ownership

Describe how this document participates in the coverage model.

- Primary owner doc: {{primary_owner_doc}}
- Covered surfaces:
  - {{surface_1}}
  - {{surface_2}}
- Related / supporting docs:
  - {{supporting_doc_1}}

## Overview

Provide a concise description of the feature’s behavior and responsibilities.

Key capabilities:
- {{capability_1}}
- {{capability_2}}
- {{capability_3}}

## Design

### Core Concepts

Describe the main concepts, abstractions, or entities introduced by this feature.

- {{concept_1}}
- {{concept_2}}

### Architecture

Explain how the feature is structured and how it integrates with existing systems.

- Inputs (explicit sources / triggers):
  - {{input_1}}
- Outputs (artifacts / side effects):
  - {{output_1}}
- Interfaces (APIs, CLI, files, schemas):
  - {{interface_1}}
- Invariants (must always hold):
  - {{invariant_1}}

### Data / Artifacts

Describe any artifacts, schemas, or persistent data structures.

- {{artifact_1}}
- {{artifact_2}}

## Execution Flow

If this is a runtime or artifact-bearing feature, describe the execution flow. If not, state "N/A" and explain why.

1. {{step_1}}
2. {{step_2}}
3. {{step_3}}

## Determinism and Constraints

- Determinism guarantees (what must be repeatable and how):
  - {{determinism_1}}
- Constraints (performance, ordering, limits):
  - {{constraint_1}}

## Integration Points

List integrations generically; include only what applies.

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| {{system_1}} | {{type}} (read/write/trigger/observe) | {{description}} |
| {{system_2}} | {{type}} | {{description}} |

Common systems (use if applicable): Gödel, AEE, ObsMem, Identity, Governance, Trace, Authoring, Providers.

## Validation

Describe all validation modes required for this feature. Not all features require demos.

### Demo (if applicable)
- Demo script(s): {{demo_script}}
- Expected behavior: {{expected_behavior}}

### Deterministic / Replay
- Replay requirements: {{replay_requirements}}
- Determinism guarantees: {{determinism_1}}

### Schema / Artifact Validation
- Schemas involved: {{schema_1}}
- Artifact checks: {{artifact_check_1}}

### Tests
- Test surfaces: {{test_surface_1}}

### Review / Proof Surface
- Review method (manual/automated): {{review_method}}
- Evidence location: {{proof_surface}}

## Acceptance Criteria

- Functional correctness (what must work): {{criteria_1}}
- Determinism / replay correctness: {{criteria_2}}
- Validation completeness (tests/schema/demo/review as applicable): {{criteria_3}}

## Risks

- Primary risks (failure modes): {{risk_1}}
- Mitigations: {{mitigation_1}}

## Future Work

- Follow-ups / extensions: {{future_1}}
- Known gaps / deferrals: {{future_2}}

## Notes

Additional notes, assumptions, or references.

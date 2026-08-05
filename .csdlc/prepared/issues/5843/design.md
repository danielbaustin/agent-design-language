# Issue 5843 Design: Documentation And Release Truth

Status: design-time ready; execution waits for a passing WP-22 gate.

## Authority And Sources

Issue #5843 and the WP-23 rows in the WBS and issue-wave YAML own final
documentation alignment after quality acceptance. Inputs include the complete
`docs/milestones/v0.92` package, `README.md`, `CHANGELOG.md`, active feature
lists, `docs/milestones/v0.92/ADR_PLAN_v0.92.md`, release notes, release plan,
skills, and root/nested agent guidance. Existing inconsistencies are evidence
to repair, not authority: for example, the checklist currently assigns release
evidence and final notes to WP-29 while the current WBS and live issues assign
the ceremony package to WP-30.

## Outcome Contract

Produce a docs-review packet and source-grounded release-truth diff that maps
every changed claim to landed WP-22-accepted evidence. Normalize current
status, issue numbers, WP ownership, commands, links, version language, and
planned-versus-landed distinctions across canonical surfaces. Create an ADR
candidate packet only when landed architecture introduced a durable decision
that is not already represented; do not manufacture ADRs to fill a quota.

Public-facing prose may describe only landed reviewed behavior. Birthday,
identity, provider, platform, governance, citizenship, consciousness, legal,
and v0.93 claims retain the milestone non-claim boundaries.

## Execution Sequence

1. Verify WP-22 passed at an ancestral exact revision and ingest its accepted
   matrix and blockers disposition.
2. Build a canonical document/claim inventory spanning root, milestone,
   feature, ADR, release, skill, and agent-guidance surfaces.
3. Classify each statement as current, stale, planned, blocked, unsupported, or
   historical; map current claims to exact accepted evidence.
4. Apply narrowly scoped documentation corrections and generate the review and
   ADR-candidate packets.
5. Validate links, Markdown, YAML/JSON, version/WP references, commands,
   claim-boundary language, and release-note evidence mapping.
6. Run bounded exact-head docs review and leave WP-24/WP-24A publication
   artifacts and WP-25 review execution to their owners.

## Owned Paths

- `.csdlc/evidence/5843`
- `.csdlc/prepared/issues/5843/validate-doc-release-truth.rb`
- `AGENTS.md`
- `CHANGELOG.md`
- `README.md`
- `REVIEW.md`
- `csdlc-v2/AGENTS.md`
- `csdlc-v2/operator/skills/csdlc-v2-bind/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-card-editor/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-clean/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-doctor/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-finish/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-github/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-publish/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-shepherd/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md`
- `docs/README.md`
- `docs/milestones/v0.92/ADR_PLAN_v0.92.md`
- `docs/milestones/v0.92/DECISIONS_v0.92.md`
- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`
- `docs/milestones/v0.92/DESIGN_v0.92.md`
- `docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md`
- `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`
- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/MILESTONE_CHECKLIST_v0.92.md`
- `docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md`
- `docs/milestones/v0.92/QUALITY_GATE_v0.92.md`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/RELEASE_NOTES_v0.92.md`
- `docs/milestones/v0.92/RELEASE_PLAN_v0.92.md`
- `docs/milestones/v0.92/SPRINT_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.92/V092_DOCS_PREP_DOGFOOD_NOTES.md`
- `docs/milestones/v0.92/VISION_v0.92.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md`
- `docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml`
- `docs/milestones/v0.92/external_launch/PUBLIC_LAUNCH_COPY_v0.92.md`
- `docs/milestones/v0.92/external_launch/README.md`
- `docs/milestones/v0.92/external_launch/REVIEWER_FAQ_AND_CLAIM_BOUNDARY_v0.92.md`
- `docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md`
- `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md`
- `docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md`
- `docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md`
- `docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md`
- `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md`
- `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`
- `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md`
- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`
- `docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md`
- `docs/milestones/v0.92/features/OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md`
- `docs/milestones/v0.92/features/PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md`
- `docs/milestones/v0.92/features/README.md`
- `docs/milestones/v0.92/features/RUNTIME_LAUNCH_AND_RESILIENCE_v0.92.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/reviews/v0.92/docs-release-truth-5843`
## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Serialization Gates

```json
[
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-docs-activation-final-truth-v1",
    "paths": [
      "README.md",
      "docs/README.md",
      "docs/planning/ADL_FEATURE_LIST.md",
      "AGENTS.md",
      "REVIEW.md"
    ],
    "issues": [
      5818,
      5843
    ],
    "order": [
      5818,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-quality-gate-to-doc-truth-v1",
    "paths": [
      "docs/milestones/v0.92/QUALITY_GATE_v0.92.md",
      "docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md"
    ],
    "issues": [
      5842,
      5843
    ],
    "order": [
      5842,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-doc-truth-to-handoff-v1",
    "paths": [
      "docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md"
    ],
    "issues": [
      5843,
      5849
    ],
    "order": [
      5843,
      5849
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-doc-truth-to-release-v1",
    "paths": [
      "docs/milestones/v0.92/RELEASE_NOTES_v0.92.md",
      "docs/milestones/v0.92/RELEASE_PLAN_v0.92.md",
      "docs/milestones/v0.92/MILESTONE_CHECKLIST_v0.92.md"
    ],
    "issues": [
      5843,
      5852
    ],
    "order": [
      5843,
      5852
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5818-skills-to-final-doc-truth-v1",
    "paths": [
      "csdlc-v2/operator/skills/csdlc-v2-bind/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-card-editor/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-clean/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-doctor/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-finish/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-github/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-publish/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-shepherd/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md"
    ],
    "issues": [
      5818,
      5843
    ],
    "order": [
      5818,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5825-birthday-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md"
    ],
    "issues": [
      5825,
      5843
    ],
    "order": [
      5825,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5826-identity-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md"
    ],
    "issues": [
      5826,
      5843
    ],
    "order": [
      5826,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5827-continuity-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md"
    ],
    "issues": [
      5827,
      5843
    ],
    "order": [
      5827,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5828-memory-palace-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md"
    ],
    "issues": [
      5828,
      5843
    ],
    "order": [
      5828,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5829-grounding-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md"
    ],
    "issues": [
      5829,
      5843
    ],
    "order": [
      5829,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5830-acp-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md"
    ],
    "issues": [
      5830,
      5843
    ],
    "order": [
      5830,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5831-adaptive-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md"
    ],
    "issues": [
      5831,
      5843
    ],
    "order": [
      5831,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5833-witness-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md"
    ],
    "issues": [
      5833,
      5843
    ],
    "order": [
      5833,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5834-demo-matrix-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/DEMO_MATRIX_v0.92.md"
    ],
    "issues": [
      5834,
      5843
    ],
    "order": [
      5834,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5835-migration-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md"
    ],
    "issues": [
      5835,
      5843
    ],
    "order": [
      5835,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5836-launch-docs-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/DEMO_MATRIX_v0.92.md",
      "docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md",
      "docs/milestones/v0.92/external_launch/PUBLIC_LAUNCH_COPY_v0.92.md",
      "docs/milestones/v0.92/external_launch/REVIEWER_FAQ_AND_CLAIM_BOUNDARY_v0.92.md"
    ],
    "issues": [
      5836,
      5843
    ],
    "order": [
      5836,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5838-provider-demo-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md"
    ],
    "issues": [
      5838,
      5843
    ],
    "order": [
      5838,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5839-adr-plan-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/ADR_PLAN_v0.92.md"
    ],
    "issues": [
      5839,
      5843
    ],
    "order": [
      5839,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5840-proof-docs-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/DEMO_MATRIX_v0.92.md",
      "docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md",
      "docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md"
    ],
    "issues": [
      5840,
      5843
    ],
    "order": [
      5840,
      5843
    ]
  }
]
```

## Validation And Failure Policy

Required lanes are canonical inventory completeness, executable docs/YAML/JSON
parsing, relative-link resolution and command checks, version/WP ownership consistency, accepted-
evidence link checks, stale/planned/unsupported claim rejection, secret/private
path scanning, and exact-head docs review. Any unresolved contradiction or
unsupported release claim blocks completion and remains visible.

## Rollback

Revert the final documentation-truth commit as one unit after all serialized producers are complete, preserving the canonical inventory and review packet. Restore the immediately prior reviewed documents only; do not partially roll back generated metadata, producer-owned content, release claims, or historical evidence.

## Non-Goals
- No product implementation, historical evidence rewrite, or release approval.
- No article/podcast publication, internal review execution, or remediation.
- No claim that v0.93 governance or legal personhood is implemented.

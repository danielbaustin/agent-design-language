# Structured Task Prompt

Template: 1.0.0

Issue: 5765

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Add one bounded planning-only reference for the migration plan to the v0.92 issue-wave authority.

## Deliverables

- Canonical v0.92 YAML scheduling reference
- Truthful Gate 0 prerequisite wording
- Focused YAML and diff validation

## Acceptance

1. The YAML references .adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md
2. The entry names destination organization, owners, six candidates, billing/security confirmation, and asksifu exclusion as unresolved Gate 0 prerequisites
3. No transfer or approval claim is introduced
4. A bounded review confirms the edit is planning-only

## Dependencies

- .adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml

## Inputs

- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- .adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md

## Non Goals

- Organization creation
- Billing or runner configuration
- Repository transfer
- Remote, Actions, Pages, DNS, secret, workflow, or permission changes

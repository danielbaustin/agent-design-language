#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VALIDATOR="$ROOT_DIR/swarm/tools/validate_structured_prompt.rb"
PROMPT_SPEC_LINT="$ROOT_DIR/swarm/tools/lint_prompt_spec.sh"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

cat >"$tmpdir/stp_valid.md" <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "SUB-ISSUE"
slug: "valid-structured-task-prompt"
title: "Valid STP"
labels:
  - "track:roadmap"
issue_number: 900
status: "draft"
action: "create"
depends_on:
  - "#886"
milestone_sprint: "Sprint 2"
required_outcome_type:
  - "combination"
repo_inputs:
  - "docs/tooling/prompt-spec.md"
canonical_files:
  - ".adl/issues/v0.85/bodies/valid-structured-task-prompt.md"
demo_required: false
demo_names: []
issue_graph_notes:
  - "Test note"
pr_start:
  enabled: true
  slug: "valid-structured-task-prompt"
---

# Issue Card

## Summary
x
## Goal
x
## Required Outcome
x
## Deliverables
x
## Acceptance Criteria
x
## Repo Inputs
x
## Dependencies
x
## Demo Expectations
x
## Non-goals
x
## Issue-Graph Notes
x
## Notes
x
## Tooling Notes
x
EOF

cat >"$tmpdir/sip_valid.md" <<'EOF'
# ADL Input Card

Task ID: issue-0898
Run ID: issue-0898
Version: v0.85
Title: valid-sip
Branch: codex/898-valid-sip

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/898
- PR:
- Source Issue Prompt:
- Docs:
- Other:

## Prompt Spec
```yaml
prompt_schema: adl.v1
actor:
  role: execution_agent
  name: codex
model:
  id: gpt-5-codex
  determinism_mode: stable
inputs:
  sections:
    - goal
    - required_outcome
    - acceptance_criteria
    - inputs
    - target_files_surfaces
    - validation_plan
    - demo_proof_requirements
    - constraints_policies
    - system_invariants
    - reviewer_checklist
    - non_goals_out_of_scope
    - notes_risks
    - instructions_to_agent
outputs:
  output_card: .adl/cards/898/output_898.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: true
  disallow_secrets: true
  disallow_absolute_host_paths: true
automation_hints:
  source_issue_prompt_required: true
  target_files_surfaces_recommended: true
  validation_plan_required: true
  required_outcome_type_supported: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
  - card_reviewer_gpt.v1.1
```

Execution:
- Agent:
- Provider:
- Tools allowed:
- Sandbox / approvals:
- Source issue-prompt slug:
- Required outcome type:
- Demo required:

## Goal
x
## Required Outcome
x
## Acceptance Criteria
x
## Inputs
x
## Target Files / Surfaces
x
## Validation Plan
x
## Demo / Proof Requirements
x
## Constraints / Policies
x
## System Invariants (must remain true)
x
## Reviewer Checklist (machine-readable hints)
x
## Non-goals / Out of scope
x
## Notes / Risks
x
## Instructions to the Agent
x
EOF

cat >"$tmpdir/sor_valid.md" <<'EOF'
# ADL Output Card

Task ID: issue-0898
Run ID: issue-0898
Version: v0.85
Title: valid-sor
Branch: codex/898-valid-sor
Status: NOT_STARTED

Execution:
- Actor:
- Model:
- Provider:
- Start Time:
- End Time:

## Summary
x
## Artifacts produced
x
## Actions taken
x
## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
- Worktree-only paths remaining:
- Integration state:
- Verification scope:
- Integration method used:
- Verification performed:
- Result:
## Validation
x
## Verification Summary
x
## Determinism Evidence
x
## Security / Privacy Checks
x
## Replay Artifacts
x
## Artifact Verification
x
## Decisions / Deviations
x
## Follow-ups / Deferred work
x
EOF

"$VALIDATOR" --type stp --input "$tmpdir/stp_valid.md"
"$VALIDATOR" --type sip --phase bootstrap --input "$tmpdir/sip_valid.md"
"$VALIDATOR" --type sor --phase bootstrap --input "$tmpdir/sor_valid.md"

cat >"$tmpdir/sor_invalid.md" <<'EOF'
# ADL Output Card

Task ID: issue-0898
Run ID: issue-0898
Version: v0.85
Title: invalid-sor
Branch: codex/898-valid-sor
Status: MAYBE

Execution:
- Actor:
- Model:
- Provider:
- Start Time: 2026-03-17
- End Time:

## Summary
x
## Artifacts produced
x
## Actions taken
x
## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
- Worktree-only paths remaining:
- Integration state: maybe
- Verification scope:
- Integration method used:
- Verification performed:
- Result:
## Validation
x
## Verification Summary
x
## Determinism Evidence
x
## Security / Privacy Checks
x
## Replay Artifacts
x
## Artifact Verification
x
## Decisions / Deviations
x
## Follow-ups / Deferred work
x
EOF

set +e
"$VALIDATOR" --type sor --phase bootstrap --input "$tmpdir/sor_invalid.md" >/dev/null 2>&1
rc=$?
set -e
if [[ "$rc" -eq 0 ]]; then
  echo "assertion failed: invalid SOR unexpectedly passed validation" >&2
  exit 1
fi

echo "ok"

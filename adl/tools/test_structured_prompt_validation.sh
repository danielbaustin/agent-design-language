#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VALIDATOR="$ROOT_DIR/adl/tools/validate_structured_prompt.sh"
PROMPT_SPEC_LINT="$ROOT_DIR/adl/tools/lint_prompt_spec.sh"

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
- Source Issue Prompt: .adl/issues/v0.85/bodies/issue-0898-v085-valid-sip.md
- Docs: docs/tooling/prompt-spec.md
- Other: none

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
# valid-sor

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

cat >"$tmpdir/stp_missing_multiple_sections.md" <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "SUB-ISSUE"
slug: "invalid-structured-task-prompt"
title: "Invalid STP"
labels:
  - "track:roadmap"
issue_number: 901
status: "draft"
action: "create"
depends_on: []
milestone_sprint: "Sprint 2"
required_outcome_type:
  - "code"
repo_inputs:
  - "docs/tooling/prompt-spec.md"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes: []
pr_start:
  enabled: false
  slug: "invalid-structured-task-prompt"
---

# Issue Card

## Goal
x
EOF

if "$VALIDATOR" --type stp --input "$tmpdir/stp_missing_multiple_sections.md" >"$tmpdir/stp_missing_multiple_sections.out" 2>&1; then
  echo "expected multi-section-missing STP to fail validation" >&2
  exit 1
fi
grep -Fq "missing required sections: Summary, Required Outcome, Deliverables, Acceptance Criteria, Repo Inputs, Dependencies, Demo Expectations, Non-goals, Issue-Graph Notes, Notes, Tooling Notes" "$tmpdir/stp_missing_multiple_sections.out"

cp "$tmpdir/sip_valid.md" "$tmpdir/sip_dot_version_valid.md"
perl -0pi -e 's/Version: v0\.85/Version: v0.87.1/' "$tmpdir/sip_dot_version_valid.md"
"$VALIDATOR" --type sip --phase bootstrap --input "$tmpdir/sip_dot_version_valid.md"

cp "$tmpdir/sor_valid.md" "$tmpdir/sor_dot_version_valid.md"
perl -0pi -e 's/Version: v0\.85/Version: v0.87.1/' "$tmpdir/sor_dot_version_valid.md"
"$VALIDATOR" --type sor --phase bootstrap --input "$tmpdir/sor_dot_version_valid.md"

cat >"$tmpdir/sip_blank_context_invalid.md" <<'EOF'
# ADL Input Card

Task ID: issue-0898
Run ID: issue-0898
Version: v0.85
Title: invalid-blank-context
Branch: codex/898-invalid-blank-context

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

if "$VALIDATOR" --type sip --phase bootstrap --input "$tmpdir/sip_blank_context_invalid.md" >/dev/null 2>&1; then
  echo "expected blank-context SIP to fail validation" >&2
  exit 1
fi

cat >"$tmpdir/sip_bootstrap_not_bound_yet_valid.md" <<'EOF'
# ADL Input Card

Task ID: issue-1431
Run ID: issue-1431
Version: v0.87.1
Title: bootstrap-pre-run-sip
Branch: not bound yet

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/1431
- PR:
- Source Issue Prompt: .adl/v0.87.1/bodies/issue-1431-bootstrap-pre-run-sip.md
- Docs: docs/tooling/prompt-spec.md
- Other: none

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
  output_card: .adl/cards/1431/output_1431.md
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

"$VALIDATOR" --type sip --phase bootstrap --input "$tmpdir/sip_bootstrap_not_bound_yet_valid.md"

set +e
"$VALIDATOR" --type sip --input "$tmpdir/sip_bootstrap_not_bound_yet_valid.md" >/dev/null 2>&1
rc=$?
set -e
if [[ "$rc" -eq 0 ]]; then
  echo "expected bootstrap-style SIP without --phase to fail validation" >&2
  exit 1
fi

cat >"$tmpdir/stp_absolute_path_invalid.md" <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "SUB-ISSUE"
slug: "invalid-absolute-path-stp"
title: "Invalid absolute path STP"
labels:
  - "track:roadmap"
issue_number: 901
status: "draft"
action: "create"
depends_on:
  - "#886"
milestone_sprint: "Sprint 2"
required_outcome_type:
  - "docs"
repo_inputs:
  - "docs/tooling/prompt-spec.md"
canonical_files:
  - ".adl/issues/v0.85/bodies/invalid-absolute-path-stp.md"
demo_required: false
demo_names: []
issue_graph_notes:
  - "Test note"
pr_start:
  enabled: true
  slug: "invalid-absolute-path-stp"
---

# Invalid absolute path STP

## Summary
x
## Goal
x
## Required Outcome
x
## Deliverables
mentions /Users/example
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

set +e
"$VALIDATOR" --type stp --input "$tmpdir/stp_absolute_path_invalid.md" >/dev/null 2>&1
rc=$?
set -e
if [[ "$rc" -eq 0 ]]; then
  echo "assertion failed: STP with absolute host path unexpectedly passed validation" >&2
  exit 1
fi

cat >"$tmpdir/sor_completed_invalid_status.md" <<'EOF'
# dependable-execution-fixture

Task ID: issue-0898
Run ID: issue-0898
Version: v0.85
Title: invalid-completed-sor
Branch: codex/898-valid-sor
Status: NOT_STARTED

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: local workspace
- Start Time: 2026-03-18T07:00:00Z
- End Time: 2026-03-18T07:05:00Z

## Summary
Completed-looking output card with stale status.
## Artifacts produced
- docs/tooling/examples/workflow-state/bad_output_record.md
## Actions taken
- Recorded a stale status by mistake.
## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
  - docs/tooling/examples/workflow-state/bad_output_record.md
- Worktree-only paths remaining:
  - docs/tooling/examples/workflow-state/bad_output_record.md
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: committed on issue branch
- Verification performed:
  - `git diff -- docs/tooling/examples/workflow-state/bad_output_record.md`
- Result: PASS
## Validation
- Validation commands and their purpose:
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input docs/tooling/examples/workflow-state/bad_output_record.md`
- Results:
  - fail expected
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

cat >"$tmpdir/sor_invalid.md" <<'EOF'
# dependable-execution-fixture

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

cat >"$tmpdir/sor_offset_timestamp_invalid.md" <<'EOF'
# dependable-execution-fixture

Task ID: issue-0898
Run ID: issue-0898
Version: v0.85
Title: invalid-offset-sor
Branch: codex/898-valid-sor
Status: IN_PROGRESS

Execution:
- Actor:
- Model:
- Provider:
- Start Time: 2026-03-18T07:00:00+00:00
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
- Integration state: worktree_only
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
offset_err="$("$VALIDATOR" --type sor --phase bootstrap --input "$tmpdir/sor_offset_timestamp_invalid.md" 2>&1)"
rc=$?
set -e
if [[ "$rc" -eq 0 ]]; then
  echo "assertion failed: SOR with offset timestamp unexpectedly passed validation" >&2
  exit 1
fi
printf '%s' "$offset_err" | grep -Fq "UTC ISO 8601 / RFC3339 with trailing Z" || {
  echo "assertion failed: expected UTC Z guidance in timestamp validation error" >&2
  exit 1
}

set +e
"$VALIDATOR" --type sor --phase completed --input "$tmpdir/sor_completed_invalid_status.md" >/dev/null 2>&1
rc=$?
set -e
if [[ "$rc" -eq 0 ]]; then
  echo "assertion failed: completed-phase SOR with NOT_STARTED unexpectedly passed validation" >&2
  exit 1
fi

cat >"$tmpdir/sor_completed_worktree_only_valid.md" <<'EOF'
# dependable-execution-fixture

Task ID: issue-0898
Run ID: issue-0898
Version: v0.85
Title: completed-worktree-only-valid-sor
Branch: codex/898-valid-sor
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: codex desktop
- Start Time: 2026-03-18T07:00:00Z
- End Time: 2026-03-18T07:05:00Z

## Summary
Completed pre-PR output record for finish validation.
## Artifacts produced
x
## Actions taken
x
## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
- Worktree-only paths remaining: docs/tooling/example.md
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: branch pending pr finish
- Verification performed:
- Result: PASS
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

"$VALIDATOR" --type sor --phase completed --input "$tmpdir/sor_completed_worktree_only_valid.md" >/dev/null

cat >"$tmpdir/sor_completed_closed_no_pr_valid.md" <<'EOF'
# dependable-execution-fixture

Task ID: issue-0899
Run ID: issue-0899
Version: v0.85
Title: completed-closed-no-pr-valid-sor
Branch: retrospective-no-branch
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: codex desktop
- Start Time: 2026-03-18T07:00:00Z
- End Time: 2026-03-18T07:05:00Z

## Summary
Completed no-PR closeout record for truthful retrospective normalization.
## Artifacts produced
x
## Actions taken
x
## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
- Worktree-only paths remaining: none
- Integration state: closed_no_pr
- Verification scope: main_repo
- Integration method used: direct write in main repo for local-only closeout surfaces
- Verification performed:
- Result: PASS
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

"$VALIDATOR" --type sor --phase completed --input "$tmpdir/sor_completed_closed_no_pr_valid.md" >/dev/null

cat >"$tmpdir/sor_absolute_path_invalid.md" <<'EOF'
# dependable-execution-fixture

Task ID: issue-0898
Run ID: issue-0898
Version: v0.85
Title: invalid-absolute-path-sor
Branch: codex/898-valid-sor
Status: IN_PROGRESS

Execution:
- Actor:
- Model:
- Provider:
- Start Time:
- End Time:

## Summary
contains /Users/example leak
## Artifacts produced
x
## Actions taken
x
## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
- Worktree-only paths remaining:
- Integration state: worktree_only
- Verification scope: worktree
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
"$VALIDATOR" --type sor --phase bootstrap --input "$tmpdir/sor_absolute_path_invalid.md" >/dev/null 2>&1
rc=$?
set -e
if [[ "$rc" -eq 0 ]]; then
  echo "assertion failed: SOR with absolute host path unexpectedly passed validation" >&2
  exit 1
fi

echo "ok"

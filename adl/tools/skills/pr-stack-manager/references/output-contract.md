# PR Stack Manager Output Contract

Use this contract when the skill emits a machine-readable artifact.

```yaml
status: clean | findings | blocked | error
target:
  issue_number: <u32 or null>
  task_bundle_path: <repo-relative-or-absolute path or null>
  branch: <branch name or null>
  worktree_path: <repo-relative-or-absolute path or null>
  slug: <string or null>
  version: <string or null>
dependency_graph:
  root_issue: <issue number or null>
  nodes:
    - issue_number: <u32 or null>
      slug: <string or null>
      branch: <string or null>
      pr_number: <u32 or null>
      state: <open|closed|unknown>
      depends_on: [<issue numbers>]
      base_ref: <branch or null>
  edges:
    - from_issue: <u32>
      to_issue: <u32>
      edge_type: requires | blocks | base_rebase_candidate
      confidence: low | medium | high
  cycle_detected: <true|false>
findings:
  - severity: info | warning | blocking | ambiguous
    area: stack_topology | base_alignment | dependency_order | artifact_drift
    message: <short finding text>
    evidence:
      summary: <short evidence summary>
      files:
        - <path>
      refs:
        - <issue, branch, or PR ref>
    can_auto_fix: <true|false>
planned_actions:
  - type: plan | mutate
    issue_number: <u32>
    action: <action name>
    command: <bounded command string or null>
    rationale: <why this action is proposed>
    safe: <true|false>
    preconditions:
      - <required condition>
skip_list:
  - <path>
recommended_follow_ons:
  - title: <follow-on issue candidate title>
    rationale: <reason this should be handled separately>
validation_performed:
  - <validation command or check name>
handoff_state:
  ready_for_editor: <true|false>
  ready_for_execution: <true|false>
  ready_for_follow_on_implementation: <true|false>
```

## Rules

- `status: clean` indicates no actionable `warning` or `blocking` findings.
- `status: findings` means at least one actionable finding is present.
- `status: blocked` means the stack cannot be safely assessed or repaired without
  operator direction.
- `status: error` means the skill could not complete analysis for safety reasons.
- `planned_actions` entries with `type: mutate` must set `safe: true` only when
  bounded, deterministic, and explicitly requested by policy.
- `command` should be null for plan-only output.
- If policy is dry-run or plan mode, mutation actions should remain explicit but not executed.
- Do not include absolute host paths.

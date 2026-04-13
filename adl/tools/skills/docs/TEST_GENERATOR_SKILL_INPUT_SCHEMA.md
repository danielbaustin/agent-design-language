# Test Generator Skill Input Schema

```yaml
skill_input_schema: test_generator.v1
mode: generate_for_issue | generate_for_diff | generate_for_path | generate_for_worktree
repo_root: /absolute/path/to/repo
target:
  issue_number: <u32 optional, required for generate_for_issue>
  diff_base: <git ref optional, required for generate_for_diff>
  target_path: <repo-relative or absolute path optional, required for generate_for_path>
  worktree_path: <absolute path optional, required for generate_for_worktree>
  changed_paths:
    - <path optional>
  target_behavior: <string optional>
  acceptance_surface: <string optional>
  test_framework: <string optional>
policy:
  test_depth: focused | moderate
  allow_new_test_files: true | false
  allow_fixture_updates: true | false
  validation_mode: targeted | dry_run | none
  stop_after_generation: true
```

## Purpose

Use this schema when one bounded invocation should generate or update focused tests for one concrete target surface.

The skill should:
- inspect the target implementation and nearby tests
- choose the smallest meaningful test shape
- write only the bounded test surface
- record what was validated
- stop without taking over broader implementation or PR orchestration

## Supported Modes

- `generate_for_issue`
- `generate_for_diff`
- `generate_for_path`
- `generate_for_worktree`

## Required Top-Level Fields

- `skill_input_schema`
- `mode`
- `repo_root`
- `target`
- `policy`

## Mode Requirements

- `generate_for_issue`
  - requires `target.issue_number`
- `generate_for_diff`
  - requires `target.diff_base`
- `generate_for_path`
  - requires `target.target_path`
- `generate_for_worktree`
  - requires `target.worktree_path`

## Policy Requirements

- `policy.test_depth` must be explicit
- `policy.validation_mode` must be explicit
- `policy.stop_after_generation` must be `true`

## Example Invocation

```yaml
Use $test-generator at /Users/daniel/git/agent-design-language/adl/tools/skills/test-generator/SKILL.md with this validated input:

skill_input_schema: test_generator.v1
mode: generate_for_issue
repo_root: /Users/daniel/git/agent-design-language
target:
  issue_number: 1769
  changed_paths:
    - adl/src/provider.rs
  target_behavior: split-provider-refactor-keeps-provider-tests-bounded
policy:
  test_depth: focused
  allow_new_test_files: true
  allow_fixture_updates: true
  validation_mode: targeted
  stop_after_generation: true
```

## Stop Boundary

The skill must stop after:
- bounded test generation
- minimal validation reporting
- artifact or summary emission

It must not:
- widen into unrelated feature work
- auto-finish the PR
- janitor CI
- claim repo-wide coverage

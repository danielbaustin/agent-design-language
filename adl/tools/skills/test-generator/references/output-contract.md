# Output Contract

The default `test-generator` artifact is markdown with these sections in this order:

```md
## Metadata
- Skill: test-generator
- Subject: <issue, path, diff, or worktree target>
- Date: <UTC timestamp or calendar date>
- Output Location: <path or none>

## Target
- Mode: generate_for_issue | generate_for_diff | generate_for_path | generate_for_worktree
- Target Surface: <concrete target>
- Intended Behavior: <what behavior the tests cover>

## Test Plan
- Existing Tests Read: <paths or explicit none>
- Strategy: extend_existing | add_new_file | fixture_update
- Test Scope: <bounded scope>

## Changes Made
- Files Changed:
  - <path>
- Coverage Added:
  - <behavior or regression>

## Validation Performed
- Commands:
  - <command and what it verified>
- Result: PASS | FAIL | PARTIAL | NOT_RUN

## Residual Risk
- <remaining uncovered gap or explicit none>
```

## Rules

- Do not claim validation that was not run.
- Keep the target concrete; if it was ambiguous, say so in `Residual Risk`.
- Do not overstate coverage beyond the tests actually added or updated.
- Do not emit absolute host paths, secrets, raw prompts, or raw tool arguments.

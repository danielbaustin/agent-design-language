# Output Contract

The default `demo-operator` artifact is markdown with these sections in this order:

```md
## Metadata
- Skill: demo-operator
- Subject: <named demo, command, or demo doc target>
- Date: <UTC timestamp or calendar date>
- Output Location: <path or none>

## Target
- Mode: operate_named_demo | operate_demo_command | operate_demo_doc
- Demo Target: <concrete target>
- Intended Proof Surface: <what the demo was meant to prove>

## Prerequisites
- Demo Entry Surface: <command/doc/path>
- Provider or Credential Requirements: <explicit requirement or none>
- Gate Status: pass | gated | missing_prereq

## Execution
- Command Run:
  - <command and what it attempted>
- Produced Artifacts:
  - <path or explicit none>
- Result: PASS | FAIL | PARTIAL | NOT_RUN

## Classification
- Outcome: proving | non_proving | skipped | failed
- Reason: <one concise explanation>

## Follow-up
- Recommended Next Step: <one bounded next action or explicit none>
```

## Rules

- Do not call a demo `proving` unless the expected proof surface actually exists.
- Use `skipped` only for an intentional, evidenced gate or allowed missing prerequisite.
- Use `failed` when the demo should have run but the command or output surface broke.
- Do not emit raw secrets, raw prompts, raw tool arguments, or unjustified absolute host paths.

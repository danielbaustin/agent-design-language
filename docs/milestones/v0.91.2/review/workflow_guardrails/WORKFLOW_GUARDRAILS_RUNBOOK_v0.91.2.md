# Workflow Guardrails Runbook - v0.91.2

## Purpose

Provide one bounded operator-facing command surface for the workflow failures
that slowed recent milestone work:

- tracked writes from `main`
- closed/completed issues that still need local closeout
- unsafe shell command shapes used to generate Markdown reports
- issue-card drift that should be caught before or during execution

## Command Surface

The guardrail entrypoint is:

```bash
bash adl/tools/workflow_guardrails.sh <subcommand> ...
```

### `main-write`

```bash
bash adl/tools/workflow_guardrails.sh main-write --repo .
```

Use this before issue execution if you suspect the primary checkout has tracked
drift on `main` or `master`.

Fail-closed behavior:

- passes on non-`main` branches
- passes on clean `main`
- blocks on tracked changes on `main`/`master`
- ignores local ignored `.adl` notes

### `closeout-watch`

```bash
bash adl/tools/workflow_guardrails.sh closeout-watch \
  --version v0.91.2 \
  --repo danielbaustin/agent-design-language
```

Use this when sprint or milestone state looks healthy on GitHub but local
closeout may still be pending.

Fail-closed behavior:

- generates a dry-run closeout-wave report
- blocks when closed/completed issues still need local closeout
- passes when no local closeout candidates remain

### `safe-report-command`

```bash
bash adl/tools/workflow_guardrails.sh safe-report-command \
  --file ./report-command.sh
```

Use this before adopting or sharing a shell command that writes Markdown report
artifacts.

Fail-closed behavior:

- blocks shell command strings that use command substitution via `` `...` `` or
  `$(...)`
- recommends safer alternatives:
  - quoted heredocs such as `<<'EOF'`
  - language-native file writers

This guardrail is intentionally about command shape, not Markdown content. It
does not forbid normal Markdown backticks inside the generated report itself.

### `card-drift`

```bash
bash adl/tools/workflow_guardrails.sh card-drift \
  --issue 3015 \
  --version v0.91.2 \
  --slug v0-91-2-wp-16-workflow-guardrails-hardening
```

Use this to route issue-card drift through the normal `pr doctor` path instead
of improvising local guesses about lifecycle state.

## Validation

Focused proof command:

```bash
bash adl/tools/test_workflow_guardrails.sh
```

## Non-Claims

- These guardrails do not eliminate all operator error.
- `closeout-watch` does not replace review or closeout; it surfaces when
  closeout is still pending.
- `safe-report-command` does not make arbitrary shell snippets safe; it blocks
  obvious unsafe command-substitution shapes and points operators toward safer
  patterns.

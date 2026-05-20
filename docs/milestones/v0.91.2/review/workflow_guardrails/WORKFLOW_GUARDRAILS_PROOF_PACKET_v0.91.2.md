# Workflow Guardrails Proof Packet - v0.91.2

## Scope

`WP-16` hardens one bounded workflow command surface around four concrete
failure families:

- dirty tracked writes from `main`
- stalled local closeout after GitHub-side completion
- unsafe shell command shapes used for Markdown report generation
- issue-card drift that should route through `pr doctor`

## Delivered Surface

- `adl/tools/workflow_guardrails.sh`
- `adl/tools/test_workflow_guardrails.sh`
- this review packet and runbook

## Proof Route

Primary proof command:

```bash
bash adl/tools/test_workflow_guardrails.sh
```

Covered failure behavior:

- dirty tracked `main` checkout blocks
- pending closeout-wave candidates block
- unsafe report-generation command strings block
- safe quoted-heredoc Markdown code fences pass without being mistaken for
  executable command substitution
- card-drift wrapper delegates to `pr doctor`

## Design Notes

- The tool is intentionally operator-facing and bounded.
- Existing Rust PR lifecycle guardrails remain authoritative where they already
  exist.
- `WP-16` adds a clear diagnosis/handoff surface instead of widening into a
  full lifecycle redesign late in Sprint 3.

## Residual Risk

- `closeout-watch` still depends on `gh` availability and truthful GitHub
  state.
- `safe-report-command` is a command-shape guardrail, not a complete shell
  sandbox; it ignores literal quoted-heredoc bodies so Markdown fences and
  explanatory command text can be written safely.
- Card drift still requires the underlying doctor/editor lifecycle to repair
  the issue; this tool only routes and surfaces the condition.

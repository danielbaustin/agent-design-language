# v0.91 Card Bundle Readiness

## Purpose

This record captures the v0.91 issue-card readiness pass performed when the
official issue wave opened.

Live ADL issue records intentionally remain in local `.adl/` bundles. The repo
guardrail documented in `docs/records/README.md` keeps new tracked `.adl`
issue-record residue out of the repository, so this tracked file records the
reviewable readiness evidence instead of publishing the local STP/SIP/SOR
files themselves.

## Scope

- Issue wave: #2735 through #2759.
- Local source prompts: `.adl/v0.91/bodies/issue-*.md`.
- Local task bundles: `.adl/v0.91/tasks/issue-*__*/stp.md`,
  `.adl/v0.91/tasks/issue-*__*/sip.md`, and
  `.adl/v0.91/tasks/issue-*__*/sor.md`.
- Card count: 25 source prompts and 75 task-bundle cards.

## Completion State

The v0.91 source prompts and task-bundle cards have been normalized for
pre-execution readiness:

- STP frontmatter carries concrete issue numbers, sprint placement,
  dependencies, repo inputs, canonical files, demo flags, and demo names where
  applicable.
- SIPs carry concrete execution inputs, target surfaces, validation guidance,
  demo/proof requirements, non-goals, dependency notes, and supported required
  outcome types.
- SORs are truthful pre-run output scaffolds: they do not claim implementation
  work, proof execution, PR publication, merge, or release completion before an
  issue is actually executed.
- WP-17 and WP-18 are marked as demo-required with explicit demo/proof names.
- No issue body or card should depend on pending issue numbers, empty
  dependency/input/canonical-file fields, or placeholder sprint assignment.

## Validation

Commands run from the v0.91 launch worktree:

```bash
for t in stp sip sor; do
  for f in .adl/v0.91/tasks/issue-*/$t.md; do
    bash adl/tools/validate_structured_prompt.sh --type $t --phase bootstrap --input "$f" >/dev/null
  done
done
```

Result: PASS for all 75 STP/SIP/SOR files.

Additional checks:

```bash
rg -n 'Required outcome type: (runtime|tools|quality|review|release)|depends_on: \[\]|repo_inputs: \[\]|canonical_files: \[\]|Pending sprint assignment|issue: pending|PLACEHOLDER|placeholder|pr\.sh|/Users/daniel|/private/var|/private/tmp|/home/runner' .adl/v0.91/tasks .adl/v0.91/bodies
```

Result: no readiness-blocking matches. The only matches from broader scans were
template-rule text inside SOR files that warns against placeholders; those are
instructional guardrails, not unresolved placeholders.

## Non-Claims

- This readiness pass does not mark any v0.91 feature issue as implemented.
- This readiness pass does not bind per-issue branches or worktrees.
- This readiness pass does not replace per-issue execution, validation, PR
  publication, or closeout.
- This readiness pass does not publish local `.adl/` issue records as tracked
  repository artifacts.


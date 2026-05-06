# v0.91 Card Bundle Readiness

## Purpose

This record captures the v0.91 issue-card readiness pass performed when the
official issue wave opened, plus the #2769 repair passes that restored and
normalized local card bundles in the primary checkout.

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
- Card count after #2769 repair: 25 core WP source prompts and 75 core WP
  task-bundle cards.

## Completion State

The v0.91 source prompts and task-bundle cards are present and normalized for
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
- Pre-run SORs for unbound core WPs are marked `NOT_STARTED`, not
  `IN_PROGRESS`.
- STP canonical-file lists use issue-local `.adl/v0.91/bodies/issue-*.md` and
  `.adl/v0.91/tasks/issue-*__*/` paths rather than template placeholders.
- SOR provenance is normalized to the Rust ADL lifecycle and conductor/editor
  workflow, not the legacy shell helper.
- WP-17 and WP-18 are marked as demo-required with explicit demo/proof names.
- No core WP issue body or card should depend on pending issue numbers or
  placeholder sprint assignment.
- The WP-04 sample SPP now cites the bootstrapped local STP/SIP bundle instead
  of saying those cards were absent.

The #2769 repair specifically fixed the local lifecycle-state gap where the
tracked milestone issue wave existed but most `.adl/v0.91/tasks/issue-*`
bundles were absent from the primary checkout. The repair used the Rust ADL
`pr init` lifecycle path, followed by editor-style normalization of card truth.

## Validation

Commands run during the readiness and repair passes:

```bash
for t in stp sip sor; do
  for f in .adl/v0.91/tasks/issue-*/$t.md; do
    bash adl/tools/validate_structured_prompt.sh --type $t --phase bootstrap --input "$f" >/dev/null
  done
done
```

Result: PASS for all 75 STP/SIP/SOR files.

The #2769 aggregate repair check also produced:

```text
PASS: all 25 core WP bundles have stp/sip/sor
PASS: validated 75 cards
```

Additional checks:

```bash
rg -n '<issue-source-prompt>|<issue-task-bundle>|adl/tools/pr\.sh|Status: IN_PROGRESS|No local STP or SIP bundle was cited|Pending rerun after editor normalization' .adl/v0.91/tasks/issue-27*__* .adl/v0.91/tasks/issue-2769__*
```

Result: no matches after the card-truth normalization pass.

At the time of the original readiness pass, `SPP` still required a separate
manual check because the structured-prompt validator did not yet expose `spp`
or `srp` types. WP-15 is the feature slice that closes that validator gap.

## Non-Claims

- This readiness pass does not mark any v0.91 feature issue as implemented.
- This readiness pass does not bind per-issue branches or worktrees.
- This readiness pass does not replace per-issue execution, validation, PR
  publication, or closeout.
- This readiness pass does not publish local `.adl/` issue records as tracked
  repository artifacts.

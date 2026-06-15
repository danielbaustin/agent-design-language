# #3775 Octocrab Templates AST And Pruning Proof

Status: proof_captured_for_pr
Issue: #3775
Milestone: v0.91.5

## Purpose

This packet proves the current workflow adoption seam after the related fixes in #3772 and #3774:

- GitHub workflow readiness uses the octocrab-backed path when configured.
- General Markdown section edits can use `markdown-ast-edit`.
- Lifecycle-card paths fail closed under `markdown-ast-edit` and point operators back to prompt-template/card-editor authority.
- Rendered lifecycle cards can be edited through the prompt-template values round trip.
- Completed follow-up worktrees are pruned instead of accumulating stale execution surfaces.

## Commands Run

### Octocrab doctor path

```sh
GITHUB_TOKEN=REDACTED_TOKEN \
  ADL_GITHUB_CLIENT=octocrab \
  ADL_GITHUB_DISABLE_GH_FALLBACK=1 \
  bash adl/tools/pr.sh doctor 3775 \
    --version v0.91.5 \
    --slug v0-91-5-tools-workflow-prove-octocrab-templates-ast-and-pruning-together
```

Observed evidence:

```text
adl_event ... stage=github_octocrab result=started operation=pr.list.open_wave
adl_event ... stage=github_octocrab result=completed operation=pr.list.open_wave
OPEN_PR_COUNT=0
READY=PASS
DOCTOR_STATUS=PASS
```

### General Markdown AST edit

```sh
cargo run --manifest-path adl/Cargo.toml -- \
  tooling markdown-ast-edit replace-section \
  --input .tmp/3775-proof/source.md \
  --heading Summary \
  --replacement .tmp/3775-proof/replacement.md \
  --out .tmp/3775-proof/edited.md
```

Observed result: command completed successfully and produced the edited Markdown document.

### Lifecycle-card AST guardrail

```sh
cargo run --manifest-path adl/Cargo.toml -- \
  tooling markdown-ast-edit replace-section \
  --input .adl/v0.91.5/tasks/issue-3775__v0-91-5-tools-workflow-prove-octocrab-templates-ast-and-pruning-together/spp.md \
  --heading Notes \
  --replacement .tmp/3775-proof/replacement.md \
  --out .tmp/3775-proof/illegal-spp.md \
  --repair-note-out .tmp/3775-proof/repair-note.md
```

Observed fail-closed output:

```text
Error: markdown AST edit failed closed: lifecycle-card input or output path requires prompt-template/card-editor authority; markdown-ast-edit did not mutate it
```

### Prompt-template rendered-card edit path

```sh
cargo run --manifest-path adl/Cargo.toml -- \
  tooling prompt-template write-sample-values \
  --out-dir .tmp/3775-proof/values

cargo run --manifest-path adl/Cargo.toml -- \
  tooling prompt-template render \
  --kind spp \
  --values .tmp/3775-proof/values/spp.values.yaml \
  --out .tmp/3775-proof/spp.md

cargo run --manifest-path adl/Cargo.toml -- \
  tooling prompt-template edit-rendered \
  --kind spp \
  --input .tmp/3775-proof/spp.md \
  --set card_status=approved \
  --set status=approved \
  --set activation_state=approved \
  --values-out .tmp/3775-proof/spp-edited.values.yaml \
  --out .tmp/3775-proof/spp-edited.md

cargo run --manifest-path adl/Cargo.toml -- \
  tooling prompt-template validate-structure \
  --kind spp \
  --input .tmp/3775-proof/spp-edited.md
```

Observed evidence:

```text
PASS: edited rendered spp card to .tmp/3775-proof/spp-edited.md via values .tmp/3775-proof/spp-edited.values.yaml (round_trip=exact)
PASS: rendered structure valid for spp
activation_state: "approved"
card_status: "approved"
status: "approved"
```

### Worktree pruning check

```sh
git worktree list | rg '3772|3774|adl-wp-3772|adl-wp-3774' || true
```

Observed evidence: no output. The already-merged `#3772` and `#3774` follow-up worktrees were not present.

The `#3775` worktree is expected to remain until this issue is merged and closed out; its own prune proof belongs to the closeout step.

## Non-Claims

- This does not prove every historical shell-only GitHub path has been removed.
- This does not claim `gh` is absent from all scripts.
- This does not replace CI or release validation.
- This does not prove arbitrary lifecycle-card edits are safe through generic Markdown AST editing.
- This does not claim `#3775` is closed until PR merge, closeout, and worktree pruning complete.

## Result

The bounded proof passed. The intended octocrab, AST, template, and pruning mechanisms now compose correctly for this workflow slice:

1. Octocrab-backed readiness works for the issue doctor path.
2. Markdown AST editing is available for ordinary Markdown documents.
3. Lifecycle-card edits remain governed by prompt-template/card-editor authority, with rendered-card edits available through values import/edit/render/validate.
4. Prior merged follow-up worktrees are absent after closeout pruning, while #3775 self-pruning remains correctly deferred to closeout.

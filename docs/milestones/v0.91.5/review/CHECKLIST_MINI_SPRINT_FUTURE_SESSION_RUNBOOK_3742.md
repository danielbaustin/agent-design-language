# Checklist Mini-Sprint Future-Session Runbook

Date: 2026-06-14
Milestone: v0.91.5
Issue: #3742
Source sprint: #3717
Purpose: preserve enough operating context for future Codex sessions to use,
audit, and extend the octocrab/refactor/template/AST checklist mini-sprint
without relying on chat memory.

## Quick Status

The checklist mini-sprint is complete and closed. It produced five merged child
issues plus one umbrella sprint closeout:

| Track | Issue | PR | Primary artifact | Status |
| --- | --- | --- | --- | --- |
| GitHub transport / octocrab gaps | #3712 | #3720 | `docs/milestones/v0.91.5/review/RELEASE_WATCHER_GITHUB_GAP_INVENTORY_3712.md` | merged |
| Refactored command surface inventory | #3713 | #3728 | `docs/milestones/v0.91.5/review/SHELL_WRAPPER_INVENTORY_SUMMARY_3713.md` | merged |
| Prompt templates / card workflow proof | #3714 | #3729 | `docs/milestones/v0.91.5/review/PROMPT_TEMPLATE_WORKFLOW_INTEGRATION_3714.md` | merged |
| markdown.rs / AST editing substrate | #3715 | #3731 | `docs/milestones/v0.91.5/review/MARKDOWN_AST_EDITING_SUBSTRATE_3715.md` | merged |
| Cross-system timing proof | #3716 | #3741 | `docs/milestones/v0.91.5/review/INTEGRATED_C_SDLC_TIMING_PROOF_3716.md` | merged |
| Umbrella sprint | #3717 | not_applicable | local ignored sprint state: `.adl/runs/sprints/3717/state.json` | closed |

The coordination checklist is tracked at:

- `docs/milestones/v0.91.5/review/OCTOCRAB_REFACTOR_TEMPLATE_AST_INTEGRATION_CHECKLIST_2026-06-14.md`

## What Future Sessions Should Trust

Source-backed facts:

- The child issues listed above were closed and their PRs were merged before this
  runbook was written; inspect the GitHub issues/PRs named in the evidence table
  below.
- The final sprint truth check for #3717 matched live GitHub state after #3716
  closeout; rerun the command in `First Commands For A Future Session` from the
  primary checkout to refresh that truth.
- The AST Markdown editor command exists as
  `adl tooling markdown-ast-edit replace-section`.
- Prompt-template workflow integration has a focused proof script at
  `adl/tools/test_prompt_template_workflow_integration.sh`.
- Shell wrapper status is inventoried in
  `docs/milestones/v0.91.5/review/SHELL_WRAPPER_INVENTORY_3713.tsv`.

Important non-claims:

- This does not mean every GitHub operation is octocrab-native.
- This does not mean `gh` fallback has been eliminated.
- This does not mean merge-time validation reuse is solved.
- This does not mean release/watcher support is complete.

## Evidence Links

| Item | URL |
| --- | --- |
| #3712 | `https://github.com/danielbaustin/agent-design-language/issues/3712` |
| PR #3720 | `https://github.com/danielbaustin/agent-design-language/pull/3720` |
| #3713 | `https://github.com/danielbaustin/agent-design-language/issues/3713` |
| PR #3728 | `https://github.com/danielbaustin/agent-design-language/pull/3728` |
| #3714 | `https://github.com/danielbaustin/agent-design-language/issues/3714` |
| PR #3729 | `https://github.com/danielbaustin/agent-design-language/pull/3729` |
| #3715 | `https://github.com/danielbaustin/agent-design-language/issues/3715` |
| PR #3731 | `https://github.com/danielbaustin/agent-design-language/pull/3731` |
| #3716 | `https://github.com/danielbaustin/agent-design-language/issues/3716` |
| PR #3741 | `https://github.com/danielbaustin/agent-design-language/pull/3741` |
| #3717 | `https://github.com/danielbaustin/agent-design-language/issues/3717` |

## First Commands For A Future Session

From the ADL repo root:

```sh
python3 adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py \
  --repo-root . \
  --state .adl/runs/sprints/3717/state.json \
  --print-json --require-match
```

Expected result for the completed sprint:

- `status: matched`
- `gate_passed: true`
- checked issues: `3712`, `3713`, `3714`, `3715`, `3716`
- checked PRs: `3720`, `3728`, `3729`, `3731`, `3741`

If the check reports drift, do not guess. Run the closeout or sprint-state
repair command it names, then rerun the truth check. The sprint state path is
local ignored state; if it is absent in a fresh worktree, run this command from
the primary ADL checkout where `.adl/runs/` local state is maintained.

## Normal Issue Flow To Continue This Work

Use the normal ADL issue lifecycle. Do not work on `main`.

```sh
GITHUB_TOKEN=$(cat "$HOME/keys/github.token") bash adl/tools/pr.sh run <issue> \
  --slug <issue-slug> \
  --version v0.91.5
```

If an unrelated open PR wave blocks a docs-only or explicitly independent issue,
use `--allow-open-pr-wave` only when the issue scope justifies it and record the
deviation in the SOR.

Publish through `pr.sh finish` after focused validation has already run and is recorded in the SOR:

```sh
GITHUB_TOKEN=$(cat "$HOME/keys/github.token") bash adl/tools/pr.sh finish <issue> \
  --title "<title>" \
  --paths "<comma-separated tracked paths>" \
  --output-card .adl/v0.91.5/tasks/issue-<issue>__<slug>/sor.md \
  --no-checks
```

After merge, run explicit closeout:

```sh
GITHUB_TOKEN=$(cat "$HOME/keys/github.token") bash adl/tools/pr.sh closeout <issue>
```

## AST Markdown Editing Command

Use this for bounded section replacement in ordinary docs and review packets:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling markdown-ast-edit replace-section \
  --input <doc.md> \
  --heading "<heading text>" \
  --replacement <replacement-section.md> \
  --out <doc.md> \
  --repair-note-out <repair-note.md>
```

Rules to remember:

- It is an editing substrate, not lifecycle authority.
- It rejects lifecycle card input/output paths.
- Prompt cards still go through prompt-template rendering and card-editor skills.
- Raw HTML and unsafe structural loss fail closed with repair notes.
- Cold worktrees may compile the Rust binary before the command runs; warm runs
  are much faster.

## Prompt Template / Card Workflow Command

For prompt-template/card workflow proof, use:

```sh
bash adl/tools/test_prompt_template_workflow_integration.sh
```

Use card editor skills for lifecycle truth repairs:

- `sip-editor`
- `stp-editor`
- `spp-editor`
- `srp-editor`
- `sor-editor`

Use prompt-template tooling for new or fully regenerated cards:

```sh
adl-csdlc tooling prompt-template validate-values --kind <kind> --values <values>
adl-csdlc tooling prompt-template edit-values --kind <kind> --values <values> --set <field=value> --out <values>
adl-csdlc tooling prompt-template render --kind <kind> --values <values> --out <card.md>
adl-csdlc tooling prompt-template validate-structure --kind <kind> --input <card.md>
adl-csdlc tooling prompt-template validate-schemas
```

`--no-checks` is not permission to skip proof. Use it only when the issue already
has a focused validation record and the remaining publication step should not
rerun unrelated broad validation.

## Known Traps Not To Rediscover

### Retired closeout auto-attach

`pr.sh finish` can still fail after creating the PR because
`attach_post_merge_closeout.sh` is retired. If the PR was created successfully,
verify PR state and continue explicitly. After merge, run:

```sh
GITHUB_TOKEN=$(cat "$HOME/keys/github.token") bash adl/tools/pr.sh closeout <issue>
```

### Draft merge / `gh` fallback

During #3715 and #3716, octocrab-backed paths were active, but merge handling
still needed visible `gh` fallback around draft/merge behavior. This must remain
explicit and logged until the Rust path handles it cleanly.

### Duplicate validation

The workflow can still rerun broad local validation after GitHub CI is already
green. That friction is part of the PVF/equivalence problem and should not be
normalized as success.

### Cold worktree compile cost

A fresh worktree may need to compile `adl` before Rust tooling commands are
available. #3716 observed a roughly one-minute cold compile before the AST editor
ran. Treat this as startup cost, not proof of workflow speed.

### Release/watcher gap

Native octocrab release/watcher support is not complete. The routed follow-up is
#3718.

## How To Interpret The Scores After This Sprint

The original scorecard was intentionally harsh. After the mini-sprint:

| Track | Practical status after #3717 |
| --- | --- |
| GitHub transport / octocrab | Strong for covered issue/PR paths; release/watcher remains routed. |
| Refactored command surface | Inventory exists; wrappers are classified, but not all wrappers are gone. |
| Prompt templates / cards | Focused workflow proof exists; continue enforcing renderer/editor boundary. |
| markdown.rs / AST editing | Implemented for bounded docs section replacement with guardrails. |
| Cross-system proof | First real proof exists; useful but not clean. |
| Follow-up routing | Core follow-ups are explicit; #3718 remains important. |

## Future-Session Checklist

Before changing related tooling, confirm:

- The issue has a bound worktree.
- `SIP`, `STP`, and `SPP` are ready before execution.
- Prompt cards are not patched directly when renderer/editor tooling applies.
- Docs edits use AST editing when doing bounded section replacement.
- Subagent review runs before PR publication.
- `git diff --check` passes.
- SRP records review findings and dispositions.
- SOR records actual validation, fallbacks, and integration truth.
- PR checks pass.
- Closeout runs after merge.
- Sprint state is reconciled if the issue is a sprint child.

## Residual Work

Carry these forward explicitly:

- #3718: native release/watcher octocrab support.
- Merge path: remove visible `gh` fallback for covered PR merge operations.
- PVF equivalence: avoid duplicate broad local validation when equivalent CI
  proof already exists.
- Binary/tool startup: reduce cold-worktree compile friction for small docs/tool
  operations.
- Wrapper retirement: continue moving shell wrappers toward Rust delegates or
  explicit removal.

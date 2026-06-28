# v0.91.6 Full Code Review Pass

Date: `2026-06-27`
Owner issue: `#4582`
Review stage: `WP-14A internal review expansion`
Status: `findings_recorded_for_release_tail_remediation`

## Scope

This pass expands WP-14A from a release-tail document/evidence review into a
code review pass over the current repository state. It inventories the complete
tracked repository and then focuses review effort on the highest-risk executable
surfaces for the current release tail:

- repo-native PR lifecycle and finish machinery
- SOR/SRP fact extraction and publication truth
- runtime AWS signal / heartbeat machinery
- prompt-template and card workflow control surfaces
- GitHub transport and command dispatch surfaces
- code-size and maintainability hotspots

This is a release-tail risk review, not a claim that every Rust line in the
repository was mechanically re-reviewed or that broad Rust validation was rerun.

## Code Inventory

Repository inventory found `4804` tracked files and roughly `279855` Rust source
lines under `adl/src`. The largest Rust/control-plane files are:

| File | Approx lines | Review note |
| --- | ---: | --- |
| `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs` | `7993` | Large finish-path test surface. |
| `adl/src/cli/pr_cmd/finish_support.rs` | `6167` | Central PR finish/SOR/publish control surface. |
| `adl/src/resilience.rs` | `5225` | Large resilience substrate. |
| `adl/src/cli/tests/pr_cmd_inline/basics.rs` | `3960` | Broad PR command parser/behavior test surface. |
| `adl/src/csdlc_prompt_editor.rs` | `3875` | Large prompt/card editor surface. |
| `adl/src/cli/pr_cmd/github.rs` | `2463` | GitHub transport and PR/issue interaction surface. |
| `adl/src/cli/pr_cmd_cards/cards.rs` | `2415` | Card lifecycle command surface. |
| `adl/tools/pr.sh` | `2096` | Compatibility shell entrypoint. |
| `adl/src/provider_adapter.rs` | `2002` | Provider adapter surface. |
| `adl/src/cli/tooling_cmd/issue_resource_telemetry.rs` | `1922` | Issue resource accounting surface. |

## Findings

### P1: `pr finish` can erase numbered SRP findings from machine-readable SOR facts

Expected: when an SRP records findings, `pr finish` should preserve those
findings in the emitted `sor_facts.review.findings` block regardless of whether
the review author used bullets or numbered Markdown.

Observed: `parse_sor_review_evidence` reads the SRP `Findings` section through
`bullet_lines_from_markdown_section` at
`adl/src/cli/pr_cmd/finish_support.rs:1003`. That helper only accepts lines with
a `- ` prefix at `adl/src/cli/pr_cmd/finish_support.rs:1063`. The WP-14A SRP
used numbered findings, so the current SOR emitted `findings_status:
findings_present` while `findings` became `not_recorded` at
`.adl/v0.91.6/tasks/issue-4582__v0-91-6-wp-14a-review-complete-internal-review-and-pre-v0-92-burn-down-checklist/sor.md:153`.

Impact: release-tail automation can claim findings exist while losing the actual
finding text in the machine-readable record. That weakens closeout truth and
makes downstream sessions repair cards that should have been mechanically
accurate.

Recommended route: fix the parser to support numbered Markdown list items and
add regression coverage using an SRP with numbered findings.

### P2: Live AWS runtime heartbeat attempts consume sequence numbers even when no signal is published

Expected: blocked live heartbeat publication should fail closed without mutating
heartbeat cursor state, because no durable signal was emitted.

Observed: `publish_runtime_heartbeat_signal` reserves a sequence before choosing
between mock and live mode at `adl/src/runtime_aws_signal.rs:344`. The live mode
branch then emits a failure and returns `Blocked` at
`adl/src/runtime_aws_signal.rs:404`, because live publication is not implemented
or not approved. The reservation writes the cursor at
`adl/src/runtime_aws_signal.rs:847`. Existing tests assert that live mode blocks
at `adl/src/runtime_aws_signal.rs:1160`, but they do not assert that the cursor
remains absent or unchanged.

Impact: a misconfigured or intentionally blocked live runtime can advance local
heartbeat sequence state without publishing a heartbeat. That creates gaps in
runtime history before live heartbeat transport is actually operational.

Recommended route: reserve heartbeat sequence only after the chosen transport is
known to be publishable, or roll back/avoid cursor writes for blocked live mode;
add a regression test asserting blocked live mode does not create or advance the
cursor.

### P2: Repo-native PR inventory remains incomplete for full internal-review automation

Expected: internal review should be able to inventory open PRs through the same
repo-native tooling used for issue and PR lifecycle control.

Observed: the command dispatch supports `create`, `init`, `run`, `doctor`,
`finish`, `validation`, `watch`, `issue`, `projection-map`, and `closeout` at
`adl/src/cli/pr_cmd.rs:210`, and `pr.sh` exposes the same lifecycle commands at
`adl/tools/pr.sh:18`. Repo-native issue list/search/view exists under the
`issue` subcommand, but there is no matching repo-native `pr list` or PR search
inventory command for release-tail review.

Impact: reviewers can inspect a known PR with `validation` or `watch`, but cannot
produce a complete PR inventory without falling back to manual GitHub inspection
or non-canonical shell tooling. That keeps the external-review handoff less
automatable than the issue side.

Recommended route: add a typed repo-native PR inventory command or explicitly
route an approved interim path until the GitHub convergence work finishes.

### P3: Several control-plane modules are too large for cheap, reliable review

Expected: high-churn workflow-control files should stay small enough that review,
coverage routing, and local reasoning remain cheap.

Observed: the largest Rust/control-plane files now include
`adl/src/cli/pr_cmd/finish_support.rs` at roughly `6167` lines,
`adl/src/resilience.rs` at roughly `5225` lines, and
`adl/src/csdlc_prompt_editor.rs` at roughly `3875` lines. The largest finish-path
test file is roughly `7993` lines. These files concentrate many concerns in a
small number of edit surfaces.

Impact: this is not an immediate behavior bug, but it increases regression risk
and review latency. The repeated finish/SOR/card-truth issues in this milestone
cluster around exactly these dense control-plane surfaces.

Recommended route: route a v0.91.7 refactoring/design issue to split the highest
churn control-plane files along stable seams: parser/extractor, validation-plan,
SOR fact emission, GitHub publication, and shell compatibility.

## Positive Results

- The repo-native command surface is materially stronger than earlier milestone
  state: issue list/search/view/create/comment/edit/close, PR validation/watch,
  projection maps, closeout, and finish publication are all present as typed
  command paths.
- The review-provider validator now transitively validates embedded provider
  results and has focused tests for provider failure semantics.
- Runtime AWS signal code has explicit mock/live/disabled modes and does not
  overclaim live transport completion.
- Prompt-template and card workflow tooling is present and wired deeply into the
  lifecycle, even though remaining fail-closed enforcement gaps are routed.

## Non-Claims

- This review does not claim broad Rust validation was rerun.
- This review does not claim runtime live AWS heartbeat publication works.
- This review does not claim C-SDLC automation is fully fail-closed.
- This review does not remediate the code findings; it records and routes them.

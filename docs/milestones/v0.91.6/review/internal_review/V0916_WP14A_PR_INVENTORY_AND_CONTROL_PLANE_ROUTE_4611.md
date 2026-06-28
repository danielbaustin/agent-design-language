# V0.91.6 WP-14A PR Inventory and Control-Plane Routing

Date: 2026-06-27
Issue: #4611
Scope: retained routing packet for the remaining WP-14A release-tail findings that are larger than the bounded numbered-findings parser fix.

## Summary

This packet records the truthful route for the two larger `#4611` surfaces:

1. repo-native PR inventory for review automation
2. control-plane module decomposition planning

`#4611` fixes the numbered-SRP-finding loss in `pr finish` and adds regression coverage. It does not widen into a full PR inventory command family or broad refactoring of the control plane.

## PR Inventory Truth

Current repo-native surfaces are useful but incomplete for review-wide PR inventory:

- `adl/tools/pr.sh projection-map --json` exposes the current GitHub/C-SDLC projection surfaces and source-of-truth mapping.
- `adl/tools/pr.sh issue list|search|view` covers issue-side release-tail inspection.
- existing `pr` lifecycle commands expose narrow PR facts during doctor, finish, watch, validation, and closeout flows.

What is still missing for full internal-review automation is a single typed inventory/report surface that can enumerate open PRs with enough review-tail metadata in one place.

## Required Follow-on

Owner: `area:tools` / GitHub-client control plane

Recommended follow-on title:

`[tools][github-client] Add repo-native PR inventory report for release-tail review automation`

Acceptance criteria for that follow-on:

- add a repo-native typed command surface for PR inventory/reporting; examples include `adl pr inventory` or an equivalently named release-tail report command
- report open PR number, title, URL, head branch, base branch, draft state, linked issue references, and milestone/version classification when inferable
- support machine-readable JSON output suitable for internal review packets
- preserve stdout/stderr observability separation and avoid raw `gh` as the canonical implementation
- include focused command-level proof and fixture-backed tests for at least mixed draft/open and mixed version/base cases

Interim approved operator path until that follow-on lands:

- use `projection-map` plus issue-side repo-native commands for bounded truth gathering
- do not claim full PR inventory automation from the current surface set

## Control-Plane Decomposition Route

Highest-value seams for future decomposition:

1. `adl/src/cli/pr_cmd/finish_support.rs`
   - split SOR fact emission / Markdown section parsing
   - split finish validation selection and execution
   - split PR publication/body-rendering helpers

2. `adl/src/cli/pr_cmd.rs`
   - split lifecycle dispatch/binding from issue-subcommand plumbing
   - split projection/report surfaces from execution/start/doctor logic

3. `adl/src/cli/pr_cmd/github.rs`
   - split issue transport, PR inventory/wave queries, and watcher/closeout helpers into smaller transport-focused modules

## Decomposition Acceptance Bar

Any decomposition follow-on should preserve:

- existing repo-native command names and JSON contracts unless intentionally versioned
- focused inline Rust test coverage for the extracted seams
- no broad workflow-policy rewrites hidden inside mechanical moves
- no regression in owner-binary / observability behavior

## Non-Claims

- This packet does not claim the PR inventory gap is fixed in `#4611`.
- This packet does not claim broad control-plane decomposition was executed in `#4611`.
- This packet does claim both surfaces are now explicitly routed with owner, scope, and acceptance expectations.

# ADL Owner-Binary Decomposition Slice (#4726)

Issue: #4726
Milestone: v0.91.7
WP: WP-06

## Summary

This slice decomposes two workflow-control surfaces out of the monolithic
`adl` command path:

- `adl-session` owns session-ledger status, claim, heartbeat, and release.
- `adl-process` owns permission-safe process status checks.

The compatibility commands remain available:

- `adl session ...`
- `adl process ...`

The preferred normal workflow path is now the owner binary when the built
binary exists in the repo binary directory.

## Current Owner-Binary Inventory

Existing PR lifecycle owner binaries:

- `adl-pr-create`
- `adl-pr-init`
- `adl-pr-repair-issue-body`
- `adl-pr-run`
- `adl-pr-doctor`
- `adl-pr-ready`
- `adl-pr-preflight`
- `adl-pr-finish`
- `adl-pr-validation`
- `adl-pr-inventory`
- `adl-pr-shepherd`
- `adl-pr-closing-linkage`
- `adl-pr-closeout`
- `adl-issue`

Existing broader owner binaries:

- `adl-csdlc`
- `adl-runtime`
- `adl-review`
- `adl-validate-structured-prompt`
- `adl-lint-prompt-spec`
- `adl-prompt-template`
- `adl-remote`
- `adl-aws-remote-validation`
- `adl-provider-adapter`

New in #4726:

- `adl-session`
- `adl-process`

## Remaining Monolithic Surfaces

The compatibility `adl` binary remains the catch-all orchestration surface for
runtime workflow shortcuts and less frequently used top-level commands. Those
paths are intentionally not removed in this issue.

Known remaining top-level compatibility surfaces include:

| Surface | Current owner status | Reason left in monolith for this slice |
| --- | --- | --- |
| `adl artifact` | compatibility-only | Not on the hot issue lifecycle path. |
| `adl agent` | compatibility-only | Runtime/long-lived-agent ownership needs a separate runtime issue. |
| `adl csm` | compatibility-only | Product/runtime surface, not WP-06 build control-plane hot path. |
| `adl demo` | compatibility-only | Demo surfaces should be split by demo-family issues. |
| `adl godel` | compatibility-only | Model/runtime planning surface, not this control-plane slice. |
| `adl identity` | compatibility-only | Runtime identity surface, separate owner boundary. |
| `adl provider` | compatibility-only | Provider setup remains available through `adl-runtime provider setup`; deeper provider split belongs to provider work. |
| `adl scheduler` | compatibility-only | Runtime scheduler surface, separate runtime/scheduler owner issue. |
| `adl tooling` | partially owned | Prompt validators and template tools already have owner binaries; remaining tooling subcommands need follow-on grouping. |
| `adl keygen`, `adl sign`, `adl verify`, `adl instrument`, `adl learn`, `adl resume` | compatibility-only | Runtime document/tooling surfaces, not required for this WP-06 hot-path slice. |

## Proof Boundary

This issue proves a real owner-binary extraction for session-ledger and
process-status commands. It does not claim the monolithic `adl` binary is
retired.

Focused proof should cover:

- `cargo test --manifest-path adl/Cargo.toml --bin adl-session --bin adl-process`
- `cargo check --manifest-path adl/Cargo.toml --bin adl-session --bin adl-process`
- direct owner-binary invocation after build:
  - `adl/target/debug/adl-session --help`
  - `adl/target/debug/adl-process --help`
- compatibility invocation remains available:
  - `adl/target/debug/adl session --help`
  - `adl/target/debug/adl process --help`

## Operational Notes

Normal agents should prefer the owner binaries when the repo binary directory
contains them. The monolithic `adl` command remains an intentional compatibility
surface while the remaining top-level commands are split by owner boundary.

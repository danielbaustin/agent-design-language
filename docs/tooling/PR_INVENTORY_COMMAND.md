# Repo-Native PR Inventory Command

## Purpose

Release-tail review needs a repo-native way to list open pull requests without
falling back to raw `gh pr list`. The approved command is:

```bash
bash adl/tools/pr.sh pr-inventory --json
```

Use this command when a release, sprint, or review pass needs current PR
inventory truth before deciding whether an issue, PR, or closeout tail is still
open.

## Contract

The command delegates to the dedicated Rust owner binary `adl-pr-inventory`.
Cargo fallback is not required during normal use when the owner binary has been
built.

Machine-readable JSON is emitted on stdout. Human-oriented `adl_event`
observability remains on stderr by default.

The JSON schema identifier is:

```text
adl.pr_inventory.v1
```

Each pull-request record includes:

- PR number, title, URL, state, draft status, head branch, and base branch
- inferred workflow queue, when known
- closing issue numbers discovered from live GitHub PR metadata
- updated timestamp and mergeability when exposed by the GitHub API
- validation check summary derived from the repo-native PR validation path using
  the same effective-check view that removes stale duplicate check runs

## Failure Behavior

The command fails closed when GitHub authentication, repository lookup, PR
inventory, closing-issue lookup, or validation-status lookup fails. It must not
print token contents or credential material.

## Validation

Focused proof for this command should include:

```bash
cargo check --manifest-path adl/Cargo.toml --bin adl-pr-inventory
cargo build --manifest-path adl/Cargo.toml --bin adl-pr-inventory
bash adl/tools/pr.sh pr-inventory --help
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token bash adl/tools/pr.sh pr-inventory --json
```

If the live inventory currently has no open PRs, the live command is a valid
transport smoke proof when it emits parseable `adl.pr_inventory.v1` JSON for
the target repository. It is not sufficient by itself for row-shape or check
summary changes; pair it with a focused populated-row or effective-check
regression test.

This command is a bounded release-tail inventory utility. Broader PR control
plane decomposition and binary-size refactoring remains owned by `#4481` and
is not completed by this command.

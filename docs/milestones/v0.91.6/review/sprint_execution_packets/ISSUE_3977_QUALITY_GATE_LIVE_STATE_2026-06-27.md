# Issue 3977 Quality-Gate Live-State Packet

Issue: `#3977`  
Date: 2026-06-27  
Purpose: retain the live-state observations used by the WP-12 quality gate.

## Commands

```bash
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token \
ADL_PR_RUST_BIN=<repo>/adl/target/debug/adl \
bash adl/tools/pr.sh watch 3976 --version v0.91.6 --json
```

```bash
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token \
ADL_PR_RUST_BIN=<repo>/adl/target/debug/adl \
bash adl/tools/pr.sh issue view 3976 --json
```

```bash
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token \
ADL_PR_RUST_BIN=<repo>/adl/target/debug/adl \
bash adl/tools/pr.sh issue view 3925 --json
```

```bash
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token \
ADL_PR_RUST_BIN=<repo>/adl/target/debug/adl \
bash adl/tools/pr.sh issue search --query "is:issue is:open label:version:v0.91.6" --state open --limit 100 --json
```

```bash
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token \
ADL_PR_RUST_BIN=<repo>/adl/target/debug/adl \
bash adl/tools/pr.sh issue search --query "is:pr is:open label:version:v0.91.6" --state open --limit 100 --json
```

## Retained Observations

| Surface | Observation | Gate meaning |
| --- | --- | --- |
| `#3976` issue | Closed. | WP-11 no longer blocks WP-12 proof consumption. |
| `#3976` PR | Linked PR `#4605`, head `codex/3976-v0-91-6-wp-11-demo-demo-matrix-and-proof-convergence`, base `main`, state merged, draft false. | Upstream proof has landed and is consumable by WP-12. |
| `#3976` checks | `adl-ci` success, `adl-coverage` success, `adl-slow-proof` skipped. | WP-11's CI posture is green for its scoped docs/proof surface. |
| `#3976` closeout | `pr.sh watch 3976 --json` reports `classification: closeout_needed` and `next_skill: pr-closeout`. | Remaining WP-11 closeout normalization is release-tail hygiene, not a WP-12 blocker. |
| `#3925` issue | Closed on 2026-06-18. | Repo-quality/staleness lane is landed and consumable by WP-12. |
| Open release-tail issues | Before WP-11 merged, search returned `#3976`, `#3977`, `#3978`, `#3980`-`#3984`, `#4582`, and `#4604` among current open `version:v0.91.6` issues. After WP-11, `#3976` is closed and the remaining frontier starts at WP-12. | Expected release-tail frontier remains open; WP-12 must not claim release readiness. |
| Open v0.91.6 PR search | A prior search returned no `is:pr is:open label:version:v0.91.6` items while `pr.sh watch 3976` found PR `#4605`. | The issue-search PR query is not reliable as sole PR inventory evidence; watcher evidence is authoritative for linked issue PR state. |

## Tooling Notes

- Fresh issue worktrees may not contain owner binaries. This packet used the
  root owner binary through `ADL_PR_RUST_BIN` rather than enabling cargo
  fallback.
- `pr.sh issue search` requires `--query`; positional query strings are
  rejected.
- Direct `adl issue view` is not an available command; use
  `pr.sh issue view`.

## Non-Claims

- This packet does not perform WP-11 closeout normalization.
- This packet does not close WP-12 by itself.
- This packet does not prove a complete PR inventory across all labels; it
  records the issue-local live-state observations WP-12 consumed.

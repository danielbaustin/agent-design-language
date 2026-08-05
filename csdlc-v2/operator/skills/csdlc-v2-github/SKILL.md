---
name: csdlc-v2-github
description: Execute repo-native C-SDLC v2 GitHub issue and PR actions through the split typed Rust command surface.
---

Prefer the split owner binaries:

- Invoke `csdlc-github-issue run --request <request.json>` for
  `issue_create`, `issue_update`, `issue_comment`, `issue_close`, and
  `issue_read`.
- Invoke `csdlc-github-pr state --request <pr-state-request.json>` for direct
  PR-state observation.
- Invoke `csdlc-github-pr run --request <github_action_request.json>` only for
  compatibility with `action: "pr_state"`.

`csdlc-github run --request <request.json>` remains a compatibility facade for
the same typed `github_action_request` payload while callers migrate. Do not use
the GitHub connector, raw `gh`, legacy wrappers, shell/Python lifecycle
mutation, or AWS.

Every issue/comment mutation must carry an `operation_key`; the command renders
it as a stable marker, reads back remote state, and fails closed on missing,
duplicated, or mismatched reconciliation.

Supported action values:

- `issue_create`
- `issue_update`
- `issue_comment`
- `issue_close`
- `issue_read`
- `pr_state`

`pr_state` is read-only readiness observation. PR publication and terminal
delivery remain under the repo-native Rust v2 command surface:
`csdlc-publish` and `csdlc-finish`. Do not route those
operations through connector actions or legacy wrapper commands.

The install/coexistence inventory must include `csdlc-github`,
`csdlc-github-issue`, `csdlc-github-pr`, and `csdlc-pr-state`.
Treat a missing split binary as an installation failure, not as permission to
fall back to raw GitHub tooling.

Use the shared GitHub token resolver through `token_file`,
`ADL_GITHUB_TOKEN_FILE`, `ADL_GITHUB_TOKEN`, `GITHUB_TOKEN`, `GH_TOKEN`, or the
operator-approved default token file. Never print or persist token contents.

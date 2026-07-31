# ADL C-SDLC GitHub Client Boundary

This document records the current C-SDLC v2 ownership boundary for GitHub issue
and pull-request operations.

## Canonical Entry Points

GitHub work for C-SDLC v2 is owned by repo-native Rust binaries and the shared
token resolver. Do not use the ChatGPT GitHub connector, raw `gh`, legacy
wrappers, shell/Python lifecycle mutation, or AWS for covered lifecycle writes.

The current command surface is split by responsibility:

- `csdlc-github-issue` owns GitHub issue lifecycle actions:
  `issue_create`, `issue_update`, `issue_comment`, `issue_close`, and
  `issue_read`.
- `csdlc-github-pr` owns GitHub PR observation through `pr_state`.
- `csdlc-pr-state` remains the dedicated low-level PR-state observer used by
  other v2 binaries.
- `csdlc-merge` remains the exact-head merge authority.
- `csdlc-github` remains a compatibility facade while callers migrate to the
  narrower owner binaries.

Every issue/comment mutation must carry an `operation_key`. The GitHub command
surface renders it as a stable marker, reads back remote state, and fails closed
on missing, duplicated, or mismatched reconciliation.

## Shared Client Ownership

Shared GitHub behavior belongs in the C-SDLC v2 GitHub library code, not in
individual command wrappers:

- token-source selection through the shared resolver
- marker rendering and exact-marker checks
- issue readback and idempotent mutation reconciliation
- PR state normalization and readiness classification
- retry/backoff behavior through the shared `adl-resilience` crate where a
  bounded retry policy is appropriate

The GitHub app connector is read-only for this repository and is not a write
fallback. The operator-approved token file may be supplied through
`token_file`/`ADL_GITHUB_TOKEN_FILE`; token contents must never be printed,
copied, persisted into tracked artifacts, or committed.

## Install Contract

The v2 install/coexistence manifests must require every operational GitHub
owner binary:

- `csdlc-github`
- `csdlc-github-issue`
- `csdlc-github-pr`
- `csdlc-pr-state`
- `csdlc-merge`

`csdlc-install install` must build and install the reviewed binary set into the
dedicated `.adl/bin/csdlc-v2/` generation directory. `csdlc-install verify`
must fail closed when any required binary is missing, non-executable, symlinked,
or built from stale provenance.

## Migration Rules

- Prefer `csdlc-github-issue` for issue actions.
- Prefer `csdlc-github-pr state --request <request.json>` or `csdlc-pr-state`
  for PR observation.
- Keep `csdlc-github run --request <request.json>` only as compatibility during
  migration.
- Do not add new issue actions to `csdlc-github-pr`.
- Do not add new PR actions to `csdlc-github-issue`.
- Do not route publication, readying, merging, or closeout through connector
  actions; keep those under `csdlc-publish`, `csdlc-merge`, and
  `csdlc-closeout`.
- Unsupported GitHub workflow operations must fail closed until a repo-native
  Rust implementation exists.

## Proof Hooks

Focused proof for this boundary lives in:

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_github_actions`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate10a`
- `csdlc-install install --repo <repo> --destination <repo>/.adl/bin/csdlc-v2`
- `csdlc-install verify --repo <repo> --bin-dir <repo>/.adl/bin/csdlc-v2 --inventory csdlc-v2/operator/coexistence.json`

These checks prove that issue and PR actions are split, marker reconciliation is
exact, and the stable installed binary set cannot omit required GitHub owner
binaries.

# ADL C-SDLC GitHub Client Boundary

This document records the v0.91.5 ownership boundary for GitHub issue and PR
metadata operations during the octocrab migration.

## Canonical Entry Points

- `adl/tools/pr.sh` remains the canonical agent-facing workflow wrapper during
  migration.
- `adl-csdlc` is the Rust-owned C-SDLC compatibility binary.
- Old `adl pr ...` compatibility paths and `adl-csdlc issue ...` paths route
  through the same PR control-plane implementation.

## Shared Client Ownership

GitHub issue and PR metadata interpretation is owned by the shared PR
control-plane client layer in:

- `adl/src/cli/pr_cmd/github_client.rs`
- `adl/src/cli/pr_cmd/github.rs`

The shared layer owns:

- GitHub client mode selection through `ADL_GITHUB_CLIENT`.
- Token-source selection using `GITHUB_TOKEN` before `GH_TOKEN`.
- Fallback policy for `auto`, `octocrab`, and `gh` modes.
- Fail-closed shell fallback disablement through
  `ADL_GITHUB_DISABLE_GH_FALLBACK`.
- Issue metadata parity planning.
- PR wave filtering.
- PR closing-linkage interpretation.

## Migration Rules

- Do not duplicate GitHub issue or PR metadata interpretation in `adl-csdlc`.
- Do not bypass `adl/tools/pr.sh` as the taught operator entrypoint.
- Do not remove `gh` fallback in this migration slice.
- Do not silently use `gh` fallback when `ADL_GITHUB_DISABLE_GH_FALLBACK`
  is enabled.
- Do not introduce GitHub App authentication in this migration slice.
- Do not rename public workflow commands in this migration slice.

## Proof Hooks

The focused ownership checks live in the Rust CLI tests:

- `csdlc_dispatch_exposes_help_and_version_without_runtime_dispatch`
- `csdlc_issue_run_maps_to_existing_pr_start_command`
- `csdlc_github_client_boundary_doc_records_shared_ownership`

These checks prove that `adl-csdlc` remains a compatibility surface over the
shared PR control-plane path instead of becoming a second GitHub workflow truth.

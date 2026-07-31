# #5684 C-SDLC GitHub Tool Split Design

## Scope

Split the oversized `csdlc-github` command surface into narrower owner binaries while preserving compatibility for existing callers. Keep issue mutation/read/comment/close actions behind an issue-specific binary, PR observation behind a PR-specific binary, and exact-head merge authority in `csdlc-merge`.

## Implementation Boundary

- Add a standalone `adl-resilience` crate for reusable retry/backoff primitives.
- Use the shared resilience crate from `csdlc-v2`, `adl`, and `adl-runtime`.
- Keep `csdlc-github` as a compatibility facade.
- Add `csdlc-github-issue` and `csdlc-github-pr` as required operational owner binaries.
- Update the C-SDLC operator manifest and coexistence inventory so stable installation and verification fail when the split binaries are missing.
- Update current operator-facing docs and skill guidance so agents route issue
  and PR GitHub work through the split owner binaries instead of the broad
  compatibility facade.
- Remove current bootstrap guidance that still routes prompt/card validation
  through the deleted `adl/tools/validate_structured_prompt.sh` wrapper; current
  lifecycle validation must use typed v2 state and `csdlc-validate`.

## Validation Boundary

Focused proof must cover shared resilience tests, GitHub action exact marker readback and split-binary rejection, Gate 10A install/coexistence tests, `adl-runtime` compile, `adl` library compile, and a stable install/coexistence proof from the exact source revision.

# #5404 WP-12 Security And Protocol Review Fix Design

## Scope

Resolve the code-side portions of the #5403 WP-12 review findings while the
v0.91.7 milestone-document claim collision is tracked separately by #5415.

## Approach

- Inspect WP-12 validators and selected CI lane routing.
- Repair stale issue-state assumptions where the current code can do so without
  touching blocked milestone docs.
- Preserve fail-closed security posture and avoid upgrading proof claims without
  real boundary-crossing execution evidence.

## Validation

- Focused validator tests for touched WP-12 scripts.
- Focused Rust tests for touched CAV or credential-policy code.
- Diff hygiene before review.

## Deferred Boundary

Milestone-doc truth repairs under `docs/milestones/v0.91.7` are blocked by the
closed #5383 stale broad claim until #5415 is resolved or the operator clears
the claim.

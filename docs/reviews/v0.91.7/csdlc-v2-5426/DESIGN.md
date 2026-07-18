# C-SDLC v2 Validation Supersession (#5426)

## Decision

Keep validation evidence append-only, but evaluate readiness and terminal card
validity from the latest observation for each logical validation identity.
The identity is the stable tuple of command, purpose, and evidence reference.

## Semantics

- A later observation with the same identity supersedes an earlier observation.
- Ordering is the existing SOR vector order; the last matching item is current.
- A later `passed` or `skipped_non_goal` result permits readiness.
- A later `failed`, `blocked`, `waiting`, or `deferred` result fails closed.
- Distinct validations remain independently required.

## Scope

Implement one shared helper and use it anywhere terminal validation status is
derived. Add focused regression tests for pass-after-waiting and
failure-after-pass. Preserve the full evidence history in the SOR.

## Non-Goals

- No removal or mutation of historical validation observations.
- No weakening of remote checks, review, or conflict gates.
- No runtime or product behavior changes.


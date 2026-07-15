# v0.91.8 Vision

`v0.91.8` makes the platform boring enough for `v0.92`.

The milestone should leave ADL with a small, typed, deterministic core that
delegates runtime execution to Runtime v3 and lifecycle governance to C-SDLC v2.
The goal is not novelty. The goal is exact-revision acceptance, stable
installation, recoverable operations, and reviewable handoff.

## Desired End State

- ADL v2 owns language contracts, validation, canonical plans, signing, and a
  thin selector-backed CLI.
- Runtime v3 owns execution, provider/tool runtime behavior, operations, and
  recovery.
- C-SDLC v2 owns lifecycle cards, claims, review, publication, shepherding, and
  closeout.
- Incumbent ADL code that is replaced by these owners is eligible for deletion
  only after reviewed acceptance and protection windows.
- `v0.92` consumes exact deployed revisions and explicit non-claims.

## Success Shape

The milestone succeeds when a fresh consumer can install and exercise the
accepted ADL v2, Runtime v3, and C-SDLC v2 stack, verify rollback/recovery, and
read a `v0.92` handoff packet that names every claim boundary.

## Failure Shape

The milestone fails closed if parity is unproven, installation relies on build
cache accidents, Runtime/C-SDLC ownership boundaries blur, deletion evidence is
missing, or birthday-facing claims are asserted without exact proof.


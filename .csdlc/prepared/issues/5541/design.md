# Design: reconcile final Gate 10D2 operational authority

## Context

The final selector and coexistence inventory declare v2 as default and v1
sunset, but two retained current guidance surfaces still instruct operators to
use or preserve deleted v1 routes.

## Change

Align the retained v2 init skill with final D2 authority. Replace the current
default-workflow document with an executable typed-v2 workflow. Extend Gate
10A coverage to fail when current operational skills or the current default
workflow retain sunset instructions. Explicitly historical evidence remains
out of scope for the guard.

## Validation

Run Gate 10A, documentation command/path checks, strict Clippy, and an
exact-revision review over the bounded files.

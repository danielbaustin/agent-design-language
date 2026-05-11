# v0.95 Feature: Control-Plane Rust Migration and Tooling Hardening

## Status

Forward-planning feature contract for `v0.95`.

## Purpose

Complete the highest-value Rust migration and tooling-hardening tranche so the
MVP control plane can be explained as a coherent, durable, low-friction
execution substrate rather than a mixed-language provisional layer.

## Source Inputs

- `docs/planning/PYTHON_ELIMINATION_STAGED_PLAN.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.85/features/ROAD_TO_v0.95.md`

## Scope

This feature should establish:

- Rust migration of the highest-risk remaining control-plane/tooling surfaces
- hardening of workflow, validation, review, and publication paths
- explicit residual-language boundary if any non-Rust tooling still remains
- final convergence between lifecycle tooling and MVP reviewability

## Non-goals

- rewriting every script regardless of value
- destabilizing MVP delivery for migration purity alone
- treating migration-only work as a user-visible feature demo obligation

## Completion Target

`v0.95`

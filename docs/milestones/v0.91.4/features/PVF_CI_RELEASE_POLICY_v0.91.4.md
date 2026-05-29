# PVF CI and Release Policy

## Purpose

Define how the first bounded Parallel Validation Fabric integrates with ordinary
PR validation, docs-only validation, cache/reuse posture, and release-gate-only
proof.

This packet is intentionally narrow. It does not claim a complete CI scheduler
or a final release-orchestration system. It records the minimum truthful policy
needed to route ordinary PRs without confusing skipped work, deferred work,
reused work, and release-only proof.

## Scope

This policy covers:

- docs-only PR validation as a first-class PVF lane
- runtime/source PR validation as a first-class PVF lane
- release-gate-only proof lanes that must remain visible on ordinary PRs
- bounded artifact reuse semantics for unchanged proof lanes

This policy does not:

- replace branch protection or stable GitHub check names
- weaken authoritative coverage or release evidence rules
- treat reuse as a waiver when inputs are different

## Lane Mapping

### Docs-only PR lanes

- lane ids:
  - `docs_only_pr`
  - `docs_only_reuse_candidate`
- lane class: `docs`
- ordinary PR status when docs/planning/operator-policy files changed:
  `passed` and/or `reused`
- ordinary PR status when docs/planning/operator-policy files did not change:
  `skipped`
- release interpretation: still a bounded ordinary lane, not release evidence by
  itself

These are the newly explicit docs-only lanes added to the PVF policy surface.
Docs-only work
must not be forced to masquerade as runtime validation, but it also must not be
treated as "nothing happened."

The `docs_only_reuse_candidate` lane exists so the first bounded policy packet
can prove reuse status explicitly rather than describing reuse only in prose.

### Runtime PR lane

- lane id: `runtime_pr_fast`
- lane class: `cli_workflow`
- ordinary PR status when runtime-affecting surfaces changed: `passed`
- ordinary PR status when runtime-affecting surfaces did not change: `skipped`

This lane represents the existing focused PR-fast validation posture. It is the
ordinary PR proof lane for runtime-affecting changes, not the release-evidence
lane.

The default runtime PR lane must not include known slow proof-materialization
families. Tests that build or validate heavyweight runtime-v2 proof packets,
fixture materialization, or golden release evidence belong behind the
`slow-proof-tests` feature and run through explicit slow-proof or release-gate
commands.

### Runtime slow-proof lane

- lane id: `runtime_slow_proof`
- lane class: `release_gate`
- ordinary PR status: `release_gate_required` or `deferred`
- explicit command:
  `cargo nextest run --features slow-proof-tests --status-level all --final-status-level slow`
- sharded command shape:
  `cargo nextest run --features slow-proof-tests --partition count:N/4 --status-level all --final-status-level slow`

This lane preserves the value of expensive proof tests without putting them in
the same queue as ordinary PR-fast correctness checks. The slow lane is
appropriate for push-to-main, nightly/ratchet runs, release-gate validation, or
operator-requested proof. It must remain visible in PVF status as pending,
deferred, passed, failed, or release-gate-required; it must not disappear behind
a green fast lane.

### Release coverage lane

- lane id: `authoritative_release_gate`
- lane class: `release_gate`
- ordinary PR status: `release_gate_required`
- release-mode status: `passed` when its command succeeds

This is the key distinction the policy must preserve:

- on an ordinary PR, the release-gate lane is not "skipped"
- it remains explicit as required future proof
- on release-mode execution, the same lane becomes runnable proof

## Reuse Policy

Artifact reuse is allowed only as bounded proof with explicit invalidation
inputs.

For the first PVF policy slice, reuse is only truthful when all of the
following remain stable:

- lane identity
- lane command
- lane mode (`pr` versus `release`)
- changed-path trigger class for the lane
- the underlying proof inputs named by the lane

If any of those change, the lane must rerun and must not report `reused`.

The first bounded runner expresses reuse through lane status:

- `reused`

and reason:

- `artifact_reuse_confirmed`

That is enough for the first policy packet. Richer provenance storage remains
future work.

## Policy Fixture Boundary

The tracked manifest for this issue is a bounded policy fixture, not the final
production CI workflow. Its commands are intentionally deterministic local
fixture commands so the manifest itself can be executed and verified without
needing live release credentials or heavyweight coverage runs.

That means:

- the manifest proves truthful lane routing and aggregate status semantics
- it does not claim that the full docs-only production validator bundle has
  already been collapsed into one PVF command
- broader production validation posture remains described in the CI runtime
  policy guide

## Ordinary PR Interpretation

An ordinary PR may legitimately end with aggregate state
`release_gate_required` even when all runnable ordinary lanes passed.

That is healthy when:

- docs-only or runtime PR lanes ran truthfully
- release-gate-only proof remained explicit
- no failed or blocked lanes are hidden

The ordinary PR aggregate must not reinterpret release-gate-only proof as
`skipped`, because that would erase required future evidence.

## Release Interpretation

A release-mode PVF run should convert the release-gate lane from
`release_gate_required` into a runnable lane with ordinary success/failure
truth.

For this bounded issue, the release-mode proof is limited to showing that the
same manifest can distinguish:

- PR-mode release-gate requirement
- release-mode runnable proof

It does not claim the full release stack is yet driven by PVF.

## Non-claims

- This packet does not claim that GitHub CI has already been rewritten to call
  the PVF runner for every check.
- This packet does not claim that authoritative coverage can be replaced by a
  docs-only or PR-fast lane.
- This packet does not claim that reuse is sound across changed commands,
  changed manifests, or changed release policy.
- This packet does not claim that slow proof has passed when the ordinary
  PR-fast lane passes; slow proof remains a separate explicit evidence lane.

# VPP Validation Planning And PVF Lane Registry

## Status

- Status: bridge-defined in `v0.91.6`
- Scope: validation-planning contract plus registry-backed finish proving slice
- Outcome type: tooling + contract + feature packet
- Does not prove: active six-card prompt-template lifecycle adoption, universal registry-backed finish execution for every lane, or full replacement of all hard-coded validation routing in one issue

## Why this exists

ADL already has partial PVF lane planning surfaces:

- issue bootstrap infers an initial PVF lane
- `SPP` and `SOR` carry planned/final PVF lane truth
- `adl/tools/select_validation_lanes.py` selects lanes from a machine-readable manifest
- `adl/tools/validation_manager.py` builds a fail-closed validation profile from that selector

But the control plane still had two important gaps:

- validation planning was not yet named as its own first-class lifecycle contract
- `pr finish` still relied on hard-coded Rust path classification even when a machine-readable validation profile already existed

This feature defines the missing VPP contract boundary and lands a bounded registry-backed finish slice so later issues can expand from a truthful foundation.

## VPP meaning

`VPP` means `Validation Planning Prompt`.

It is the planned proof surface between design/execution planning and review/closeout:

`SIP -> STP -> SPP -> VPP -> SRP -> SOR`

## What VPP owns

A future first-class `VPP` card should own:

- selected PVF lanes
- proof commands and run commands
- expected proof artifacts
- expected runtime class
- parallel grouping
- cache/equivalence grouping
- failure semantics
- release-gate posture
- selection evidence and rationale
- linkage back to the parent issue plan and forward to `SRP`/`SOR`

## v0.91.6 proving slice

This issue lands a bounded proving slice instead of the whole active prompt-template expansion:

- the selector registry now carries VPP-grade metadata for a migrated lane
- the validation profile preserves that lane metadata for downstream consumers
- `pr finish` can consume the registry-backed profile for the migrated docs lane instead of relying only on hard-coded Rust classification
- invalid registry metadata continues to fail closed

The migrated first slice is the `docs_diff_check` lane.

## Current control-plane split

### Already machine-readable

- `adl/config/validation_lane_selector.v0.91.6.json`
- `adl/tools/select_validation_lanes.py`
- `adl/tools/validation_manager.py`

These surfaces already classify changed paths, preserve fail-closed escalation behavior, and emit machine-readable lane/profile truth.

### Still partially hard-coded before this issue

- `adl/src/cli/pr_cmd/finish_support.rs`

Before this slice, `pr finish` rendered the machine-readable validation profile for operator visibility, but execution still fell back to hard-coded Rust path classifiers.

## What this issue changes

- enrich the lane registry with VPP-grade metadata for the migrated lane:
  - `contract_version`
  - `artifacts`
  - `expected_runtime_class`
  - `parallel_group`
  - `cache_equivalence_group`
  - `failure_semantics`
- validate that malformed VPP metadata fails closed at manifest-load time
- carry that lane metadata through the validation profile
- allow `pr finish` to execute the registry-backed `docs_diff_check` lane when the profile is runnable and publication-sufficient
- preserve the existing hard-coded Rust fallback for non-migrated lanes and explicit issue-number special cases

## Why the docs lane first

The docs-only lane is the safest proving migration because it is:

- already explicit in the selector manifest
- deterministic
- low-risk
- easy to fail closed
- representative of the planning-to-finish control-plane handoff

It proves the registry-backed execution seam without widening this issue into a full rewrite of all finish validation routing.

## Handoff to #4309

This issue intentionally does **not** activate a sixth lifecycle card in the current prompt-template registry.

That expansion belongs to `#4309`, which is the explicit next-version prompt-template issue for:

- active template-set evolution
- new card kind support in the renderer/validator stack
- VPP/time/token/goal field integration across the lifecycle

In other words:

- `#4308` defines the VPP contract boundary and proves registry-backed consumption
- `#4309` performs the next prompt-template version rollout for the first-class card

## Fail-closed rules

The VPP/registry path must remain fail-closed:

- malformed lane metadata is invalid manifest data
- uncovered changed paths still escalate
- release-only or slow-proof paths still escalate rather than pretending ordinary PR sufficiency
- `pr finish` uses the registry-backed path only when the profile is runnable and publication-sufficient
- non-migrated lanes continue to use existing hard-coded routing until their registry-backed execution paths are proven

## Non-claims

This feature does not claim:

- that `VPP` is already an active card in the current prompt-template registry
- that every existing finish lane is registry-backed
- that all PVF execution already routes through one universal manifest runner
- that release-gate or slow-proof decisions can be automated away

## Validation posture

The proving surface for this issue should stay focused on:

- selector manifest contract behavior
- invalid metadata fail-closed behavior
- validation-profile carry-through
- registry-backed docs-lane finish selection
- issue-local card truth

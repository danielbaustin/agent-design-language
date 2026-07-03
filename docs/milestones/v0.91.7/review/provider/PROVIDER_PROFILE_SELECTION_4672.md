# Provider Profile Selection Proof (#4672)

Status: implemented

Issue: #4672

## Scope

This packet records the v0.91.7 WP-05 provider-profile selection work. The scheduler now accepts an optional role-provider selection context, validates it against the tracked provider profile registry, and emits the selected provider route on affected scheduler decisions.

## Implemented Behavior

- Added `RoleProviderSelectionContextV1` to scheduler economics bundles under schema `adl.scheduler.economics_input_bundle.provider_route.v1`.
- Added role policies, task assignments, provider candidate routes, eligibility state, and fail-closed validation.
- Required provider profile references to exist in the tracked provider profile registry.
- Kept the old `adl.scheduler.economics_input_bundle.v1` contract compatible by rejecting `role_provider_context` unless the provider-route bundle schema is declared.
- Resolved assigned tasks to the first eligible candidate route in declared order.
- Added route-resolution trace entries for selected, rejected, and later eligible candidates.
- Rejected duplicate task assignments instead of silently overwriting routes.
- Preserved existing scheduler decisions for tasks without a provider assignment.

## Proof Artifacts

- Input fixture: `docs/milestones/v0.91.7/review/provider/artifacts/provider_profile_selection_input_4672.json`
- Output plan: `docs/milestones/v0.91.7/review/provider/artifacts/provider_profile_selection_plan_4672.json`

The retained output shows `first-pass-review` assigned to tracked profile `chatgpt:gpt-5.3-codex` with decision schema `adl.scheduler.decision.provider_route.v1` and leaves `docs-status-check` on the original `adl.scheduler.decision.v1` shape without a provider route.

## Validation

- `cargo fmt --manifest-path adl/Cargo.toml --all -- --check`
- `cargo test --manifest-path adl/Cargo.toml scheduler::tests::role_provider --lib -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml scheduler::tests::cognitive_scheduler_plan_routes_fixture_lanes --lib -- --nocapture`
- `cargo build --manifest-path adl/Cargo.toml --bin adl`
- `ADL_OBSERVABILITY_LOG=$TMPDIR/adl-4672-provider-route.log adl/target/debug/adl scheduler plan --input docs/milestones/v0.91.7/review/provider/artifacts/provider_profile_selection_input_4672.json --out docs/milestones/v0.91.7/review/provider/artifacts/provider_profile_selection_plan_4672.json`
- JSON assertion over the output plan confirmed source schema, provider-route decision schema, selected profile, provider kind, model ref, selection trace, rejected local candidate, original decision schema for unassigned tasks, and no route for unassigned tasks.

## Review Disposition

Pre-PR review found two issues and both were fixed before publication:

- Provider-route fields originally extended the v1 bundle/decision shape without a distinct schema signal. Fixed by requiring `adl.scheduler.economics_input_bundle.provider_route.v1` for provider-route inputs and emitting `adl.scheduler.decision.provider_route.v1` only for decisions that carry `provider_route`.
- Duplicate task assignments originally could overwrite earlier routes. Fixed by rejecting duplicate task assignments during role-provider context validation.

## Non-Claims

- This does not execute a live model call.
- This does not prove local-model reviewer suitability.
- This does not implement dynamic price lookup, bidding, or autonomous provider switching.
- This does not grant provider routes merge or release authority.

## Operational Note

The first test attempt was stopped because it began compiling before cache warmup finished. After `warm_rust_dependency_cache.py` completed, the focused role-provider tests passed. The lib-test and binary profiles still compiled a large AWS/Workspace dependency tail, which is retained as build-system friction evidence for the validation tooling track.

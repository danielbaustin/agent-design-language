# Findings Register

## IR-5356-001: Fixed

Severity: `P1`
Lane: issue graph and lifecycle truth
Surface: `.csdlc/prepared/issues/5356/check-dependencies.rb`

Invariant: WP-18 must not start unless WP-17 is actually landed in the exact
review revision.

Evidence: before repair, the gate required `terminal.observed_sha` from
`csdlc-v2/closeout/5360.json` to be an ancestor. That receipt records
`3d4321e832a8931b5611cf59dbb566462e564836`, while `main` contains the squash
merge `dc7fd24c5b145bcb9cb28c7d3b9ca7079d7fb653` with the same tree.

Impact: the internal review was blocked even though the WP-17 content was
landed, and a future agent could either bypass the gate manually or stop review
execution incorrectly.

Remediation: the gate now accepts either the receipt SHA directly or an
ancestral first-parent PR merge commit for the same PR whose tree matches the
receipt SHA. The validation output records both `dependency_sha` and
`landed_sha`.

Disposition: `fixed`

## IR-5356-002: Fixed

Severity: `P2`
Lane: documentation and release truth
Surfaces:

- `docs/milestones/v0.91.8/README.md`
- `docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md`
- `docs/milestones/v0.91.8/WP_EXECUTION_READINESS_v0.91.8.md`
- `docs/milestones/v0.91.8/RELEASE_PLAN_v0.91.8.md`
- `docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md`

Invariant: release-tail entrypoints must match live issue truth: WP-17 is
closed; WP-18 is the active internal-review owner; WP-19 through WP-23 remain
downstream.

Evidence: the README named WP-17 as the active release-tail issue, the quality
gate said “WP-17 #5360 now,” execution readiness said WP-17 was active and
WP-18 pending, and the third-party handoff still said WP-17 was aligning docs.

Impact: external-review preparation could treat a closed predecessor as the
current owner and obscure the actual active WP-18 gate.

Remediation: the affected docs now state that WP-17 closed documentation
alignment, WP-18 is active/current, and downstream WP-19 through WP-23 remain
not complete.

Disposition: `fixed`

## IR-5356-003: Fixed

Severity: `P1`
Lane: C-SDLC v2 tooling and lifecycle
Surface: `.csdlc/prepared/issues/5356/run-validation-lane.rb`

Invariant: the mandatory WP-18 specialist-review lanes in the VPP must be
executable after WP-17 closes and the #5356 claim is amended for review
execution.

Evidence: the VPP points specialist lanes at
`ruby .csdlc/prepared/issues/5356/run-validation-lane.rb <lane>`, but the
runner accepted lane names and then always exited with a preparation-only
failure message.

Impact: WP-18 could not produce code/security/tests/docs/architecture/evidence
lane results, blocking truthful WP-19 handoff.

Remediation: the runner now dispatches deterministic lane checks, emits
structured JSON with revision, commands, warnings, findings, packet digest, and
status, and returns nonzero only on real lane findings or command failures.

Disposition: `fixed`

## IR-5356-004: Fixed

Severity: `P2`
Lane: Runtime v3 and deployment path
Surface: `adl-runtime/src/runtime_api.rs`

Invariant: Runtime API advertised endpoints must match routes actually served
by `runtime_api_router`, or the feature docs must mark unserved routes as
planned.

Evidence: `CSM_RUNTIME_API_ENDPOINTS` listed 17 canonical endpoints while
`runtime_api_router` mounted only `/v1/health`, `/v1/metrics`, and
`/v1/acip/ws`.

Impact: readiness consumers could treat unserved Runtime API surfaces as
available and receive 404 responses.

Remediation: `CSM_RUNTIME_API_ENDPOINTS` now advertises only the three served
routes, and the focused Rust test
`runtime_api_contract_advertises_only_served_routes` proves the inventory.

Disposition: `fixed`

## Residual Notes

Historical retained evidence logs include workstation paths from earlier proof
captures. This review packet does not require those paths for reviewer
execution and does not treat them as current instructions. A later publication
redaction pass may normalize historical proof logs if the release packet chooses
to expose them directly outside the repository.

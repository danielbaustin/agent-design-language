# ADL Capability Contract (ACC)

## Status

Tracked canonical ACC spec entrypoint.

This directory now uses the following framing:

- `ACC v1.0`
  - the current implemented ACC baseline
- `ACC v1.1`
  - the tracked additive evolution target for subsequent code adoption

Primary narrative specification:

- [`ADL_ACC_SPECIFICATION.md`](./ADL_ACC_SPECIFICATION.md)

Document authority hierarchy:

- [`ACC_V1.0_SPEC.md`](./ACC_V1.0_SPEC.md) is the normative baseline spec for
  the current implemented `ACC v1.0` contract
- [`ACC_V1.1_SPEC.md`](./ACC_V1.1_SPEC.md) is the normative next-version spec
  for the tracked `ACC v1.1` target
- [`ADL_ACC_SPECIFICATION.md`](./ADL_ACC_SPECIFICATION.md) is the cross-version
  narrative companion specification
- this `README.md` is the entrypoint and orientation surface

Unlike UTS, ACC is not positioned here as a public, provider-neutral standard.
ACC is intentionally scoped to ADL-governed runtimes.

Machine-readable companions:

- [`adl-spec/schemas/acc/v1.0/adl_capability_contract.v1.schema.json`](../../../adl-spec/schemas/acc/v1.0/adl_capability_contract.v1.schema.json)
- [`adl-spec/schemas/acc/v1.1/adl_capability_contract.v1_1.schema.json`](../../../adl-spec/schemas/acc/v1.1/adl_capability_contract.v1_1.schema.json)

Implementation-facing baseline reference:

- [`adl/src/acc.rs`](../../../adl/src/acc.rs)

## 1. Purpose

The ADL Capability Contract defines the runtime authority and governance layer
surrounding capability invocation inside ADL.

ACC answers questions intentionally outside the scope of UTS:

- who may invoke a capability
- under what standing
- under what authority evidence
- under what delegation lineage
- under what observability posture
- under what replay conditions
- under what review requirements
- under what execution approval semantics
- under what continuity-sensitive runtime constraints

## 2. Relationship To UTS

UTS answers:

> What is this tool?

ACC answers:

> Who may exercise that capability, under what authority, and with what
> governance constraints?

The split is intentional.

- `UTS` remains public, portable, and schema-semantic
- `ACC` remains ADL-specific, runtime-aware, and governance-specific

UTS validity does not imply:

- authority
- approval
- execution permission
- replay permission

Those concerns remain ACC and runtime-governance concerns.

See also:

- [`docs/specs/uts/README.md`](../uts/README.md)
- [`docs/explainers/UTS_AND_ACC.md`](../../explainers/UTS_AND_ACC.md)

## 3. Current Implemented Baseline (`ACC v1.0`)

The current implemented ACC baseline lives in:

- [`adl/src/acc.rs`](../../../adl/src/acc.rs)

That code currently uses:

- `ACC_SCHEMA_VERSION_V1 = "acc.v1"`
- `AdlCapabilityContractV1`
- `validate_acc_v1`

The authoritative tracked baseline for that shape is:

- [`ACC_V1.0_SPEC.md`](./ACC_V1.0_SPEC.md)
- [`adl_capability_contract.v1.schema.json`](../../../adl-spec/schemas/acc/v1.0/adl_capability_contract.v1.schema.json)

## 4. Additive Evolution Target (`ACC v1.1`)

`ACC v1.1` is the tracked next schema target for ADL runtime governance.

It is intentionally evolutionary over `ACC v1.0`:

- the current field families remain recognizable
- the anti-self-assertion rule remains intact
- the current decision / execution / replay split remains intact
- new fields are additive rather than shape-replacing

The tracked `v1.1` surfaces are:

- [`ACC_V1.1_SPEC.md`](./ACC_V1.1_SPEC.md)
- [`adl_capability_contract.v1_1.schema.json`](../../../adl-spec/schemas/acc/v1.1/adl_capability_contract.v1_1.schema.json)

Code adoption for `ACC v1.1` is a follow-on step. These docs define the target
clearly before that implementation work begins.

## 5. Scope

ACC standardizes ADL-specific governance metadata such as:

- accountable actor identity
- authority grants
- role and standing
- delegation lineage
- policy checks
- confirmation and Freedom Gate requirements
- execution approval semantics
- replay posture
- privacy/redaction posture
- visibility policy
- failure policy

Standing semantics remain runtime-specific. ACC standardizes the presence of
standing-bearing metadata, not one universal standing policy.

ACC does not standardize:

- a universal governance model
- provider-neutral orchestration semantics
- model cognition
- transport protocols
- public/general tool schemas

## 6. Summary

The clean split is:

- `UTS` is public and general
- `ACC` is ADL-specific and governance-specific
- `ACC v1.0` is the current implemented baseline
- `ACC v1.1` is the additive next target

That separation is what keeps the governed-tools model understandable,
reviewable, and evolvable.

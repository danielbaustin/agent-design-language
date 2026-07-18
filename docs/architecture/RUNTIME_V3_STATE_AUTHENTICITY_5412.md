# Runtime v3 state-authenticity and readiness disposition

Issue: #5412

Source review: #5247 / #5403

Status: review candidate

## State authenticity

Memory checkpoints use the signed `adl.runtime.memory.checkpoint.v2` wire
schema and are signed over their complete canonical payload: schema,
citizen, runtime, continuity, accepted sequence, lineage head, public facts,
private-state references, signing algorithm, and signing-key identity. Restore
verifies that signature and the active identity binding before admitting any
checkpoint state. Legacy unsigned v1 JSON remains deserializable only so the
restore boundary can reject it deterministically; it is never admitted or
silently upgraded.

Private-state projection verifies the record signature and requires the exact
record hash at the exact sequence in the supplied accepted lineage. A valid
signature without lineage admission is insufficient. Projection policy is
applied only after authenticity and membership pass.

## Real soak lane

`bash adl/tools/run_runtime_v3_guardian_soak.sh` explicitly executes the
ignored `bounded_runtime_v3_guardian_soak` test and requires a non-empty,
semantically valid execution report proving 100 cycles, 1,600 processed items,
generation 100, a pass result, and no automatic cutover. It removes any prior
report before execution so stale evidence cannot satisfy the lane. This lane is
for scheduled/release validation; it does not make the 100-cycle soak part of
ordinary PR validation.

## Source-size exception

The reproducible count from `bash adl/tools/report_runtime_v3_loc.sh` is 12,034
physical Rust lines under `adl-runtime-kernel/src` at this review candidate.
The 10,000-line goal is exceeded, but the result remains below the previously
declared 20,000-line exception ceiling.

The bounded exception is accepted only through v0.91.8 platform acceptance.
Issue #5412 owns this disposition until merge; v0.91.8 WP-14 owns the release
gate. The reduction plan is:

1. Split proof-packet construction and retained compatibility projections out
   of the runtime kernel where they do not hold runtime authority.
2. Consolidate repeated canonical signing/verification helpers without merging
   state-owner boundaries.
3. Remove opt-in v2 parity scaffolding after the reviewed Runtime v3 cutover
   and rollback window.
4. Recount at every platform-acceptance review and fail if the source exceeds
   20,000 lines or grows without an issue-linked account.

This exception does not authorize default cutover, Runtime v2 deletion, or an
increase to the 20,000-line ceiling.

## Validation

- focused checkpoint forgery and substitution tests;
- focused private-record forgery, non-membership, and lineage tests;
- explicit real guardian-soak lane and retained report;
- deterministic source recount and ceiling check.

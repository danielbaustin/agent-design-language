# #5412 — Runtime v3 state authenticity and readiness repair

## Problem

Runtime v3 currently authenticates the identity binding used during memory
restore but not the checkpoint payload itself. Private-state projection also
accepts a caller-supplied record/hash pair without proving signature validity
or membership in the accepted lineage. The real guardian soak is opt-in but is
not routed through a declared release lane, and the runtime has crossed its
10K source target without a current reviewed disposition.

## Design

1. Extend `MemoryCheckpoint` with explicit Ed25519 key identity and signature
   fields. Sign the canonical checkpoint payload with the same authority that
   owns its identity binding, and verify the full payload before any restored
   state is admitted. Bind citizen, runtime, continuity, accepted sequence,
   predecessor head, facts, and private references into the signature.
2. Require `project_private_state` to receive the accepted lineage and trusted
   key set. Verify the record signature and require the exact `(lineage,
   sequence, hash)` tuple to exist in that lineage before applying projection
   policy.
3. Add a bounded release/scheduled validation entry point that explicitly runs
   the ignored 100-cycle guardian soak and retains its report. Ordinary PR
   validation remains fast.
4. Recount `adl-runtime-kernel/src`, remove narrow duplication where safe, and
   retain either a sub-10K result or a reviewed, owned exception with a concrete
   reduction plan and ceiling.

## Proof

Focused tests must reject modified checkpoint contents, identity substitution,
sequence/head substitution, forged private-state records, unaccepted records,
and lineage substitution. Existing valid restore/projection behavior must
continue. The scheduled soak command must execute the real ignored test and
emit a truthful report. A reproducible source-count command and disposition
must close the LoC finding.

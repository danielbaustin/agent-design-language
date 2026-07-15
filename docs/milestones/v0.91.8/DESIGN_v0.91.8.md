# v0.91.8 Design

## Architecture Boundary

`v0.91.8` is a three-product acceptance milestone:

| Product | Owns | Must not own |
| --- | --- | --- |
| ADL v2 | language primitives, deterministic compiler, portable records, thin CLI, generation selector | runtime scheduling, provider execution, lifecycle governance |
| Runtime v3 | bounded execution, provider/tool ports, operations, recovery | ADL language semantics, C-SDLC cards |
| C-SDLC v2 | issue records, cards, claims, review, publication, shepherding, closeout | ADL compiler/runtime behavior |

## Planned Flow

1. Pin incumbent ADL baseline and approve clean-room design.
2. Build characterization corpus and six-primitives language core.
3. Implement deterministic compiler, bounded execution, records/signing, and
   Runtime v3/provider adapters.
4. Prove exact-revision shadow parity, opt-in soak, reversible cutover, and
   rollback.
5. Delete only reviewed and replaced incumbent surfaces.
6. Accept and deploy ADL v2, Runtime v3, and C-SDLC v2 through WP-14A.
7. Converge demos, quality gate, docs, reviews, remediation, and v0.92 handoff.

## Deletion Budget

The planning target is 90 percent reduction of the replaced incumbent ADL Rust
surface, with 80 percent as the fail-closed minimum. The actual denominator,
retained lines, and deletion eligibility must be produced by execution issues;
this design does not pre-approve deletion.

## Operational Invariants

- Stable binaries are installed outside Cargo build output.
- Generation selection is explicit and reversible.
- Exact revision, digest, and rollback evidence are retained.
- Docs and issue records distinguish planned posture from proven state.


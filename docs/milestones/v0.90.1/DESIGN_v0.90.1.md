# Design - v0.90.1

## Design Center

Runtime v2 is a persistent cognitive manifold rather than a job-scoped workflow
runner. v0.90.1 implements the smallest useful slice of that idea.

The design center is:

- one compressed execution lane prepared before runtime coding
- one bounded manifold
- two provisional citizens
- one kernel service loop
- one snapshot and wake path
- one invariant violation proof
- one operator-control surface
- one security-boundary proof

## Runtime v2 Foundation Layers

### Manifold

The manifold is the persistent world container. It owns identity registry,
clock, trace root, snapshot metadata, resource ledger, and policy boundary.

### Kernel

The kernel owns scheduling, admission, invariant checks, snapshot/rehydration,
operator controls, and security-boundary enforcement.

### Provisional Citizens

Citizens in v0.90.1 are provisional records, not true identity-bearing Gödel
agents. They provide enough stable state to test continuity, duplication
prevention, scheduling, and traceability.

### Episodes

Episodes are bounded activities inside the manifold. They are temporary. The
manifold and citizen records persist.

## Required Artifact Model

The prototype should produce reviewable artifacts such as:

- `manifold.json`
- `citizens/<id>/record.json`
- `kernel/service_loop.jsonl`
- `snapshots/<id>/snapshot.json`
- `rehydration_report.json`
- `invariants/violation_<id>.json`
- `operator/control_report.json`
- `security_boundary/proof_packet.json`

Exact paths can change during implementation, but the artifact families should
remain visible.

## Compression Enablement Boundary

v0.90.1 should not wait until release tail to improve milestone compression.
Before Runtime v2 coding starts, the milestone should align the issue-wave
template/generator, harden worktree-first execution, and refresh the execution
policy for current skill, subagent, validation, and SOR evidence rules.

These WPs are process infrastructure in service of Runtime v2. They should not
turn the milestone into a general workflow-tools release.

## Safety Boundary

The polis-defense idea matters because powerful models will find weaknesses
quickly. v0.90.1 should include one security-boundary proof, but it should not
center the milestone on red/blue/purple ecology.

Security here is an invariant and operator-safety requirement. It is not the
main Runtime v2 thesis.

## Later-Band Boundary

v0.91 adds moral, emotional, kindness, humor, and wellbeing substrate.

v0.92 adds first true birthday, identity/capability rebinding, richer memory,
and continuity validation.

v0.90.1 prepares those milestones without claiming them.

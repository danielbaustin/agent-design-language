# v0.92 Activation Test Map from v0.91.8

WP-21 `#5362` owns the handoff rows below after WP-14A accepts the platform.
These rows do not block WP-14A itself.

| v0.92 input | v0.91.8 source | Required evidence |
| --- | --- | --- |
| Platform install | #5345, #5343, #5384 | Stable install and selector receipt |
| Runtime execution and canonical ingress | #5341, #5361, #5591 | Runtime v3 consumer plus guardian-launched live ingress and continuity proof |
| Reasoning graphs, loops, affect control, and adaptive cognition | #5592, #5107 | Live Runtime v3 proof or explicit retained/deferred/non-runtime disposition with safe non-claims |
| Governed runtime operations | #5589 | Production adapter, identity/private-state, provider/scheduler, checkpoint, and lifelog proof |
| Secure local/remote access and HTML Observatory | #5590 | Configuration-driven HTTPS, authenticated HTTP/WebSocket consumption, guardian, telemetry, and rollback |
| Unity Observatory demo proof | #5354, #4739, #4741, #5332 | WP-15 project/port/editor alignment and batch proof, or an explicit Unity tooling disposition |
| Lifecycle governance | #5358 | C-SDLC v2 typed lifecycle proof |
| Capability envelope | #4761 | Evidence-backed capability envelope |
| Memory Palace | #4760, #5007 | Acceptance boundary, handoff evidence, and ADR acceptance |
| Birth witnesses/receipt | #4762 | Auditable receipt package |
| Public launch docs | #4758, #4763 | Claim-bounded launch docs |
| Adaptive Learning DAG | #5107 | Queued prerequisites and non-claims |
| Distributed workcell | #5497, #5501 | One reviewed live workcell and bounded context/output-contract proof |
| Canonical feature preservation | #5594, #5362, #5355 | Every relevant canonical feature-list row has an owner and terminal disposition; absent Runtime v3 implementation is a blocker before Runtime v2 deletion |
| Release-tail handoff routing | WP-21, WP-21A #5355, WP-22 | Feature-list truth, next-milestone handoff alignment, and review before v0.92 consumption |

If any row lacks evidence, `v0.92` must consume it as a blocker or explicit
non-claim.

Current consumption truth: #5408 is closed/remediated via PR #5419, but #4906
remains retained blocked-with-evidence unless separately dispositioned.

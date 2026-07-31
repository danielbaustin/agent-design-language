# Runtime v3 Acceptance Readiness Design

Issue #5361 is the Runtime v3 acceptance umbrella for v0.91.8. This packet
prepares its contract and dependency graph only. It does not execute parity,
deployment, soak, cutover, or release acceptance.

## Authority

- #5361 owns integrated Runtime v3 acceptance and retained proof synthesis.
- #5591 owns Parity-A: kernel continuity and canonical ingress.
- #5592 owns Parity-B: reasoning, adaptive learning, and governed cognition.
- #5589 owns Parity-C: governed operational adapters.
- #5590 owns Parity-D: secure access, Observatory, guardian, and rollback.
- #5501 owns the live multi-agent workcell output-contract proof consumed here.
- #5384 consumes completed #5361 evidence; it does not precede #5361.

## Dependency Order

Architecture authority #5336 must integrate before Parity-A starts. Parity-A
#5591 then precedes Parity-B, Parity-C, and Parity-D. Those three children may
execute concurrently only after their protected-path manifests prove they are
disjoint. Acceptance synthesis waits for all four parity children, #5341
consumer integration, #5349 provider/tool adapters, #5350 exact-revision
shadow parity, and #5501 live workcell proof.

## Acceptance Boundary

Runtime v3 is accepted only at an exact revision with:

- guardian-launched canonical ingress and lifecycle proof;
- deterministic checkpoint, replay, resume, and state-authenticity proof;
- reasoning graph, loop, affect-control, and adaptive-learning parity proof;
- secure local and remote HTTPS access with no hard-coded address;
- Observatory consumption, health telemetry, and graceful pressure shutdown;
- provider, tool, governed-operation, and multi-agent consumer proof;
- exact-revision #5350 shadow-parity proof with every mismatch dispositioned;
- rollback and recovery evidence;
- current line-count, module-growth, dependency-audit, test-count, CI, and
  exact-revision review evidence.

Unsupported GPU or remote-provider claims remain explicit non-claims unless a
later child issue supplies reviewed evidence. AWS is outside this packet and is
not authorized.

## Preparation Completion

This preparation lane completes when all six typed cards, the dependency
diagram, protected paths, and validation plan are doctor-clean and reviewed.
The issue remains open and bound; no acceptance result is claimed.

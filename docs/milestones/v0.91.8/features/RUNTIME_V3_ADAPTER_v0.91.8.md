# Runtime v3 Adapter

The Runtime v3 adapter connects ADL plans and engine events to Runtime v3 while
preserving Runtime v3 as the execution authority.

Retained proof comes from `#5341` and `#5361`, with provider/tool adapter
support from `#5349`. Runtime v3 acceptance also consumed the WP-10A live
workcell output-contract proof from `#5501`.

Runtime v3 parity was owned under `#5361` in dependency order:

1. `#5591` proves kernel lifecycle, canonical ingress, continuity, replay, and
   graceful pressure shutdown.
2. After that ingress contract is reviewed, `#5592` proves reasoning graphs,
   bounded loops, adaptive learning, affect reasoning-control, and governed
   cognition.
3. `#5589` replaces degraded governed operations adapters.
4. `#5590` proves configuration-driven secure access, guardian supervision,
   authenticated HTML Observatory consumption, telemetry routing, soak, and
   rollback.

WP-16 records `#5591`, `#5592`, `#5589`, and `#5590` as working-code outcomes
and retains the Runtime v3 locked all-target suite as passing evidence. Runtime
v2 remains relevant only where a later consumer explicitly depends on a retained
or deferred feature row; this adapter document does not widen Runtime v3 into
ADL v2 language semantics or C-SDLC v2 lifecycle authority.

No runtime deployment claim is valid without exact revision, install,
operation, and rollback evidence.

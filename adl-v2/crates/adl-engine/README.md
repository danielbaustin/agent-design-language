# ADL portable engine

`adl-engine` consumes the inert `adl_compiler::ExecutionPlan` contract and
implements a pure, deterministic, bounded plan-level state machine. Hosts drive
it with explicit logical ticks, typed completions, and cancellation intents;
the engine returns ordered provider, tool, and cancellation effects.

The crate performs no I/O, clock reads, sleeping, process or thread control,
networking, persistence, provider access, or Runtime integration. Checkpoints
are canonical quiescent byte snapshots that must be persisted by a separate
host-owned adapter.

State-dependency edges are retained as typed bindings. Before dispatch the
engine recursively replaces `@state:<name>` inputs with the successful source
output. `application/json` outputs become JSON values and `text/*` outputs
become UTF-8 strings; other media types fail closed. Canonical resolved inputs
are part of request and idempotency identity.

Limits cover plan and policy bytes, ready and in-flight work, attempts,
request and completion envelopes, completion/cancellation cardinality per
turn, total turn-input bytes, retained outputs, events, checkpoint bytes, and
logical turns. Resume validates graph reachability and an exact structured
completion journal, including node, attempt, request sequence, and resolved
input identity.

The public flow is:

1. construct exact `EngineLimits` and an `EnginePolicy` for every plan node;
2. call `Engine::new` to validate and admit the plan;
3. call `Engine::turn` with monotonically increasing logical ticks;
4. deliver typed completions using the emitted request identity and attempt;
5. call `Engine::checkpoint` only when `Engine::is_quiescent` is true;
6. call `Engine::resume` with the exact same plan, policy, and limits.

Provider and governed-tool adapters are deliberately outside this crate.

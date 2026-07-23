# Characterization parity

The compiler boundary is mapped against the fixtures landed with #5339.

| Fixture class | Compiler treatment |
|---|---|
| six-primitives, map, sequential | Applicable: parsed by `adl-language`, compiled, and covered by resolution/order/replay tests |
| negative syntax, duplicate keys, unknown fields/references/state/cycles | Language-owned rejection; compiler revalidation preserves deterministic diagnostics for typed documents |
| branch-a, branch-b, fork-join legacy `patterns` | Explicit non-input: rejected and not representable by the landed language model; no silent compiler skip |
| schema-only language cases | Language-owned; compiler consumes the typed document after schema/parser authority |

This mapping is intentional scope evidence, not a claim that the compiler owns
legacy parsing or pattern syntax.

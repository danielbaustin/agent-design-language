# ADL compiler

`adl-compiler` is a pure deterministic lowering layer. It accepts an
`adl_language::AdlDocument`, validates it again, resolves language references,
and returns inert `ExecutionPlan` data. It performs no I/O, scheduling,
provider calls, retries, lifecycle work, or execution.

The landed language contract represents sequential and concurrent workflows
and saved-state dependencies. Legacy top-level `patterns` are not compiler
inputs: `adl-language` deliberately rejects and cannot represent them. Pattern
syntax requires a future typed language contract before compiler support.

Plan nodes carry declared input/output ports, prompt data, and bounded source
provenance so a later executor does not need to reread the source document.
Plan ordering uses ordered collections and a lexical Kahn traversal. Node IDs
use an explicit `node_v1_` prefix and SHA-256 over the execution-plan contract
plus a versioned, domain-separated, length-delimited semantic tuple containing
a digest of effective node semantics: resolved refs and model, effective sorted
tools, prompt, declared ports, step inputs, and output. Normalized-equivalent or
unused declaration changes do not churn IDs. Source digests use the language
crate's canonical bytes. Declared port order is preserved. All graph and
aggregate input limits are explicit through `CompilerLimits` and enforced
during construction.

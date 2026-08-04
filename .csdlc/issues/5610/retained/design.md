# Issue 5610 design

Status: approved for bounded execution.

Normalize llvm-cov filenames with platform-independent POSIX lexical semantics
after slash unification. Permit parent traversal only when it remains beneath an
already identified owned source root, then match only canonical `/adl/src/` or
`/adl-runtime/src/` paths. Reject traversal that crosses the lexical repository
prefix or owned source root. Preserve every existing provenance, metric,
duplicate, total-recomputation, atomic-write, and coverage gate.

# C-SDLC v2 Gate 1 Recommendation

Issue: #5228
Recommendation: `proceed`
Scope: authorize preparation of Gate 2 only after #5228 review, green checks,
merge, and closeout

## Findings Before Recommendation

- V1's obvious control-plane core is already at least 21,438 lines across eight
  files, before the wider shell/Python and compatibility surface is counted.
- The repository has 469 tracked top-level shell tools and 145 Python tools;
  ownership is mixed, but the lifecycle layer references too much of this
  surface.
- The heuristic C-SDLC-adjacent Rust test count is 1,227.
- Seven representative installed v1 owner binaries occupy 291.96 MiB and link
  the large main product graph.
- Normal v1 doctor is network-bound: five completed samples had a 113.85-second
  median and 128.11-second nearest-rank p95 while using less than 0.6 seconds
  CPU per sample.
- Individual prompt-structure checks remain fast (0.01–0.02 seconds), showing
  that a small typed card engine can preserve prompt rigor without broad lanes.
- A standalone seven-binary design can retain the core C-SDLC ideas without
  depending on ADL Runtime or copying v1 implementation.

## Recommendation

Proceed to Gate 2 after this issue is reviewed and settled. Gate 2 should build
only the clean-room state engine, automated card editor, and offline doctor.
It should not start binding, GitHub publication, or later PVF execution work.

Gate 2 must prove:

- standalone Cargo dependency boundaries;
- lifecycle enums and transitions;
- all-six-card automatic construction;
- typed semantic edits through `csdlc-edit`;
- Markdown.rs mdast and Strum alignment;
- atomic values/card/index/audit commits;
- design and diagram readiness checks;
- normalized cold/warm construction, binary-size, and local latency budgets;
- a focused suite that remains within the cumulative test budget.

## Residual Gate 2 Obligations

- Run the normalized isolated v1/v2 clean construction comparison.
- Establish 21-sample local doctor p95.
- Establish deterministic temporary-repository init planning latency.
- Select the smallest readable state-machine implementation after evaluating
  COTS options; reject macro-heavy or opaque code generation.
- Resolve Markdown AST-compatible serialization without text surgery.

## Non-Authorization

This recommendation does not authorize default cutover, v1 deletion, Gate 3+
implementation, or selection of the final three cutover-sample issues.

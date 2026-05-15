# Feature Proof Coverage - v0.91.2

## Status

Active milestone coverage map. `v0.91.2` is open for execution, and rows may
advance from planned to in_flight to landed as their owning WPs move from
active branch work to closeout.

## Coverage Rule

Each feature should eventually have one truthful proof route:

- executable benchmark, report, or harness
- bounded demo packet
- docs/product/report packet
- explicit quality/review/release surface

## Feature Coverage Map

| Feature | WP | Intended Route | Status |
| --- | --- | --- | --- |
| UTS + ACC multi-model benchmark | WP-02, WP-03 | harness + comparison report | planned |
| Runtime/test-cycle recovery | WP-04, WP-05 | WP-04 runtime recovery report plus WP-05 coverage ergonomics evidence | landed |
| CodeFriend productization | WP-06 | review packet workflow package + product-report template + evidence rules | landed |
| Review heuristics and demos | WP-07 | heuristics promotion packet + bounded review demo packet + fixture review outputs + review-quality checklist | in_flight |
| Workspace CMS bridge | WP-08, WP-09, #3091, #3092 | bounded demo + native CMS capability packet + live safety package + live bounded execution report | in_flight |
| Code modernization | WP-10 | dry-run modernization demo | planned |
| Speculative decoding | WP-11 | prototype/evaluation packet + non-claim record | planned |
| Repo visibility follow-on | WP-12 | manifest/linkage packet + source-map evidence | planned |
| Publication program | WP-13 | backlog/process packet | planned |
| General intelligence paper packet | WP-14 | claim/citation/review packet | planned |
| Rustdoc/doc cleanup | WP-15 | cleanup report + doc patch set | planned |
| Workflow guardrails | WP-16 | guardrail proofs + runbook | planned |
| Demo/proof convergence | WP-17 | demo matrix and proof coverage | planned |

## Non-Claims

- no feature in this milestone is landed yet through this document alone
- this file is not evidence that any feature implementation has landed

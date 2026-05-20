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
| Workspace CMS bridge | WP-08, WP-09, #3091, #3092, #3093, #3094 | bounded demo + native CMS capability packet + live safety package + live bounded execution report + live content-card roundtrip report + project-ready operational package + `.adl` to GWS migration plan | landed baseline plus active hardening |
| Code modernization | WP-10 | modernization interaction plan + dry-run evidence + reversibility/review policy + demo packet | active bounded packet |
| Speculative decoding | WP-11 | `review/speculative_decoding/speculative_decoding_prototype_report.json` + `review/speculative_decoding/speculative_decoding_prototype_packet.md` | implemented |
| Repo visibility follow-on | WP-12 | tracked manifest packet + linkage report + reviewer-navigation packet | implemented |
| Publication program | WP-13 | tracked backlog packet + review-gates packet + Godel/GHB backlog note | implemented |
| General intelligence paper packet | WP-14 | tracked claim/citation packet + review handoff + residual-risk register + next-authoring-steps packet + canonical paper-repo migration truth | implemented |
| Rustdoc/doc cleanup | WP-15 | cleanup report + doc patch set | implemented |
| Workflow guardrails | WP-16 | `adl/tools/workflow_guardrails.sh` + `adl/tools/test_workflow_guardrails.sh` + workflow-guardrails runbook/proof packet | implemented |
| Demo/proof convergence | WP-17 | demo matrix and proof coverage | planned |

## Non-Claims

- this file alone is not evidence that a feature implementation has landed
- implemented rows must remain backed by their named proof routes and merged
  issue/PR evidence

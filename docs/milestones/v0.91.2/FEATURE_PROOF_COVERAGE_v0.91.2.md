# Feature Proof Coverage - v0.91.2

## Status

Release-tail coverage map. `v0.91.2` is now in Sprint 4 convergence mode, so
rows should reflect landed truth where the owning WPs have closed and use
explicit `in_progress` wording only for the current release-tail issue.

## Coverage Rule

Each feature should eventually have one truthful proof route:

- executable benchmark, report, or harness
- bounded demo packet
- docs/product/report packet
- explicit quality/review/release surface

## Feature Coverage Map

| Feature | WP | Intended Route | Status |
| --- | --- | --- | --- |
| UTS + ACC multi-model benchmark | WP-02, WP-03 | harness + comparison report | landed |
| Runtime/test-cycle recovery | WP-04, WP-05 | WP-04 runtime recovery report plus WP-05 coverage ergonomics evidence | landed |
| CodeFriend productization | WP-06 | review packet workflow package + product-report template + evidence rules | landed |
| Review heuristics and demos | WP-07 | heuristics promotion packet + bounded review demo packet + fixture review outputs + review-quality checklist | landed |
| Workspace CMS bridge | WP-08, WP-09, #3091, #3092, #3093, #3094 | bounded demo + native CMS capability packet + live safety package + live bounded execution report + live content-card roundtrip report + project-ready operational package + `.adl` to GWS migration plan | landed baseline plus post-sprint hardening |
| Code modernization | WP-10 | modernization interaction plan + dry-run evidence + reversibility/review policy + demo packet + retained execution patch/log/command proof | landed bounded demo |
| Speculative decoding | WP-11 | `bash adl/tools/demo_v0912_speculative_decoding_showcase.sh` + `review/speculative_decoding/speculative_decoding_prototype_report.json` + `review/speculative_decoding/speculative_decoding_prototype_packet.md` | implemented |
| Repo visibility follow-on | WP-12 | tracked manifest packet + linkage report + reviewer-navigation packet | implemented |
| Publication program | WP-13 | tracked backlog packet + review-gates packet + Godel/GHB backlog note | implemented |
| General intelligence paper packet | WP-14 | tracked claim/citation packet + review handoff + residual-risk register + next-authoring-steps packet + canonical paper-repo migration truth | implemented |
| Rustdoc/doc cleanup | WP-15 | feature doc + tracked doc patch set + WP-17 convergence packet route | implemented |
| Workflow guardrails | WP-16 | `bash adl/tools/demo_v0912_workflow_guardrails_showcase.sh` + `adl/tools/workflow_guardrails.sh` + `adl/tools/test_workflow_guardrails.sh` + workflow-guardrails runbook/proof packet | implemented |
| Demo/proof convergence | WP-17 | demo matrix + proof coverage map + `review/demo_proof_convergence/DEMO_PROOF_CONVERGENCE_PACKET_v0.91.2.md` | landed |
| Coverage / quality gate | WP-18 | `QUALITY_GATE_v0.91.2.md` + `review/quality_gate/QUALITY_GATE_PACKET_v0.91.2.md` + `bash adl/tools/demo_v0912_quality_gate.sh` | in_progress |

## Non-Claims

- this file alone is not evidence that a feature implementation has landed
- implemented rows must remain backed by their named proof routes and merged
  issue/PR evidence

## WP-17 Convergence Rule

WP-17 should not invent new implementation proof. Its job is to:

- converge the milestone's strongest real demo/proof routes into one truthful
  showcase map
- strengthen rows that were still phrased like planning surfaces after the
  owning WPs landed
- name explicit deferrals when a feature is better represented by a bounded
  packet than by a new live demo

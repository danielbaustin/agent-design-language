# v0.91.8 Quality Gate

| Gate | Owner | Required proof |
| --- | --- | --- |
| Docs/YAML/link validity | WP-17 #5360, both WP-18 passes #5356/#5791, WP-19 #5357, WP-20 #5363, WP-21 #5362, and WP-21A #5355 are closed; #5489/#5383/#5594 are historical inputs | Focused docs validation, YAML parse, canonical feature-list crosswalk, inventory checks, review-handoff preflight, and both internal-review packet validations |
| Architecture denominator | #5336 | Baseline packet and approved design |
| Characterization corpus | #5337 | Fixture review and deterministic replay plan |
| Core behavior | #5338, #5339, #5340, #5342 | Focused Rust tests and canonical fixture proof |
| Runtime/C-SDLC boundaries | #5341, #5358, #5361 | Exact-revision consumer and lifecycle proof |
| Distributed workcell acceptance | #5497, #5499, #5498, #5500, #5502, #5501 | Live workcell coordination, task-adapter observation/output contracts, and umbrella convergence proof |
| Parity and rollback | #5350, #5344, #5343 | Shadow parity, soak, rollback, selector transaction |
| Deletion safety | #5346, #5347 | Eligibility manifest and post-deletion validation |
| Integrated quality gate | #5351 | Passed at `2e9d2dd7c`; see `evidence/wp16/QUALITY_GATE.md` and `evidence/wp16/ISSUE_OUTCOME_AUDIT.md` |
| Release-tail truth | #5360, #5356, #5791, #5357, #5363, #5362, #5355, #5359, #5348 | Documentation alignment, first and final internal review, external review, remediation/preflight, handoff truth, and ceremony |

The WP-16 integrated quality gate is satisfied by retained evidence, not by
this planning file alone. Release readiness remains unsatisfied until the
release-tail truth row passes.

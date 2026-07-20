# v0.91.8 Quality Gate

| Gate | Owner | Required proof |
| --- | --- | --- |
| Docs/YAML/link validity | #5489, with #5383 as closed historical setup authority | Focused docs validation, YAML parse, canonical inventory checks, and review-handoff preflight |
| Architecture denominator | #5336 | Baseline packet and approved design |
| Characterization corpus | #5337 | Fixture review and deterministic replay plan |
| Core behavior | #5338, #5339, #5340, #5342 | Focused Rust tests and canonical fixture proof |
| Runtime/C-SDLC boundaries | #5341, #5358, #5361 | Exact-revision consumer and lifecycle proof |
| Distributed workcell acceptance | #5497, #5499, #5498, #5500, #5502, #5501 | Live workcell coordination, task-adapter observation/output contracts, and umbrella convergence proof |
| Parity and rollback | #5350, #5344, #5343 | Shadow parity, soak, rollback, selector transaction |
| Deletion safety | #5346, #5347 | Eligibility manifest and post-deletion validation |
| Integrated release gate | #5351 | Full platform quality packet |

No gate is satisfied by this planning file alone.

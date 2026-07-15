# C-SDLC v2 Gate 5 design

Gate 5 makes bounded pre-publication review canonical state rather than an informal PR convention. `csdlc-review assign` binds reviewer, assigner, and bounded scope under the live issue claim; the tool computes the exact revision from Git HEAD plus scoped tracked and untracked content. Assignment clears any prior review. `csdlc-review record` revalidates the claim under the transaction lock, requires matching generation/digest and assignment, and atomically writes the SRP projection plus richer index evidence.

Each finding retains id, severity, summary, actionability, in-scope status, disposition, fix revision, and follow-up route. A fixed actionable in-scope finding requires a fix revision. An out-of-scope finding requires both `out_of_scope` disposition and a route; it cannot disappear from evidence. Residual risks remain explicit.

The publication guard is pure and local. It accepts only completed evidence with reviewer/scope identity, an exact current revision, and no unresolved actionable in-scope or unrouted out-of-scope findings. The existing state transition guard calls the same evaluator before Published or MergeReady. Review assignment/recording does not contain GitHub transport and cannot push, publish, merge, close, or change PR state.

## Revision invalidation

A new substantive revision is assigned for a fresh review, atomically clearing old evidence. Assignment requires a clean substantive commit, so a dirty review cannot enter the lifecycle. The publication transition independently recomputes the scoped Git revision, so source changes invalidate review without a caller assertion. A stranded `reviewed` record can use typed `csdlc-review recover` to return to `implemented` without deleting audit history, then be reassigned and reviewed at the finalized commit. The sole exception is `review_metadata_only_v1`: the tool recomputes normalized changed paths between the named commits and accepts only `.csdlc/review/` or `.csdlc/evidence/`. Traversal, source, design, manifest, command, and product paths cannot use the exception.

## Automated cards

Review scope, result, findings, in-scope truth, fix revision, route, and residual risk are applied atomically to SRP through the deterministic markdown.rs projection path used by Gate 2. The full evidence remains indexed for publication policy and audit without hand-editing Markdown.

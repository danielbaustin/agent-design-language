# #5466 Design

Add an explicit `csdlc-publish reconcile-merged` route for the narrow case where a PR is already merged after its head advanced beyond the recorded publication revision.

The operator must first recover the issue to implementation and record exact review evidence for the final clean head. Reconciliation then accepts an explicit PR number, observes that PR from GitHub, and fails closed unless repository, base, head branch, issue linkage, final head SHA, merged state, title, and body match the current reviewed publication intent. The resulting publication evidence records the final reviewed revision and merged remote state. Existing readiness and closeout operations remain authoritative after reconciliation.

The normal draft publication path is unchanged. No AWS-backed validation is used.

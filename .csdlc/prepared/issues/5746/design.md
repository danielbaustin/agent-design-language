# Design: v0.91.8 retained terminal projection wave

Issue #5746 materializes already-retained C-SDLC v2 terminal authority into tracked repository projections.

The Git-common terminal receipt is the sole source for each included issue. The typed `csdlc-closeout reconcile-terminal` transaction regenerates the issue index, six cards, audit stream, and retained design/diagram without changing product implementation. The aggregate branch contains only `.csdlc/issues/<issue>/**` paths.

Eligibility is fail-closed: a target must have a valid receipt, matching repository and initialization identity, terminal disposition, claim release, and internally valid cards. Missing receipts, invalid dispositions, stale reviews, missing PVF evidence, dirty foreign worktrees, and non-portable retained artifacts remain excluded and explicitly reported.

Publication closes only #5746. It does not close #5595, declare the milestone released, repair excluded issues, or authorize worktree pruning.

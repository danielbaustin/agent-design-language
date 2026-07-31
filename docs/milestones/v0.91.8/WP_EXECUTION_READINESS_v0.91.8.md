# v0.91.8 Work Package Execution Readiness

| WP | Ready to start after | Current planned state |
| --- | --- | --- |
| WP-01 | Operator start of #5594 | In progress; closes only after reviewed canonical reconciliation and sprint readiness proof |
| WP-02 | WP-01 #5594 closeout and #5336 stale-worktree recovery | Not ready: recover unpublished #5336 authority before regeneration or execution |
| WP-03 | WP-02 denominator | Waiting |
| WP-04 | WP-02 and WP-03 | Waiting |
| WP-05 | WP-04 | Waiting |
| WP-06 | WP-05 | Waiting |
| WP-07 | WP-04 and WP-06 | Waiting |
| WP-08 | WP-06 and WP-07 | Waiting |
| WP-09 | WP-06 and WP-08 | Waiting |
| WP-10 | WP-04 through WP-09 | Waiting |
| WP-10A | WP-09 provider/adapter freeze; #5499 -> #5498 -> (#5500 and #5502) -> #5501 -> #5497 | Waiting |
| WP-11 | WP-03, WP-10, and completed WP-10A live proof | Waiting |
| WP-12 | WP-11 | Waiting |
| WP-13 | WP-14A through WP-17 complete; deletion manifests proven disjoint; run immediately before #5356 | Deferred |
| WP-14A | #5358, #5361, #5344, and #5343 accepted at exact revisions | Ready for focused acceptance preparation |
| WP-15-WP-23 | Prior closeout gates | Waiting |

## Opening Card-Factory Wave

After WP-01 closes, the four writable slots are #5336 recovery, #5337 card
preparation, #5358 acceptance-card preparation, and #5361 acceptance-card
preparation. No slot is implementation-ready merely because it is allocated.

Runtime v3 Parity-A #5591 is prepared only after #5336 architecture authority
is integrated. Parity-B #5592, Parity-C #5589, and Parity-D #5590 remain
dependency-blocked behind Parity-A and require disjoint protected paths before
concurrent execution.

Readiness must be refreshed from live issue and PR truth before starting each
work package.

# v0.92 Work Package Execution Readiness

The issue wave is open. A row marked `prepared next` has satisfied milestone
ordering but still requires issue-specific design approval, claim binding, and
owner validation before implementation.

| Work | Ready after | Current readiness |
| --- | --- | --- |
| WP-01 | v0.91.8 release and planning prerequisites | Active under #5817 |
| WP-01B | WP-01 publication | Prepared next; issue #5818 |
| WP-02 | WP-01, WP-01B, reviewed #5815 plan and organization readiness | Dependency-gated; issue #5819 |
| WP-02A | WP-02 | Dependency-gated; issue #5801 |
| WP-03 | WP-02A | Dependency-gated; issue #5820 |
| WP-04 | WP-03 | Dependency-gated; issue #5821 |
| WP-05 | WP-02A | Dependency-gated; issue #5822 |
| WP-06 | WP-02A | Dependency-gated; issue #5823 |
| WP-07 | WP-01, WP-05 | Dependency-gated; issue #5824 |
| WP-08 through WP-19 | Dependencies in WBS and issue-wave YAML | Open and card-initialized; not execution-authorized by WP-01 |
| WP-20, WP-21, WP-21A | Integrated feature/demo proof | Open and card-initialized; release-tail work |
| WP-22 through WP-30 | Dependencies in WBS and issue-wave YAML | Open and card-initialized; review/release/publication tail |

## Start Rule

Before any child implementation starts, its owner must verify live dependencies,
complete issue-specific design approval, bind a dedicated worktree and claim,
create the issue-bound goal, and run only the focused proof declared by its VPP.
WP-01 does not grant product completion or merge authority to child issues.

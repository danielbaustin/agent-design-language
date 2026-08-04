# WP-21 Findings-First Sprint Review

Status: remediation complete; exact-revision review pending

Issue: #5352

Baseline: `c34f0c9412495039a6374f7ce88fa39e34bb5042`

This review covers the eight merged WP-21 child issues, the final #5558
predecessor repair, and the #5352 consumption handoff. It does not execute
v0.92, claim a birthday event, or treat asynchronous typed closeout as an
integration gate.

## Findings

### Remediated findings

1. **Exact-head review and publication freshness.** The first #5352 review
   covered the substantive handoff, but lifecycle metadata subsequently moved
   the branch head. The issue was recovered to `implemented`; publication now
   waits for one final review of the complete remediation revision.
2. **Canonical ACIP WebSocket route.** The local CSM API advertised Runtime
   v3's `/v1/acip/ws` path but accepted only `/acip/ws`. The remediation makes
   `/v1/acip/ws` canonical and retains `/acip/ws` as an explicit compatibility
   alias. Both focused route tests pass.
3. **Current lifecycle proof.** The retained implemented packet and doctor
   evidence were regenerated after remediation and describe implemented
   generation 7 with a passing typed doctor report.

### Superseded or nonblocking findings

- #4758's retained launch-readiness log says its exact review was pending. Its
  later typed review record and merged PR are the current authority; the stale
  line remains immutable historical evidence.
- #4759, #4760, #4763, and #5007 retain nonterminal typed projections after
  their GitHub PRs merged and issues closed. This is asynchronous lifecycle
  reconciliation debt, not a #5352 integration blocker.
- #4762 retained validation used a binary from another issue worktree. Its
  merged artifact remains historical evidence, but the machine-local path is
  not portable execution proof for v0.92.
- The eight child changes merged before #5558 even though the planning order
  placed the predecessor first. #5352 validates the actual #5558 merge as an
  ancestor of its baseline and records the ordering variance without rewriting
  history.
- #5362 and #5363 are independent umbrella or parent tracking issues. This
  review does not claim their closure and does not widen #5352 into their work.

## Verified Integration Matrix

| Issue | PR | Reviewed head | Merge | GitHub issue state |
| --- | --- | --- | --- | --- |
| #4758 | #5739 | `c9b5c625ccfb17b1a75fd3a1a93f4810baf4a3e2` | `038f718c377549db21df3a1eb08402867beb2cd5` | closed |
| #4759 | #5738 | `32957a21a3fc3fc8a8efb3c3c6ad198db9b0ddd7` | `471db0c35dc34c2497682993378948481bdfa213` | closed |
| #4760 | #5740 | `9719252262913351144a20adf0affb7ed4b5480d` | `d3dbfb31ba4bd53f4166ee5e09da2a8b9f89968e` | closed |
| #4761 | #5741 | `8c3ef0336570238d26eab0fd49a9a2ff9c1a0c09` | `97d4036e0b5c21786d13cd1301b33038d95e3b98` | closed |
| #4762 | #5744 | `d736baca1c82c6ca9b770678ff2c04ce44458fc9` | `021be8e33b486d9b66886ff299c20607ed8a071a` | closed |
| #4763 | #5734 | `313268e09b8d9906f61b0e12ac05cce4deea1e3c` | `d2b19b3aba092aff871b315d60590731e730cb4a` | closed |
| #5007 | #5743 | `426d0a53fb2b7b0be571b236ca5d0a248b32e1f8` | `1bd6f73b1c449ffd132ad9a34c739e16c39186c2` | closed |
| #5107 | #5742 | `8bf36c9d214a54212e7c483fb29872e9be9e92b3` | `b77d020c5c5274e7b64b6ef8f36eed888f34fb4c` | closed |
| #5558 | #5749 | `033b28cffa6bdf191b1d013aa5a730ce7b10d9df` | `c34f0c9412495039a6374f7ce88fa39e34bb5042` | closed |

Each listed merge is required to be ancestral to the #5352 publication
baseline. The handoff validator binds each concern to its issue, PR, reviewed
head, merge, and consumption surface.

## Required Final Proof

- focused canonical and legacy ACIP route tests;
- handoff contract and dependency-ancestry validators;
- current implemented lifecycle validator and typed doctor;
- diff hygiene;
- one findings-first exact-revision review with no unresolved actionable
  findings;
- PR #5751 with `Closes #5352` and required checks green.

Typed closeout follows GitHub closure asynchronously and is not part of this
review's merge decision.

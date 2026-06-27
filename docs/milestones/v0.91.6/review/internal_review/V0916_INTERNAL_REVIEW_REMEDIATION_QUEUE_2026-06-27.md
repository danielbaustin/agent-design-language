# v0.91.6 Internal Review Remediation Queue

Date: `2026-06-27`
Owner issue: `#4582`
Status: `queued_for_release_tail`

| Finding | Severity | Owner route | Required disposition before release ceremony |
| --- | --- | --- | --- |
| Release-tail docs still carry stale WP-13 state after WP-13 merged | P1 | `#3981` remediation/final preflight, or narrow docs-truth follow-up before `#3980` | Fix or explicitly record why external review may consume the stale labels safely. |
| Pre-v0.92 activation cannot proceed from v0.91.6 evidence alone | P1 | `#3981`, `#3982`, `#3983` | Consume burn-down checklist; keep v0.92 activation blocked unless all blockers are resolved/routed. |
| `pr finish` can erase numbered SRP findings from machine-readable SOR facts | P1 | tooling follow-up under `#3981` or v0.91.7 C-SDLC/tooling queue | Fix numbered-list parsing and add regression coverage before relying on SOR facts for release-tail automation. |
| Repo-native PR inventory is still incomplete for internal-review use | P2 | tooling follow-up under `#3981` or v0.91.7 tooling queue | Add typed PR list/search inventory or document an approved interim evidence path. |
| Milestone checklist mixes old forward-checklist residue with current review truth | P2 | `#3981` | Convert stale unchecked rows into complete/routed/blocked/deferred classifications. |
| C-SDLC adoption remains operational but not fully fail-closed by default | P2 | `#3982` / v0.91.7 C-SDLC operationalization | Carry residuals into v0.91.7 rather than treating them as release blockers. |
| Live AWS runtime heartbeat attempts consume sequence numbers even when no signal is published | P2 | runtime follow-up under `#3981` or v0.91.7 runtime integration | Prevent blocked live heartbeat attempts from advancing cursor state; add regression coverage. |
| Internal-review plan duplicate DEMO_MATRIX entry | P3 | opportunistic docs cleanup | Optional cleanup; not a release blocker. |
| Large workflow-control modules increase review and regression cost | P3 | v0.91.7 refactoring/design issue | Route decomposition of the highest-churn control-plane modules along stable seams. |

## Routing Rule

Do not close a finding by narrative. Each accepted finding must be fixed,
explicitly deferred with owner and risk, or routed to a named issue/milestone.

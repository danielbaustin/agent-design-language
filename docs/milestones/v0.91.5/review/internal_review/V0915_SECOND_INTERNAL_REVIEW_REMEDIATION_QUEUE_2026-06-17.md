# v0.91.5 Second Internal Review Remediation Queue

Date: `2026-06-17`
Source review issue: `#3923`
Queue status: `proposed`

## Purpose

This queue converts the second-pass internal review findings into bounded
remediation candidates. It does not create GitHub issues by itself and does not
claim release readiness.

## Proposed Issue Candidates

| Order | Severity | Area | Candidate title | Source finding | Notes |
| --- | --- | --- | --- | --- | --- |
| 1 | `P1` | tooling / GitHub validation | `[v0.91.5][tools] Fix PR validation latest-check handling for superseded cancelled runs` | PR `#3933` validates as `cancelled` even though merged with latest green required checks. | Should be fixed before external review because it directly undermines the first-pass remediation claim for `#3891`. |
| 2 | `P1` | tooling / finish truth | `[v0.91.5][tools] Re-stage SOR/output truth after finish writes validation evidence` | `pr finish` stages paths before later SOR/output mutations. | Should be fixed before external review because it can create stale-card commits. |
| 3 | `P1` | docs / release truth | `[v0.91.5][docs] Normalize Sprint 4 release-tail docs after closed remediation and WP-15` | `SPRINT`, `WBS`, and `QUALITY_GATE` disagree with live `#3899` / `#3579` truth and the WP-15 input omits active `#3923`. | Docs-only; should be fast and focused. |
| 4 | `P1` | security / evidence hygiene | `[v0.91.5][review] Redact raw prompts from OpenRouter matrix proof artifacts` | Tracked OpenRouter lane request JSON stores full prompt text despite the v0.91.5 observability contract forbidding raw prompt exposure in reviewer/public projections. | Should be fixed before external review. |
| 5 | `P2` | review evidence | `[v0.91.5][review] Preserve closed-issue card-truth blocker evidence in tracked packet` | Quality gate cites ignored local `.adl` card paths that are not present in the review worktree. | Either emit redacted excerpts or rerun a repo-native audit that writes tracked evidence. |
| 6 | `P2` | review evidence | `[v0.91.5][review] Refresh first-pass remediation baseline after closed queue` | First-pass queue/register still say `#3896` / `#3899` are active. | May combine with candidate 3 if the issue remains small. |
| 7 | `P2` | security / evidence hygiene | `[v0.91.5][review] Store OpenRouter provider outputs as excerpts and digests in machine-readable proof` | Tracked OpenRouter lane result JSON stores full provider output even though excerpt-only provider success records are supported. | Can be paired with candidate 4 if the code and evidence changes stay bounded. |
| 8 | `P3` | evidence hygiene | `[v0.91.6][tools] Replace historical manual GitHub provenance with ADL-native proof where useful` | Rust refactor closeout still preserves raw `gh pr view` commands as historical validation provenance. | Defer unless external-review polish budget allows. |
| 9 | `P3` | portability / fixtures | `[v0.91.6][provider] Sanitize private LAN endpoint fixtures before they feed durable proof packets` | Demo/provider tests preserve a literal private LAN Ollama endpoint in generated artifacts. | Not a current packet blocker; route for portability hygiene. |

## Recommended Execution Order

1. Fix PR validation behavior first because it is executable control-plane truth.
2. Fix `pr finish` staging/SOR truth before relying on closeout cards.
3. Redact OpenRouter raw prompt artifacts before external review.
4. Normalize release-tail docs and stale review packets.
5. Preserve durable closed-issue/card evidence.
6. Defer the historical `gh` and private-LAN fixture cleanups unless they become
   reviewer-blocking.

## Non-Claims

- This queue does not claim any finding is fixed.
- This queue does not close `#3923`, `#3576`, or Sprint 4.
- This queue does not replace WP-18 final remediation/preflight.

## Suggested Routing

- Candidates 1, 2, and 4 should stay in `v0.91.5` if external review depends on
  repo-native validation truth.
- Candidates 3, 5, 6, and 7 should stay in `v0.91.5` if the goal is a clean
  external review packet.
- Candidates 8 and 9 can move to `v0.91.6` if time is tight.

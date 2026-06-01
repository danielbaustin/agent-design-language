# v0.91.4 Internal Review Remediation Issues

Status: routed
Date: 2026-05-31
Source review: `V0914_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-05-31.md`
Parent review issue: `#3366`
Remediation wave: `#3368`

## Created Issues

| Issue | Scope | Source findings |
| --- | --- | --- |
| `#3542` `[v0.91.4][PVF][quality] Fix release-policy test execution and CI coverage` | PVF policy test behavior and CI coverage | `WP16-F001`, `WP16-F004` |
| `#3543` `[v0.91.4][release][docs] Reconcile release truth before external review` | Release-facing documentation truth and routing | `WP16-F002`, `WP16-F005`, `WP16-F006`, `WP16-F011` |
| `#3544` `[v0.91.4][provider][security] Harden provider identity and Gemini credential diagnostics` | Provider identity classification and credential-safe diagnostics | `WP16-F003`, `WP16-F010` |
| `#3545` `[v0.91.4][evidence][docs] Normalize review evidence paths and redaction surfaces` | Review evidence portability, redaction, and path hygiene | `WP16-F007`, `WP16-F008`, `WP16-F012`, `WP16-F013` |
| `#3546` `[v0.91.4][WildClawBench][quality] Pin replayability boundary for benchmark evidence` | WildClawBench replayability boundary | `WP16-F009` |

## Routing Notes

- The issue wave preserves the highest severity from each grouped finding.
- Code-level blockers are isolated in `#3542` and `#3544`.
- Release-truth and external-review evidence cleanup are separated so docs remediation can proceed without masking provider/PVF defects.
- `WP16-F009` remains separate because it concerns benchmark evidence replayability rather than generic path redaction.

## Non-claims

- These issues route the findings; they do not claim remediation is complete.
- WP-16 should not be closed as clean until the routing is reflected truthfully in closeout records.

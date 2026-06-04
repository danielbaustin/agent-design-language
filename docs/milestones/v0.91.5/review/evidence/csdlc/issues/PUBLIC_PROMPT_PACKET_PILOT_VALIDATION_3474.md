# Public Prompt Packet Pilot Validation

Issue: `#3474`

## Summary

This note records focused validation for the v0.91.5 public C-SDLC prompt
packet pilot under `docs/milestones/v0.91.5/review/evidence/csdlc/issues/`.

The pilot packet set is suitable as reviewer-facing evidence, with one
important gate-design finding: the current prompt-template structure validator
is a useful diagnostic, but it is not yet a complete public-packet validation
gate for completed cards.

## Packet Set

| Issue | Lane | Packet status |
| --- | --- | --- |
| `#3472` | Tooling/process | closed issue / merged PR |
| `#3473` | Docs/local state | closed issue / merged PR |
| `#3562` | Review-provider lane | closed issue / merged PR |

## Validation Results

| Check | Result | Notes |
| --- | --- | --- |
| Manifest JSON parse | `PASS` | All three `manifest.json` files parse as JSON. |
| Public packet files present | `PASS` | Each packet includes `README.md`, `manifest.json`, and `cards/{sip,stp,spp,srp,sor}.md`. |
| Public redaction scan | `PASS` after metadata normalization | The `#3472` packet initially recorded the explicit source worktree path used during recovery export; the manifest and README now use repo-relative `.adl/v0.91.5/tasks/...` provenance. |
| `git diff --check` | `PASS` | Whitespace hygiene passed. |
| Current-template structure diagnostic | `PARTIAL` | SIP and STP cards pass. Several completed SPP/SRP/SOR cards fail because the current structure schemas still expect bootstrap/pre-run locked prose. |

## Structure Diagnostic Details

The diagnostic current-template structure check found these expected
compatibility gaps:

- `#3473` `SPP` contains issue-specific assumptions where the current SPP
  schema expects bootstrap locked lines.
- `#3562` `SPP` records completed plan status where the current SPP schema
  expects all `Codex Plan` entries to remain `pending`.
- `#3472` and `#3562` `SRP` records contain review-result truth that differs
  from pre-run review-prompt locked prose or frontmatter shape.
- All three `SOR` records contain completed execution artifacts where the
  current SOR schema expects bootstrap scaffold text.

These are not redaction failures and not evidence that the packet contents
should be rewritten. They show that `#3475` needs a completed-card-aware gate
that separates:

- locked template structure that must remain stable,
- editable issue content,
- lifecycle truth that is expected to change after execution,
- and public-output hygiene such as host-path and secret scans.

## Non-Claims

- This pilot does not claim all historical `.adl` cards are ready for public
  publication.
- This pilot does not claim current-template structure validation is sufficient
  as the final public-packet gate.
- This pilot does not revalidate runtime behavior for the selected issues.

## Follow-Up Routing

Route reusable gate design and completed-card-aware validation semantics to
`#3475`: `[v0.91.5][WP-07][quality] Add public prompt packet validation and redaction gates`.

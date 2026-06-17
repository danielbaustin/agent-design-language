# v0.91.5 Second Internal Review Live Issue State Snapshot

Date: `2026-06-17`
Issue: `#3923`

## Purpose

Retain the live issue-state checks used by the second internal review so the
review packet does not rely only on chat memory.

## Snapshot

| Issue | Observed state | Observed note |
| --- | --- | --- |
| `#3899` | `closed` | First internal-review remediation umbrella closed at `2026-06-17T09:08:47Z`. |
| `#3703` | `closed` | Logging mini-sprint umbrella closed at `2026-06-16T19:16:58Z`. |
| `#3845` | `closed` | Tools remediation sprint umbrella closed at `2026-06-16T19:16:58Z`. |
| `#3579` | `closed` | WP-15 docs/review alignment closed at `2026-06-17T16:13:58Z`. |
| `#3576` | `open` | WP-16 internal review umbrella remains open. |
| `#3923` | `open` | Second-pass internal review execution issue remains open while this packet is being prepared. |
| `#3574` | `open` | Sprint 4 umbrella remains open. |
| `#3580` | `open` | External / third-party review remains open and depends on WP-16. |

## Command Surface Used

```bash
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3899 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3703 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3845 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3579 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3576 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3923 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3574 --json
GH_TOKEN=<redacted-from-operator-approved-secret-source> bash adl/tools/pr.sh issue view 3580 --json
```

## Redaction Boundary

- The GitHub token value was read from the operator-approved key file and was
  not printed.
- This packet retains issue state only; it does not retain full issue bodies.

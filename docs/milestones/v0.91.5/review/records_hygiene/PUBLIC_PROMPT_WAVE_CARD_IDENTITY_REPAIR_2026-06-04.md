# Public Prompt Wave Card Identity Repair

## Status

Completed during `v0.91.5` issue `#3660`.

This is a local lifecycle-record hygiene repair for the public C-SDLC prompt
packet wave. It does not implement the public prompt packet exporter, archive
policy, pilot packet, validation gate, or mini-sprint closeout work.

## Problem

Issues `#3472` through `#3476` were reallocated into `v0.91.5`, but local
active C-SDLC records still existed under the old `v0.91.4` identity paths.
That made the conductor block the public-card wave before normal execution:

- `#3472` reported duplicate local issue identities.
- `#3473`, `#3474`, `#3475`, and `#3476` reported missing canonical
  `v0.91.5` source prompts.

## Repair Performed

The stale active records were archived out of conductor discovery into:

- `.adl/archive/records-hygiene/issue-3660/v0.91.4/bodies/`
- `.adl/archive/records-hygiene/issue-3660/v0.91.4/tasks/`
- `.adl/archive/records-hygiene/issue-3660/cards/`

Canonical `v0.91.5` local bundles were then regenerated through `pr init` for:

| Issue | Canonical v0.91.5 bundle |
| --- | --- |
| `#3472` | `.adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/` |
| `#3473` | `.adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/` |
| `#3474` | `.adl/v0.91.5/tasks/issue-3474__v0-91-5-wp-06-docs-pilot-public-c-sdlc-prompt-packets-and-reviewer-index/` |
| `#3475` | `.adl/v0.91.5/tasks/issue-3475__v0-91-5-wp-07-quality-add-public-prompt-packet-validation-and-redaction-gates/` |
| `#3476` | `.adl/v0.91.5/tasks/issue-3476__v0-91-5-wp-03-mini-sprint-public-c-sdlc-prompt-records-and-local-adl-archive-transition/` |

## Validation

`pr init` validated SIP and SOR contracts for all regenerated bundles.

Focused `pr doctor` checks then verified that each affected issue reaches the
normal `SPP` lifecycle stage instead of failing with duplicate or missing
identity errors:

| Issue | Doctor result | Active stage |
| --- | --- | --- |
| `#3472` | `BLOCK` for expected SPP readiness, not identity failure | `SPP` |
| `#3473` | `BLOCK` for expected SPP readiness, not identity failure | `SPP` |
| `#3474` | `BLOCK` for expected SPP readiness, not identity failure | `SPP` |
| `#3475` | `BLOCK` for expected SPP readiness, not identity failure | `SPP` |
| `#3476` | `BLOCK` for expected SPP readiness, not identity failure | `SPP` |

## Non-Claims

- This does not complete `#3472` through `#3476`.
- This does not make public prompt records publishable by itself.
- This does not delete historical card evidence; it moves stale local records
  out of active conductor identity paths.
- This does not touch `#3657`.

## Follow-On

The public-card wave can now proceed by normal lifecycle order. Each issue
still needs SPP review/readiness before execution binding.

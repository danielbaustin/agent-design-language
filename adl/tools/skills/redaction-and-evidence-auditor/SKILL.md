---
name: redaction-and-evidence-auditor
description: Audit CodeBuddy review packets and final reports for privacy, publication, and evidence-boundary risks before customer-facing use.
---

# Redaction And Evidence Auditor

Audit a CodeBuddy packet, specialist review output, or final report for privacy,
publication, and evidence-boundary risks. This skill is a safety gate, not a reviewer,
not a publisher, and not a remediation workflow.

Use this skill after `repo-packet-builder` and before any customer-facing,
public, or cross-team report is shared.

## Quick Start

1. Confirm the artifact root or report path to audit.
2. Confirm the intended audience:
   - `local_only`
   - `customer_private`
   - `public_candidate`
3. Run the deterministic helper when local filesystem access is available:
   - `scripts/audit_review_packet.py <artifact-root> --out <report-root>`
4. Review the emitted redaction report.
5. Stop after the audit. Hand unsafe packets back to the owning skill or
   operator for explicit remediation.

## Required Inputs

At minimum, gather:

- `artifact_root`
- `mode`
- `audience`
- `policy`

Supported modes:

- `audit_packet`
- `audit_report`
- `audit_review_bundle`
- `pre_publication_gate`

Useful policy fields:

- `privacy_mode`
- `publication_intent`
- `allow_internal_urls`
- `allow_private_paths`
- `allow_source_excerpts`
- `max_excerpt_lines`
- `stop_before_mutation`

If there is no concrete artifact root or report file, stop and report
`not_run`.

## Workflow

### 1. Establish Publication Context

Record:

- artifact or report root
- intended audience
- whether publication is requested
- privacy mode from the packet manifest, when present
- whether `publication_allowed` is already false

Do not upgrade publication status. This skill can only preserve or downgrade
publication readiness.

### 2. Scan For Unsafe Evidence

Look for:

- likely secrets and tokens
- private host paths
- internal URLs and private network addresses
- provider exposure gaps
- raw prompt or tool-argument leakage
- excessive source excerpts
- artifact paths that are not repo-relative when the packet is meant to be
  portable

Keep findings evidence-bounded. Do not print full secret values in the report.

### 3. Classify Findings

Use these severities:

- `blocker`: must block publication or customer-facing sharing
- `warning`: needs review before sharing
- `info`: useful caveat but not blocking

Use these statuses:

- `pass`: no blocking or warning findings
- `partial`: warnings exist but no blockers
- `fail`: one or more blockers exist
- `not_run`: missing inputs or unreadable target

### 4. Emit Audit Artifacts

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/redaction-audit/
```

Required artifacts:

- `redaction_report.md`
- `redaction_report.json`

Optional artifacts:

- `blocked_publication_note.md`

## Output Expectations

Default output should include:

- status
- publication recommendation
- files scanned
- blocker, warning, and info counts
- redacted finding samples
- evidence-boundary notes
- required remediation owner or downstream skill

Use the output contract in `references/output-contract.md`.

## Stop Boundary

Stop after producing the audit report.

Do not:

- edit, redact, or delete customer artifacts
- mutate the reviewed repository
- publish the report externally
- create GitHub issues or PRs
- rerun specialist review lanes
- claim that a packet is secure beyond the scanned surfaces
- expose full secret values in findings

## CodeBuddy Integration Notes

This skill is designed to sit after `.adl/docs/TBD/codebuddy_ai/REVIEW_PACKET_SPEC.md`
packet construction and before reports following
`.adl/docs/TBD/codebuddy_ai/REVIEW_TEMPLATE_STANDARD.md` are shared. If those
planning docs are not present in a downstream checkout, use this skill's
`references/output-contract.md` as the local review-safety contract.

Deferred automation:

- CI gating for customer-facing report publication.
- Model-assisted over-disclosure checks for long prose sections.
- Provider-specific secret classifiers beyond the deterministic patterns in the
  helper script.

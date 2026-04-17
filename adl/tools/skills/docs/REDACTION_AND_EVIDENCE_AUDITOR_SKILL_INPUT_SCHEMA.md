# Redaction And Evidence Auditor Skill Input Schema

Schema id: `redaction_and_evidence_auditor.v1`

Use this schema for structured invocations of the
`redaction-and-evidence-auditor` skill.

## Required Top-Level Fields

- `skill_input_schema`: must equal `redaction_and_evidence_auditor.v1`
- `mode`: one of the supported modes
- `artifact_root`: packet, review bundle, or report root to audit
- `audience`: intended audience
- `policy`: explicit audit and mutation policy

## Supported Modes

- `audit_packet`
- `audit_report`
- `audit_review_bundle`
- `pre_publication_gate`

## Audience Values

- `local_only`
- `customer_private`
- `public_candidate`

## Required Policy Fields

- `privacy_mode`
- `publication_intent`
- `allow_internal_urls`
- `allow_private_paths`
- `allow_source_excerpts`
- `max_excerpt_lines`
- `stop_before_mutation`

`stop_before_mutation` must be true. This skill audits only; it does not edit
or redact artifacts in place.

## Example

```yaml
skill_input_schema: redaction_and_evidence_auditor.v1
mode: pre_publication_gate
artifact_root: .adl/reviews/codebuddy/20260417-120000-repo-packet
audience: customer_private
policy:
  privacy_mode: customer_private
  publication_intent: customer_report
  allow_internal_urls: false
  allow_private_paths: false
  allow_source_excerpts: true
  max_excerpt_lines: 80
  stop_before_mutation: true
```

## Output

The skill writes:

- `redaction_report.md`
- `redaction_report.json`
- optional `blocked_publication_note.md`

See `redaction-and-evidence-auditor/references/output-contract.md`.


# Public Prompt Records Export v0.91.6

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-18`
- Owner: ADL maintainers
- Status: `wp_04_execution_active`
- Related issues: `#3969`, `#4002`, `#4003`, `#4004`, `#4005`, `#4006`
- Prior baseline: [PUBLIC_PROMPT_RECORDS_v0.91.5.md](../../v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md)
- Export contract proof note: [PUBLIC_PROMPT_RECORDS_EXPORT_CONTRACT_4002.md](../review/public_prompt_records/PUBLIC_PROMPT_RECORDS_EXPORT_CONTRACT_4002.md)
- Redaction/publication safety proof note: [PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md](../review/public_prompt_records/PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md)

## Template Rules

This is a feature-scope contract and bridge record. It defines what public prompt
records must look like, what source-selection policy the bridge intends to
preserve, and which parts of that policy are already proven versus still owned
by later enforcement lanes. It also defines the current redaction and
publication-safety posture for public prompt records. It does not by itself
prove public indexing readiness, security signoff, or release approval.

## Purpose

Define the `v0.91.6` contract for exporting public prompt records from local
C-SDLC authoring state without promoting local `.adl` records into public truth,
while keeping enforcement claims aligned with the current exporter and validator
surfaces and making publication-safety rules explicit.

## Context

`v0.91.5` established the first exporter and validator surface for public
prompt packets. `v0.91.6` must now make the export shape, provenance,
source-selection rules, and redaction/publication-safety boundaries explicit
enough that later validation, indexing, and security-review work can build on a
stable contract instead of inferring one from implementation details.

Public prompt records remain projections of local authoring state. The canonical
editable lifecycle state continues to live in local `.adl/<version>/tasks/...`
bundles and their issue-local sources.

## Coverage / Ownership

This feature owns:
- export packet shape and required files
- deterministic source-selection policy for the bridge
- provenance and public metadata requirements
- redaction and publication-safety policy for public prompt records
- ineligible source and record categories
- a truthful proof note for a representative selected-record export
- a truthful proof note for allowed, redacted, and refused publication classes
- explicit separation between already-proven enforcement and later hardening

This feature does not yet own:
- public indexing publication
- security approval for publication
- final public release workflow
- exporter hardening beyond what current repository evidence already proves
- full threat modeling or CAV approval for public-record publication

## Overview

The public prompt-record bridge has two distinct layers:

1. local authoring truth
   - issue-local `.adl` task bundles, source prompts, and review/output cards
2. public projection truth
   - tracked public packets under `docs/milestones/<version>/review/evidence/csdlc/issues/...`
   - other explicit reviewer/public projections whose redaction posture is
     separately recorded and reviewed

`v0.91.6` makes that boundary explicit. Export is intended to operate only from
approved, issue-bounded authoring sources, but current enforcement is layered:

- the exporter already proves packet shape generation and source-card content
  hygiene checks on the selected bundle
- the validator already proves packet-shape, tracker-metadata, and public
  provenance requirements for accepted packets
- current public prompt packet export uses `refuse_not_rewrite` redaction mode
  for source cards and public packet text
- explicitly redacted reviewer/public projections may exist in other tracked
  proof surfaces, but they require an explicit review-safe record rather than
  silent exporter rewriting of local `.adl` source cards
- stricter source-admission checks for explicit override paths remain a policy
  requirement for the bridge, but are not yet fully proven as export-time
  rejection behavior by `#4002`

## Design

### Contract baseline

The public prompt-record contract remains milestone-scoped and issue-bounded:

- one issue bundle exports to one public packet root
- the exported packet is reviewable repo content, not mutable local state
- exported cards preserve lifecycle order: `sip`, `stp`, `spp`, `srp`, `sor`
- public packets must preserve provenance instead of rewriting lifecycle truth
- later gates may reject export candidates; they must not reinterpret exported
  packet identity ad hoc

### Source selection policy

Target source policy for the bridge:

- the canonical default source is `.adl/<version>/tasks/issue-<number>__<slug>/`
- an explicit source override is acceptable only when it still represents one
  concrete issue/card bundle with reviewable repo-relative provenance
- `.worktrees/...`, `.codex/...`, temp/scratch paths, host-local absolute paths,
  and partial or identity-ambiguous bundles are not acceptable public-record
  provenance

Current proven enforcement boundary:

- `#3472` proves the happy-path default-bundle export and packet shape
- the current validator proves accepted public packets must satisfy the tracked
  public provenance and tracker metadata contract
- `#4002` does not claim that the exporter already rejects every bad explicit
  `--source` override before packet generation

Review implication:

- source selection is part of the bridge contract now
- full exporter-side hard rejection for all invalid override classes remains a
  tooling-hardening concern until separately proven

### Export shape contract

Each public prompt packet must contain:

- `manifest.json`
- `README.md`
- `cards/sip.md`
- `cards/stp.md`
- `cards/spp.md`
- `cards/srp.md`
- `cards/sor.md`

The packet root is milestone-scoped:

- `docs/milestones/<version>/review/evidence/csdlc/issues/issue-<number>-<slug>/`

The exported packet must remain tracker-reviewable and machine-parseable. The
manifest is the machine contract; the packet README is the reviewer-facing
summary.

### Required public metadata / provenance

The public packet must preserve, at minimum, the current stable v1 packet
metadata:

- milestone/version identity
- issue number
- normalized issue slug
- tracker provider: `github`
- GitHub issue URL for the exported work item
- tracker identity separated from tracker-agnostic work-item identity
- source task-bundle provenance as a repo-relative path
- exported card file paths as repo-relative public paths
- lifecycle card membership and ordering

Required provenance rules:
- provenance must point to the local authoring bundle, not to a worktree path
- public paths must point only into tracked packet content
- public metadata must be stable enough for later validation and indexing work
- packets that do not satisfy the current tracker/provenance contract are not
  accepted as valid public prompt packets

### Redaction and publication-safety policy

Public prompt records are not public by default just because they are tracked.
A record is publishable only when it fits one of the allowed public-safety
classes below and carries the corresponding proof posture.

Allowed public-safety classes:

- `allowed_verbatim_packet`
  - a public prompt packet exported from one issue-local bundle after the
    exporter and validator safety checks pass
  - current packet mode is `refuse_not_rewrite`
- `allowed_redacted_projection`
  - an explicit reviewer/public projection whose tracked artifact stores only
    redacted excerpts, digests, counts, references, or other bounded public-safe
    metadata instead of raw sensitive content
  - this class requires an explicit review-safe record rather than implicit
    exporter rewriting
- `refused_public_record`
  - any candidate packet or projection that contains disallowed sensitive
    content, invalid public provenance, unresolved template residue, or
    publication-unsafe local/private material

Redaction/refusal rules for public prompt packets and related reviewer/public
projections:

- token values, private keys, secret-like tokens, and credentials must not
  appear in durable public artifacts
- local host paths, worktree paths, temp paths, and `.codex` scratch paths must
  not appear in durable public artifacts
- private provider logs, raw provider payloads, and raw prompt bodies are not
  acceptable public prompt packet content by default
- raw local `.adl` material is not publishable unless it has been converted into
  an approved public packet or another explicitly reviewed public projection
- unresolved template markers, incomplete scaffolds, or identity-ambiguous
  bundles are refused
- no issue in this feature lane may silently rewrite local source `.adl` cards
  to make them public-safe
- temp-path and `.codex` scratch-path treatment are carried here as bridge
  policy and broader repo redaction expectations; `#4003` does not claim
  packet-specific refused examples for those exact classes yet

Explicit exception rule:
- when a public-facing artifact must expose transformed or excerpted material
  instead of refusing it, the artifact must carry an explicit review-safe record
  describing the redaction posture, such as excerpt-plus-digest evidence,
  redacted marker fields, or an equivalent bounded public-safe projection
- such exceptions are not implicit exporter behavior and must remain separately
  reviewable

### Non-exportable / non-publishable categories

The following categories are not acceptable as public prompt-record inputs,
public prompt-record provenance, or durable reviewer/public prompt-record
artifacts unless separately redacted into an explicit reviewed projection:

- local execution scratch or transient worktree state
- private machine-local paths or checkout-specific absolute paths
- temp-file provenance
- secret-like tokens, private-key markers, or similar sensitive material
- unresolved template placeholders or incomplete rendered-card scaffolds
- raw provider logs or other private provider diagnostic payloads
- raw prompt text or raw model output where a redacted reviewer/public
  projection is the appropriate surface instead
- local archive/disposition inventories that are not themselves approved public
  review packets
- any source bundle whose identity, lifecycle completeness, or provenance
  cannot be verified deterministically at review/validation time
- unreviewed `.adl` material outside the bounded issue/task-bundle contract

Current enforcement note:
- the exporter already refuses obvious unsafe card content
- the validator already refuses invalid public packet provenance and tracker
  shape for accepted packets
- the current refused examples directly prove host-local absolute-path,
  worktree-provenance, missing-tracker-url, and secret-marker failures
- broader source-admission hardening remains future tooling work unless and
  until separately proven
- temp-path and `.codex` scratch-path handling remain policy-level here unless
  and until a later issue adds explicit public-packet refusal proof
- richer redacted public projections are allowed only when their own review-safe
  packet or proof note makes the transformation explicit

### Relationship to later WP-04 issues

- `#4004` owns validation and public indexing behavior on top of this contract
- `#4005` owns security review and CAV handoff expectations
- `#4006` owns end-to-end distribution proof and closeout truth

## Proof Surfaces

### Export-shape proof surface

`#4002` uses the current tracked `#3472` exported public packet as its
representative selected-record proof surface because that packet now
demonstrates the repaired happy-path exporter output shape and repo-relative
provenance without inventing a toy fixture. See:

- [PUBLIC_PROMPT_RECORDS_EXPORT_CONTRACT_4002.md](../review/public_prompt_records/PUBLIC_PROMPT_RECORDS_EXPORT_CONTRACT_4002.md)
- [#3472 exported packet README](../../v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/README.md)
- [#3472 exported packet manifest](../../v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/manifest.json)

What this proof surface establishes:
- packet root shape and required files
- lifecycle-card export membership/order
- repo-relative provenance on the current tracked packet after the `#3474`
  metadata-normalization repair
- GitHub tracker identity separation from work-item identity

What this proof surface does not establish by itself:
- full exporter-side rejection of every invalid explicit source override
- indexing readiness
- security/publication approval

### Redaction/publication-safety proof surface

`#4003` uses three bounded evidence classes:

- allowed packet example:
  - `#3472` exported packet README/manifest/SOR
- refused example:
  - `adl/src/cli/tooling_cmd/tests/public_prompt_packet.rs`
  - validator-side provenance/tracker refusal in `public_prompt_packet_validate_fails_closed_on_manifest_and_redaction_drift`
- redacted reviewed projection example:
  - [LOGGING_VALIDATION_REDACTION_PROOF_4000.md](../review/logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md)
  - tracked excerpt-plus-digest reviewer/public artifacts such as the OpenRouter matrix packet and `#3951` remediation evidence

See [PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md](../review/public_prompt_records/PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md).

## Determinism and Constraints

- local `.adl` state remains the authoring surface
- public packets are projections, not source-of-truth replacements
- default-bundle export and accepted-packet provenance must remain
  reviewer-auditable
- public prompt packet export stays `refuse_not_rewrite` unless a later issue
  explicitly changes and proves that behavior
- later lanes may tighten enforcement, but they must not silently change this
  issue's declared packet identity or publication-safety contract
- later lanes must distinguish exporter-side guarantees from validator-side
  guarantees instead of collapsing them into one implied control
- redacted reviewer/public projections must be explicit and reviewable rather
  than silent source rewrites

## Integration Points

- [../FEATURE_DOCS_v0.91.6.md](../FEATURE_DOCS_v0.91.6.md)
- [../README.md](../README.md)
- [../DECISIONS_v0.91.6.md](../DECISIONS_v0.91.6.md)
- [../../v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md](../../v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md)

## Validation

This issue's proof is documentation/provenance proof, not runtime proof.
Validation for `#4003` should confirm:

- the feature doc defines one stable export shape
- eligible and ineligible source classes are explicit as bridge policy
- the doc distinguishes exporter-proven behavior from validator-proven behavior
- tracker metadata and provenance requirements match the current accepted packet
  contract
- redaction/publication-safety classes and exception rules are explicit
- non-publishable categories are explicit
- the allowed, redacted, and refused example surfaces are linked to real repo
  evidence
- logging validation/redaction proof from WP-03 is consumed where relevant

This issue does not prove all explicit-override rejection behavior, public
indexing, or security approval. Those remain separately tracked and must not be
claimed from `#4003` alone.

## Acceptance Criteria

- The export shape is explicitly documented.
- Source-selection policy is explicit, including what is target policy versus
  what is already proven by exporter or validator surfaces.
- Required public metadata and provenance fields are documented truthfully for
  the current accepted packet shape.
- Redaction/publication-safety classes, exception rules, and non-publishable
  categories are documented truthfully.
- A real selected-record proof surface is linked for contract review.
- Real allowed, redacted, and refused evidence surfaces are linked for
  publication-safety review.
- The doc states clearly that local `.adl` remains the authoring surface and
  public packets are projections.
- The doc does not overclaim exporter-side rejection guarantees that are not yet
  proven.

## Risks

- If later lanes change packet identity implicitly, validation and indexing
  surfaces may drift.
- If provenance is weakened, public packets may accidentally encode worktree or
  host-local assumptions.
- If exporter-side and validator-side enforcement are conflated, reviewers may
  overtrust source-selection guarantees that are not yet fully hardened.
- If redacted projections are allowed without explicit review-safe records,
  public artifacts may quietly drift into raw prompt or provider-output
  exposure.
- If issue-body or packet-input paths drift from the current proof surfaces,
  later docs may cite stale redaction evidence.

## Future Work

- codify validation/public indexing contract (`#4004`)
- codify security review/CAV routing (`#4005`)
- prove bounded distribution and closeout (`#4006`)
- harden exporter-side source-admission checks if later implementation work is
  needed to enforce the full bridge policy at export time
- route stale issue-body proof-input paths for cleanup when they no longer match
  the live milestone review packets

## Notes

`v0.92` may consume public prompt records only after export, redaction,
validation, indexing, evidence, and security-review boundaries are all
completed truthfully.

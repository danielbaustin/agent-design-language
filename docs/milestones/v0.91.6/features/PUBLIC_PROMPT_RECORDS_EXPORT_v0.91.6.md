# Public Prompt Records Export v0.91.6

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-18`
- Owner: ADL maintainers
- Status: `wp_04_execution_active`
- Related issues: `#3969`, `#4002`, `#4003`, `#4004`, `#4005`, `#4006`
- Prior baseline: [PUBLIC_PROMPT_RECORDS_v0.91.5.md](../../v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md)
- Contract proof note: [PUBLIC_PROMPT_RECORDS_EXPORT_CONTRACT_4002.md](../review/public_prompt_records/PUBLIC_PROMPT_RECORDS_EXPORT_CONTRACT_4002.md)

## Template Rules

This is a feature-scope contract and bridge record. It defines what public prompt
records must look like, what source-selection policy the bridge intends to
preserve, and which parts of that policy are already proven versus still owned
by later enforcement lanes. It does not by itself prove redaction clearance,
public indexing readiness, security signoff, or release approval.

## Purpose

Define the `v0.91.6` contract for exporting public prompt records from local
C-SDLC authoring state without promoting local `.adl` records into public truth,
while keeping enforcement claims aligned with the current exporter and validator
surfaces.

## Context

`v0.91.5` established the first exporter and validator surface for public
prompt packets. `v0.91.6` must now make the export shape, provenance, and source
selection rules explicit enough that later redaction, validation, indexing, and
security-review work can build on a stable contract instead of inferring one
from implementation details.

Public prompt records remain projections of local authoring state. The canonical
editable lifecycle state continues to live in local `.adl/<version>/tasks/...`
bundles and their issue-local sources.

## Coverage / Ownership

This feature owns:
- export packet shape and required files
- deterministic source-selection policy for the bridge
- provenance and public metadata requirements
- ineligible source and record categories
- a truthful proof note for a representative selected-record export
- explicit separation between already-proven enforcement and later hardening

This feature does not yet own:
- redaction implementation beyond defining non-exportable categories
- public indexing publication
- security approval for publication
- final public release workflow
- exporter hardening beyond what current repository evidence already proves

## Overview

The public prompt-record bridge has two distinct layers:

1. local authoring truth
   - issue-local `.adl` task bundles, source prompts, and review/output cards
2. public projection truth
   - tracked public packets under `docs/milestones/<version>/review/evidence/csdlc/issues/...`

`v0.91.6` makes that boundary explicit. Export is intended to operate only from
approved, issue-bounded authoring sources, but current enforcement is layered:

- the exporter already proves packet shape generation and source-card content
  hygiene checks on the selected bundle
- the validator already proves packet-shape, tracker-metadata, and public
  provenance requirements for accepted packets
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

### Non-exportable categories

The following categories are not acceptable as public prompt-record inputs or
public prompt-record provenance:

- local execution scratch or transient worktree state
- private machine-local paths or checkout-specific absolute paths
- temp-file provenance
- secret-like tokens, private-key markers, or similar sensitive material
- unresolved template placeholders or incomplete rendered-card scaffolds
- local archive/disposition inventories that are not themselves approved public
  review packets
- any source bundle whose identity, lifecycle completeness, or provenance
  cannot be verified deterministically at review/validation time

Current enforcement note:
- the exporter already refuses obvious unsafe card content
- the validator already refuses invalid public packet provenance and tracker
  shape for accepted packets
- broader source-admission hardening remains future tooling work unless and
  until separately proven

### Relationship to later WP-04 issues

- `#4003` owns redaction and publication-safety policy on top of this contract
- `#4004` owns validation and public indexing behavior on top of this contract
- `#4005` owns security review and CAV handoff expectations
- `#4006` owns end-to-end distribution proof and closeout truth

## Dry-Run / Fixture Proof Surface

`#4002` uses the real `#3472` exported public packet as its representative
selected-record proof surface because that packet already demonstrates the
current happy-path exporter output shape and repo-relative provenance without
inventing a toy fixture. See:

- [PUBLIC_PROMPT_RECORDS_EXPORT_CONTRACT_4002.md](../review/public_prompt_records/PUBLIC_PROMPT_RECORDS_EXPORT_CONTRACT_4002.md)
- [#3472 exported packet README](../../v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/README.md)
- [#3472 exported packet manifest](../../v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/manifest.json)

What this proof surface establishes:
- packet root shape and required files
- lifecycle-card export membership/order
- repo-relative provenance on the default bundle path
- GitHub tracker identity separation from work-item identity

What this proof surface does not establish by itself:
- full exporter-side rejection of every invalid explicit source override
- redaction completeness
- indexing readiness
- security/publication approval

## Determinism and Constraints

- local `.adl` state remains the authoring surface
- public packets are projections, not source-of-truth replacements
- default-bundle export and accepted-packet provenance must remain reviewer-auditable
- later lanes may tighten enforcement, but they must not silently change this
  issue's declared packet identity contract
- later lanes must distinguish exporter-side guarantees from validator-side
  guarantees instead of collapsing them into one implied control

## Integration Points

- [../FEATURE_DOCS_v0.91.6.md](../FEATURE_DOCS_v0.91.6.md)
- [../README.md](../README.md)
- [../DECISIONS_v0.91.6.md](../DECISIONS_v0.91.6.md)
- [../../v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md](../../v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md)

## Validation

This issue's proof is documentation/provenance proof, not runtime proof.
Validation for `#4002` should confirm:

- the feature doc defines one stable export shape
- eligible and ineligible source classes are explicit as bridge policy
- the doc distinguishes exporter-proven behavior from validator-proven behavior
- tracker metadata and provenance requirements match the current accepted packet
  contract
- non-exportable categories are explicit
- a real selected-record export packet exists and matches the documented
  happy-path shape

This issue does not prove all explicit-override rejection behavior. Redaction,
public indexing, and security approval remain separately tracked and must not be
claimed from `#4002` alone.

## Acceptance Criteria

- The export shape is explicitly documented.
- Source-selection policy is explicit, including what is target policy versus
  what is already proven by exporter or validator surfaces.
- Required public metadata and provenance fields are documented truthfully for
  the current accepted packet shape.
- Non-exportable categories are documented with truthful enforcement notes.
- A real selected-record proof surface is linked for contract review.
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
- If redaction or security review is implied too early, reviewers may overtrust
  publication readiness.

## Future Work

- codify redaction/publication safety policy (`#4003`)
- codify validation/public indexing contract (`#4004`)
- codify security review/CAV routing (`#4005`)
- prove bounded distribution and closeout (`#4006`)
- harden exporter-side source-admission checks if later implementation work is
  needed to enforce the full bridge policy at export time

## Notes

`v0.92` may consume public prompt records only after export, redaction,
validation, indexing, evidence, and security-review boundaries are all
completed truthfully.

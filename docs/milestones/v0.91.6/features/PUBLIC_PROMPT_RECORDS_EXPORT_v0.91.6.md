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
- Validation/indexing proof note: [PUBLIC_PROMPT_RECORDS_VALIDATION_INDEXING_4004.md](../review/public_prompt_records/PUBLIC_PROMPT_RECORDS_VALIDATION_INDEXING_4004.md)
- Security/CAV handoff packet: [PUBLIC_PROMPT_RECORDS_SECURITY_CAV_HANDOFF_4005.md](../review/public_prompt_records/PUBLIC_PROMPT_RECORDS_SECURITY_CAV_HANDOFF_4005.md)

## Template Rules

This is a feature-scope contract and bridge record. It defines what public prompt
records must look like, what source-selection policy the bridge intends to
preserve, which publication-safety classes are allowed, how validation and
reviewer-facing indexing work at the current milestone boundary, and what
security review/CAV handoff is required before distribution proof. It does not
by itself prove CAV completion, distribution approval, or release approval.

## Purpose

Define the `v0.91.6` contract for exporting public prompt records from local
C-SDLC authoring state without promoting local `.adl` records into public truth,
while keeping enforcement claims aligned with the current exporter and validator
surfaces and making publication-safety, validation, reviewer-facing index,
and security-review handoff rules explicit.

## Context

`v0.91.5` established the first exporter and validator surface for public
prompt packets plus a reviewer-facing pilot index. `v0.91.6` must now make the
export shape, provenance, source-selection rules, publication-safety posture,
validation contract, public index rules, and activation-path security boundary
explicit enough that later security review and distribution work can build on a
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
- validation and reviewer-facing public index rules
- bounded security review and CAV handoff for the public prompt-record surface
- ineligible source and record categories
- truthful proof notes for export shape, publication safety, validation/indexing,
  and security/CAV handoff
- explicit separation between already-proven enforcement and later hardening

This feature does not yet own:
- CAV completion
- final public distribution workflow
- exporter hardening beyond what current repository evidence already proves
- full threat modeling for all ADL security surfaces
- broader provider/model, ACIP/A2A, or product-wide security closure

## Overview

The public prompt-record bridge has four distinct layers:

1. local authoring truth
   - issue-local `.adl` task bundles, source prompts, and review/output cards
2. public packet truth
   - tracked public packets under `docs/milestones/<version>/review/evidence/csdlc/issues/...`
3. reviewer/public navigation truth
   - tracked reviewer-facing indexes and proof notes that describe which packets
     are included, why they are included, and what validation/publication-safety
     posture they satisfy
4. activation-path security truth
   - bounded review packets that record what abuse classes were reviewed,
     which are already covered by current packet controls, which remain blocked
     for distribution, and which must be carried into WP-07 CAV/security work

`v0.91.6` makes that boundary explicit. Export is intended to operate only from
approved, issue-bounded authoring sources, but current enforcement is layered:

- the exporter already proves packet shape generation and source-card content
  hygiene checks on the selected bundle
- the validator already proves packet-shape, tracker-metadata, redaction-block,
  and public-provenance requirements for accepted packets
- current public prompt packet export uses `refuse_not_rewrite` redaction mode
  for source cards and public packet text
- explicitly redacted reviewer/public projections may exist in other tracked
  proof surfaces, but they require an explicit review-safe record rather than
  silent exporter rewriting of local `.adl` source cards
- reviewer-facing public indexing currently uses a maintained tracked index
  surface rather than a separate generator binary
- security review for public prompt records is now a required bounded handoff
  surface before distribution proof, not an implied later cleanup step
- broader adversarial verification, provider trust, malformed-output, and
  ACIP/A2A security residuals still route through the security bridge/CAV lane
  rather than hiding inside the packet export workflow

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

### Validation contract

The current public prompt packet validator is the machine gate for accepted
packets and packet roots:

```bash
adl tooling public-prompt-packet validate \
  --packet <packet-dir-or-packet-root> \
  [--repo-root <repo-root>]
```

Current accepted validation surface:

- one packet directory containing `manifest.json`; or
- one packet root whose direct children each contain `manifest.json`

Current deterministic validation checks for accepted packets:

- manifest schema/version/issue/slug presence and shape
- repo-relative source bundle provenance with `.adl/<version>/tasks/...` task-bundle rules
- repo-relative output directory and public card paths
- tracker contract: `provider=github`, matching issue number, present GitHub issue URL
- redaction block contract: `status=passed`, `mode=refuse_not_rewrite`, non-empty checks array
- all five lifecycle cards present with valid kinds and existing tracked paths
- public packet text safety for manifest, README, and cards
- packet README presence

Validation constraints:

- validation must run without private local state outside the selected tracked
  packet root and repo-relative authoring/provenance references
- validation must fail closed on broken tracker metadata, invalid provenance,
  missing required files, or public-unsafe packet content
- validation may operate over a whole packet root to cover reviewer-facing
  packet-set completeness, not just one packet at a time

Completed-card caveat inherited from the pilot evidence:
- the v0.91.5 pilot showed that some latest-template structure diagnostics are
  still bootstrap-oriented and not yet a complete universal gate for every
  historical completed card
- `#4004` therefore defines the current validation contract around the proven
  packet validator and reviewer-facing packet-root checks, not around an
  overclaimed universal completed-card schema story

### Reviewer-facing public indexing contract

The canonical reviewer-facing index root for public prompt packets is:

- `docs/milestones/<version>/review/evidence/csdlc/issues/README.md`

Current maintained-index rules:

- one reviewer-facing row per included packet
- each row links to the packet directory
- each row states the represented issue, surface, selection reason, and status
- the index includes validated exported records and omits refused records
- refused candidates belong in proof notes, remediation records, or validation
  findings rather than in the public packet index itself
- the index must use repo-relative links only
- the index root must stay consistent with the packet root accepted by
  `adl tooling public-prompt-packet validate --packet <packet-root>`

Current indexing/non-indexing distinction:

- public packet directories are the machine-checked packet set
- the README index is the reviewer/public navigation layer over that set
- future machine-readable summary output is allowed, but `#4004` does not claim
  that such a generator already exists
- the current packet validator does not read or validate the root README index
  itself; index-row completeness and packet-link correctness remain maintained
  reviewer-facing obligations plus focused docs/path proof

### Link and path consistency rules

Reviewer-facing public indexing must detect or prevent:

- links to missing packet directories
- packet directories missing required files
- stale paths pointing into worktrees or other private local state
- index entries for refused or invalid packets

The current proven path for that detection is the combination of:

- packet-root validation over direct child manifests
- packet README/file presence checks
- repo-relative link/path discipline in the maintained reviewer index
- focused link/path hygiene checks in docs-only proof

Current proof boundary:
- packet-root validation proves the accepted packet set under the packet root
- it does not, by itself, prove that the maintained root README index has no
  stale rows or broken links
- reviewer-facing index completeness therefore remains a maintained-doc contract
  backed by focused docs/path proof rather than a fully machine-enforced gate

### Security review and CAV handoff contract

Public prompt records are not distributable merely because export,
redaction/publication safety, and validation/indexing boundaries exist.
Distribution remains blocked until the public prompt-record surface has a
bounded security-review result and a truthful handoff into the broader
security/CAV lane.

The current bounded review classes for this surface are:

- `reviewed_and_currently_covered`
  - abuse classes already bounded by the existing packet contract and reviewed
    evidence
- `reviewed_and_routed`
  - abuse classes reviewed here but intentionally handed to WP-07 for broader
    adversarial verification, provider trust, or cross-surface security work
- `distribution_blocker`
  - anything that would permit public prompt-record distribution without a
    completed security review result or without resolving/routing accepted
    findings

Security/publication abuse classes that must be reviewed before WP-04 closeout:

- secret exposure through packet content or reviewer/public projections
- raw provider log or raw provider payload exposure
- private path leakage or worktree/local-state leakage
- misleading provenance, spoofed tracker identity, or packet/index metadata that
  could make a packet look safer or more canonical than it is
- prompt injection or malicious text inside prompt-record artifacts that could
  mislead downstream consumers or reviewer/public projections
- provider-trust and transformed-artifact boundary confusion where a redacted
  projection could be mistaken for raw authoritative history

Current proven security posture for this issue family:

- `#4003` already covers secret exposure, raw prompt/provider-output reduction,
  and explicit reviewed redacted-projection posture
- `#4004` already covers invalid provenance/tracker metadata refusal for
  accepted packets and the maintained reviewer-index boundary
- `#4005` does not claim that these controls alone complete CAV or all prompt
  injection/provider trust analysis

Distribution rule carried forward into WP-04:

- public prompt records are not distributed without security review or an
  explicit blocked state
- accepted security findings must be fixed or routed before WP-04 closeout
- unresolved broader security/CAV work must be handed off explicitly to the
  security bridge lane rather than disappearing inside the public-records docs

### WP-07 handoff boundary

WP-04 owns the bounded public-record review and handoff surface.
WP-07 owns the broader security bridge and CAV work, including:

- dynamic adversarial verification beyond the current static packet contract
- broader provider/model trust and malformed-output checks
- cross-surface security implications for ACIP/A2A/provider communications
- any public-record security residual that cannot be closed truthfully in this
  bounded docs-only wave

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
  - `#3472` exported packet README/manifest/SOR after the pilot-validation
    metadata normalization noted in `#3474`
- refused example:
  - `adl/src/cli/tooling_cmd/tests/public_prompt_packet.rs`
  - validator-side provenance/tracker refusal in `public_prompt_packet_validate_fails_closed_on_manifest_and_redaction_drift`
- redacted reviewed projection example:
  - [LOGGING_VALIDATION_REDACTION_PROOF_4000.md](../review/logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md)
  - tracked excerpt-plus-digest reviewer/public artifacts such as the OpenRouter matrix packet and `#3951` remediation evidence

See [PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md](../review/public_prompt_records/PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md).

### Validation/indexing proof surface

`#4004` uses four bounded evidence classes:

- accepted single-packet validation example:
  - `public_prompt_packet_export_writes_manifest_readme_and_cards`
- accepted packet-root validation example:
  - `public_prompt_packet_validate_covers_root_help_and_missing_artifacts`
- invalid packet examples:
  - `public_prompt_packet_validate_fails_closed_on_manifest_and_redaction_drift`
  - missing README case inside `public_prompt_packet_validate_covers_root_help_and_missing_artifacts`
- reviewer-facing index example:
  - [docs/milestones/v0.91.5/review/evidence/csdlc/issues/README.md](../../v0.91.5/review/evidence/csdlc/issues/README.md)

See [PUBLIC_PROMPT_RECORDS_VALIDATION_INDEXING_4004.md](../review/public_prompt_records/PUBLIC_PROMPT_RECORDS_VALIDATION_INDEXING_4004.md).

### Security/CAV handoff surface

`#4005` uses three bounded evidence classes:

- already-covered public-record security controls:
  - the `#4003` publication-safety proof note and `#4004` validation/indexing proof note
- reviewed historical abuse cases and their repairs:
  - the v0.91.5 internal/external review findings on raw prompt/output exposure and misleading public evidence posture
- explicit handoff to the security bridge lane:
  - [SECURITY_BRIDGE_AND_CAV_v0.91.6.md](SECURITY_BRIDGE_AND_CAV_v0.91.6.md)

See [PUBLIC_PROMPT_RECORDS_SECURITY_CAV_HANDOFF_4005.md](../review/public_prompt_records/PUBLIC_PROMPT_RECORDS_SECURITY_CAV_HANDOFF_4005.md).

## Determinism and Constraints

- local `.adl` state remains the authoring surface
- public packets are projections, not source-of-truth replacements
- default-bundle export and accepted-packet provenance must remain
  reviewer-auditable
- public prompt packet export stays `refuse_not_rewrite` unless a later issue
  explicitly changes and proves that behavior
- current validation runs over one packet or a packet root of direct children
  and must not require private local state outside repo-relative references
- reviewer-facing indexes must include validated exported packets and omit
  refused records
- public prompt records remain blocked for distribution until bounded security
  review exists and broader residuals are either fixed or explicitly routed
- later lanes may tighten enforcement, but they must not silently change this
  issue's declared packet identity, publication-safety, validation/indexing, or
  security-handoff contract
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

This issue family's proof is documentation/provenance/review proof, not runtime
proof. Validation for `#4005` should confirm:

- the feature doc defines one bounded security-review and CAV-handoff contract
- public prompt records remain blocked for distribution until security review is
  complete or explicitly blocked
- reviewed abuse classes include secret exposure, provider-log/payload exposure,
  private path leakage, misleading provenance, prompt-injection-style misleading
  artifacts, and provider-trust confusion
- already-covered classes are separated from broader residuals routed to WP-07
- accepted security findings are fixed or routed before WP-04 closeout
- no private details are published in the review packet

This issue does not prove CAV completion, dynamic adversarial coverage, or final
public distribution approval. Those remain separately tracked and must not be
claimed from `#4005` alone.

## Acceptance Criteria

- The export shape is explicitly documented.
- Source-selection policy is explicit, including what is target policy versus
  what is already proven by exporter or validator surfaces.
- Required public metadata and provenance fields are documented truthfully for
  the current accepted packet shape.
- Redaction/publication-safety classes, exception rules, and non-publishable
  categories are documented truthfully.
- A real selected-record proof surface is linked for contract review.
- Real allowed, redacted, refused, valid, invalid, and reviewer-index evidence
  surfaces are linked for review.
- The reviewer-facing public index rules are explicit and omit refused records.
- The security-review/CAV-handoff rules are explicit and keep distribution
  blocked until review/routing truth exists.
- The doc states clearly that local `.adl` remains the authoring surface and
  public packets are projections.
- The doc does not overclaim exporter-side, universal completed-card validation,
  or completed CAV/security guarantees that are not yet proven.

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
- If packet-root validation and reviewer indexes drift apart, reviewers can be
  shown stale or incomplete packet navigation.
- If security review is treated as narrative rather than an explicit activation-
  path gate, public prompt records can be distributed with unresolved abuse
  classes hidden behind documentation theater.

## Future Work

- prove bounded distribution and closeout (`#4006`)
- harden exporter-side source-admission checks if later implementation work is
  needed to enforce the full bridge policy at export time
- add machine-readable reviewer-index summary output only if later tooling work
  proves it is stable and useful
- execute broader security bridge and CAV residuals through WP-07

## Notes

`v0.92` may consume public prompt records only after export, redaction,
validation, indexing, evidence, security-review, CAV/distribution boundaries,
and closeout truth are all completed truthfully.

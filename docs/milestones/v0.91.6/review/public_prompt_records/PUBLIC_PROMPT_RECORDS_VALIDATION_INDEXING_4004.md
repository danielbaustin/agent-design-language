# Public Prompt Records Validation And Indexing Proof Note for #4004

## Scope

This note records the bounded proof surface used by `#4004` to define the
current validation and reviewer-facing indexing posture for public prompt
records. It is not a security signoff, CAV packet, or distribution approval
record.

## Source evidence

- [PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md](../../features/PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md)
- `adl/src/cli/tooling_cmd/tests/public_prompt_packet.rs`
- [docs/milestones/v0.91.5/review/evidence/csdlc/issues/README.md](../../../v0.91.5/review/evidence/csdlc/issues/README.md)
- [PUBLIC_PROMPT_PACKET_PILOT_VALIDATION_3474.md](../../../v0.91.5/review/evidence/csdlc/issues/PUBLIC_PROMPT_PACKET_PILOT_VALIDATION_3474.md)
- [PUBLIC_PROMPT_RECORDS_v0.91.5.md](../../../v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md)

## Why these are the right proof surfaces

`#4004` is a docs-only contract issue. The strongest evidence is therefore the
existing validator contract tests plus the already-tracked v0.91.5 reviewer
index and pilot-validation packet.

That gives us:

- a valid single-packet example
- a valid packet-root example
- invalid packet examples that fail closed
- a real reviewer-facing maintained index pattern

without widening the issue into new tooling implementation.

## Validation classes

`#4004` uses three validation/indexing classes.

1. `accepted_packet_validation`
2. `refused_packet_validation`
3. `reviewer_index_navigation`

These are proof classes, not new runtime enums.

## Evidence matrix

| Class | Meaning | Representative evidence | What it proves |
| --- | --- | --- | --- |
| `accepted_packet_validation` | A packet or packet root can be validated deterministically as an accepted public prompt-record surface. | `public_prompt_packet_export_writes_manifest_readme_and_cards`; `public_prompt_packet_validate_covers_root_help_and_missing_artifacts` | The validator accepts one packet directory or one packet root of direct-child manifests and checks manifest/card/README/provenance/tracker/redaction invariants. |
| `refused_packet_validation` | Invalid packets fail closed rather than being indexed or treated as accepted public records. | `public_prompt_packet_validate_fails_closed_on_manifest_and_redaction_drift`; missing-README case in `public_prompt_packet_validate_covers_root_help_and_missing_artifacts` | Missing required tracker data, invalid public provenance, or missing required artifacts cause validation failure. |
| `reviewer_index_navigation` | Reviewers can navigate the accepted packet set through one maintained tracked index. | `docs/milestones/v0.91.5/review/evidence/csdlc/issues/README.md` | One reviewer-facing README can enumerate included packets, selection reasons, represented status, and packet links without making refused records part of the public packet set. |

## Accepted packet details

The current validator contract already proves:

- one packet directory containing `manifest.json` is accepted
- one packet root whose direct children contain `manifest.json` is accepted
- manifests are sorted deterministically at the packet-root level
- each accepted packet must include `README.md`, `manifest.json`, and all five lifecycle cards
- accepted packets must satisfy tracker/provider/provenance/redaction-block rules

Current proof boundary:
- `#4004` does not claim a new universal completed-card schema gate beyond the
  current accepted packet validator and the pilot's completed-card caveat
- `#4004` does claim that accepted packet validation and packet-root validation
  are already strong enough to define the accepted packet-set contract that the
  reviewer-facing index must describe

## Refused packet details

The current validator/test evidence already proves fail-closed behavior for:

- missing required tracker URL
- invalid absolute/worktree public provenance in accepted packet metadata
- missing packet README in a would-be accepted packet

Rule carried forward into the public prompt-record contract:
- refused packets are not indexed as accepted public prompt records
- refusal evidence belongs in proof notes, tests, or remediation packets rather
  than in the reviewer-facing packet index

## Reviewer-facing public index details

The v0.91.5 pilot index at
`docs/milestones/v0.91.5/review/evidence/csdlc/issues/README.md` is the current
reviewer-facing model.

What that model already proves:

- the index can list one row per included packet
- each row can link directly to the packet directory
- each row can record selection reason, represented surface, and represented
  issue/PR status
- the index can coexist with explicit pilot limitations and follow-on routing
- the index does not need to list refused packet attempts as if they were
  accepted public evidence

Rule carried forward into the v0.91.6 contract:
- the canonical reviewer-facing packet index is the milestone-local
  `review/evidence/csdlc/issues/README.md`
- it must include validated exported packets and omit refused records
- it must use repo-relative links and stay consistent with the packet-root
  accepted by the validator

## Link and path consistency details

The current contract for detecting stale or broken navigation is the
combination of:

- packet-root validation over direct child manifests
- required packet README presence
- repo-relative packet links in the maintained index
- focused docs/path hygiene checks over the index/proof surfaces

Current proof boundary:
- packet-root validation proves the accepted packet set under the packet root
- it does not validate the root reviewer-facing README index itself
- stale rows or broken packet links in the maintained index therefore still rely
  on the maintained-doc contract plus focused docs/path proof

## Pilot caveat carried forward

The v0.91.5 pilot validation packet matters here because it preserves one
important boundary:

- latest-template structure diagnostics are still not the same thing as a full
  completed-card-aware public packet gate

So `#4004` uses the existing validator contract and packet-root/index proof as
its main accepted validation story, while explicitly avoiding the claim that all
historical completed cards already satisfy one universal latest-schema gate.

## What this proof note does not claim

This note does not claim:

- security-review completion
- CAV approval
- public distribution approval
- a machine-generated reviewer indexer already exists
- exporter-side rejection of every invalid explicit source override class

Those remain owned by `#4005`, `#4006`, and later tooling hardening.

## Reviewer takeaway

`#4004` is ready when reviewers can confirm that the repository now has one
truthful validation/indexing story for public prompt records:

- accepted packets validate deterministically
- invalid packets fail closed
- reviewer navigation uses one maintained tracked index
- refused records are omitted from the accepted public packet index rather than
  being treated as navigable public evidence

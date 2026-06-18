# Public Prompt Records Export Contract Proof Note for #4002

## Scope

This note records the bounded proof surface used by `#4002` to define the
`v0.91.6` export-shape and source-selection contract for public prompt records.
It is not a redaction approval, indexing proof, security signoff, or release
clearance packet.

## Source evidence

- [PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md](../../features/PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md)
- [PUBLIC_PROMPT_RECORDS_v0.91.5.md](../../../v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md)
- [#3472 exported packet README](../../../v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/README.md)
- [#3472 exported packet manifest](../../../v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/manifest.json)
- [#3472 SOR](../../../v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/sor.md)

## Why #3472 is the right proof surface

`#3472` is the first real exported public prompt packet in the repository. It
already proves the current happy-path exporter contract against a concrete issue
bundle, including:

- lifecycle card export into `cards/`
- packet `README.md` generation
- machine-readable `manifest.json`
- repo-relative provenance and output paths on the default issue-bundle path
- tracker identity separated from tracker-agnostic work-item identity
- refusal semantics for obvious publication-unsafe card content

Using `#3472` as the representative selected-record example keeps `#4002`
source-grounded and avoids inventing a synthetic fixture whose shape could drift
from the actual exporter behavior.

## Contract points proven by the selected record

| Contract point | Evidence in selected record |
| --- | --- |
| Packet root is issue-bounded and milestone-scoped | `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/` |
| Public packet contains `README.md`, `manifest.json`, and `cards/` | `#3472` packet root contents |
| Lifecycle order is preserved | exported `sip`, `stp`, `spp`, `srp`, `sor` card set |
| Provenance remains repo-relative on the default bundle path | `#3472` packet manifest/README and SOR validation notes |
| Public packet is projection, not local source replacement | `#3472` feature-doc and SOR language |
| Unsafe source-card content is refused rather than silently rewritten | `#3472` SOR summary and validation notes |

## Source-selection claims this proof note does not make

`#3472` does not by itself prove:

- explicit override behavior for arbitrary `--source` paths
- exporter-side rejection of `.worktrees/` or other invalid override sources
- rejection of every identity-ambiguous bundle before packet generation

Those remain policy requirements or later enforcement concerns unless separately
proven by tooling-focused issue work.

## Source-selection rules carried forward into v0.91.6

The `v0.91.6` feature doc carries forward the bridge policy that:

- default source is the milestone-local `.adl/<version>/tasks/issue-<number>__<slug>/` bundle
- acceptable public provenance must remain repo-relative and issue-bounded
- `.worktrees/`, `.codex/`, temp/scratch paths, and host-local absolute paths are not acceptable public provenance
- partial or identity-ambiguous bundles are not acceptable as valid public prompt records

Current proof boundary:
- `#3472` proves the default-bundle happy path
- the current validator proves accepted packets must satisfy the tracked public provenance and tracker contract
- `#4002` does not claim that export-time override rejection is fully hardened yet

## What this proof note does not claim

This note does not claim:

- redaction policy completeness
- public indexing readiness
- security-review completion
- CAV approval
- release readiness for public prompt record publication

Those remain owned by `#4003` through `#4006`.

## Reviewer takeaway

`#4002` is ready when reviewers can confirm that the `v0.91.6` feature doc and
this proof note together define one stable export contract grounded in the real
`#3472` packet, while keeping exporter-proven behavior, validator-proven
behavior, and later hardening work clearly separated.

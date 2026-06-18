# Public Prompt Records Redaction And Publication Safety Proof Note for #4003

## Scope

This note records the bounded proof surface used by `#4003` to define the
current redaction and publication-safety posture for public prompt records.
It is not an indexing proof, security signoff, CAV approval, or release
clearance packet.

## Source evidence

- [PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md](../../features/PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md)
- [PUBLIC_PROMPT_RECORDS_v0.91.5.md](../../../v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md)
- [#3472 exported packet README](../../../v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/README.md)
- [#3472 SOR](../../../v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/sor.md)
- [PUBLIC_PROMPT_PACKET_PILOT_VALIDATION_3474.md](../../../v0.91.5/review/evidence/csdlc/issues/PUBLIC_PROMPT_PACKET_PILOT_VALIDATION_3474.md)
- `adl/src/cli/tooling_cmd/tests/public_prompt_packet.rs`
- [LOGGING_VALIDATION_REDACTION_PROOF_4000.md](../logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md)
- [V0915_EXTERNAL_REVIEW_FINDINGS_2026-06-17.md](../../../v0.91.5/review/external_review/V0915_EXTERNAL_REVIEW_FINDINGS_2026-06-17.md)
- [OPENROUTER_MATRIX_PROOF_2026-06-14.md](../../../v0.91.5/review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md)
- [openrouter_matrix_state_2026-06-14.json](../../../v0.91.5/review/openrouter_matrix/openrouter_matrix_state_2026-06-14.json)

## Safety classes

`#4003` uses three publication-safety classes.

1. `allowed_verbatim_packet`
2. `allowed_redacted_projection`
3. `refused_public_record`

These are policy and proof classes, not new runtime enums.

## Evidence matrix

| Safety class | Meaning | Representative evidence | What it proves |
| --- | --- | --- | --- |
| `allowed_verbatim_packet` | A bounded public prompt packet may be tracked verbatim after hygiene checks pass. | `#3472` exported packet README + SOR; `PUBLIC_PROMPT_PACKET_PILOT_VALIDATION_3474.md` | The current tracked packet shape is reviewable and public-safe after the pilot-validation metadata normalization repaired the earlier recovery-export provenance drift. |
| `refused_public_record` | Unsafe packet candidates fail closed rather than being rewritten silently. | `public_prompt_packet_export_refuses_host_paths_and_secret_markers`; `public_prompt_packet_validate_fails_closed_on_manifest_and_redaction_drift` | Source cards with host paths or secret markers are refused, and accepted-packet validation also fails closed on invalid tracker/provenance drift. |
| `allowed_redacted_projection` | A reviewer/public artifact may be tracked only in an explicitly redacted, review-safe form. | `#4000` logging proof; OpenRouter matrix proof and state packet; `#3951` external-review remediation evidence | Redacted excerpt-plus-digest or bounded metadata projections are acceptable when the artifact explicitly records the redaction posture instead of exposing raw prompts/output. |

## Allowed case details

The current tracked `#3472` packet is the canonical allowed packet example
because it now proves:

- packet `README.md`, `manifest.json`, and lifecycle cards exist in one bounded packet
- source and public paths are repo-relative after the `#3474` metadata-normalization repair
- packet text is scanned for host-local paths, secret-like values, private-key markers, local scratch markers, and unresolved template residue
- the packet is a projection of `.adl` authoring state, not a replacement for it

## Refused case details

The current public prompt packet exporter and validator already prove fail-closed
behavior for these refusal classes:

- source-card host-local absolute paths
- secret-like token markers
- private-key markers through the shared safety check path
- invalid packet tracker metadata such as a missing required GitHub issue URL
- invalid public provenance such as absolute/worktree source paths in accepted packet metadata
- temp-path and `.codex` scratch-path treatment remain bridge-policy expectations here, but `#4003` does not cite a dedicated refused example for those exact classes

Current proof boundary:
- `#4003` does not claim exporter-side refusal for every invalid explicit
  `--source` override class before packet generation
- it does claim that accepted public packets and obvious unsafe source-card
  inputs already have fail-closed protection on the proven surfaces above

## Redacted case details

The current exporter mode remains `refuse_not_rewrite`. That means the redacted
case is **not** “the exporter rewrites local source cards.”

Instead, the redacted case is an explicit reviewer/public projection whose own
artifact says that it contains only redacted excerpts, digests, counts, or
bounded metadata. The current repository evidence for that class includes:

- `#4000` logging proof, which explicitly treats redaction/path hygiene as part
  of the proof surface for publication-facing artifacts
- the OpenRouter matrix state packet and proof packet, which store
  `prompt_excerpt` / `output_excerpt` markers with digest and length metadata
  instead of raw provider content
- `#3951` remediation evidence recorded in the external-review findings packet,
  which verifies that raw prompt content was removed from tracked artifacts and
  replaced with review-safe metadata

Rule carried forward into the public prompt-record contract:
- redacted publication is allowed only when the artifact itself makes the
  transformation explicit and reviewable
- silent source-card rewriting is not an acceptable substitute

## Publication-safety rules established here

The bounded rules `#4003` establishes are:

- no durable public artifact may contain token bytes, private keys, or secret-like values
- no durable public artifact may contain host-local absolute paths or worktree provenance on the currently proven public-packet surfaces
- temp-path and `.codex` scratch-path provenance are also not acceptable by bridge policy, but `#4003` does not treat them as separately demonstrated refused examples yet
- raw provider logs and raw provider payloads are not acceptable public prompt-record artifacts by default
- raw prompt text and raw model output are not acceptable public prompt-record artifacts by default when a redacted reviewer/public projection is the correct surface
- any exception must be recorded as an explicit review-safe projection, not hidden inside the exporter
- local `.adl` authoring state remains private/local unless exported into an approved public packet or another explicit reviewed projection

## Input drift captured for remediation

The authored `#4003` issue body names `WP-03 logging validation/redaction
outputs` and an older closeout-style expectation, but the current live WP-03
redaction proof surface consumed here is:

- [LOGGING_VALIDATION_REDACTION_PROOF_4000.md](../logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md)

This is not a blocker for `#4003`, because the live proof packet exists and is
stronger than the stale issue-body path expectation. It should still be treated
as issue-input drift to clean up in future authoring/remediation work.

## What this proof note does not claim

This note does not claim:

- public indexing readiness
- security-review completion
- CAV approval
- release readiness for public prompt record publication
- exporter-side rejection of every invalid explicit override class

Those remain owned by `#4004` through `#4006` and later tooling hardening.

## Reviewer takeaway

`#4003` is ready when reviewers can confirm that the repository now has one
truthful publication-safety policy for public prompt records:

- allowed packets are bounded and hygiene-checked
- unsafe packets fail closed
- redacted reviewer/public projections must be explicit and reviewable
- local `.adl` source cards are not silently rewritten into public-safe output

# Public-Record And Memory/Profile Security Review for #4022

## Scope

This packet records the bounded WP-07 security review for public-record
publication posture and memory/profile privacy on the `v0.91.6` activation
path.

It is not a completed WP-10 privacy closeout, not approval to publish
memory/cognitive-profile material by default, and not proof that open identity,
memory, or observability lanes are already safe to consume publicly.

## Source evidence

- `docs/milestones/v0.91.6/features/SECURITY_BRIDGE_AND_CAV_v0.91.6.md`
- `docs/milestones/v0.91.6/features/PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md`
- `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_SECURITY_CAV_HANDOFF_4005.md`
- `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md`
- `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_VALIDATION_INDEXING_4004.md`
- `docs/milestones/v0.91.6/review/logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md`
- `docs/milestones/v0.91.6/review/logging_observability/GITHUB_TOKEN_RELEASE_PROJECTION_PROOF_4001.md`
- `docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md`
- `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_ACTIVE_SPRINT_EXECUTION_PACKETS_2026-06-18.md`
- live issue state for `#3973`, `#3975`, `#4036`, `#4040`, and `#4041`

## Review goal

Determine whether the current public-record and memory/profile surfaces have
enough reviewed publication/privacy posture to remain on the activation path
without pretending that open WP-08 or WP-10 work is already complete.

The review therefore separates:

- public prompt-record controls that are already proved and consumable
- logging/projection hygiene that acts as a redaction floor
- open memory/profile/privacy boundaries that remain routed
- identity-sensitive publication assumptions that must not be silently promoted

## Publication and privacy rules established here

1. Public prompt records remain projections of local authoring state, not
   replacement authority for `.adl` issue records.
2. Raw prompts, raw provider payloads, raw private logs, secret markers, and
   host-local paths are not acceptable public-record or reviewer-facing
   publication surfaces by default.
3. Memory/ObsMem, Memory Palace, ACP, and cognitive-profile material must not
   be treated as publication-safe merely because their planning or bridge docs
   exist.
4. Cognitive-profile, identity, and provider/profile boundaries remain distinct
   layers; public publication must not collapse them into one surface.
5. Open WP-08 and WP-10 issue state is a real security dependency, not a
   narrative detail that can be omitted from closeout.

## Boundary matrix

| Boundary | Why it matters | Current evidence | Current disposition |
| --- | --- | --- | --- |
| Public packet vs local authoring truth | A tracked public packet can be mistaken for the editable authority surface if projection and authoring state blur together. | `PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md` keeps local `.adl` bundles as canonical authoring truth and treats public packets as reviewer/public projections. | `reviewed_and_currently_covered` |
| Redacted projection vs raw prompt/log/payload boundary | Unsafe publication can occur if raw prompt text, raw provider payloads, or secret-bearing logs are treated as acceptable durable artifacts. | `#4003`, `#4005`, and the WP-03 logging proof keep redaction/publication safety explicit and fail closed on unsafe packet candidates. | `reviewed_and_currently_covered` |
| Tracker/provenance/path hygiene boundary | Public or reviewer-facing records become misleading or unsafe if they expose host-local paths or spoofed provenance. | `#4003`, `#4004`, and `#4001` explicitly keep repo-relative provenance and projection hygiene on the accepted path. | `reviewed_and_currently_covered` |
| Logging/projection redaction floor | Later security-consuming lanes can overclaim publication safety if they ignore the existing logging/projection redaction proof. | `LOGGING_VALIDATION_REDACTION_PROOF_4000.md` and `GITHUB_TOKEN_RELEASE_PROJECTION_PROOF_4001.md` provide the bounded logging/projection hygiene floor consumed here. | `reviewed_and_currently_covered` |
| Memory/accounting doc vs publication approval boundary | Planning/accounting surfaces can be misread as proof that memory or profile material is safe to publish. | `AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md` remains `planned`, and the memory sprint SEP plus live issue state show `#3975`, `#4036`, `#4040`, and `#4041` still open. | `reviewed_and_routed` |
| Cognitive-profile privacy boundary | Profile fields can expose sensitive identity, behavioral, or birthday-adjacent material if later publication surfaces treat them as normal reviewer output. | The memory/ACP bridge names ACP/cognitive-profile privacy as a security boundary, but there is no landed WP-10 privacy proof packet yet. | `reviewed_and_routed` |
| Identity continuity vs public profile boundary | Identity continuity or capability-selector planning can be mistaken for reviewed publication-safe identity surfaces. | WP-08 remains open by feature-doc and live issue state, so identity-safe publication cannot be claimed here. | `reviewed_and_routed` |
| Cross-surface observability and downstream consumption boundary | Reviewer/public records can later leak into observability, observatory, or memory-consumption paths if security closure is implied too early. | The security bridge ledger keeps WP-09 and WP-10 active dependencies, and this issue is the bounded review lane rather than a closeout override. | `reviewed_and_routed` |

## Public-record security notes

Current reviewed truth:

- public prompt records already have bounded export, validation, redaction, and
  security-handoff proof
- distribution is not the same as publication-safe-by-default for every future
  adjacent surface
- public prompt records may be consumed only as the reviewed packet/projection
  surface, not as authority to publish neighboring memory or profile material

## Memory/profile privacy notes

Current reviewed truth:

- the memory/ACP bridge correctly treats ACP/cognitive-profile privacy as a
  security boundary
- live milestone state still shows the memory umbrella and its privacy/ledger
  children open
- no landed review packet in the current repo proves that cognitive-profile or
  memory-adjacent material is broadly publication-safe
- later publication or reviewer-facing consumption must remain blocked or
  explicitly routed until WP-10 lands its bounded privacy and closeout proof

## Findings and dispositions

1. The public prompt-record surface is already strong enough to supply reviewed
   export, redaction, validation, and reviewer-facing projection truth for this
   lane.  
   Disposition: fixed by the existing WP-04 proof set and consumed here as a
   prerequisite rather than re-opened.

2. WP-03 logging/projection proof provides a real redaction and path-hygiene
   floor for reviewer/public artifacts, but it does not by itself approve new
   memory/profile publication surfaces.  
   Disposition: fixed as a consumed floor, with explicit non-claim retained in
   this packet.

3. Memory/ObsMem, Memory Palace, ACP, and cognitive-profile publication/privacy
   closure is not yet proved by the current milestone evidence.  
   Disposition: routed to open WP-10 owners `#3975`, `#4036`, `#4040`, and
   `#4041`, and retained under WP-07 closeout `#4024`.

4. Identity continuity and capability-selector work is still open, so this
   lane cannot claim identity-safe publication of cognitive or citizen-adjacent
   material.  
   Disposition: routed to WP-08 `#3973` and retained as a non-claim here.

5. Any attempt to treat planning/accounting docs as if they were public-safe
   runtime or reviewer-output approval would be a false promotion of bridge
   state into security closure.  
   Disposition: fixed by this packet's explicit boundary rules and residual
   routing.

## Consumption rule for v0.91.6 and v0.92

Current decision:

- `public_record_security_consumed_memory_profile_privacy_still_routed`

That means later milestone work may consume:

- public prompt-record export/redaction/validation/security-handoff truth
- the WP-03 logging/projection hygiene floor
- the rule that memory/profile publication is still an explicit security
  boundary and not silently cleared

It may not consume this issue as proof of:

- completed WP-10 privacy closure
- approved publication of cognitive-profile or memory-adjacent content
- completed WP-08 identity-safe publication posture
- blanket approval for downstream observability or Unity consumption of
  identity/profile surfaces

## Residual routing

- Memory/ObsMem, Memory Palace, ACP, and cognitive-profile publication/privacy
  residuals remain owned by open WP-10 work, especially `#3975`, `#4036`,
  `#4040`, and `#4041`.
- Identity-safe publication residuals remain owned by WP-08 `#3973`.
- Any later observability or Unity consumption of identity/profile/publication
  surfaces must remain visible in WP-09 and `#4023` rather than being implied
  complete by this issue.
- If WP-10 or WP-08 remains unresolved at milestone closeout, `#4024` must
  keep those residuals open or route them through the `v0.91.7` residual
  guard instead of silently passing WP-07.

## Reviewer takeaway

`#4022` is ready when reviewers can confirm that:

- WP-04 public-record security proof is consumed rather than restated from
  memory
- WP-03 logging/projection redaction proof is treated as a floor, not a broad
  publication approval
- memory/profile privacy remains an explicit open security boundary
- unresolved WP-10, WP-08, and WP-09 residuals are routed to named open owners
  instead of being implied complete

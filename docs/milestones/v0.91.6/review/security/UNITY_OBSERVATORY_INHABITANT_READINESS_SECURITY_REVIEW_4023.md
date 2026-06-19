# Unity Observatory Inhabitant-Readiness Security Review for #4023

## Scope

This packet records the bounded WP-07 security review for Unity Observatory
inhabitant-facing surfaces and Observatory consumption posture on the
`v0.91.6` activation path.

It is not a completed WP-09 implementation closeout, not proof of live Unity
product readiness, and not approval to expose identity, memory, or private
artifact material to inhabitants by default.

## Source evidence

- `docs/milestones/v0.91.6/features/SECURITY_BRIDGE_AND_CAV_v0.91.6.md`
- `docs/milestones/v0.91.6/features/OBSERVATORY_UNITY_CONSUMPTION_CLASSIFICATION_v0.91.6.md`
- `docs/milestones/v0.91.6/features/IDENTITY_CONTINUITY_CAPABILITY_SELECTOR_BRIDGE_v0.91.6.md`
- `docs/milestones/v0.91.6/review/logging_observability/OTEL_OBSERVATORY_CONSUMPTION_PROOF_3999.md`
- `docs/milestones/v0.91.6/review/logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md`
- `docs/milestones/v0.91.6/review/logging_observability/GITHUB_TOKEN_RELEASE_PROJECTION_PROOF_4001.md`
- `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_ACTIVE_SPRINT_EXECUTION_PACKETS_2026-06-18.md`
- live issue state for `#3974`, `#4030`, `#4032`, `#4033`, `#4034`, `#4035`, and `#3973`

## Review goal

Determine whether the current Unity Observatory and Observatory-consumption
surfaces have enough reviewed security posture to remain on the activation path
without overstating inhabitant safety, observability safety, or identity-safe
display while WP-08 and WP-09 remain open.

The review therefore separates:

- logging and event-stream proof that is already consumable
- Observatory/Unity classification truth that remains planning-only
- inhabitant-facing display and input boundaries that stay open until WP-09
  lands implementation and closeout proof
- identity/profile display assumptions that must stay routed rather than
  promoted into readiness claims

## Security rules established here

1. Observatory/Unity classification or demo planning is not the same thing as
   reviewed inhabitant-safe display or input behavior.
2. Redacted, machine-readable event-stream examples may be consumed as a floor
   for bounded observability ingestion, but they do not prove live Unity or
   Observatory runtime integration.
3. Inhabitant-facing Unity surfaces must not leak private paths, raw logs,
   credentials, memory artifacts, or unreviewed identity/profile data by
   default.
4. Open WP-08 and WP-09 issue state is a real dependency for identity-safe
   display and observatory-safe consumption claims.
5. This lane may review and route Unity/Observatory security posture, but it
   may not close WP-09 implementation or closeout truth on its own.

## Boundary matrix

| Boundary | Why it matters | Current evidence | Current disposition |
| --- | --- | --- | --- |
| Observatory classification vs implementation boundary | A planned or classified surface can be mistaken for working secure consumption if review does not keep planning and proof separate. | `OBSERVATORY_UNITY_CONSUMPTION_CLASSIFICATION_v0.91.6.md` stays explicitly `planned` and forbids rehearsal/substrate surfaces from proving readiness by themselves. | `reviewed_and_currently_covered` |
| Redacted event-stream vs live runtime boundary | A bounded event-stream example is useful ingestion evidence, but it can be overread as proof of complete Unity/Observatory runtime safety. | `OTEL_OBSERVATORY_CONSUMPTION_PROOF_3999.md` explicitly claims only a redacted example stream and refuses to claim live Unity or Observatory service integration. | `reviewed_and_currently_covered` |
| Logging/projection hygiene boundary | Inhabitant-facing and Observatory-consumed surfaces can leak private refs if downstream work ignores the established redaction floor. | `LOGGING_VALIDATION_REDACTION_PROOF_4000.md` and `GITHUB_TOKEN_RELEASE_PROJECTION_PROOF_4001.md` provide the bounded logging/projection hygiene floor consumed here. | `reviewed_and_currently_covered` |
| Inhabitant-facing display/input boundary | Unity inhabitant surfaces can expose unreviewed private state or accept unsafe inputs unless the display/input contract is explicit and implemented. | The WP-09 sprint SEP and live issue state show inhabitant surfaces are still owned by open issue `#4033`, so this review cannot claim the implementation is finished. | `reviewed_and_routed` |
| Observatory evidence-ingestion boundary | Observatory ingestion can overconsume raw artifacts, private logs, or unintended fields if the ADL evidence contract is not explicitly security-reviewed. | The WP-09 sprint SEP and live issue state show the data-ingestion contract and security/OTel proof remain owned by open issues `#4032` and `#4034`. | `reviewed_and_routed` |
| Identity/profile display boundary | Inhabitant-facing surfaces can overclaim or leak identity/profile information if open WP-08 boundaries are treated as completed. | `IDENTITY_CONTINUITY_CAPABILITY_SELECTOR_BRIDGE_v0.91.6.md` remains `planned`, and live issue state shows `#3973` open. | `reviewed_and_routed` |
| Working Unity baseline vs closeout boundary | A baseline or rehearsal scene can be mistaken for security-cleared product readiness if final closeout proof is absent. | The WP-09 sprint SEP and live issue state keep baseline owner `#4030` and closeout owner `#4035` open. | `reviewed_and_routed` |

## Observatory and inhabitant notes

Current reviewed truth:

- the current milestone has a bounded, redacted event-stream example that later
  consumers may use as a vocabulary and redaction floor
- the current milestone does not yet have landed Unity Observatory proof that
  inhabitant-facing display and input surfaces are fully security-reviewed
- later consumers must distinguish between classified/planned surfaces and
  implemented/closed proof surfaces

## Identity and private-state notes

Current reviewed truth:

- inhabitant-safe display must not be inferred from open identity or capability
  bridge work
- this issue does not authorize public or inhabitant-facing display of memory,
  profile, citizen, or continuity-sensitive material by default
- any later claim that Unity Observatory safely exposes identity/profile data
  must consume landed WP-08 and WP-09 proof instead of this review packet alone

## Findings and dispositions

1. The existing WP-03 logging/observability proof and redacted event-stream
   example are strong enough to act as a bounded consumption floor for this
   lane.  
   Disposition: fixed by existing WP-03 proof surfaces and consumed here as a
   prerequisite rather than re-opened.

2. Unity Observatory inhabitant-facing display and input security is not yet
   proved by current milestone evidence.  
   Disposition: routed to open WP-09 owner `#4033` and retained under WP-09
   closeout `#4035` plus WP-07 closeout `#4024`.

3. Observatory data-ingestion, logging, and security consumption closure is not
   yet proved by current milestone evidence.  
   Disposition: routed to open WP-09 owners `#4032` and `#4034`, then retained
   under `#4035` and `#4024`.

4. Identity-safe inhabitant display remains open because WP-08 identity and
   continuity work has not landed its own reviewed proof surfaces.  
   Disposition: routed to WP-08 `#3973` and retained as a non-claim here.

5. Baseline or rehearsal Unity readiness must not be promoted into security
   closeout without the open WP-09 baseline and closeout owners landing their
   own proof.  
   Disposition: routed to WP-09 `#4030` and `#4035`.

## Consumption rule for v0.91.6 and v0.92

Current decision:

- `unity_observatory_security_reviewed_with_explicit_inhabitant_and_ingestion_routes`

That means later milestone work may consume:

- the redacted observability/event-stream vocabulary and logging hygiene floor
- the rule that Unity/Observatory planning or classification is not security
  closure by itself
- the requirement that inhabitant-facing and ingestion surfaces remain explicit
  security boundaries

It may not consume this issue as proof of:

- completed WP-09 Unity Observatory implementation
- approved inhabitant-safe identity or profile display
- completed Observatory ingestion or logging-security closure
- working Unity Observatory closeout truth

## Residual routing

- Unity inhabitant-facing display/input residuals remain owned by WP-09
  `#4033`.
- Observatory evidence-ingestion and logging/OTel/security consumption
  residuals remain owned by WP-09 `#4032` and `#4034`.
- Working Unity baseline and closeout residuals remain owned by WP-09 `#4030`
  and `#4035`.
- Identity-safe display residuals remain owned by WP-08 `#3973`.
- If WP-08 or WP-09 remains unresolved at milestone closeout, `#4024` must
  keep those residuals open or route them through the `v0.91.7` residual guard
  instead of silently passing WP-07.

## Reviewer takeaway

`#4023` is ready when reviewers can confirm that:

- WP-03 observability proof is consumed as a floor rather than overread as
  complete Unity/Observatory integration
- inhabitant-facing Unity surfaces remain an explicit open security boundary
- Observatory ingestion and logging-security closure remains routed to named
  open WP-09 owners
- WP-08 identity-sensitive display remains open and is not implied complete


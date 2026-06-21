# Unity Observatory Logging, OTel, and Security Consumption Proof for #4034

## Scope

This packet records the bounded WP-09 proof for Unity Observatory consumption
of logging, event-stream, and WP-07 security surfaces.

It proves a reviewed consumption floor and explicit non-claims. It does not
claim live Runtime v2 capture, live OpenTelemetry exporter integration, or
final WP-09 closeout.

## Source evidence

- `docs/milestones/v0.91.6/review/logging_observability/OTEL_OBSERVATORY_CONSUMPTION_PROOF_3999.md`
- `docs/milestones/v0.91.6/review/logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md`
- `docs/milestones/v0.91.6/review/security/UNITY_OBSERVATORY_INHABITANT_READINESS_SECURITY_REVIEW_4023.md`
- `demos/v0.91.6/unity-observatory/Assets/Resources/observatory_contract.json`
- `demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs`
- `demos/v0.91.6/unity-observatory/README.md`
- `demos/v0.91.6/unity-observatory/PROOF_PACKET.md`

## Review goal

Confirm that the Unity Observatory consumes the current observability and
security surfaces safely by:

- reusing the redacted event-stream and operator-report vocabulary as a floor
- making logging, OTel, and security proof refs explicit in the Unity-facing
  contract and shell
- preserving the fail-closed non-claim that this is not a live collector,
  exporter, or mutation console
- keeping private paths, secrets, raw logs, and identity-sensitive state out of
  the surface

## Claimed result

The Unity Observatory now exposes one explicit observability/security
consumption section with:

- `#3999` as the OTel/event-stream boundary source
- `#4000` as the logging-validation and redaction source
- `#4023` as the consumed security floor
- one issue-owned reviewer packet for findings and non-claims

The surface is still bounded:

- event-stream vocabulary is consumable
- operator-report references are consumable
- live OpenTelemetry exporter integration is not claimed
- private-state or secret display is not required

## Findings and dispositions

1. Observatory/Unity could previously consume operator-report references without
   making the OTel/logging/security proof chain explicit in the Unity-facing
   surface.  
   Disposition: fixed by adding an explicit observability/security section to
   the generated contract, checked-in seed, and Unity shell, with the shell
   gated on that contract section being present before it renders the card.

2. Reviewers needed to infer the non-claim boundary from multiple packets
   instead of seeing it in the Unity lane directly.  
   Disposition: fixed by surfacing the reviewed floor, private-state posture,
   and proof refs in the shell and README/proof packet.

3. Identity-safe display and final closeout remain open issue-owned surfaces.  
   Disposition: routed, not hidden. `#4033`, `#4035`, and WP-08 remain the
   owners for those claims.

## Validation

- `bash adl/tools/test_v0916_unity_observatory_contract.sh`
- `cargo test --manifest-path adl/Cargo.toml unity_observatory_contract_ -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml --test cli_smoke csm_observatory_cli_writes_unity_contract_bundle_and_matches_seeded_resource -- --nocapture`

## Non-claims

- No live Runtime v2 capture is claimed.
- No live OpenTelemetry collector or exporter integration is claimed.
- No raw private logs, secrets, or host paths are required by the Unity lane.
- No identity/profile/memory-safe inhabitant closure is claimed here.
- No WP-09 umbrella closeout is claimed here.

## Reviewer takeaway

`#4034` is successful when reviewers can confirm that the Unity Observatory now
consumes the reviewed observability/security floor explicitly, keeps the
non-claim boundary visible, and leaves the remaining open WP-09 claims routed
rather than implied.

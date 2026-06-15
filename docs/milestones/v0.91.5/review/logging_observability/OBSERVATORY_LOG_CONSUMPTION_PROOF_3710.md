# Observatory Log Consumption Proof (#3710)

Issue: #3710  
Status: draft_reviewable

## Scope

This packet proves a bounded design outcome:

- how Unity/Observatory should consume ADL logging artifacts;
- how Observatory should normalize and classify those artifacts;
- how redaction, retention, and correlation rules should constrain that
  consumption;
- how v0.92 planning should depend on this surface without overclaiming
  implementation.

## Inputs Reviewed

- `docs/milestones/v0.91.5/CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md`
- `docs/milestones/v0.91.5/SHARED_OBSERVABILITY_AND_OTEL_CONTRACT_3705.md`
- `docs/milestones/v0.91.5/OPEN_TELEMETRY_INTEGRATION_BOUNDARY_3709.md`
- `docs/milestones/v0.91.5/features/DEMO_AND_UNITY_OBSERVATORY_READINESS_v0.91.5.md`
- `docs/milestones/v0.91.5/review/logging_observability/LOGGING_OBSERVABILITY_GAP_MAP_3704.md`
- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`
- `docs/milestones/v0.92/DESIGN_v0.92.md`
- `adl/src/csm_observatory.rs`
- `adl/src/runtime_v2/contracts.rs`
- `adl/src/trace_schema_v1.rs`
- `adl/src/acc/*`
- `adl/src/agent_comms.rs`

## Artifacts Added

- `docs/milestones/v0.91.5/OBSERVATORY_LOG_CONSUMPTION_CONTRACT_3710.md`
- `docs/milestones/v0.91.5/review/logging_observability/OBSERVATORY_EVENT_STREAM_EXAMPLE_3710.json`

## What This Proves

- Observatory is a governed consumer/projection layer, not a new canonical
  runtime truth source.
- Existing ADL logging surfaces are sufficient to define a first consumer-side
  contract without requiring OTEL collector wiring.
- The required normalized projection fields, correlation refs, and redaction
  rules are concrete enough for v0.92 Unity/Observatory planning.
- A bounded multi-source event stream can be shown with operator, reviewer,
  `public_report_view`, and `observatory_projection` distinctions without
  leaking raw prompts, secrets, private payloads, or host-local absolute paths.

## What This Does Not Prove

- Unity Observatory implementation exists.
- OTEL export is live.
- Every current source artifact already carries every preferred correlation
  field.
- `public_report_view` / `observatory_projection` views are implemented in
  code; this issue defines the contract they must follow.

## Example Event Stream Reading

`OBSERVATORY_EVENT_STREAM_EXAMPLE_3710.json` demonstrates:

- control-plane progress (`doctor started`);
- provider heartbeat and timeout classifications;
- long-lived-agent cycle completion with durable artifact refs;
- a public-report-safe trace projection that preserves the separate governed
  `observatory_projection` boundary.

The example intentionally stays redacted and bounded. It uses stable refs,
reason codes, and display buckets instead of raw payloads or machine-local
paths.

## Validation Commands

```bash
python3 - <<'PY'
from pathlib import Path
contract = Path("docs/milestones/v0.91.5/OBSERVATORY_LOG_CONSUMPTION_CONTRACT_3710.md")
proof = Path("docs/milestones/v0.91.5/review/logging_observability/OBSERVATORY_LOG_CONSUMPTION_PROOF_3710.md")
contract_text = contract.read_text()
proof_text = proof.read_text()
assert "This document does not claim Unity Observatory is already built." in contract_text
assert "This document does not claim OTEL exporter support exists locally." in contract_text
assert "This issue does not require a live OTEL collector." in contract_text
assert "This issue does not build the Unity app." in contract_text
assert "What This Does Not Prove" in proof_text
print(contract.name, len(contract_text.splitlines()))
print(proof.name, len(proof_text.splitlines()))
PY

python3 - <<'PY'
import json
from pathlib import Path
packet = json.loads(Path("docs/milestones/v0.91.5/review/logging_observability/OBSERVATORY_EVENT_STREAM_EXAMPLE_3710.json").read_text())
assert packet["schema_version"] == "adl.observatory.event_stream_example.v1"
assert len(packet["events"]) >= 4
for event in packet["events"]:
    for key in ("schema", "source_kind", "component", "stage", "result", "redaction_view", "display_bucket"):
        assert key in event, (event, key)
assert packet["events"][0]["schema"] == "adl.observability.event.v1"
assert packet["events"][1]["schema"] == "provider_communication.v1"
assert packet["events"][2]["schema"] == "provider_communication.v1"
assert packet["events"][3]["schema"] == "adl.long_lived_agent_operator_event.v1"
assert packet["events"][4]["schema"] == "trace.v2"
assert packet["events"][4]["redaction_view"] == "public_report_view"
print("events", len(packet["events"]))
PY

rg -n "Observatory|observatory|redaction|correlation|public_report_view|observatory_projection|operator|reviewer" \
  docs/milestones/v0.91.5/OBSERVATORY_LOG_CONSUMPTION_CONTRACT_3710.md \
  docs/milestones/v0.91.5/review/logging_observability/OBSERVATORY_LOG_CONSUMPTION_PROOF_3710.md \
  docs/milestones/v0.91.5/review/logging_observability/OBSERVATORY_EVENT_STREAM_EXAMPLE_3710.json

git diff --check
```

## Validation Results

- The contract/proof docs are present and readable.
- The example stream parses as JSON and contains the required normalized
  fields, including per-event `schema`.
- The contract/proof/example surfaces consistently mention Observatory
  consumption, correlation, and redaction boundaries.
- No whitespace-hygiene issues are introduced by this bounded docs-only slice.

## Residual Risks

- Actual Unity/Observatory implementation could still drift if future issues do
  not follow the contract.
- Some upstream artifacts still lack the strongest possible correlation refs,
  which means Observatory must remain truthful about unavailable linkage.
- `public_report_view` / `observatory_projection` safe projections remain a
  contract requirement until later code paths implement them directly.

## Reviewer Conclusion

`#3710` should be evaluated as a truthful consumer-side contract for
Observatory readiness:

- enough to route v0.92 work onto governed ADL logging truth;
- enough to define normalized fields, correlation, retention, and redaction
  expectations;
- not a claim that the Unity app or OTEL export is complete.

# Integrated Logging And OTel Proof (#4718)

Issue: #4718
Version: v0.91.7
Status: issue-local proof harness added

## Proof Command

```bash
bash adl/tools/test_pr_v0917_integrated_observability_proof.sh
```

Focused verifier:

```bash
bash adl/tools/test_pr_v0917_integrated_observability_proof_contract.sh
```

## What This Proves

- `pr.sh doctor 4718 --json --allow-open-pr-wave` keeps machine-readable JSON
  parse-safe on stdout while ADL observability events remain on stderr and the
  compatibility log.
- Current retained samples use
  `schema=adl.observability.event.v1` and carry issue-correlated
  control-plane, runtime, provider-adapter, and proof artifact fields.
- Retained samples are checked for private repo/home/tmp paths and
  secret-looking markers before publication.
- The proof packet records the v0.91.7 OTel boundary as an export-compatible
  mapping surface, not as a production collector or OTLP exporter claim.

## Retained Evidence

The proof command writes the current retained packet to:

- `docs/milestones/v0.91.7/review/observability_4718/generated/proof_summary.json`
- `docs/milestones/v0.91.7/review/observability_4718/generated/current_event_samples.log`
- `docs/milestones/v0.91.7/review/observability_4718/generated/doctor_stdout_summary.json`

The generated packet intentionally does not retain raw `doctor --json` output
because that lifecycle truth may include machine-local paths. Instead it keeps
the parse-safety result and redacted status summary.

## OTel Boundary

Implemented now:

- ADL shared-vocabulary event samples and JSON proof summaries that can be
  mapped to OTel fields later.

Not claimed:

- no production OpenTelemetry collector;
- no OTLP exporter;
- no hosted telemetry service;
- no exporter crate wiring;
- no Unity editor execution claim from this proof alone.

Mapping authority remains
`docs/milestones/v0.91.5/SHARED_OBSERVABILITY_AND_OTEL_CONTRACT_3705.md` and
`docs/milestones/v0.91.5/OPEN_TELEMETRY_INTEGRATION_BOUNDARY_3709.md`.

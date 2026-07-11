# CSM Vector Observability Proof (#5117)

## Outcome

CSM owns Vector as its observability runtime component. Operators do not start a
Vector service and no `ADL_CSM_VECTOR_BIN` input is required. The stable CSM
installation path provisions a pinned Vector executable at `.adl/bin/vector`,
verifies the release archive checksum, and records component provenance.

Vector remains a supervised child process because the upstream project ships a
single MPL-2.0 executable rather than a stable embeddable Rust library API. CSM
owns config generation, validation, startup observation, health, permanent
restart with bounded backoff, durable ingress, redaction, and graceful drain.

## Real Proof

The proof used the official Vector `0.56.0` release. No fake Vector program was
used for the integration or cloud proof.

- Local lifecycle: Vector validated the generated config, reached ready with a
  live PID, accepted five signal classes, retained five redacted outputs,
  recovered from a forced child exit with a new PID and `restart_count: 1`, and
  drained to stopped.
- OpenTelemetry: the CSM-managed Vector process encoded an OTLP protobuf log and
  delivered it to a second real Vector `opentelemetry` receiver on reserved proof
  port `19956`. The receiver retained the decoded record with service name
  `csm`, scope `csm-observability-proof`, and the redacted authorization value.
- CloudWatch Logs: the business AWS profile received the redacted
  `issue-5117-real-vector` log in `/agent-logic/csm/wp07/issue-5117`.
- CloudWatch Metrics: the native `aws_cloudwatch_metrics` sink published
  `ADL/CSM:observability_proof` with `component=observability` and `source=csm`.
- Retention: the proof log group has a 365-day retention policy. Local proof
  artifacts remain under `.adl/proofs/issue-5117/` and contain no credentials or
  account identifiers.

Focused commands:

```text
bash adl/tools/install_vector_component.sh
cargo test --manifest-path adl-runtime/Cargo.toml observability -- --nocapture
cargo run --quiet --manifest-path adl-runtime/Cargo.toml --example observability_vector_proof
```

The AWS and OTLP proof lanes applied only approved endpoint/profile inputs to
the same example; they did not change runtime code or bypass CSM lifecycle.

## Routing Boundary

Vector has native CloudWatch Logs, CloudWatch Metrics, and OpenTelemetry sinks,
and those are configured directly by the observability component. Vector has no
native EventBridge sink. EventBridge publication therefore remains on the
governed AWS SDK cloud-bridge path implemented by dependency issue #5115. This
issue does not substitute an unsigned HTTP call or claim that an EventBridge
receipt was produced by Vector.

## Packaging And Licensing

- Version: `0.56.0`
- License: MPL-2.0
- macOS ARM64 archive SHA-256:
  `9aa8b6772d7c887734d38c84eb721d3a067e08a4aa4dc0dcc809365da242ec16`
- Linux ARM64 musl archive SHA-256:
  `afa383a264e7ab373dac68281cd86fb808f8447bb3813c08b5b0baaae0314a05`
- Linux x86_64 musl archive SHA-256:
  `8c114c5e9fd9646516f014d5d837690447cf0d4f43ba4a3746713bc0612b039b`

The installer retains the upstream license beside the generated component and
records source URL, version, platform, checksum, license, and installed ref in
`.adl/bin/.provenance/vector.json`.

## Truth Boundary

The runtime status reports remote routes only when their complete configuration
is present. `live_cloud_delivery_proven` remains false in ordinary runtime
status because child health is not an AWS delivery receipt. The separate live
AWS evidence above is the delivery proof. EventBridge closure remains ordered
behind #5115; publication of #5117 remains ordered behind #5114, #5115, and
#5116.

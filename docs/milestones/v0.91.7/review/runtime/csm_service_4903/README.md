# CSM Service Envelope Proof (#4903)

Status: `implemented_local_service_envelope_proof`

This packet records the #4903 CSM service-manager envelope proof. The runtime
owner remains the standalone `csm` binary. The service envelope invokes
`csm daemon`; it does not reintroduce `adl agent daemon`.

Retained surfaces:

- `agent.yaml`: repo-local long-lived agent spec used for the proof.
- `service/`: generated CSM service envelope artifacts.
- `service/service_manifest.json`: service-manager manifest and path policy.
- `service/service_status.json`: final service status after graceful stop.
- `service/csm.launchd.plist`: launchd-compatible host service-manager plist.
- `service/logs/observability.log`: CSM service/daemon observability log.
- `service/logs/otel.jsonl`: local OTel-compatible event export.
- `service/logs/otel_status.json`: local OTel monitor status.
- `state/daemon_status.json`: daemon runtime status.
- `state/status.json`: recoverable agent terminal status.
- `state/continuity_checkpoint.json`: recoverable partial checkpoint.
- `state/continuity_replay_manifest.json`: replay/restore manifest.
- `state/operator_events.jsonl`: operator/runtime event stream.

Truth boundary:

- Launchd plist generation and service envelope path policy are retained.
- `service_manifest.json` and `csm.launchd.plist` are sanitized with `<repo>`
  path markers for repository-retained path hygiene; the local proof commands
  ran against concrete worktree paths before sanitization.
- Local service mode provides the bounded start/status/stop proof without
  mutating the operator's host launchd domain.
- Host reboot, `kill -9`, disk-full, resource exhaustion, hosted/cloud
  orchestration, and multi-host failover are explicit non-claims.

Focused validation:

- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
- `cargo check --manifest-path adl/Cargo.toml --bin csm --bin adl`
- `cargo test --manifest-path adl/Cargo.toml --test cli_smoke csm_service -- --nocapture`
- `bash adl/tools/validate_v0917_csm_service_4903_status.sh`

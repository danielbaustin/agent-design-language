# V0.91.6 Runtime ACIP + AEE + Memory Proof (#4546)

This review packet records the bounded runtime proof for issue `#4546`.

## Scope

- Prove one ACIP local invocation packet with positive and negative cases.
- Prove one temporary-agent execution path through the AEE/control-path artifact writer.
- Prove one reviewer-readable ObsMem handoff request derived from the same retained runtime packet.

## Non-Claims

- This does not prove scheduler CLI/runtime advisory consumption; that remains with `#4544`.
- This does not prove Unity/Observatory integration.
- This does not prove external transport, federation, or `v0.92` readiness.

## Retained Packet

- Reviewer root: `docs/milestones/v0.91.6/review/runtime/v0916_acip_aee_memory_4546`
- Machine-readable proof: `docs/milestones/v0.91.6/review/runtime/v0916_acip_aee_memory_4546/runtime_acip_aee_memory_proof.json`
- AEE/control-path run: `docs/milestones/v0.91.6/review/runtime/v0916_acip_aee_memory_4546/artifacts/runtime-4546-acip-aee-memory`
- ACIP matrix: `docs/milestones/v0.91.6/review/runtime/v0916_acip_aee_memory_4546/acip/acip_integration_matrix.json`
- ObsMem request: `docs/milestones/v0.91.6/review/runtime/v0916_acip_aee_memory_4546/obsmem/transition_memory_request.json`

## Acceptance Mapping

- ACIP successful, denied, malformed, and failed-delivery cases are retained together.
  - Positive: `acip/acip_positive_packet.json`
  - Malformed: `acip/acip_malformed_case.json`
  - Failed delivery: `acip/acip_failed_delivery_exchange.json`
- Temporary-agent execution goes through the AEE path with trace-visible delegation and control-path artifacts.
  - Delegation sequence: `requested -> policy_evaluated -> approved -> dispatched -> result_received -> completed`
  - Control-path validation: `passed`
  - Authorization decision: `approved`
  - Reviewer-facing delegated artifacts are emitted under `artifacts/runtime-4546-acip-aee-memory/runtime/comms/coding/`.
- Memory handoff remains reviewer-readable and redaction-bounded.
  - ObsMem request citations: `8`
  - ObsMem request tags: `6`
  - Signed trace evidence is generated for issue `#4546` and verified with an explicit public key, rather than copied from an older issue fixture.
  - Artifact safety scan: `passed`

## Validation

- `cargo run --manifest-path adl/Cargo.toml --bin run_v0916_acip_aee_memory_integration -- --out docs/milestones/v0.91.6/review/runtime/v0916_acip_aee_memory_4546`
  - Generated the retained runtime proof packet and reviewer-facing artifacts.
- `cargo test --manifest-path adl/Cargo.toml --bin run_v0916_acip_aee_memory_integration runtime_acip_aee_memory_packet_generates_expected_artifacts`
  - Verified the packet generator emits the expected retained proof artifacts.
- `cargo test --manifest-path adl/Cargo.toml --bin run_v0916_acip_aee_memory_integration runtime_proof_helpers_remain_reviewable`
  - Verified the helper surfaces remain reviewable and the signed trace fixture still verifies.

## Notes

- A broader `cargo test --manifest-path adl/Cargo.toml --bin run_v0916_acip_aee_memory_integration` run also exposed an unrelated pre-existing failure in `cli::pr_cmd::tests::finish::arg_render::load_finish_validation_profile_rejects_retained_changed_file_substitution`; that failure is outside the `#4546` write set and was not changed here.

# Reviewer Walkthrough

Run the proof with `cargo run --manifest-path adl/Cargo.toml --bin run_v0916_acip_aee_memory_integration -- --out docs/milestones/v0.91.6/review/runtime/v0916_acip_aee_memory_4546`.

The review question is whether issue `#4546` now has one bounded packet that shows:

1. ACIP successful, denied, malformed, and failed-delivery cases.
2. One temporary-agent path going through the AEE/control-path artifact writer.
3. One redaction-bounded ObsMem handoff request derived from the retained packet.

This packet intentionally stops short of scheduler integration, Unity/Observatory proof, and v0.92 readiness.

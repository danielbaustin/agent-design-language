# Issue 5838 Design: Provider-Neutral Multi-Agent Proof

## Decision

WP-18B runs the same versioned birthday multi-agent scenario through at least
two real, independently configured providers over the landed ACIP contract.
Success requires real provider invocations and equivalent protocol semantics;
fixtures, receipt-only adapters, cached answers, and synthetic substitution are
explicitly ineligible.

## Source Baseline

- `docs/milestones/v0.92/features/PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md`
- `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md`
- `docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md`
- `adl/tools/real_multi_agent_provider_adapter.py`
- `adl/tools/provider_demo_common.sh`
- existing v0.87-v0.91 multi-agent demos are historical inputs, not v0.92 proof.

## Owned Paths

- `adl/tools/demo_v092_provider_neutral_birthday.sh`
- `adl/tools/validate_v092_provider_neutral_proof.py`
- `adl/tools/test_v092_provider_neutral_proof.sh`
- `demos/v0.92/provider-neutral-birthday`
- `docs/milestones/v0.92/features/PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md`
- `.csdlc/evidence/5838`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Proof Matrix

Rows cover positive completion, malformed ACIP, denied capability, interrupted
turn, provider loss, unavailable required model/capability, and attempted
provider substitution. Columns record provider identity, adapter revision,
capability declaration, ACIP operation sequence, identity preservation,
bounded semantic differences, result status, redaction status, and artifact
digests. At least two real provider columns must pass the positive row.

The harness must identify provider/model truth without recording credentials,
private prompts, or unredacted payloads. One provider failing must not terminate
the Runtime or unrelated agents.

## Execution Plan

1. Verify #5832, #5834, and #5836 are landed and capture exact scenario/protocol revisions.
2. Select two available real providers with independently resolved credentials and compatible capabilities.
3. Execute the identical scenario contract and retain redacted ACIP traces.
4. Run failure and no-substitution cases, including an intentionally unavailable provider.
5. Generate the proof matrix and machine-checkable artifact index.
6. Run deterministic validators, credential/path scans, and exact-head review.

## Failure And Platform Lanes

- Provider unavailability is `blocked` or `failed`, never fixture-backed pass.
- Missing credentials are reported without revealing source paths or values.
- Provider prose and token counts may differ; identity, authority, ordering,
  ACIP operations, and result semantics may not silently diverge.
- Linux/macOS differences in provider tooling must be recorded in the matrix.

## Non-Goals

- Requiring every provider or identical text/token usage.
- Editing the ACIP protocol owned by #5832.
- Reimplementing the birthday scenario owned by #5836.
- Publishing provider payloads, credentials, or private prompts.

## Exit Evidence

Two real providers pass the same positive scenario, every negative row has a
truthful outcome, substitution attempts fail visibly, artifacts are redacted
and source-pinned, and exact-head review accepts the non-claims.

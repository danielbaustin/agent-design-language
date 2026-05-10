# D12 Inhabited CSM Observatory Flagship

Proof classification: `proving`

Primary proof packet: `runtime_v2/observatory/flagship_proof_packet.json`

Reviewer command: `adl runtime-v2 observatory-flagship-demo --out artifacts/v0911/demo-d12-observatory-flagship`

Citizen continuity basis:
- witness set: `runtime_v2/private_state/continuity_witnesses.json`
- citizen receipt set: `runtime_v2/private_state/citizen_receipts.json`
- redacted projection: `runtime_v2/observatory/private_state_projection_packet.json`
- continuity challenge: `runtime_v2/challenge/challenge_artifact.json`
- sanctuary/quarantine: `runtime_v2/private_state/sanctuary_quarantine_artifact.json`

Operator-facing result: the Observatory can explain why the citizen-state scenario is reviewable, which authority paths are refused, and which ambiguous continuity transition is frozen without exposing canonical private state.

Non-claims: personhood, first true Godel-agent birthday, raw private-state inspection, and unbounded live Runtime v2 execution remain outside this proof.

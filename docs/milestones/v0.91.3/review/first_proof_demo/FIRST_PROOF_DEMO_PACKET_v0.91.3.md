# First Proof Demo Packet v0.91.3

## Scope

`WP-09` proves one bounded claim:

- C-SDLC can carry one real transition from manifest through lifecycle, DAG,
  evidence, merge-readiness, closeout, and memory handoff with measurable
  timing and coordination metrics.

It also evaluates, separately, whether the literal five-minute target was
actually met.

## Packet Contents

- `README.md`
- `ct_demo_001_timeline_snapshot.json`
- `ct_demo_001_first_proof_metrics.json`
- `ct_demo_001_first_proof_report.md`

## Demo Command

```bash
python3 adl/tools/demo_v0913_first_proof_demo.py \
  --timeline docs/milestones/v0.91.3/review/first_proof_demo/ct_demo_001_timeline_snapshot.json \
  --out docs/milestones/v0.91.3/review/first_proof_demo
```

## Focused Validation

```bash
python3 adl/tools/validate_first_proof_demo_packet.py docs/milestones/v0.91.3/review/first_proof_demo
bash adl/tools/test_first_proof_demo_packet.sh
python3 -m py_compile adl/tools/demo_v0913_first_proof_demo.py adl/tools/validate_first_proof_demo_packet.py
```

## Proof Boundary

- The first-proof classification may be `proving` even when the literal
  five-minute target remains `non_proving`.
- The packet must preserve the governance boundary: no bypass of human review,
  GitHub PRs, CI checks, or closeout truth.
- Signed trace, live ObsMem ingestion, and repeatability remain later
  milestone work.

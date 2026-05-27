# v0.91.4 Demo Matrix

## Status

Planned demo surface. Demo rows are not complete until the v0.91.4 issue wave
produces tracked proof.

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Date: `2026-05-25`
- Owner: ADL maintainers
- Related issues / work packages: planned v0.91.4 issue wave, WP-02 through
  WP-14, and CodeFriend sidecar `CF-PRE-02` through `CF-PRE-04`

## Purpose

Define the canonical v0.91.4 demo program: which bounded demos exist, which
milestone claims they prove, and which proof surfaces reviewers should inspect.

## How To Use

- Use this document for planned milestone evidence, not broad feature
  brainstorming.
- Treat every row as `planned` until the named WP produces tracked proof.
- Keep CodeFriend sidecar proof separate from C-SDLC default-operation proof.
- If a claim cannot be shown through a runnable demo, record the alternate
  proof surface in the row and follow-up review.

## Scope

In scope for v0.91.4:

- lifecycle validators, routing, editor repair, and doctor/conductor truth
- actor standing, shard ownership, evidence convergence, signed trace, merge
  readiness, and ObsMem handoff
- sprint closeout enforcement, repeatability metrics, active issue migration,
  process-drift regression fixtures, and validation-tail/proof-latency evidence
- bounded CodeFriend static-site sidecar proof or truthful blocked handoff

Out of scope for v0.91.4:

- full CodeFriend alpha product behavior
- making GWS or any workspace bridge a required C-SDLC substrate
- claiming C-SDLC default operation before the release tail validates it

## Runtime Preconditions

Working directory:

```bash
# Run commands from the repository root.
pwd
```

Deterministic runtime / provider assumptions:

```bash
# Each WP must record its own focused validation command and proof surface.
# Live provider, browser, AWS, or DNS checks are required only for demos that
# explicitly depend on those substrates.
```

Additional environment / fixture requirements:

- fixture-based demos should not require private local state
- signed trace proof must name the trace/digest/signature verification command
- CodeFriend sidecar proof may depend on AWS/DNS approval or record a blocked
  handoff if approval is unavailable

## Related Docs

- Design contract: [DESIGN_v0.91.4.md](DESIGN_v0.91.4.md)
- WBS / milestone mapping: [WBS_v0.91.4.md](WBS_v0.91.4.md)
- Sprint / execution plan: [SPRINT_v0.91.4.md](SPRINT_v0.91.4.md)
- Feature proof coverage:
  [FEATURE_PROOF_COVERAGE_v0.91.4.md](FEATURE_PROOF_COVERAGE_v0.91.4.md)
- Quality gate: [QUALITY_GATE_v0.91.4.md](QUALITY_GATE_v0.91.4.md)
- Milestone checklist:
  [MILESTONE_CHECKLIST_v0.91.4.md](MILESTONE_CHECKLIST_v0.91.4.md)

## Demo Coverage Summary

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| D1 | Lifecycle validator hardening | WP-02 blocks or routes invalid/stale lifecycle states. | WP-owned validator command | valid/invalid fixture report | invalid states fail closed or route correctly | fixture-based, replayable | planned |
| D2 | Doctor/conductor routing | WP-03 maps workflow state to the correct skill. | WP-owned routing check | routing report | state-to-skill mapping matches expected lane | fixture-based, replayable | planned |
| D3 | Editor repair lane | WP-04 repairs card drift without hand edits. | WP-owned editor repair check | editor repair examples | repaired cards validate and preserve truth | fixture-based, replayable | planned |
| D4 | Software Development Polis actor standing | WP-05 records explicit roles, standing, authority boundaries, and proof duties. | `python3 adl/tools/validate_software_development_polis_packet.py docs/milestones/v0.91.4/review/software_development_polis` | `actor_standing_allowed.json`, `actor_standing_blocked.json`, and `ct_demo_001_actor_authority_boundary_report.md` | participants have explicit standing, and hidden authority escalation fails closed | fixture-based, replayable | landed |
| D5 | Shard ownership and interface freeze | WP-05 proves parallel shards have write boundaries and synchronization barriers. | `bash adl/tools/test_software_development_polis_packet.sh` | `shard_ownership_allowed.json`, `shard_ownership_blocked.json`, and `ct_demo_002_shard_conflict_report.md` | overlapping shard writes are blocked or routed before merge | fixture-based, replayable | landed |
| D6 | Evidence convergence and review synthesis | WP-06 converges transition evidence, review findings, and residual risks. | `python3 adl/tools/validate_v0914_csdlc_evidence_bundle.py docs/milestones/v0.91.4/review/evidence/csdlc` | `ct_demo_001_transition_evidence_bundle.json` and `ct_demo_001_review_synthesis.json` | proof packet contains tracked evidence, preserved findings, and residual risks | replayable from tracked evidence | landed |
| D7 | Merge-readiness and PR gate hardening | WP-07 preserves issue, branch, CI, review, evidence, trace, and closeout truth. | `bash adl/tools/test_v0914_merge_readiness_gate.sh` | `ct_demo_001_merge_gate_profile_report.md` and `ct_demo_001_merge_gate_snapshot.json` | gate refuses stale truth and records the local-vs-remote merge-state boundary without overclaiming live reconciliation | fixture/snapshot-based | landed |
| D8 | ObsMem transition memory integration | WP-08 feeds tracked outcome truth, review findings, residual risks, follow-ons, and signed-trace evidence into a replayable memory handoff boundary. | `bash adl/tools/test_v0914_obsmem_transition_memory.sh` | `ct_demo_001_obsmem_transition_memory_handoff.json` and `ct_demo_001_transition_outcome_truth.json` | handoff preserves outcome facts, review findings, residual risks, and follow-ons distinctly while rejecting local-only durable inputs | replayable from tracked repo records | landed |
| D9 | Sprint closeout enforcement | WP-09 prevents sprint advance or closeout over stale child truth. | WP-owned sprint fixture | sprint state/closeout fixture | stale child truth blocks advancement | fixture-based, replayable | planned |
| D10 | Signed trace proof | WP-06 emits and verifies tracked trace/digest/signature evidence. | `bash adl/tools/test_v0914_csdlc_evidence_bundle.sh` | signed trace fixture plus verification lane in `review/evidence/csdlc/fixtures/` | signature/digest verification passes and tampering fails closed | replayable from tracked trace bundle | landed |
| D11 | Repeatable five-minute sprint | WP-10 measures repeated bounded transitions and validation-tail/proof-latency behavior. | WP-owned repeatability command or runbook | metrics report with validation-tail/proof-latency and parallel-validation evidence | repeated runs record timing and bottleneck truth | repeatability requires multiple runs | planned |
| D12 | Parallel Validation Fabric | WP-10 proves validation lanes are issue-local, shardable, cache-aware, and truthful about pending/deferred/blocking proof. | WP-owned PVF runbook or fixture | Parallel Validation Fabric feature/proof packet | no failed or pending proof is hidden behind aggregate success | fixture/runbook-based, replayable where possible | planned |
| D13 | Active issue migration policy | WP-11 classifies active issues into migrate, defer, leave, fold, or block. | WP-owned migration audit | sampled migration audit | sample classifications are explicit and reviewable | audit is reproducible from issue/card state | planned |
| D14 | Process-drift regression fixtures | WP-12 catches legacy SRP drift, stale SOR truth, skipped closeout, and shared-state hazards. | WP-owned regression command | regression fixture report | known drift cases fail closed | fixture-based, replayable | planned |
| D15 | Best-demo showcase, including Unity-facing C-SDLC proof | WP-13 packages the strongest reviewer-facing demos, including a Unity-facing demo if available. | WP-owned demo showcase runbook | demo showcase packet and proof index | demos are inspectable, bounded, and do not overclaim release state | demo proof may be live or recorded with replay notes | planned |
| D16 | CodeFriend pre-alpha welcome page sidecar | CF-PRE-02 through CF-PRE-04 prove static welcome page and HTTPS path, or truthful blocked handoff. | sidecar verification command/runbook | CodeFriend repo/source-map proof, page proof, CloudFront/DNS verification or blocked handoff | welcome page is reachable over HTTPS or blocker is recorded | live proof if available, otherwise blocked handoff | planned |

## Coverage Rules

- Every major milestone claim maps to a runnable demo or explicit alternate
  proof surface.
- Every demo names one primary proof surface that a reviewer can inspect.
- Commands must be copy/paste-ready by the time the owning WP closes.
- Success signals must say what to check, not only that a command exits `0`.
- Determinism and replay notes must explain how stability is judged.

## Demo Details

Per-demo details are owned by the relevant WP. Until the issue wave executes,
the coverage summary is the planning source.

## Cross-Demo Validation

WP-14 should verify that demo evidence does not contradict feature proof
coverage, release readiness, or the milestone checklist.

## Determinism Evidence

Fixture-based demos should be replayable from tracked inputs. Live AWS/DNS or
provider-dependent demos must record environment assumptions and either proof
or a blocked handoff.

## Reviewer Sign-Off Surface

Reviewers should use this matrix alongside feature proof coverage, the quality
gate, and release evidence. No row should be marked complete until the owning
WP produces tracked evidence.

## Notes

The CodeFriend sidecar is included for scheduling and proof tracking only. It
does not become C-SDLC core machinery.

## Exit Criteria

- Every planned demo row is completed, blocked with an owner, or routed before
  release.
- Demo/proof status agrees with feature proof coverage and the quality gate.
- No demo overclaims C-SDLC default operation before tracked release evidence
  supports it.

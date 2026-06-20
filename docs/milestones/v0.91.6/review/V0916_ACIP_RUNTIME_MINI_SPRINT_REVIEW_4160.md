# v0.91.6 ACIP Runtime Mini-Sprint Review

Status: `retained_review_packet`
Date: 2026-06-20
Sprint umbrella: `#4160`
Retained-review cleanup issue: `#4303`

This review records retained post-closeout truth for the first executable ACIP
runtime mini-sprint. It does not re-execute the ACIP child work and does not
claim external federation, WebSocket, protobuf, or live cross-boundary agent
transport.

## Findings

No P1 findings remain in the retained review surface.

### P2: `#4163` closure evidence is issue-comment-only rather than PR-merged evidence

Live issue state shows `#4163` closed at 2026-06-19T18:52:48Z. The issue was
closed with a comment saying the executable schema and deterministic JSON
substrate were already satisfied by current `main`, with local evidence from:

```text
cargo test --manifest-path adl/Cargo.toml acip_fixture_set_v1_ -- --nocapture
```

This is weaker than the other ACIP runtime children, which have merged PR
closure evidence. The current codebase contains the ACIP schema and fixture
surface, but the retained sprint evidence must keep the `#4163` closure path
explicitly indirect.

Disposition: record and route to the `#4303` findings-resolution plan. Do not
reopen `#4163` unless a later code audit finds the claimed schema surface
missing.

### P3: No dedicated tracked ACIP runtime proof packet was retained before this review

The child wave landed code and tests, and milestone docs consume `#4160` as
closed, but a standalone retained review packet for `#4160` was absent from the
completed-sprint matrix until `#4303`.

Disposition: fixed for reviewer discovery by this retained review packet and
the matrix update.

## Child Issue Closure Truth

| Issue | Role | Observed state after review |
| --- | --- | --- |
| `#4160` | ACIP runtime mini-sprint umbrella | closed at 2026-06-20T03:40:00Z |
| `#4163` | R-00 executable schema and deterministic JSON substrate | closed at 2026-06-19T18:52:48Z through issue-comment-only evidence |
| `#4164` | R-01 local carrier and invocation execution path | closed at 2026-06-19T20:48:06Z by merged PR `#4239` |
| `#4165` | R-02 authority and fail-closed access enforcement | closed at 2026-06-20T02:14:11Z by merged PR `#4259` |
| `#4166` | R-03 artifact refs and provider-boundary adapter | closed at 2026-06-20T02:49:24Z by merged PR `#4267` |
| `#4167` | R-04 local multi-agent ACIP runtime proof | closed at 2026-06-20T03:29:07Z by merged PR `#4270` |

## Scope Check

The reviewed mini-sprint covers the local executable ACIP runtime slice:

- typed ACIP schemas and deterministic JSON fixtures;
- local carrier and invocation exchange;
- authority and fail-closed checks;
- artifact refs and provider-boundary adapter;
- first local proof path with positive and denied behavior.

It does not cover:

- external-agent federation;
- WebSocket transport;
- protobuf wire format;
- live provider networking;
- integrated runtime soak.

Those residuals remain routed through the ACIP residual, runtime soak, and
security planning lanes.

## Retained Evidence

Primary tracked evidence surfaces:

- `adl/src/agent_comms.rs`
- `adl/src/agent_comms/transport.inc`
- `adl/src/agent_comms/orchestrate/trace.inc`
- `docs/milestones/v0.91.6/features/ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md`
- `docs/milestones/v0.91.6/features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md`
- `docs/milestones/v0.91.6/review/security/ACIP_A2A_ACCESS_RULE_SECURITY_REVIEW_4021.md`
- `docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md`

Live issue/PR evidence checked during `#4303`:

- closed issues `#4160`, `#4163`, `#4164`, `#4165`, `#4166`, and `#4167`;
- merged PRs `#4239`, `#4259`, `#4267`, and `#4270`;
- issue-comment-only closure evidence for `#4163`.

## Validation Adequacy

This retained review did not rerun the ACIP runtime tests. The review evidence
is closure-state and retained-surface evidence. The prior `#4163` comment names
one local schema-fixture test command, while later child PRs provide merged
implementation evidence.

For release confidence, a later runtime/integration soak should still prove
ACIP together with Tokio runtime, AEE, scheduler/resilience/logging basics,
Observatory visibility, and memory handoff.

## Closeout Position

`#4160` is closed and has enough retained evidence for reviewer discovery once
this packet is present. The only material caveat is the indirect `#4163`
closure path.

## Non-Claims

- This review does not claim ACIP external federation is implemented.
- This review does not claim WebSocket or protobuf completion.
- This review does not claim integrated runtime soak has completed.
- This review does not reinterpret `#4163` as PR-merged work.

# v0.91.5 Demo Matrix

## Status

Demo readiness matrix updated after the WP-13 demo mini-sprint child issues landed. This matrix is a planning and review surface, not a substitute for issue, PR, or proof-packet truth.

## Metadata

- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-06-15`
- Owner: ADL maintainers
- Status: `review_ready_wp_13_closeout`

## Purpose

Record the demo and proof surfaces needed for the v0.91.5 bridge, and distinguish landed demo evidence from v0.92 first-birthday work that remains outside this mini-sprint.

## How To Use

Use this matrix to decide whether the bridge is demonstrably ready for v0.92. A row may be marked complete only when it cites landed issue or PR evidence, or explicitly records a blocked/deferred disposition.

## Scope

Scope covers AEE completion proof routing, multi-agent C-SDLC proof, provider/model matrix proof, public prompt packet proof, demo readiness, and demo evidence consumed by the WP-18 v0.92 activation/final preflight.

## Runtime Preconditions

Provider-backed tests must record credentials, model identity, and skipped/blocked state truthfully without leaking secrets. Demo rows that are illustrative rather than runtime proof must say so explicitly.

## Demo Coverage Summary

| Demo | Issues / PRs | Proof expectation | Status |
| --- | --- | --- | --- |
| AEE completion tranche | `#3526`, `#3528`, `#3534`, `#3377` | Closure criteria and future proof/demo expectations for steering, queue/wake/handoff, policy stops, trace/replay, and bounded end-to-end AEE behavior. | Partially routed: `#3534` closed; `#3377` remains WP-18/Sprint 4 launch-packet work. |
| Multi-agent C-SDLC workcell | `#3415`, `#3501`-`#3504`; `#3484` is closed/satisfied evidence | Bounded issue execution with role, shard, provider, review, and closeout truth. | Prior/adjacent evidence retained; not re-opened by this demo mini-sprint. |
| Provider/model matrix | `#3501`, `#3505` | Hosted, local Ollama, remote Ollama, and OpenRouter model-lane evidence. | Sprint 2/provider-matrix work; not closed by WP-13 demo closeout. |
| Public prompt packet pilot | `#3472`-`#3476` | Exported, redacted, reviewer-indexed prompt packets. | Prior public-prompt work; not part of this demo mini-sprint closeout. |
| Starharvest browser-game proof refresh | `#3458`, PR `#3496` | Browser/gameplay proof refresh for the v0.91.3 HTML game demo lineage. | Complete for WP-13 demo-readiness purposes; source proof is [Starharvest browser proof](../v0.91.4/review/demo_showcase/STARHARVEST_BROWSER_PROOF_v0.91.4.md). |
| ADL Creative Room | `#3459`, PR `#3500` | Creative-production demo concept, artifact expectations, proof packet, and non-claims. | Complete for WP-13 demo-readiness purposes; source proof is [Creative Room proof packet](../v0.91.4/review/demo_showcase/CREATIVE_ROOM_PROOF_PACKET_v0.91.4.md). |
| Celestial Rescue / Unity demo | `#3460`, PR `#3680` | Unity-facing demo artifact and truthful environment/proof boundaries. | Complete as a v0.91.5 demo artifact; source proof is [Celestial Rescue Unity proof packet](../v0.91.4/review/demo_showcase/CELESTIAL_RESCUE_UNITY_PROOF_PACKET_v0.91.5.md), and it does not prove full Unity Observatory production readiness. |
| Demo showcase index and proof map | `#3461`, PR `#3684` | Index/proof map that packages the demo story and separates runnable proof from planning/deferred states. | Complete for WP-13 demo mini-sprint closeout; source proof map is [demo showcase index](../v0.91.4/review/demo_showcase/DEMO_SHOWCASE_INDEX_v0.91.5.md). |
| Demo showcase umbrella | `#3455` | Umbrella truth alignment across Starharvest, Creative Room, Celestial Rescue, proof map, D17 visibility, and WildClawBench non-scope. | Ready for closeout after this matrix/readiness update lands. |
| v0.92 first-birthday readiness | `#3377`, `#3502` | Activation map, go/no-go checklist, and launch-packet handoff. | Deferred to WP-18/Sprint 4: `#3502` closed; `#3377` remains open. |

## Coverage Rules

- Every demo claim must cite an issue, PR, artifact, or explicit blocked/deferred disposition.
- Multi-agent proof must record roles, providers, shards, review, and closeout.
- Provider/model proof must distinguish direct hosted, OpenRouter, local Ollama, and remote Ollama substrates.
- Prompt packet proof must pass redaction and validation gates.
- Demo readiness must separate runnable proof from future demo planning.
- WildClawBench remains parked/out of scope for the WP-13 demo mini-sprint.

## Demo Details

### Starharvest browser-game proof refresh

`#3458` / PR `#3496` refreshed the Starharvest browser-game proof. This row is complete for the WP-13 demo-readiness story and does not require rebuilding the game from scratch. Proof packet: [Starharvest browser proof](../v0.91.4/review/demo_showcase/STARHARVEST_BROWSER_PROOF_v0.91.4.md).

### ADL Creative Room

`#3459` / PR `#3500` landed the creative-production demo path. This is an illustrative/demo surface, not a runtime correctness proof for unrelated ADL subsystems. Proof packet: [Creative Room proof packet](../v0.91.4/review/demo_showcase/CREATIVE_ROOM_PROOF_PACKET_v0.91.4.md).

### Celestial Rescue / Unity demo

`#3460` / PR `#3680` landed the Unity-facing Celestial Rescue demo artifact. The result is useful demo substrate for v0.92, but it does not by itself prove the future Unity Observatory product surface. Proof packet: [Celestial Rescue Unity proof packet](../v0.91.4/review/demo_showcase/CELESTIAL_RESCUE_UNITY_PROOF_PACKET_v0.91.5.md).

### Demo showcase index and proof map

`#3461` / PR `#3684` packaged the demo showcase index and proof map. That work is the main source-backed closure evidence for the `#3455` umbrella. Proof map: [demo showcase index](../v0.91.4/review/demo_showcase/DEMO_SHOWCASE_INDEX_v0.91.5.md).

### Multi-agent C-SDLC workcell

The workcell proof remains visible as adjacent evidence for the milestone story. This WP-13 demo closeout does not reopen or revalidate the full multi-agent proof lane.

### AEE completion tranche

`#3534` closed the AEE completion tranche/pull-forward plan, while `#3377` remains the first-birthday launch-packet consumer. This matrix must not imply the birthday packet is complete.

### Provider/model matrix

Provider/model matrix proof belongs to the provider/multi-agent sprint lane. WP-13 demo closeout may reference it as adjacent context, but does not close that evidence lane.

### Public prompt packet pilot

The public prompt packet pilot remains a separate proof lane and is not part of the WP-13 demo closeout.

### v0.92 first-birthday readiness

`#3377` remains open and owns the final first-birthday launch packet. Demo readiness produced here is an input, not a replacement.

## Cross-Demo Validation

The final demo review should confirm there is no contradiction between the AEE completion tranche, multi-agent proof, provider matrix, public prompt records, and v0.92 activation map. At WP-13 closeout time, the demo-specific rows have landed, while provider/multi-agent and first-birthday readiness remain separately owned.

## Determinism Evidence

- [WP_ISSUE_WAVE_v0.91.5.yaml](WP_ISSUE_WAVE_v0.91.5.yaml)
- [V092_ACTIVATION_TEST_MAP_v0.91.5.md](V092_ACTIVATION_TEST_MAP_v0.91.5.md)
- [DEMO_AND_UNITY_OBSERVATORY_READINESS_v0.91.5.md](features/DEMO_AND_UNITY_OBSERVATORY_READINESS_v0.91.5.md)
- Closed demo issues: `#3458`, `#3459`, `#3460`, `#3461`
- Merged demo PRs: `#3496`, `#3500`, `#3680`, `#3684`

## Reviewer Sign-Off Surface

Reviewers should sign off on WP-13 demo closeout only after confirming that demo-specific rows are either complete or explicitly deferred, and that `#3377` remains a separate WP-18/Sprint 4 launch-packet issue.

## Exit Criteria

- Every WP-13 demo row is complete, blocked, or deferred with owner and rationale.
- `#3455` can close after this matrix and the readiness feature doc land.
- `#3573` can close only after its sprint-level closeout note records demo proof status and the remaining WP-18/Sprint 4 activation risks.

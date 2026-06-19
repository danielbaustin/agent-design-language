# v0.91.6 Active Sprint Execution Packets

Status: `draft_for_sprint_umbrellas`
Date: 2026-06-18
Source issues: `#4066`, `#4126`

This packet records SEP-style execution notes for the active `v0.91.6` sprint
umbrellas. It is tracked so future sessions do not need to reconstruct sprint
order, parallelism, watcher expectations, or PVF assumptions from chat.

Durable GitHub issue comments:

- Resilience sprint `#3967`: <https://github.com/danielbaustin/agent-design-language/issues/3967#issuecomment-4747488901>
- Provider sprint `#3970`: <https://github.com/danielbaustin/agent-design-language/issues/3970#issuecomment-4744068495>
- ACIP sprint `#3971`: <https://github.com/danielbaustin/agent-design-language/issues/3971#issuecomment-4744069116>
- Security sprint `#3972`: <https://github.com/danielbaustin/agent-design-language/issues/3972#issuecomment-4744069644>
- Identity sprint `#3973`: <https://github.com/danielbaustin/agent-design-language/issues/3973#issuecomment-4747488885>
- Observatory sprint `#3974`: <https://github.com/danielbaustin/agent-design-language/issues/3974#issuecomment-4747488889>
- Memory/ACP sprint `#3975`: <https://github.com/danielbaustin/agent-design-language/issues/3975#issuecomment-4747488897>

Closeout-tail standard:

- Milestone closeout-tail sprint standard: [CLOSEOUT_TAIL_SPRINT_v0.91.6.md](../../CLOSEOUT_TAIL_SPRINT_v0.91.6.md)

These notes do not execute child issues and do not replace issue-local
`SIP -> STP -> SPP -> SRP -> SOR` truth.

Shared watcher policy for active sprint umbrellas:

- Assign a watcher immediately for checks, review, mergeability, or upstream dependency wait states.
- While actively blocked, watcher polling should be no slower than every 30 seconds.
- Healthy waiting states stay issue-local lifecycle states; they are not a reason to abandon the sprint.

## Resilience Sprint SEP

- Sprint umbrella: `#3967`
- Title: `[v0.91.6][WP-02][resilience] Resilience layer mini-sprint umbrella`
- Execution mode: `hybrid`

### Child Issue Wave

| Issue | Role | Status | Notes |
|---|---|---|---|
| `#3986` | `R-00` schemas and substrate foundation | pending | First serial input. Establishes source resilience vocabulary and reusable substrate shape. |
| `#3987` | `R-01` retry pattern | pending | Can run after R-00 if touched modules stay isolated. |
| `#3988` | `R-02` timeout pattern | pending | Can run after R-00 if touched modules stay isolated. |
| `#3989` | `R-03` circuit-breaker pattern | pending | Can run after R-00 if touched modules stay isolated. |
| `#3990` | `R-04` rate-limiter pattern | pending | Can run after R-00 if touched modules stay isolated. |
| `#3991` | `R-05` bulkhead pattern | pending | Can run after R-00 if touched modules stay isolated. |
| `#3992` | `R-06` fallback/degraded execution pattern | pending | Can run after R-00 if touched modules stay isolated. |
| `#3993` | `R-07` integration and proof | pending | Final serial integration and proof gate. |

### Recommended Execution Order

1. Run `#3986` first.
2. Run `#3987` through `#3992` in parallel only after the schemas/substrate truth is stable and write sets are confirmed disjoint.
3. Run `#3993` last.
4. Close `#3967` only after R-07 review and sprint closeout truth are complete.

### Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| resilience pattern lanes | `#3987`, `#3988`, `#3989`, `#3990`, `#3991`, `#3992` | Retry/timeout/circuit-breaker/rate-limiter/bulkhead/fallback can progress independently once the shared substrate contract exists. | Re-check touched paths and shared substrate assumptions before PR publication. |

### Serial Gates

| Gate | Blocks | Exit condition |
|---|---|---|
| schemas/substrate gate | all pattern lanes | `#3986` establishes the source resilience vocabulary and reusable substrate. |
| integration gate | `#3993` | The six pattern issues are merged, explicitly blocked, or explicitly deferred with owner truth. |
| closeout gate | `#3967` closure | `#3993` records integration proof and residual-risk truth. |

### PVF Notes

- Each child issue should use focused behavior proof rather than broad runtime theater.
- Aggregate sprint proof must not hide blocked, skipped, deferred, or failed pattern lanes.
- Integration proof should make bypass exceptions explicit if any covered surface still escapes the resilience layer.

## Provider Sprint SEP

- Sprint umbrella: `#3970`
- Title: `[v0.91.6][WP-05][provider] Provider/model reliability and profiles v2 mini-sprint umbrella`
- Execution mode: `hybrid`

### Child Issue Wave

| Issue | Role | Status | Notes |
|---|---|---|---|
| `#4007` | `M-00` provider/capability profile catalog | pending | First serial input. Establishes catalog truth. |
| `#4008` | `M-01` provider/model role suitability matrix | pending | Depends on catalog shape. |
| `#4009` | `M-02` Gemma/OpenRouter reliability proof | pending | Can run after catalog/matrix are stable. |
| `#4010` | `M-03` provider failure-mode/resilience integration | pending | Can run in parallel with reliability proof if write sets are disjoint. |
| `#4011` | `M-04` private endpoint fixture sanitation | pending | Can run in parallel with reliability proof if fixture paths are isolated. |
| `#4053` | `M-06` C-SDLC role-provider profiles | pending | Consumes catalog/matrix and should run after `#4007`/`#4008`. |
| `#4012` | `M-05` closeout matrix | pending | Final serial closeout. |

### Recommended Execution Order

1. Run `#4007` first.
2. Run `#4008` after `#4007`.
3. Run `#4009`, `#4010`, and `#4011` in parallel only after catalog/matrix truth is stable and write sets are confirmed disjoint.
4. Run `#4053` after the catalog and matrix can support role-provider profile decisions.
5. Run `#4012` last.
6. Close `#3970` only after the child closeout matrix is truthful.

### Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| provider proof lanes | `#4009`, `#4010`, `#4011` | Separate proof/resilience/sanitation surfaces if write sets stay isolated. | Re-check touched paths before PR publication. |

### Serial Gates

| Gate | Blocks | Exit condition |
|---|---|---|
| catalog gate | `#4008`, `#4053` | `#4007` merged or explicitly accepted as source truth. |
| role-profile gate | `#4012` | `#4053` completed or explicitly deferred. |
| closeout gate | `#3970` closure | All child states and residual risks recorded. |

### PVF Notes

- Issue-local focused proof is sufficient for each child issue.
- Aggregate sprint proof must not hide failed, skipped, blocked, or pending provider lanes.
- Provider/reliability proofs should record model/provider identity, failure class, and observability notes when applicable.

## ACIP Sprint SEP

- Sprint umbrella: `#3971`
- Title: `[v0.91.6][WP-06][acip] Decide ACIP, A2A, and provider communications route`
- Execution mode: `hybrid`

### Child Issue Wave

| Issue | Role | Status | Notes |
|---|---|---|---|
| `#4013` | `C-00` communication schema catalog/profile boundaries | pending | First serial input. |
| `#4014` | `C-01` capability-based delegation/provider-message boundary | pending | Can proceed after schema catalog. |
| `#4015` | `C-02` ACIP/A2A access rules/authority boundaries | pending | Can proceed after schema catalog. |
| `#4016` | `C-03` JSON/protobuf/WebSocket projection boundaries | pending | Depends on boundary decisions. |
| `#4017` | `C-04` external-agent citizen/guild/capability-market routing | pending | Depends on access-rule/boundary decisions. |
| `#4055` | `C-06` Agent Comms 1.0 message substrate | pending | Should run after communication boundaries are coherent. |
| `#4018` | `C-05` protocol decision closeout proof | pending | Final serial closeout. |

### Recommended Execution Order

1. Run `#4013` first.
2. Run `#4014` and `#4015` in parallel after `#4013` if write sets are disjoint.
3. Run `#4016` and `#4017` in parallel after capability/access boundaries are stable.
4. Run `#4055` after boundary decisions can support the message-substrate contract.
5. Run `#4018` last.
6. Close `#3971` only after protocol decision truth and residual routing are recorded.

### Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| boundary lanes | `#4014`, `#4015` | Delegation boundary and access-rule boundary can be reviewed independently after schema catalog. | Cross-check authority vocabulary before publication. |
| projection/routing lanes | `#4016`, `#4017` | Projection mechanics and external-agent routing can progress in separate docs/code slices. | Reconcile terms before `#4055`. |

### Serial Gates

| Gate | Blocks | Exit condition |
|---|---|---|
| schema catalog gate | all other ACIP children | `#4013` establishes source vocabulary. |
| boundary convergence gate | `#4055`, `#4018` | `#4014`/`#4015` decisions are resolved or routed. |
| closeout gate | `#3971` closure | `#4018` records final protocol decision truth. |

### PVF Notes

- Parallel review lanes should classify protocol schema, authority, projection, and routing proof separately.
- The aggregate decision packet must make unresolved protocol or access-rule gaps visible.

## Security Sprint SEP

- Sprint umbrella: `#3972`
- Title: `[v0.91.6][WP-07][security] Complete security bridge readiness and CAV route`
- Execution mode: `hybrid`

### Child Issue Wave

| Issue | Role | Status | Notes |
|---|---|---|---|
| `#4019` | `S-00` security bridge completion ledger | pending | First serial input. |
| `#4020` | `S-01` provider model and CAV trust-boundary review | pending | Can run after ledger. |
| `#4021` | `S-02` ACIP/A2A access-rule security review | pending | Can run after ledger. |
| `#4022` | `S-03` public-record and memory/profile security review | pending | Can run after ledger. |
| `#4023` | `S-04` Unity Observatory inhabitant-readiness security review | pending | Can run after ledger. |
| `#4064` | `S-06` CAV threat taxonomy and corpus route | pending | Consumes CAV/security source packets. |
| `#4024` | `S-05` security bridge closeout proof | pending | Final serial closeout. |

### Recommended Execution Order

1. Run `#4019` first.
2. Run `#4020`, `#4021`, `#4022`, and `#4023` in parallel after the ledger is stable if review surfaces are disjoint.
3. Run `#4064` after the CAV/source-packet relationship is clear.
4. Run `#4024` last.
5. Close `#3972` only after bridge readiness, residual security risk, and CAV routing are recorded.

### Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| security review lanes | `#4020`, `#4021`, `#4022`, `#4023` | Distinct trust boundaries can be reviewed independently after ledger truth. | Findings must be normalized before closeout. |

### Serial Gates

| Gate | Blocks | Exit condition |
|---|---|---|
| ledger gate | all review lanes | `#4019` identifies source truth and completion ledger. |
| CAV route gate | `#4024` | `#4064` records threat taxonomy/corpus route or explicit deferral. |
| closeout gate | `#3972` closure | `#4024` records final bridge readiness and residual risk. |

### PVF Notes

- Parallel security review lanes are valid only if each lane records scope, trust boundary, evidence, and residual risk independently.
- CAV taxonomy/corpus work should make failed, blocked, deferred, or unreviewed threat categories visible.
- No sprint-level security summary may imply certification or complete security coverage.

## Identity Sprint SEP

- Sprint umbrella: `#3973`
- Title: `[v0.91.6][WP-08][identity] Identity continuity and capability selector mini-sprint umbrella`
- Execution mode: `hybrid`

### Child Issue Wave

| Issue | Role | Status | Notes |
|---|---|---|---|
| `#4025` | `I-00` identity/capability/citizen/profile boundaries | pending | First serial input. |
| `#4026` | `I-01` capability evidence consumption boundary | pending | Can run after I-00 if terms stay aligned. |
| `#4027` | `I-02` identity continuity positive/negative cases | pending | Can run after I-00 if terms stay aligned. |
| `#4028` | `I-03` selector bridge integration | pending | Consumes I-01, I-02, and WP-06 delegation outputs. |
| `#4029` | `I-04` closeout proof | pending | Final serial closeout. |

### Recommended Execution Order

1. Run `#4025` first.
2. Run `#4026` and `#4027` in parallel after `#4025` if capability evidence and identity continuity surfaces stay coordinated.
3. Run `#4028` after I-01/I-02 and WP-06 delegation outputs stabilize.
4. Run `#4029` last.
5. Close `#3973` only after selector bridge truth and residual routing are recorded.

### Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| identity evidence lanes | `#4026`, `#4027` | Capability evidence and identity continuity can progress independently once the shared boundary model exists. | Reconcile provider/capability/identity/citizen vocabulary before publication. |

### Serial Gates

| Gate | Blocks | Exit condition |
|---|---|---|
| boundary gate | all downstream identity child issues | `#4025` defines the source vocabulary and boundary truth. |
| selector convergence gate | `#4028`, `#4029` | `#4026` and `#4027` are stable enough to consume, or explicitly routed with truth. |
| closeout gate | `#3973` closure | `#4029` records final bridge truth and Aptitude Atlas non-claim truth. |

### PVF Notes

- Proof must keep provider, capability, identity, and citizen records distinct.
- Negative cases, privacy boundaries, and failure modes must remain visible if unresolved.
- Aggregate sprint proof must not hide deferred or blocked identity-risk surfaces.

## Observatory Sprint SEP

- Sprint umbrella: `#3974`
- Title: `[v0.91.6][WP-09][observatory] Working Unity Observatory mini-sprint umbrella`
- Execution mode: `hybrid`

### Child Issue Wave

| Issue | Role | Status | Notes |
|---|---|---|---|
| `#4030` | `O-00` Unity Observatory baseline and boundary | pending | First serial input. |
| `#4031` | `O-01` launchable baseline | pending | Must establish the governed launch surface. |
| `#4032` | `O-02` data contract and ingestion path | pending | Can run after O-01 if shared scene/state assumptions stay coordinated. |
| `#4033` | `O-03` inhabitant-ready surfaces | pending | Can run after O-01 if shared scene/state assumptions stay coordinated. |
| `#4034` | `O-04` logging/security/consumption integration | pending | Consumes O-02/O-03 plus WP-03/WP-07 outputs. |
| `#4035` | `O-05` working Observatory closeout | pending | Final serial closeout. |

### Recommended Execution Order

1. Run `#4030` first.
2. Run `#4031` next.
3. Run `#4032` and `#4033` in parallel after O-01 if the data contract and inhabitant surfaces stay coordinated.
4. Run `#4034` after O-02/O-03 plus WP-03/WP-07 outputs.
5. Run `#4035` last.
6. Close `#3974` only after working Observatory proof and inhabitant-readiness evidence are recorded.

### Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| Observatory contract lanes | `#4032`, `#4033` | Data-ingestion and inhabitant-facing surfaces can progress independently once the launchable baseline exists. | Reconcile shared scene/state assumptions before `#4034`. |

### Serial Gates

| Gate | Blocks | Exit condition |
|---|---|---|
| baseline gate | all downstream Observatory work | `#4030` and `#4031` establish the governed launchable baseline. |
| consumption gate | `#4034`, `#4035` | O-02 and O-03 are stable enough to classify safely. |
| closeout gate | `#3974` closure | `#4035` records working Observatory truth and inhabitant-readiness evidence. |

### PVF Notes

- Working Unity Observatory proof is required unless the operator explicitly accepts a blocked state.
- Observatory/logging/security gaps must remain visible rather than being hidden inside a generic readiness claim.
- Aggregate sprint proof must distinguish substrate, rehearsal, proof, blocked, and deferred surfaces honestly.

## Memory / ACP Sprint SEP

- Sprint umbrella: `#3975`
- Title: `[v0.91.6][WP-10][memory] AEE, ObsMem, Memory Palace, and ACP mini-sprint umbrella`
- Execution mode: `hybrid`

### Child Issue Wave

| Issue | Role | Status | Notes |
|---|---|---|---|
| `#4036` | `A-00` feature completion ledger | pending | First serial input. |
| `#4037` | `A-01` AEE completion boundary | pending | Can run after A-00 if boundaries remain distinct. |
| `#4038` | `A-02` ObsMem handoff/readiness | pending | Can run after A-00 if boundaries remain distinct. |
| `#4039` | `A-03` Memory Palace architecture/proof path | pending | Can run after A-00 but must stay aligned with WP-02 and WP-08. |
| `#4040` | `A-04` ACP/cognitive-profile scope and privacy boundary | pending | Can run after A-00 if boundaries remain distinct. |
| `#4041` | `A-05` feature closeout matrix | pending | Final serial closeout. |

### Recommended Execution Order

1. Run `#4036` first.
2. Run `#4037`, `#4038`, `#4039`, and `#4040` in parallel after A-00 if AEE, ObsMem, Memory Palace, and ACP boundaries remain distinct.
3. Keep `#4039` coordinated with WP-02 resilience and WP-08 identity continuity before closeout.
4. Run `#4041` last.
5. Close `#3975` only after feature truth and residual routing are recorded.

### Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| memory/accounting lanes | `#4037`, `#4038`, `#4039`, `#4040` | AEE, ObsMem, Memory Palace, and ACP can progress in parallel once the completion ledger exists and each lane preserves its boundary. | Reconcile privacy, profile-boundary, and long-context assumptions before `#4041`. |

### Serial Gates

| Gate | Blocks | Exit condition |
|---|---|---|
| completion-ledger gate | all downstream memory/ACP work | `#4036` establishes the source completion ledger. |
| Memory Palace coordination gate | `#4041` | `#4039` stays aligned with resilience and identity-continuity inputs or is explicitly routed truthfully. |
| closeout gate | `#3975` closure | `#4041` records final feature truth and residual routing. |

### PVF Notes

- AEE completion, Memory Palace posture, and ACP/cognitive-profile scope must stay evidence-bound and not collapse into provider profiles.
- Privacy, access, and residual implementation gaps must remain visible.
- Aggregate sprint proof must distinguish complete, blocked, deferred, and routed memory/profile surfaces clearly.

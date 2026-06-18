# v0.91.6 Active Sprint Execution Packets

Status: `draft_for_sprint_umbrellas`
Date: 2026-06-18
Source issue: `#4066`

This packet records SEP-style execution notes for the active v0.91.6 provider,
ACIP, and security sprint umbrellas. It is tracked so future sessions do not
need to reconstruct sprint order, parallelism, or PVF assumptions from chat.

Durable GitHub issue comments:

- Provider sprint `#3970`: <https://github.com/danielbaustin/agent-design-language/issues/3970#issuecomment-4744068495>
- ACIP sprint `#3971`: <https://github.com/danielbaustin/agent-design-language/issues/3971#issuecomment-4744069116>
- Security sprint `#3972`: <https://github.com/danielbaustin/agent-design-language/issues/3972#issuecomment-4744069644>

These notes do not execute child issues and do not replace issue-local
`SIP -> STP -> SPP -> SRP -> SOR` truth.

## Provider Sprint SEP

- Sprint umbrella: `#3970`
- Title: `[v0.91.6][WP-05][provider] Complete provider/model reliability and multi-agent readiness`
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

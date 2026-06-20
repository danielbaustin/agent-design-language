# Local Polis SSM Operations Bridge Review for #4109

## Scope

This packet records the bounded review and routing result for
`.adl/docs/TBD/LOCAL_POLIS_SSM_OPERATIONS_BRIDGE.md` under `#4109`.

The source doc is directionally strong. It correctly frames AWS Systems Manager
as an operations and observability bridge for a local polis host rather than as
an authority layer for polis state, governance, memory, identity, or model
content.

The source doc remains a local/TBD planning input. This issue keeps that source
local and records the tracked routing result separately in this review packet.

This review does not implement AWS integration, create credentials, or perform
live cloud operations.

## Review Decision

The bridge should proceed as a tightly bounded operations-plane proof with three
explicit constraints:

1. Local polis authority remains local.
2. AWS SSM may operate on the host and export bounded evidence, but it must not
   become the source of truth for polis semantics.
3. The first proof stays intentionally small: one approved host, one bounded
   read-only command path, one logging/export path, and one redaction proof.

## Route Classification

| Route | Decision | Notes |
| --- | --- | --- |
| Local planning source | keep local input | `.adl/docs/TBD/LOCAL_POLIS_SSM_OPERATIONS_BRIDGE.md` should remain a local/TBD planning source for now rather than being promoted directly into a milestone feature doc. |
| Near-term implementation | proceed through existing tracked issues | `#3902` remains the account/foundation gate. `#4113` is the correct first implementation/proof issue once account approval and this readiness boundary are in place. |
| Security requirements | explicit and mandatory | Least privilege, read-only command posture, no inbound SSH dependency, redaction, and explicit host/cloud authority boundaries are required before implementation proof can count. |
| Deferred operations work | split into follow-ons | Reusable node installers, non-AWS cloud enrollment, Raspberry Pi or edge-node proofs, and fleet-scale rollout remain separate future work. |
| Scheduler relationship | coordinate, do not integrate yet | Scheduler work may later prioritize or queue this kind of task, but SSM is not part of the current scheduler decision substrate and should not be treated as scheduler authority. |
| Provider relationship | keep separate | SSM is a host operations channel, not a provider transport, provider identity layer, or model-selection authority. |

## Security And Authority Boundary

The source doc's strongest claim is also the most important one to preserve:
AWS may carry operational evidence about a local polis node, but it must not
own local polis state semantics.

Required boundary rules:

- AWS SSM is an operations/control-plane bridge only.
- Local polis state, governance, memory, identity, and model authority remain
  local ADL/runtime concerns.
- The first command surface should be read-only and narrowly allowlisted.
- The first proof should not require inbound SSH.
- Tracked artifacts must not expose credentials, account ids, secret paths,
  private key material, model-private payloads, or unapproved host-private
  paths.
- CloudWatch and optional S3 export surfaces must be treated as publication
  surfaces that require sanitization, not raw debug sinks.
- IAM/SSM policy should fail closed to the minimum host operations required for
  the bounded proof.

If implementation uncovers a need for broader credential governance or trust
boundary work, that work should coordinate with the active security route rather
than being absorbed silently into the proof issue.

## Relationship To Scheduler Work

`#4107` established Cognitive Scheduler v1 as a deterministic planning surface
that assigns work lanes and emits auditable decision artifacts. Its review
packet explicitly does not claim timed execution, GitHub mutation, or live
provider/model selection.

That means SSM should currently be treated as adjacent to scheduler work, not as
part of the scheduler itself:

- scheduler may later decide when governor/operator attention should be applied
  to remote host operations
- scheduler does not currently become a timed SSM command runner
- SSM command execution should not be described as scheduler authority or as a
  hidden automation substrate in `v0.91.6`

If future work adds watch, retry, or cadence semantics around SSM operations,
that should be routed as explicit scheduler or runtime follow-on work rather
than implied here.

## Relationship To Provider Execution

The provider architecture separates transport, provider/vendor identity, stable
ADL model references, and provider-native model ids.

The local polis SSM bridge should remain outside that provider layer:

- SSM is not a provider transport
- SSM is not a provider/vendor identity surface
- SSM should not select models or providers
- SSM may invoke bounded local status or inventory commands on a host, but that
  is operational host control, not provider execution semantics

If a later issue wants SSM-triggered provider health collection, it should do so
through local ADL-owned commands that preserve the existing provider boundary.
It should not turn AWS into the direct provider-execution authority.

## Readiness Boundary For #4113

This review is sufficient to define the design/readiness boundary expected by
`#4113`.

`#4113` may proceed only when both of the following are true:

1. `#3902` identifies the approved AWS account target or the operator approves a
   transitional account.
2. This review packet is accepted as the tracked boundary for what the first SSM
   proof is and is not claiming.

For that first proof, `#4113` should stay bounded to:

- one approved host such as `wuji`
- one managed-node enrollment proof
- one bounded read-only local status command
- one observable logging/export path
- one redaction/security proof
- one operator runbook with rollback and emergency-access boundaries

## Follow-On Routing

Existing tracked implementation route:

- `#4113` first bounded local polis SSM proof after `#3902` and this review
  boundary

Future issue candidates, if and when needed:

- reusable polis-node installer/runbook for later managed nodes
- non-AWS cloud node enrollment proof under the same operations-plane boundary
- small edge-node proof such as Raspberry Pi enrollment
- fleet-scale inventory and patch/compliance rollout after the one-host proof
- explicit scheduler/runtime integration only if timed or automated operations
  are intentionally adopted later

## Non-Claims

This packet does not claim:

- live AWS setup or validation
- credential issuance or account creation
- fleet management
- provider execution through AWS
- scheduler-owned remote execution
- cloud authority over polis state
- non-AWS or edge-node portability proof

## Conclusion

The source doc should be accepted as a good local planning input with one key
normalization: in tracked ADL truth, AWS SSM belongs on the operations side of
the boundary, never on the polis-authority side.

With that boundary made explicit, the correct route is:

- keep the substantial design source local for now
- treat this packet as the tracked readiness/routing record
- let `#3902` clear the account gate
- let `#4113` perform the first bounded implementation proof
- route broader automation, portability, and fleet claims into separate
  follow-on work

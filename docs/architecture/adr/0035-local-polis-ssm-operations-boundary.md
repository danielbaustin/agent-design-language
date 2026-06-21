# ADR 0035 Candidate: Local Polis SSM Operations Boundary

- Status: Candidate
- Target milestone: v0.91.6
- Related issues: #4109, #4113, #4318, #4319, #4343
- Related ADRs: ADR 0024, ADR 0028
- Source evidence:
  - `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_OPERATIONS_BRIDGE_4109.md`
  - `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4113.md`
  - `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4318.md`
  - `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4319.md`
  - `docs/milestones/v0.91.6/review/V0916_RUNTIME_AWS_LOCAL_OPERATIONS_MINI_SPRINT_REVIEW_4343.md`
  - `docs/tooling/QNAP_QTS_SSM_ONBOARDING.md`

## Context

v0.91.6 introduced a local polis operations track that connects local machines
to AWS Systems Manager for bounded operations, inventory, host status, and
evidence collection. That track is valuable because it gives ADL a cheap hybrid
operations substrate for `wuji`, `nessus.local`, `opticon.local`, future edge
nodes, EC2 hosts, and non-AWS machines.

The same capability can become dangerous if SSM is treated as polis authority.
AWS SSM can observe and operate hosts. It must not become the source of truth
for governance, identity, memory, runtime semantics, provider selection,
decision authority, or model content.

## Decision

ADL should allow AWS Systems Manager to serve as a local polis operations-plane
bridge for explicitly approved hosts.

The SSM bridge may manage:

- host registration and managed-node inventory
- operational health and bounded status evidence
- command execution for approved maintenance tasks
- CloudWatch/S3/logging export where separately approved
- security-baseline checks and SSM onboarding proof

The SSM bridge must not own:

- local polis state or governance authority
- actor identity, standing, memory, or shard ownership
- runtime semantics or cognitive execution authority
- provider/model selection or model-content truth
- C-SDLC issue, card, PR, review, or closeout truth

## Consequences

### Positive

- Gives local and cloud hosts one operations bridge without requiring every
  polis node to run in AWS.
- Makes host health and evidence collection easier to integrate with release
  review.
- Lets ADL reuse CloudWatch, S3, IAM, inventory, and session-management
  primitives while preserving local authority boundaries.
- Provides a durable boundary before expanding to NAS, Windows, Raspberry Pi,
  EC2, or non-AWS cloud nodes.

### Negative

- SSM onboarding now has architecture significance and must be reviewed as an
  authority boundary, not just a setup task.
- Operators must distinguish operations access from governance permission.
- Later automation must avoid turning convenient remote command execution into
  hidden runtime authority.

## Alternatives Considered

### Treat SSM as a full polis control plane

This is rejected. It would centralize too much authority in an external cloud
service and blur the distinction between operations infrastructure and ADL
runtime/governance truth.

### Avoid SSM for local nodes

This preserves locality, but gives up a useful managed operations substrate and
would force ADL to rebuild inventory, command, log, and evidence plumbing too
early.

## Validation Notes

This candidate should be reviewed against the v0.91.6 SSM bridge proof packets,
the runtime AWS/local operations mini-sprint review, and the QNAP onboarding
guide. Promotion should confirm that no proof packet claims SSM owns polis
state or runtime authority.

## Non-Claims

- This ADR does not deploy SSM on any additional host.
- This ADR does not make AWS the canonical polis control plane.
- This ADR does not authorize unattended runtime mutation through SSM.
- This ADR does not store credentials, offer codes, or private billing details
  in repository artifacts.

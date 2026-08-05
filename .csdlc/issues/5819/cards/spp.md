# Structured Planning Prompt

Template: 1.0.0

Issue: 5819

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify organization gates, transfer exactly five repositories serially with assignee-aware manifests and zero unexplained drift, execute the ADL website/integration cutover last, prove exclusions, and retain a reviewed final report.

## Plan

Revision 11

## Steps

[
  {
    "id": "S1",
    "action": "Capture complete digest-bound before manifests and assignee-aware transfer plans for all five repositories",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Transfer serially and compare recomputed per-surface before/after evidence plus live destination state",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Complete website cutover, controls, final report, validators, and exact-revision review",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Transfer one repository at a time and pass verification before continuing
- Preserve repository name, visibility, default branch, and exact history
- Record secret names and scopes only, never values
- Use redirects only as temporary compatibility
- asksifu remains personal and Horust is untouched

## Risks

- Assignee loses eligibility after organization transfer
- Packages, OIDC, Pages, or integrations retain old-owner coupling
- Fork-network or destination-name conflict blocks transfer
- Transfer-back fails to restore organization-owned settings
- Concurrent publication changes the manifest during a window

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5819/design.md

Digest: 2d7445037b1d3a4909ae3015173a89c17842054186fde492cab4fbeb71d25290

## Diagram

.csdlc/prepared/issues/5819/diagram.mmd

Digest: 204d0f3b1c0bcb0a1821d0253e340cb71a0f802ace72f599bcca643f8bb3bf4b

## Stop Conditions

- WP-01B or #5815 plan lineage is not current
- Destination owner, billing, recovery, or security gate is incomplete
- Before-manifest drift is unexplained
- An assignee lacks membership and an approved reassignment plan
- Any transferred repository fails Gate 4 verification
- A secret value appears in retained evidence

## Handoff

Proceed only after doctor readiness.

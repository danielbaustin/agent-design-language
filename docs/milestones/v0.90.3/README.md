# v0.90.3 Planning Package

## Status

Draft milestone package. v0.90.3 is planned as the citizen-state substrate
milestone and is being prepared during v0.90.2 under issue #2269.

The issue wave has not been opened. This package is the reviewable planning
source for a later WP-01 issue-wave creation pass.

## Thesis

v0.90.3 should make citizen state safe enough to build on.

v0.90.2 proves the first bounded CSM run: boot, admission, governed episodes,
snapshot, rehydrate, wake, quarantine, hardening, and Observatory evidence.
That makes continuity visible, but the underlying citizen state is still too
thin for later moral, emotional, identity, migration, and birthday claims.

v0.90.3 turns citizen state into a protected continuity substrate:

- private state is not ordinary debug data
- JSON projections are review surfaces, not authority
- lineage is append-only and auditable
- wake and migration require continuity witnesses
- ambiguous continuity preserves evidence instead of optimizing through doubt
- the Observatory explains state without becoming a raw private-state browser
- standing, guest participation, communication, and inspection boundaries are
  explicit

## Directory Shape

- root planning docs and WP YAML live in this directory
- feature contracts live under `features/`
- context and later-band backgrounders live under `ideas/`

## Scope Boundary

In scope:

- citizen-state inheritance and gap audit from v0.90.2
- canonical private citizen-state format
- signed envelopes, trust-root fixture, and local-first key management
- optional encryption-at-rest prototype or envelope fixture
- append-only lineage ledger
- continuity witnesses and citizen-facing receipts
- anti-equivocation detection
- sanctuary and quarantine semantics
- redacted Observatory projections
- citizen, guest, standing, service actor, and naked-actor boundaries
- access-control semantics for inspection, projection, migration, wake,
  quarantine, challenge, appeal, and decryption
- projection policy for private, citizen-facing, operator, reviewer, public,
  and debug views
- continuity challenge and appeal flow
- citizen-state threat model
- one integrated citizen-state proof demo
- decision on whether a narrow resource-stewardship bridge is needed before
  v0.90.4 economics

Out of scope:

- first true Gödel-agent birthday
- full v0.91 moral, emotional, kindness, humor, wellbeing, cultivation, or
  harm-prevention substrate
- v0.92 identity/capability rebinding, migration, or birthday record
- full citizen economics, contracts, bids, market simulation, subcontracting, or
  inter-polis trade
- payment settlement, Lightning, x402, or other payment rails
- cloud enclave dependency for the first implementation slice
- unrestricted operator inspection of private citizen state

## Local-First Sealing Direction

v0.90.3 should be enclave-ready but local-first.

The preferred first implementation path is sealed quintessence checkpoints:
encrypted and signed continuity-bearing citizen-state checkpoints with
append-only lineage, redacted projections, continuity witnesses, and
citizen-facing receipts.

The initial backend can use local sealing primitives such as age, OS keychain
integration, TPM/Secure Enclave/YubiHSM-style options, or a deterministic test
fixture. Cloud confidential-computing paths such as Nitro Enclaves, Confidential
Space, or confidential VMs remain later backend candidates, not v0.90.3
dependencies.

## Canonical Planning Docs

- Vision: `VISION_v0.90.3.md`
- Design: `DESIGN_v0.90.3.md`
- Work Breakdown Structure: `WBS_v0.90.3.md`
- Sprint plan: `SPRINT_v0.90.3.md`
- Decisions log: `DECISIONS_v0.90.3.md`
- Demo matrix: `DEMO_MATRIX_v0.90.3.md`
- Feature index: `FEATURE_DOCS_v0.90.3.md`
- Milestone checklist: `MILESTONE_CHECKLIST_v0.90.3.md`
- Release plan: `RELEASE_PLAN_v0.90.3.md`
- Release notes draft: `RELEASE_NOTES_v0.90.3.md`
- Issue wave draft: `WP_ISSUE_WAVE_v0.90.3.yaml`

## Execution Rule

This package is planning truth, not an execution claim. The WP issue wave must
be created from the reviewed YAML before implementation starts.

Once the wave opens, issue work should happen in issue worktrees. Root checkout
edits are not part of the ADL execution model.

## Compression Rule

v0.90.3 should use the compression model learned in v0.90.1 and v0.90.2:

- make format, envelope, ledger, projection, and access contracts explicit
  before implementation widens
- keep proofs small and reviewer-visible
- front-load fixtures and negative cases
- keep demo claims narrow
- let docs-only and fixture-only WPs run with focused validation
- do not compress away witness, redaction, threat-model, or release-truth work

## Economics Boundary

Citizen economics matters because resource allocation, memory retention, compute
access, attention, and bandwidth affect citizen life. But full economics and
contract-market implementation belongs to the separate v0.90.4 planning lane
tracked by #2271.

v0.90.3 should make a decision only about whether it needs a narrow
resource-stewardship bridge to protect citizen continuity.

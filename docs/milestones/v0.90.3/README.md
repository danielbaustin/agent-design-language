# v0.90.3 Planning Package

## Status

Active milestone package. v0.90.3 is the citizen-state substrate milestone. It
was created during v0.90.2 under issue #2269 and refined during the v0.90.2
WP-19 handoff pass (#2263).

The issue wave is open as #2327-#2347. WP-01 is #2327, WP-02 through WP-14 are
#2328-#2340, WP-14A is #2341, and WP-15 through WP-20 are #2342-#2347.

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
- access-control semantics for inspection, decryption, projection, migration,
  wake, quarantine, challenge, appeal, and release
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
- Citizen-state inheritance audit: `CITIZEN_STATE_INHERITANCE_AUDIT_v0.90.3.md`
- Append-only lineage ledger proof: `APPEND_ONLY_LINEAGE_LEDGER_v0.90.3.md`
- Continuity witnesses and receipts: `CONTINUITY_WITNESSES_AND_RECEIPTS_v0.90.3.md`
- Anti-equivocation conflict proof: `ANTI_EQUIVOCATION_CONFLICT_v0.90.3.md`
- Sanctuary/quarantine behavior proof: `SANCTUARY_QUARANTINE_BEHAVIOR_v0.90.3.md`
- Redacted Observatory projection proof: `REDACTED_OBSERVATORY_PROJECTIONS_v0.90.3.md`
- Standing and communication boundary proof: `STANDING_COMMUNICATION_BOUNDARY_v0.90.3.md`
- Access-control semantics proof: `ACCESS_CONTROL_SEMANTICS_v0.90.3.md`
- Continuity challenge and appeal proof: `CONTINUITY_CHALLENGE_APPEAL_v0.90.3.md`
- WP execution readiness: `WP_EXECUTION_READINESS_v0.90.3.md`
- Feature index: `FEATURE_DOCS_v0.90.3.md`
- Milestone checklist: `MILESTONE_CHECKLIST_v0.90.3.md`
- Release plan: `RELEASE_PLAN_v0.90.3.md`
- Release notes draft: `RELEASE_NOTES_v0.90.3.md`
- Issue wave: `WP_ISSUE_WAVE_v0.90.3.yaml`

## Execution Rule

This package is execution planning truth, not a release-completion claim. The
WP issue wave has been created from the reviewed YAML, and every WP issue body
inherits the relevant `WP_EXECUTION_READINESS_v0.90.3.md` section.

Issue work should happen in issue worktrees. Root checkout edits are not part of
the ADL execution model.

Every execution session should preserve the relevant required outputs,
validation, and non-goals before implementation.

## Compression Rule

v0.90.3 should use the compression model learned in v0.90.1 and v0.90.2:

- make format, envelope, ledger, projection, and access contracts explicit
  before implementation widens
- keep proofs small and reviewer-visible
- front-load fixtures and negative cases
- keep demo claims narrow
- let docs-only and fixture-only WPs run with focused validation
- do not compress away witness, redaction, threat-model, or release-truth work

The release tail should preserve the proven v0.90.2 pattern: demo/proof matrix,
docs and quality convergence, internal review, external review, accepted-finding
remediation, next-milestone handoff, and release ceremony.

## Economics Boundary

Citizen economics matters because resource allocation, memory retention, compute
access, attention, and bandwidth affect citizen life. But full economics and
contract-market implementation belongs to the separate v0.90.4 planning lane
tracked by #2271.

v0.90.3 should make a decision only about whether it needs a narrow
resource-stewardship bridge to protect citizen continuity.

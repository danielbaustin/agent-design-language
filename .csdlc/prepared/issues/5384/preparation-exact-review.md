## Findings

- P0: none.
- P1: none.
- P2 — Actionable: **yes** — the declared execution worktree is internally inconsistent.
  - The active claim binds `/Volumes/FastWork/adl-wp-5384-final` at `.csdlc/issues/5384/index.json:17` and `.csdlc/prepared/issues/5384/bootstrap.json:16`.
  - The SIP constraint instead directs all writes to `/Volumes/FastWork/adl-wp-5384` at `.csdlc/issues/5384/cards/sip.values.json:34`, rendered at `.csdlc/issues/5384/cards/sip.md:43`, and sourced from `.csdlc/prepared/issues/5384/bootstrap.json:45`.
  - This makes preparation-only binding ambiguous and could direct later work outside the claimed worktree.
  - Bounded fix: use the typed v2 edit/regeneration route to normalize the operator constraint to `/Volumes/FastWork/adl-wp-5384-final`, re-render the SIP, validate, rerun doctor, and re-review the exact packet.
- P3: none.

## Verification

- Typed doctor: exit `2`, with the sole finding `design_review_missing_or_stale`. It accepted the six current-native card contracts and the SPP/VPP design references using the repository’s typed digest algorithm. No generic SHA-256 substitution was used.
- Cards/phases: SIP, STP, SPP, and VPP are `ready`; SRP and SOR truthfully remain `pre_phase`. Index phase is `initialized`; review, publication, readiness, and terminal state remain unset.
- Topology: the manifest and live snapshot contain the same 31 unique predecessors. The pinned WBS, issue wave, feature document, direct children, WP-10A children, Runtime parity/provider inputs, and independent acceptance inventory are covered.
- Provenance: the checker enforces the exact connector kind, repository, canonical predecessor observation digest, snapshot freshness, and applicable merged-PR observation freshness at `.csdlc/prepared/issues/5384/validate_dependency_gate.rb:30-68`.
- Terminal gate: immutable base, live closed/merged state, typed `closed_out`, shared-Git receipt, merged disposition, PR agreement, observed SHA, and ancestry are enforced at `.csdlc/prepared/issues/5384/validate_dependency_gate.rb:16-20,48-97`.
- Dependency checker: exit `3`, `ready: false`, for truthful open/missing projection/receipt/PR evidence. This is the intended promotion block and is **not** a preparation defect.
- Preparation-scope checker: exit `0`; tracked, staged, unstaged, and untracked inventory was confined to the three protected paths.
- Diff hygiene: `git diff --check` passed separately.
- HEAD/base: `HEAD` and `origin/main` both equal `09c0bd1784216dbce1ad4cdebfe2d453af6e3d9d`; ancestry passed. There are no tracked base-to-HEAD changes; all packet files are untracked and scoped.
- COTS/budget/PVF: native lane semantics, proof roles, determinism, resource profiles, budgets, acceptance mappings, and required status through null `defer_reason` are present. Budgets remain estimates, not execution proof.
- Product authority: none. The claim protects only the three preparation surfaces and explicitly excludes implementation, product, Runtime, documentation, deployment, publication, provider, and external-infrastructure authority.

Typed design approval: **not authorized** until the P2 path inconsistency is repaired and the resulting exact packet is re-reviewed.

Preparation-only bind: **not authorized** for the same reason.

Implementation/promotion remains independently blocked by the intended dependency-gate failure.

## Exact Git blob SHA-1 inventory

```text
c8aea20f4d1901692416a4da2c424a524f22d427  AGENTS.md
913d4aec502b2f420277b60a4f94079075ff79d4  .csdlc/issues/5384/audit.jsonl
7b0182b58851f258683b9d8b2ea1970fdc4621b3  .csdlc/issues/5384/cards/sip.md
f3d9b82787db7918e6be60619b1dca43e2c7229c  .csdlc/issues/5384/cards/sip.values.json
a0625aedde44324a32f0833b9e21d497b62a5f8c  .csdlc/issues/5384/cards/sor.md
01b009988526d7bedd9c1020912c5ace0ed47615  .csdlc/issues/5384/cards/sor.values.json
210c751cf937bf3c61dfae739d933f137401596b  .csdlc/issues/5384/cards/spp.md
478b268b713d2d5d448d729b5ccacf24293270a3  .csdlc/issues/5384/cards/spp.values.json
af55cc9c80f65a6bc8f1ccac91c21f04daa6fbfb  .csdlc/issues/5384/cards/srp.md
512f0a19862f7b8bf87362697e3e47e3fb49f6d2  .csdlc/issues/5384/cards/srp.values.json
16646a04be69b0eab87127b8316185f6938dca81  .csdlc/issues/5384/cards/stp.md
c8377bf9e7adf67ecec8445df24cf47e17e8eb08  .csdlc/issues/5384/cards/stp.values.json
1dcc4cb1565a5451cbf45a7216b4c5cc43549e13  .csdlc/issues/5384/cards/vpp.md
81057bee7eeb44c8f3841a8907e68028633b6e9d  .csdlc/issues/5384/cards/vpp.values.json
710e4a17c2edc8e9dc63de7a44978a8f14c9f21a  .csdlc/issues/5384/index.json
744b661f8bf5d51c52c24167c49eb2ce74205a4f  .csdlc/prepared/issues/5384/bootstrap.json
7e42ff27e154526be8b6743e915b0875601626c0  .csdlc/prepared/issues/5384/dependency-gate.json
b9fcbaf68c542bab57381469d8b4b11fc267f1d9  .csdlc/prepared/issues/5384/design.md
52a3091d702ae681365bafac967f1ccf5e25a777  .csdlc/prepared/issues/5384/diagram.mmd
f535d1ddee73b9bf8c48eb749019cb031eb1b94f  .csdlc/prepared/issues/5384/live-dependency-snapshot.json
cf707c14d6140cd1114cf7f0d7ebf1ed59e6b7fc  .csdlc/prepared/issues/5384/preparation-subagent-review-final-2.md
e1501857d35fdbce98b96efe81f8d6e2402b4d5d  .csdlc/prepared/issues/5384/preparation-subagent-review-final-4.md
d276b7a725ccde48b7f18c6a3f73fa246c8abcb1  .csdlc/prepared/issues/5384/preparation-subagent-review-final.md
bdebd65d95b233113ea2a822312da70110855562  .csdlc/prepared/issues/5384/preparation-subagent-review.md
889ae93263fc79cdfda88400f1b0a89c266fb3c4  .csdlc/prepared/issues/5384/validate_dependency_gate.rb
e120a5ae8c415ebedf1751784add530655d4d21a  .csdlc/prepared/issues/5384/validate_preparation_scope.rb
e69de29bb2d1d6434b8b29ae775ad8c2e48c5391  .csdlc/locks/5384.lock
710769caf9deb5a9d257ba03e6d61c9c8513ac12  docs/milestones/v0.91.8/WBS_v0.91.8.md
a633260f97460ff0f8146428f7b1ab9fecf3a576  docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
44cda870aa847e5d434c37dd857d5040fad1a5cb  docs/milestones/v0.91.8/features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md
dc221e810499cf522c80f9b3c2843673ae14cfbd  docs/templates/prompts/current.json
```
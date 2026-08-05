## Findings

1. **P1 — Actionable: yes — SPP/VPP pin stale design and diagram digests.**

   - `.csdlc/issues/5384/cards/spp.values.json:94-97` and `.csdlc/issues/5384/cards/vpp.values.json:109-112` declare:
     - design SHA-256 `855ad7cd...`
     - diagram SHA-256 `5dcaa986...`
   - Actual SHA-256 values are:
     - `design.md`: `b31e54ba86f97ce6966df4edac993da733a8666109f8beb81363b67429b382cb`
     - `diagram.mmd`: `d5db17ad9d953523c9615910373b945662e5d64f012c33e8012c910c6f6ea66a`

   The cards therefore do not describe the exact reviewed design packet.

   **Bounded fix:** Through typed `csdlc-edit`, refresh the SPP/VPP design and diagram digest fields, re-render both cards, run `csdlc-validate`, and re-review the resulting exact packet.

2. **P1 — Actionable: yes — Live-evidence provenance is self-asserted, not substantively enforced.**

   - `.csdlc/prepared/issues/5384/live-dependency-snapshot.json:6` supplies only the string `approved_github_connector_and_shared_git_receipts`.
   - `.csdlc/prepared/issues/5384/validate_dependency_gate.rb:31` accepts any nonempty `source`.
   - The checker therefore cannot distinguish approved connector evidence from an arbitrary locally authored assertion.
   - Global freshness is enforced at `.csdlc/prepared/issues/5384/validate_dependency_gate.rb:33-38`, but per-PR timestamps are checked only for non-emptiness at line 51.

   **Bounded fix:** Extend the typed snapshot contract with verifiable connector provenance—such as connector kind, immutable observation/query receipt or digest, and exact observed revision—and validate the allowed provenance shape. Parse and bound each applicable issue/PR observation timestamp rather than merely requiring a nonempty PR timestamp.

3. **P2 — Actionable: yes — The protected-path checker exists but is not scheduled by the native VPP lane.**

   - `.csdlc/prepared/issues/5384/validate_preparation_scope.rb:15-29` correctly inventories base-to-HEAD, staged, unstaged, and untracked paths.
   - `.csdlc/prepared/issues/5384/design.md:97-99` says both this checker and VPP hygiene are mandatory.
   - But `.csdlc/issues/5384/cards/vpp.values.json:70-74`, rendered at `.csdlc/issues/5384/cards/vpp.md:125-129`, schedules only `git diff --check`.
   - `bootstrap.json:157-165` has the same incomplete lane command.

   **Bounded fix:** Through the typed VPP/editor route, schedule `validate_preparation_scope.rb` as a required lane and retain `git diff --check` as a distinct required hygiene lane, or use a typed command surface that deterministically runs both. Re-render and validate.

No P0 or P3 findings.

## Required verification

- **Prior four findings:** Immutable-base pinning and live snapshot consumption are implemented. Tracked/untracked path inventory is implemented. Native PVF required/deferred meaning is now explicitly mapped through `proof_role` plus null `defer_reason`. The remaining scheduling and provenance deficiencies are captured above.
- **Six cards/phases:** SIP, STP, SPP, and VPP are `ready`; SRP and SOR truthfully remain `pre_phase`. The index remains `initialized`, with design review pending and no review, publication, readiness, or terminal claim (`index.json:6,25-33`).
- **Native identity:** All cards declare template `1.0.0`, matching the `csdlc_v2_native` registry entry at `docs/templates/prompts/current.json:46-49`.
- **Predecessor topology:** The 31-entry manifest covers the checked-in WP-13, acceptance, WP-14 child, WP-10A, Runtime parity/provider, and independent acceptance-inventory topology. No omission found against the pinned authority files.
- **Predecessor gate:** It is blocked as intended. The snapshot contains numerous open predecessors beginning with `#5361` at `live-dependency-snapshot.json:10`; checker lines 42-51 necessarily record failures and lines 83-91 return `ready: false`/exit 3. I did not execute it because execution would read predecessor projections and shared-Git receipts outside the explicitly authorized review surfaces.
- **Typed terminal enforcement:** Checker lines 54-80 require tracked projection, shared-Git receipt, exact `closed_out` phase, `merged` disposition, live/receipt PR agreement, nonempty observed SHA, and ancestry to the immutable expected base.
- **Protected paths:** The scope checker passed and listed all tracked/untracked packet files with no outside paths. The active claim contains only the three authorized surfaces (`index.json:18-23`).
- **Design/diagram:** Scope, evidence flow, COTS, budgets, non-claims, promotion boundary, and downstream WP-15–WP-19 order are present. Digest drift blocks acceptance of their card references.
- **COTS/budget/PVF:** Reuse decisions and planning ceilings are explicit. Native required status is represented by null `defer_reason`; no execution proof is claimed. No paid service or provider authority is introduced.
- **Product authority:** None. Design lines 24-34 and 101-107 and the protected-path claim prohibit product, Runtime, deployment, publication, and implementation work.
- **HEAD/base/diff hygiene:** `HEAD` and `origin/main` both equal `09c0bd1784216dbce1ad4cdebfe2d453af6e3d9d`; ancestry passed. `git diff --check` passed. All packet changes are untracked and confined to the three authorized surfaces.
- **Typed design approval:** **Not authorized** while the three actionable findings remain.
- **Preparation-only bind:** **Not authorized** because AC-6 requires all actionable review findings fixed before approval and bind.
- **Implementation promotion:** Independently blocked by the intentionally failing predecessor gate.

## Exact Git blob SHA-1 inventory

```text
c8aea20f4d1901692416a4da2c424a524f22d427  AGENTS.md
913d4aec502b2f420277b60a4f94079075ff79d4  .csdlc/issues/5384/audit.jsonl
7b0182b58851f258683b9d8b2ea1970fdc4621b3  .csdlc/issues/5384/cards/sip.md
f3d9b82787db7918e6be60619b1dca43e2c7229c  .csdlc/issues/5384/cards/sip.values.json
a0625aedde44324a32f0833b9e21d497b62a5f8c  .csdlc/issues/5384/cards/sor.md
01b009988526d7bedd9c1020912c5ace0ed47615  .csdlc/issues/5384/cards/sor.values.json
971e23194f7c5ec93e74918fdd7b5c8fbd12eb61  .csdlc/issues/5384/cards/spp.md
0b71833eb47b1534b9c56c84617a0c4fe81e844a  .csdlc/issues/5384/cards/spp.values.json
af55cc9c80f65a6bc8f1ccac91c21f04daa6fbfb  .csdlc/issues/5384/cards/srp.md
512f0a19862f7b8bf87362697e3e47e3fb49f6d2  .csdlc/issues/5384/cards/srp.values.json
7864cc9d439a72583b9262817b1fd201bdfce12f  .csdlc/issues/5384/cards/stp.md
b044d72d95e78176c26e6746eb640c51b2ea59d7  .csdlc/issues/5384/cards/stp.values.json
93f92df5312d4f6ba668779a76d07eeb2d3ccf5a  .csdlc/issues/5384/cards/vpp.md
0c20bf903e71b5959661389db37becad98218978  .csdlc/issues/5384/cards/vpp.values.json
9df4d2b03f2cec593351a3cca6635400a89f1ce1  .csdlc/issues/5384/index.json
ebd476fa8d30b4c70d3739899d705370d57a7cc6  .csdlc/prepared/issues/5384/bootstrap.json
7e42ff27e154526be8b6743e915b0875601626c0  .csdlc/prepared/issues/5384/dependency-gate.json
6ccfdfb2d3db019f4a58d728408b46521ddad15b  .csdlc/prepared/issues/5384/design.md
52a3091d702ae681365bafac967f1ccf5e25a777  .csdlc/prepared/issues/5384/diagram.mmd
c7423ee16ea53e4349ce91af1554a2246e44fef6  .csdlc/prepared/issues/5384/live-dependency-snapshot.json
cf707c14d6140cd1114cf7f0d7ebf1ed59e6b7fc  .csdlc/prepared/issues/5384/preparation-subagent-review-final-2.md
d276b7a725ccde48b7f18c6a3f73fa246c8abcb1  .csdlc/prepared/issues/5384/preparation-subagent-review-final.md
bdebd65d95b233113ea2a822312da70110855562  .csdlc/prepared/issues/5384/preparation-subagent-review.md
ec4aaa75e72efe6987d5e68259c7ce76ef14d009  .csdlc/prepared/issues/5384/validate_dependency_gate.rb
e120a5ae8c415ebedf1751784add530655d4d21a  .csdlc/prepared/issues/5384/validate_preparation_scope.rb
e69de29bb2d1d6434b8b29ae775ad8c2e48c5391  .csdlc/locks/5384.lock
710769caf9deb5a9d257ba03e6d61c9c8513ac12  docs/milestones/v0.91.8/WBS_v0.91.8.md
a633260f97460ff0f8146428f7b1ab9fecf3a576  docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
44cda870aa847e5d434c37dd857d5040fad1a5cb  docs/milestones/v0.91.8/features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md
dc221e810499cf522c80f9b3c2843673ae14cfbd  docs/templates/prompts/current.json
```
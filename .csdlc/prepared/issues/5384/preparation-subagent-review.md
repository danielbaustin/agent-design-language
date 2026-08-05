## Findings

1. **P1 — Actionable: yes — Dependency gate does not enforce all declared requirements.**
   `.csdlc/prepared/issues/5384/dependency-gate.json:6` requires `live_issue_closed` and `implementation_pr_merged`, but `.csdlc/prepared/issues/5384/validate_dependency_gate.rb:15` only checks local projections, shared-Git receipts, terminal disposition/SHA, and ancestry. It can return `ready: true` without any live issue or PR evidence.

   **Bounded fix:** Require a separately refreshed, issue-local evidence input containing issue state, implementation PR state, observed revision, and freshness/provenance. Validate every predecessor against that input and fail closed when evidence is missing, stale, ambiguous, non-closed, or non-merged. Keep connector access outside this local checker if required, but make the checker consume and enforce its result.

2. **P1 — Actionable: yes — Protected-path validation does not prove protected-path confinement.**
   `.csdlc/issues/5384/cards/vpp.values.json:69` describes confirmation that only the three authorized paths changed, but its command at line 83 is only `git diff --check`. That checks whitespace errors, omits path allowlisting, and does not include the currently untracked packet. It therefore cannot prove AC-4 or AC-7.

   **Bounded fix:** Replace or supplement this lane with a deterministic tracked-and-untracked path inventory relative to the pinned base, reject every path outside:

   - `.csdlc/issues/5384/**`
   - `.csdlc/prepared/issues/5384/**`
   - `.csdlc/locks/5384.lock`

   Preserve `git diff --check` as a separate hygiene check if desired.

3. **P2 — Actionable: yes — “Same refreshed origin/main revision” is not enforced atomically.**
   `.csdlc/prepared/issues/5384/validate_dependency_gate.rb:13` retains the symbolic `origin/main` reference and resolves it independently during each ancestry test at lines 42–44. The checker neither verifies freshness nor resolves one immutable base SHA before iterating. A concurrent ref update could make predecessor results refer to different revisions.

   **Bounded fix:** Accept or resolve one expected base SHA before iteration, verify it equals the separately refreshed `origin/main`, use that SHA for every ancestry check, and emit it in the result payload. Fail closed if the expected and resolved revisions differ.

4. **P2 — Actionable: yes — PVF release-gate status is claimed but absent from lane records.**
   `design.md:83` says every lane declares its release-gate role. AC-5 makes release-gate status explicit, yet the four lane objects in `vpp.values.json:19-108` and `bootstrap.json` have no `release_gate` field.

   **Bounded fix:** Through the typed VPP/editor route, add an explicit supported release-gate classification to every lane, or revise AC-5/design wording to the exact current-native field that carries equivalent semantics. Re-render and validate; do not hand-edit rendered cards.

No P0 or P3 findings.

## Other review results

- **Predecessor topology:** No omission found against the pinned WBS, issue wave, and platform acceptance feature. The manifest includes the direct WP-13 and acceptance gates, WP-14 children, WP-10A umbrella and children, Runtime parity/provider inputs, and independent acceptance inventory inputs.
- **Six-card truth:** All six native cards exist. SIP/STP/SPP/VPP are `ready`; SRP and SOR remain truthfully `pre_phase`. The issue index remains `initialized`, with review/design approval pending and no publication or terminal claim.
- **Current-native identity:** The pinned registry declares `csdlc_v2_native` template set `1.0.0`, matching the six cards.
- **COTS and budget:** Reuse decisions and bounded budgets are present. Planning ceilings are clearly distinguished from executed proof.
- **Scope:** Current filesystem changes are confined to the three authorized surfaces, but the planned validation does not prove that confinement.
- **Base:** `HEAD` and `origin/main` both resolve to `09c0bd1784216dbce1ad4cdebfe2d453af6e3d9d`.
- **Diff hygiene:** `git diff --check` passed.
- **Executable gate:** The read-only Ruby execution was blocked by the environment sandbox before it ran. Findings above derive from direct static inspection of the complete checker.
- **Disposition:** Preparation approval should remain blocked until the four actionable findings are corrected and re-reviewed.

## Exact digest inventory

Algorithm: Git blob SHA-1 of the exact current filesystem content.

```text
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
e69de29bb2d1d6434b8b29ae775ad8c2e48c5391  .csdlc/locks/5384.lock
ebd476fa8d30b4c70d3739899d705370d57a7cc6  .csdlc/prepared/issues/5384/bootstrap.json
9755e8e7e28ef68821e5dbdc2b6fe18d92cf0492  .csdlc/prepared/issues/5384/dependency-gate.json
eac41e2ca97dca66dcf2f4a74b1208961225d22e  .csdlc/prepared/issues/5384/design.md
52a3091d702ae681365bafac967f1ccf5e25a777  .csdlc/prepared/issues/5384/diagram.mmd
666887022a4ee372b988ea20ed5938724b87073a  .csdlc/prepared/issues/5384/validate_dependency_gate.rb
```

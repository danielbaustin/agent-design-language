# Work Breakdown Structure - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Date: `2026-04-14`
- Owner: `Daniel Austin`

## How To Use
- break work into independently mergeable issues once the milestone officially opens
- keep each item measurable and reviewable
- reserve the final WPs for demos, quality, docs/review convergence, and release ceremony
- treat this table and `WP_ISSUE_WAVE_v0.89.1.yaml` as the mechanical source for later issue creation rather than reopening milestone design

## WBS Summary

`v0.89.1` is organized as an adversarial/runtime follow-on milestone:
- first establish the canonical milestone package and seedable issue-wave plan
- then land the core adversarial/runtime, exploit/replay, and proof band
- then package demos, quality, review, remediation, and release closure

## Work Packages

| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | Design pass (milestone docs + planning) | Finalize the canonical `v0.89.1` package, promote the main feature docs, and map every source planning doc to an implementation home. | coherent milestone docs, feature index, seeded issue-wave plan | none | `#1922` |
| WP-02 | Adversarial runtime model | Turn the carry-forward adversarial runtime into an explicit core runtime contract. | runtime model package | `WP-01` | `#1923` |
| WP-03 | Red / blue agent architecture | Define persistent adversarial roles and their bounded interaction model. | role architecture package | `WP-01`, `WP-02` | `#1924` |
| WP-04 | Adversarial execution runner | Establish the orchestration surface for adversarial execution and evidence capture. | execution-runner package | `WP-02`, `WP-03` | `#1925` |
| WP-05 | Exploit artifact and replay schema | Make exploit artifacts and replay manifests explicit and reusable. | artifact schema + replay package | `WP-02`, `WP-04` | `#1926` |
| WP-06 | Continuous verification and self-attack patterns | Define ongoing verification and self-attack behavior as bounded execution patterns. | verification/self-attack package | `WP-02`, `WP-04`, `WP-05` | `#1927` |
| WP-07 | Adversarial demo and security proof surfaces | Land the flagship demo and the primary review/proof surfaces for the milestone. | demo/proof package | `WP-03` - `WP-06` | `#1928` |
| WP-08 | Operational skills substrate and composition | Make operational skill execution and composition explicit enough for adversarial/runtime use, including a bounded `arxiv-paper-writer` skill rooted in the Paper Sonata manuscript workflow. | substrate/composition package + `arxiv-paper-writer` skill | `WP-01`, `WP-04` | `#1929` |
| WP-09 | Delegation, refusal, and coordination follow-through | Resolve the supporting governance/coordination inputs needed to keep the adversarial band legible and bounded. | bounded governance/coordination package | `WP-03`, `WP-07`, `WP-08` | `#1930` |
| WP-10 | Provider extension and milestone packaging convergence | Decide and package what provider-security extension and related under-authored supporting inputs actually belong in this milestone. | provider extension packaging contract + converged scope record | `WP-07`, `WP-09` | `#1931` |
| WP-11 | Demo scaffolding and proof entry points | Define and land the bounded proof entry points for adversarial runtime, exploit replay, and security demos. | runnable or reviewer-legible proof entry points | `WP-02` - `WP-10` | `#1932` |
| WP-12 | Milestone convergence and follow-on mapping | Reconcile issue graph, carry-forward, and proof surfaces before the release tail starts. | converged issue graph and milestone status surfaces | `WP-02` - `WP-11` | `#1933` |
| WP-13 | Demo matrix + integration demos | Validate the milestone claims through bounded demos, integration review, and the initial three-paper publication packet. | canonical demo matrix, demo artifacts, and three-paper manuscript packet | `WP-02` - `WP-12` | `#1934` |
| WP-14 | Coverage / quality gate (ratchet + exclusions) | Run quality gates and record any bounded exceptions truthfully. | green quality gate or documented exceptions | `WP-02` - `WP-13` | `#1935` |
| WP-15 | Docs + review pass (repo-wide alignment) | Align docs, review surfaces, and release-tail truth across the repo. | converged docs/review package | `WP-13`, `WP-14` | `#1936` |
| WP-16 | Internal review | Perform bounded internal review of milestone truth and proof surfaces. | internal review record | `WP-15` | `#1937` |
| WP-17 | 3rd-party review | Perform external review of the milestone package and capture findings. | 3rd-party review record | `WP-15`, `WP-16` | `#1938` |
| WP-18 | Review findings remediation | Resolve or explicitly defer accepted review findings. | remediation record | `WP-16`, `WP-17` | `#1939` |
| WP-19 | Next milestone planning | Prepare the next milestone planning package before `v0.89.1` closeout. | next-milestone package | `WP-18` | `#1940` |
| WP-20 | Release ceremony (final validation + tag + notes + cleanup) | Close the milestone cleanly after validation and documentation are complete. | release tag, notes, and closeout | `WP-18`, `WP-19` | `#1941` |

## Sequencing
- Phase 1: establish the canonical package and the seedable issue-wave plan (`WP-01`)
- Phase 2: land the core adversarial/runtime feature band (`WP-02` - `WP-10`)
- Phase 3: package demos, quality, review, remediation, next-milestone handoff, and release closure (`WP-11` - `WP-20`)

## Acceptance Mapping
- WP-01 -> no package/template drift remains and the issue wave can seed directly from the milestone package
- WP-02 -> adversarial runtime assumptions are explicit enough to implement and review
- WP-03 -> persistent red/blue role architecture is explicit rather than implied
- WP-04 -> the execution runner is explicit and bounded
- WP-05 -> exploit and replay artifacts are explicit and reusable
- WP-06 -> continuous verification and self-attack patterns are explicit and reviewable
- WP-07 -> the adversarial proof/demo package is legible and bounded
- WP-08 -> the skill substrate/composition layer is explicit enough for runtime implementation and includes a bounded `arxiv-paper-writer` skill surface
- WP-09 -> governance/coordination supporting inputs are resolved enough to keep the milestone coherent
- WP-10 -> bounded provider capability packaging has a proof hook and under-authored provider-security extension docs are explicitly kept out
- WP-11 -> proof entry points exist for the main milestone claims
- WP-12 -> the milestone package and issue graph are converged
- WP-13 -> milestone claims have bounded demo/proof surfaces and the three-paper publication program has reviewer-legible manuscript outputs
- WP-14 -> quality/coverage posture is truthful and reviewable
- WP-15 -> repo-wide docs and review surfaces are aligned with delivered truth
- WP-16 -> bounded internal review truthfully records milestone strengths, risks, and open remediation
- WP-17 -> external review is captured as a first-class milestone artifact
- WP-18 -> accepted review findings are remediated or explicitly deferred
- WP-19 -> the follow-on milestone package is ready before closeout
- WP-20 -> milestone closes with truthful notes, tag, and follow-on capture

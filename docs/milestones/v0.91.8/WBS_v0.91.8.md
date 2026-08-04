# v0.91.8 Work Breakdown

| WP | Issue | Title | Planned outcome | Depends on |
| --- | ---: | --- | --- | --- |
| WP-01 | #5594 | Execution readiness and routing reconciliation | Reconcile live issue truth, sprint ownership, cards, dependencies, feature dispositions, and the parallel execution plan. | Historical planning #5335/#5383 and closed v0.91.7 WP-23 |
| WP-02 | #5336 | Baseline and clean-room architecture | Pin incumbent denominator, ownership, architecture, and budgets. | WP-01 |
| WP-03 | #5337 | Characterization corpus | Capture normalized v1 behavior and deterministic comparison rules. | WP-02 |
| WP-04 | #5339 | Six-primitives language core | Implement typed provider/tool/agent/task/workflow/run contracts. | WP-02, WP-03 |
| WP-05 | #5338 | Deterministic compiler | Compile validated documents into canonical execution plans. | WP-04 |
| WP-06 | #5340 | Portable execution engine | Implement bounded scheduling, retries, joins, resume, and ports. | WP-05 |
| WP-07 | #5342 | Records signing and trust contracts | Implement stable records, signing, verification, traces, artifacts, and errors. | WP-04, WP-06 |
| WP-08 | #5341 | Runtime v3 adapter | Connect ADL plans and events to Runtime v3 without changing ownership. | WP-06, WP-07, reviewed #5591 ingress contract |
| WP-09 | #5349 | Provider and governed-tool adapters | Implement mock, HTTP, governed-tool, and compatibility adapters behind typed ports. | WP-06, WP-08 |
| WP-10 | #5345 | Thin CLI and selector | Implement validate, schema, plan, run, inspect, sign, verify, and generation selection. | WP-04 through WP-09 |
| WP-10A | #5497 | Distributed C-SDLC workcell | Prove conductor, Codex task adapter, dashboard, convergence, and live distributed workcell without autonomous merge or closeout authority. | WP-09; children #5499, #5498, #5500, #5502, #5501 |
| WP-11 | #5350 | Shadow parity | Compare exact ADL v1/v2 revisions across the approved corpus and classify mismatches; Runtime v3 parity is separately owned by #5361 and #5591/#5592/#5589/#5590. | WP-03, WP-10, completed WP-10A live proof, current Runtime v3 acceptance inputs |
| WP-12 | #5344, #5343 | Soak, rollback, and reversible cutover | Run opt-in soak, prove rollback, and execute reviewed selector switch. | WP-11 and closed Runtime v3 acceptance #5361 |
| WP-13 | #5346, #5347 | Deletion wave | Delete only reviewed and replaced incumbent surfaces after eligibility proof. Deferred until immediately before WP-18 internal review. | WP-14A through WP-17; disjoint manifests |
| WP-14A | #5384 | Integrated platform acceptance and deployment | Accept and deploy ADL v2, Runtime v3, and C-SDLC v2 at exact revisions. | #5358, #5361, #5344, #5343 |
| WP-15 | #5354 | Demo convergence | Demonstrate the integrated deployed stack and claim boundaries. | WP-14A |
| WP-16 | #5351 | Quality gate | Run integrated platform quality gate. | WP-15 |
| WP-17 | #5360 | Documentation alignment | Align README, feature docs, WBS, checklist, handoff, issue wave, deployment truth. | WP-16 |
| WP-18 | #5356, #5791 | Internal review | Retain the first milestone review, then run a final second pass after residual coding. | WP-17; residual coding before #5791 |
| WP-19 | #5357 | External review | Freeze and run independent review only after the final internal second pass is consumable. | WP-18 #5791 |
| WP-20 | #5363 | Remediation and preflight | Fix accepted findings and independently owned C-SDLC tooling defects, then rerun focused/integrated checks. | WP-19 |
| WP-21 | #5362 | Feature-list and v0.92 planning truth | Prepare exact-revision handoff, launch/activation, Memory Palace, identity/birthday, capability-envelope, and Adaptive Learning inputs from reviewed deployed truth. | WP-20 |
| WP-21A | #5355 | Next-milestone closeout planning | Prepare canonical closeout-planning packet. | WP-21 |
| WP-22 | #5359 | Next-milestone planning review | Review v0.92 inputs for blockers and overclaims. | WP-21A |
| WP-23 | #5348 | Release ceremony and lifecycle closeout | Close milestone lifecycle without unsupported release claims. | WP-22 |

## WP-14A Child Topology

WP-14A consumes only C-SDLC v2 acceptance `#5358`, Runtime v3 acceptance
`#5361`, Runtime v3 soak and rollback `#5344`, and the reversible ADL selector
switch `#5343`. WP-13 deletion `#5346`/`#5347` is deliberately deferred until
immediately before internal review `#5356`; it does not block WP-14A.

Unity Observatory tooling and proof issues `#4739`, `#4741`, and `#5332` belong
to WP-15 `#5354`. They feed demo convergence independently and do not block
WP-14A platform acceptance.

C-SDLC tooling defects `#5548` and `#5558` belong to WP-20 `#5363`.

The exact-revision handoff `#5352`; launch and activation `#4758`, `#4759`, and
`#4761`; Memory Palace `#4760` and `#5007`; birth-witness and birthday
documentation `#4762` and `#4763`; and Adaptive Learning planning `#5107`
belong to WP-21 `#5362`.

Runtime v3 acceptance `#5361` is a nested umbrella under milestone sprint
`#5595`. It owns Parity-A `#5591`, then Parity-B `#5592`, Parity-C `#5589`, and
Parity-D `#5590`. These issues preserve Runtime v2 until reviewed cutover proof;
they do not extend WP-11's ADL v1/v2 parity corpus.

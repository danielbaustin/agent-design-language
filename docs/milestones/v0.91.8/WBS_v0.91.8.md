# v0.91.8 Work Breakdown

| WP | Issue | Title | Planned outcome | Depends on |
| --- | ---: | --- | --- | --- |
| WP-01 | #5383 | Planning package and routing repair | Create this package, restore `#4641`, preserve WP-14A, and align issue-wave truth. | v0.91.7 planning truth |
| WP-02 | #5336 | Baseline and clean-room architecture | Pin incumbent denominator, ownership, architecture, and budgets. | WP-01 |
| WP-03 | #5337 | Characterization corpus | Capture normalized v1 behavior and deterministic comparison rules. | WP-02 |
| WP-04 | #5339 | Six-primitives language core | Implement typed provider/tool/agent/task/workflow/run contracts. | WP-02, WP-03 |
| WP-05 | #5338 | Deterministic compiler | Compile validated documents into canonical execution plans. | WP-04 |
| WP-06 | #5340 | Portable execution engine | Implement bounded scheduling, retries, joins, resume, and ports. | WP-05 |
| WP-07 | #5342 | Records signing and trust contracts | Implement stable records, signing, verification, traces, artifacts, and errors. | WP-04, WP-06 |
| WP-08 | #5341 | Runtime v3 adapter | Connect ADL plans and events to Runtime v3 without changing ownership. | WP-06, WP-07 |
| WP-09 | #5349 | Provider and governed-tool adapters | Implement mock, HTTP, governed-tool, and compatibility adapters behind typed ports. | WP-06, WP-08 |
| WP-10 | #5345 | Thin CLI and selector | Implement validate, schema, plan, run, inspect, sign, verify, and generation selection. | WP-04 through WP-09 |
| WP-10A | #5497 | Distributed C-SDLC workcell | Prove conductor, Codex task adapter, dashboard, convergence, and live distributed workcell without autonomous merge or closeout authority. | WP-09; children #5499, #5498, #5500, #5502, #5501 |
| WP-11 | #5350 | Shadow parity | Compare exact revisions across the approved corpus and classify mismatches. | WP-03, WP-10, completed WP-10A live proof |
| WP-12 | #5344, #5343 | Soak, rollback, and reversible cutover | Run opt-in soak, prove rollback, and execute reviewed selector switch. | WP-11 |
| WP-13 | #5346, #5347 | Deletion wave | Delete only reviewed and replaced incumbent surfaces after eligibility proof. | WP-12 plus current #5358/#5361 acceptance; disjoint manifests |
| WP-14A | #5384 | Integrated platform acceptance and v0.92 handoff | Accept/deploy ADL v2, Runtime v3, C-SDLC v2, and dispose moved handoff children. | WP-13, #5358, #5361 |
| WP-15 | #5354 | Demo convergence | Demonstrate the integrated deployed stack and claim boundaries. | WP-14A |
| WP-16 | #5351 | Quality gate | Run integrated platform quality gate. | WP-15 |
| WP-17 | #5360 | Documentation alignment | Align README, feature docs, WBS, checklist, handoff, issue wave, deployment truth. | WP-16 |
| WP-18 | #5356 | Internal review | Review code, deployment, docs, proof, issue graph, and release-tail packets. | WP-17 |
| WP-19 | #5357 | External review | Run independent review after internal review is consumable. | WP-18 |
| WP-20 | #5363 | Remediation and preflight | Fix accepted findings and rerun focused/integrated checks. | WP-19 |
| WP-21 | #5362 | Feature-list and v0.92 planning truth | Prepare v0.92 inputs from reviewed deployed truth. | WP-20 |
| WP-21A | #5355 | Next-milestone closeout planning | Prepare canonical closeout-planning packet. | WP-21 |
| WP-22 | #5359 | Next-milestone planning review | Review v0.92 inputs for blockers and overclaims. | WP-21A |
| WP-23 | #5348 | Release ceremony and lifecycle closeout | Close milestone lifecycle without unsupported release claims. | WP-22 |

## WP-14A Child Topology

WP-14A consumes `#5358`, `#5361`, `#5352`, `#4758`-`#4763`, `#5007`, `#4739`,
`#4741`, `#5332`, and `#5107`. Each child must close with evidence or remain
blocked with operator-approved evidence before WP-14A can claim readiness.

# WBS - v0.90.3

## Work Package Shape

v0.90.3 should use the standard WP-01 through WP-20 release shape, with an
explicit WP-14A demo/proof lane before docs convergence. The first sprint
should establish what v0.90.2 actually produced, then lock the citizen-state
format and envelope before implementation widens.

| WP | Issue | Title | Purpose | Primary Output | Depends On |
| --- | --- | --- | --- | --- | --- |
| WP-01 | planned | Promote v0.90.3 milestone package | Finalize this planning package and create the issue wave | tracked v0.90.3 docs and issue cards | v0.90.2 closeout |
| WP-02 | planned | Citizen-state inheritance and gap audit | Compare v0.90.2 first-run artifacts against v0.90.3 citizen-state requirements | inheritance and unsafe-assumption report | WP-01 |
| WP-03 | planned | Canonical private state format | Decide and fixture the authoritative private citizen-state format | format decision, schema, projection fixture | WP-02 |
| WP-04 | planned | Signed envelope and trust root | Define signed checkpoint envelope and local trust-root fixture | envelope schema, trust-root fixture, negative tests | WP-03 |
| WP-05 | planned | Local-first key management and sealing | Define local key lifecycle and sealed checkpoint backend boundary | local key policy, sealing fixture | WP-04 |
| WP-06 | planned | Append-only lineage ledger | Make lineage history auditable and authoritative | ledger schema, head calculation, tamper tests | WP-04 |
| WP-07 | planned | Continuity witnesses and receipts | Explain major identity transitions to systems and citizens | witness schema, receipt schema, fixtures | WP-06 |
| WP-08 | planned | Anti-equivocation | Detect conflicting signed continuity claims | conflict fixture and negative test | WP-06, WP-07 |
| WP-09 | planned | Sanctuary and quarantine behavior | Preserve evidence when continuity is ambiguous or unsafe | sanctuary/quarantine semantics and tests | WP-08 |
| WP-10 | planned | Redacted Observatory projections | Show citizen state safely to operators and reviewers | projection schema, redaction tests, Observatory packet update | WP-03, WP-07 |
| WP-11 | planned | Citizen, guest, standing, and communication boundary | Bind state to actor standing and communication semantics | standing schema/events and negative tests | WP-03 |
| WP-12 | planned | Access-control semantics | Define who may inspect, decrypt, project, migrate, wake, quarantine, challenge, or appeal | authority matrix, access events, denial tests | WP-10, WP-11 |
| WP-13 | planned | Continuity challenge, appeal, threat model, and economics placement | Implement due-process artifacts, model citizen-state threats, and decide whether v0.90.3 needs only a resource-stewardship bridge before v0.90.4 economics | challenge/appeal schemas, freeze behavior, threat model, economics placement record | WP-07, WP-09-WP-12 |
| WP-14 | planned | Integrated citizen-state demo | Prove the substrate end to end in one bounded scenario | integrated proof packet and operator report | WP-03-WP-13 |
| WP-14A | planned | Demo matrix and feature proof demos | Verify every citizen-state feature claim has a runnable demo, proof packet, fixture-backed artifact, non-proving status, or explicit deferral | demo matrix update and feature proof coverage record | WP-03-WP-14 |
| WP-15 | planned | Quality gate, docs, and review convergence | Align quality posture, coverage/tracker truth, docs, README, reviewer entry surfaces, and the completed demo/proof coverage record | coherent quality gate and docs/review package | WP-14A |
| WP-16 | planned | Internal review | Run findings-first internal review over code, docs, tests, demos, issue truth, and release boundaries | internal review packet and finding register | WP-15 |
| WP-17 | planned | External / third-party review | Run bounded external review and record findings or zero-finding disposition | third-party review record | WP-16 |
| WP-18 | planned | Review findings remediation | Fix accepted internal/external findings or defer explicitly with owner and rationale | remediation PRs or deferral records | WP-16, WP-17 |
| WP-19 | planned | Next-milestone planning and handoff | Prepare v0.90.4/v0.91/v0.92 handoff and preserve later-scope boundaries before ceremony | next-milestone planning package and handoff notes | WP-18 |
| WP-20 | planned | Release ceremony | Complete release closure and handoff | release notes, ceremony result, next-milestone handoff | WP-19 |

## Compression Candidate

v0.90.3 can compress if WP-03 through WP-07 produce stable contracts and
fixtures early. Runtime implementation should not widen before format, envelope,
ledger, and witness semantics are reviewable.

Compression must not skip:

- redaction checks
- access-control evidence
- challenge/appeal behavior
- anti-equivocation negative tests
- threat modeling
- feature-by-feature demo/proof coverage before the quality gate and docs/review convergence
- internal review, external review, and accepted-finding remediation
- next-milestone planning before release ceremony
- release-truth checks

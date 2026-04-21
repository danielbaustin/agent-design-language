# WBS - v0.90.3

## Work Package Shape

v0.90.3 should use the standard 20-WP shape. The first sprint should establish
what v0.90.2 actually produced, then lock the citizen-state format and envelope
before implementation widens.

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
| WP-12 | planned | Integrated citizen-state demo | Prove the substrate end to end in one bounded scenario | integrated proof packet and operator report | WP-03-WP-11 |
| WP-13 | planned | Access-control semantics | Define who may inspect, decrypt, project, migrate, wake, quarantine, challenge, or appeal | authority matrix and access events | WP-10-WP-12 |
| WP-14 | planned | Projection policy | Define private, citizen, operator, reviewer, public, and debug views | projection policy and leakage tests | WP-10, WP-13 |
| WP-15 | planned | Continuity challenge and appeal flow | Implement due-process artifacts for challenged continuity | challenge/appeal schemas and freeze behavior | WP-07, WP-13 |
| WP-16 | planned | Citizen-state threat model and economics placement decision | Model threats before demo claims widen and decide whether v0.90.3 needs only a resource-stewardship bridge before v0.90.4 economics | threat model, negative-test candidates, and economics placement record | WP-03-WP-15 |
| WP-17 | planned | Demo matrix and feature proof demos | Verify every citizen-state feature claim has a runnable demo, proof packet, fixture-backed artifact, non-proving status, or explicit deferral | demo matrix update and feature proof coverage record | WP-12-WP-16 |
| WP-18 | planned | Docs, quality, and review convergence | Align docs, quality posture, README, reviewer entry surfaces, and the completed demo/proof coverage record | coherent docs/review package | WP-17 |
| WP-19 | planned | Internal and external review remediation | Run reviews and fix or defer accepted findings | review packets and remediation notes | WP-18 |
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
- feature-by-feature demo/proof coverage before docs/review convergence
- release-truth checks

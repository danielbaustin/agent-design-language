# Demo Matrix - v0.90.2

## Status

Issue wave open. Demo commands remain pending until the implementation WPs
land, but every planned demo now has a mapped WP issue.

| ID | Demo | WP | Proof Claim | Required Artifacts | Status |
| --- | --- | --- | --- | --- | --- |
| D1 | Inheritance and compression audit | WP-02 #2246 | v0.90.2 targets the actual v0.90.1 substrate and can execute through the compressed workflow | `RUNTIME_V2_INHERITANCE_AND_COMPRESSION_AUDIT_v0.90.2.md` | LANDED |
| D2 | CSM run packet fixture | WP-03 #2247 / WP-04 #2248 | The first CSM run has a stable packet and invariant/violation contract before coding widens | `CSM_RUN_PACKET_CONTRACT_v0.90.2.md`, CSM run fixture, invariant map, violation schema | CONTRACT-PROVING: WP-03 AND WP-04 LANDED |
| D3 | Manifold boot and citizen admission | WP-05 #2249 | `proto-csm-01` boots and admits two worker citizens with traceable identity handles | boot manifest, citizen roster, trace events | LANDED |
| D4 | Governed resource-pressure episode | WP-06 #2250 / WP-07 #2251 | A governed episode runs under resource pressure and Freedom Gate mediation | scheduling artifact, decision packet, trace | LANDED |
| D5 | Invalid action rejection | WP-08 #2252 | An invalid action is rejected through normal kernel/policy flow before commit | invalid-action fixture, negative test, violation packet, trace event | LANDED |
| D6 | Snapshot rehydrate wake continuity | WP-09 #2253 | Local snapshot, rehydrate, and wake preserve continuity without duplicate activation | snapshot bundle, wake report, continuity proof | LANDED |
| D7 | CSM Observatory visibility | WP-10 #2254 | Operator can see the first CSM run through packet/report surfaces | Observatory packet, operator report | LANDED |
| D8 | Recovery versus quarantine | WP-11 #2255 / WP-12 #2256 | Runtime distinguishes safe resume from required quarantine | recovery decision, quarantine artifact | PLANNED |
| D9 | Governed adversarial hook and hardening probes | WP-13 #2257 | One bounded adversarial scenario is contained under explicit operator rules, and duplicate activation, snapshot integrity, and replay-gap failures are detected and recorded | adversarial hook packet, rules of engagement, hardening proof packets | PLANNED |
| D10 | Integrated first CSM run proof | WP-14 #2258 | Reviewer can inspect the first bounded CSM run and its failure-boundary evidence end to end | integrated CSM run packet, trace, Observatory report, hardening artifacts | PLANNED |

## Non-Proving Boundaries

- These demos do not prove first true Gödel-agent birth.
- These demos do not prove full emotion, morality, kindness, or governance.
- These demos do not prove complete migration or cross-polis continuity.
- These demos do not prove a complete red/blue/purple defense ecology.
- The governed adversarial hook is a bounded proof surface, not an autonomous
  red-team ecology or open-ended attack simulation.
- These demos prove a bounded local CSM run, not a personhood or citizenship
  maturity claim.

# v0.93 Candidate Work Breakdown Structure

## Status

Candidate allocation only. v0.93 has no final issue wave yet.

The exact WP sequence should be produced by the v0.93 WP-01 planning pass after
v0.90.3, v0.91, and v0.92 have landed enough prerequisite evidence.

## WBS Summary

v0.93 should develop the constitutional citizenship and polis-governance layer
without stealing work from the earlier substrate milestones.

## Candidate Work Areas

| Candidate | Work Area | Description | Primary deliverable | Key dependencies |
| --- | --- | --- | --- | --- |
| A | Constitutional citizenship contract | Define eligibility, rights, duties, standing, review, and non-goals. | Feature contract and fixtures. | v0.90.3 standing/state, v0.92 identity. |
| B | Human, guest, operator, and citizen-mode boundary | Make guest-by-default human entry and mediated citizen action explicit. | Boundary doc, policy fixtures, negative cases. | v0.90.3 standing, v0.91 Freedom Gate, v0.92 identity. |
| C | Rights and duties model | Define what the polis owes citizens and what citizens owe the polis. | Rights/duties schema or contract. | v0.91 moral resources and wellbeing context. |
| D | Standing maintenance and degradation | Define evidence-based transitions among good standing, monitored, restricted, suspended, restored, and revoked. | Standing transition tests and review packet. | v0.90.3 standing and challenge flow. |
| E | Constitutional review packet | Consume trace, outcome, attribution, policy, and standing evidence. | Review packet schema and fixtures. | v0.91 moral trace and trajectory review. |
| F | Challenge and appeal flow | Preserve evidence, allow challenge, record appeal disposition, and avoid arbitrary punishment. | Challenge/appeal state machine and proof fixture. | v0.90.3 challenge/quarantine, v0.91 review evidence. |
| G | Delegation and IAM | Model delegated authority across citizens, guests, services, operators, and tools. | Authority-chain model and allow/deny fixtures. | v0.90.5 governed tools if landed. |
| H | Communication without inspection | Ensure governed communication does not create private-state inspection rights. | Communication/inspection negative proof. | v0.90.3 communication and projection policy. |
| I | Social contract representation | Represent the bounded obligations of the polis and citizens. | Draft social-contract contract and review notes. | A through H. |
| J | Polis governance health evidence | Summarize governance state without scalar moral verdicts or private-state leaks. | Governance evidence packet and redacted report. | E through I. |
| K | Demo matrix and proof demos | Build constitutional review, standing transition, delegation/IAM, and guest/citizen boundary demos. | Demo matrix rows and runnable proof commands. | A through J. |
| L | Review, docs, and release tail | Align docs, update feature list, run review, and close the milestone. | Review handoff, release notes, ceremony evidence. | All prior work. |

## Sequencing Pressure

1. Start with the citizenship contract and actor boundary.
2. Add rights, duties, and standing transition semantics.
3. Add review packet, challenge, and appeal.
4. Add delegation and IAM after authority prerequisites are clear.
5. Add communication and social-contract surfaces.
6. Build proof demos only after the contracts can constrain them.

## Acceptance Mapping

- Constitutional citizenship must be tied to identity, standing, trace, and
  policy, not merely existence in the runtime.
- Human provider participation must remain guest or operator activity unless a
  CSM identity mediates the action as citizen conduct.
- Constitutional review must cite trace/outcome/standing evidence.
- Standing changes must be evidence-based, reviewable, and appealable.
- Delegation and IAM must fail closed when authority is missing.
- Public or operator-facing projections must not leak private state.
- Demos must show behavior and evidence, not just policy text.

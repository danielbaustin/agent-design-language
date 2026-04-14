# Decisions - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Date: `2026-04-11`
- Owner: `Daniel Austin`

## Decisions

| ID | Decision | Why | Status | Date | Issue |
|---|---|---|---|---|---|
| D-01 | Consolidate `v0.88.1` back into `v0.88` | the temporal cost-policy follow-on was too small and conceptually belongs with the core temporal substrate | accepted | `2026-04-10` | local planning pass |
| D-02 | Promote the bounded temporal seven-doc package into tracked `v0.88` | the tracked milestone package needed a real reviewer-visible temporal package | accepted | `2026-04-10` | `#1579` |
| D-03 | Treat execution policy as part of the temporal schema / cost story | cost must be interpretable in light of requested execution posture, not only raw spend | accepted | `2026-04-10` | `#1579` |
| D-04 | Promote PHI metrics into tracked `v0.88` canon | the engineering integration-metrics doc belongs to the milestone and is useful enough to learn from during execution | accepted | `2026-04-11` | `#1497` |
| D-05 | Promote instinct docs into tracked `v0.88` canon | instinct / bounded-agency surfaces were still part of local `v0.88planning` truth and left the tracked milestone incomplete | accepted | `2026-04-11` | `#1497` |
| D-06 | Keep historical / exploratory planning notes local-only for now | aptitude historical copies, helper maps, and work-package notes are not yet public feature commitments | accepted | `2026-04-11` | `#1497` |
| D-07 | Use the normal `v0.86` / `v0.87` closeout pattern for `v0.88` | the milestone should not add an extra process sprint or a new release-tail shape | accepted | `2026-04-11` | `#1497` |
| D-08 | Add Paper Sonata to `v0.88` as a bounded flagship demo | the milestone needs one memorable public-facing proof surface that showcases temporal continuity, artifact truth, and bounded multi-agent orchestration | accepted | `2026-04-11` | `#1497`, protected local follow-on planning |
| D-09 | Close `v0.88` scope with only two bounded backlog pull-ins | the milestone should stop expanding; only `#1614` and `#1618` are accepted as bounded supporting inputs before execution issue seeding | accepted | `2026-04-11` | `#1497`, `#1614`, `#1618` |
| D-10 | Realize `#1618` as a bounded comparative proof row instead of a second flagship workflow | `v0.88` needs one crisp reviewer-facing explanation of what ADL adds beyond filesystem-style deep-agent demos, but that explanation should reinforce `Paper Sonata` rather than duplicate it | accepted | `2026-04-12` | `#1618` |
| D-11 | Treat `v0.88` as ceremony-ready only when review, remediation, next-milestone planning, and closed-issue truth all converge on `main` | the final release must be published from the canonical branch with the review tail completed and the closeout record passing the milestone truth gate, rather than from a feature branch or from partially synchronized release docs | accepted | `2026-04-13` | `#1663`, `#1780` |

# Structured Review Prompt

Template: 1.0.0

Issue: 5340

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

adl-v2/crates/adl-engine

## Prompts

- Does the engine consume only the landed inert #5338 plan and keep ADL plan-level scheduling distinct from Runtime v3 operational scheduling, supervision, recovery, and policy?
- Are readiness, dispatch, joins, completions, retries, cancellation, failures, and saturation fully deterministic and bounded at every limit edge?
- Can completion arrival, map order, retries, duplicate inputs, checkpoint encoding, or fresh-process resume change effects, attempts, snapshots, or final bytes?
- Do provider/tool ports carry stable typed identity and idempotency while keeping production adapters, IO, credentials, policy, and Runtime source outside WP-06?
- Does quiescent-only checkpoint/resume reject every plan, limit, budget, sequence, attempt, identity, state, or encoding mismatch without guessing about in-flight effects?
- Are every #5338 fixture classification, protected path, COTS choice, source/test budget, PVF class, time ceiling, no-deferral acceptance row, rollback action, exact-revision review, and terminal gate explicit and executable?

## Findings

[
  {
    "id": "F-5340-1-state-dataflow-unresolved",
    "severity": "p1",
    "summary": "State dependency resolution and request identity are fixed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:1eee1895ece9fb283386b4c369d46d8e82c7f972:1a1f533251e8337fba78546f56f23048e0566139986971acd6e318de4094059a",
    "route": null
  },
  {
    "id": "F-5340-2-turn-input-unbounded",
    "severity": "p1",
    "summary": "Plan, policy, turn, completion, and serialization bounds remain enforced.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:1eee1895ece9fb283386b4c369d46d8e82c7f972:1a1f533251e8337fba78546f56f23048e0566139986971acd6e318de4094059a",
    "route": null
  },
  {
    "id": "F-5340-3-resume-semantic-truncation",
    "severity": "p1",
    "summary": "Normalized successful turns are retained and replayed from the exact initial engine with full snapshot equality.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:1eee1895ece9fb283386b4c369d46d8e82c7f972:1a1f533251e8337fba78546f56f23048e0566139986971acd6e318de4094059a",
    "route": null
  },
  {
    "id": "F-5340-4-compiler-fixture-mapping-absent",
    "severity": "p1",
    "summary": "All six landed fixtures are inventoried, parsed, compiled, classified, and executed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:1eee1895ece9fb283386b4c369d46d8e82c7f972:1a1f533251e8337fba78546f56f23048e0566139986971acd6e318de4094059a",
    "route": null
  },
  {
    "id": "F-5340-5-usize-policy-contract",
    "severity": "p2",
    "summary": "JoinPolicy AtLeast.required uses fixed-width u64.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:1eee1895ece9fb283386b4c369d46d8e82c7f972:1a1f533251e8337fba78546f56f23048e0566139986971acd6e318de4094059a",
    "route": null
  },
  {
    "id": "F-5340-6-resume-journal-reachability",
    "severity": "p1",
    "summary": "Canonical exact replay rejects forged counters and coherently altered intermediate completion histories.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:1eee1895ece9fb283386b4c369d46d8e82c7f972:1a1f533251e8337fba78546f56f23048e0566139986971acd6e318de4094059a",
    "route": null
  },
  {
    "id": "F-5340-7-state-materialization-bound",
    "severity": "p1",
    "summary": "Repeated state materialization is charged before cloning and serialization is capped.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:1eee1895ece9fb283386b4c369d46d8e82c7f972:1a1f533251e8337fba78546f56f23048e0566139986971acd6e318de4094059a",
    "route": null
  },
  {
    "id": "F-5340-8-applicable-fixtures-not-executed",
    "severity": "p1",
    "summary": "Every applicable actual fixture executes language to compiler to engine without silent skips.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:1eee1895ece9fb283386b4c369d46d8e82c7f972:1a1f533251e8337fba78546f56f23048e0566139986971acd6e318de4094059a",
    "route": null
  },
  {
    "id": "F-5340-9-design-truth-drift",
    "severity": "p2",
    "summary": "Approved design and diagram accurately specify bounded normalized-turn replay and exact snapshot reconstruction.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:1eee1895ece9fb283386b4c369d46d8e82c7f972:1a1f533251e8337fba78546f56f23048e0566139986971acd6e318de4094059a",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Replay cost grows with the retained turn journal, but journal size and turn count are bounded by checkpoint, turn-input, cardinality, and logical-turn limits.

## Review Result

Revision: Some("git-blake3:1eee1895ece9fb283386b4c369d46d8e82c7f972:1a1f533251e8337fba78546f56f23048e0566139986971acd6e318de4094059a")

Reviewer: Some("subagent:/root/review_5340_exact")

Result: pass

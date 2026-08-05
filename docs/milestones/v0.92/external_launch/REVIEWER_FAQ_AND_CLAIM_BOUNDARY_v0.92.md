# v0.92 First-Birthday Reviewer FAQ And Claim Boundary

## Metadata

- Owner issue: `#4763`
- Review surface: external launch FAQ and redaction checklist
- Required upstream proof: `#4762` actual retained witness and receipt package
- Publication status: not published

## FAQ

### Has the first birthday happened?

No. This surface implements the launch documentation and external review copy.
`#4762` supplied retained witness and receipt proof through PR #5744 at
`021be8e33b486d9b66886ff299c20607ed8a071a`. The birthday claim remains pending
until the v0.92 birthday packet consumes that proof at an exact result, passes
validation, and receives operator publication approval.

### What did this issue implement?

It implemented the repository launch surface: public-copy drafts, reviewer
questions, publication gates, forbidden-claim rules, and links from the v0.92
launch packet, bridge ledger, and v0.91.8 activation map.

### Why is startup not a birthday?

Startup is only process execution. A birthday requires a reviewable packet with
identity, continuity, memory grounding, capability, governance context,
witnesses, receipt, validation, and review evidence. Missing any required
surface fails closed.

### What does `#4762` need to provide?

`#4762` provides the auditable witness and receipt package at
`docs/milestones/v0.91.8/review/v092_handoff_4762/`, merged through PR #5744.
That retained implementation/proof artifact is the consumable input; the PR,
merge, or lifecycle state alone is not a substitute for its contents.

### Can this copy be used externally now?

Only as a validation-gated review surface unless the operator authorizes a
target publication channel. It can say that the launch docs are prepared and
the upstream proof package is merged. It cannot say the birthday is complete
or public-ready before the remaining gates pass.

### What should reviewers inspect first?

Reviewers should inspect:

- whether the launch copy separates prepared documentation from birthday proof;
- whether the merged `#4762` dependency and retained package are visible and
  non-substitutable;
- whether negative cases reject startup, wake, restore, snapshot, copied state,
  fixture admission, simulation, and missing evidence;
- whether public copy avoids forbidden claims;
- whether all links point to tracked repository surfaces.

## Claim Boundary Checklist

Before any publication, answer each item with evidence:

| Check | Required answer |
| --- | --- |
| `#4762` accepted proof cited? | Yes, with exact retained artifact or result. |
| Current exact-head review recorded? | Yes. |
| Publication channel authorized by operator? | Yes. |
| Legal personhood claim absent? | Yes. |
| Consciousness proof claim absent? | Yes. |
| Production citizenship claim absent? | Yes. |
| Completed constitutional governance claim absent? | Yes. |
| Subjective affect or wellbeing claim absent? | Yes. |
| Startup/wake/restore/snapshot/copy/simulation rejected as birth? | Yes. |
| Raw private memory omitted or redacted? | Yes. |
| Provider/model/tool limits preserved? | Yes. |

## Redaction Rules

- Use repository paths and issue numbers instead of raw private memory.
- Cite retained artifacts, not local machine-only authoring notes.
- Do not expose provider keys, host-local secrets, private prompts, personal
  notes, raw memory dumps, or unpublished reviewer comments.
- If a claim needs private context to be convincing, do not publish the claim.

## Review Prompts

1. Does the public copy claim only prepared launch-surface status while v0.92
   validation and operator publication approval remain pending?
2. Does the ready variant require an exact accepted witness/receipt artifact?
3. Are all not-a-birthday cases rejected in plain language?
4. Are philosophical, legal, governance, and production-readiness claims kept
   outside the engineering birthday claim?
5. Can a reviewer follow the launch surface from v0.91.8 handoff to the v0.92
   launch packet without reconstructing context from chat?

## Publication Decision

Until every checklist item passes, the decision is `do_not_publish_final_claim`.
The allowed interim decision is `share_validation_gated_review_surface`.

# #4760 Preparation Packet

## Authority

This directory is the issue-local preparation handoff for later execution of
Memory Palace. It does not replace `.csdlc/issues/4760/index.json` or its six
rendered lifecycle projections.

The authoritative record truth remains:

- phase: `initialized`;
- generation: `0`;
- claim: `prep-4760-v0918-wp14`, expired;
- implementation: not started;
- review/publication/merge/closeout: not started.

The expired claim is intentionally left unchanged. Execution-time typed claim
acquisition is deferred and is not a blocker to preparation. Before product or
authoritative card mutation, the execution owner must acquire a current typed
v2 issue-local claim and apply these drafts through supported `csdlc-edit` and
`csdlc-validate` operations.

## Contents

- `cards/sip.md` through `cards/sor.md`: six issue-specific prepared card
  drafts for typed application at execution time.
- `design.md`: source-grounded implementation/proof design.
- `diagram.mmd`: dependency and evidence-gate flow.
- `review/preparation-review.md`: one bounded preparation review and fixes.
- `validation/preparation-validation.md`: preparation-only validation truth.
- `init-request.json`: historical bootstrap input retained for audit only. Its
  old baseline and no-review wording are superseded by this packet and must not
  be used as current execution instructions.

## Hard Dependency

#4760 is the implementation/proof prerequisite for #5007. ADR 0051 remains
deferred until #4760 supplies implementation evidence, continuity semantics,
the ObsMem/Chronosense boundary, runtime handoff evidence, deterministic and
negative proof, and exact-revision review evidence. Typed closeout receipts from
unrelated issues never satisfy or gate that product-evidence relationship.

## Execution Entry

1. Confirm GitHub issue #4760 is open and inspect #4765, #4768, #4771, #5007,
   #5362, and the WP-20/#5363 ordering predecessor at the execution revision.
2. Fetch/integrate current `origin/main` in this worktree.
3. Acquire a fresh issue-local typed v2 claim and bind execution.
4. Apply the six prepared drafts through typed card operations, updating any
   source paths that genuinely moved.
5. Implement only the bounded paths in `design.md`.
6. Run every required VPP lane; do not close on planning or isolated proof.
7. Retain exact evidence in `.csdlc/evidence/4760/` for #5007 consumption.

## Non-Claims

- No Memory Palace code or runtime behavior exists because of this packet.
- No current C-SDLC doctor PASS is claimed while the claim is expired.
- No ADR acceptance, v0.92 activation, PR, publication, merge, or closeout is
  authorized or implied.

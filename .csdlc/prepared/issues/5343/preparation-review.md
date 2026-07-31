# #5343 Preparation Review

Reviewer: `external:deepseek:deepseek-chat`

Provider identity: provider-asserted

Review request: `deepseek-review-5343-prep-001`

Scope: the six generated cards, typed issue record, design, diagram, dependency
gate, preparation validator, validation-lane adapter, and the canonical
v0.91.8 planning sources supplied in the bounded review packet.

## Findings And Dispositions

1. **P1: the dependency validator used an undocumented terminal-receipt
   location.** Not accepted. The shared Git path
   `csdlc-v2/closeout/<issue>.json` is the repository's documented typed-v2
   retained receipt authority and is used by current terminal closeout proof.
2. **P2: checking a null claim does not also check a stale ownership lock.**
   Not accepted as a separate gate. A typed `closed_out` retained receipt with
   `claim: null`, merged terminal disposition, exact observed SHA, and current
   terminal projection is the authority. An unrelated stale filesystem lock
   is not execution authority.
3. **P2: the executable dependency gate omitted #5345.** Fixed. The gate now
   requires both #5344 and #5345 to have retained merged `closed_out` receipts,
   released claims, terminal projections, and merge SHAs ancestral to #5343.
4. **P2: preparation validation did not bind the approved design and diagram
   digests.** Fixed. The validator now requires typed approval and checks the
   SPP/VPP design digest against the approved revision and the SPP/VPP diagram
   digests against each other. `csdlc-doctor` remains the authority that
   recomputes the current design digest.
5. **P3: the 1200-second budget term used inconsistent formatting.** Fixed by
   checking the canonical `1200` form.

The ancestry check was also changed to consume `system`'s boolean result
directly instead of relying on an undeclared `$CHILD_STATUS` alias.

## Review Result

Actionable findings were fixed in preparation scope. No selector transaction,
product implementation, publication, PR, merge, Runtime v2 edit, raw GitHub
CLI call, or AWS action was performed.

The preparation remains fail-closed until #5344 is merged and typed
`closed_out`, its retained receipt and exact accepted soak/rollback handoff are
present, its merge SHA is ancestral, and #5345's selector/installer authority
is likewise merged and typed `closed_out`.

## Final Re-review

Reviewer: `external:deepseek:deepseek-chat`

Request: `deepseek-review-5343-final-001`

Result: **PASS, zero actionable blockers.**

The reviewer rechecked the fixed dependency gate, approved design/card digest
bindings, canonical budgets, ancestry behavior, all six cards, protected paths,
COTS/PVF contracts, and preparation-only prohibitions. It found no remaining
actionable issue and confirmed that selector execution remains fail-closed.

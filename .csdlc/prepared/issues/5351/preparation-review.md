# #5351 Preparation Review

Reviewer: `subagent:019f8660-858f-7382-a4ee-003d685c4f74`

## First Pass

Result: `FAIL` with six actionable findings.

1. Removed the stronger GitHub issue-completion claim and bound admission to the
   user-authorized merged terminal receipt, PR identity, typed closeout, claim
   release, and ancestry predicate.
2. Removed AC-1 and AC-8 credit from the preparation lane; terminal and full
   lifecycle criteria are proved only by their owning future lanes.
3. Retained `design_approved: false` pending the post-fix review. It will become
   true only after that review passes and before typed initialization consumes
   the request.
4. Added per-card native current-registry template, repository, version, title,
   slug, structure, and issue identity checks.
5. Added exact PVF resource, time, acceptance mapping, ordering, deferral, and
   issue-local assertion-count enforcement.
6. Removed machine-local absolute paths from retained preparation content.

## Final Pass

The second pass confirmed five findings fixed and retained one blocker: rendered
card structure was not directly validated against the native shape manifest.
The validator now parses every rendered `##` heading, compares the exact ordered
shape for its card kind, and verifies rendered template, issue, repository, and
card identity. Final approval remains pending one last bounded check of this
repair.

The third pass found one false-positive identity hole: rendered metadata used
substring checks. The validator now parses the four rendered identity fields
and compares their complete values. The repair also removes a recursive typed
validation call; the outer `csdlc-validate` request remains the sole PVF
orchestrator.

Final reviewer: `subagent:019f866a-1edf-7d52-bcc1-3d65127a5fa3`

Result: `PASS` with zero blockers. The reviewer confirmed exact rendered
identity comparisons, ordered native card shapes, retained dispositions for all
earlier findings, Ruby syntax, JSON parsing, and the non-recursive validation
boundary, then authorized `design_approved: true`.

Typed initialization subsequently rejected the unsupported planning profile
value `integration` before creating lifecycle state. The request now uses the
supported `large` profile, preserving the reviewed large integrated-gate budget
and scope without changing product authority.

Typed initialization next rejected aggregate automatic validation budgets: the
six lane ceilings exceeded the `large` profile's 7,200 seconds. The complete
and post-merge ceilings are now 2,280 seconds each, making the exact lane sum
7,200 seconds while retaining every required proof lane and acceptance mapping.

The first typed preparation PVF then exposed host Ruby 2.6 compatibility:
`Array#filter_map` was unavailable. Both bounded parsing sites now use the
equivalent `map` plus `compact`; no validation semantics or authority changed.

The next PVF pass exposed an assertion-normalization bug: the validator changed
`closed_out` to `closed out` before searching reviewed artifacts. Required
contract terms are now declared as their exact retained spellings.

## Post-Bind Review

Reviewer: `subagent:019f8670-015b-7f31-adef-0ec65094090b`

Result: `FAIL` with two reported blockers.

1. The reported design/diagram digest mismatch compared SHA-256 values against
   typed C-SDLC v2 BLAKE3 digests. `csdlc-v2/src/cards.rs::digest` defines the
   typed algorithm as BLAKE3, and the bound doctor validates the current
   design/diagram family successfully. No digest rewrite is appropriate.
2. The reviewer correctly found that the previous PASS predated the typed
   planning-profile, aggregate-budget, Ruby 2.6, and exact-term fixes. A fresh
   final review of the complete bound packet is required before commit.

Final bound-packet reviewer: `subagent:019f8672-8b41-7220-b383-80c424c84d37`

Result: `PASS` with zero blockers. The reviewer dismissed the SHA-256/BLAKE3
algorithm mismatch, verified typed digest continuity, and freshly reviewed all
generated cards, bound state, preparation files, budgets, PVF evidence, Ruby
2.6 compatibility, zero-product/COTS boundary, and the complete #5354 terminal
gate.

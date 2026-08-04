# #4758 Bounded Preparation Review

Review count: 1 of 1.

Reviewer: `codex:019fb954-d620-7e23-a297-736a936fdcff`

Review base revision: `ea60a78fe02e9ace1862cab28042313bdf8ee565`

## Scope

- `.csdlc/prepared/issues/4758/design.md`
- `.csdlc/prepared/issues/4758/diagram.mmd`
- `.csdlc/prepared/issues/4758/launch-readiness-preparation.v1.md`
- `.csdlc/prepared/issues/4758/validate_preparation.rb`
- `.csdlc/prepared/issues/4758/validate.json`
- `.csdlc/evidence/4758/preparation-validation/validation-ledger.v1.md`
- `.csdlc/evidence/4758/preparation-validation/diagram.svg`

The review tested issue/source alignment, the integrated artifact and consumer boundary, six-card semantics, dependencies, intended paths, COTS, LoC/time/token budgets, PVF, rollback, no-deferral behavior, and preparation-only non-claims.

## Findings And Fixes

### P2: Canonical projections retain stale preparation fields

The generated card identities still say WP-14 instead of live WP-21. The existing preparation validator also required a live claim, contradicting the operator's explicit execution-time claim deferral. Direct projection repair is unavailable without the deferred live claim.

Fix: `launch-readiness-preparation.v1.md` now defines the exact SIP-through-SOR replacement contract, correct WP-21 identity, real PVF lanes, and a mandatory typed refresh immediately after execution-time claim acquisition. The issue-local validator and PVF manifest now require `phase=bound`, `claim=null`, all six contract sections, budgets, rollback/no-deferral language, and the WP-21 correction. The canonical projections remain truthful historical state and are not presented as execution-ready.

Disposition: fixed for preparation; typed projection application is an execution-start gate.

### P2: Consumption was ordered before final review

The first SPP draft produced the consumption record before exact-revision review, allowing a consumer to observe an unreviewed manifest.

Fix: SPP now orders rollback and pre-consumption PVF first, exact-revision review/fixes second, release-review handoff third, and consumer-integration proof last.

Disposition: fixed.

### P3: Human projection name was ambiguous

`release-review.v1.md` could be mistaken for the independent consuming release review.

Fix: the projection is now `launch-readiness.v1.md`, explicitly non-authoritative; `consumption.v1.json` remains the integration proof.

Disposition: fixed.

### P3: COTS posture lacked an existing-tool inventory

The first draft said only that no new COTS dependency was introduced, without identifying reused tools.

Fix: the contract now inventories Git, Ruby, `jq`, repository-owned typed v2 binaries, and preparation-only Mermaid CLI/local Chrome use. New SDKs, services, connectors, credentials, and packages remain replan triggers.

Disposition: fixed.

## Fix Verification Digests

- design: `f8bd6b203baa183a1ea2e59df8fbfaa739db7d9ed1b3434ec3f3a29061ee1c4b`
- diagram source: `53755b9046ca41bb53c362e4476b603fe60cffce874480e8f8a48c3d92fac012`
- six-card preparation contract: `69e6141e5c47845b46413212f8cdedb289dc7152b82b8f7966b55eb3ca5ff48b`
- preparation validator: `09fa87807f2c7e1b667bf18693059cc648a2a08d2cf2b95645f45a843ed3eb71`
- PVF manifest: `16fbb41d0106c5d07f516dbe800b6fa2fdd13c74614c006b2a5664d2052e5636`
- rendered diagram: `799a7b098fd9fa98aeb06eeba454727ac08a23c1e5b8309a35083c9faff4a7f9`

## Result

Result: PASS for preparation-only handoff after fixes.

This result does not approve implementation, launch content, publication, merge, or closeout. It does not prove launch readiness. Execution-time typed claim acquisition and canonical six-card refresh remain mandatory.

Residual execution blockers at review time: #5363, #5362, and #5352 are open; later execution must refresh their live state and block rather than defer missing proof.

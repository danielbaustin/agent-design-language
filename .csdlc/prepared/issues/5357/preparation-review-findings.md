# #5357 Preparation Review Findings

Reviewer: `subagent:5357-preparation-review`

Initial result: BLOCKED, 3 P1 and 1 P2.

## Dispositions

1. P1 PVF inventory mismatch: fixed by aligning all six VPP lanes, budgets, acceptance mapping, and runner admission.
2. P1 incomplete dispatch identity: fixed by adding exact base/head, timestamps, provider outcome, degradation, append-only attempt fields, attempts digest, and supersession identity.
3. P1 prose-only findings contract: fixed with `review-output.schema.json`, including findings-first severity and required observed-evidence, inference, and open-author-decision fields.
4. P2 sequencing ambiguity: fixed by documenting init, review, typed approval, pre-bind card-integrity PVF, bind, and post-bind preparation validation as distinct non-dispatch stages.

## Exact Rereview Findings

1. P1 six-lane execution mismatch: fixed by declaring all six lanes in the typed validation manifest and replacing the future-lane placeholder with artifact-verifying fail-closed contracts.
2. P1 incomplete identity/retry enforcement: fixed with distinct corpus, prompt, provider, process, funder, and retry-controller identities; a receipt schema; contiguous append-only attempt validation; and a canonical attempts digest.
3. P1 findings ordering and evidence enforcement: fixed with structured repository-relative file/line evidence plus executable P0-P3 and findings-before-summary checks.
4. P1 premature PASS: fixed by retaining pending review truth until a fresh exact rereview succeeds; typed approval and bind remain subsequent gates.

Open actionable findings: 0

## Second Exact Rereview Findings

1. P1 complete/post-merge overclaim: fixed by enforcing completed provider outcome, no deferred completion, exact digest bindings, target-to-merge ancestry, green required checks, typed synthesis recorded, and explicit WP-20 non-authority.
2. P1 incomplete corpus proof: fixed by matching every record to the exact target Git tree, recomputing canonical record and handoff digests, checking WP-18 ancestry, and excluding all issue evidence from the corpus.
3. P1 weak retry immutability: fixed by conditional non-null schema rules, complete attempt schema, and digest-bound supersession whose prior attempts must remain an exact prefix.
4. P1 weak evidence proof: fixed by requiring every citation in the frozen corpus, resolving exact target-revision line bytes, and checking an excerpt SHA-256.
5. P1 prose-only release boundaries: fixed by executable redaction scanning, non-deferred completion, immutable supersession/rollback, green-check evidence, typed-synthesis evidence, and WP-20 non-authority.

## Third Exact Rereview Findings

1. P1 incomplete six-lane parity: fixed by comparing proof role, timeout, token budget, argv, parallel group, required-gate status, offline policy, and empty credentials for every lane.
2. P1 weak independence/schema boundary: fixed by rejecting reviewer identity equal to any project-control identity and requiring provider/model/independence plus completed-dispatch/output fields.
3. P1 incomplete exact receipt binding: fixed by checking base ancestry, handoff digest, prompt target bytes, and exact output path/size/digest.

## Fourth Exact Rereview Findings

1. P1 VPP-only policy fields: disposition corrected. Every field representable by the active VPP schema is compared; required release gate, denied network, and empty credentials are declared and enforced as typed-manifest policy extensions because the VPP schema has no such fields.
2. P1 nullable active dispatch fields: fixed by requiring provider, model, independence statement, and at least one successful terminal attempt for completed review in both schema and runtime validation.

## Final Focused Rereview Finding

1. P1 schema could not identify the final successful attempt: fixed by adding a required completed-state `terminal_attempt` object with `outcome=success`; runtime requires it to equal the last append-only attempt identity.

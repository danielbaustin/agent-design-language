# ADL v0.91.4 — Whole-Milestone Third-Party Review (WP-17)

- Reviewer: Independent technical review (Claude)
- Date: 2026-06-01
- Lane: WP-17 external review, per `ADL_v0.91.4_THIRD_PARTY_REVIEW_HANDOFF.md`

## Method and honesty boundary

This is a real review of tracked repo state. I read source and docs directly and
report what is actually there. What I did **not** do, and therefore do not
assert: I did not compile the Rust, run the test suite, run the PVF lanes, or
make live provider calls in this environment. Runtime-dependent conclusions are
marked "read, not executed." A numeric per-category score is deliberately **not**
given, because scoring categories I did not exercise (full test execution, broad
security, performance) would not be evidence-backed. The earlier rounds of these
reviews did exactly that and it was wrong.

### Coverage map (so the gaps are visible)

- **Deep-read (line-level):** `signing.rs` (+ its tests), `provider_adapter.rs`,
  `model_identity.rs`, `.github/workflows/ci.yaml`,
  `tools/test_pvf_ci_release_policy.sh`, signed-trace fixtures,
  `MILESTONE_CHECKLIST`, `QUALITY_GATE`, `NEXT_MILESTONE_HANDOFF`, the WP-16
  internal-review findings register + synthesis, and the C-SDLC paper docs.
- **Targeted-read (consistency/claims):** README/RELEASE_NOTES/handoff release
  truth; v0.91.4↔v0.91.5 boundary; WildClaw safety-results doc.
- **Sampled (not line-audited):** the remaining ~70 milestone docs, the feature
  docs, the JSON fixtures, CodeFriend/WildClaw sidecar docs, browser runbook.
- **Not opened:** the bulk of the 1.36 MB Rust tree outside the files above
  (e.g. `long_lived_agent.rs`, `obsmem_*`, `csm_observatory.rs`, runtime_v2/*,
  the multi-agent planner source). I make no claims about those.

## Verified positives (checked in code, not taken on faith)

1. **Signed trace is real cryptography (read, not executed).** `signing.rs`
   signs a canonical `{header, document}` envelope with ed25519-dalek, strips
   only the top-level typed `signature` field (no recursive over-strip), sorts
   keys deterministically, and verifies the body — not just a thin header. Good
   typed failure classes, embedded/explicit key sourcing, ed25519-only policy,
   and a substantial in-file test suite (round-trip, malformed sig/key, policy
   enforcement). My initial worry that the signature covered only a near-empty
   header was wrong.
   - Honest note (not a finding): the signed header binds `adl_version` and
     `workflow_id` only — no timestamp/expiry — so signatures are non-expiring
     by design. Normal for content-integrity signing; the doc comments are
     accurate about scope.
2. **F010 (Gemini credential-in-URL) — verified fixed.** Key is sent via
   `x-goog-api-key` header; `gemini_generate_url` builds the URL from model name
   only; error paths don't interpolate the URL into persisted diagnostics.
3. **F004 (PVF tests not in CI) — verified fixed.** `ci.yaml` runs the PVF
   release-policy contract and exercises the validation lane (path-policy gated).
4. **F008 (`/Users/` absolute paths in paper docs) — verified fixed** at all
   cited locations; now placeholder tokens.
5. **F002 (release docs overclaim) — verified fixed and honest.** QUALITY_GATE
   outcome is `blocked`; MILESTONE_CHECKLIST leaves ship-gates unchecked rather
   than pre-ticked; handoff disclaims release-readiness. Cross-document release
   truth is mutually consistent — a genuine positive.
6. **WP-16 internal review quality is high** — specific, file-and-line evidence,
   correct severity instincts, explicit non-claims. The "five routes merged"
   claim is substantially true.

## Findings (this review)

### R1 (P2) — PVF policy test retains the F001 bug class on the release lane
`tools/test_pvf_ci_release_policy.sh` (~line 38). The F001 fix wrapped the
docs- and runtime-lane runner calls in `set +e`/capture, but the third
(release-mode) call runs bare under `set -e`. If that runner ever exits
non-zero, the script dies before its Python assertions and `grep` checks — the
exact failure mode F001 described, on the one path the fix didn't cover. Read,
not executed: it may exit 0 today, so this is fragility, not a proven failure.
Fix before WP-18 closes, because PVF is a headline proof surface and a test that
can die before asserting yields false green. Route: WP-18.

### R2 (P3) — WildClawBench host path persists (F007/F009 partial close)
`WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:~54` still names
`$HOME/temp/wildclawbench-3380` as the stable benchmark copy. Remediation did
real work (replayability-boundary framing, `/private/tmp` now historical,
ADL-superiority disclaimed) and `$HOME` is not a username leak, so this is no
longer P2. But "an external reviewer cannot reconstruct this checkout" is still
literally true. De-risked, not fully closed. Acceptable to keep as explicit
sidecar caveat if you decide that deliberately.

### R3 (P3, informational) — F004 CI wiring is path-policy gated
The PVF contract step is conditioned on `ci_contracts_required == 'true'`. A PR
touching only the PVF runner under a docs-classified path could skip it. Add a
one-line path-policy assertion so the PVF surface always forces the contract.

### R4 (P2, traceability) — F003 closure is not verifiable from tracked state
The WP-16 synthesis routed F003 (remote Ollama misclassified as `hosted_http`)
as "fix before release **or** route to v0.91.5," and the handoff's disposition
table does not list F003 among the five merged remediation routes — only F001,
F002, F003-adjacent provider identity (#3544), F007-area, and F009 appear, with
#3544 framed as "provider identity and Gemini credential diagnostics." From the
code I can confirm `model_identity.rs` *knows* the local-vs-hosted distinction
(its tests use both `hosted_http` and `local_http`), but `provider_adapter.rs`
uses a different surface vocabulary (`hosted_api`/`ollama_http`) and I could not
trace, from tracked artifacts alone, whether remote-Ollama specifically now
classifies correctly. This is not a claim that the code is wrong — it's that the
finding's disposition is ambiguous in tracked state. Before WP-18 closes, record
F003's actual disposition (fixed-by-PR with a pointer, or explicitly routed to
v0.91.5) so an external reviewer doesn't have to guess.

### Acknowledged-and-correctly-deferred (not new findings)
- **F011** (next-milestone handoff scaffold tension about v0.91.5 selection) is
  real but explicitly routed to WP-19/WP-20 at P3; the doc's own text both
  assumes v0.91.5 and instructs WP-19 to confirm/revise. Acceptable as deferred.
- **F012 / F013** (browser-runbook machine paths; CodeFriend infra fingerprint)
  — sampled, consistent with their P3 routing; not re-verified line-by-line.

## Verdict

On every surface I actually opened, the engineering is real and the WP-16
remediation claims hold up, with the exceptions logged above. The release-tail
honesty (blocked gate, unchecked ship boxes, consistent cross-doc truth) is a
real strength and the opposite of overclaiming.

Recommendation: **proceed to WP-18 remediation**, with R1 and R4 as
fix-before-close items (R1 because it can produce false-green on a headline
proof; R4 because closure must be auditable, not inferred), and R2/R3 as
cleanup. No release-blocking P0/P1 found in the reviewed surfaces — but note the
explicit coverage gaps above: this is not a clean bill of health for the ~70%
of the Rust tree I did not open, and a full pass should either be run with the
test suite executing or scoped across more of the runtime source.

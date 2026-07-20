# v0.91.7 External Review Verification (WP-19 chat-lane)

- Reviewer: Independent technical review (Claude, chat lane — distinct from the
  automated Fable 5 API lane recorded in `external_review_4646/`)
- Date: 2026-07-19
- Inputs: `ADL_v0.91.7_THIRD_PARTY_REVIEW_HANDOFF.md`,
  `external_review_4646/FINDINGS_REGISTER.md` (22 findings), live source.

## Method and honesty boundary

This pass verifies a sample of the automated WP-19 findings against live
working-tree source. Not done here: no build, no test execution, no digest
computation against the dispatch-receipt SHA (no git access in this
environment), no AWS calls. I reviewed the current working tree, which may
differ from the frozen target revision `bd9b7a3c...`; none of the sampled
findings appear remediated, consistent with WP-20 being open.

## Sampled finding verification (4 of 22, including 1 of 2 P1s)

- WP19-02 (P1, Bedrock account identity) — VERIFIED REAL.
  `adl/src/provider/http_family.rs`: `AwsBedrockProvider::from_target` enforces
  only the profile NAME (`agent-logic-admin`), which is a local alias any
  account's credentials can occupy. `complete_async` calls STS
  `GetCallerIdentity` but only records `account_id_sha256`; no comparison to an
  operator-approved expected account occurs. The invocation record writes
  `account_profile_validation_status: "sts_verified"` unconditionally — an
  overstatement (identity retrieved, not verified). Register remediation is
  correct. Additional observation: the hard-coded default profile is an ADMIN
  profile for a model-invocation path; a least-privilege bedrock-invoke-only
  role would be better even after identity pinning.
- WP19-08 (P2, IPv6 loopback) — VERIFIED REAL.
  `adl/src/provider/http_family/config.rs`: `endpoint_host` returns
  `url.host_str()`, which includes brackets for IPv6 (`[::1]`), so the match arm
  `Some("::1")` in `is_loopback_endpoint` is dead code and `http://[::1]`
  endpoints are classified non-loopback. Fail-closed direction (credentials
  refused to legitimate loopback, not leaked); P2 appropriate.
- WP19-10 (P2, invocation lock) — MECHANISM CONFIRMED in the same file.
  `InvocationArtifactLock` cleans up via `Drop`; a crash between
  `fs::create_dir` and drop leaves the lock dir permanently. Subsequent
  acquisition times out (200×10ms) and the resulting error propagates AFTER the
  billable provider call already succeeded, so retry policy can duplicate spend.
  Register remediation (lease metadata, stale-lock recovery, partial-success
  classification) is apt.
- WP19-13 (P2, issue-wave contradiction) — VERIFIED LIVE.
  `WP_ISSUE_WAVE_v0.91.7.yaml`: `closeout_tail_truth.closed_wps` includes
  WP-21A while the WP-21A work-package entry says `status: open`.

All four sampled findings are genuine and unremediated. I did not find any
sampled finding to be fabricated or already fixed — the automated lane's output
quality, on this sample, is real.

## Version discipline

`adl/Cargo.toml` = 0.91.7. Intact.

## Verdict on closability

v0.91.7 is NOT closeable, by its own artifacts and confirmed by this sample:

1. WP-20 (#4647) — the owner of remediation for all 22 findings — is
   intentionally open; no finding has a recorded disposition yet.
2. Both P1s are unremediated (WP19-02 verified live above; WP19-01 not sampled
   but no remediation evidence exists).
3. The issue wave itself records `release_ready: false` and open WP-20/WP-23.
4. Retained #4906 `blocked_with_evidence` rows remain an explicit
   release-readiness boundary per the handoff's own non-claims.

Recommended path: run WP-20 on the 22-finding register (P1s first: LLVM profile
isolation + Bedrock identity pinning), re-verify the two P1 fixes at file level,
resolve the WP-21A YAML contradiction, then proceed to WP-23. No numeric score
is assigned; execution-dependent categories were not executed.

## Structural note on review identity

The `external_review_4646` packet labels one lane "third-party Fable 5
identity." Corpus integrity (dispatch receipt, digest, frozen revision) is
well designed and non-self-referential — genuinely good mechanism. But
reviewer INDEPENDENCE is not established by model identity: the lane was
dispatched, prompted, budgeted, and filed by the system under review. A
reviewer the pipeline invokes is a tool of the pipeline, whatever model runs
it. The register itself is honest that shadow lanes are "not third-party
provider identity"; the same limitation applies, more softly, to the Fable
lane. Recommend renaming the lane classification from "third-party" to
"automated external-model lane" and reserving "third-party" for review the
operator does not dispatch or control.

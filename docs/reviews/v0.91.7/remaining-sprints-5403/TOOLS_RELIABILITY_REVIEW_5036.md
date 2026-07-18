# Tools Workflow Reliability Tail Review

Issue: #5036
Review issue: #5403
Status: remediation implemented; canonical register reconciliation pending
Remediation: #5407; shared records issue #5406

## Findings

### P1: Build-action logging closed after implementing only one producer

At pre-remediation revision `7e0ed914`,
`docs/tooling/BUILD_ACTION_LOGS.md:3` and line 42 identify
`validation_manager.py --run` as the integrated producer and leave CI
integration to future consumers. Repository references locate packet production
only at `adl/tools/validation_manager.py:1404`.

#5032 also required `pr finish`, owner lanes, remote builders, CI ingestion,
watcher/shepherd reporting, and fail-closed closeout behavior. Those acceptance
surfaces are not implemented.

Impact: build and validation actions outside validation-manager execution can
still disappear without durable action evidence despite the child being closed.

Disposition: fixed by #5407. `docs/tooling/BUILD_ACTION_LOGS.md` now limits the
implemented contract to `validation_manager.py --run` and explicitly excludes
every other original #5032 producer and consumer from current claims. Expansion
requires a separate reviewed issue.

### P1: The retained CLI taxonomy directs operators to sunset v1 commands

At pre-remediation revision `7e0ed914`,
`docs/tooling/ADL_PLATFORM_CLI_BINARY_TAXONOMY.md:28` and line 36 recommend
`adl/tools/pr.sh` and the removed compatibility binary. Current Gate 10D2
authority at `AGENTS.md:5` and line 43 says those wrappers are removed and the
typed v2 binaries are the sole operational authority.

Impact: current operator-facing documentation directs users to unsupported,
deleted lifecycle commands.

Disposition: fixed by #5407. The taxonomy now names the typed binaries under
`csdlc-v2/` as the sole Gate 10D2 lifecycle authority and rejects the removed
v1 wrappers and compatibility route.

### P2: The umbrella lacks an internally current retained closeout synthesis

`docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md:98` records
that the execution packet still contains pending children, the named closeout
artifact is absent, and integrated #4938 proof is not retained.

Impact: #5036 closure is supported by live issue and PR state rather than a
complete durable lifecycle and closeout packet.

Disposition: fixed by #5406 and #5407. #5406 provides terminal typed-v2 record
authority. `docs/reviews/v0.91.7/tools-5407/TOOLS_RELIABILITY_CLOSEOUT_5036.md`
retains the complete eleven-child issue and merged-PR matrix.

### P2: The claimed material CI speedup lacks comparative hosted-run evidence

`docs/milestones/v0.91.7/review/build_throughput/CI_CONTRACT_SPLIT_5037.md:57`
says wall-clock improvement must be confirmed from GitHub-hosted runs, but line
61 retains only near-zero local policy-script timings.

Impact: green checks establish correctness, not the claimed material reduction
in CI duration.

Disposition: fixed by #5407 through claim narrowing. #5037 proves the focused
CI contract split and green integration only; no material hosted wall-clock
speedup is claimed without comparable before/after hosted runs.

## Child Coverage

Reviewed #5034, #5032, #5037, #5031, #5028, #5012, #5002, #4999, #4995,
#4987, and #4938. All are live-closed through merged PRs. The bounded snapshot
at `docs/reviews/v0.91.7/tools-5407/github-closeout-snapshot-5036.json` retains
issue closure, merge topology, and the observed check-rollup conclusions. It
does not infer which checks were required. #5037 was omitted from the
operator-selected execution list but later added to the declared umbrella wave
and is included in this review.

Previously discovered and fixed defects, including #5037's two pre-PR P1
findings, are not counted among this review's four findings. All four current
findings are review-discovered.

## Validation And Limits

No tests or mutating commands were run during the read-only specialist pass.
Historical local SRP/SOR cards are absent after v1 sunset; PR descriptions were
used as lifecycle summaries but not treated as durable card truth.

## Review Result

The four findings were remediated by #5406 and #5407. The current logging and
CLI documents now match implemented authority, the complete sprint wave has
retained closeout evidence, and the unsupported hosted-performance claim has
been withdrawn. The canonical sprint register still carries the earlier
`changes required` state. #5423 owns reconciliation of #5403's unreleased typed
claim and that separately protected register path, so this review remains
explicitly non-terminal until the register is updated.

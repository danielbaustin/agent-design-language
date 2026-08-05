# WP-02B Post-Migration Build Acceleration Experiment Design

## Decision Boundary

WP-02B owns one controlled comparison between the current `ubuntu-latest`
control and one restricted GitHub-hosted 16-core Ubuntu runner. It starts only
after WP-02 migration verification, WP-02A CI reliability, organization-owner
budget approval, alerts, and selected-repository runner access are proven.

The tracked issue-local validator at
`.csdlc/prepared/issues/5853/validate-experiment.rb` is the evidence-shape
authority. The operator-local source plan is historical planning input, not a
portable runtime dependency.

## Experiment Contract

1. Freeze one exact commit, workflow revision, toolchain, lockfiles, commands,
   permissions, cache design, proof inputs, workloads, and required-check
   topology.
2. Capture five cold and ten warm trials for the shared benchmark and each
   selected Tier 2 Rust lane on both platforms.
3. Retain queue, setup, cache, compile/link, execution, artifact, total-time,
   critical-path, reliability, retry/cancellation, and cost data without
   dropping unexplained outliers.
4. Prove result, artifact, validation, and required-check parity before exactly
   one canary lane is routed to the candidate runner.
5. Apply the predeclared thresholds and record `adopt`, `reject`, or `defer` for
   every measured lane.
6. Preserve `ubuntu-latest` fallback. Observe ten representative runs for an
   adopted route or remove rejected/deferred experimental configuration.

## Security And Negative Boundary

- The runner group is selected-repository only with maximum concurrency one.
- Paid execution requires an owner-approved maximum cost and alerts.
- Untrusted fork code receives no privileged runner or secret access.
- Required-check names, branch protection, validation breadth, and proof
  semantics do not change.
- Missing gates, incomparable inputs, absent cache-hit evidence, incomplete
  samples, parity failure, cost breach, or failed cleanup invalidates adoption.
- AWS, self-hosting, 32-core runners, coverage topology, custom images, and
  ARM64 are separate decisions.

## Rollback

Rollback changes only the centralized runner selection back to
`ubuntu-latest`. It must not require product code, test expectation, lifecycle
record, required-check, or branch-protection changes.

## Completion Evidence

Completion requires `eligibility.json`, `frozen-manifest.json`, complete
`trials.jsonl`, `parity.json`, `decision.json`, and `final-state.json`, all
accepted by the tracked validator, plus workflow contract checks, diff hygiene,
and a clean exact-head review. Provisioning a runner or producing planning
prose is not completion.

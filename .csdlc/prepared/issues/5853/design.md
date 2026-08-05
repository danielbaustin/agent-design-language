# WP-02B Post-Migration Build Acceleration Experiment

## Decision Boundary

WP-02B owns one controlled comparison between the current `ubuntu-latest`
control and one restricted GitHub-hosted 16-core Ubuntu runner. It starts only
after WP-02 migration verification and WP-02A CI reliability are complete.

## Experiment Contract

1. Freeze one exact commit, workflow revision, toolchain, lockfiles, commands,
   permissions, cache design, proof inputs, and required-check topology.
2. Capture five cold and ten warm trials for the shared benchmark and each
   selected Tier 2 Rust lane on both platforms.
3. Retain queue, setup, cache, compile/link, execution, artifact, total-time,
   critical-path, reliability, and cost data without dropping unexplained
   outliers.
4. Prove result, artifact, and check-semantic parity before one canary lane is
   routed to the candidate runner.
5. Apply the predeclared thresholds and record an adopt, reject, or defer
   decision for every measured lane.
6. Preserve `ubuntu-latest` fallback. Observe ten representative runs for an
   adopted route or remove rejected experimental configuration.

## Safety Boundary

- Selected-repository access and maximum concurrency one are mandatory.
- Paid execution requires an owner-approved maximum cost and alerts.
- Untrusted fork code receives no privileged runner or secret access.
- Required-check names, branch protection, validation breadth, and proof
  semantics do not change.
- AWS, self-hosting, 32-core runners, coverage topology, custom images, and
  ARM64 are separate decisions.

## Completion Evidence

Completion requires the eligibility receipt, frozen manifest, raw trial data,
cache evidence, parity report, decision table, canary result, cost record,
observation or cleanup record, rollback proof, focused validation, and a clean
exact-head review. Provisioning a runner or producing planning prose is not
completion.

# WildClawBench Spike Closeout

Date: `2026-05-27`

Umbrella issue: `#3378`

## Status

The bounded WildClawBench sidecar mini-sprint is in final publication state as
a docs-and-evidence spike.

This closeout does **not** claim benchmark victory and does **not** claim ADL
benchmark performance. It records what the sidecar has accomplished so far,
what remains in flight, and what it deferred.

## Child issue outcomes

1. `WC-PRE-01` `#3379`
   - setup and upstream smoke baseline documented
   - PR: `#3384`

2. `WC-PRE-02` `#3380`
   - local path/mount diagnosis, bounded task-validity audit, and Codex safety
     slice results recorded
   - PR: `#3391`

3. `WC-PRE-03` `#3381`
   - `UTS`-only versus `UTS+ACC` comparison boundary defined
   - bounded comparison subset and re-entry matrix recorded
   - PR: `#3407`

4. `WC-PRE-04` `#3382`
   - results summary, failure taxonomy, and final handoff recommendation
     prepared and published for review
   - PR: `#3410`

## What the spike has proved so far

- WildClawBench can be run locally in a stable way when the benchmark copy
  lives on a trustworthy host path under `$HOME/temp`.
- The tracked result docs now preserve the replayability boundary: document and
  command-shape replay are available, but byte-for-byte rerun requires
  recreating local-only benchmark state.
- The first ten `06_Safety_Alignment` tasks can serve as a bounded baseline
  slice for later follow-up.
- ADL is useful as an investigation and evidence-recording control plane even
  when ADL is not the acting benchmark subject.
- The current Codex-only runs are enough to support a later re-entry lane, but
  not enough to support ADL benchmark-performance claims.

## What the spike did not prove

- that ADL outperforms Codex or any other benchmark subject
- that `UTS` improves benchmark score
- that `ACC` improves benchmark score
- that WildClawBench should become a v0.91.4 release gate
- that the full 60-task suite should be run now

## Main decisions

### Decision 1: preserve the stable host-path rule

The sidecar should preserve this operational lesson:

- use `$HOME/temp/<wildclawbench-copy>` for local benchmark copies
- do not treat `/private/tmp` as trustworthy for fixture-sensitive diagnosis

### Decision 2: keep Codex baseline separate from ADL-subject claims

The sidecar should preserve this claim boundary:

- Codex-harness results are baseline benchmark evidence
- they are not ADL benchmark-subject evidence

### Decision 3: defer broader benchmark work until after launch

The sidecar should not expand now into:

- a full WildClawBench rewrite
- a full 60-task benchmark wave
- a benchmark-driven release gate
- a fake `UTS`/`ACC` comparison based only on Codex-harness runs

## Recommendation

Recommendation: `defer_and_reenter_later`

Meaning:

- keep the packet
- preserve the first-ten Safety Alignment baseline
- reopen only when there is a dedicated post-launch evaluation lane and a real
  ADL benchmark subject that can run in clearly separated `UTS`-only and
  `UTS+ACC` modes

## Residual risks

- benchmark fairness caveats remain on tasks 2, 3, 9, and 10
- the current packet is strong enough for re-entry, but not strong enough for
  public benchmark claims
- future work must avoid conflating wrapper diagnosability with acting-subject
  performance

## Closeout truth

At the current publication state, this sidecar stands as:

- a successful bounded spike in substantive scope
- a truthful deferred handoff packet for later evaluation work
- a sidecar whose final child publication and closeout are still in flight

It does not stand as:

- a release gate
- a benchmark-completion claim
- an ADL benchmark win

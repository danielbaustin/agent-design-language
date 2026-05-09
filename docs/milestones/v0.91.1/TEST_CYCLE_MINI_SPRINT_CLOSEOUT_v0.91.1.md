# v0.91.1 Test-Cycle Mini-Sprint Closeout

## Purpose

This note closes out the `#2897` test-cycle recovery mini-sprint with one
tracked, reviewable summary of what changed, what was measured, and what
follow-on tooling work remains.

It is intentionally narrower than the milestone-wide quality gate. This is a
sprint-scoped retrospective and evidence note, not a claim that `v0.91.1`
itself is quality-complete.

## Scope

Ordered child issues:

1. `#2867` via `#2902`
2. `#2864` via `#2903`
3. `#2865` via `#2904`
4. `#2866` via `#2905`
5. `#2868` via `#2906`

## Realized Sprint Shape

This mini-sprint should be described as a mixed planning/tooling/disposition
wave, not as five fresh runtime-reduction implementations.

What actually landed:

- `#2867`: planning and sequencing truth
- `#2864`: narrower coverage escalation policy
- `#2865`: smaller PR authoritative reporting tail
- `#2866`: broader focused-lane and coverage-impact mappings
- `#2868`: evidence-based residual closeout because the named heavyweight proof
  family collapse had already landed through merged `GW-01` through `GW-05`

That means the sprint improved real cycle-time levers, but it did not itself
perform a second round of heavyweight proof-family runtime rewrites.

## Baseline Evidence

Baseline source recorded during `#2867`:

- authoritative `main` run: `25567349404`
- date: `2026-05-08`
- tests: `1830`
- authoritative coverage runtime: `525.078s`
- long-tail buckets:
  - `29` tests over `30s`
  - `16` tests over `45s`
  - `2` tests over `60s`

## Post-Sprint Evidence

First post-sprint `main` authoritative run after the mini-sprint merge train:

- GitHub Actions run: `25610627099`
- date: `2026-05-09`
- merge surface: `#2906` / `64411610294827ae45a962420bac7fa8f29daec7`
- `adl-coverage` job step `Coverage run and summary (json)`:
  - started: `2026-05-09T20:08:35Z`
  - completed: `2026-05-09T20:21:31Z`
  - observed wall time: about `776s`

Observed interpretation:

- the first post-sprint authoritative `main` run did not show an end-to-end
  wall-clock improvement against the `525.078s` baseline
- this mini-sprint still landed useful routing and overhead reductions, but the
  first post-sprint full-authoritative `main` measurement remained dominated by
  broader repo state rather than showing a clean isolated win

## PR-Fast Evidence

Representative child PR CI evidence remains sprint-useful but not a clean
ordinary PR-fast Rust-lane comparison.

Example:

- PR run `25610152951` for `#2866`
- `adl-ci` completed in about `13s`
- the run truthfully skipped Rust validation because the changed surfaces were
  tooling-only under the tightened path policy
- `adl-coverage` completed in about `716s`, but as a tooling-policy PR it
  deferred full workspace gate/artifact work rather than exercising the
  ordinary PR-fast Rust test lane

Closeout judgment:

- this sprint does leave concrete PR-path evidence that policy-only/tooling
  PRs now avoid some unnecessary PR-time work
- it does not leave a clean “ordinary PR-fast Rust lane before/after” timing
  pair because no merged child issue in this sprint actually exercised that
  path

## What Improved

- authoritative escalation became more selective for workflow-policy-only edits
- PR authoritative reporting tail dropped for tooling-only policy PRs
- focused routing became more truthful for nested runtime and CLI source
  families
- the heavyweight runtime residual issue was closed honestly by evidence rather
  than duplicate implementation claims

## What Did Not Fully Close Here

- a clean end-to-end authoritative wall-time win was not proven by the first
  post-sprint `main` run
- a representative ordinary PR-fast Rust-lane timing comparison was not
  produced by the merged child train

## Sprint Tooling Follow-On

The sprint also exposed orchestration friction that should stay attached to
this retrospective rather than being treated as unrelated drift:

- `#2901`: sprint-conductor umbrella creation should bootstrap the local issue
  bundle in the same flow or stop blocked
- `#2907`: sprint lifecycle gaps surfaced by this mini-sprint, including
  published-child progress state, open-PR-wave handling, canonical bundle
  reuse, and `pr finish` local-card staging noise

## Closeout Judgment

This mini-sprint was still useful and worth merging.

Truthful final read:

- it improved three real cycle-time levers
- it documented the residual heavyweight-runtime surface honestly
- it exposed sprint-tooling gaps clearly enough to fix next
- it did not yet prove a net full-authoritative wall-time reduction on `main`

That makes the correct disposition:

- `successful mixed tooling/planning/disposition sprint`
- with `measurement closure partially complete`
- and `follow-on sprint-tooling repairs required`

# WP-10A Distributed Workcell Umbrella

## Purpose

#5497 coordinates the five existing WP-10A children. It owns no product code and
does not implement a second scheduler, task transport, dashboard, convergence
engine, or proof harness.

## Execution Order

1. #5499 freezes the deterministic assignment/refusal contract.
2. #5498 implements explicit Codex task transport against that contract.
3. #5500 and #5502 execute in parallel after the observation/output contracts
   are frozen.
4. #5501 runs the live distributed workcell proof.
5. #5497 records convergence after every child is merged or truthfully
   dispositioned.

Live merge state and ancestry release downstream work. Typed closeout receipts
are retained audit evidence and are never execution blockers.

## Authority

- Typed C-SDLC v2 claims remain write-ownership authority.
- Child issues own all product paths and validation.
- #5497 owns only its issue-local lifecycle records.
- Review, merge, and closeout remain explicit human-authorized operations.
- The umbrella cannot create tasks, mutate GitHub, merge code, or close children.

## Readiness

WP-09 is merged into current `origin/main`. Each child may be prepared now.
Implementation follows the order above and stops on overlapping active claims,
stale interfaces, or ambiguous authority. No receipt is a readiness gate.

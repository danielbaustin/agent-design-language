# Demo Operator Playbook

Use this file after the main skill triggers and the demo target is already concrete.

## Priorities

Prefer this order:
1. understand the intended proof surface
2. inspect prerequisites and gating conditions
3. run the smallest truthful demo surface
4. classify the result conservatively
5. stop with one bounded follow-up recommendation

## High-Value Questions

Ask:
- What is this demo supposed to prove?
- Is there a dry-run or fixture-backed path that is enough?
- Are provider credentials or remote hosts required?
- If the demo is skipped, is that intentional and documented?
- If the demo runs, what artifact proves success?

## Selection Guidance

Prefer:
- dry-run or fixture-backed commands
- one named demo entrypoint over ad hoc shell choreography
- one clear classification over a vague narrative

Avoid:
- rewriting the demo to make it pass
- conflating review packages with ordinary demos
- overclaiming proof from console output alone

## Classification Guidance

Use:
- `proving` when the demo ran and produced the intended proof artifact
- `non_proving` when the demo ran but the proof surface is incomplete or weak
- `skipped` when an explicit gate or allowed missing prerequisite prevented execution
- `failed` when the demo should have run but broke

When in doubt, bias toward `non_proving` or `failed` rather than overclaiming success.

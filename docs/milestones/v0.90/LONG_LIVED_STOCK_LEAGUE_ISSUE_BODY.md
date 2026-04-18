## Summary

Prototype a bounded long-lived paper-market agent league demo that uses several
local or remote Ollama models as distinct persistent agents. The demo should
show continuous identity, append-only memory, paper portfolio commitments,
market outcome tracking, and accountable self-review without real trading,
personalized investment advice, broker integration, or expensive data feeds.

## Goal

Set up the first runnable proof slice for the "Market Spirits" concept:
multiple long-lived agents with stable identities make paper-only stock picks
against a deterministic fixture market, then carry their decisions, scars,
score, and self-review forward across simulated market days.

## Required Outcome

This issue should produce a truthful, reviewable demo/proof result for a
fixture-first stock league. It should prove the long-lived identity mechanics
and model-roster integration points while keeping any live Ollama, remote
Ollama, or delayed public-market-data path optional and safely bounded.

## Deliverables

- runnable fixture-mode demo wrapper
- proving test for the demo packet
- model roster discovery for local Ollama and optional remote Ollama
- fixture market data and agent identity surfaces
- paper-only portfolio ledger and scoreboard artifacts
- demo documentation with explicit non-advice and no-real-trading guardrails
- truthful issue/output records aligned with the delivered result

## Acceptance Criteria

- the demo runs without network, broker APIs, or secrets in canonical fixture mode
- the demo records at least four persistent agent identities and their paper-only decisions
- the demo can optionally inspect local and remote Ollama model rosters without requiring either for the canonical proof
- the paper broker refuses real-trading language or unsupported actions
- the output packet includes identity journals, paper ledger, scoreboard, data-source report, and hindsight/identity audit
- validation proves the demo packet shape and guardrails

## Repo Inputs

- `docs/milestones/v0.90/features/LONG_LIVED_STOCK_PICKING_AGENTS_DEMO_PLAN.md`
- `demos/v0.89/gemma4_issue_clerk_demo.md`
- `demos/v0.87.1/codex_ollama_operational_skills_demo.md`
- `adl/tools/demo_v089_gemma4_issue_clerk.sh`
- `adl/tools/demo_codex_ollama_operational_skills.sh`
- `docs/architecture/PROVIDER_CAPABILITY_AND_TRANSPORT_ARCHITECTURE.md`

## Dependencies

- local planning doc: `docs/milestones/v0.90/features/LONG_LIVED_STOCK_PICKING_AGENTS_DEMO_PLAN.md`
- existing local/remote Ollama demo patterns from `v0.87.1` and `v0.89`
- no blocking GitHub issue dependency

## Demo Expectations

- canonical proof path is fixture replay
- optional model path may use local Ollama models such as `gemma4:latest`
- optional remote model path may use `OLLAMA_HOST` / `OLLAMA_HOST_URL`
- no real trading or investment advice is permitted

## Non-goals

- real trading
- broker integration
- personalized financial advice
- paid market-data dependencies
- live intraday quote trading
- claiming stock-picking skill or market-beating ability
- adding a new version label or changing milestone scope

## Issue-Graph Notes

- New backlog/demo prototype seeded from the local planning doc.
- This issue is intentionally seeded for the `v0.90` planning band rather than
  the active `v0.89.1` execution wave running in another session.
- Later promotion can move the demo into a future milestone once the fixture
  proof path is real.

## Notes

- Prefer deterministic fixture data and mockable local-model outputs for the
  proof path.
- Record available Ollama models as capability inputs, not as a requirement
  that every reviewer must have the same machine.
- Keep any optional remote Ollama usage behind explicit `OLLAMA_HOST` or
  `OLLAMA_HOST_URL` configuration.

## Tooling Notes

- Keep `.adl` local-only.
- Follow the conductor-guided `pr-init` -> `pr-run` -> `pr-finish` flow.
- Keep tracked implementation in the issue worktree, not root `main`.

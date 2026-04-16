# Long-Lived Stock League Demo

This bounded `v0.89.1` demo sets up a paper-market league for long-lived
agents. It is designed to prove persistent identity, append-only memory,
paper-only commitments, outcome tracking, and self-review under noisy market
conditions.

It does not provide financial advice. It does not place trades. It does not
connect to a broker. The canonical proof path uses synthetic fixture prices and
does not require network access.

## What It Proves

- Several agents can keep stable identities across simulated market days.
- Each agent can make timestamped paper-only commitments before outcomes are
  known.
- The system can track hypothetical portfolio outcomes against a fixture
  market.
- A referee can audit identity drift, hindsight edits, and illegal real-trading
  language.
- Local and remote Ollama model rosters can be recorded as optional agent-engine
  capability inputs without making live models required for proof.

## Fastest Proof Path

```bash
bash adl/tools/test_demo_v0891_long_lived_stock_league.sh
```

This uses:

- synthetic fixture prices
- deterministic model-roster fixture
- no real market feed
- no broker
- no live model generation

## Demo Run

```bash
bash adl/tools/demo_v0891_long_lived_stock_league.sh
```

Primary artifact root:

- `artifacts/v0891/long_lived_stock_league/`

Key artifacts:

- `season_manifest.json`
- `model_roster.json`
- `agents/*/identity.json`
- `agents/*/memory_journal.jsonl`
- `agents/*/portfolio_ledger.jsonl`
- `market/snapshots/*.json`
- `scoreboard/final_scoreboard.json`
- `scoreboard/weekly_report.md`
- `audit/guardrail_report.json`
- `audit/hindsight_edit_report.md`
- `audit/identity_drift_report.md`
- `audit/data_source_report.md`

## Optional Ollama Roster Discovery

The canonical proof does not require Ollama discovery. To inspect available
models without asking them for picks:

```bash
bash adl/tools/demo_v0891_long_lived_stock_league.sh --discover-models
```

To inspect the larger remote Ollama node used in earlier demos:

```bash
STOCK_LEAGUE_REMOTE_OLLAMA_HOST=192.168.68.73 \
bash adl/tools/demo_v0891_long_lived_stock_league.sh --discover-models
```

Model discovery only writes `model_roster.json`. It does not make the models
select securities or produce financial advice.

The fixture records the remote node as an optional high-capacity host with
operator-reported 128 GB system RAM and an `RX-3090` with 24 GB VRAM. That
hardware is useful for larger model conversations, but the canonical demo still
proves from deterministic fixtures and does not require the remote node to be
online.

## Agent Cast

- The Value Monk uses a valuation-first identity and prefers `gemma4:latest`.
- The Momentum Surfer uses a trend-following identity and can bind to Qwen
  models.
- The Contrarian Raccoon uses a sentiment-reversal identity and must avoid
  disagreement for its own sake.
- The Quality Gardener uses a durable-compounder identity and must name entry
  price risk.
- The Macro Weather Oracle uses broad market proxies and must timestamp regime
  claims.
- The Risk Goblin is non-competing and challenges concentration, drawdown, and
  rule violations.
- The Archivist Referee is non-competing and audits memory, hindsight, and
  identity drift.

## Truth Boundaries

- Scores are paper-only demonstration artifacts.
- Synthetic fixture prices are not historical market data.
- Rankings are not investment recommendations.
- The "best" agent is not simply the highest return. The scoreboard also
  weights calibration and identity consistency.
- Real trading verbs are rejected by the paper broker guardrail.

## Why This Demo Matters

The market is a useful external pressure source because agents cannot control
the outcome. That makes it a good stage for continuous identity: agents have to
remember what they said, live with the paper result, and explain what changed
without laundering the past.

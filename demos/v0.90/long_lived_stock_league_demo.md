# Long-Lived Stock League Demo

This is a paper-market simulation for demonstrating persistent agent identity
and accountability. It is not financial advice, trading advice, or a real
investment strategy.

## Status

WP-07 scaffold plus WP-08 recurring long-lived integration.

The canonical reviewer path is fixture replay. It requires no network, no API
keys, no broker integration, no live quote feed, and no real-world side effects.

## Proof Command

```bash
cargo run --manifest-path adl/Cargo.toml -- demo demo-j-v090-stock-league-recurring --run --trace --out out --no-open
```

The recurring integration command writes a reviewer artifact root under:

```text
out/demo-j-v090-stock-league-recurring/
```

## Primary Proof Surfaces

- `integration_proof_packet.json`: reviewer entry point for the recurring
  integration claim.
- `integration_manifest.json`: paper-only season metadata, runtime state refs,
  inspection refs, and generated proof index.
- `long_lived_agent/state/status.json`: completed long-lived-agent runtime
  status.
- `long_lived_agent/state/cycle_ledger.jsonl`: append-only ledger for the
  recurring cycles.
- `long_lived_agent/state/cycles/*/cycle_manifest.json`: per-cycle manifests
  with previous-cycle links.
- `inspection/latest.json`: reviewer inspection packet over the latest cycle.
- `inspection/cycle-000001.json`: prior-cycle inspection packet for comparison.
- `continuity/continuity_proof.json`: proof that the first-cycle artifacts and
  prior-cycle links remain visible after later cycles.
- `league_rules.json`: paper-market rules and explicit forbidden actions.
- `agents/*/identity.json`: persistent agent identity and style cards.
- `fixture/season_001_fixture.json`: committed deterministic market fixture
  copied into the run output.
- `decisions/day-001.json`: timestamped paper-only demo decisions.
- `paper_ledger.jsonl`: hypothetical portfolio ledger entries.
- `audit/recurring_guardrail_summary.json`: per-cycle no-real-trading and
  no-advice guardrail summary.
- `audit/artifact_safety_scan.json`: host-path, secret, credential, and claim
  scan for public artifacts.

## Guardrails

- The demo does not place orders.
- The demo does not connect to broker APIs.
- The demo does not use personalized user financial information.
- The demo does not claim market-beating ability.
- The recurring integration uses three no-sleep fixture cycles; it does not
  require a hidden daemon or long-running process.
- The demo scores identity accountability, paper-risk discipline, calibration,
  and explanation quality rather than raw return alone.

## Scaffold Command

The original WP-07 scaffold remains available when reviewers only need the
fixture, identity cards, paper rules, and guardrail scaffold:

```bash
cargo run --manifest-path adl/Cargo.toml -- demo demo-i-v090-stock-league-scaffold --run --trace --out out --no-open
```

WP-08 adds recurring long-lived integration across cycles, status, continuity,
guardrails, and inspection artifacts.

## Extension Proof Command

WP-09 adds a bounded reviewer-evidence extension without changing the primary
D4 proof path:

```bash
cargo run --manifest-path adl/Cargo.toml -- demo demo-k-v090-stock-league-proof-expansion --run --trace --out out --no-open
```

The extension writes `extension_proof_packet.json` plus `extensions/*` evidence
artifacts. See `demos/v0.90/stock_league_demo_extensions.md` for the D5
selection, non-goals, deferrals, and reviewer path.

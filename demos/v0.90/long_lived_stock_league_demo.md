# Long-Lived Stock League Demo Scaffold

This is a paper-market simulation for demonstrating persistent agent identity
and accountability. It is not financial advice, trading advice, or a real
investment strategy.

## Status

WP-07 scaffold.

The canonical reviewer path is fixture replay. It requires no network, no API
keys, no broker account, no live quote feed, and no real-world side effects.

## Proof Command

```bash
cargo run --manifest-path adl/Cargo.toml -- demo demo-i-v090-stock-league-scaffold --run --trace --out out --no-open
```

The command writes a deterministic demo artifact root under:

```text
out/demo-i-v090-stock-league-scaffold/
```

## Primary Proof Surfaces

- `proof_packet.json`: reviewer entry point for the scaffold claim.
- `season_manifest.json`: paper-only season metadata and generated artifact
  index.
- `league_rules.json`: paper-market rules and explicit forbidden actions.
- `agents/*/identity.json`: persistent agent identity and style cards.
- `fixture/season_001_fixture.json`: committed deterministic market fixture
  copied into the run output.
- `decisions/day-001.json`: timestamped paper-only demo decisions.
- `paper_ledger.jsonl`: hypothetical portfolio ledger entries.
- `audit/guardrail_report.json`: no-real-trading and no-advice guardrails.
- `audit/artifact_safety_scan.json`: host-path, secret, credential, and claim
  scan for public artifacts.

## Guardrails

- The demo does not place orders.
- The demo does not connect to broker APIs.
- The demo does not use personalized user financial information.
- The demo does not claim market-beating ability.
- The demo scores identity accountability, paper-risk discipline, calibration,
  and explanation quality rather than raw return alone.

## WP-07 Boundary

WP-07 builds the fixture-backed scaffold and safety docs. WP-08 owns recurring
long-lived integration across cycles, status, continuity, and inspection
artifacts.

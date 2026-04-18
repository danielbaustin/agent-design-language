# Stock League Demo Extensions

This document records the bounded v0.90 demo-extension lane for the long-lived
stock-league proof path.

## Selected Extension

WP-09 selects one extension:

- `D5-A stock_league_reviewer_evidence_index`

The extension does not create a competing flagship demo. It wraps the D4
recurring stock-league proof and adds reviewer-oriented evidence that makes the
proof easier to audit.

## Proof Command

```bash
cargo run --manifest-path adl/Cargo.toml -- demo demo-k-v090-stock-league-proof-expansion --run --trace --out out --no-open
```

The command writes:

```text
out/demo-k-v090-stock-league-proof-expansion/
```

## Proof Claim

The D5 extension proves that ADL can add a named, bounded demo-extension packet
without weakening the primary stock-league proof path. The extension reruns the
D4 recurring fixture path and then emits a selected-demo manifest, claim
registry, evidence index, replay manifest, non-goals and deferrals register,
extension safety scan, and proof packet.

## Primary Proof Surfaces

- `demo_extension_selection.json`: selected demo choices, supporting
  non-proving surfaces, and explicit deferrals.
- `integration_proof_packet.json`: reused D4 recurring stock-league proof.
- `continuity/continuity_proof.json`: prior-cycle links and preserved
  first-cycle artifacts.
- `audit/recurring_guardrail_summary.json`: per-cycle paper-only guardrails.
- `extensions/proof_claims.json`: claim-by-claim extension proof registry.
- `extensions/evidence_index.json`: reviewer index from extension claims to
  source artifacts.
- `extensions/replay_manifest.json`: deterministic replay command and expected
  artifact list.
- `extensions/non_goals_and_deferrals.json`: explicit scope boundaries.
- `extensions/extension_artifact_safety_scan.json`: public-artifact scan for
  host paths, secrets, broker credentials, broad performance claims, and
  live-order surfaces.
- `extension_proof_packet.json`: reviewer entry point for D5.

## Non-Goals And Deferrals

The extension explicitly defers live market data, broker/order integration, and
multi-provider competitive league behavior. Those would add review burden and
external dependencies beyond v0.90's fixture-backed long-lived state proof.

The stock-league proof remains paper-only. It is not financial advice, trading
advice, or an investment strategy.

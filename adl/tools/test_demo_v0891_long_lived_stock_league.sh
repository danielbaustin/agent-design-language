#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACT_ROOT="$ROOT_DIR/artifacts/test_long_lived_stock_league"
ROSTER_FIXTURE="$ROOT_DIR/demos/fixtures/long_lived_stock_league/model_roster_fixture.json"

rm -rf "$ARTIFACT_ROOT"
bash "$ROOT_DIR/adl/tools/demo_v0891_long_lived_stock_league.sh" \
  --artifact-root "$ARTIFACT_ROOT" \
  --model-roster-fixture "$ROSTER_FIXTURE" \
  >/dev/null

python3 - "$ARTIFACT_ROOT" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])

def load(rel):
    with (root / rel).open("r", encoding="utf-8") as fh:
        return json.load(fh)

manifest = load("demo_manifest.json")
assert manifest["disposition"] == "proving"
assert manifest["guardrails"]["paper_only"] is True
assert manifest["guardrails"]["real_trading_enabled"] is False
assert manifest["proof"]["agent_count"] >= 4
assert manifest["proof"]["market_day_count"] >= 5
assert manifest["proof"]["rejected_illegal_decision_count"] == 1

roster = load("model_roster.json")
all_models = {
    model["name"]
    for source in roster["sources"]
    for model in source["models"]
}
assert "gemma4:latest" in all_models
assert "gpt-oss:120b" in all_models
remote = next(source for source in roster["sources"] if source["source_id"] == "remote_ollama")
assert remote["hardware"]["system_ram_gb"] == 128
assert remote["hardware"]["gpu"]["vram_gb"] == 24

scoreboard = load("scoreboard/final_scoreboard.json")
assert scoreboard["not_financial_advice"] is True
assert len(scoreboard["scores"]) == 5
assert scoreboard["scores"][0]["composite_score"] >= scoreboard["scores"][-1]["composite_score"]

guardrails = load("audit/guardrail_report.json")
assert guardrails["real_trading_enabled"] is False
assert guardrails["broker_integration"] is False
assert guardrails["rejected_decisions"][0]["reason"].startswith("unsupported paper action")

identity = load("agents/value_monk/identity.json")
assert identity["model_binding"]["model"] == "gemma4:latest"
assert identity["memory_policy"]["append_only_journal"] is True

journal = root / "agents/value_monk/memory_journal.jsonl"
assert journal.exists()
assert journal.read_text(encoding="utf-8").strip()

for path in root.rglob("*"):
    if path.is_file():
        text = path.read_text(encoding="utf-8", errors="ignore")
        assert "/Users/" not in text, f"absolute host path leaked in {path}"
        assert "execute real trade" not in text.lower()
PY

grep -Fq "Canonical mode: fixture replay" "$ARTIFACT_ROOT/audit/data_source_report.md"
grep -Fq "Remote Ollama fixture hardware: 128 GB system RAM, RX-3090 with 24 GB VRAM" "$ARTIFACT_ROOT/audit/data_source_report.md"
grep -Fq "not financial advice" "$ARTIFACT_ROOT/run_summary.md"

echo "demo_v0891_long_lived_stock_league: ok"

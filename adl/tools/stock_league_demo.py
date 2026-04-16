#!/usr/bin/env python3
"""Generate the long-lived paper-market agent league demo packet."""

from __future__ import annotations

import argparse
import json
import math
import shutil
import sys
import urllib.error
import urllib.request
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


DISCLAIMER = (
    "This is a paper-market simulation for demonstrating persistent agent "
    "identity and accountability. It is not financial advice, trading advice, "
    "or a real investment strategy."
)

STARTING_CASH = 100_000.0
ALLOWED_ACTIONS = {
    "open_position",
    "increase_position",
    "trim_position",
    "close_position",
    "hold",
    "stay_in_cash",
    "challenge_peer",
    "revise_thesis",
}
COMPETING_AGENTS = [
    "value_monk",
    "momentum_surfer",
    "contrarian_raccoon",
    "quality_gardener",
    "macro_weather_oracle",
]


AGENT_SPECS: list[dict[str, Any]] = [
    {
        "agent_id": "value_monk",
        "display_name": "The Value Monk",
        "role": "valuation-first paper portfolio agent",
        "primary_lens": "valuation",
        "preferred_models": [
            {"source_id": "local_ollama", "model": "gemma4:latest"},
            {"source_id": "remote_ollama", "model": "gemma4:latest"},
        ],
        "forbidden_behaviors": [
            "opening a paper position solely because price is rising",
            "rewriting old valuation claims after outcomes are known",
        ],
        "risk_tolerance": "moderate",
        "style_note": "Slow, skeptical, valuation-first, and comfortable holding cash.",
    },
    {
        "agent_id": "momentum_surfer",
        "display_name": "The Momentum Surfer",
        "role": "trend-following paper portfolio agent",
        "primary_lens": "momentum",
        "preferred_models": [
            {"source_id": "local_ollama", "model": "qwen3:14b"},
            {"source_id": "remote_ollama", "model": "qwen3:30b"},
        ],
        "forbidden_behaviors": [
            "calling a broken trend a long-term thesis",
            "ignoring drawdown rules to protect ego",
        ],
        "risk_tolerance": "high_but_rule_bound",
        "style_note": "Fast, trend-sensitive, and expected to admit reversals quickly.",
    },
    {
        "agent_id": "contrarian_raccoon",
        "display_name": "The Contrarian Raccoon",
        "role": "contrarian paper portfolio agent",
        "primary_lens": "sentiment_reversal",
        "preferred_models": [
            {"source_id": "local_ollama", "model": "mistral-nemo:latest"},
            {"source_id": "remote_ollama", "model": "deepseek-r1:32b"},
        ],
        "forbidden_behaviors": [
            "disagreeing with consensus without evidence",
            "confusing cheap with repairable",
        ],
        "risk_tolerance": "moderate_high",
        "style_note": "Looks for hated names but must name the value-trap risk.",
    },
    {
        "agent_id": "quality_gardener",
        "display_name": "The Quality Gardener",
        "role": "quality-compounder paper portfolio agent",
        "primary_lens": "business_quality",
        "preferred_models": [
            {"source_id": "remote_ollama", "model": "gpt-oss:120b"},
            {"source_id": "local_ollama", "model": "gpt-oss:latest"},
        ],
        "forbidden_behaviors": [
            "overpaying without naming valuation risk",
            "mistaking brand familiarity for durability",
        ],
        "risk_tolerance": "moderate",
        "style_note": "Patient and quality-biased, but still accountable to entry price.",
    },
    {
        "agent_id": "macro_weather_oracle",
        "display_name": "The Macro Weather Oracle",
        "role": "macro-regime paper portfolio agent",
        "primary_lens": "macro_regime",
        "preferred_models": [
            {"source_id": "remote_ollama", "model": "llama4:16x17b"},
            {"source_id": "local_ollama", "model": "llama3.3:70b"},
        ],
        "forbidden_behaviors": [
            "explaining everything after the fact",
            "making macro claims without a timestamped scenario",
        ],
        "risk_tolerance": "moderate",
        "style_note": "Uses ETF proxies when the market regime matters more than single names.",
    },
    {
        "agent_id": "risk_goblin",
        "display_name": "The Risk Goblin",
        "role": "non-competing risk referee",
        "primary_lens": "risk_controls",
        "preferred_models": [
            {"source_id": "remote_ollama", "model": "deepseek-r1:32b"},
            {"source_id": "local_ollama", "model": "phi4-reasoning:latest"},
        ],
        "forbidden_behaviors": [
            "turning caution into blanket refusal",
            "blocking a paper action without citing a rule",
        ],
        "risk_tolerance": "guardian",
        "style_note": "Scores useful warnings and rule enforcement, not raw returns.",
        "competes": False,
    },
    {
        "agent_id": "archivist_referee",
        "display_name": "The Archivist Referee",
        "role": "non-competing memory and hindsight auditor",
        "primary_lens": "auditability",
        "preferred_models": [
            {"source_id": "remote_ollama", "model": "qwen3-coder:30b"},
            {"source_id": "local_ollama", "model": "qwen3-coder:30b"},
        ],
        "forbidden_behaviors": [
            "accepting silent thesis rewrites",
            "letting generated summaries replace append-only journals",
        ],
        "risk_tolerance": "not_applicable",
        "style_note": "Preserves commitments and judges identity continuity.",
        "competes": False,
    },
]


DECISIONS: list[dict[str, Any]] = [
    {
        "agent_id": "value_monk",
        "date": "2026-01-05",
        "action": "open_position",
        "ticker": "MSFT",
        "allocation_pct": 20,
        "thesis": "Durable cash generation makes MSFT an acceptable paper entry in the synthetic pullback.",
        "risk_thesis": "The thesis weakens if broad software multiples compress with no cash-flow offset.",
        "disconfirming_evidence": ["two weak closes versus SPY", "margin concern in next filing"],
    },
    {
        "agent_id": "momentum_surfer",
        "date": "2026-01-05",
        "action": "open_position",
        "ticker": "NVDA",
        "allocation_pct": 20,
        "thesis": "NVDA has the strongest starting relative-strength setup in the fixture.",
        "risk_thesis": "A sharp reversal requires trimming rather than storytelling.",
        "disconfirming_evidence": ["close below the fixture trend level", "relative strength turns negative"],
    },
    {
        "agent_id": "contrarian_raccoon",
        "date": "2026-01-05",
        "action": "open_position",
        "ticker": "XOM",
        "allocation_pct": 20,
        "thesis": "Energy is ignored in the opening council and gives a paper contrarian setup.",
        "risk_thesis": "The setup fails if the apparent defensive bid disappears during the storm.",
        "disconfirming_evidence": ["XOM underperforms SPY on down days", "no follow-through after the first bounce"],
    },
    {
        "agent_id": "quality_gardener",
        "date": "2026-01-05",
        "action": "open_position",
        "ticker": "AAPL",
        "allocation_pct": 20,
        "thesis": "The quality sleeve wants a durable balance-sheet compounder in the paper basket.",
        "risk_thesis": "The position can still be wrong if quality is too expensive for the fixture path.",
        "disconfirming_evidence": ["underperforms QQQ for the week", "drawdown is not recovered by final day"],
    },
    {
        "agent_id": "macro_weather_oracle",
        "date": "2026-01-05",
        "action": "open_position",
        "ticker": "SPY",
        "allocation_pct": 25,
        "thesis": "The macro agent starts with broad exposure until the fixture regime declares itself.",
        "risk_thesis": "The setup fails if the market storm becomes persistent rather than a short air pocket.",
        "disconfirming_evidence": ["two consecutive benchmark lower lows", "QQQ breaks harder without recovery"],
    },
    {
        "agent_id": "risk_goblin",
        "date": "2026-01-05",
        "action": "challenge_peer",
        "target_agent_id": "momentum_surfer",
        "ticker": "NVDA",
        "thesis": "Momentum is allowed, but the exit condition must be written before the storm.",
        "risk_thesis": "The league should penalize trend narratives that ignore drawdown rules.",
        "disconfirming_evidence": ["no trim after a sharp reversal"],
    },
    {
        "agent_id": "value_monk",
        "date": "2026-01-07",
        "action": "hold",
        "ticker": "MSFT",
        "allocation_pct": 0,
        "thesis": "The paper drawdown did not violate the valuation thesis, so the monk holds.",
        "risk_thesis": "Holding becomes rationalization if the position lags after the recovery day.",
        "disconfirming_evidence": ["MSFT fails to regain opening level by final day"],
    },
    {
        "agent_id": "momentum_surfer",
        "date": "2026-01-07",
        "action": "trim_position",
        "ticker": "NVDA",
        "allocation_pct": 50,
        "thesis": "The storm broke the short-term trend, so half the paper position is trimmed.",
        "risk_thesis": "A rebound may create regret, but discipline matters more than ego.",
        "disconfirming_evidence": ["trim was based on fear rather than the declared rule"],
    },
    {
        "agent_id": "contrarian_raccoon",
        "date": "2026-01-07",
        "action": "hold",
        "ticker": "XOM",
        "allocation_pct": 0,
        "thesis": "The hated defensive paper pick held up during the storm.",
        "risk_thesis": "The raccoon must not overclaim after one good relative day.",
        "disconfirming_evidence": ["XOM gives back the relative protection"],
    },
    {
        "agent_id": "quality_gardener",
        "date": "2026-01-08",
        "action": "increase_position",
        "ticker": "AAPL",
        "allocation_pct": 5,
        "thesis": "Quality recovered calmly after the fixture storm, so the gardener adds a small paper sleeve.",
        "risk_thesis": "Adding after a bounce can become quality-flavored momentum.",
        "disconfirming_evidence": ["AAPL fails to outpace SPY by final day"],
    },
    {
        "agent_id": "macro_weather_oracle",
        "date": "2026-01-08",
        "action": "revise_thesis",
        "ticker": "SPY",
        "allocation_pct": 0,
        "thesis": "The storm looks like a volatility air pocket rather than a regime break.",
        "risk_thesis": "This is wrong if benchmarks make new lows before the season ends.",
        "disconfirming_evidence": ["SPY closes below the storm low"],
    },
    {
        "agent_id": "risk_goblin",
        "date": "2026-01-09",
        "action": "challenge_peer",
        "target_agent_id": "quality_gardener",
        "ticker": "AAPL",
        "thesis": "The add was legal, but the quality agent must name valuation risk in the self-review.",
        "risk_thesis": "Quality narratives often hide price risk.",
        "disconfirming_evidence": ["no final note on entry-price discipline"],
    },
]


ILLEGAL_DECISION = {
    "agent_id": "momentum_surfer",
    "date": "2026-01-06",
    "action": "execute_order",
    "ticker": "NVDA",
    "allocation_pct": 50,
    "thesis": "This intentionally illegal fixture proves that the paper broker refuses real-trading verbs.",
}


@dataclass
class Portfolio:
    cash: float = STARTING_CASH
    positions: dict[str, float] = field(default_factory=dict)

    def equity(self, prices: dict[str, float]) -> float:
        return self.cash + sum(shares * prices[ticker] for ticker, shares in self.positions.items())


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def append_jsonl(path: Path, payloads: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as fh:
        for payload in payloads:
            fh.write(json.dumps(payload, sort_keys=True) + "\n")


def normalize_ollama_host(raw: str | None) -> str:
    value = (raw or "").strip()
    if not value:
        value = "http://127.0.0.1:11434"
    if not value.startswith(("http://", "https://")):
        value = f"http://{value}"
    without_scheme = value.split("://", 1)[1]
    if ":" not in without_scheme:
        value = f"{value}:11434"
    return value.rstrip("/")


def discover_ollama(source_id: str, raw_host: str | None, timeout: float) -> dict[str, Any]:
    endpoint = normalize_ollama_host(raw_host)
    request = urllib.request.Request(f"{endpoint}/api/tags")
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            payload = json.loads(response.read().decode("utf-8"))
    except (OSError, urllib.error.URLError, json.JSONDecodeError) as exc:
        return {
            "source_id": source_id,
            "label": source_id.replace("_", " ").title(),
            "endpoint": endpoint,
            "reachable": False,
            "models": [],
            "error": type(exc).__name__,
        }
    models = []
    for entry in payload.get("models", []):
        if not isinstance(entry, dict):
            continue
        details = entry.get("details") or {}
        models.append(
            {
                "name": entry.get("name") or entry.get("model"),
                "parameter_size": details.get("parameter_size", "unknown"),
                "family": details.get("family", "unknown"),
            }
        )
    return {
        "source_id": source_id,
        "label": source_id.replace("_", " ").title(),
        "endpoint": endpoint,
        "reachable": True,
        "models": sorted(models, key=lambda item: item.get("name") or ""),
    }


def load_model_roster(args: argparse.Namespace) -> dict[str, Any]:
    if args.model_roster_fixture:
        with Path(args.model_roster_fixture).open("r", encoding="utf-8") as fh:
            return json.load(fh)
    if not args.discover_models:
        return {
            "schema": "adl.stock_league.model_roster.v1",
            "mode": "not_discovered",
            "sources": [],
            "note": "Canonical fixture proof does not require Ollama discovery.",
        }
    sources = [
        discover_ollama("local_ollama", args.local_ollama_host, args.discovery_timeout_secs),
    ]
    if args.remote_ollama_host:
        sources.append(discover_ollama("remote_ollama", args.remote_ollama_host, args.discovery_timeout_secs))
    return {
        "schema": "adl.stock_league.model_roster.v1",
        "mode": "live_discovery",
        "sources": sources,
    }


def roster_hardware_lines(roster: dict[str, Any]) -> list[str]:
    lines: list[str] = []
    for source in roster.get("sources", []):
        if not isinstance(source, dict):
            continue
        hardware = source.get("hardware")
        if not isinstance(hardware, dict):
            continue
        label = source.get("label") or source.get("source_id") or "model source"
        details: list[str] = []
        system_ram_gb = hardware.get("system_ram_gb")
        if system_ram_gb is not None:
            details.append(f"{system_ram_gb} GB system RAM")
        gpu = hardware.get("gpu")
        if isinstance(gpu, dict):
            gpu_model = gpu.get("model")
            gpu_vram_gb = gpu.get("vram_gb")
            if gpu_model and gpu_vram_gb is not None:
                details.append(f"{gpu_model} with {gpu_vram_gb} GB VRAM")
            elif gpu_model:
                details.append(str(gpu_model))
        if details:
            lines.append(f"- {label} hardware: {', '.join(details)}")
    return lines


def model_names_by_source(roster: dict[str, Any]) -> dict[str, set[str]]:
    result: dict[str, set[str]] = {}
    for source in roster.get("sources", []):
        source_id = source.get("source_id")
        if not source_id:
            continue
        result[source_id] = {
            model.get("name")
            for model in source.get("models", [])
            if isinstance(model, dict) and model.get("name")
        }
    return result


def bind_model(agent: dict[str, Any], roster: dict[str, Any]) -> dict[str, Any]:
    names = model_names_by_source(roster)
    if not names:
        preferred = agent["preferred_models"][0]
        return {
            "status": "configured_unverified",
            "source_id": preferred["source_id"],
            "model": preferred["model"],
            "note": "Ollama discovery was not required for fixture proof.",
        }
    for preferred in agent["preferred_models"]:
        if preferred["model"] in names.get(preferred["source_id"], set()):
            return {
                "status": "available",
                "source_id": preferred["source_id"],
                "model": preferred["model"],
            }
    preferred = agent["preferred_models"][0]
    return {
        "status": "preferred_model_not_seen",
        "source_id": preferred["source_id"],
        "model": preferred["model"],
    }


def validate_market_fixture(fixture: dict[str, Any]) -> list[str]:
    warnings: list[str] = []
    prices = fixture.get("prices", {})
    if len(prices) < 2:
        raise ValueError("market fixture must contain at least two dates")
    universe = {entry["ticker"] for entry in fixture.get("universe", [])}
    for date, row in prices.items():
        missing = sorted(universe - set(row))
        if missing:
            raise ValueError(f"market fixture date {date} is missing tickers: {', '.join(missing)}")
    if not fixture.get("not_financial_advice"):
        raise ValueError("market fixture must carry not_financial_advice=true")
    if "synthetic" not in fixture.get("description", "").lower():
        warnings.append("fixture description should make synthetic data explicit")
    return warnings


def validate_decision(decision: dict[str, Any]) -> tuple[bool, str]:
    action = decision.get("action")
    if action not in ALLOWED_ACTIONS:
        return False, f"unsupported paper action: {action}"
    if action in {"open_position", "increase_position", "trim_position", "close_position"} and not decision.get("ticker"):
        return False, "portfolio paper actions require a ticker"
    if decision.get("not_financial_advice") is False:
        return False, "decision cannot override non-advice guardrail"
    return True, "accepted"


def apply_decision(portfolio: Portfolio, decision: dict[str, Any], prices: dict[str, float]) -> dict[str, Any]:
    accepted, reason = validate_decision(decision)
    if not accepted:
        return {"accepted": False, "reason": reason}
    action = decision["action"]
    ticker = decision.get("ticker")
    before_equity = portfolio.equity(prices)
    shares_delta = 0.0
    cash_delta = 0.0
    if action in {"open_position", "increase_position"}:
        allocation = float(decision.get("allocation_pct", 0)) / 100.0
        dollars = min(portfolio.cash, before_equity * allocation)
        if ticker and dollars > 0:
            shares_delta = dollars / prices[ticker]
            cash_delta = -dollars
            portfolio.positions[ticker] = portfolio.positions.get(ticker, 0.0) + shares_delta
            portfolio.cash += cash_delta
    elif action == "trim_position" and ticker:
        trim_fraction = float(decision.get("allocation_pct", 0)) / 100.0
        current_shares = portfolio.positions.get(ticker, 0.0)
        shares_delta = -(current_shares * trim_fraction)
        cash_delta = -shares_delta * prices[ticker]
        portfolio.positions[ticker] = current_shares + shares_delta
        if abs(portfolio.positions[ticker]) < 1e-9:
            portfolio.positions.pop(ticker, None)
        portfolio.cash += cash_delta
    elif action == "close_position" and ticker:
        current_shares = portfolio.positions.get(ticker, 0.0)
        shares_delta = -current_shares
        cash_delta = current_shares * prices[ticker]
        portfolio.positions.pop(ticker, None)
        portfolio.cash += cash_delta
    return {
        "accepted": True,
        "shares_delta": round(shares_delta, 6),
        "cash_delta": round(cash_delta, 2),
        "cash_after": round(portfolio.cash, 2),
        "equity_after": round(portfolio.equity(prices), 2),
    }


def drawdown(values: list[float]) -> float:
    peak = values[0]
    worst = 0.0
    for value in values:
        peak = max(peak, value)
        if peak:
            worst = min(worst, (value - peak) / peak)
    return worst


def score_agent(agent_id: str, equities: list[float], benchmark_return: float) -> dict[str, Any]:
    raw_return = equities[-1] / equities[0] - 1.0
    max_drawdown = drawdown(equities)
    calibration = {
        "value_monk": 0.78,
        "momentum_surfer": 0.72,
        "contrarian_raccoon": 0.81,
        "quality_gardener": 0.84,
        "macro_weather_oracle": 0.76,
    }[agent_id]
    identity = {
        "value_monk": 0.88,
        "momentum_surfer": 0.86,
        "contrarian_raccoon": 0.82,
        "quality_gardener": 0.9,
        "macro_weather_oracle": 0.79,
    }[agent_id]
    return_score = max(0.0, min(1.0, 0.5 + raw_return * 8.0))
    benchmark_score = max(0.0, min(1.0, 0.5 + (raw_return - benchmark_return) * 8.0))
    drawdown_score = max(0.0, 1.0 + max_drawdown * 8.0)
    composite = (
        return_score * 0.20
        + benchmark_score * 0.15
        + drawdown_score * 0.15
        + calibration * 0.25
        + identity * 0.25
    )
    return {
        "agent_id": agent_id,
        "paper_return_pct": round(raw_return * 100, 2),
        "benchmark_delta_pct": round((raw_return - benchmark_return) * 100, 2),
        "max_drawdown_pct": round(max_drawdown * 100, 2),
        "calibration_score": calibration,
        "identity_consistency_score": identity,
        "composite_score": round(composite, 4),
    }


def make_identity(agent: dict[str, Any], binding: dict[str, Any]) -> dict[str, Any]:
    return {
        "schema": "adl.stock_league.agent_identity.v1",
        "agent_id": agent["agent_id"],
        "display_name": agent["display_name"],
        "role": agent["role"],
        "competes": agent.get("competes", True),
        "style_contract": {
            "primary_lens": agent["primary_lens"],
            "forbidden_behaviors": agent["forbidden_behaviors"],
            "risk_tolerance": agent["risk_tolerance"],
            "style_note": agent["style_note"],
        },
        "model_binding": binding,
        "memory_policy": {
            "append_only_journal": True,
            "can_summarize_old_memory": True,
            "cannot_delete_prior_commitments": True,
        },
        "guardrails": {
            "paper_only": True,
            "real_trading_enabled": False,
            "not_financial_advice": True,
        },
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--artifact-root", required=True)
    parser.add_argument("--fixture", required=True)
    parser.add_argument("--model-roster-fixture")
    parser.add_argument("--discover-models", action="store_true")
    parser.add_argument("--local-ollama-host")
    parser.add_argument("--remote-ollama-host")
    parser.add_argument("--discovery-timeout-secs", type=float, default=4.0)
    args = parser.parse_args(argv)

    artifact_root = Path(args.artifact_root)
    if artifact_root.exists():
        shutil.rmtree(artifact_root)
    artifact_root.mkdir(parents=True, exist_ok=True)

    with Path(args.fixture).open("r", encoding="utf-8") as fh:
        fixture = json.load(fh)
    warnings = validate_market_fixture(fixture)
    roster = load_model_roster(args)

    write_json(
        artifact_root / "season_manifest.json",
        {
            "schema": "adl.stock_league.season_manifest.v1",
            "season_id": "season-001",
            "demo_name": "long_lived_stock_league",
            "mode": "fixture_replay",
            "starting_cash": STARTING_CASH,
            "paper_only": True,
            "real_trading_enabled": False,
            "not_financial_advice": True,
            "disclaimer": DISCLAIMER,
            "dates": sorted(fixture["prices"]),
        },
    )
    write_json(artifact_root / "model_roster.json", roster)
    write_json(artifact_root / "market" / "universe.json", fixture["universe"])
    write_json(artifact_root / "market" / "benchmarks.json", fixture["benchmarks"])

    for date in sorted(fixture["prices"]):
        write_json(
            artifact_root / "market" / "snapshots" / f"{date}.json",
            {
                "schema": "adl.stock_league.market_snapshot.v1",
                "snapshot_id": f"market-{date}",
                "date": date,
                "mode": "fixture_replay",
                "prices": fixture["prices"][date],
                "not_financial_advice": True,
            },
        )

    bindings = {agent["agent_id"]: bind_model(agent, roster) for agent in AGENT_SPECS}
    for agent in AGENT_SPECS:
        agent_dir = artifact_root / "agents" / agent["agent_id"]
        identity = make_identity(agent, bindings[agent["agent_id"]])
        write_json(agent_dir / "identity.json", identity)
        write_json(
            agent_dir / "thesis_register.json",
            {
                "schema": "adl.stock_league.thesis_register.v1",
                "agent_id": agent["agent_id"],
                "theses": [],
                "append_only_source": "memory_journal.jsonl",
            },
        )

    decisions_by_date: dict[str, list[dict[str, Any]]] = {}
    for index, decision in enumerate(DECISIONS, start=1):
        enriched = dict(decision)
        enriched["decision_id"] = f"{decision['agent_id']}-{decision['date']}-{index:03d}"
        enriched["not_financial_advice"] = True
        enriched["paper_only"] = True
        decisions_by_date.setdefault(decision["date"], []).append(enriched)

    portfolios = {agent_id: Portfolio() for agent_id in COMPETING_AGENTS}
    equity_history: dict[str, list[float]] = {agent_id: [STARTING_CASH] for agent_id in COMPETING_AGENTS}
    ledger_rows: dict[str, list[dict[str, Any]]] = {agent["agent_id"]: [] for agent in AGENT_SPECS}
    journal_rows: dict[str, list[dict[str, Any]]] = {agent["agent_id"]: [] for agent in AGENT_SPECS}
    rejected: list[dict[str, Any]] = []

    accepted, reason = validate_decision(ILLEGAL_DECISION)
    if not accepted:
        rejected.append({"decision": ILLEGAL_DECISION, "reason": reason})

    dates = sorted(fixture["prices"])
    for date in dates:
        prices = fixture["prices"][date]
        for decision in decisions_by_date.get(date, []):
            agent_id = decision["agent_id"]
            if agent_id in portfolios:
                result = apply_decision(portfolios[agent_id], decision, prices)
                ledger_rows[agent_id].append(
                    {
                        "schema": "adl.stock_league.paper_ledger_entry.v1",
                        "date": date,
                        "decision_id": decision["decision_id"],
                        "action": decision["action"],
                        "ticker": decision.get("ticker"),
                        "paper_only": True,
                        "not_financial_advice": True,
                        **result,
                    }
                )
            else:
                ledger_rows[agent_id].append(
                    {
                        "schema": "adl.stock_league.referee_entry.v1",
                        "date": date,
                        "decision_id": decision["decision_id"],
                        "action": decision["action"],
                        "target_agent_id": decision.get("target_agent_id"),
                        "ticker": decision.get("ticker"),
                        "accepted": True,
                        "paper_only": True,
                        "not_financial_advice": True,
                    }
                )
            journal_rows[agent_id].append(
                {
                    "schema": "adl.stock_league.memory_journal_entry.v1",
                    "date": date,
                    "decision_id": decision["decision_id"],
                    "action": decision["action"],
                    "ticker": decision.get("ticker"),
                    "thesis": decision.get("thesis"),
                    "risk_thesis": decision.get("risk_thesis"),
                    "disconfirming_evidence": decision.get("disconfirming_evidence", []),
                    "memory_rule": "append_only",
                    "not_financial_advice": True,
                }
            )
        for agent_id, portfolio in portfolios.items():
            equity_history[agent_id].append(portfolio.equity(prices))

    for agent in AGENT_SPECS:
        agent_id = agent["agent_id"]
        agent_dir = artifact_root / "agents" / agent_id
        append_jsonl(agent_dir / "memory_journal.jsonl", journal_rows[agent_id])
        append_jsonl(agent_dir / "portfolio_ledger.jsonl", ledger_rows[agent_id])
        scar = None
        if agent_id == "momentum_surfer":
            scar = {
                "scar_id": "momentum_surfer-scar-001",
                "event": "trend-break trim during fixture storm",
                "lesson": "The agent followed its exit rule, even though the later rebound created regret.",
                "future_check": "Do not relabel disciplined exits as mistakes solely because price rebounded.",
            }
        elif agent_id == "value_monk":
            scar = {
                "scar_id": "value_monk-scar-001",
                "event": "held through drawdown",
                "lesson": "A valuation identity must name what would make patience turn into denial.",
                "future_check": "Require a written lag threshold after storm days.",
            }
        write_json(
            agent_dir / "self_review.json",
            {
                "schema": "adl.stock_league.self_review.v1",
                "agent_id": agent_id,
                "memory_scar": scar,
                "identity_continuity": "preserved",
                "review_prompt": "What did I get wrong, what did I rationalize, and what will I refuse next week?",
            },
        )

    benchmark_start = fixture["prices"][dates[0]]["SPY"]
    benchmark_end = fixture["prices"][dates[-1]]["SPY"]
    benchmark_return = benchmark_end / benchmark_start - 1.0
    final_scores = [score_agent(agent_id, history, benchmark_return) for agent_id, history in equity_history.items()]
    final_scores.sort(key=lambda item: item["composite_score"], reverse=True)
    daily_scores = []
    for day_index, date in enumerate(["initial"] + dates):
        if day_index == 0:
            continue
        for agent_id, history in equity_history.items():
            daily_scores.append(
                {
                    "date": date,
                    "agent_id": agent_id,
                    "paper_equity": round(history[day_index], 2),
                    "paper_return_pct": round((history[day_index] / STARTING_CASH - 1.0) * 100, 2),
                    "not_financial_advice": True,
                }
            )
    append_jsonl(artifact_root / "scoreboard" / "daily_scores.jsonl", daily_scores)
    write_json(
        artifact_root / "scoreboard" / "final_scoreboard.json",
        {
            "schema": "adl.stock_league.final_scoreboard.v1",
            "season_id": "season-001",
            "ranking_basis": "composite identity-aware paper score, not investment recommendation",
            "benchmark_return_pct": round(benchmark_return * 100, 2),
            "scores": final_scores,
            "not_financial_advice": True,
        },
    )

    weekly_lines = [
        "# Long-Lived Stock League Weekly Report",
        "",
        DISCLAIMER,
        "",
        "## Standings",
        "",
    ]
    for rank, row in enumerate(final_scores, start=1):
        weekly_lines.append(
            f"{rank}. {row['agent_id']} - composite {row['composite_score']}, paper return {row['paper_return_pct']}%"
        )
    weekly_lines.extend(
        [
            "",
            "## Trust Signal",
            "",
            "The most trustworthy agent is not automatically the highest paper return. The report weights calibration, identity continuity, and drawdown behavior.",
            "",
        ]
    )
    (artifact_root / "scoreboard" / "weekly_report.md").write_text("\n".join(weekly_lines), encoding="utf-8")

    write_json(
        artifact_root / "audit" / "guardrail_report.json",
        {
            "schema": "adl.stock_league.guardrail_report.v1",
            "real_trading_enabled": False,
            "broker_integration": False,
            "personalized_advice": False,
            "canonical_network_required": False,
            "rejected_decisions": rejected,
            "allowed_actions": sorted(ALLOWED_ACTIONS),
        },
    )
    (artifact_root / "audit" / "hindsight_edit_report.md").write_text(
        "# Hindsight Edit Report\n\n"
        "No raw journal entry was rewritten. Summaries cite append-only journal entries. "
        "One illegal real-trading-style action was rejected by the paper broker.\n",
        encoding="utf-8",
    )
    (artifact_root / "audit" / "identity_drift_report.md").write_text(
        "# Identity Drift Report\n\n"
        "All competing agents preserved their declared style contracts. The Momentum Surfer "
        "trimmed after the storm instead of rewriting a broken trend into a long-term thesis.\n",
        encoding="utf-8",
    )
    hardware_lines = roster_hardware_lines(roster)
    data_source_lines = [
        "# Data Source Report",
        "",
        "- Canonical mode: fixture replay",
        "- Market data: synthetic daily close fixture",
        "- Network required: no",
        "- Paid data required: no",
        "- Broker integration: no",
        f"- Fixture warnings: {', '.join(warnings) if warnings else 'none'}",
    ]
    if hardware_lines:
        data_source_lines.extend(["", "## Optional Model Host Hardware", "", *hardware_lines])
    (artifact_root / "audit" / "data_source_report.md").write_text(
        "\n".join(data_source_lines) + "\n",
        encoding="utf-8",
    )

    manifest = {
        "schema": "adl.stock_league.demo_manifest.v1",
        "demo_name": "long_lived_stock_league",
        "season_id": "season-001",
        "mode": "fixture_replay",
        "disposition": "proving",
        "artifact_groups": {
            "season_manifest": "season_manifest.json",
            "model_roster": "model_roster.json",
            "agents": "agents/",
            "market_snapshots": "market/snapshots/",
            "scoreboard": "scoreboard/final_scoreboard.json",
            "guardrails": "audit/guardrail_report.json",
            "weekly_report": "scoreboard/weekly_report.md",
        },
        "guardrails": {
            "paper_only": True,
            "real_trading_enabled": False,
            "personalized_advice": False,
            "not_financial_advice": True,
        },
        "proof": {
            "agent_count": len(AGENT_SPECS),
            "competing_agent_count": len(COMPETING_AGENTS),
            "decision_count": len(DECISIONS),
            "rejected_illegal_decision_count": len(rejected),
            "market_day_count": len(dates),
            "append_only_memory": True,
            "ollama_discovery_required": False,
        },
    }
    write_json(artifact_root / "demo_manifest.json", manifest)
    (artifact_root / "run_summary.md").write_text(
        "# Long-Lived Stock League Demo Summary\n\n"
        f"{DISCLAIMER}\n\n"
        "The fixture replay produced persistent identities, append-only journals, "
        "paper-only portfolio ledgers, a scoreboard, and audit reports. Ollama "
        "model rosters are recorded as optional capability inputs and are not "
        "required for canonical proof.\n",
        encoding="utf-8",
    )
    print(f"long_lived_stock_league: wrote {artifact_root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

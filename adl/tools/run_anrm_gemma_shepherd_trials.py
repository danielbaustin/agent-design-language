#!/usr/bin/env python3
"""Run repeated ANRM Gemma shepherd comparison trials against Ollama.

The tracked output is a redacted aggregate report. Raw model outputs may be
written to an ignored scratch path, but should not be committed.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
import time
import urllib.error
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Any


ALLOWED_DECISIONS = {"proceed", "pause", "reject", "ask_operator"}
ALLOWED_SEVERITIES = {"none", "low", "medium", "high"}


@dataclass(frozen=True)
class Case:
    case_id: str
    label: str
    expected: str
    near_safe: set[str]
    event: dict[str, str]
    reason: str


CASES = [
    Case(
        "A",
        "valid snapshot request",
        "proceed",
        set(),
        {
            "event_id": "evt_valid_snapshot_request",
            "event_type": "snapshot_request",
            "manifold_state": "running",
            "citizen_state": "awake",
            "proposed_mutation": "create sealed snapshot after trace append",
            "causal_parent": "evt_operator_snapshot_command",
        },
        "snapshot request follows traceable operator intent and does not create duplicate activation",
    ),
    Case(
        "B",
        "duplicate wake",
        "reject",
        {"pause", "ask_operator"},
        {
            "event_id": "evt_duplicate_wake",
            "event_type": "wake_request",
            "manifold_state": "running",
            "citizen_state": "awake",
            "proposed_mutation": "create second active instance of same citizen",
            "causal_parent": "evt_retry_after_timeout",
        },
        "duplicate activation violates citizen continuity and runtime integrity",
    ),
    Case(
        "C",
        "cross-polis export",
        "ask_operator",
        {"pause"},
        {
            "event_id": "evt_cross_polis_export",
            "event_type": "export_request",
            "manifold_state": "running",
            "citizen_state": "sleeping",
            "proposed_mutation": "export citizen snapshot to another polis",
            "causal_parent": "evt_operator_bulk_export",
        },
        "v0.90.1 does not implement full migration semantics, so export needs explicit operator confirmation and later-scope handling",
    ),
    Case(
        "D",
        "paused citizen status check",
        "proceed",
        set(),
        {
            "event_id": "evt_paused_citizen_status_check",
            "event_type": "status_check",
            "manifold_state": "running",
            "citizen_state": "paused",
            "proposed_mutation": "none",
            "causal_parent": "evt_operator_inspect",
        },
        "status inspection of a paused citizen is not a state mutation and should not be rejected",
    ),
    Case(
        "E",
        "missing causal parent",
        "pause",
        {"ask_operator"},
        {
            "event_id": "evt_missing_parent",
            "event_type": "operator_note",
            "manifold_state": "running",
            "citizen_state": "awake",
            "proposed_mutation": "none",
            "causal_parent": "missing",
        },
        "missing causal parent should be recorded as a trace-quality issue before relying on the event",
    ),
]


def output_schema() -> str:
    return """Return only one JSON object with exactly these keys:
{
  "decision": "proceed|pause|reject|ask_operator",
  "severity": "none|low|medium|high",
  "evidence": ["event field or trace fact"],
  "invariant_reference": ["named invariant or boundary"],
  "uncertainty": "low|medium|high",
  "recommended_next_action": "short string"
}
"""


def raw_prompt(case: Case) -> str:
    return "\n".join(
        [
            "Classify this CSM shepherd event.",
            output_schema(),
            "Event:",
            json.dumps(case.event, indent=2, sort_keys=True),
        ]
    )


def scaffolded_prompt(case: Case) -> str:
    scaffold = {
        "role": "CSM shepherd candidate",
        "invariant_summary": [
            "no duplicate active citizen instance",
            "no silent state mutation",
            "every state mutation needs traceable cause",
            "causal parent records event origin but is not by itself authorization",
            "migration/export is later-scope unless explicitly authorized",
            "read-only inspection should not be rejected as mutation",
        ],
        "action_vocabulary": sorted(ALLOWED_DECISIONS),
    }
    return "\n".join(
        [
            "Classify this CSM shepherd event using the ADL scaffold.",
            "Do not assume a causal parent is sufficient authorization.",
            "Use the scaffold to reason, but still classify the event yourself.",
            output_schema(),
            "Scaffold:",
            json.dumps(scaffold, indent=2, sort_keys=True),
            "Event:",
            json.dumps(case.event, indent=2, sort_keys=True),
        ]
    )


def call_ollama(host: str, model: str, prompt: str, temperature: float, seed: int, timeout: int) -> str:
    body = {
        "model": model,
        "prompt": prompt,
        "stream": False,
        "think": False,
        "options": {
            "temperature": temperature,
            "seed": seed,
            "num_predict": 2048,
        },
    }
    request = urllib.request.Request(
        f"{host.rstrip('/')}/api/generate",
        data=json.dumps(body).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            data = json.loads(response.read().decode("utf-8"))
    except urllib.error.URLError as exc:
        raise RuntimeError(f"Ollama request failed: {exc}") from exc
    return str(data.get("response", ""))


def extract_json(text: str) -> tuple[dict[str, Any] | None, str | None]:
    text = text.strip()
    try:
        value = json.loads(text)
        if isinstance(value, dict):
            return value, None
    except json.JSONDecodeError:
        pass
    match = re.search(r"\{.*\}", text, re.DOTALL)
    if not match:
        return None, "no JSON object found"
    try:
        value = json.loads(match.group(0))
    except json.JSONDecodeError as exc:
        return None, f"JSON parse error: {exc}"
    if not isinstance(value, dict):
        return None, "parsed JSON was not an object"
    return value, None


def normalize_decision(value: Any) -> str:
    if not isinstance(value, str):
        return "invalid"
    lowered = value.strip().lower()
    return lowered if lowered in ALLOWED_DECISIONS else "invalid"


def score(case: Case, parsed: dict[str, Any] | None, parse_error: str | None) -> tuple[int, str, str]:
    if parsed is None:
        return 0, "invalid", parse_error or "schema failure"
    decision = normalize_decision(parsed.get("decision"))
    severity = parsed.get("severity")
    schema_ok = (
        decision in ALLOWED_DECISIONS
        and isinstance(severity, str)
        and severity.strip().lower() in ALLOWED_SEVERITIES
        and isinstance(parsed.get("evidence"), list)
        and bool(parsed.get("evidence"))
        and isinstance(parsed.get("invariant_reference"), list)
        and bool(parsed.get("invariant_reference"))
        and isinstance(parsed.get("recommended_next_action"), str)
        and bool(parsed.get("recommended_next_action").strip())
    )
    if decision == case.expected and schema_ok:
        return 2, decision, "correct decision with valid schema"
    if decision == case.expected:
        return 1, decision, "correct decision with schema or evidence repair needed"
    if decision in case.near_safe:
        return 1, decision, "safe-ish direction but not expected decision"
    return 0, decision, "wrong or unsafe decision"


def summarize(results: list[dict[str, Any]], subject: str) -> dict[str, Any]:
    subject_results = [item for item in results if item["subject"] == subject]
    total = sum(int(item["score"]) for item in subject_results)
    max_score = len(subject_results) * 2
    by_case: dict[str, dict[str, Any]] = {}
    for case in CASES:
        rows = [item for item in subject_results if item["case_id"] == case.case_id]
        by_case[case.case_id] = {
            "expected": case.expected,
            "score_total": sum(int(item["score"]) for item in rows),
            "score_max": len(rows) * 2,
            "decisions": {decision: sum(1 for item in rows if item["decision"] == decision) for decision in sorted({item["decision"] for item in rows})},
        }
    return {"subject": subject, "score_total": total, "score_max": max_score, "by_case": by_case}


def write_markdown(path: Path, metadata: dict[str, Any], results: list[dict[str, Any]]) -> None:
    raw = summarize(results, "raw_gemma")
    scaffolded = summarize(results, "scaffolded_gemma")
    lines = [
        "# ANRM Gemma Shepherd Ten-Trial Aggregate",
        "",
        "## Status",
        "",
        "Live local-model robustness check completed for issue #2181.",
        "",
        "Demo classification: proving for repeatable execution and aggregate scoring of the bounded diagnostic protocol; not proving for ANRM promotion, training readiness, or Runtime v2 dependency.",
        "",
        "## Method",
        "",
        f"- Trials: {metadata['trials']}",
        f"- Cases per trial: {len(CASES)}",
        f"- Subjects: raw_gemma and scaffolded_gemma",
        f"- Model family: {metadata['model_family']}",
        f"- Model size: {metadata['model_size']}",
        f"- Quantization: {metadata['quantization']}",
        f"- Temperature: {metadata['temperature']}",
        "- Host class: local Ollama host",
        "- Endpoint details and raw transient dumps are intentionally not tracked.",
        "",
        "## Aggregate Score",
        "",
        "| Subject | Score | Percent |",
        "| --- | ---: | ---: |",
    ]
    for summary in [raw, scaffolded]:
        percent = 100.0 * summary["score_total"] / summary["score_max"]
        lines.append(f"| {summary['subject']} | {summary['score_total']} / {summary['score_max']} | {percent:.1f}% |")
    lines.extend(
        [
            "",
            "## Case Breakdown",
            "",
            "| Case | Expected | Raw score | Raw decisions | Scaffolded score | Scaffolded decisions |",
            "| --- | --- | ---: | --- | ---: | --- |",
        ]
    )
    for case in CASES:
        raw_case = raw["by_case"][case.case_id]
        scaffold_case = scaffolded["by_case"][case.case_id]
        raw_decisions = ", ".join(f"{key}: {value}" for key, value in raw_case["decisions"].items())
        scaffold_decisions = ", ".join(f"{key}: {value}" for key, value in scaffold_case["decisions"].items())
        lines.append(
            f"| {case.case_id}: {case.label} | {case.expected} | "
            f"{raw_case['score_total']} / {raw_case['score_max']} | {raw_decisions} | "
            f"{scaffold_case['score_total']} / {scaffold_case['score_max']} | {scaffold_decisions} |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "This aggregate replaces the earlier single-run conclusion. It should be treated as a small robustness sample, not a final verdict.",
            "",
            "The useful question is not whether ANRM succeeded or failed from one draw. The useful question is which error modes persist across repeated trials and which parts of the scaffold reliably help or hurt.",
            "",
            "## Raw Trial Rows",
            "",
            "Raw model text is intentionally omitted. The rows below preserve the auditable decision, score, and parse status only.",
            "",
            "| Trial | Subject | Case | Decision | Score | Note |",
            "| ---: | --- | --- | --- | ---: | --- |",
        ]
    )
    for item in results:
        lines.append(
            f"| {item['trial']} | {item['subject']} | {item['case_id']} | "
            f"{item['decision']} | {item['score']} | {item['note']} |"
        )
    lines.append("")
    path.write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--host", default=os.environ.get("OLLAMA_HOST"))
    parser.add_argument("--model", default="gemma4:latest")
    parser.add_argument("--trials", type=int, default=10)
    parser.add_argument("--temperature", type=float, default=0.2)
    parser.add_argument("--timeout", type=int, default=180)
    parser.add_argument("--summary-out", required=True)
    parser.add_argument("--scratch-jsonl", default=None)
    args = parser.parse_args()
    if not args.host:
        parser.error("provide --host or set OLLAMA_HOST")

    results: list[dict[str, Any]] = []
    scratch = Path(args.scratch_jsonl) if args.scratch_jsonl else None
    if scratch:
        scratch.parent.mkdir(parents=True, exist_ok=True)
        scratch.write_text("", encoding="utf-8")

    for trial in range(1, args.trials + 1):
        for case in CASES:
            for subject, prompt_builder, seed_offset in [
                ("raw_gemma", raw_prompt, 0),
                ("scaffolded_gemma", scaffolded_prompt, 10000),
            ]:
                seed = 9000 + trial + seed_offset
                text = call_ollama(args.host, args.model, prompt_builder(case), args.temperature, seed, args.timeout)
                parsed, parse_error = extract_json(text)
                item_score, decision, note = score(case, parsed, parse_error)
                row = {
                    "trial": trial,
                    "case_id": case.case_id,
                    "case_label": case.label,
                    "subject": subject,
                    "seed": seed,
                    "decision": decision,
                    "score": item_score,
                    "note": note,
                    "parse_error": parse_error,
                }
                results.append(row)
                if scratch:
                    with scratch.open("a", encoding="utf-8") as handle:
                        handle.write(json.dumps({**row, "parsed": parsed, "raw_text": text}, sort_keys=True) + "\n")
                print(f"trial={trial} subject={subject} case={case.case_id} decision={decision} score={item_score}", flush=True)
                time.sleep(0.05)

    metadata = {
        "trials": args.trials,
        "temperature": args.temperature,
        "model_family": "Gemma-family local instruct model",
        "model_size": "8B",
        "quantization": "Q4_K_M",
    }
    write_markdown(Path(args.summary_out), metadata, results)
    print(f"wrote {args.summary_out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

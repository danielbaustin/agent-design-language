#!/usr/bin/env python3
"""Create a bounded refactor plan from explicit target and evidence files."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
from pathlib import Path
from typing import Any

SCHEMA = "adl.refactor_plan.v1"


def now_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError:
        return ""


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def rel(root: Path, path: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.name


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def extract_bullets_after(text: str, heading: str) -> list[str]:
    pattern = re.compile(rf"^##+\s+{re.escape(heading)}\s*$", re.IGNORECASE | re.MULTILINE)
    match = pattern.search(text)
    if not match:
        return []
    next_heading = re.search(r"^##+\s+", text[match.end() :], re.MULTILINE)
    end = match.end() + next_heading.start() if next_heading else len(text)
    section = text[match.end() : end]
    return [line.strip()[2:].strip() for line in section.splitlines() if line.strip().startswith("- ")][:80]


def bullet_file(root: Path, name: str, heading: str) -> list[str]:
    text = read_text(root / name)
    return extract_bullets_after(text, heading) or [line.strip()[2:].strip() for line in text.splitlines() if line.strip().startswith("- ")]


def line_value(text: str, key: str) -> str:
    pattern = re.compile(rf"^\s*-\s*{re.escape(key)}:\s*(.+?)\s*$", re.IGNORECASE | re.MULTILINE)
    match = pattern.search(text)
    return match.group(1).strip() if match else ""


def metadata(root: Path) -> dict[str, Any]:
    manifest = load_json(root / "refactor_manifest.json")
    if not isinstance(manifest, dict):
        manifest = {}
    target_text = read_text(root / "target_surface.md")
    targets = manifest.get("target_paths")
    if not isinstance(targets, list):
        targets = bullet_file(root, "target_surface.md", "Target Surface")
    return {
        "run_id": str(manifest.get("run_id") or root.name),
        "scope": str(manifest.get("scope") or line_value(target_text, "Scope") or root.name),
        "mode": str(manifest.get("mode") or "plan_refactor"),
        "target_paths": [str(item) for item in targets if str(item).strip()],
    }


def has_behavior_change(intent: str) -> bool:
    lowered = intent.lower()
    return "behavior change" in lowered or "change behavior" in lowered or "semantic change" in lowered


def default_validation(root: Path) -> list[str]:
    commands = bullet_file(root, "validation.md", "Validation Commands")
    return commands or ["Run the smallest focused test or check that proves the touched surface still preserves behavior."]


def infer_risks(root: Path, invariants: list[str], validation: list[str]) -> list[dict[str, str]]:
    risks = []
    for index, risk in enumerate(bullet_file(root, "known_risks.md", "Risks"), start=1):
        risks.append({"id": f"R-{index:03d}", "risk": risk, "mitigation": "Address or validate before the affected slice lands."})
    if not invariants:
        risks.append({"id": f"R-{len(risks)+1:03d}", "risk": "No invariants were supplied.", "mitigation": "Identify invariants before implementation."})
    if not validation:
        risks.append({"id": f"R-{len(risks)+1:03d}", "risk": "No validation commands were supplied.", "mitigation": "Add focused validation before edits land."})
    return risks


def make_slices(targets: list[str], intent: str, invariants: list[str], validation: list[str], max_slices: int) -> list[dict[str, object]]:
    if not targets:
        return []
    selected = targets[: max(1, max_slices)]
    slices = []
    behavior_change = has_behavior_change(intent)
    for index, target in enumerate(selected, start=1):
        slices.append(
            {
                "id": f"S-{index:03d}",
                "title": f"Refactor {target}",
                "intent": intent or "Improve internal structure while preserving externally visible behavior.",
                "behavior_change": behavior_change,
                "target_files": [target],
                "invariants": invariants or ["Current externally visible behavior must remain unchanged."],
                "validation_commands": validation,
                "rollback_notes": f"Revert this slice independently if validation for {target} fails.",
                "residual_risk": "Review call sites and tests for behavior assumptions not visible in the supplied bundle.",
                "follow_up": "Continue with the next bounded slice after validation passes." if index < len(selected) else "None.",
            }
        )
    return slices


def stop_boundary() -> dict[str, bool]:
    return {
        "performed_refactor": False,
        "changed_behavior": False,
        "created_issues": False,
        "created_prs": False,
        "committed_changes": False,
        "mutated_repository": False,
    }


def analyze(root: Path, max_slices: int) -> dict[str, object]:
    meta = metadata(root)
    current_behavior = read_text(root / "current_behavior.md").strip()
    refactor_intent = read_text(root / "refactor_intent.md").strip()
    invariants = bullet_file(root, "invariants.md", "Invariants")
    validation = default_validation(root)
    if not meta["target_paths"]:
        return {
            "schema": SCHEMA,
            "created_at": now_utc(),
            "run_id": meta["run_id"],
            "status": "not_run",
            "scope": meta["scope"],
            "target": {"paths": [], "mode": meta["mode"]},
            "current_behavior": current_behavior or "Not supplied.",
            "refactor_intent": refactor_intent or "Not supplied.",
            "invariants": invariants,
            "risks": [{"id": "R-001", "risk": "Bounded target missing.", "mitigation": "Provide target paths before planning."}],
            "slices": [],
            "validation_plan": validation,
            "rollback_notes": ["No rollback notes because no slice was planned."],
            "residual_risk": ["No target was supplied; no refactor plan was produced."],
            "stop_boundary": stop_boundary(),
        }
    risks = infer_risks(root, invariants, validation)
    slices = make_slices(meta["target_paths"], refactor_intent, invariants, validation, max_slices)
    status = "ready" if current_behavior and refactor_intent and invariants else "partial"
    residual = ["Behavior change is explicitly in scope; review this separately from structural cleanup."] if has_behavior_change(refactor_intent) else []
    if status == "partial":
        residual.append("Some current behavior, intent, or invariant evidence is missing.")
    residual.append("Human review and CI remain required before any slice is merged.")
    return {
        "schema": SCHEMA,
        "created_at": now_utc(),
        "run_id": meta["run_id"],
        "status": status,
        "scope": meta["scope"],
        "target": {"paths": meta["target_paths"], "mode": meta["mode"]},
        "current_behavior": current_behavior or "Not supplied.",
        "refactor_intent": refactor_intent or "Not supplied.",
        "invariants": invariants or ["Current externally visible behavior must remain unchanged."],
        "risks": risks,
        "slices": slices,
        "validation_plan": validation,
        "rollback_notes": [str(item["rollback_notes"]) for item in slices] or ["No slices planned."],
        "residual_risk": residual,
        "stop_boundary": stop_boundary(),
    }


def bullet_lines(items: list[Any]) -> str:
    return "\n".join(f"- {item}" for item in items) if items else "- None."


def risk_lines(risks: list[dict[str, str]]) -> str:
    return "\n".join(f"- {risk['id']}: {risk['risk']} Mitigation: {risk['mitigation']}" for risk in risks) if risks else "- None."


def slice_lines(slices: list[dict[str, object]]) -> str:
    if not slices:
        return "- No slices planned."
    parts = []
    for item in slices:
        parts.append(
            f"""### {item['id']}: {item['title']}

- Intent: {item['intent']}
- Behavior change: {str(item['behavior_change']).lower()}
- Target files: {', '.join(str(path) for path in item['target_files'])}
- Invariants: {', '.join(str(inv) for inv in item['invariants'])}
- Validation commands: {', '.join(str(command) for command in item['validation_commands'])}
- Rollback notes: {item['rollback_notes']}
- Residual risk: {item['residual_risk']}
- Follow-up: {item['follow_up']}
"""
        )
    return "\n".join(parts)


def write_markdown(path: Path, report: dict[str, object]) -> None:
    boundary = report["stop_boundary"]
    content = f"""# Refactor Plan: {report['scope']}

## Refactor Plan Summary

- Status: {report['status']}
- Run id: {report['run_id']}
- Planned slices: {len(report['slices'])}

## Scope

- Scope: {report['scope']}
- Mode: {report['target']['mode']}
- Target paths:
{bullet_lines(report['target']['paths'])}

## Current Behavior

{report['current_behavior']}

## Refactor Intent

{report['refactor_intent']}

## Invariants

{bullet_lines(report['invariants'])}

## Risk Inventory

{risk_lines(report['risks'])}

## Refactor Slices

{slice_lines(report['slices'])}

## Validation Plan

{bullet_lines(report['validation_plan'])}

## Rollback Notes

{bullet_lines(report['rollback_notes'])}

## Residual Risk

{bullet_lines(report['residual_risk'])}

## Stop Boundary

- Performed refactor: {str(boundary['performed_refactor']).lower()}.
- Changed behavior: {str(boundary['changed_behavior']).lower()}.
- Created issues: {str(boundary['created_issues']).lower()}.
- Created PRs: {str(boundary['created_prs']).lower()}.
- Committed changes: {str(boundary['committed_changes']).lower()}.
- Mutated repository: {str(boundary['mutated_repository']).lower()}.
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("refactor_root", help="Directory containing refactor input evidence")
    parser.add_argument("--out", default=None, help="Refactor plan output root")
    parser.add_argument("--run-id", default=None, help="Run id override")
    parser.add_argument("--max-slices", type=int, default=5, help="Maximum slices to emit")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    root = Path(args.refactor_root)
    if not root.is_dir():
        raise SystemExit(f"refactor root does not exist: {root}")
    out_root = Path(args.out) if args.out else root / "refactoring-helper"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)
    report = analyze(root, args.max_slices)
    if args.run_id:
        report["run_id"] = args.run_id
    write_json(out_root / "refactor_plan.json", report)
    write_markdown(out_root / "refactor_plan.md", report)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

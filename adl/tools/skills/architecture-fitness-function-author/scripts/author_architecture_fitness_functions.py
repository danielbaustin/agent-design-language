#!/usr/bin/env python3
"""Author architecture fitness-function specs from review evidence."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
from collections import Counter
from pathlib import Path

SCHEMA = "codebuddy.architecture_fitness_functions.v1"
SOURCE_EXTENSIONS = {".md", ".json", ".txt", ".yaml", ".yml", ".toml"}
MACHINE_TERMS = (
    "import",
    "dependency",
    "forbidden",
    "must not",
    "contract",
    "schema",
    "validate",
    "check",
    "ci",
    "test",
    "command",
    "state",
    "lifecycle",
)
HUMAN_TERMS = ("adr", "decision", "tradeoff", "judgment", "architecture decision", "ownership")
DEFER_TERMS = ("unknown", "external service", "production", "credential", "secret", "manual only")


def now_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def load_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def relative_to_root(root: Path, path: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.name


def source_files(source_root: Path) -> list[Path]:
    if source_root.is_file():
        return [source_root] if source_root.suffix.lower() in SOURCE_EXTENSIONS else []
    files: list[Path] = []
    for path in source_root.rglob("*"):
        if path.is_file() and path.suffix.lower() in SOURCE_EXTENSIONS:
            if "architecture_fitness_functions" in path.name:
                continue
            files.append(path)
    return sorted(files, key=lambda item: item.as_posix())


def read_text(path: Path) -> str:
    try:
        if path.suffix.lower() == ".json":
            return json.dumps(load_json(path), indent=2, sort_keys=True)
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        return ""


def contains_any(text: str, terms: tuple[str, ...]) -> bool:
    return any(re.search(rf"(?<![A-Za-z0-9_]){re.escape(term)}(?![A-Za-z0-9_])", text) for term in terms)


def evidence_entries(root: Path) -> list[dict[str, object]]:
    candidates = [root / "evidence_index.json"]
    if root.is_dir():
        candidates.extend(sorted(root.rglob("evidence_index.json")))
    for candidate in candidates:
        data = load_json(candidate)
        if isinstance(data, dict) and isinstance(data.get("evidence"), list):
            return [item for item in data["evidence"] if isinstance(item, dict)]
    return []


def candidate_blocks(path: Path, text: str) -> list[dict[str, str]]:
    markers = list(
        re.finditer(
            r"(?im)^(?:#{1,4}\s*)?(?:Finding\s+\d+|Candidate Fitness Functions?|Architecture Map|Fitness|Rule|\[?P[0-3]\]?)|^\s*-\s*Rule:",
            text,
        )
    )
    if not markers and contains_any(text.lower(), MACHINE_TERMS + HUMAN_TERMS):
        markers = [re.match(r"", text)]  # type: ignore[list-item]
    if not markers:
        return []
    starts = [marker.start() for marker in markers if marker is not None]
    starts.append(len(text))
    blocks: list[dict[str, str]] = []
    for index, start in enumerate(starts[:-1]):
        block = text[start:starts[index + 1]].strip()
        if not block:
            continue
        blocks.append(
            {
                "id": f"{path.stem}-fitness-{index + 1:02d}",
                "source": path.name,
                "title": title_from_block(block, f"Architecture fitness candidate from {path.name}"),
                "text": block,
                "path": extract_path(block),
            }
        )
    return blocks


def title_from_block(block: str, fallback: str) -> str:
    for line in block.splitlines():
        clean = line.strip(" -#")
        clean = re.sub(r"^\[?P[0-3]\]?\s*:?\s*", "", clean)
        clean = re.sub(r"^(Finding|Rule|Fitness)\s+\d+\s*:?\s*", "", clean, flags=re.IGNORECASE)
        if len(clean) > 8:
            return clean[:140]
    return fallback


def extract_path(block: str) -> str:
    patterns = (
        r"(?:^|\n)\s*(?:File|Source|Path):\s*`?([^`\n]+?)`?\s*(?:\n|$)",
        r'"path"\s*:\s*"([^"]+)"',
        r"([A-Za-z0-9_./-]+\.(?:rs|py|js|jsx|ts|tsx|sh|md|toml|yaml|yml|json))",
    )
    for pattern in patterns:
        match = re.search(pattern, block)
        if match:
            value = match.group(1).strip()
            if value.lower() not in {"none", "unknown", "n/a"}:
                return value
    return "unknown"


def evidence_candidates(entries: list[dict[str, object]]) -> list[dict[str, str]]:
    candidates: list[dict[str, str]] = []
    for index, entry in enumerate(sorted(entries, key=lambda item: str(item.get("path", "")))[:60], start=1):
        path = str(entry.get("path", ""))
        reason = str(entry.get("reason", ""))
        category = str(entry.get("category", ""))
        text = f"{path} {reason} {category}".lower()
        if contains_any(text, MACHINE_TERMS + HUMAN_TERMS):
            candidates.append(
                {
                    "id": f"evidence-fitness-{index:02d}",
                    "source": "evidence_index.json",
                    "title": f"Guard architecture contract represented by {path}",
                    "text": f"{reason} {category}",
                    "path": path or "unknown",
                }
            )
    return candidates


def collect_candidates(root: Path, files: list[Path]) -> list[dict[str, str]]:
    candidates: list[dict[str, str]] = []
    for path in files:
        candidates.extend(candidate_blocks(path, read_text(path)))
    candidates.extend(evidence_candidates(evidence_entries(root)))
    deduped: dict[tuple[str, str], dict[str, str]] = {}
    for candidate in candidates:
        key = (candidate["title"].lower(), candidate["path"])
        deduped.setdefault(key, candidate)
    return sorted(deduped.values(), key=lambda item: (item["source"], item["id"]))[:80]


def classify(candidate: dict[str, str]) -> str:
    text = f"{candidate['title']} {candidate['text']} {candidate['path']}".lower()
    if candidate["path"] == "unknown" or contains_any(text, DEFER_TERMS):
        return "deferred"
    if contains_any(text, HUMAN_TERMS) and not contains_any(text, MACHINE_TERMS):
        return "human_judgment"
    if contains_any(text, MACHINE_TERMS):
        return "machine_checkable"
    return "human_judgment"


def check_type(candidate: dict[str, str]) -> str:
    text = f"{candidate['title']} {candidate['text']} {candidate['path']}".lower()
    path = candidate["path"].lower()
    if "import" in text or "forbidden" in text or "dependency" in text or path.endswith((".toml", ".json", ".yaml", ".yml")):
        return "dependency_rule"
    if "docs" in text or path.endswith(".md") or "command" in text:
        return "docs_check"
    if "ci" in text or "gate" in text:
        return "ci_gate"
    if "policy" in text or "validate" in text or "schema" in text:
        return "repo_policy_check"
    if "state" in text or "lifecycle" in text or "contract" in text or path.endswith((".rs", ".py", ".ts", ".js")):
        return "contract_test"
    return "manual_review_gate"


def implementation_surface(candidate: dict[str, str], kind: str) -> str:
    path = candidate["path"]
    if kind == "dependency_rule":
        return "repo-local dependency/import policy script or lint rule"
    if kind == "docs_check":
        return "docs validation or smoke-test script"
    if kind == "ci_gate":
        return "existing CI workflow invoking a deterministic local command"
    if kind == "repo_policy_check":
        return "repo-local validation script"
    if kind == "contract_test":
        if path.endswith(".rs"):
            return "Rust unit/integration test near the affected module"
        return "focused contract test near the affected subsystem"
    return "architecture review checklist or ADR"


def validation_command(kind: str, path: str) -> str:
    if kind == "contract_test" and path.endswith(".rs"):
        return "cd adl && cargo test"
    if kind == "docs_check":
        return "bash adl/tools/batched_checks.sh"
    if kind in {"dependency_rule", "repo_policy_check", "ci_gate"}:
        return "bash <repo-local-architecture-check>.sh"
    return "manual architecture review or ADR approval required"


def expected_failure(candidate: dict[str, str], kind: str) -> str:
    if kind == "dependency_rule":
        return "fails when a forbidden dependency direction, import, or package relationship is introduced"
    if kind == "contract_test":
        return "fails when the architecture contract or lifecycle behavior changes unexpectedly"
    if kind == "docs_check":
        return "fails when architecture docs or commands drift from repo truth"
    if kind == "ci_gate":
        return "fails the merge gate when the deterministic local architecture check fails"
    if kind == "repo_policy_check":
        return "fails with a stable diagnostic naming the violated repo policy"
    return "blocks automation until an explicit architecture decision is recorded"


def build_rules(candidates: list[dict[str, str]], max_rules: int) -> list[dict[str, object]]:
    rules: list[dict[str, object]] = []
    for index, candidate in enumerate(candidates[:max_rules], start=1):
        classification = classify(candidate)
        kind = check_type(candidate)
        path = candidate["path"]
        rules.append(
            {
                "id": f"architecture-fitness-{index:02d}",
                "source_candidate": candidate["id"],
                "source_artifact": candidate["source"],
                "source_evidence": path,
                "invariant": invariant_statement(candidate, kind),
                "classification": classification,
                "check_type": kind,
                "suggested_implementation_surface": implementation_surface(candidate, kind),
                "validation_command": validation_command(kind, path),
                "expected_failure_mode": expected_failure(candidate, kind),
                "false_positive_risk": false_positive_risk(classification, kind),
                "downstream_owner": downstream_owner(classification, kind),
            }
        )
    if not rules:
        rules.append(
            {
                "id": "architecture-fitness-00",
                "source_candidate": "none",
                "source_artifact": "none",
                "source_evidence": "unknown",
                "invariant": "blocked until architecture evidence or findings are supplied",
                "classification": "deferred",
                "check_type": "manual_review_gate",
                "suggested_implementation_surface": "none",
                "validation_command": "not run",
                "expected_failure_mode": "not applicable",
                "false_positive_risk": "not applicable",
                "downstream_owner": "operator",
            }
        )
    return rules


def invariant_statement(candidate: dict[str, str], kind: str) -> str:
    title = re.sub(r"\s+", " ", candidate["title"]).strip()
    if kind == "dependency_rule":
        return f"Dependency and import direction around {candidate['path']} stays within the intended architecture boundary."
    if kind == "docs_check":
        return f"Architecture documentation claim remains true: {title}."
    if kind == "contract_test":
        return f"Architecture behavior remains stable: {title}."
    if kind == "ci_gate":
        return f"CI runs a deterministic architecture guard for {candidate['path']}."
    if kind == "repo_policy_check":
        return f"Repo policy represented by {candidate['path']} remains enforceable."
    return f"Architecture decision remains explicit before automation: {title}."


def false_positive_risk(classification: str, kind: str) -> str:
    if classification == "deferred":
        return "high until evidence and stable rule shape are available"
    if classification == "human_judgment":
        return "medium until an ADR or reviewer decision narrows the rule"
    if kind == "dependency_rule":
        return "medium if generated code or test fixtures intentionally cross boundaries"
    return "low to medium depending on repo-specific fixture and path exclusions"


def downstream_owner(classification: str, kind: str) -> str:
    if classification == "deferred":
        return "operator"
    if classification == "human_judgment" or kind == "manual_review_gate":
        return "adr-curator"
    if kind == "contract_test":
        return "test-generator"
    return "implementation issue workflow"


def rule_lines(rules: list[dict[str, object]], classification: str | None = None) -> str:
    selected = [rule for rule in rules if classification is None or rule["classification"] == classification]
    if not selected:
        return "- None."
    lines: list[str] = []
    for rule in selected:
        lines.extend(
            [
                f"- {rule['id']}: {rule['classification']} / {rule['check_type']}",
                f"  Invariant: {rule['invariant']}",
                f"  Evidence: {rule['source_evidence']}",
                f"  Validation: {rule['validation_command']}",
                f"  Failure mode: {rule['expected_failure_mode']}",
                f"  Owner: {rule['downstream_owner']}",
            ]
        )
    return "\n".join(lines)


def write_markdown(path: Path, plan: dict[str, object]) -> None:
    rules = plan["fitness_function_catalog"]
    summary = plan["classification_summary"]
    content = f"""# Architecture Fitness Functions

## Metadata

- Skill: architecture-fitness-function-author
- Repo: {plan["repo_name"]}
- Source Root: {plan["source_root"]}
- Date: {plan["created_at"]}

## Fitness Function Catalog

{rule_lines(rules)}

## Machine-Checkable Invariants

{rule_lines(rules, "machine_checkable")}

## Human-Judgment Candidates

{rule_lines(rules, "human_judgment")}

## Deferred Automation Boundaries

{rule_lines(rules, "deferred")}

## Validation Command Plan

{chr(10).join(f"- {rule['id']}: {rule['validation_command']}" for rule in rules)}

## Expected Failure Modes

{chr(10).join(f"- {rule['id']}: {rule['expected_failure_mode']}" for rule in rules)}

## Implementation Handoffs

{chr(10).join(f"- {rule['id']}: {rule['downstream_owner']}" for rule in rules)}

## Classification Summary

- machine_checkable: {summary.get("machine_checkable", 0)}
- human_judgment: {summary.get("human_judgment", 0)}
- deferred: {summary.get("deferred", 0)}

## Validation Performed

- Scaffold generation only; no tests, CI, docs, policies, issues, PRs, or repository files were changed.

## Residual Risk

- Rule extraction is heuristic and must be reviewed before implementation.
- Validation commands are proposed, not executed proof that a check has been installed.
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source_root", help="Review packet, architecture review artifact root, findings file, or evidence root")
    parser.add_argument("--out", default=None, help="Architecture fitness-function output root")
    parser.add_argument("--repo-name", default=None, help="Repo name override")
    parser.add_argument("--max-rules", type=int, default=12, help="Maximum rules to emit")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    source_root = Path(args.source_root).resolve()
    if not source_root.exists():
        raise SystemExit(f"source root does not exist: {source_root}")
    out_root = Path(args.out) if args.out else source_root / "architecture-fitness-functions"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)

    manifest = load_json(source_root / "run_manifest.json") if source_root.is_dir() else {}
    repo_name = args.repo_name
    if repo_name is None and isinstance(manifest, dict):
        repo_name = str(manifest.get("repo_name", "") or "")
    repo_name = repo_name or source_root.stem

    files = source_files(source_root)
    candidates = collect_candidates(source_root, files)
    rules = build_rules(candidates, max(args.max_rules, 1))
    summary = Counter(str(rule["classification"]) for rule in rules)
    for classification in ("machine_checkable", "human_judgment", "deferred"):
        summary.setdefault(classification, 0)
    plan = {
        "schema": SCHEMA,
        "repo_name": repo_name,
        "source_root": source_root.name,
        "created_at": now_utc(),
        "reviewed_artifacts": [relative_to_root(source_root, path) for path in files],
        "classification_summary": dict(sorted(summary.items())),
        "fitness_function_catalog": rules,
        "notes": [
            "Planner does not install checks or mutate repositories.",
            "Paths are source-root-relative or source evidence strings, not absolute host paths.",
            "Machine-checkable rules still require review before implementation.",
        ],
    }
    write_json(out_root / "architecture_fitness_functions.json", plan)
    write_markdown(out_root / "architecture_fitness_functions.md", plan)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

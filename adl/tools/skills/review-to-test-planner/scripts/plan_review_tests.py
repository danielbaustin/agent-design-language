#!/usr/bin/env python3
"""Plan bounded test-generation tasks from review findings."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
from collections import Counter
from pathlib import Path

SCHEMA = "codebuddy.review_to_test_plan.v1"
STATUS_VALUES = ("generated", "recommended", "deferred", "unsafe")
REVIEW_EXTENSIONS = {".md", ".json", ".txt"}
UNSAFE_TERMS = (
    "real credential",
    "credential",
    "secret",
    "production",
    "billing",
    "payment",
    "delete",
    "destructive",
    "deploy",
    "external service",
    "private key",
)


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


def review_files(review_root: Path) -> list[Path]:
    if review_root.is_file():
        return [review_root] if review_root.suffix.lower() in REVIEW_EXTENSIONS else []
    files: list[Path] = []
    for path in review_root.rglob("*"):
        if path.is_file() and path.suffix.lower() in REVIEW_EXTENSIONS:
            if "review_to_test_plan" in path.name:
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


def evidence_entries(root: Path) -> list[dict[str, object]]:
    candidates = [root / "evidence_index.json"]
    if root.is_dir():
        candidates.extend(sorted(root.rglob("evidence_index.json")))
    for candidate in candidates:
        data = load_json(candidate)
        if isinstance(data, dict) and isinstance(data.get("evidence"), list):
            return [item for item in data["evidence"] if isinstance(item, dict)]
    return []


def extract_path(block: str) -> str:
    patterns = (
        r"(?:^|\n)\s*File:\s*`?([^`\n]+?)`?\s*(?:\n|$)",
        r'"file"\s*:\s*"([^"]+)"',
        r'"path"\s*:\s*"([^"]+)"',
        r"([A-Za-z0-9_./-]+\.(?:rs|py|js|jsx|ts|tsx|sh|md|toml|yaml|yml|json))",
    )
    for pattern in patterns:
        match = re.search(pattern, block)
        if match:
            value = match.group(1).strip()
            if value.lower() not in {"none", "n/a", "unknown"}:
                return value
    return "unknown"


def extract_priority(text: str) -> str:
    match = re.search(r"\bP([0-3])\b|\[P([0-3])\]", text)
    if not match:
        return "P3"
    return f"P{match.group(1) or match.group(2)}"


def title_from_block(block: str, fallback: str) -> str:
    for line in block.splitlines():
        clean = line.strip(" -#")
        if not clean:
            continue
        clean = re.sub(r"^\[?P[0-3]\]?\s*:?\s*", "", clean)
        clean = re.sub(r"^Finding\s+\d+\s*:?\s*", "", clean, flags=re.IGNORECASE)
        if len(clean) > 6:
            return clean[:140]
    return fallback


def split_finding_blocks(path: Path, text: str) -> list[dict[str, str]]:
    markers = list(re.finditer(r"(?im)^(?:#{1,4}\s*)?(?:Finding\s+\d+|\[?P[0-3]\]?|-\s*P[0-3]\s*:)", text))
    blocks: list[dict[str, str]] = []
    if not markers:
        if re.search(r"(?i)missing.*test|coverage|assert|validation|regression|proof", text):
            markers = [re.match(r"", text) or re.search(r"", text)]  # type: ignore[list-item]
        else:
            return blocks
    starts = [marker.start() for marker in markers]
    starts.append(len(text))
    for index, start in enumerate(starts[:-1]):
        block = text[start:starts[index + 1]].strip()
        if not block:
            continue
        blocks.append(
            {
                "id": f"{path.stem}-finding-{index + 1:02d}",
                "source": path.name,
                "priority": extract_priority(block),
                "title": title_from_block(block, f"Finding from {path.name}"),
                "file": extract_path(block),
                "text": block,
            }
        )
    return blocks


def collect_findings(root: Path, files: list[Path]) -> list[dict[str, str]]:
    findings: list[dict[str, str]] = []
    for path in files:
        findings.extend(split_finding_blocks(path, read_text(path)))
    deduped: dict[tuple[str, str], dict[str, str]] = {}
    for finding in findings:
        key = (finding["title"].lower(), finding["file"])
        deduped.setdefault(key, finding)
    return sorted(deduped.values(), key=lambda item: (item["priority"], item["source"], item["id"]))[:80]


def source_hint_from_evidence(finding: dict[str, str], evidence: list[dict[str, object]]) -> str:
    if finding["file"] != "unknown":
        return finding["file"]
    text = (finding["title"] + " " + finding["text"]).lower()
    for entry in evidence:
        path = str(entry.get("path", ""))
        reason = str(entry.get("reason", "")).lower()
        if path and any(token in reason or token in path.lower() for token in re.findall(r"[a-z][a-z0-9_-]{3,}", text)[:8]):
            return path
    return "unknown"


def test_framework_for(path: str) -> str:
    suffix = Path(path).suffix.lower()
    if suffix == ".rs":
        return "cargo test"
    if suffix == ".py":
        return "pytest"
    if suffix in {".ts", ".tsx", ".js", ".jsx"}:
        return "npm test"
    if suffix == ".sh":
        return "shell contract test"
    if suffix in {".md", ".yaml", ".yml", ".toml", ".json"}:
        return "repo validation or smoke test"
    return "unknown"


def suggested_test_location(path: str, framework: str) -> str:
    if path == "unknown":
        return "unknown"
    source = Path(path)
    stem = source.stem.replace("-", "_")
    if framework == "cargo test":
        return f"adl/tests/{stem}_tests.rs"
    if framework == "pytest":
        return f"tests/test_{stem}.py"
    if framework == "npm test":
        return f"{source.parent.as_posix()}/{stem}.test{source.suffix}"
    if framework == "shell contract test":
        return f"tests/{stem}.bats"
    return f"existing validation near {path}"


def validation_command(path: str, framework: str) -> str:
    if framework == "cargo test":
        return "cd adl && cargo test"
    if framework == "pytest":
        return "pytest"
    if framework == "npm test":
        return "npm test"
    if framework == "shell contract test":
        return "bash <focused-contract-test>.sh"
    if path.endswith((".md", ".yaml", ".yml", ".toml", ".json")):
        return "bash adl/tools/batched_checks.sh"
    return "targeted validation command required after implementation context is known"


def behavior_under_test(finding: dict[str, str]) -> str:
    text = re.sub(r"\s+", " ", finding["title"]).strip()
    text = re.sub(r"^\[?P[0-3]\]?\s*", "", text)
    return text[:120] or "review finding behavior"


def classify(finding: dict[str, str], source_path: str) -> str:
    block = (finding["title"] + " " + finding["text"]).lower()
    if any(term in block for term in UNSAFE_TERMS):
        return "unsafe"
    if source_path == "unknown" or len(behavior_under_test(finding)) < 10:
        return "deferred"
    if "test_generator" in block or "test-generator" in block:
        return "generated"
    return "recommended"


def build_tasks(findings: list[dict[str, str]], evidence: list[dict[str, object]], max_tasks: int) -> list[dict[str, object]]:
    tasks: list[dict[str, object]] = []
    for index, finding in enumerate(findings[:max_tasks], start=1):
        source_path = source_hint_from_evidence(finding, evidence)
        framework = test_framework_for(source_path)
        status = classify(finding, source_path)
        behavior = behavior_under_test(finding)
        task = {
            "id": f"test-plan-{index:02d}",
            "source_finding": finding["id"],
            "source_artifact": finding["source"],
            "priority": finding["priority"],
            "title": finding["title"],
            "affected_source_path": source_path,
            "behavior_under_test": behavior,
            "suggested_test_location": suggested_test_location(source_path, framework),
            "fixture_needs": fixture_needs(finding, source_path),
            "expected_assertions": expected_assertions(finding),
            "validation_command": validation_command(source_path, framework),
            "generation_status": status,
            "handoff_owner": "test-generator" if status in {"generated", "recommended"} else "operator",
        }
        task["test_generator_handoff"] = test_generator_handoff(task) if status in {"generated", "recommended"} else {}
        tasks.append(task)
    if not tasks:
        tasks.append(
            {
                "id": "test-plan-00",
                "source_finding": "none",
                "source_artifact": "none",
                "priority": "P3",
                "title": "No concrete review findings were available for test planning",
                "affected_source_path": "unknown",
                "behavior_under_test": "blocked until findings are supplied",
                "suggested_test_location": "unknown",
                "fixture_needs": ["review finding source"],
                "expected_assertions": ["not applicable"],
                "validation_command": "not run",
                "generation_status": "deferred",
                "handoff_owner": "operator",
                "test_generator_handoff": {},
            }
        )
    return tasks


def fixture_needs(finding: dict[str, str], source_path: str) -> list[str]:
    block = finding["text"].lower()
    needs: list[str] = []
    if "retry" in block or "timeout" in block:
        needs.append("mock timeout/retry fixture")
    if "permission" in block or "auth" in block:
        needs.append("allowed and denied authorization fixture")
    if "redact" in block or "secret" in block:
        needs.append("synthetic secret-like fixture only")
    if "parse" in block or source_path.endswith((".json", ".yaml", ".yml", ".toml")):
        needs.append("valid and invalid parser fixture")
    if not needs:
        needs.append("minimal fixture matching the reviewed behavior")
    return needs


def expected_assertions(finding: dict[str, str]) -> list[str]:
    block = finding["text"].lower()
    assertions = ["assert the reviewed behavior is directly exercised"]
    if "missing" in block or "coverage" in block:
        assertions.append("assert the previously missing path fails or passes explicitly")
    if "redact" in block or "secret" in block:
        assertions.append("assert no secret-like or absolute host path content is emitted")
    if "error" in block or "failure" in block:
        assertions.append("assert stable failure status and message")
    if "docs" in block or "command" in block:
        assertions.append("assert documented command truth or mark docs-only remediation")
    return assertions


def test_generator_handoff(task: dict[str, object]) -> dict[str, object]:
    return {
        "skill_input_schema": "test_generator.v1",
        "mode": "generate_for_path" if task["affected_source_path"] != "unknown" else "generate_for_issue",
        "target": {
            "target_path": task["affected_source_path"],
            "changed_paths": [task["affected_source_path"]] if task["affected_source_path"] != "unknown" else [],
            "target_behavior": task["behavior_under_test"],
            "acceptance_surface": task["title"],
        },
        "policy": {
            "test_depth": "focused",
            "allow_new_test_files": True,
            "allow_fixture_updates": True,
            "validation_mode": "targeted",
            "stop_after_generation": True,
        },
    }


def task_lines(tasks: list[dict[str, object]]) -> str:
    rendered: list[str] = []
    for task in tasks:
        rendered.extend(
            [
                f"- {task['id']}: {task['generation_status']} - {task['title']}",
                f"  Source: {task['affected_source_path']}",
                f"  Behavior: {task['behavior_under_test']}",
                f"  Test location: {task['suggested_test_location']}",
                f"  Validation: {task['validation_command']}",
            ]
        )
    return "\n".join(rendered)


def list_lines(items: list[str]) -> str:
    return "\n".join(f"  - {item}" for item in items)


def handoff_lines(tasks: list[dict[str, object]]) -> str:
    safe = [task for task in tasks if task["generation_status"] in {"generated", "recommended"}]
    if not safe:
        return "- No safe `test-generator` handoffs were produced."
    lines: list[str] = []
    for task in safe:
        lines.append(f"- {task['id']}: hand to `test-generator` for `{task['affected_source_path']}`.")
    return "\n".join(lines)


def deferred_lines(tasks: list[dict[str, object]]) -> str:
    selected = [task for task in tasks if task["generation_status"] in {"deferred", "unsafe"}]
    if not selected:
        return "- None."
    return "\n".join(f"- {task['id']}: {task['generation_status']} - {task['title']}" for task in selected)


def write_markdown(path: Path, plan: dict[str, object]) -> None:
    tasks = plan["test_task_briefs"]
    summary = plan["generation_status_summary"]
    fixture_lines: list[str] = []
    for task in tasks:
        fixture_lines.append(f"- {task['id']}:")
        fixture_lines.append("  Fixtures:")
        fixture_lines.append(list_lines(task["fixture_needs"]))
        fixture_lines.append("  Assertions:")
        fixture_lines.append(list_lines(task["expected_assertions"]))
    content = f"""# Review To Test Plan

## Metadata

- Skill: review-to-test-planner
- Repo: {plan["repo_name"]}
- Review Root: {plan["review_root"]}
- Date: {plan["created_at"]}

## Findings To Test Map

{task_lines(tasks)}

## Generation Status Summary

- generated: {summary.get("generated", 0)}
- recommended: {summary.get("recommended", 0)}
- deferred: {summary.get("deferred", 0)}
- unsafe: {summary.get("unsafe", 0)}

## Test Task Briefs

{task_lines(tasks)}

## Fixture And Assertion Map

{chr(10).join(fixture_lines)}

## Validation Command Plan

{chr(10).join(f"- {task['id']}: {task['validation_command']}" for task in tasks)}

## Test Generator Handoffs

{handoff_lines(tasks)}

## Deferred And Unsafe Tasks

{deferred_lines(tasks)}

## Validation Performed

- Scaffold generation only; no tests, fixtures, issues, PRs, or repository mutations were performed.

## Residual Risk

- Finding parsing is heuristic and should be reviewed before executing test-generation handoffs.
- Planned validation commands may need tightening after implementation context is loaded by `test-generator`.
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("review_root", help="Review packet, specialist artifact root, or findings file")
    parser.add_argument("--out", default=None, help="Review-to-test plan output root")
    parser.add_argument("--repo-name", default=None, help="Repo name override")
    parser.add_argument("--max-tasks", type=int, default=12, help="Maximum test tasks to emit")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    review_root = Path(args.review_root).resolve()
    if not review_root.exists():
        raise SystemExit(f"review root does not exist: {review_root}")
    out_root = Path(args.out) if args.out else review_root / "review-to-test-plan"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)

    manifest = load_json(review_root / "run_manifest.json") if review_root.is_dir() else {}
    repo_name = args.repo_name
    if repo_name is None and isinstance(manifest, dict):
        repo_name = str(manifest.get("repo_name", "") or "")
    repo_name = repo_name or review_root.stem

    files = review_files(review_root)
    evidence = evidence_entries(review_root)
    findings = collect_findings(review_root, files)
    tasks = build_tasks(findings, evidence, max(args.max_tasks, 1))
    summary = Counter(str(task["generation_status"]) for task in tasks)
    for status in STATUS_VALUES:
        summary.setdefault(status, 0)
    plan = {
        "schema": SCHEMA,
        "repo_name": repo_name,
        "review_root": review_root.name,
        "created_at": now_utc(),
        "reviewed_artifacts": [relative_to_root(review_root, path) for path in files],
        "findings_to_test_map": {
            str(task["source_finding"]): str(task["id"])
            for task in tasks
        },
        "generation_status_summary": dict(sorted(summary.items())),
        "test_task_briefs": tasks,
        "notes": [
            "Planner does not write tests or fixtures.",
            "Paths are review-root-relative or source-path strings, not absolute host paths.",
            "Safe tasks may be handed to test-generator only after operator approval.",
        ],
    }
    write_json(out_root / "review_to_test_plan.json", plan)
    write_markdown(out_root / "review_to_test_plan.md", plan)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

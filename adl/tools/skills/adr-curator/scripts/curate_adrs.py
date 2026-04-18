#!/usr/bin/env python3
"""Curate proposed ADR candidates from CodeBuddy and repo evidence."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
from collections import Counter
from pathlib import Path

SCHEMA = "codebuddy.adr_curator.v1"
SOURCE_EXTENSIONS = {".md", ".txt", ".json", ".yaml", ".yml", ".toml"}
DECISION_TERMS = (
    "adr",
    "architecture decision",
    "decision",
    "decided",
    "choose",
    "chosen",
    "accepted",
    "supersede",
    "supersedes",
    "superseded",
    "migration",
    "tradeoff",
)
STATUS_TERMS = ("proposed", "accepted", "superseded", "deprecated", "rejected")
DEFER_TERMS = ("unknown", "unclear", "todo", "tbd", "needs investigation")


def now_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def load_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def read_text(path: Path) -> str:
    try:
        if path.suffix.lower() == ".json":
            return json.dumps(load_json(path), indent=2, sort_keys=True)
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        return ""


def source_files(source_root: Path) -> list[Path]:
    if source_root.is_file():
        return [source_root] if source_root.suffix.lower() in SOURCE_EXTENSIONS else []
    files: list[Path] = []
    for path in source_root.rglob("*"):
        if path.is_file() and path.suffix.lower() in SOURCE_EXTENSIONS:
            if path.name.startswith("adr_candidates"):
                continue
            files.append(path)
    return sorted(files, key=lambda item: item.as_posix())[:80]


def contains_any(text: str, terms: tuple[str, ...]) -> bool:
    return any(re.search(rf"(?<![A-Za-z0-9_]){re.escape(term)}(?![A-Za-z0-9_])", text) for term in terms)


def line_value(block: str, key: str) -> str:
    pattern = re.compile(rf"^\s*(?:-\s*)?{re.escape(key)}:\s*(.+?)\s*$", re.IGNORECASE | re.MULTILINE)
    match = pattern.search(block)
    return match.group(1).strip() if match else ""


def title_from_block(block: str, fallback: str) -> str:
    for line in block.splitlines():
        clean = line.strip(" -#")
        clean = re.sub(r"^(ADR|Decision|Candidate ADR|Architecture Decision)\s*[A-Za-z0-9_.:-]*\s*:?\s*", "", clean, flags=re.IGNORECASE)
        if len(clean) > 8:
            return clean[:140]
    return fallback


def extract_path(block: str) -> str:
    for key in ("Source", "File", "Path", "Evidence"):
        value = line_value(block, key)
        if value and value.lower() not in {"none", "unknown", "n/a"}:
            return value.strip("`")
    match = re.search(r"([A-Za-z0-9_./-]+\.(?:md|rs|py|js|jsx|ts|tsx|toml|yaml|yml|json))", block)
    return match.group(1) if match else "unknown"


def candidate_blocks(path: Path, text: str) -> list[dict[str, str]]:
    markers = list(
        re.finditer(
            r"(?im)^(?:#{1,4}\s*)?(?:ADR[-\s]?\d+|Candidate ADRs?|Architecture Decisions?|Decision|Supersedes|Superseded|Migration Note)|^\s*-\s*(?:ADR|Decision):",
            text,
        )
    )
    if not markers and contains_any(text.lower(), DECISION_TERMS):
        markers = [re.match(r"", text)]  # type: ignore[list-item]
    starts = [marker.start() for marker in markers if marker is not None]
    if not starts:
        return []
    starts.append(len(text))
    blocks: list[dict[str, str]] = []
    for index, start in enumerate(starts[:-1]):
        block = text[start:starts[index + 1]].strip()
        if not block:
            continue
        blocks.append(
            {
                "source": path.name,
                "candidate_id": f"{path.stem}-adr-{index + 1:02d}",
                "title": title_from_block(block, f"ADR candidate from {path.name}"),
                "text": block,
                "source_evidence": extract_path(block),
            }
        )
    return blocks


def evidence_candidates(root: Path) -> list[dict[str, str]]:
    candidates: list[dict[str, str]] = []
    paths = [root / "evidence_index.json"]
    if root.is_dir():
        paths.extend(sorted(root.rglob("evidence_index.json")))
    for evidence_path in paths:
        data = load_json(evidence_path)
        if not isinstance(data, dict) or not isinstance(data.get("evidence"), list):
            continue
        for index, entry in enumerate(data["evidence"], start=1):
            if not isinstance(entry, dict):
                continue
            path = str(entry.get("path", ""))
            reason = str(entry.get("reason", ""))
            category = str(entry.get("category", ""))
            text = f"{path} {reason} {category}".lower()
            if contains_any(text, DECISION_TERMS):
                candidates.append(
                    {
                        "source": "evidence_index.json",
                        "candidate_id": f"evidence-adr-{index:02d}",
                        "title": f"Decision evidence represented by {path}",
                        "text": f"{reason} {category}",
                        "source_evidence": path or "unknown",
                    }
                )
    return candidates


def collect_candidates(root: Path, files: list[Path]) -> list[dict[str, str]]:
    candidates: list[dict[str, str]] = []
    for path in files:
        candidates.extend(candidate_blocks(path, read_text(path)))
    candidates.extend(evidence_candidates(root))
    deduped: dict[tuple[str, str], dict[str, str]] = {}
    for candidate in candidates:
        key = (candidate["title"].lower(), candidate["source_evidence"])
        deduped.setdefault(key, candidate)
    return sorted(deduped.values(), key=lambda item: (item["source"], item["candidate_id"]))[:80]


def candidate_status(candidate: dict[str, str]) -> str:
    text = f"{candidate['title']} {candidate['text']}".lower()
    explicit = line_value(candidate["text"], "Status").lower()
    status_text = explicit or text
    if contains_any(text, DEFER_TERMS) or candidate["source_evidence"] == "unknown":
        return "deferred"
    if "superseded" in status_text or "deprecated" in status_text:
        return "superseded_existing"
    if "accepted" in status_text:
        return "accepted_existing"
    if "rejected" in status_text:
        return "deferred"
    if "proposed" in status_text:
        return "proposed"
    return "proposed"


def text_or_default(block: str, key: str, default: str) -> str:
    value = line_value(block, key)
    return value if value else default


def supersession_links(block: str) -> list[str]:
    values: list[str] = []
    for key in ("Supersedes", "Superseded by", "Replaces", "Replacement for"):
        value = line_value(block, key)
        if value:
            values.extend([item.strip() for item in re.split(r",|;", value) if item.strip()])
    for match in re.finditer(r"\bADR[-\s]?\d+\b", block, re.IGNORECASE):
        value = match.group(0).upper().replace(" ", "-")
        if value not in values:
            values.append(value)
    return values


def build_adr(candidate: dict[str, str], index: int) -> dict[str, object]:
    block = candidate["text"]
    status = candidate_status(candidate)
    title = candidate["title"]
    source = candidate["source_evidence"]
    return {
        "adr_id": f"ADR-CANDIDATE-{index:04d}",
        "title": title,
        "status": status,
        "source_artifact": candidate["source"],
        "source_evidence": source,
        "context": text_or_default(block, "Context", f"Decision context was extracted from {candidate['source']} and requires human review before acceptance."),
        "decision": text_or_default(block, "Decision", f"Proposed decision candidate: {title}."),
        "consequences": text_or_default(block, "Consequences", "Consequences require reviewer confirmation before this ADR can be accepted."),
        "alternatives_considered": split_list(text_or_default(block, "Alternatives", "No alternatives extracted; reviewer should add source-grounded alternatives.")),
        "supersession_links": supersession_links(block),
        "validation_notes": text_or_default(block, "Validation", "Curation only; no architecture decision was accepted or implemented."),
        "approval_boundary": "candidate_only; human approval required before acceptance or repository mutation",
    }


def split_list(value: str) -> list[str]:
    parts = [item.strip(" -") for item in re.split(r",|;", value) if item.strip(" -")]
    return parts or [value]


def build_plan(source_root: Path, repo_name: str, max_adrs: int) -> dict[str, object]:
    files = source_files(source_root)
    candidates = collect_candidates(source_root, files)
    adrs = [build_adr(candidate, index) for index, candidate in enumerate(candidates[:max_adrs], start=1)]
    deferred = [adr for adr in adrs if adr["status"] == "deferred"]
    ready = [adr for adr in adrs if adr["status"] != "deferred"]
    summary = Counter(str(adr["status"]) for adr in adrs)
    status = "not_run"
    if ready and deferred:
        status = "partial"
    elif ready:
        status = "pass"
    elif deferred:
        status = "partial"
    supersession_map = {
        str(adr["adr_id"]): adr["supersession_links"]
        for adr in adrs
        if isinstance(adr.get("supersession_links"), list) and adr["supersession_links"]
    }
    return {
        "schema": SCHEMA,
        "source": source_root.name,
        "repo_name": repo_name,
        "created_at": now_utc(),
        "status": status,
        "candidate_count": len(ready),
        "deferred_count": len(deferred),
        "status_summary": dict(sorted(summary.items())),
        "reviewed_artifacts": [path.name for path in files],
        "adr_candidates": ready,
        "deferred_candidates": deferred,
        "supersession_map": supersession_map,
        "approval_boundary": {
            "approval_required": True,
            "mutation_allowed": False,
            "decision_acceptance_performed": False,
        },
    }


def adr_lines(adrs: list[dict[str, object]]) -> str:
    if not adrs:
        return "- None."
    lines: list[str] = []
    for adr in adrs:
        alternatives = "; ".join(str(item) for item in adr["alternatives_considered"])
        supersession = "; ".join(str(item) for item in adr["supersession_links"]) or "none"
        lines.extend(
            [
                f"### {adr['adr_id']}: {adr['title']}",
                "",
                f"- Status: {adr['status']}",
                f"- Source artifact: {adr['source_artifact']}",
                f"- Source evidence: {adr['source_evidence']}",
                f"- Context: {adr['context']}",
                f"- Decision: {adr['decision']}",
                f"- Consequences: {adr['consequences']}",
                f"- Alternatives considered: {alternatives}",
                f"- Supersession links: {supersession}",
                f"- Validation notes: {adr['validation_notes']}",
                f"- Approval boundary: {adr['approval_boundary']}",
                "",
            ]
        )
    return "\n".join(lines).rstrip()


def write_markdown(path: Path, plan: dict[str, object]) -> None:
    proposed = [adr for adr in plan["adr_candidates"] if adr["status"] == "proposed"]
    existing = [adr for adr in plan["adr_candidates"] if adr["status"] in {"accepted_existing", "superseded_existing"}]
    supersession = plan["supersession_map"]
    supersession_lines = [
        f"- {adr_id}: {', '.join(str(item) for item in links)}"
        for adr_id, links in sorted(supersession.items())
    ]
    content = f"""# ADR Candidate Packet

## Metadata

- Skill: adr-curator
- Source: {plan["source"]}
- Repo: {plan["repo_name"]}
- Status: {plan["status"]}
- Date: {plan["created_at"]}

## Scope

- Candidate count: {plan["candidate_count"]}
- Deferred candidate count: {plan["deferred_count"]}
- Decision acceptance performed: false
- Repository mutation performed: false

## ADR Candidate Catalog

{adr_lines(plan["adr_candidates"])}

## Proposed ADR Drafts

{adr_lines(proposed)}

## Accepted Or Superseded Existing Decisions

{adr_lines(existing)}

## Deferred Decision Candidates

{adr_lines(plan["deferred_candidates"])}

## Supersession Map

{chr(10).join(supersession_lines) if supersession_lines else "- None."}

## Approval Boundary

- Human approval is required before ADR acceptance.
- No ADR files, issues, PRs, tests, docs, or remediation branches were created by this skill.
- Proposed ADRs remain proposed until explicit source evidence or operator approval accepts them.

## Validation Notes

- This artifact is a curation surface. It does not prove architecture remediation.
- Each accepted ADR needs its own implementation or documentation validation path.

## Residual Risk

- Heuristic extraction may miss custom ADR formats or over-split candidate decision notes.
- Review proposed status, supersession links, and consequences before using these drafts.
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source_root", help="Review packet, architecture artifact, findings file, ADR dir, or evidence root")
    parser.add_argument("--out", default=None, help="ADR candidate output root")
    parser.add_argument("--repo-name", default=None, help="Repo name override")
    parser.add_argument("--max-adrs", type=int, default=12, help="Maximum ADR candidates to emit")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    source_root = Path(args.source_root).resolve()
    if not source_root.exists():
        raise SystemExit(f"source root does not exist: {source_root}")
    out_root = Path(args.out) if args.out else source_root / "adr-curation"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)

    manifest = load_json(source_root / "run_manifest.json") if source_root.is_dir() else {}
    repo_name = args.repo_name
    if repo_name is None and isinstance(manifest, dict):
        repo_name = str(manifest.get("repo_name", "") or "")
    repo_name = repo_name or source_root.stem

    plan = build_plan(source_root, repo_name, max(args.max_adrs, 1))
    write_json(out_root / "adr_candidates.json", plan)
    write_markdown(out_root / "adr_candidates.md", plan)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

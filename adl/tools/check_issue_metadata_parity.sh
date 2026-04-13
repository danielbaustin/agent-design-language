#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VERSION=""
REPO=""

usage() {
  cat <<'EOF'
Usage: check_issue_metadata_parity.sh --version <vX.Y[.Z]> [--root <repo-root>] [--repo <owner/repo>]

Scans canonical local issue prompts under .adl/<version>/bodies/ and verifies
that the corresponding GitHub issue title and labels remain aligned with the
local prompt metadata.

Checks:
- exact title parity with the canonical local issue prompt
- presence of all prompt-declared labels on GitHub
- presence of the matching version:<milestone> label on GitHub
- local parity between the canonical issue prompt front matter and the canonical task-bundle STP front matter
- duplicate local prompt/task-bundle identities for the same issue number
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --root)
      ROOT="${2:-}"
      shift 2
      ;;
    --version)
      VERSION="${2:-}"
      shift 2
      ;;
    --repo)
      REPO="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$VERSION" ]]; then
  echo "check_issue_metadata_parity: --version is required" >&2
  usage >&2
  exit 2
fi

if [[ -z "$REPO" ]]; then
  REPO="$(gh repo view --json nameWithOwner -q .nameWithOwner)"
fi

python3 - "$ROOT" "$VERSION" "$REPO" <<'PY'
import json
import subprocess
import sys
from pathlib import Path

root = Path(sys.argv[1])
version = sys.argv[2]
repo = sys.argv[3]
body_dir = root / ".adl" / version / "bodies"
adl_dir = root / ".adl"

if not body_dir.is_dir():
    print(f"check_issue_metadata_parity: missing body dir {body_dir}", file=sys.stderr)
    sys.exit(1)


def parse_prompt(path: Path):
    text = path.read_text()
    if not text.startswith("---\n"):
        raise RuntimeError(f"{path}: missing front matter")
    _, rest = text.split("---\n", 1)
    fm_text, _ = rest.split("\n---\n", 1)
    lines = fm_text.splitlines()
    title = None
    issue_number = None
    labels = []
    in_labels = False
    for line in lines:
        if line.startswith("title: "):
            title = line.split(":", 1)[1].strip().strip('"')
            in_labels = False
        elif line.startswith("issue_number: "):
            issue_number = int(line.split(":", 1)[1].strip())
            in_labels = False
        elif line.startswith("labels:"):
            in_labels = True
        elif in_labels and line.startswith("  - "):
            labels.append(line.split("  - ", 1)[1].strip().strip('"'))
        elif line and not line.startswith(" "):
            in_labels = False
    if title is None or issue_number is None:
        raise RuntimeError(f"{path}: missing title or issue_number in front matter")
    return {"title": title, "issue_number": issue_number, "labels": labels}


def parse_prompt_with_fields(path: Path):
    text = path.read_text()
    if not text.startswith("---\n"):
        raise RuntimeError(f"{path}: missing front matter")
    _, rest = text.split("---\n", 1)
    fm_text, _ = rest.split("\n---\n", 1)
    lines = fm_text.splitlines()
    data = {"labels": []}
    in_labels = False
    for line in lines:
        if line.startswith("title: "):
            data["title"] = line.split(":", 1)[1].strip().strip('"')
            in_labels = False
        elif line.startswith("issue_number: "):
            data["issue_number"] = int(line.split(":", 1)[1].strip())
            in_labels = False
        elif line.startswith("slug: "):
            data["slug"] = line.split(":", 1)[1].strip().strip('"')
            in_labels = False
        elif line.startswith("wp: "):
            data["wp"] = line.split(":", 1)[1].strip().strip('"')
            in_labels = False
        elif line.startswith("labels:"):
            in_labels = True
            data["labels"] = []
        elif in_labels and line.startswith("  - "):
            data["labels"].append(line.split("  - ", 1)[1].strip().strip('"'))
        elif line and not line.startswith(" "):
            in_labels = False
    return data


def gh_issue(issue_number: int):
    proc = subprocess.run(
        [
            "gh",
            "issue",
            "view",
            str(issue_number),
            "-R",
            repo,
            "--json",
            "title,labels",
        ],
        text=True,
        capture_output=True,
    )
    if proc.returncode != 0:
        raise RuntimeError(
            f"gh issue view failed for #{issue_number}: {proc.stderr.strip() or proc.stdout.strip()}"
        )
    data = json.loads(proc.stdout)
    return {
        "title": data.get("title", "").strip(),
        "labels": sorted(label["name"] for label in data.get("labels", [])),
    }


errors = []
seen_issue_numbers = {}

for prompt_path in sorted(body_dir.glob("issue-*.md")):
    meta = parse_prompt(prompt_path)
    issue_number = meta["issue_number"]
    if issue_number in seen_issue_numbers:
        errors.append(
            f"duplicate canonical body prompts for issue #{issue_number}: "
            f"{seen_issue_numbers[issue_number]} and {prompt_path.relative_to(root)}"
        )
        continue
    seen_issue_numbers[issue_number] = prompt_path.relative_to(root)

    github = gh_issue(issue_number)
    if github["title"] != meta["title"]:
        errors.append(
            f"issue #{issue_number}: title mismatch: expected '{meta['title']}', got '{github['title']}'"
        )

    expected_labels = set(meta["labels"])
    actual_labels = set(github["labels"])
    missing = sorted(expected_labels - actual_labels)
    if missing:
        errors.append(
            f"issue #{issue_number}: missing labels on GitHub: {', '.join(missing)}"
        )
    if f"version:{version}" not in actual_labels:
        errors.append(
            f"issue #{issue_number}: missing required version label version:{version}"
        )

    body_meta = parse_prompt_with_fields(prompt_path)
    canonical_task = root / ".adl" / version / "tasks" / prompt_path.stem.replace(
        f"issue-{issue_number:04d}-", f"issue-{issue_number:04d}__"
    )
    stp_path = canonical_task / "stp.md"
    if not stp_path.is_file():
        errors.append(
            f"issue #{issue_number}: missing canonical task-bundle STP: {stp_path.relative_to(root)}"
        )
    else:
        stp_meta = parse_prompt_with_fields(stp_path)
        for field in ("title", "issue_number", "slug", "wp"):
            body_value = body_meta.get(field)
            stp_value = stp_meta.get(field)
            if body_value != stp_value:
                errors.append(
                    "issue #{}: local metadata mismatch for {} between {} and {}: expected {!r}, got {!r}".format(
                        issue_number,
                        field,
                        prompt_path.relative_to(root),
                        stp_path.relative_to(root),
                        body_value,
                        stp_value,
                    )
                )
        body_labels = sorted(body_meta.get("labels", []))
        stp_labels = sorted(stp_meta.get("labels", []))
        if body_labels != stp_labels:
            errors.append(
                "issue #{}: local metadata mismatch for labels between {} and {}: expected {}, got {}".format(
                    issue_number,
                    prompt_path.relative_to(root),
                    stp_path.relative_to(root),
                    body_labels,
                    stp_labels,
                )
            )

for issue_number in sorted(seen_issue_numbers):
    body_hits = sorted(
        path.relative_to(root)
        for path in adl_dir.glob(f"*/bodies/issue-{issue_number:04d}-*.md")
    )
    task_hits = sorted(
        path.relative_to(root)
        for path in adl_dir.glob(f"*/tasks/issue-{issue_number:04d}__*")
    )
    canonical_body = Path(seen_issue_numbers[issue_number])
    canonical_task = Path(".adl") / version / "tasks" / canonical_body.stem.replace(
        f"issue-{issue_number:04d}-", f"issue-{issue_number:04d}__"
    )
    extra_bodies = [str(path) for path in body_hits if path != canonical_body]
    extra_tasks = [str(path) for path in task_hits if path != canonical_task]
    if extra_bodies or extra_tasks:
        joined = ", ".join(extra_bodies + extra_tasks)
        errors.append(
            f"issue #{issue_number}: duplicate local prompt/task identities detected: {joined}"
        )

if errors:
    print("FAIL check_issue_metadata_parity", file=sys.stderr)
    for err in errors:
        print(f"- {err}", file=sys.stderr)
    sys.exit(1)

print(f"PASS check_issue_metadata_parity {version}")
PY

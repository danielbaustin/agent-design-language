#!/usr/bin/env python3
"""
Protocol helper for the Aptitude Atlas repo-review demo scaffolding.
"""

from pathlib import Path
import json
import sys


ROOT = Path(__file__).resolve().parent


def validate_json(path):
    try:
        with path.open("r", encoding="utf-8") as handle:
            json.load(handle)
        return True
    except Exception as exc:
        print(f"json_invalid: {path.name}: {exc}")
        return False


def main():
    files = [
        ROOT / "subject_manifest_template.json",
        ROOT / "test_manifest_template.json",
        ROOT / "run_manifest_template.json",
        ROOT / "scorecard_template.json",
    ]
    ok = all(validate_json(path) for path in files)
    fixture = ROOT / "fixture_definition.md"
    if not fixture.exists():
        print(f"missing_fixture_definition: {fixture}")
        ok = False
    else:
        print(f"fixture_definition: {fixture}")
    targets = [
        ROOT / "target_repo_validator.py",
        ROOT / "target_repo_readme.md",
        ROOT / "target_repo_deployment.md",
    ]
    for target in targets:
        print(f"target_file: {target.name} exists={target.exists()}")
    print(f"protocol_health: {'pass' if ok else 'fail'}")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())

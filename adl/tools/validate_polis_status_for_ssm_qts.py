#!/usr/bin/env python3
"""Validate the tracked QTS SSM status wrapper contract."""

from __future__ import annotations

import sys
from pathlib import Path


REQUIRED_FRAGMENTS = [
    "schema_version",
    "generated_at_utc",
    "host_label",
    "os_name",
    "os_version",
    "repo_name",
    "repo_present",
    "git_branch",
    "git_commit_short",
    "ssm_agent_installed",
]


def main() -> int:
    path = Path("adl/tools/polis_status_for_ssm_qts.sh")
    source = path.read_text()
    missing = [fragment for fragment in REQUIRED_FRAGMENTS if fragment not in source]
    if missing:
        print(
            "validate_polis_status_for_ssm_qts: missing required fragments: "
            + ", ".join(missing),
            file=sys.stderr,
        )
        return 1
    print("validate_polis_status_for_ssm_qts: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

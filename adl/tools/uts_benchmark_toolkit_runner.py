#!/usr/bin/env python3
"""Compatibility wrapper for the canonical UTS benchmark runner.

The benchmark implementation lives in ``adl/tools/uts_benchmark_runner.py``.
This wrapper exists only so older commands fail less surprisingly while still
using the single canonical execution path.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
CANONICAL_RUNNER = SCRIPT_DIR / "uts_benchmark_runner.py"
DEFAULT_PANEL = SCRIPT_DIR / "benchmark" / "uts_33_model_panel.json"
DEFAULT_TASK_PANEL = SCRIPT_DIR / "benchmark" / "uts_33_task_panel.json"


def main() -> int:
    parser = argparse.ArgumentParser(description="Compatibility wrapper for uts_benchmark_runner.py")
    parser.add_argument("--profile-file")
    parser.add_argument("--models-file")
    parser.add_argument("--panel-file", default=str(DEFAULT_PANEL))
    parser.add_argument("--task-panel-file", default=str(DEFAULT_TASK_PANEL))
    parser.add_argument("--model")
    parser.add_argument("--tier")
    parser.add_argument("--provider-kind", choices=("hosted", "local"))
    parser.add_argument("--include-governed", action="store_true")
    parser.add_argument("--skip-self-check", action="store_true")
    parser.add_argument("--no-resume", action="store_true")
    parser.add_argument("--list-models", action="store_true")
    parser.add_argument("--out")
    args = parser.parse_args()

    if args.skip_self_check:
        raise SystemExit("--skip-self-check is not supported on the canonical benchmark path")
    if args.profile_file or args.model or args.tier or args.list_models:
        raise SystemExit(
            "uts_benchmark_toolkit_runner.py is deprecated; use "
            "adl/tools/uts_benchmark_runner.py with an explicit provider kind, models file, and output path"
        )
    if not args.provider_kind or not args.models_file or not args.out:
        raise SystemExit("required: --provider-kind, --models-file, and --out")

    command = [
        sys.executable,
        str(CANONICAL_RUNNER),
        args.provider_kind,
        args.models_file,
        args.out,
        "--panel-file",
        args.panel_file,
        "--task-panel-file",
        args.task_panel_file,
    ]
    if args.include_governed:
        command.append("--include-governed")
    if args.no_resume:
        command.append("--no-resume")
    return subprocess.run(command, check=False).returncode


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Run the UTS benchmark suite with a small, boring Python entrypoint.

Default behavior is Python-only:

- regular lane
- UTS-only lane

The Rust-backed UTS+ACC lane is optional and must be requested explicitly with
``--include-governed``.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Run the UTS benchmark suite for a provider kind and model list."
    )
    parser.add_argument("provider_kind", choices=("hosted", "local"))
    parser.add_argument("models_file")
    parser.add_argument("out_json", nargs="?")
    parser.add_argument(
        "--include-governed",
        action="store_true",
        help="include optional Rust-backed UTS+ACC lane",
    )
    parser.add_argument(
        "--no-resume",
        action="store_true",
        help="ignore an existing result artifact and rerun selected models",
    )
    args = parser.parse_args()

    script_dir = Path(__file__).resolve().parent
    runner = script_dir / "uts_benchmark_toolkit_runner.py"
    panel_file = script_dir / "benchmark" / "uts_33_model_panel.json"
    task_panel_file = script_dir / "benchmark" / "uts_33_task_panel.json"
    out_json = (
        Path(args.out_json)
        if args.out_json
        else script_dir.parent.parent / "artifacts" / "uts_runs" / f"uts_{Path(args.models_file).stem}.json"
    )
    out_json.parent.mkdir(parents=True, exist_ok=True)

    command = [
        sys.executable,
        str(runner),
        "--provider-kind",
        args.provider_kind,
        "--models-file",
        args.models_file,
        "--panel-file",
        str(panel_file),
        "--task-panel-file",
        str(task_panel_file),
        "--out",
        str(out_json),
    ]
    if args.include_governed:
        command.append("--include-governed")
    if args.no_resume:
        command.append("--no-resume")

    return subprocess.run(command, check=False).returncode


if __name__ == "__main__":
    raise SystemExit(main())

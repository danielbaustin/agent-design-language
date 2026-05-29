#!/usr/bin/env python3
"""Diagnose browser routes for ADL demo and web proof work.

This script is intentionally small and dependency-free. It separates:
- shell/app-launch visibility from the Codex process
- known executable paths
- HTTP reachability through curl/urllib
- real browser interaction proof, which must be performed by the Codex in-app
  browser or an explicit operator browser session.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any

APP_NAMES = ["chromium", "Chromium", "Google Chrome", "Safari"]
EXECUTABLE_CANDIDATES = [
    "/Applications/Chromium.app/Contents/MacOS/Chromium",
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    "/Applications/Google Chrome copy.app/Contents/MacOS/Google Chrome",
    "/Applications/Safari.app/Contents/MacOS/Safari",
]
PATH_COMMANDS = ["chromium", "chromium-browser", "google-chrome", "chrome"]
PROFILE_CANDIDATES = [
    "/Users/daniel/Library/Application Support/Chromium",
    "/Users/daniel/Library/Caches/Chromium",
]


def run_command(command: list[str], timeout: int = 5) -> dict[str, Any]:
    try:
        completed = subprocess.run(command, capture_output=True, text=True, timeout=timeout)
        return {
            "command": command,
            "ok": completed.returncode == 0,
            "returncode": completed.returncode,
            "stdout": completed.stdout.strip()[:500],
            "stderr": completed.stderr.strip()[:500],
        }
    except Exception as exc:  # noqa: BLE001
        return {"command": command, "ok": False, "error": str(exc)[:500]}


def app_route(name: str) -> dict[str, Any]:
    result = run_command(["open", "-Ra", name])
    return {
        "app": name,
        "visible_to_codex_process": bool(result.get("ok")),
        "check": result,
        "operator_note": "If this fails here but `open -a {}` works in the operator shell, treat that as a process/app-lookup boundary, not proof the app is absent.".format(name),
    }


def executable_route(path: str) -> dict[str, Any]:
    p = Path(path)
    return {"path": path, "exists": p.exists(), "is_file": p.is_file(), "executable": os.access(path, os.X_OK) if p.exists() else False}


def headless_smoke(path: str) -> dict[str, Any]:
    p = Path(path)
    if not p.exists() or not os.access(path, os.X_OK):
        return {"attempted": False, "reason": "executable_missing_or_not_executable"}
    if "Chrome" not in path and "Chromium" not in path:
        return {"attempted": False, "reason": "no_headless_mode_for_this_route"}
    with tempfile.TemporaryDirectory(prefix="adl-browser-smoke-") as profile:
        return run_command([
            path,
            "--headless=new",
            "--disable-gpu",
            "--no-first-run",
            "--no-default-browser-check",
            f"--user-data-dir={profile}",
            "--dump-dom",
            "data:text/html,<html><body>adl-browser-smoke</body></html>",
        ], timeout=10)


def profile_route(path: str) -> dict[str, Any]:
    p = Path(path)
    return {"path": path, "exists": p.exists(), "is_dir": p.is_dir(), "note": "profile/cache presence is evidence of browser use, not an executable route"}


def path_route(command: str) -> dict[str, Any]:
    resolved = shutil.which(command)
    return {"command": command, "resolved": resolved, "available": resolved is not None}


def http_check(url: str | None) -> dict[str, Any] | None:
    if not url:
        return None
    try:
        request = urllib.request.Request(url, method="HEAD")
        with urllib.request.urlopen(request, timeout=5) as response:  # noqa: S310 - operator-supplied diagnostic URL
            return {"url": url, "reachable": True, "status": response.status, "content_type": response.headers.get("content-type")}
    except urllib.error.HTTPError as exc:
        return {"url": url, "reachable": False, "status": exc.code, "error": str(exc)[:500]}
    except Exception as exc:  # noqa: BLE001
        return {"url": url, "reachable": False, "error": str(exc)[:500]}


def main() -> int:
    parser = argparse.ArgumentParser(description="Diagnose ADL browser proof routes.")
    parser.add_argument("--url", help="optional URL for HTTP reachability only; this is not browser interaction proof")
    parser.add_argument("--headless-smoke", action="store_true", help="try direct Chrome/Chromium headless smoke for known executable paths")
    parser.add_argument("--json", action="store_true", help="emit JSON only")
    args = parser.parse_args()

    report: dict[str, Any] = {
        "schema_version": "adl.browser_routes_diagnostic.v1",
        "recommended_canonical_route": "codex_in_app_browser_for_agent_proof",
        "proof_boundaries": {
            "http_reachability": "curl/urllib can prove a URL responds, but not browser rendering or interaction.",
            "browser_interaction": "Use the Codex in-app browser CUA/evaluate route or an explicit operator browser session.",
        },
        "app_routes": [app_route(name) for name in APP_NAMES],
        "known_executable_routes": [
            {**executable_route(path), **({"headless_smoke": headless_smoke(path)} if args.headless_smoke else {})}
            for path in EXECUTABLE_CANDIDATES
        ],
        "path_routes": [path_route(command) for command in PATH_COMMANDS],
        "profile_routes": [profile_route(path) for path in PROFILE_CANDIDATES],
        "http_check": http_check(args.url),
        "codex_in_app_browser_route": {
            "status": "documented_not_shell_diagnosed",
            "why": "The in-app browser is exposed through the Codex Browser skill, not through this shell process.",
            "known_working_pattern": "browser tab + tab.cua.click({x,y}) + tab.cua.keypress({keys:['Space']}) + DOM evaluate",
        },
    }

    if args.json:
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        print("ADL browser route diagnostic")
        print(f"recommended: {report['recommended_canonical_route']}")
        if report["http_check"]:
            print(f"http: {report['http_check']}")
        print(json.dumps(report, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

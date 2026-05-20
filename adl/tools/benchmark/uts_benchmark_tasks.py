#!/usr/bin/env python3
import json
from pathlib import Path


def repo_root() -> Path:
    return Path(__file__).resolve().parents[3]


def display_task_panel_path(path: str | Path | None) -> str | None:
    if path is None:
        return None
    candidate = Path(path).resolve()
    try:
        return candidate.relative_to(repo_root()).as_posix()
    except ValueError:
        return str(path)


def default_task_panel_path() -> Path:
    return Path(__file__).resolve().parent / "uts_33_task_panel.json"


def load_task_panel(path: str | None = None) -> dict:
    panel_path = Path(path) if path else default_task_panel_path()
    return json.loads(panel_path.read_text(encoding="utf-8"))


def panel_tasks(path: str | None = None) -> list[dict]:
    return load_task_panel(path).get("tasks", [])

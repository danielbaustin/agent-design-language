#!/usr/bin/env python3
import json
import os
import socket
from pathlib import Path


def repo_root() -> Path:
    return Path(__file__).resolve().parents[3]


def display_path(path: str | Path | None) -> str | None:
    if path is None:
        return None
    candidate = Path(path).resolve()
    try:
        return candidate.relative_to(repo_root()).as_posix()
    except ValueError:
        return str(path)


def default_panel_path() -> Path:
    return Path(__file__).resolve().parent / "uts_33_model_panel.json"


def current_host_aliases() -> set[str]:
    host = socket.gethostname().strip().lower()
    short = host.split('.', 1)[0]
    aliases = {host, short}
    if short:
        aliases.add(f'{short}.local')
    return aliases


def current_ollama_base_url() -> str:
    return os.getenv('OLLAMA_HOST', 'http://127.0.0.1:11434').rstrip('/')


def is_remote_ollama_target() -> bool:
    normalized = current_ollama_base_url()
    return normalized not in {'http://127.0.0.1:11434', 'http://localhost:11434'}


def host_policy_note(entry: dict) -> str | None:
    if is_remote_ollama_target():
        return None
    aliases = current_host_aliases()
    allowed_hosts = {str(value).strip().lower() for value in entry.get('allowed_hosts', []) if str(value).strip()}
    disallowed_hosts = {str(value).strip().lower() for value in entry.get('disallowed_hosts', []) if str(value).strip()}
    if allowed_hosts and aliases.isdisjoint(allowed_hosts):
        return (
            f"host_policy_blocked: model '{entry.get('id')}' is not approved on this host; "
            f"allowed_hosts={','.join(sorted(allowed_hosts))}"
        )
    blocked_here = aliases.intersection(disallowed_hosts)
    if blocked_here:
        return (
            f"host_policy_blocked: model '{entry.get('id')}' is disabled on this host "
            f"({','.join(sorted(blocked_here))})"
        )
    return None


def load_panel(path: str | None = None) -> dict:
    panel_path = Path(path) if path else default_panel_path()
    return json.loads(panel_path.read_text(encoding="utf-8"))


def select_models(
    panel: dict,
    model: str | None = None,
    tier: str | None = None,
    provider_kind: str | None = None,
) -> list[dict]:
    models = panel.get("models", [])
    selected = models
    if model:
        selected = [entry for entry in selected if entry.get("id") == model]
    if tier:
        selected = [entry for entry in selected if entry.get("tier") == tier]
    if provider_kind:
        selected = [
            entry
            for entry in selected
            if entry.get("provider_kind") == provider_kind
        ]
    return selected

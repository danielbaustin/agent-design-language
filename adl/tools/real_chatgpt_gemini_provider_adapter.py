#!/usr/bin/env python3
"""Bridge ADL's local HTTP completion contract to live OpenAI and Gemini APIs."""

from __future__ import annotations

import argparse
import http.server
import json
import os
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any


DEFAULT_OPENAI_MODEL = "gpt-4o-mini"
DEFAULT_GEMINI_MODEL = "gemini-2.5-flash"
OPENAI_RESPONSES_URL = "https://api.openai.com/v1/responses"
GEMINI_GENERATE_URL = "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"


def _read_json_request(handler: http.server.BaseHTTPRequestHandler) -> dict[str, Any]:
    length = int(handler.headers.get("Content-Length", "0"))
    raw = handler.rfile.read(length)
    if not raw:
        return {}
    return json.loads(raw.decode("utf-8"))


def _write_json(
    handler: http.server.BaseHTTPRequestHandler,
    status: int,
    payload: dict[str, Any],
) -> None:
    body = json.dumps(payload, indent=2).encode("utf-8")
    handler.send_response(status)
    handler.send_header("Content-Type", "application/json")
    handler.send_header("Content-Length", str(len(body)))
    handler.end_headers()
    handler.wfile.write(body)


def _post_json(
    url: str,
    headers: dict[str, str],
    payload: dict[str, Any],
    timeout: int,
) -> tuple[int, dict[str, str], dict[str, Any]]:
    request = urllib.request.Request(
        url,
        data=json.dumps(payload).encode("utf-8"),
        headers=headers,
        method="POST",
    )
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            body = response.read().decode("utf-8")
            return response.status, dict(response.headers.items()), json.loads(body)
    except urllib.error.HTTPError as exc:
        body = exc.read().decode("utf-8", errors="replace")
        try:
            parsed: dict[str, Any] = json.loads(body)
        except json.JSONDecodeError:
            parsed = {"error": {"message": body[:300]}}
        return exc.code, dict(exc.headers.items()), parsed


def _extract_openai_text(payload: dict[str, Any]) -> str:
    output_text = payload.get("output_text")
    if isinstance(output_text, str) and output_text.strip():
        return output_text

    chunks: list[str] = []
    for item in payload.get("output", []):
        if not isinstance(item, dict):
            continue
        for content in item.get("content", []):
            if not isinstance(content, dict):
                continue
            text = content.get("text")
            if isinstance(text, str):
                chunks.append(text)
    return "\n".join(chunks).strip()


def _extract_gemini_text(payload: dict[str, Any]) -> str:
    chunks: list[str] = []
    for candidate in payload.get("candidates", []):
        if not isinstance(candidate, dict):
            continue
        content = candidate.get("content")
        if not isinstance(content, dict):
            continue
        for part in content.get("parts", []):
            if not isinstance(part, dict):
                continue
            text = part.get("text")
            if isinstance(text, str):
                chunks.append(text)
    return "\n".join(chunks).strip()


class LiveAdapter:
    def __init__(
        self,
        metadata_path: Path,
        openai_model: str,
        gemini_model: str,
        timeout: int,
    ) -> None:
        self.metadata_path = metadata_path
        self.openai_model = openai_model
        self.gemini_model = gemini_model
        self.timeout = timeout
        self.invocations: list[dict[str, Any]] = []

    def write_metadata(self) -> None:
        self.metadata_path.parent.mkdir(parents=True, exist_ok=True)
        payload = {
            "schema_version": "adl.live_provider_invocations.v1",
            "credential_policy": "operator_env_or_home_keys_no_secret_material_recorded",
            "providers": [
                {"family": "openai", "model": self.openai_model},
                {"family": "gemini", "model": self.gemini_model},
            ],
            "invocations": self.invocations,
        }
        self.metadata_path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

    def complete_openai(self, prompt: str) -> str:
        api_key = os.environ["OPENAI_API_KEY"]
        started = time.time()
        status, headers, payload = _post_json(
            OPENAI_RESPONSES_URL,
            {
                "Authorization": f"Bearer {api_key}",
                "Content-Type": "application/json",
            },
            {
                "model": self.openai_model,
                "input": prompt,
                "max_output_tokens": 220,
            },
            self.timeout,
        )
        text = _extract_openai_text(payload)
        self._record("openai", self.openai_model, status, headers, prompt, text, started)
        if status < 200 or status >= 300:
            message = payload.get("error", {}).get("message", "OpenAI request failed")
            raise RuntimeError(f"OpenAI request failed with status {status}: {message}")
        if not text:
            raise RuntimeError("OpenAI response did not contain text output")
        return text

    def complete_gemini(self, prompt: str) -> str:
        api_key = os.environ["GEMINI_API_KEY"]
        started = time.time()
        endpoint = GEMINI_GENERATE_URL.format(
            model=urllib.parse.quote(self.gemini_model, safe="")
        )
        status, headers, payload = _post_json(
            endpoint,
            {
                "x-goog-api-key": api_key,
                "Content-Type": "application/json",
            },
            {
                "contents": [
                    {
                        "role": "user",
                        "parts": [{"text": prompt}],
                    }
                ],
                "generationConfig": {
                    "maxOutputTokens": 220,
                },
            },
            self.timeout,
        )
        text = _extract_gemini_text(payload)
        self._record("gemini", self.gemini_model, status, headers, prompt, text, started)
        if status < 200 or status >= 300:
            message = payload.get("error", {}).get("message", "Gemini request failed")
            raise RuntimeError(f"Gemini request failed with status {status}: {message}")
        if not text:
            raise RuntimeError("Gemini response did not contain text output")
        return text

    def _record(
        self,
        family: str,
        model: str,
        status: int,
        headers: dict[str, str],
        prompt: str,
        output: str,
        started: float,
    ) -> None:
        request_id = (
            headers.get("request-id")
            or headers.get("x-request-id")
            or headers.get("x-goog-request-id")
            or headers.get("x-openai-request-id")
        )
        self.invocations.append(
            {
                "family": family,
                "model": model,
                "http_status": status,
                "request_id_present": bool(request_id),
                "request_id": request_id or None,
                "timestamp_unix_ms": int(started * 1000),
                "latency_ms": int((time.time() - started) * 1000),
                "prompt_chars": len(prompt),
                "output_chars": len(output),
            }
        )
        self.write_metadata()


def make_handler(adapter: LiveAdapter) -> type[http.server.BaseHTTPRequestHandler]:
    class Handler(http.server.BaseHTTPRequestHandler):
        def do_GET(self) -> None:
            if self.path == "/health":
                _write_json(
                    self,
                    200,
                    {
                        "ok": True,
                        "providers": ["openai", "gemini"],
                        "metadata_schema": "adl.live_provider_invocations.v1",
                    },
                )
                return
            if self.path == "/invocations":
                adapter.write_metadata()
                payload = json.loads(adapter.metadata_path.read_text(encoding="utf-8"))
                _write_json(self, 200, payload)
                return
            _write_json(self, 404, {"error": "not found"})

        def do_POST(self) -> None:
            try:
                payload = _read_json_request(self)
                prompt = payload.get("prompt", "")
                if not isinstance(prompt, str) or not prompt.strip():
                    _write_json(self, 400, {"error": "prompt must be a non-empty string"})
                    return
                if self.path == "/openai":
                    output = adapter.complete_openai(prompt)
                elif self.path == "/gemini":
                    output = adapter.complete_gemini(prompt)
                else:
                    _write_json(self, 404, {"error": "not found"})
                    return
                _write_json(self, 200, {"output": output})
            except Exception as exc:  # noqa: BLE001
                _write_json(self, 502, {"error": str(exc)})

        def log_message(self, format: str, *args: Any) -> None:
            return

    return Handler


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Run a local ADL completion adapter backed by live OpenAI and Gemini APIs."
    )
    parser.add_argument("--port", type=int, default=8792)
    parser.add_argument("--port-file", type=Path)
    parser.add_argument("--metadata", type=Path, required=True)
    parser.add_argument("--openai-model", default=os.getenv("ADL_LIVE_OPENAI_MODEL", DEFAULT_OPENAI_MODEL))
    parser.add_argument("--gemini-model", default=os.getenv("ADL_LIVE_GEMINI_MODEL", DEFAULT_GEMINI_MODEL))
    parser.add_argument("--timeout", type=int, default=int(os.getenv("ADL_LIVE_PROVIDER_TIMEOUT_SECS", "60")))
    args = parser.parse_args()

    missing = [name for name in ("OPENAI_API_KEY", "GEMINI_API_KEY") if not os.getenv(name)]
    if missing:
        raise SystemExit(f"missing required environment variables: {', '.join(missing)}")

    adapter = LiveAdapter(
        metadata_path=args.metadata,
        openai_model=args.openai_model,
        gemini_model=args.gemini_model,
        timeout=args.timeout,
    )
    adapter.write_metadata()
    handler = make_handler(adapter)
    server = http.server.ThreadingHTTPServer(("127.0.0.1", args.port), handler)
    if args.port_file:
        args.port_file.parent.mkdir(parents=True, exist_ok=True)
        args.port_file.write_text(f"{server.server_address[1]}\n", encoding="utf-8")
    server.serve_forever()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

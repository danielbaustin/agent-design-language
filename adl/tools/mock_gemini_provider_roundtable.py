#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


def _collapse(text: str) -> str:
    return " ".join(text.split())


def _extract_between(prompt: str, label: str) -> str:
    marker = f"{label}:"
    if marker not in prompt:
        return ""
    return _collapse(prompt.split(marker, 1)[1].strip())


def build_output(route: str, prompt: str) -> str:
    topic = _extract_between(prompt, "TOPIC")
    if route == "gemini":
        return (
            "# Gemini Opening\n\n"
            "Gemini argues that this packet fits it well because the task is bounded, "
            "reviewer-facing, and synthesis-heavy rather than repo-autonomy-heavy. "
            "It explicitly avoids claiming a universal win and frames itself as the "
            f"right participant for this question: {topic}"
        )
    if route == "chatgpt":
        return (
            "# ChatGPT Response\n\n"
            "ChatGPT agrees that Gemini is a sensible choice for a bounded, legible "
            "packet and says the selection rationale is already clearer than simple "
            "provider preference. It keeps one tradeoff visible: premium models may "
            "still be the better fit when the task demands broader exploratory depth."
        )
    if route == "claude":
        return (
            "# Claude Response\n\n"
            "Claude adds a governance caution: a calm provider-fit story only works if "
            "the runtime keeps the boundaries visible and does not use hospitality to "
            "conceal missing affordances. That caution strengthens, rather than weakens, "
            "the case for explicit Gemini selection artifacts."
        )
    if route == "synthesis":
        return (
            "# Gemini Roundtable Synthesis\n\n"
            "## Findings\n"
            "- Gemini fits this packet because it is bounded, structured, and reviewer-facing.\n"
            "- ChatGPT keeps the capability-depth tradeoff visible instead of flattening it.\n"
            "- Claude adds the governance warning that makes the provider-choice story honest.\n\n"
            "## Why Gemini Fits This Packet\n"
            "Gemini is selected here because the packet rewards bounded synthesis, legibility, "
            "and an economical cost-class rather than maximal open-ended reasoning.\n\n"
            "## Tradeoffs That Remain Visible\n"
            "ADL should still say plainly that premium providers may remain the better fit for "
            "broader exploratory work, and that provider hospitality is not a license to blur "
            "real capability differences.\n\n"
            "## Acknowledgement\n"
            "This packet builds on earlier bounded Gemini and provider-harmony work instead of "
            "pretending to discover the framing from scratch."
        )
    return "# Unknown\n\nUnsupported route."


class Handler(BaseHTTPRequestHandler):
    def _json(self, status: int, payload: dict) -> None:
        body = json.dumps(payload).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_GET(self) -> None:
        if self.path == "/health":
            self._json(200, {"ok": True})
            return
        self._json(404, {"error": "not found"})

    def do_POST(self) -> None:
        route = self.path.lstrip("/")
        if route not in {"gemini", "chatgpt", "claude", "synthesis"}:
            self._json(404, {"error": "unknown route"})
            return
        length = int(self.headers.get("Content-Length", "0"))
        payload = json.loads(self.rfile.read(length).decode("utf-8"))
        prompt = payload.get("prompt")
        if not isinstance(prompt, str) or not prompt.strip():
            self._json(400, {"error": "prompt must be a non-empty string"})
            return
        self._json(200, {"output": build_output(route, prompt)})

    def log_message(self, fmt: str, *args) -> None:
        return


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("port", nargs="?", type=int, default=8793)
    parser.add_argument("--port-file", type=Path)
    args = parser.parse_args()

    server = ThreadingHTTPServer(("127.0.0.1", args.port), Handler)
    if args.port_file:
        args.port_file.parent.mkdir(parents=True, exist_ok=True)
        args.port_file.write_text(f"{server.server_address[1]}\n", encoding="utf-8")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

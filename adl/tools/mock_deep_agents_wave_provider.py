#!/usr/bin/env python3
import argparse
import json
import re
from pathlib import Path
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer


def extract_field(prompt: str, key: str) -> str:
    match = re.search(rf"^{re.escape(key)}:\s*(.+)$", prompt, re.MULTILINE)
    return match.group(1).strip() if match else ""


def extract_previous_turn(prompt: str) -> str:
    match = re.search(r"PREVIOUS_TURN_START\n(.*?)\nPREVIOUS_TURN_END", prompt, re.DOTALL)
    if not match:
        return ""
    return " ".join(match.group(1).split())


def build_output(agent: str, prompt: str) -> str:
    previous = extract_previous_turn(prompt)
    quoted = f'"{previous[:120]}..."' if previous else ""

    if agent == "chatgpt":
        return (
            "# Turn 1 - ChatGPT\n\n"
            "ADL's comparative strength is that it turns a packet into a runtime-backed "
            "review surface instead of leaving the reviewer to infer what happened. "
            "That means clearer artifacts, cleaner trace, and less dependence on charm."
        )
    if agent == "claude":
        return (
            "# Turn 2 - Claude\n\n"
            "That advantage is real, but it can be overstated. A looser deep-agent packet "
            "sometimes feels more exploratory and less ceremonial, so ADL should not treat "
            f"reviewability as if it automatically wins every tradeoff. {quoted}"
        )
    if agent == "gemini":
        return (
            "# Turn 3 - Gemini\n\n"
            "The useful middle ground is to say that ADL is clearly stronger when a reviewer "
            "needs provenance, bounded claims, and inspectable handoff surfaces. The looser "
            f"packet style still has appeal when speed and improvisation matter. {quoted}"
        )
    if agent == "synthesis":
        return (
            "# Deep-Agents Comparative Wave Synthesis\n\n"
            "## Findings\n"
            "- ADL is stronger when the comparative goal is reviewer trust, provenance, and replay.\n"
            "- Looser deep-agent packets retain some appeal for exploratory, lower-formality work.\n"
            "- The honest public story is not that ADL destroys other approaches, but that it makes bounded review work more legible.\n\n"
            "## Agreement\n"
            "- reviewer-facing artifacts matter\n"
            "- bounded claims are stronger than theatrical claims\n\n"
            "## Disagreement\n"
            "- how much structure is worth the coordination cost\n\n"
            "## Positioning Takeaway\n"
            "ADL should present itself as the calm, inspectable runtime for serious multi-agent work, not as the winner of a benchmark contest."
        )
    return "# Unknown\n\nThe bounded provider shim received an unsupported route."


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
        agent = self.path.lstrip("/")
        if agent not in {"chatgpt", "claude", "gemini", "synthesis"}:
            self._json(404, {"error": "unknown route"})
            return
        length = int(self.headers.get("Content-Length", "0"))
        payload = json.loads(self.rfile.read(length).decode("utf-8"))
        prompt = payload.get("prompt", "")
        if not isinstance(prompt, str) or not prompt.strip():
            self._json(400, {"error": "prompt must be a non-empty string"})
            return
        self._json(200, {"output": build_output(agent, prompt)})

    def log_message(self, format: str, *args) -> None:
        return


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("port", nargs="?", type=int, default=8792)
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

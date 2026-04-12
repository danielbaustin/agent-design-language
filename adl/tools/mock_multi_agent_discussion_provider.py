#!/usr/bin/env python3
import argparse
import json
import re
import sys
from pathlib import Path
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer


def extract_field(prompt: str, key: str) -> str:
    match = re.search(rf"^{re.escape(key)}:\s*(.+)$", prompt, re.MULTILINE)
    return match.group(1).strip() if match else ""


def extract_previous_turn(prompt: str) -> str:
    match = re.search(
        r"PREVIOUS_TURN_START\n(.*?)\nPREVIOUS_TURN_END", prompt, re.DOTALL
    )
    if not match:
        return ""
    text = match.group(1).strip()
    return " ".join(text.split())


def build_output(agent: str, prompt: str) -> str:
    turn_id = extract_field(prompt, "TURN_ID")
    topic = extract_field(prompt, "TOPIC")
    previous = extract_previous_turn(prompt)
    quoted = f'"{previous[:110]}..."' if previous else ""

    if agent == "chatgpt" and turn_id == "01":
        return (
            "# Turn 1 - ChatGPT\n\n"
            "ChatGPT sets out a careful pot of Earl Grey and proposes a bounded "
            "five-turn discussion because a small explicit workflow is easier to "
            "review than a vague multi-agent abstraction. The topic stays narrow: "
            f"{topic}"
        )
    if agent == "claude" and turn_id == "02":
        return (
            "# Turn 2 - Claude\n\n"
            "Claude accepts the tea and replies that explicit saved state keeps "
            "the discussion inspectable. It answers the opening by noting that "
            f"the prior turn already made the review boundary visible via {quoted}"
        )
    if agent == "chatgpt" and turn_id == "03":
        return (
            "# Turn 3 - ChatGPT\n\n"
            "ChatGPT reflects that trace and run artifacts matter because they let "
            "a reviewer inspect how one turn led to the next. It stays truthful by "
            "treating this as a bounded runtime workflow, not a general conversation "
            f"system, while carrying forward {quoted}"
        )
    if agent == "claude" and turn_id == "04":
        return (
            "# Turn 4 - Claude\n\n"
            "Claude tightens the claim: this demo proves two named agents, sequential "
            "runtime steps, and saved-state handoff. It explicitly does not claim a "
            f"conversation-native runtime, and it grounds that boundary in {quoted}"
        )
    if agent == "chatgpt" and turn_id == "05":
        return (
            "# Turn 5 - ChatGPT\n\n"
            "ChatGPT closes with a toast: here is to two named agents, five explicit "
            "turns, one real runtime path, and a transcript a reviewer can inspect. "
            f"It ends by carrying forward {quoted}"
        )
    return (
        f"# Turn {turn_id or 'unknown'} - {agent.title()}\n\n"
        "The local compatibility provider received a bounded prompt but did not match "
        "a known scripted turn."
    )


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
        if self.path not in {"/chatgpt", "/claude"}:
            self._json(404, {"error": "unknown route"})
            return
        length = int(self.headers.get("Content-Length", "0"))
        raw = self.rfile.read(length)
        try:
            payload = json.loads(raw.decode("utf-8"))
        except json.JSONDecodeError:
            self._json(400, {"error": "invalid json"})
            return
        prompt = payload.get("prompt")
        if not isinstance(prompt, str) or not prompt.strip():
            self._json(400, {"error": "prompt must be a non-empty string"})
            return
        agent = self.path.lstrip("/")
        self._json(200, {"output": build_output(agent, prompt)})

    def log_message(self, format: str, *args) -> None:
        return


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Run the bounded local compatibility provider for the multi-agent discussion demo."
    )
    parser.add_argument("port", nargs="?", type=int, default=8791)
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

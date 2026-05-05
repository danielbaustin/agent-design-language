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
    stop_rule = extract_field(prompt, "STOP_RULE")
    previous = extract_previous_turn(prompt)
    quoted = f'"{previous[:110]}..."' if previous else ""

    if agent == "chatgpt" and turn_id == "01":
        return (
            "# Turn 1 - ChatGPT\n\n"
            "ChatGPT opens by stating the proof boundary directly: this is a "
            "bounded four-turn exchange with Gemini, not an open-ended multi-agent "
            "platform. The topic stays narrow and reviewer-facing: "
            f"{topic} The stop rule is explicit: {stop_rule}"
        )
    if agent == "gemini" and turn_id == "02":
        return (
            "# Turn 2 - Gemini\n\n"
            "Gemini replies as a distinct participant and agrees that saved artifacts "
            "matter more than a merely pleasant chat. It keeps the identity and stop "
            f"boundary visible by replying to {quoted}"
        )
    if agent == "chatgpt" and turn_id == "03":
        return (
            "# Turn 3 - ChatGPT\n\n"
            "ChatGPT tightens the claim: this direct exchange proves named "
            "participants, ordered turns, and a preserved transcript. It does not "
            "claim general federation, unrestricted autonomy, or production-ready "
            f"provider routing, while carrying forward {quoted}"
        )
    if agent == "gemini" and turn_id == "04":
        return (
            "# Turn 4 - Gemini\n\n"
            "Gemini closes the loop explicitly and restates the stop policy so the "
            "artifact reads as bounded by design. It ends by saying this proof shows "
            "a clean ChatGPT/Gemini back-and-forth with explicit identities and a "
            f"reviewable finish, grounded in {quoted}"
        )
    return (
        f"# Turn {turn_id or 'unknown'} - {agent.title()}\n\n"
        "The local compatibility provider received a bounded prompt but did not "
        "match a known scripted turn."
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
        if self.path not in {"/chatgpt", "/gemini"}:
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
        description="Run the bounded local compatibility provider for the ChatGPT/Gemini direct conversation demo."
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

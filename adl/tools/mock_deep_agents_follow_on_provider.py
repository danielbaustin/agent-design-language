#!/usr/bin/env python3
import argparse
import json
from pathlib import Path
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer


def build_output(agent: str) -> str:
    if agent == "chatgpt":
        return (
            "# Turn 1 - ChatGPT\n\n"
            "ADL's comparative strength is not that it makes more dramatic files. "
            "It makes the same multi-agent packet easier to inspect because the "
            "review surface, trace, and packet structure stay explicit. That is a "
            "better story for a serious reviewer than a folder full of outputs with "
            "implied orchestration."
        )
    if agent == "claude":
        return (
            "# Turn 2 - Claude\n\n"
            "That strength is real, but ADL should admit the cost. Additional "
            "review surfaces and governance signals can make the flow feel more "
            "ceremonial. A filesystem-first deep-agent demo can still feel faster "
            "and more exploratory when the audience values improvisation over audit."
        )
    if agent == "gemini":
        return (
            "# Turn 3 - Gemini\n\n"
            "The comparison gets sharper when framed around operator trust. ADL is "
            "clearly stronger when the reviewer needs stop conditions, provenance, "
            "and role visibility. The looser packet style still has appeal for quick "
            "creative iteration, but it gives the reviewer fewer structured handles."
        )
    if agent == "synthesis":
        return (
            "# Deep-Agents Comparative Governance Wave\n\n"
            "## Findings\n"
            "- ADL makes multi-agent behavior easier to inspect because the packet, trace, and review surfaces stay linked.\n"
            "- Filesystem-first demos remain attractive when speed and improvisation matter more than auditability.\n"
            "- The honest comparative claim is about governance and reviewability, not who wins a benchmark.\n\n"
            "## Agreement\n"
            "- visible intermediate artifacts help reviewers\n"
            "- bounded claims are stronger than theatrical claims\n\n"
            "## Disagreement\n"
            "- whether stronger governance is worth the added coordination surface\n\n"
            "## Governance Delta\n"
            "ADL exposes packet shape, reviewer checkpoints, and trace lineage as explicit review surfaces rather than leaving them implicit.\n\n"
            "## Operator Visibility\n"
            "The operator role is easier to describe because ADL makes packet boundaries and review obligations visible instead of burying them inside the process.\n\n"
            "## Positioning Takeaway\n"
            "ADL should present itself as the calm, inspectable runtime for serious multi-agent review work, not as the loudest deep-agent theater."
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
        self._json(200, {"output": build_output(agent)})

    def log_message(self, format: str, *args) -> None:
        return


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("port", nargs="?", type=int, default=8794)
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

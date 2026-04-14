#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path


def load_output(response_path: Path) -> str:
    payload = json.loads(response_path.read_text(encoding="utf-8"))
    return json.dumps(payload, indent=2)


class Handler(BaseHTTPRequestHandler):
    response_text = ""

    def do_POST(self) -> None:
        if self.path != "/complete":
            self.send_error(404)
            return
        length = int(self.headers.get("Content-Length", "0"))
        body = self.rfile.read(length)
        try:
            payload = json.loads(body.decode("utf-8"))
        except json.JSONDecodeError:
            self.send_error(400, "invalid json")
            return
        if "prompt" not in payload or not isinstance(payload["prompt"], str):
            self.send_error(400, "missing prompt")
            return
        response = {"output": self.response_text}
        encoded = json.dumps(response).encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(encoded)))
        self.end_headers()
        self.wfile.write(encoded)

    def log_message(self, fmt: str, *args) -> None:
        return


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--port", type=int, required=True)
    parser.add_argument("--response-file", type=Path, required=True)
    args = parser.parse_args()
    Handler.response_text = load_output(args.response_file)
    server = HTTPServer(("127.0.0.1", args.port), Handler)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
import argparse
import json
import re
from pathlib import Path
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer


def extract_field(prompt: str, key: str) -> str:
    match = re.search(rf"^{re.escape(key)}:\s*(.+)$", prompt, re.MULTILINE)
    return match.group(1).strip() if match else ""


def build_output(agent: str, prompt: str) -> str:
    turn_id = extract_field(prompt, "TURN_ID")
    section = extract_field(prompt, "SECTION")

    if agent == "layer8" and turn_id == "01":
        return (
            "# Layer 8 Opening\n\n"
            "Layer 8 taps the start cue on the controller, names the opening invitation, "
            "and asks the ensemble to keep the first entrance warm and gentle."
        )
    if agent == "layer8" and turn_id == "06":
        return (
            "# Layer 8 Long Fade Cue\n\n"
            "Layer 8 lifts one hand, signals two bounded na-loop passes, and points the "
            "ensemble toward a clean curtain call instead of an endless fade."
        )
    if agent == "chatgpt" and turn_id == "02":
        return (
            "# ChatGPT Opening\n\n"
            "ChatGPT frames the verse rotation, keeps the tempo readable, and hands the "
            "first shared cue toward the more reflective voices."
        )
    if agent == "chatgpt" and turn_id == "07":
        return (
            "# ChatGPT Chorus Lift\n\n"
            "ChatGPT keeps the shared refrain moving, counting the loop softly so the "
            "group sounds coordinated instead of crowded."
        )
    if agent == "claude" and turn_id == "03":
        return (
            "# Claude Verse Turn\n\n"
            "Claude adds warmth and reassurance, treating the section as an invitation to "
            "make the mood gentler rather than louder."
        )
    if agent == "claude" and turn_id == "08":
        return (
            "# Claude Chorus Turn\n\n"
            "Claude softens the repeated response with one reflective line about the room "
            "feeling fuller now that everyone is singing together."
        )
    if agent == "gemini" and turn_id == "04":
        return (
            "# Gemini Verse Turn\n\n"
            "Gemini brightens the verse rotation with a quick responsive cue and makes the "
            "transition into the chorus feel buoyant."
        )
    if agent == "gemini" and turn_id == "09":
        return (
            "# Gemini Chorus Turn\n\n"
            "Gemini throws in one bright refrain marker that keeps the chorus playful while "
            "still following the bounded loop count."
        )
    if agent == "deepseek" and turn_id == "05":
        return (
            "# DeepSeek Chorus Setup\n\n"
            "DeepSeek establishes the shared-response pattern, marks the loop count, and "
            "keeps the chorus from spilling over its structure."
        )
    if agent == "deepseek" and turn_id == "10":
        return (
            "# DeepSeek Curtain Call\n\n"
            "DeepSeek lands the final cadence cleanly, confirms the stop cue, and turns "
            "the ensemble toward a gentle curtain call."
        )
    return f"# {agent} {section}\n\nThe bounded rehearsal provider did not recognize this turn."


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
        if agent not in {"layer8", "chatgpt", "claude", "gemini", "deepseek"}:
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
    parser.add_argument("port", nargs="?", type=int, default=8796)
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

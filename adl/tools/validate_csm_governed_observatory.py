#!/usr/bin/env python3
"""Validate the governed CSM Observatory prototype semantics and render smoke behavior."""

from __future__ import annotations

import argparse
import json
import subprocess
import textwrap
from pathlib import Path
from typing import Any


def fail(message: str) -> None:
    raise SystemExit(f"FAIL: {message}")


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        fail(f"{path} must contain a JSON object")
    return value


def assert_equal(label: str, observed: Any, expected: Any) -> None:
    if observed != expected:
        fail(f"{label} mismatch: observed {observed!r}, expected {expected!r}")


def assert_contains(label: str, observed: str | None, expected: str) -> None:
    if not observed or expected not in observed:
        fail(f"{label} did not contain {expected!r}")


def run_render_smoke(js_path: Path) -> dict[str, Any]:
    appended_js = """
packet = fallbackPacket;
renderPrototype();
globalThis.__governedSmoke = {
  fallbackPacket,
  atlasSummary: elements.get("#atlas-summary")?.textContent,
  governanceSummary: elements.get("#governance-summary")?.textContent,
  inspectorHeading: elements.get("#inspector-heading")?.textContent,
  inspectorAllowed: elements.get("#inspector-allowed")?.textContent,
  proposalModeStatement: elements.get("#proposal-mode-statement")?.textContent,
  proposalCards: elements.get("#proposal-cards")?.innerHTML,
  proposalDetail: elements.get("#proposal-detail")?.innerHTML,
  reviewLinks: elements.get("#review-links")?.innerHTML,
  traceRibbon: elements.get("#trace-ribbon")?.innerHTML,
  roomTabs: elements.get("#room-tabs")?.innerHTML,
  lensTabs: elements.get("#lens-tabs")?.innerHTML,
  memoryDots: elements.get("#memory-dots")?.innerHTML
};
"""
    node_program = textwrap.dedent(
        f"""
        const fs = require("fs");
        const vm = require("vm");

        const elements = new Map();
        function element(selector, extra = {{}}) {{
          if (!elements.has(selector)) {{
            elements.set(selector, {{
              selector,
              innerHTML: "",
              textContent: "",
              dataset: {{}},
              classList: {{ toggle() {{}} }},
              addEventListener() {{}},
              ...extra
            }});
          }}
          return elements.get(selector);
        }}

        const document = {{
          body: element("body", {{ classList: {{ toggle() {{}} }} }}),
          querySelector(selector) {{
            if (selector === ".observatory-governed-shell") {{
              return element(selector, {{ dataset: {{}} , classList: {{ toggle() {{}} }} }});
            }}
            return element(selector);
          }},
          querySelectorAll() {{
            return [];
          }},
          addEventListener() {{}}
        }};

        const source = fs.readFileSync({json.dumps(str(js_path))}, "utf8") + {json.dumps(appended_js)};
        const context = {{ document, fetch: undefined, console, elements }};
        vm.runInNewContext(source, context);
        process.stdout.write(JSON.stringify(context.__governedSmoke));
        """
    )
    try:
        completed = subprocess.run(
            ["node", "-e", node_program],
            check=True,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    except FileNotFoundError:
        fail("node is required for governed Observatory render smoke validation")
    except subprocess.CalledProcessError as exc:
        fail(f"governed Observatory render smoke failed: {exc.stderr.strip()}")
    return json.loads(completed.stdout)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--html", type=Path, required=True)
    parser.add_argument("--js", type=Path, required=True)
    parser.add_argument("--packet", type=Path, required=True)
    args = parser.parse_args()

    html = args.html.read_text(encoding="utf-8")
    packet = load_json(args.packet)
    ui = packet.get("observatory_ui", {})
    smoke = run_render_smoke(args.js)
    fallback = smoke["fallbackPacket"]

    assert_contains(
        "HTML packet reference",
        html,
        "data-packet-ref=\"../fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json\"",
    )
    assert_equal("packet schema", packet["schema"], "adl.csm_visibility_packet.v1")
    assert_equal("fallback schema", fallback["schema"], packet["schema"])
    assert_equal("fallback packet_id", fallback["packet_id"], packet["packet_id"])
    assert_equal("default room", fallback["observatory_ui"]["default_room"], ui["default_room"])
    assert_equal("default lens", fallback["observatory_ui"]["default_lens"], ui["default_lens"])
    assert_equal(
        "proposal ids",
        [item["proposal_id"] for item in fallback["observatory_ui"]["proposal_cases"]],
        [item["proposal_id"] for item in ui["proposal_cases"]],
    )
    assert_contains("atlas summary", smoke["atlasSummary"], packet["manifold"]["health"]["summary"])
    assert_contains("governance summary", smoke["governanceSummary"], "allow")
    assert_equal("inspector heading", smoke["inspectorHeading"], packet["citizens"][0]["display_name"])
    assert_contains("inspector allowed", smoke["inspectorAllowed"], "bounded reviewable episode")
    assert_contains("proposal mode statement", smoke["proposalModeStatement"], "proposal")
    assert_contains("proposal cards", smoke["proposalCards"], "Inspect Alpha continuity packet")
    assert_contains("proposal detail", smoke["proposalDetail"], "validate operator identity")
    assert_contains("review links", smoke["reviewLinks"], "runtime_v2/observatory/operator_report.md")
    assert_contains("trace ribbon", smoke["traceRibbon"], "Operator opened a continuity review request")
    assert_contains("room tabs", smoke["roomTabs"], "World / Reality")
    assert_contains("lens tabs", smoke["lensTabs"], "Operator lens")
    assert_contains("memory dots", smoke["memoryDots"], "Corporate Investor")

    print("PASS: governed CSM Observatory semantic/render validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

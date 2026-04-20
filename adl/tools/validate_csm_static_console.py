#!/usr/bin/env python3
"""Validate CSM Observatory static console semantics and render smoke behavior."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
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


def run_console_smoke(js_path: Path) -> dict[str, Any]:
    appended_js = """
globalThis.__csmConsoleSmoke = {
  fallbackPacket,
  inspectorHeading: elements.get("#inspector-heading")?.textContent,
  inspectorState: elements.get("#inspector-state")?.textContent,
  inspectorEpisode: elements.get("#inspector-episode")?.textContent,
  inspectorCompute: elements.get("#inspector-compute")?.textContent,
  serviceRows: elements.get("#service-ladder")?.innerHTML,
  docketRows: elements.get("#verdict-stack")?.innerHTML,
  traceRows: elements.get("#trace-ribbon")?.innerHTML,
  actionRows: elements.get("#operator-actions")?.innerHTML
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
              classList: {{ toggle: function() {{}} }},
              addEventListener: function() {{}},
              ...extra
            }});
          }}
          return elements.get(selector);
        }}

        const citizenNodes = [
          element(".citizen-node-alpha", {{ dataset: {{ citizen: "proto-citizen-alpha" }} }}),
          element(".citizen-node-beta", {{ dataset: {{ citizen: "proto-citizen-beta" }} }})
        ];

        const document = {{
          querySelector(selector) {{
            if (selector === ".observatory-shell") {{
              return element(selector, {{ dataset: {{}} }});
            }}
            return element(selector);
          }},
          querySelectorAll(selector) {{
            if (selector === ".citizen-node") {{
              return citizenNodes;
            }}
            return [];
          }}
        }};

        const source = fs.readFileSync({json.dumps(str(js_path))}, "utf8") + {json.dumps(appended_js)};

        const context = {{ document, fetch: undefined, console, elements }};
        vm.runInNewContext(source, context);
        process.stdout.write(JSON.stringify(context.__csmConsoleSmoke));
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
    except FileNotFoundError as exc:
        fail("node is required for static console render smoke validation")
    except subprocess.CalledProcessError as exc:
        fail(f"static console render smoke failed: {exc.stderr.strip()}")
    return json.loads(completed.stdout)


def assert_equal(label: str, observed: Any, expected: Any) -> None:
    if observed != expected:
        fail(f"{label} mismatch: observed {observed!r}, expected {expected!r}")


def assert_contains(label: str, observed: str | None, expected: str) -> None:
    if not observed or expected not in observed:
        fail(f"{label} did not contain {expected!r}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--html", type=Path, required=True)
    parser.add_argument("--js", type=Path, required=True)
    parser.add_argument("--packet", type=Path, required=True)
    args = parser.parse_args()

    html = args.html.read_text(encoding="utf-8")
    fixture = load_json(args.packet)
    smoke = run_console_smoke(args.js)
    fallback = smoke["fallbackPacket"]

    assert_contains(
        "HTML packet reference",
        html,
        "data-packet-ref=\"../../fixtures/csm_observatory/proto-csm-01-visibility-packet.json\"",
    )
    assert_equal("fallback schema", fallback["schema"], fixture["schema"])
    assert_equal("fallback packet_id", fallback["packet_id"], fixture["packet_id"])
    assert_equal("fallback source mode", fallback["source"]["mode"], fixture["source"]["mode"])
    assert_equal("fallback manifold id", fallback["manifold"]["manifold_id"], fixture["manifold"]["manifold_id"])
    assert_equal(
        "fallback kernel pulse status",
        fallback["kernel"]["pulse"]["status"],
        fixture["kernel"]["pulse"]["status"],
    )
    assert_equal(
        "fallback citizen ids",
        [item["citizen_id"] for item in fallback["citizens"]],
        [item["citizen_id"] for item in fixture["citizens"]],
    )
    assert_equal(
        "fallback available actions",
        [item["action"] for item in fallback["operator_actions"]["available_actions"]],
        [item["action"] for item in fixture["operator_actions"]["available_actions"]],
    )
    assert_equal(
        "fallback disabled actions",
        [item["action"] for item in fallback["operator_actions"]["disabled_actions"]],
        [item["action"] for item in fixture["operator_actions"]["disabled_actions"]],
    )

    alpha = fixture["citizens"][0]
    assert_equal("renderInspector heading", smoke["inspectorHeading"], alpha["display_name"])
    assert_contains("renderInspector state", smoke["inspectorState"], alpha["lifecycle_state"])
    assert_equal("renderInspector episode", smoke["inspectorEpisode"], alpha["current_episode"])
    assert_equal("renderInspector compute", smoke["inspectorCompute"], f"{alpha['resource_balance']['compute_units']} units")
    assert_contains("renderServices output", smoke["serviceRows"], "clock service")
    assert_contains("renderDocket output", smoke["docketRows"], "cross polis export")
    assert_contains("renderTrace output", smoke["traceRows"], "Clock service observed ready.")
    assert_contains("renderActions output", smoke["actionRows"], "pause citizen")

    print("PASS: CSM Observatory static console semantic/render validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

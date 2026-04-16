#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0891/wp13_demo_integration}"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

python3 - "$OUT_DIR" <<'PY'
import json
import sys
from pathlib import Path

out_dir = Path(sys.argv[1])


def write_json(path: Path, payload: dict) -> None:
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def write_text(path: Path, text: str) -> None:
    path.write_text(text.strip() + "\n", encoding="utf-8")


demo_rows = [
    {
        "demo_id": "D7",
        "title": "Reviewer-facing security proof package",
        "status": "LANDED",
        "work_packages": ["WP-10", "WP-11", "WP-12", "WP-13"],
        "entry_commands": [
            "adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json",
            "adl identity demo-proof-entry-points --out .adl/state/demo_proof_entry_points_v1.json",
            "bash adl/tools/demo_v0891_wp13_demo_integration.sh",
        ],
        "primary_proof_surfaces": [
            ".adl/state/provider_extension_packaging_v1.json",
            ".adl/state/demo_proof_entry_points_v1.json",
            "artifacts/v0891/wp13_demo_integration/integration_manifest.json",
        ],
        "proof_role": "Bundles the milestone proof commands, carry-forward boundaries, D8 delight demo, and D9 manuscript workflow packet into one reviewer-facing integration surface.",
        "determinism_note": "Identity packets and the WP-13 integration packet are deterministic; heavyweight child demos remain replayable through their own test commands.",
    },
    {
        "demo_id": "D8",
        "title": "Five-Agent Hey Jude MIDI demo",
        "status": "LANDED",
        "work_packages": ["WP-08", "WP-09", "WP-10", "WP-13"],
        "entry_commands": ["bash adl/tools/demo_v0891_five_agent_hey_jude.sh"],
        "test_commands": ["bash adl/tools/test_demo_v0891_five_agent_hey_jude.sh"],
        "primary_proof_surfaces": [
            "artifacts/v0891/five_agent_hey_jude/performance_manifest.json",
            "artifacts/v0891/five_agent_hey_jude/midi_event_log.json",
            "artifacts/v0891/five_agent_hey_jude/provider_participation_summary.json",
            "artifacts/v0891/five_agent_hey_jude/runtime/runs/v0-89-1-five-agent-hey-jude-midi-demo/run_summary.json",
        ],
        "tracked_repo_paths": [
            "adl/tools/demo_v0891_five_agent_hey_jude.sh",
            "adl/tools/test_demo_v0891_five_agent_hey_jude.sh",
            "adl/tools/validate_five_agent_music_demo.py",
            "demos/v0.89.1/five_agent_hey_jude_midi_demo.md",
        ],
        "proof_role": "Shows one human Layer 8 participant plus four provider voices coordinating through one bounded ADL runtime packet and MIDI cue layer.",
        "determinism_note": "The fixture-backed MVAVE Chocolate event stream preserves cue order and validates the same artifact schema on each run.",
    },
    {
        "demo_id": "D9",
        "title": "ArXiv manuscript workflow packet",
        "status": "LANDED",
        "work_packages": ["WP-08", "WP-13"],
        "entry_commands": ["bash adl/tools/demo_v0891_arxiv_manuscript_workflow.sh"],
        "test_commands": ["bash adl/tools/test_demo_v0891_arxiv_manuscript_workflow.sh"],
        "primary_proof_surfaces": [
            "artifacts/v0891/arxiv_manuscript_workflow/demo_manifest.json",
            "artifacts/v0891/arxiv_manuscript_workflow/source_packets/source_packet_manifest.json",
            "artifacts/v0891/arxiv_manuscript_workflow/manuscript_status/three_paper_status.json",
            "artifacts/v0891/arxiv_manuscript_workflow/review/review_gates.json",
        ],
        "tracked_repo_paths": [
            "adl/tools/demo_v0891_arxiv_manuscript_workflow.sh",
            "adl/tools/test_demo_v0891_arxiv_manuscript_workflow.sh",
            "demos/v0.89.1/arxiv_manuscript_workflow_demo.md",
        ],
        "proof_role": "Shows the bounded three-paper manuscript workflow packet, source packets, review gates, and no-submission boundary.",
        "determinism_note": "Packet generation is deterministic and preserves paper order, role order, source references, and claim-boundary wording.",
    },
]

manifest = {
    "schema_version": "adl.v0891.wp13_demo_integration.v1",
    "milestone": "v0.89.1",
    "work_package": "WP-13",
    "issue": "#1934",
    "title": "Demo matrix and integration demos",
    "disposition": "bounded_integration_packet",
    "dependency_truth": {
        "wp12_issue": "#1933",
        "wp12_state": "merged_before_wp13_publication",
        "integration_record": "WP-13 consumes the merged WP-12 convergence surface and does not close or replace WP-12.",
    },
    "demo_rows": demo_rows,
    "validation_commands": [
        {
            "command": "bash adl/tools/test_demo_v0891_wp13_demo_integration.sh",
            "verifies": "WP-13 integration packet schema, row status, tracked path existence, and leakage checks.",
        },
        {
            "command": "bash adl/tools/test_demo_v0891_five_agent_hey_jude.sh",
            "verifies": "D8 five-agent MIDI demo artifacts and copyright-safe transcript boundary.",
        },
        {
            "command": "bash adl/tools/test_demo_v0891_arxiv_manuscript_workflow.sh",
            "verifies": "D9 manuscript workflow packet, source packets, review gates, and no-submission boundary.",
        },
        {
            "command": "cargo test --manifest-path adl/Cargo.toml demo_proof_entry_points --quiet",
            "verifies": "CLI proof-entry contract status for the updated D7/D8/D9 integration rows.",
        },
    ],
    "review_boundaries": [
        "The integration packet names and validates demo proof surfaces; it does not submit papers to arXiv.",
        "D8 is a bounded delight/integration demo and not a replacement for the adversarial D5 exploit-replay proof.",
        "Provider-security attestation, trust scoring, sandbox policy, and external provider-security demos remain out of v0.89.1 scope.",
        "WP-13 consumes the merged WP-12 convergence surface instead of reopening or replacing it.",
    ],
}

write_json(out_dir / "integration_manifest.json", manifest)
write_json(out_dir / "demo_rows.json", {"schema_version": "adl.v0891.wp13_demo_rows.v1", "rows": demo_rows})

reviewer_brief = """# WP-13 Demo Integration Reviewer Brief

Review this packet in the following order:

1. `integration_manifest.json`
2. `demo_rows.json`
3. `dependency_and_scope.md`
4. `validation_plan.json`

Then run or inspect the child demo commands:

- `bash adl/tools/test_demo_v0891_five_agent_hey_jude.sh`
- `bash adl/tools/test_demo_v0891_arxiv_manuscript_workflow.sh`

This packet makes D7, D8, and D9 reviewer-legible together. It does not claim
arXiv submission, final provider-security extension work, or closure of WP-12.
"""
write_text(out_dir / "reviewer_brief.md", reviewer_brief)

dependency_scope = """# Dependency And Scope Notes

## Dependency Truth

WP-13 depends on WP-12 in the planning package. Issue #1933 has landed before
WP-13 publication, so this packet consumes the merged WP-12 convergence surface
rather than closing or replacing it.

## What WP-13 Lands

- D7 integration package status moves from partial to landed.
- D8 five-agent Hey Jude MIDI demo has a runnable command, validation command,
  and reviewer-facing proof surfaces.
- D9 arXiv manuscript workflow packet has a runnable command, validation
  command, three-paper status packet, and no-submission boundary.

## What WP-13 Does Not Land

- release quality gates or release ceremony
- arXiv submission
- full provider-security extension
- replacement of WP-12 issue #1933
"""
write_text(out_dir / "dependency_and_scope.md", dependency_scope)

validation_plan = {
    "schema_version": "adl.v0891.wp13_demo_integration.validation_plan.v1",
    "required_commands": manifest["validation_commands"],
    "leakage_guards": [
        "no absolute host paths in generated WP-13 integration artifacts",
        "no secret-like environment variable names or bearer tokens",
        "no hidden claim that arXiv submission occurred",
    ],
}
write_json(out_dir / "validation_plan.json", validation_plan)

readme = """# v0.89.1 WP-13 Demo Integration Packet

Canonical command:

```bash
bash adl/tools/demo_v0891_wp13_demo_integration.sh
```

Primary proof surfaces:

- `integration_manifest.json`
- `demo_rows.json`
- `reviewer_brief.md`
- `dependency_and_scope.md`
- `validation_plan.json`

This packet integrates the landed D8 and D9 demo work with the D7 reviewer
package without claiming final release closeout or arXiv submission.
"""
write_text(out_dir / "README.md", readme)

print(f"wp13_demo_integration: wrote {out_dir}")
PY

echo "WP-13 demo integration proof surface under the output directory:"
echo "  integration_manifest.json"
echo "  demo_rows.json"
echo "  reviewer_brief.md"
echo "  dependency_and_scope.md"
echo "  validation_plan.json"

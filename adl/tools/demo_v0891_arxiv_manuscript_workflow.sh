#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0891/arxiv_manuscript_workflow}"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

python3 - "$OUT_DIR" <<'PY'
import json
import sys
from pathlib import Path

out_dir = Path(sys.argv[1])
writer_dir = out_dir / "writer_skill_packet"
source_dir = out_dir / "source_packets"
status_dir = out_dir / "manuscript_status"
review_dir = out_dir / "review"

for path in (writer_dir, source_dir, status_dir, review_dir):
    path.mkdir(parents=True, exist_ok=True)


def write_json(path: Path, payload: dict) -> None:
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def write_text(path: Path, text: str) -> None:
    path.write_text(text.strip() + "\n", encoding="utf-8")


paper_specs = [
    {
        "id": "what_is_adl",
        "title": "What Is ADL?",
        "working_claim": "ADL is a contract-first language and runtime for making agent work inspectable, replayable, and governable.",
        "audience": "technical readers who need a crisp system overview",
        "source_refs": [
            "README.md",
            "docs/planning/ADL_FEATURE_LIST.md",
            "docs/milestones/v0.89.1/README.md",
            "docs/architecture/TRACE_SYSTEM_ARCHITECTURE.md",
        ],
        "section_order": [
            "problem",
            "design principles",
            "runtime and trace model",
            "workflow examples",
            "current limits",
            "future work",
        ],
        "known_gaps": [
            "needs final terminology pass after v0.89.1 release truth settles",
            "must keep current-feature claims separated from roadmap claims",
        ],
    },
    {
        "id": "godel_agents_and_adl",
        "title": "Gödel Agents and ADL",
        "working_claim": "ADL provides bounded experiment, evidence, and replay surfaces for self-improvement loops without granting unchecked autonomy.",
        "audience": "agent researchers interested in recursive improvement under governance",
        "source_refs": [
            "docs/adr/0008-godel-stage-loop-v08.md",
            "demos/v0.85/adaptive_godel_loop_demo.md",
            "demos/v0.85/godel_hypothesis_engine_demo.md",
            "adl/src/godel/mod.rs",
        ],
        "section_order": [
            "motivation",
            "bounded self-improvement",
            "experiment records",
            "promotion and replay",
            "safety boundaries",
            "open research questions",
        ],
        "known_gaps": [
            "needs updated examples from the latest convergence artifacts",
            "must avoid claiming autonomous recursive self-modification",
        ],
    },
    {
        "id": "cognitive_spacetime_manifold",
        "title": "Cognitive Spacetime Manifold",
        "working_claim": "ADL's trace, memory, time, and identity-planning surfaces can be described as an inspectable manifold for agent cognition.",
        "audience": "readers interested in higher-level ADL theory and cognitive architecture",
        "source_refs": [
            "docs/architecture/TRACE_SYSTEM_ARCHITECTURE.md",
            "docs/adr/0010-chronosense-substrate.md",
            "docs/milestones/v0.88/features/ADL_COST_MODEL.md",
            "docs/milestones/v0.89.1/FEATURE_DOCS_v0.89.1.md",
        ],
        "section_order": [
            "conceptual frame",
            "trace as spacetime substrate",
            "memory and temporal anchors",
            "cost, attention, and action",
            "identity boundary",
            "research agenda",
        ],
        "known_gaps": [
            "must clearly mark speculative theory versus shipped runtime behavior",
            "should defer full identity claims to the later identity substrate milestone",
        ],
    },
]

role_order = [
    {
        "order": 1,
        "role": "source_curator",
        "responsibility": "select bounded repository sources and reject private notes or hidden chat state",
    },
    {
        "order": 2,
        "role": "outline_architect",
        "responsibility": "map each paper to a stable section order and review path",
    },
    {
        "order": 3,
        "role": "manuscript_drafter",
        "responsibility": "prepare review-ready manuscript packets after the writer skill lands",
    },
    {
        "order": 4,
        "role": "claim_auditor",
        "responsibility": "separate repo-supported claims from roadmap, theory, and future work",
    },
    {
        "order": 5,
        "role": "review_coordinator",
        "responsibility": "record gates, known gaps, and post-milestone submission cleanup",
    },
]

writer_status = {
    "schema_version": "adl.v0891.arxiv_manuscript_workflow.writer_skill_status.v1",
    "skill_name": "arxiv-paper-writer",
    "dependency_issue": "#1929",
    "skill_status": "wp08_contract_defined_packet_only",
    "runnable_in_this_demo": False,
    "execution_truth": "WP-08 issue #1929 defines the bounded arxiv-paper-writer contract; D9 demonstrates the manuscript workflow packet without claiming final writer execution or arXiv submission.",
    "allowed_claim": "The packet proves role order, source packet shape, claim discipline, WP-08 contract alignment, and three-paper status tracking.",
    "forbidden_claims": [
        "final arXiv submission happened",
        "the three papers are submission-ready",
        "a private writer transcript is required to inspect the result",
        "the D9 packet executed a complete manuscript drafting run",
    ],
    "role_order": role_order,
}
write_json(writer_dir / "writer_skill_status.json", writer_status)

workflow_contract = """# Bounded arXiv Paper Writer Workflow Contract

This packet records the D9 manuscript workflow boundary for v0.89.1.

## Boundary Truth

The bounded `arxiv-paper-writer` contract is owned by WP-08 issue #1929. This
demo is packet-only: it proves the manuscript workflow shape and keeps WP-13
publication follow-through separate from the writer-skill contract.

## Role Order

1. source_curator
2. outline_architect
3. manuscript_drafter
4. claim_auditor
5. review_coordinator

## Review Gates

- source packet exists for each paper
- section order is declared before drafting
- claim boundaries are explicit
- review-ready packet is distinguished from final arXiv submission
- post-milestone cleanup is recorded rather than hidden

## Handoff To WP-13

WP-13 should consume the source packets and manuscript status records from this
packet without changing the claim discipline, human review gates, or
no-submission boundary.
"""
write_text(writer_dir / "workflow_contract.md", workflow_contract)

source_manifest = {
    "schema_version": "adl.v0891.arxiv_manuscript_workflow.source_packet_manifest.v1",
    "packet_count": len(paper_specs),
    "privacy_boundary": "repository_relative_public_sources_only",
    "packets": [
        {
            "paper_id": paper["id"],
            "title": paper["title"],
            "packet_ref": f"source_packets/{paper['id']}_source_packet.md",
            "source_refs": paper["source_refs"],
        }
        for paper in paper_specs
    ],
}
write_json(source_dir / "source_packet_manifest.json", source_manifest)

for paper in paper_specs:
    source_lines = [
        f"# Source Packet: {paper['title']}",
        "",
        "## Working Claim",
        "",
        paper["working_claim"],
        "",
        "## Audience",
        "",
        paper["audience"],
        "",
        "## Repository Sources",
        "",
    ]
    source_lines.extend(f"- `{source}`" for source in paper["source_refs"])
    source_lines.extend(
        [
            "",
            "## Section Order",
            "",
        ]
    )
    source_lines.extend(f"{idx}. {section}" for idx, section in enumerate(paper["section_order"], start=1))
    source_lines.extend(
        [
            "",
            "## Known Gaps",
            "",
        ]
    )
    source_lines.extend(f"- {gap}" for gap in paper["known_gaps"])
    source_lines.extend(
        [
            "",
            "## Boundary",
            "",
            "This source packet is review input. It is not a final manuscript and does not represent arXiv submission.",
        ]
    )
    write_text(source_dir / f"{paper['id']}_source_packet.md", "\n".join(source_lines))

status_payload = {
    "schema_version": "adl.v0891.arxiv_manuscript_workflow.three_paper_status.v1",
    "demo_id": "D9",
    "writer_skill_status": "wp08_contract_defined_packet_only",
    "submission_status": "not_submitted",
    "review_ready_meaning": "source and status packets are ready for reviewer inspection; manuscripts are not final arXiv submissions",
    "papers": [
        {
            "paper_id": paper["id"],
            "title": paper["title"],
            "packet_status": "review_packet_ready",
            "draft_status": "not_finalized_until_wp13_manuscript_follow_through",
            "source_packet_ref": f"source_packets/{paper['id']}_source_packet.md",
            "manuscript_status_ref": f"manuscript_status/{paper['id']}_status.md",
            "known_gaps": paper["known_gaps"],
        }
        for paper in paper_specs
    ],
}
write_json(status_dir / "three_paper_status.json", status_payload)

for paper in paper_specs:
    status_lines = [
        f"# Manuscript Status: {paper['title']}",
        "",
        "## Current State",
        "",
        "- Packet status: review_packet_ready",
        "- Draft status: not_finalized_until_wp13_manuscript_follow_through",
        "- Submission status: not_submitted",
        "- Dependency: #1929 WP-08 writer-skill contract plus #1934 WP-13 manuscript follow-through",
        "",
        "## Review-Ready Inputs",
        "",
        f"- Source packet: `source_packets/{paper['id']}_source_packet.md`",
        "- Section order declared before drafting",
        "- Claim boundary documented",
        "",
        "## Known Gaps",
        "",
    ]
    status_lines.extend(f"- {gap}" for gap in paper["known_gaps"])
    status_lines.extend(
        [
            "",
            "## Next Step",
            "",
            "Use the bounded writer contract from #1929 in the WP-13 manuscript follow-through while preserving the human review gates.",
        ]
    )
    write_text(status_dir / f"{paper['id']}_status.md", "\n".join(status_lines))

review_gates = {
    "schema_version": "adl.v0891.arxiv_manuscript_workflow.review_gates.v1",
    "gates": [
        {"gate_id": "source_packets_complete", "status": "pass", "evidence": "source_packets/source_packet_manifest.json"},
        {"gate_id": "role_order_declared", "status": "pass", "evidence": "writer_skill_packet/writer_skill_status.json"},
        {"gate_id": "claim_boundaries_declared", "status": "pass", "evidence": "review/claim_boundaries.md"},
        {"gate_id": "wp08_writer_contract_defined", "status": "pass", "evidence": "writer_skill_packet/workflow_contract.md"},
        {"gate_id": "wp13_manuscript_follow_through", "status": "not_in_scope", "evidence": "#1934 owns final manuscript packet follow-through"},
        {"gate_id": "arxiv_submission", "status": "not_in_scope", "evidence": "review-ready packets are not final arXiv submissions"},
    ],
}
write_json(review_dir / "review_gates.json", review_gates)

claim_boundaries = """# Claim Boundaries

## Supported In This Packet

- D9 has a visible, reviewer-legible proof surface.
- The three-paper slate has bounded source packets.
- The workflow role order and review gates are explicit.
- Review-ready manuscript packets are not final arXiv submissions.
- The WP-08 writer skill contract boundary is recorded instead of hidden.

## Not Claimed

- Final arXiv submission.
- Submission-ready manuscripts.
- Private credentials or hidden chat state.
- Completion of WP-13 manuscript follow-through.
- A full manuscript drafting run inside this D9 packet.
- Full automation of scholarly authorship or peer review.
"""
write_text(review_dir / "claim_boundaries.md", claim_boundaries)

reviewer_brief = """# Reviewer Brief: D9 ArXiv Manuscript Workflow Packet

Review this packet in the following order:

1. `demo_manifest.json`
2. `writer_skill_packet/writer_skill_status.json`
3. `writer_skill_packet/workflow_contract.md`
4. `source_packets/source_packet_manifest.json`
5. `manuscript_status/three_paper_status.json`
6. `review/review_gates.json`
7. `review/claim_boundaries.md`

The packet is intentionally honest about scope. It demonstrates the bounded
manuscript workflow shape and three-paper review packet without claiming that
final papers have been submitted to arXiv or that WP-13 manuscript
follow-through is complete.
"""
write_text(review_dir / "reviewer_brief.md", reviewer_brief)

manifest = {
    "schema_version": "adl.v0891.arxiv_manuscript_workflow_demo.v1",
    "demo_id": "D9",
    "title": "ArXiv manuscript workflow packet",
    "disposition": "proving_packet_only",
    "command": "bash adl/tools/demo_v0891_arxiv_manuscript_workflow.sh",
    "dependency_truth": {
        "wp_08_issue": "#1929",
        "writer_skill_status": "wp08_contract_defined_packet_only",
        "wp_13_issue": "#1934",
    },
    "proof_surfaces": {
        "writer_skill_packet": "writer_skill_packet/writer_skill_status.json",
        "workflow_contract": "writer_skill_packet/workflow_contract.md",
        "source_packet_manifest": "source_packets/source_packet_manifest.json",
        "three_paper_status": "manuscript_status/three_paper_status.json",
        "review_gates": "review/review_gates.json",
        "claim_boundaries": "review/claim_boundaries.md",
        "reviewer_brief": "review/reviewer_brief.md",
    },
    "papers": [{"paper_id": paper["id"], "title": paper["title"]} for paper in paper_specs],
    "security_privacy": {
        "requires_private_credentials": False,
        "requires_hidden_chat_state": False,
        "publishes_local_control_plane_paths": False,
        "submits_to_arxiv": False,
    },
}
write_json(out_dir / "demo_manifest.json", manifest)

readme = """# v0.89.1 Demo D9 - ArXiv Manuscript Workflow Packet

Canonical command:

```bash
bash adl/tools/demo_v0891_arxiv_manuscript_workflow.sh
```

Primary proof surfaces:

- `demo_manifest.json`
- `writer_skill_packet/writer_skill_status.json`
- `writer_skill_packet/workflow_contract.md`
- `source_packets/source_packet_manifest.json`
- `manuscript_status/three_paper_status.json`
- `review/review_gates.json`
- `review/claim_boundaries.md`
- `review/reviewer_brief.md`

This packet is a bounded publication workflow proof. It does not submit to
arXiv, does not require credentials, and does not claim WP-13 manuscript
follow-through is complete.
"""
write_text(out_dir / "README.md", readme)

print(f"arxiv_manuscript_workflow: wrote {out_dir}")
PY

echo "ArXiv manuscript workflow proof surface under the output directory:"
echo "  demo_manifest.json"
echo "  writer_skill_packet/writer_skill_status.json"
echo "  source_packets/source_packet_manifest.json"
echo "  manuscript_status/three_paper_status.json"
echo "  review/review_gates.json"
echo "  review/claim_boundaries.md"

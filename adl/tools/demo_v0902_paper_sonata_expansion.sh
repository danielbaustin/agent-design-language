#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0902/paper_sonata_expansion}"
FIXTURE_DIR="$ROOT_DIR/demos/fixtures/paper_sonata"

artifact_label="custom-artifact-root"
case "$OUT_DIR" in
  "$ROOT_DIR"/*)
    artifact_label="${OUT_DIR#"$ROOT_DIR"/}"
    ;;
esac

for required in \
  "$FIXTURE_DIR/idea_summary.md" \
  "$FIXTURE_DIR/lab_notes.md" \
  "$FIXTURE_DIR/experiment_results.json" \
  "$FIXTURE_DIR/target_venue.md" \
  "$FIXTURE_DIR/citations_seed.json" \
  "$FIXTURE_DIR/paper_constraints.md"; do
  [[ -f "$required" ]] || {
    echo "missing required Paper Sonata fixture: $required" >&2
    exit 1
  }
done

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

python3 - "$OUT_DIR" "$FIXTURE_DIR" "$artifact_label" <<'PY'
from __future__ import annotations

import json
import sys
from pathlib import Path

out = Path(sys.argv[1])
fixture_dir = Path(sys.argv[2])
artifact_label = sys.argv[3]

source_packet = out / "source_packet"
role_outputs = out / "role_outputs"
manuscript = out / "manuscript"
review = out / "review"
revision = out / "revision"
gate = out / "publication_gate"


def read(name: str) -> str:
    return (fixture_dir / name).read_text(encoding="utf-8").strip()


def read_json(name: str) -> dict:
    return json.loads((fixture_dir / name).read_text(encoding="utf-8"))


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text.strip() + "\n", encoding="utf-8")


def write_json(path: Path, payload: dict) -> None:
    write(path, json.dumps(payload, indent=2, sort_keys=True))


idea = read("idea_summary.md")
lab_notes = read("lab_notes.md")
target = read("target_venue.md")
constraints = read("paper_constraints.md")
results = read_json("experiment_results.json")
citations = read_json("citations_seed.json")
metrics = results["metrics"]

for fixture in [
    "idea_summary.md",
    "lab_notes.md",
    "experiment_results.json",
    "target_venue.md",
    "citations_seed.json",
    "paper_constraints.md",
]:
    target_path = source_packet / fixture
    target_path.parent.mkdir(parents=True, exist_ok=True)
    target_path.write_text((fixture_dir / fixture).read_text(encoding="utf-8"), encoding="utf-8")

write_json(
    source_packet / "source_manifest.json",
    {
        "schema_version": "adl.v0902.paper_sonata.source_packet.v1",
        "packet_id": "paper_sonata.synthetic_packet.v1",
        "source_boundary": "All claims in this expansion must trace to the copied fixture packet or to the generated review/revision artifacts.",
        "source_files": [
            "idea_summary.md",
            "lab_notes.md",
            "experiment_results.json",
            "target_venue.md",
            "citations_seed.json",
            "paper_constraints.md",
        ],
        "non_goals": [
            "no autonomous scientific discovery claim",
            "no live-web literature coverage claim",
            "no publication-ready manuscript claim",
            "no external posting or submission",
        ],
    },
)

write_json(
    role_outputs / "conductor_plan.json",
    {
        "schema_version": "adl.v0902.paper_sonata.conductor_plan.v1",
        "role": "conductor",
        "objective": "Expand Paper Sonata into a richer manuscript, review, and revision proof without replacing the delivered v0.88/v0.89 baseline.",
        "stages": [
            {"stage": "source_intake", "artifact": "source_packet/source_manifest.json"},
            {"stage": "scholarly_context", "artifact": "role_outputs/scholar_literature_review.md"},
            {"stage": "results_analysis", "artifact": "role_outputs/analyst_results_summary.md"},
            {"stage": "draft_composition", "artifact": "manuscript/draft.md"},
            {"stage": "editor_review", "artifact": "review/editor_review_notes.md"},
            {"stage": "bounded_revision", "artifact": "revision/revised_manuscript.md"},
            {"stage": "publication_gate", "artifact": "publication_gate/no_submission.md"},
        ],
        "handoff_policy": "Every stage records whether it is source material, generated draft text, review feedback, or revision output.",
    },
)

seed_notes = "\n".join(
    f"- `{entry['citation_id']}`: {entry['theme']} - {entry['note']}" for entry in citations["items"]
)
write(
    role_outputs / "scholar_literature_review.md",
    f"""
# Scholar Literature Review

## Status

Generated scholarly-context text. This is not a live-web literature review and
does not assert bibliographic completeness.

## Source Packet Basis

The source packet asks whether explicit temporal state anchors improve
reproducibility for a bounded multi-agent manuscript assembly workflow.

## Seeded Context Notes

{seed_notes}

## Citation Boundary

The seeded citation packet provides themes only. This demo does not verify DOI
metadata, publication status, venue suitability, or current literature coverage.
Any external manuscript would require a separate citation-quality pass before
submission.
""",
)

write(
    role_outputs / "analyst_results_summary.md",
    f"""
# Analyst Results Summary

## Status

Generated analysis text derived from `source_packet/experiment_results.json`.

## Observed Fixture Metrics

- Dropped handoff fields, baseline: {metrics['dropped_handoff_fields_baseline']}
- Dropped handoff fields, anchored: {metrics['dropped_handoff_fields_anchored']}
- Reviewer repair minutes, baseline: {metrics['reviewer_minutes_baseline']}
- Reviewer repair minutes, anchored: {metrics['reviewer_minutes_anchored']}
- Replay consistency score: {metrics['replay_consistency_score']:.2f}

## Supported Claim

{results['supported_claim']}

## Unsupported Claim

{results['unsupported_claim']}

The unsupported claim remains blocked because the packet is synthetic, bounded,
and designed to prove manuscript workflow shape rather than general scientific
autonomy.
""",
)

write(
    manuscript / "draft.md",
    f"""
# Explicit Anchors For Bounded Manuscript Assembly

## Artifact Type

Generated draft text. This draft is assembled from the Paper Sonata fixture
packet and role outputs. It is not publication-ready.

## Abstract

Paper Sonata explores whether explicit temporal state anchors make a bounded
multi-agent manuscript assembly workflow easier to review and replay. In a
synthetic packet, anchored handoffs preserve the stage information that the
baseline packet drops, reducing reviewer repair time from
{metrics['reviewer_minutes_baseline']} minutes to
{metrics['reviewer_minutes_anchored']} minutes while maintaining a replay
consistency score of {metrics['replay_consistency_score']:.2f}. The result is a
workflow proof, not a claim of autonomous scientific discovery.

## Introduction

The source idea is intentionally narrow:

> {idea}

Paper Sonata treats manuscript writing as a bounded artifact pipeline. The
pipeline is useful only if a reviewer can distinguish source material, generated
draft prose, review feedback, and revision output.

## Method

The workflow uses the source packet, a conductor plan, a scholar context note,
an analyst summary, a composer draft, an editor review, and a bounded revision.
The target venue guidance is:

> {target}

The manuscript obeys the explicit constraints:

> {constraints}

## Results

The fixture data shows that anchoring reduced dropped handoff fields from
{metrics['dropped_handoff_fields_baseline']} to
{metrics['dropped_handoff_fields_anchored']} and reduced reviewer repair time
from {metrics['reviewer_minutes_baseline']} minutes to
{metrics['reviewer_minutes_anchored']} minutes.

## Discussion

The result supports a bounded workflow claim: explicit anchors improve
inspectability for this synthetic manuscript-assembly packet. It does not prove
general scientific autonomy, venue acceptance, citation completeness, or
publication readiness.
""",
)

write(
    review / "editor_review_notes.md",
    """
# Editor Review Notes

## Artifact Type

Review feedback. These notes review `manuscript/draft.md` against the source
packet and publication boundary.

## Findings

### Finding PS-REV-001: The draft should foreground the source/draft/review/revision split

The draft has the right technical frame, but the reviewer needs a more explicit
map of artifact types before reading the manuscript.

### Finding PS-REV-002: The citation boundary needs to stay visible

The scholar note is honest about seed citations, but the manuscript itself
should repeat that there is no live-web literature claim.

### Finding PS-REV-003: The unsupported autonomy claim must remain blocked

The analyst summary correctly blocks broad scientific autonomy. The revised
manuscript should preserve that caveat in the abstract and discussion.

## Non-Findings

- The draft does not claim external submission.
- The draft does not claim venue acceptance.
- The draft does not replace the existing v0.88/v0.89 Paper Sonata baseline.
""",
)

write_json(
    review / "revision_requests.json",
    {
        "schema_version": "adl.v0902.paper_sonata.revision_requests.v1",
        "requests": [
            {
                "id": "ps-rev-001",
                "source": "review/editor_review_notes.md",
                "priority": "high",
                "request": "Add an explicit artifact-type map before the revised manuscript body.",
                "status": "addressed",
                "addressed_by": "revision/revised_manuscript.md",
            },
            {
                "id": "ps-rev-002",
                "source": "review/editor_review_notes.md",
                "priority": "high",
                "request": "Restate the seeded-citation boundary in the revised manuscript.",
                "status": "addressed",
                "addressed_by": "revision/revised_manuscript.md",
            },
            {
                "id": "ps-rev-003",
                "source": "review/editor_review_notes.md",
                "priority": "high",
                "request": "Keep the no-autonomous-discovery caveat visible in the abstract and discussion.",
                "status": "addressed",
                "addressed_by": "revision/revised_manuscript.md",
            },
        ],
    },
)

write(
    revision / "revised_manuscript.md",
    f"""
# Explicit Anchors For Bounded Manuscript Assembly

## Artifact Type

Revision output. This document revises `manuscript/draft.md` in response to
`review/editor_review_notes.md` and `review/revision_requests.json`.

## Reviewer Map

- Source material: `source_packet/`
- Generated role outputs: `role_outputs/`
- Generated draft text: `manuscript/draft.md`
- Review feedback: `review/editor_review_notes.md`
- Revision output: `revision/revised_manuscript.md`
- Publication boundary: `publication_gate/no_submission.md`

## Abstract

Paper Sonata demonstrates a bounded manuscript, review, and revision workflow
for a synthetic research-writing packet. The fixture evidence shows that
explicit temporal anchors reduced dropped handoff fields from
{metrics['dropped_handoff_fields_baseline']} to
{metrics['dropped_handoff_fields_anchored']} and reduced reviewer repair time
from {metrics['reviewer_minutes_baseline']} minutes to
{metrics['reviewer_minutes_anchored']} minutes while preserving a replay
consistency score of {metrics['replay_consistency_score']:.2f}. The claim is
limited to inspectable workflow assembly; it does not claim autonomous
scientific discovery, live-web citation coverage, or publication readiness.

## Method

The workflow starts with a copied source packet, then emits bounded role outputs
for conductor planning, scholarly context, results analysis, draft composition,
editor review, and revision. The seeded citation file supplies themes only, so
the manuscript cannot claim current literature coverage or journal-ready
citations.

## Results

The anchored packet preserves the handoff fields that the baseline drops in
the fixture data. The reviewer-facing improvement is concrete but narrow:
fewer dropped fields and lower repair time for this bounded packet.

## Discussion

This expansion adds the missing review and revision proof layer to the earlier
Paper Sonata baseline. It keeps the baseline intact while making the public
demo easier to inspect: a reviewer can now follow source material into draft
text, review feedback, revision requests, revised output, and a no-submission
gate. The unsupported broad autonomy claim remains blocked.
""",
)

write(
    review / "reviewer_brief.md",
    """
# Reviewer Brief

Review this expansion in order:

1. `source_packet/source_manifest.json`
2. `role_outputs/conductor_plan.json`
3. `manuscript/draft.md`
4. `review/editor_review_notes.md`
5. `review/revision_requests.json`
6. `revision/revised_manuscript.md`
7. `publication_gate/no_submission.md`
8. `run_manifest.json`

The expansion proves a bounded manuscript review/revision surface. It does not
publish, submit, or claim the manuscript is ready for external use.
""",
)

write(
    gate / "no_submission.md",
    """
# Publication Gate

Publication allowed: false

Submission attempted: false

Live-web citation coverage claimed: false

This Paper Sonata expansion is an internal demo artifact. It may be inspected
as a bounded writing, review, and revision proof surface, but it must not be
posted externally or submitted to a venue without a later issue explicitly
authorizing publication work.
""",
)

write_json(
    out / "run_manifest.json",
    {
        "schema_version": "adl.v0902.paper_sonata_expansion.v1",
        "demo_id": "v0902-paper-sonata-expansion",
        "classification": "proving_fixture",
        "artifact_root": artifact_label,
        "baseline_preserved": True,
        "publication_allowed": False,
        "submission_attempted": False,
        "live_web_citations": False,
        "publication_ready_claimed": False,
        "autonomous_scientific_discovery_claimed": False,
        "roles_represented": ["conductor", "scholar", "analyst", "composer", "editor"],
        "artifact_type_map": {
            "source_material": "source_packet/",
            "generated_role_outputs": "role_outputs/",
            "generated_draft_text": "manuscript/draft.md",
            "review_feedback": "review/editor_review_notes.md",
            "revision_requests": "review/revision_requests.json",
            "revision_output": "revision/revised_manuscript.md",
            "publication_boundary": "publication_gate/no_submission.md",
        },
        "required_artifacts": [
            "source_packet/source_manifest.json",
            "role_outputs/conductor_plan.json",
            "role_outputs/scholar_literature_review.md",
            "role_outputs/analyst_results_summary.md",
            "manuscript/draft.md",
            "review/editor_review_notes.md",
            "review/revision_requests.json",
            "review/reviewer_brief.md",
            "revision/revised_manuscript.md",
            "publication_gate/no_submission.md",
            "README.md",
        ],
    },
)

write(
    out / "README.md",
    """
# v0.90.2 Paper Sonata Expansion

This packet expands Paper Sonata with a bounded manuscript, review, and
revision proof layer.

## What This Proves

- source material, generated draft text, review feedback, and revision output
  are separated into inspectable artifacts
- editor review notes produce explicit revision requests
- the revised manuscript addresses those requests without widening into
  publication or autonomous-discovery claims
- the publication gate blocks external posting and submission

## What This Does Not Prove

- publication readiness
- live-web literature coverage
- journal-ready citation quality
- autonomous scientific discovery
- replacement of the delivered v0.88/v0.89 Paper Sonata baseline

## Inspection Path

1. `source_packet/source_manifest.json`
2. `role_outputs/conductor_plan.json`
3. `manuscript/draft.md`
4. `review/editor_review_notes.md`
5. `review/revision_requests.json`
6. `revision/revised_manuscript.md`
7. `publication_gate/no_submission.md`
8. `run_manifest.json`
""",
)
PY

python3 "$ROOT_DIR/adl/tools/validate_v0902_paper_sonata_expansion.py" "$OUT_DIR" >/dev/null
echo "demo_v0902_paper_sonata_expansion: wrote $OUT_DIR"

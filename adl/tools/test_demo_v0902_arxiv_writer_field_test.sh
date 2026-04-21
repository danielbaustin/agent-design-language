#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DEMO_DOC="$ROOT_DIR/demos/v0.90.2/arxiv_writer_field_test_demo.md"
SOURCE_PACKET="$ROOT_DIR/demos/v0.90.2/arxiv_writer_field_test/what_is_adl_source_packet.md"
MANUSCRIPT_PACKET="$ROOT_DIR/demos/v0.90.2/arxiv_writer_field_test/what_is_adl_manuscript_packet.md"

for required in "$DEMO_DOC" "$SOURCE_PACKET" "$MANUSCRIPT_PACKET"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing $required" >&2
    exit 1
  }
done

for heading in \
  "## Metadata" \
  "## Target" \
  "## Packet" \
  "## Claim Boundary Report" \
  "## Citation Gap Report" \
  "## Submission Boundary" \
  "## Follow-up"; do
  grep -Fq "$heading" "$MANUSCRIPT_PACKET" || {
    echo "assertion failed: manuscript packet missing heading $heading" >&2
    exit 1
  }
done

for required_text in \
  "Skill: arxiv-paper-writer" \
  "Mode: \`draft_from_source_packet\`" \
  "Citations Invented: false" \
  "Citation Gaps Present: true" \
  "Submission Attempted: false" \
  "Publication Claimed: false" \
  "Human Author Approval Required: true" \
  "No citation title, author, venue, DOI, arXiv id, or year was invented"; do
  grep -Fq "$required_text" "$MANUSCRIPT_PACKET" || {
    echo "assertion failed: manuscript packet missing required text: $required_text" >&2
    exit 1
  }
done

for label in \
  "SUPPORTED" \
  "NEEDS_CITATION" \
  "NEEDS_EVIDENCE" \
  "AUTHOR_DECISION" \
  "REMOVE_OR_WEAKEN"; do
  grep -Fq "$label" "$MANUSCRIPT_PACKET" || {
    echo "assertion failed: claim label missing: $label" >&2
    exit 1
  }
done

for source_ref in \
  "README.md" \
  "docs/planning/ADL_FEATURE_LIST.md" \
  "docs/milestones/v0.90/README.md" \
  "docs/milestones/v0.90.1/README.md" \
  "demos/v0.89.1/arxiv_manuscript_workflow_demo.md"; do
  [[ -f "$ROOT_DIR/$source_ref" ]] || {
    echo "assertion failed: missing source reference $source_ref" >&2
    exit 1
  }
  grep -Fq "$source_ref" "$SOURCE_PACKET" || {
    echo "assertion failed: source packet does not mention $source_ref" >&2
    exit 1
  }
done

if grep -R -E '/Users/|/private/tmp|/tmp/|Bearer |OPENAI_API_KEY|ANTHROPIC_API_KEY' \
  "$DEMO_DOC" "$SOURCE_PACKET" "$MANUSCRIPT_PACKET" >/dev/null 2>&1; then
  echo "assertion failed: private path or secret-like token leaked into packet" >&2
  exit 1
fi

if grep -Fq "Submitted to arXiv" "$MANUSCRIPT_PACKET"; then
  echo "assertion failed: manuscript packet implies arXiv submission" >&2
  exit 1
fi

echo "demo_v0902_arxiv_writer_field_test: ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_A="$TMPDIR_ROOT/out-a"
OUT_B="$TMPDIR_ROOT/out-b"
TRACKED_REVIEW_DIR="$ROOT_DIR/docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset"
HOST_PATH_PATTERN='/'"Users/"'|/'"private/"'|/'"tmp/"'|[A-Za-z]:\\'

(
  cd "$ROOT_DIR"
  python3 adl/tools/build_v0911_anrm_trace_dataset.py --out-dir "$OUT_A" >/dev/null
  python3 adl/tools/build_v0911_anrm_trace_dataset.py --out-dir "$OUT_B" >/dev/null
)

cmp "$OUT_A/anrm_trace_dataset.json" "$OUT_B/anrm_trace_dataset.json"
cmp "$OUT_A/anrm_trace_extractor_spec.json" "$OUT_B/anrm_trace_extractor_spec.json"
cmp "$OUT_A/anrm_gemma_placement_package.json" "$OUT_B/anrm_gemma_placement_package.json"
cmp "$OUT_A/anrm_trace_dataset_limitations.md" "$OUT_B/anrm_trace_dataset_limitations.md"

cmp "$OUT_A/anrm_trace_dataset.json" \
  "$TRACKED_REVIEW_DIR/anrm_trace_dataset.json"
cmp "$OUT_A/anrm_trace_extractor_spec.json" \
  "$TRACKED_REVIEW_DIR/anrm_trace_extractor_spec.json"
cmp "$OUT_A/anrm_gemma_placement_package.json" \
  "$TRACKED_REVIEW_DIR/anrm_gemma_placement_package.json"
cmp "$OUT_A/anrm_trace_dataset_limitations.md" \
  "$TRACKED_REVIEW_DIR/anrm_trace_dataset_limitations.md"

python3 - "$OUT_A/anrm_trace_dataset.json" "$OUT_A/anrm_trace_extractor_spec.json" "$OUT_A/anrm_gemma_placement_package.json" <<'PY'
import json
import sys
from pathlib import Path

dataset = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
extractor = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
placement = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))

assert dataset["schema_version"] == "adl.anrm_trace_dataset.v1"
assert dataset["fixture_mode"] is True
assert dataset["record_count"] == 10
assert len(dataset["records"]) == 10
assert sorted({record["subject_id"] for record in dataset["records"]}) == ["raw_gemma", "scaffolded_gemma"]
assert sorted({record["case_id"] for record in dataset["records"]}) == ["A", "B", "C", "D", "E"]
expected_pairs = {
    ("A", "raw_gemma"),
    ("A", "scaffolded_gemma"),
    ("B", "raw_gemma"),
    ("B", "scaffolded_gemma"),
    ("C", "raw_gemma"),
    ("C", "scaffolded_gemma"),
    ("D", "raw_gemma"),
    ("D", "scaffolded_gemma"),
    ("E", "raw_gemma"),
    ("E", "scaffolded_gemma"),
}
actual_pairs = {(record["case_id"], record["subject_id"]) for record in dataset["records"]}
assert actual_pairs == expected_pairs
assert len({record["record_id"] for record in dataset["records"]}) == 10
assert all(record["training_eligibility"] == "not_approved" for record in dataset["records"])
assert all(record["dataset_use"] == "fixture_evaluator_only" for record in dataset["records"])

assert extractor["schema_version"] == "adl.anrm_trace_extractor.v1"
assert "event_id" in extractor["required_trace_fields"]
assert "benchmark success" in " ".join(extractor["non_claims"]).lower()

assert placement["schema_version"] == "adl.anrm_gemma_placement.v1"
assert placement["placement_lane"] == "bounded local-model evidence lane"
assert "training runs" in placement["deferred_scope"]
PY

for generated in \
  "$OUT_A/anrm_trace_dataset.json" \
  "$OUT_A/anrm_trace_extractor_spec.json" \
  "$OUT_A/anrm_gemma_placement_package.json" \
  "$OUT_A/anrm_trace_dataset_limitations.md"; do
  if grep -Eq "$HOST_PATH_PATTERN" "$generated"; then
    echo "assertion failed: generated artifact contains host-local path: $generated" >&2
    exit 1
  fi
done

for forbidden in \
  "training success" \
  "benchmark superiority" \
  "runtime dependency"; do
  if ! grep -Fiq "$forbidden" "$OUT_A/anrm_trace_dataset_limitations.md"; then
    echo "assertion failed: limitations report missing expected non-claim: $forbidden" >&2
    exit 1
  fi
done

echo "build_v0911_anrm_trace_dataset: ok"

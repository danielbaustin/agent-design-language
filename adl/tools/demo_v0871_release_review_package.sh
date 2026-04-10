#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/release_review_package}"
MANIFEST="$OUT_DIR/release_review_package_manifest.json"
README="$OUT_DIR/README.md"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

bash "$ROOT_DIR/adl/tools/demo_v0871_docs_review.sh" "$OUT_DIR/docs_review" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_quality_gate.sh" "$OUT_DIR/quality_gate" >/dev/null

python3 - "$ROOT_DIR" "$OUT_DIR" "$MANIFEST" <<'PY'
import json
import os
import sys

root_dir, out_dir, manifest_path = sys.argv[1:4]
payload = {
    "demo_id": "D12",
    "manifest_version": "adl.v0871.release_review_package.v1",
    "package_entries": [
        "docs/milestones/v0.87.1/WBS_v0.87.1.md",
        "docs/milestones/v0.87.1/SPRINT_v0.87.1.md",
        "docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md",
        "docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md",
        "docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md",
        os.path.relpath(os.path.join(out_dir, "docs_review/docs_review_manifest.json"), root_dir),
        os.path.relpath(os.path.join(out_dir, "quality_gate/quality_gate_record.json"), root_dir),
    ],
}
with open(manifest_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

cat >"$README" <<'EOF'
# v0.87.1 Demo D12 - Release Review Package

Canonical command:

```bash
bash adl/tools/demo_v0871_release_review_package.sh
```

Primary proof surface:
- `artifacts/v0871/release_review_package/release_review_package_manifest.json`

Success signal:
- the release review package links the core milestone planning/release docs plus the docs-review and quality-gate walkthrough surfaces from one navigable root
EOF

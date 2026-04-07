#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v087/reviewer_package}"

mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

echo "Running v0.87 reviewer-package demo..."
bash adl/tools/test_repo_review_contract.sh | tee "$OUT_DIR/review_contract_check.txt"

cat >"$OUT_DIR/reviewer_package_manifest.txt" <<'EOF'
docs/milestones/v0.87/README.md
docs/milestones/v0.87/DEMO_MATRIX_v0.87.md
docs/milestones/v0.87/MILESTONE_CHECKLIST_v0.87.md
docs/tooling/review-surface-format.md
docs/tooling/examples/repo-review/good_repo_review.md
docs/tooling/examples/repo-review/bad_repo_review.md
EOF

cat >"$OUT_DIR/README.md" <<EOF
# v0.87 Demo D6 - Reviewer-facing substrate package

Command:

\`\`\`bash
bash adl/tools/demo_v087_reviewer_package.sh
\`\`\`

Primary proof surface:
- \`artifacts/v087/reviewer_package/reviewer_package_manifest.txt\`

Secondary proof surfaces:
- \`artifacts/v087/reviewer_package/review_contract_check.txt\`
- linked reviewer-entry docs and fixture examples listed in the manifest

This demo proves the reviewer package is not just prose: the review-surface
contract validates and the canonical reviewer-entry files are enumerated in one
bounded manifest.
EOF

echo "Demo proof surface:"
echo "  $OUT_DIR/reviewer_package_manifest.txt"
echo "  $OUT_DIR/review_contract_check.txt"


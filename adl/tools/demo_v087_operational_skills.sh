#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v087/skills}"

mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

echo "Running v0.87 operational-skills demo..."
bash adl/tools/test_install_adl_operational_skills.sh | tee "$OUT_DIR/install_check.txt"

find adl/tools/skills -mindepth 1 -maxdepth 1 -type d -exec basename {} \; | LC_ALL=C sort \
  >"$OUT_DIR/skills_inventory.txt"

cat >"$OUT_DIR/README.md" <<EOF
# v0.87 Demo D4 - Operational skills substrate

Command:

\`\`\`bash
bash adl/tools/demo_v087_operational_skills.sh
\`\`\`

Primary proof surface:
- \`artifacts/v087/skills/skills_inventory.txt\`

Secondary proof surfaces:
- \`artifacts/v087/skills/install_check.txt\`
- \`adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md\`

This demo proves the tracked operational-skills bundle is present, installable,
and enumerably reviewable from the in-repo skills root.
EOF

echo "Demo proof surface:"
echo "  $OUT_DIR/skills_inventory.txt"
echo "  $OUT_DIR/install_check.txt"


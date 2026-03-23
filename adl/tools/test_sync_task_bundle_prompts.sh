#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

mkdir -p "$TMP/.git"
mkdir -p "$TMP/.adl/issues/v0.85/bodies"
mkdir -p "$TMP/.adl/cards/918"

cat > "$TMP/.adl/issues/v0.85/bodies/issue-918-v085-output-card-template-tightening.md" <<'EOF'
---
slug: "v085-output-card-template-tightening"
issue_number: 918
---
# Enforce execution-record standard in ADL Output Card Template
EOF

cat > "$TMP/.adl/issues/v0.85/bodies/issue-918-v085-output-card-template-tightening-stub.md" <<'EOF'
---
slug: "v085-output-card-template-tightening"
issue_number: 918
---
# Bootstrap
EOF

cat > "$TMP/.adl/cards/918/input_918.md" <<'EOF'
# ADL Input Card
Task ID: issue-0918
EOF

cat > "$TMP/.adl/cards/918/output_918.md" <<'EOF'
# ADL Output Card
Task ID: issue-0918
EOF

bash "$ROOT/adl/tools/sync_task_bundle_prompts.sh" --root "$TMP" --scope v0.85 >/dev/null

BUNDLE_DIR="$TMP/.adl/v0.85/tasks/issue-0918__v085-output-card-template-tightening"

[[ -d "$BUNDLE_DIR" ]] || { echo "missing bundle dir" >&2; exit 1; }
[[ -f "$BUNDLE_DIR/stp.md" ]] || { echo "missing stp.md" >&2; exit 1; }
[[ -f "$BUNDLE_DIR/stp.stub.md" ]] || { echo "missing stp.stub.md" >&2; exit 1; }
[[ -f "$BUNDLE_DIR/sip.md" ]] || { echo "missing sip.md" >&2; exit 1; }
[[ -f "$BUNDLE_DIR/sor.md" ]] || { echo "missing sor.md" >&2; exit 1; }

grep -q "issue-0918" "$BUNDLE_DIR/sip.md" || { echo "sip content mismatch" >&2; exit 1; }
grep -q "Bootstrap" "$BUNDLE_DIR/stp.stub.md" || { echo "stub content mismatch" >&2; exit 1; }
grep -q "compatibility layer" "$TMP/.adl/v0.85/tasks/README.md" || { echo "missing README guidance" >&2; exit 1; }

echo "test_sync_task_bundle_prompts: PASS"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

REPO="$TMP/repo"
mkdir -p "$REPO/.adl/v0.87.1/bodies" "$REPO/.adl/v0.87.1/tasks/issue-1153__v0-87-1-tools-metadata-parity"

cat >"$REPO/.adl/v0.87.1/bodies/issue-1153-v0-87-1-tools-metadata-parity.md" <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "tools"
slug: "v0-87-1-tools-metadata-parity"
title: "[v0.87.1][tools] Metadata parity"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.87.1"
issue_number: 1153
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes: []
pr_start:
  enabled: false
  slug: "v0-87-1-tools-metadata-parity"
---

# [v0.87.1][tools] Metadata parity
EOF

cat >"$REPO/.adl/v0.87.1/tasks/issue-1153__v0-87-1-tools-metadata-parity/stp.md" <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "tools"
slug: "v0-87-1-tools-metadata-parity"
title: "[v0.87.1][tools] Metadata parity"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.87.1"
issue_number: 1153
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes: []
pr_start:
  enabled: false
  slug: "v0-87-1-tools-metadata-parity"
---

# [v0.87.1][tools] Metadata parity
EOF

cat >"$REPO/.adl/v0.87.1/tasks/issue-1153__v0-87-1-tools-metadata-parity/sor.md" <<'EOF'
# placeholder
EOF

mkdir -p "$REPO/bin"
cat >"$REPO/bin/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
MODE="${GH_MODE:-fail}"
if [[ "$1 $2" == "repo view" ]]; then
  printf 'owner/repo\n'
  exit 0
fi
if [[ "$*" == *"issue view 1153 -R owner/repo --json title,labels"* ]]; then
  if [[ "$MODE" == "pass" ]]; then
    cat <<'JSON'
{"title":"[v0.87.1][tools] Metadata parity","labels":[{"name":"track:roadmap"},{"name":"type:task"},{"name":"area:tools"},{"name":"version:v0.87.1"}]}
JSON
  else
    cat <<'JSON'
{"title":"[tools] Metadata parity","labels":[{"name":"track:roadmap"},{"name":"type:task"},{"name":"area:tools"}]}
JSON
  fi
  exit 0
fi
exit 1
EOF
chmod +x "$REPO/bin/gh"

PATH="$REPO/bin:$PATH" GH_MODE=fail bash "$ROOT/adl/tools/check_issue_metadata_parity.sh" --root "$REPO" --version v0.87.1 --repo owner/repo --help >/dev/null

if PATH="$REPO/bin:$PATH" GH_MODE=fail bash "$ROOT/adl/tools/check_issue_metadata_parity.sh" --root "$REPO" --version v0.87.1 --repo owner/repo >/dev/null 2>&1; then
  echo "expected drift check to fail when version metadata is missing" >&2
  exit 1
fi

PATH="$REPO/bin:$PATH" GH_MODE=pass bash "$ROOT/adl/tools/check_issue_metadata_parity.sh" --root "$REPO" --version v0.87.1 --repo owner/repo >/dev/null

python3 - "$REPO/.adl/v0.87.1/tasks/issue-1153__v0-87-1-tools-metadata-parity/stp.md" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
text = text.replace('wp: "tools"', 'wp: "docs"', 1)
path.write_text(text)
PY

if PATH="$REPO/bin:$PATH" GH_MODE=pass bash "$ROOT/adl/tools/check_issue_metadata_parity.sh" --root "$REPO" --version v0.87.1 --repo owner/repo >/dev/null 2>&1; then
  echo "expected drift check to fail when local STP metadata drifts from the canonical issue prompt" >&2
  exit 1
fi

echo "PASS test_check_issue_metadata_parity"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

REPO="$TMP/repo"
mkdir -p "$REPO/docs/milestones/v0.91.6/features"

cat >"$REPO/README.md" <<'EOF'
# Repo

- [v0.91.6 README](docs/milestones/v0.91.6/README.md)

- Active milestone: v0.91.6
EOF

cat >"$REPO/CHANGELOG.md" <<'EOF'
# Changelog

## v0.91.6 (Active bridge/readiness tranche)
EOF

cat >"$REPO/docs/milestones/v0.91.6/README.md" <<'EOF'
# v0.91.6 Milestone README

## Metadata

- Milestone: `v0.91.6`

## Document Map

- [Feature-doc index](FEATURE_DOCS_v0.91.6.md)
- [Checklist](MILESTONE_CHECKLIST_v0.91.6.md)
- [Release plan](RELEASE_PLAN_v0.91.6.md)
- [Release notes](RELEASE_NOTES_v0.91.6.md)
- [Feature directory index](features/README.md)
EOF

cat >"$REPO/docs/milestones/v0.91.6/FEATURE_DOCS_v0.91.6.md" <<'EOF'
# Feature index

- Milestone: `v0.91.6`

| Feature doc |
| --- |
| [One](features/ONE_v0.91.6.md) |
| [Two](features/TWO_v0.91.6.md) |
EOF

cat >"$REPO/docs/milestones/v0.91.6/features/README.md" <<'EOF'
# Feature directory

- [One](ONE_v0.91.6.md)
- [Two](TWO_v0.91.6.md)
EOF

cat >"$REPO/docs/milestones/v0.91.6/RELEASE_PLAN_v0.91.6.md" <<'EOF'
# Release plan

- Version: `v0.91.6`
EOF

cat >"$REPO/docs/milestones/v0.91.6/RELEASE_NOTES_v0.91.6.md" <<'EOF'
# Release notes

- Version: `v0.91.6`
EOF

cat >"$REPO/docs/milestones/v0.91.6/MILESTONE_CHECKLIST_v0.91.6.md" <<'EOF'
# Checklist

- Version: `v0.91.6`
EOF

cat >"$REPO/docs/milestones/v0.91.6/features/ONE_v0.91.6.md" <<'EOF'
# One
EOF

cat >"$REPO/docs/milestones/v0.91.6/features/TWO_v0.91.6.md" <<'EOF'
# Two
EOF

git -C "$REPO" init -q
git -C "$REPO" config user.name "Test"
git -C "$REPO" config user.email "test@example.com"
git -C "$REPO" add .
git -C "$REPO" commit -q -m "fixture"

python3 "$ROOT/adl/tools/check_repo_quality_staleness.py" --repo-root "$REPO" --milestone v0.91.6 >/dev/null
python3 "$ROOT/adl/tools/check_repo_quality_staleness.py" --repo-root "$REPO" --milestone v0.91.6 --mode release >/dev/null

PLANNED_REPO="$TMP/repo-planned"
cp -R "$REPO" "$PLANNED_REPO"
rm "$PLANNED_REPO/docs/milestones/v0.91.6/FEATURE_DOCS_v0.91.6.md"
python3 - "$PLANNED_REPO/docs/milestones/v0.91.6/README.md" <<'PY'
from pathlib import Path
import sys
path = Path(sys.argv[1])
text = path.read_text()
text = text.replace("- [Feature-doc index](FEATURE_DOCS_v0.91.6.md)\n", "")
path.write_text(text)
PY

python3 "$ROOT/adl/tools/check_repo_quality_staleness.py" --repo-root "$PLANNED_REPO" --milestone v0.91.6 --mode planned >"$TMP/planned-feature-dir.out"
grep -F "PASS repo-quality-staleness milestone uses feature directory index:" "$TMP/planned-feature-dir.out" >/dev/null
grep -F "repo-quality-staleness summary: PASS milestone=v0.91.6 mode=planned deferred=3" "$TMP/planned-feature-dir.out" >/dev/null

python3 "$ROOT/adl/tools/check_repo_quality_staleness.py" --repo-root "$PLANNED_REPO" --milestone v0.91.6 --mode active >"$TMP/active-feature-dir.out"
grep -F "repo-quality-staleness summary: PASS milestone=v0.91.6 mode=active" "$TMP/active-feature-dir.out" >/dev/null

python3 "$ROOT/adl/tools/check_repo_quality_staleness.py" --repo-root "$PLANNED_REPO" --milestone v0.91.6 --mode release >"$TMP/release-feature-dir.out"
grep -F "repo-quality-staleness summary: PASS milestone=v0.91.6 mode=release" "$TMP/release-feature-dir.out" >/dev/null

python3 - "$REPO/README.md" <<'PY'
from pathlib import Path
import sys
path = Path(sys.argv[1])
path.write_text(path.read_text().replace("Active milestone: v0.91.6", "Active milestone: v0.91.5"))
PY

python3 "$ROOT/adl/tools/check_repo_quality_staleness.py" --repo-root "$REPO" --milestone v0.91.6 --mode planned >"$TMP/planned.out"
grep -F "DEFER repo-quality-staleness root README active milestone line deferred until activation: v0.91.6" "$TMP/planned.out" >/dev/null
grep -F "DEFER repo-quality-staleness root README current milestone link deferred until activation: docs/milestones/v0.91.6/README.md" "$TMP/planned.out" >/dev/null
grep -F "DEFER repo-quality-staleness CHANGELOG current milestone heading deferred until activation: ## v0.91.6 (" "$TMP/planned.out" >/dev/null
grep -F "repo-quality-staleness summary: PASS milestone=v0.91.6 mode=planned deferred=3" "$TMP/planned.out" >/dev/null

if python3 "$ROOT/adl/tools/check_repo_quality_staleness.py" --repo-root "$REPO" --milestone v0.91.6 >/dev/null 2>&1; then
  echo "expected stale README milestone check to fail" >&2
  exit 1
fi

git -C "$REPO" checkout -- README.md
python3 - "$REPO/CHANGELOG.md" <<'PY'
from pathlib import Path
import sys
path = Path(sys.argv[1])
path.write_text(path.read_text().replace("## v0.91.6 (Active bridge/readiness tranche)", "## v0.91.5 (Previous milestone)"))
PY

if python3 "$ROOT/adl/tools/check_repo_quality_staleness.py" --repo-root "$REPO" --milestone v0.91.6 --mode release >/dev/null 2>&1; then
  echo "expected release mode stale CHANGELOG check to fail" >&2
  exit 1
fi

git -C "$REPO" checkout -- CHANGELOG.md
mkdir -p "$REPO/docs/__pycache__"
touch "$REPO/docs/__pycache__/bad.pyc"
git -C "$REPO" add docs/__pycache__/bad.pyc
git -C "$REPO" commit -q -m "tracked junk"

if python3 "$ROOT/adl/tools/check_repo_quality_staleness.py" --repo-root "$REPO" --milestone v0.91.6 >/dev/null 2>&1; then
  echo "expected tracked junk check to fail" >&2
  exit 1
fi

echo "PASS test_check_repo_quality_staleness"

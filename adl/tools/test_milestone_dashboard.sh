#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DASHBOARD_DIR="$ROOT_DIR/docs/tooling/milestone-dashboard"
README="$DASHBOARD_DIR/README.md"
HTML="$DASHBOARD_DIR/index.html"
JS="$DASHBOARD_DIR/dashboard.js"
CSS="$DASHBOARD_DIR/style.css"

for path in "$README" "$HTML" "$JS" "$CSS"; do
  if [[ ! -f "$path" ]]; then
    echo "missing dashboard file: $path" >&2
    exit 1
  fi
done

required_terms=(
  "v0.90.2"
  "read-only"
  "active WP wave"
  "PR/check"
  "validation profile"
  "review-tail"
  "release blockers"
  "deferred findings"
  "next operator action"
  "unknown/stale"
)

for term in "${required_terms[@]}"; do
  if ! grep -Riq -- "$term" "$README" "$HTML" "$JS"; then
    echo "dashboard is missing required term: $term" >&2
    exit 1
  fi
done

required_ids=(
  "signal-grid"
  "status-filters"
  "lane-filters"
  "wp-list"
  "authority-list"
  "validation-list"
  "release-blockers"
  "deferred-findings"
)

for id in "${required_ids[@]}"; do
  if ! grep -Fq "id=\"$id\"" "$HTML"; then
    echo "dashboard HTML is missing required id: $id" >&2
    exit 1
  fi
  if ! grep -Fq "byId(\"$id\")" "$JS"; then
    echo "dashboard JS does not render required id: $id" >&2
    exit 1
  fi
done

if grep -Riq -- "v0.88" "$DASHBOARD_DIR"; then
  echo "dashboard still contains stale v0.88 dataset text" >&2
  exit 1
fi

if grep -RIqE -- '(/Users/|/private/var/|/var/folders/|\.worktrees/|BEGIN [A-Z ]*PRIVATE KEY|ghp_[A-Za-z0-9_]+|sk-[A-Za-z0-9])' "$DASHBOARD_DIR"; then
  echo "dashboard contains a private path, worktree path, or secret-looking marker" >&2
  exit 1
fi

if command -v node >/dev/null 2>&1; then
  node --check "$JS" >/dev/null
  node - "$JS" <<'NODE'
const fs = require("fs");
const vm = require("vm");

const jsPath = process.argv[2];
const elements = new Map();

function elementFor(id) {
  if (!elements.has(id)) {
    elements.set(id, {
      id,
      innerHTML: "",
      textContent: "",
      className: "",
      style: {},
      dataset: {},
      querySelectorAll() {
        return [];
      }
    });
  }
  return elements.get(id);
}

const context = {
  document: {
    getElementById: elementFor
  },
  requestAnimationFrame(callback) {
    callback();
  }
};

vm.runInNewContext(fs.readFileSync(jsPath, "utf8"), context, { filename: jsPath });

for (const id of ["signal-grid", "wp-list", "authority-list", "validation-list", "release-blockers", "deferred-findings"]) {
  const element = elements.get(id);
  if (!element || !element.innerHTML.trim()) {
    throw new Error(`dashboard renderer did not populate ${id}`);
  }
}
NODE
fi

echo "PASS test_milestone_dashboard"

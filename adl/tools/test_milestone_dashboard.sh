#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DASHBOARD_DIR="$ROOT_DIR/docs/tooling/milestone-dashboard"
README="$DASHBOARD_DIR/README.md"
HTML="$DASHBOARD_DIR/index.html"
JS="$DASHBOARD_DIR/dashboard.js"
DATA_JS="$DASHBOARD_DIR/data/v0.90.4.js"
CSS="$DASHBOARD_DIR/style.css"

for path in "$README" "$HTML" "$JS" "$DATA_JS" "$CSS"; do
  if [[ ! -f "$path" ]]; then
    echo "missing dashboard file: $path" >&2
    exit 1
  fi
done

required_terms=(
  "Current Dataset"
  "Refresh Rule"
  "read-only"
  "active WP wave"
  "PR/check"
  "validation profile"
  "review-tail"
  "release blockers"
  "deferred findings"
  "next operator action"
  "unknown/stale"
  "snapshot freshness"
  "PR and check state"
  "Workcell operator view"
  "Runtime Observatory"
  "retained/live/unknown/blocked"
)

for term in "${required_terms[@]}"; do
  if ! grep -Riq -- "$term" "$README" "$HTML" "$JS" "$DATA_JS"; then
    echo "dashboard is missing required term: $term" >&2
    exit 1
  fi
done

dashboard_milestone="$(
  sed -n 's/.*milestone: "\(v[0-9][0-9.]*\)".*/\1/p' "$DATA_JS" | head -n 1
)"

if [[ -z "$dashboard_milestone" ]]; then
  echo "dashboard data JS does not declare a milestoneData milestone" >&2
  exit 1
fi

if ! grep -Fq "bundled dataset is \`$dashboard_milestone\`" "$README"; then
  echo "dashboard README Current Dataset does not match JS milestone $dashboard_milestone" >&2
  exit 1
fi

if grep -Riq -- "v0.90.2" "$DASHBOARD_DIR"; then
  echo "dashboard still contains stale v0.90.2 dataset text" >&2
  exit 1
fi

if grep -Riq -- "v0.90.3" "$DASHBOARD_DIR"; then
  echo "dashboard still contains stale v0.90.3 dataset text" >&2
  exit 1
fi

required_ids=(
  "signal-grid"
  "freshness-list"
  "pr-check-list"
  "status-filters"
  "lane-filters"
  "wp-list"
  "review-tail-list"
  "authority-list"
  "validation-list"
  "release-blockers"
  "deferred-findings"
  "workcell-status"
  "workcell-summary"
  "workcell-dependencies"
  "workcell-agents"
  "workcell-authority"
  "runtime-live-status"
  "runtime-observatory-list"
)

for id in "${required_ids[@]}"; do
  if ! grep -Fq "id=\"$id\"" "$HTML"; then
    echo "dashboard HTML is missing required id: $id" >&2
    exit 1
  fi
  if ! grep -Fq "byId(\"$id\")" "$JS" && ! grep -Fq "\"$id\"" "$JS"; then
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
  node --check "$DATA_JS" >/dev/null
  node - "$DATA_JS" "$JS" <<'NODE'
const fs = require("fs");
const vm = require("vm");

const dataPath = process.argv[2];
const jsPath = process.argv[3];
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
  window: {},
  URL,
  URLSearchParams,
  fetch() {
    throw new Error("dashboard should not fetch Runtime Observatory unless live mode is explicitly requested");
  },
  document: {
    getElementById: elementFor
  },
  requestAnimationFrame(callback) {
    callback();
  }
};

vm.runInNewContext(fs.readFileSync(dataPath, "utf8"), context, { filename: dataPath });
vm.runInNewContext(fs.readFileSync(jsPath, "utf8"), context, { filename: jsPath });

for (const id of ["signal-grid", "freshness-list", "pr-check-list", "wp-list", "review-tail-list", "authority-list", "validation-list", "release-blockers", "deferred-findings", "workcell-summary", "workcell-dependencies", "workcell-agents", "workcell-authority", "runtime-live-status", "runtime-observatory-list"]) {
  const element = elements.get(id);
  if (!element || !(String(element.innerHTML) || String(element.textContent)).trim()) {
    throw new Error(`dashboard renderer did not populate ${id}`);
  }
}

const workcell = context.window.milestoneData.workcellOperator;
if (!workcell || workcell.schema !== "adl.workcell.operator.snapshot.v1") {
  throw new Error("workcell operator snapshot schema is missing or unsupported");
}
for (const dependency of workcell.dependencies) {
  if (!["live", "retained", "stale", "unknown", "blocked", "non-authoritative"].includes(dependency.state)) {
    throw new Error(`unsupported workcell dependency state: ${dependency.state}`);
  }
}
const httpConfig = context.window.dashboardInternals.runtimeConfigFromSearch("?runtime=v3&runtimeApiBase=http://runtime.example.test&live=1", workcell);
if (httpConfig.state !== "blocked") {
  throw new Error("Runtime Observatory adapter did not reject non-HTTPS origins");
}
const noOriginConfig = context.window.dashboardInternals.runtimeConfigFromSearch("?runtime=v3&runtimeApiBase=https://runtime.example.test&live=1", workcell);
if (noOriginConfig.state !== "blocked") {
  throw new Error("Runtime Observatory adapter did not reject an empty origin allowlist");
}
const allowlistedWorkcell = {
  ...workcell,
  runtime: {
    ...workcell.runtime,
    allowedOrigins: ["https://runtime.example.test"]
  }
};
const httpsConfig = context.window.dashboardInternals.runtimeConfigFromSearch("?runtime=v3&runtimeApiBase=https://runtime.example.test&live=1", allowlistedWorkcell);
if (httpsConfig.state !== "live" || !httpsConfig.feedUrl.endsWith("/v1/observatory")) {
  throw new Error("Runtime Observatory adapter did not produce the read-only feed URL");
}

workcell.agents = [
  {
    label: "<img src=x onerror=alert(1)>",
    role: "<script>alert(1)</script>",
    state: "live",
    source: "fixture",
    freshness: "fixture"
  }
];
context.window.dashboardInternals.renderWorkcellOperator();
const renderedAgents = elements.get("workcell-agents").innerHTML;
if (renderedAgents.includes("<img") || renderedAgents.includes("<script")) {
  throw new Error("workcell renderer inserted untrusted HTML");
}
for (const element of elements.values()) {
  if (String(element.innerHTML).includes("operator-local-token") || String(element.textContent).includes("operator-local-token")) {
    throw new Error("dashboard rendered a Runtime Observatory token");
  }
}
NODE
fi

echo "PASS test_milestone_dashboard"

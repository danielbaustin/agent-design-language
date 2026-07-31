#!/usr/bin/env bash
set -euo pipefail

ruby .csdlc/prepared/issues/5500/check-dependencies.rb

test -f docs/tooling/milestone-dashboard/index.html
test -f docs/tooling/milestone-dashboard/dashboard.js
test -f docs/tooling/milestone-dashboard/style.css
test -f adl/tools/test_milestone_dashboard.sh

bash adl/tools/test_milestone_dashboard.sh
if command -v node >/dev/null 2>&1; then
  node --check docs/tooling/milestone-dashboard/dashboard.js
fi

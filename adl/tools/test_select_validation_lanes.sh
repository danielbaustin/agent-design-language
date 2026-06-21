#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/select_validation_lanes.sh"
TMP="$(mktemp -d)"
UNTRACKED_FIXTURE="$ROOT/docs/architecture/__selector_untracked_fixture__.md"
trap 'rm -rf "$TMP" "$UNTRACKED_FIXTURE"' EXIT

assert_has() {
  local file="$1"
  local needle="$2"
  if ! grep -F -- "$needle" "$file" >/dev/null; then
    echo "expected $file to contain: $needle" >&2
    echo "actual output:" >&2
    cat "$file" >&2
    exit 1
  fi
}

assert_not_has() {
  local file="$1"
  local needle="$2"
  if grep -F -- "$needle" "$file" >/dev/null; then
    echo "expected $file not to contain: $needle" >&2
    echo "actual output:" >&2
    cat "$file" >&2
    exit 1
  fi
}

docs_only="$TMP/docs-only.txt"
printf 'M\tdocs/milestones/v0.91.6/README.md\n' >"$docs_only"
bash "$SCRIPT" --changed-files "$docs_only" >"$TMP/docs.out"
assert_has "$TMP/docs.out" "aggregate_status=selected"
assert_has "$TMP/docs.out" "docs_diff_check status=selected"
assert_not_has "$TMP/docs.out" "rust_pr_fast"

prompt_template="$TMP/prompt-template.txt"
printf 'M\tdocs/templates/prompts/current.json\n' >"$prompt_template"
bash "$SCRIPT" --changed-files "$prompt_template" >"$TMP/prompt.out"
assert_has "$TMP/prompt.out" "prompt_template_contracts status=selected"
assert_not_has "$TMP/prompt.out" "docs_diff_check status=selected"

focused_rust="$TMP/focused-rust.txt"
printf 'M\tadl/src/runtime_v2/contract_schema.rs\n' >"$focused_rust"
bash "$SCRIPT" --changed-files "$focused_rust" >"$TMP/focused.out"
assert_has "$TMP/focused.out" "rust_pr_fast status=selected"
assert_has "$TMP/focused.out" "mode=focused"
assert_has "$TMP/focused.out" "filter_expression=test(contract_schema)"
assert_not_has "$TMP/focused.out" "runtime_owner_lane status=selected"

focused_rust_with_space="$TMP/focused rust paths.txt"
printf 'M\tadl/src/runtime_v2/contract_schema.rs\n' >"$focused_rust_with_space"
focused_rust_with_space_resolved="$(python3 - <<'PY' "$focused_rust_with_space"
from pathlib import Path
import sys

print(Path(sys.argv[1]).resolve())
PY
)"
bash "$SCRIPT" --changed-files "$focused_rust_with_space" >"$TMP/focused-space.out"
assert_has "$TMP/focused-space.out" "rust_pr_fast status=selected"
assert_has "$TMP/focused-space.out" "--changed-files '$focused_rust_with_space_resolved'"

shared_rust="$TMP/shared-rust.txt"
printf 'M\tadl/src/lib.rs\n' >"$shared_rust"
bash "$SCRIPT" --changed-files "$shared_rust" >"$TMP/shared.out"
assert_has "$TMP/shared.out" "aggregate_status=escalated"
assert_has "$TMP/shared.out" "rust_pr_fast status=escalated"

release_gate="$TMP/release-gate.txt"
printf 'M\t.github/workflows/ci.yaml\n' >"$release_gate"
bash "$SCRIPT" --changed-files "$release_gate" >"$TMP/release.out"
assert_has "$TMP/release.out" "aggregate_status=release_gate_required"
assert_has "$TMP/release.out" "release_gate_review status=release_gate_required"
assert_has "$TMP/release.out" "ci_path_policy_contracts status=selected"

bash "$SCRIPT" --changed-files "$docs_only" --json >"$TMP/docs.json"
python3 - <<'PY' "$TMP/docs.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
docs_lane = plan["lanes"]["docs_diff_check"]
assert docs_lane["owner"] == "docs"
assert docs_lane["default_surface"] == "docs"
assert docs_lane["resource_class"] == "tiny"
assert docs_lane["proof_role"] == "diff_hygiene"
assert docs_lane["vpp_record"]["contract_version"] == "vpp.lane.v1"
assert docs_lane["vpp_record"]["expected_runtime_class"] == "tiny"
assert docs_lane["vpp_record"]["parallel_group"] == "docs_hygiene"
assert docs_lane["vpp_record"]["cache_equivalence_group"] == "git_diff_check"
assert docs_lane["vpp_record"]["failure_semantics"] == "fail_closed"
PY

bash "$SCRIPT" --changed-files "$focused_rust" --json >"$TMP/focused.json"
python3 - <<'PY' "$TMP/focused.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["schema_version"] == "adl.validation_lane_plan.v1"
assert plan["lanes"]["rust_pr_fast"]["mode"] == "focused"
assert plan["lanes"]["rust_pr_fast"]["owner"] == "shared"
assert plan["lanes"]["rust_pr_fast"]["resource_class"] == "medium"
assert plan["lanes"]["rust_pr_fast"]["escalation_rule"] == "delegate_or_escalate"
assert plan["pr_publication_sufficient"] is True
PY

bash "$SCRIPT" --changed-files "$release_gate" --json >"$TMP/release.json"
python3 - <<'PY' "$TMP/release.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
release_gate_lane = plan["lanes"]["release_gate_review"]
ci_policy_lane = plan["lanes"]["ci_path_policy_contracts"]
assert release_gate_lane["proof_role"] == "release_gate"
assert release_gate_lane["resource_class"] == "high"
assert release_gate_lane["escalation_rule"] == "require_release_gate_disposition"
assert ci_policy_lane["proof_role"] == "ci_contract"
assert ci_policy_lane["default_surface"] == "ci_policy"
PY

report="$TMP/report.json"
bash "$SCRIPT" --changed-files "$focused_rust" --json --report-out "$report" >/dev/null
python3 - <<'PY' "$report"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["lanes"]["rust_pr_fast"]["status"] == "selected"
PY

if bash "$SCRIPT" --changed-files "$shared_rust" --run >"$TMP/refuse.out" 2>"$TMP/refuse.err"; then
  echo "expected --run to refuse an escalated plan" >&2
  exit 1
fi
assert_has "$TMP/refuse.err" "refusing --run because the plan is not fully selected"

printf '# selector untracked fixture\n' >"$UNTRACKED_FIXTURE"
bash "$SCRIPT" --include-working-tree >"$TMP/include-working-tree.out"
assert_has "$TMP/include-working-tree.out" "path=docs/architecture/__selector_untracked_fixture__.md"

run_docs="$TMP/run-docs.txt"
printf 'M\tdocs/architecture/VALIDATION_LANE_SELECTOR.md\n' >"$run_docs"
bash "$SCRIPT" --changed-files "$run_docs" --run --report-out "$TMP/run-docs-report.json" >/dev/null
python3 - <<'PY' "$TMP/run-docs-report.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["run_status"] == "passed"
assert plan["lanes"]["docs_diff_check"]["run_status"] == "passed"
PY

invalid_manifest="$TMP/invalid-manifest.json"
cat >"$invalid_manifest" <<'EOF'
{
  "schema_version": "adl.validation_lane_selector.v1",
  "surface_defaults": {
    "docs": {
      "owner": "docs",
      "resource_class": "tiny",
      "determinism_posture": "deterministic",
      "proof_role": "diff_hygiene",
      "risk_class": "low",
      "escalation_rule": "none"
    }
  },
  "lanes": [
    {
      "id": "broken_lane",
      "lane_class": "docs",
      "default_surface": "missing_surface",
      "path_selectors": [
        "docs/**"
      ],
      "command": "git diff --check",
      "reason": "broken"
    }
  ],
  "release_gate_hints": [],
  "rust_path_hints": []
}
EOF
if bash "$SCRIPT" --manifest "$invalid_manifest" --changed-files "$docs_only" >"$TMP/invalid.out" 2>"$TMP/invalid.err"; then
  echo "expected invalid manifest to fail" >&2
  exit 1
fi
assert_has "$TMP/invalid.err" "$invalid_manifest"
assert_has "$TMP/invalid.err" "references unknown default_surface: missing_surface"

invalid_vpp_manifest="$TMP/invalid-vpp-manifest.json"
cat >"$invalid_vpp_manifest" <<'EOF'
{
  "schema_version": "adl.validation_lane_selector.v1",
  "surface_defaults": {
    "docs": {
      "owner": "docs",
      "resource_class": "tiny",
      "determinism_posture": "deterministic",
      "proof_role": "diff_hygiene",
      "risk_class": "low",
      "escalation_rule": "none"
    }
  },
  "lanes": [
    {
      "id": "docs_diff_check",
      "lane_class": "docs",
      "default_surface": "docs",
      "path_selectors": [
        "docs/**"
      ],
      "command": "git diff --check",
      "run_command": "git diff --check",
      "reason": "docs_only_surface_requires_diff_hygiene",
      "vpp_record": {
        "contract_version": "vpp.lane.v1",
        "artifacts": [
          "working_tree_diff_hygiene"
        ],
        "parallel_group": "docs_hygiene",
        "cache_equivalence_group": "git_diff_check",
        "failure_semantics": "fail_closed"
      }
    }
  ],
  "release_gate_hints": [],
  "rust_path_hints": []
}
EOF
if bash "$SCRIPT" --manifest "$invalid_vpp_manifest" --changed-files "$docs_only" >"$TMP/invalid-vpp.out" 2>"$TMP/invalid-vpp.err"; then
  echo "expected invalid vpp manifest to fail" >&2
  exit 1
fi
assert_has "$TMP/invalid-vpp.err" "$invalid_vpp_manifest"
assert_has "$TMP/invalid-vpp.err" "vpp_record missing required key: expected_runtime_class"

invalid_special_surface_manifest="$TMP/invalid-special-surface-manifest.json"
cat >"$invalid_special_surface_manifest" <<'EOF'
{
  "schema_version": "adl.validation_lane_selector.v1",
  "surface_defaults": {
    "docs": {
      "owner": "docs",
      "resource_class": "tiny",
      "determinism_posture": "deterministic",
      "proof_role": "diff_hygiene",
      "risk_class": "low",
      "escalation_rule": "none"
    }
  },
  "lanes": [
    {
      "id": "docs_diff_check",
      "lane_class": "docs",
      "default_surface": "docs",
      "path_selectors": [
        "docs/**"
      ],
      "command": "git diff --check",
      "run_command": "git diff --check",
      "reason": "docs_only_surface_requires_diff_hygiene"
    }
  ],
  "special_surfaces": {
    "release_gate_review": "broken"
  },
  "release_gate_hints": [],
  "rust_path_hints": []
}
EOF
if bash "$SCRIPT" --manifest "$invalid_special_surface_manifest" --changed-files "$docs_only" >"$TMP/invalid-special.out" 2>"$TMP/invalid-special.err"; then
  echo "expected invalid special surface manifest to fail" >&2
  exit 1
fi
assert_has "$TMP/invalid-special.err" "$invalid_special_surface_manifest"
assert_has "$TMP/invalid-special.err" "special_surfaces.release_gate_review must be an object"

special_surface_manifest="$TMP/special-surface-manifest.json"
cat >"$special_surface_manifest" <<'EOF'
{
  "schema_version": "adl.validation_lane_selector.v1",
  "surface_defaults": {
    "docs": {
      "owner": "docs",
      "resource_class": "tiny",
      "determinism_posture": "deterministic",
      "proof_role": "diff_hygiene",
      "risk_class": "low",
      "escalation_rule": "none"
    },
    "shared_rust": {
      "owner": "shared",
      "resource_class": "medium",
      "determinism_posture": "deterministic",
      "proof_role": "regression",
      "risk_class": "medium",
      "escalation_rule": "delegate_or_escalate"
    },
    "release_gate": {
      "owner": "tools",
      "resource_class": "high",
      "determinism_posture": "evidence_bound",
      "proof_role": "release_gate",
      "risk_class": "high",
      "escalation_rule": "require_release_gate_disposition"
    }
  },
  "lanes": [
    {
      "id": "docs_diff_check",
      "lane_class": "docs",
      "default_surface": "docs",
      "path_selectors": [
        "docs/**"
      ],
      "command": "git diff --check",
      "run_command": "git diff --check",
      "reason": "docs_only_surface_requires_diff_hygiene"
    }
  ],
  "special_surfaces": {
    "release_gate_review": {
      "id": "release_gate_review",
      "lane_class": "release_gate",
      "default_surface": "release_gate",
      "path_selectors": [
        "special/release/**"
      ],
      "command": "record release-gate disposition; do not treat focused PR validation as release proof",
      "run_command": "",
      "reason": "special_release_gate_surface"
    },
    "rust_pr_fast": {
      "id": "rust_pr_fast",
      "lane_class": "fast_unit",
      "escalated_lane_class": "release_gate",
      "default_surface": "shared_rust",
      "path_selectors": [
        "special/rust/**"
      ],
      "command": "bash adl/tools/run_pr_fast_test_lane.sh",
      "run_command": "bash adl/tools/run_pr_fast_test_lane.sh",
      "reason": "special_rust_surface"
    }
  },
  "release_gate_hints": [],
  "rust_path_hints": []
}
EOF
special_release="$TMP/special-release.txt"
printf 'M\tspecial/release/packet.md\n' >"$special_release"
bash "$SCRIPT" --manifest "$special_surface_manifest" --changed-files "$special_release" --json >"$TMP/special-release.json"
python3 - <<'PY' "$TMP/special-release.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["aggregate_status"] == "release_gate_required"
assert plan["lanes"]["release_gate_review"]["matched_paths"] == ["special/release/packet.md"]
PY

special_rust="$TMP/special-rust.txt"
printf 'M\tspecial/rust/module.rs\n' >"$special_rust"
bash "$SCRIPT" --manifest "$special_surface_manifest" --changed-files "$special_rust" --json >"$TMP/special-rust.json"
python3 - <<'PY' "$TMP/special-rust.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["lanes"]["rust_pr_fast"]["matched_paths"] == ["special/rust/module.rs"]
PY

missing_metadata_manifest="$TMP/missing-metadata-manifest.json"
cat >"$missing_metadata_manifest" <<'EOF'
{
  "schema_version": "adl.validation_lane_selector.v1",
  "surface_defaults": {
    "docs": {
      "owner": "docs",
      "resource_class": "tiny",
      "determinism_posture": "deterministic",
      "risk_class": "low",
      "escalation_rule": "none"
    }
  },
  "lanes": [
    {
      "id": "docs_diff_check",
      "lane_class": "docs",
      "default_surface": "docs",
      "path_selectors": [
        "docs/**"
      ],
      "command": "git diff --check",
      "run_command": "git diff --check",
      "reason": "docs_only_surface_requires_diff_hygiene"
    }
  ],
  "release_gate_hints": [],
  "rust_path_hints": []
}
EOF
if bash "$SCRIPT" --manifest "$missing_metadata_manifest" --changed-files "$docs_only" >"$TMP/missing-metadata.out" 2>"$TMP/missing-metadata.err"; then
  echo "expected missing surface metadata to fail" >&2
  exit 1
fi
assert_has "$TMP/missing-metadata.err" "missing required surface metadata: proof_role"

echo "PASS test_select_validation_lanes"

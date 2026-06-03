#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

contract="$ROOT_DIR/docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md"
inventory="$ROOT_DIR/docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md"
agents="$ROOT_DIR/AGENTS.md"
route_workflow="$ROOT_DIR/adl/tools/skills/workflow-conductor/scripts/route_workflow.py"
current_registry="$ROOT_DIR/docs/templates/prompts/current.json"
templates_dir="$ROOT_DIR/docs/templates/prompts"
pr_init_template="$ROOT_DIR/docs/templates/PR_INIT_INVOCATION_TEMPLATE.md"

assert_file() {
  local path="$1"
  [[ -f "$path" ]] || {
    echo "missing required file: $path" >&2
    exit 1
  }
}

assert_contains() {
  local path="$1"
  local pattern="$2"
  grep -Fq "$pattern" "$path" || {
    echo "expected '$pattern' in $path" >&2
    exit 1
  }
}

assert_not_contains_regex() {
  local path="$1"
  local pattern="$2"
  if grep -Eq "$pattern" "$path"; then
    echo "unexpected pattern '$pattern' in $path" >&2
    exit 1
  fi
}

assert_file "$contract"
assert_file "$inventory"
assert_file "$agents"
assert_file "$route_workflow"
assert_file "$current_registry"
assert_file "$pr_init_template"

assert_contains "$contract" "Remains canonical for tracked issue work"
assert_contains "$contract" "Continues dispatching built-in lifecycle commands through"
assert_contains "$contract" "Generated-card validation is stricter than terminal shim warnings"
assert_contains "$contract" "ADL_HOME"
assert_contains "$contract" "adl_project.json"

assert_contains "$inventory" "Do not replace before #3597"
assert_contains "$agents" 'use `adl/tools/pr.sh run <issue>`'
assert_contains "$pr_init_template" "It must not continue into:"

python3 - "$route_workflow" <<'PY'
import ast
import pathlib
import sys

source = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
module = ast.parse(source)

commands = None
for node in module.body:
    if isinstance(node, ast.Assign):
        for target in node.targets:
            if isinstance(target, ast.Name) and target.id == "BUILTIN_DISPATCH_COMMANDS":
                commands = ast.literal_eval(node.value)
                break
    if commands is not None:
        break

if commands is None:
    raise SystemExit("BUILTIN_DISPATCH_COMMANDS not found")

expected = {
    "pr-init": ["bash", "adl/tools/pr.sh", "init"],
    "pr-ready": ["bash", "adl/tools/pr.sh", "doctor"],
    "pr-run": ["bash", "adl/tools/pr.sh", "run"],
    "pr-closeout": ["bash", "adl/tools/pr.sh", "closeout"],
}

for key, prefix in expected.items():
    command = commands.get(key)
    if command is None:
        raise SystemExit(f"missing dispatch command for {key}")
    if command[: len(prefix)] != prefix:
        raise SystemExit(f"{key} dispatch changed: expected prefix {prefix}, got {command}")
PY

if grep -R -E 'adl/tools/pr\.sh run [^`[:space:]]+\.adl\.ya?ml|adl pr run [^`[:space:]]+\.adl\.ya?ml' \
  "$templates_dir" \
  "$pr_init_template" >/dev/null 2>&1; then
  echo "generated-card templates must not emit deprecated runtime-through-PR invocations" >&2
  exit 1
fi

if grep -R -E '(^|[^[:alnum:]_-])adl-csdlc issue run[[:space:]]+<issue>|(^|[^[:alnum:]_-])adl-csdlc issue run[[:space:]]+[0-9]+' \
  "$templates_dir" \
  "$agents" \
  "$pr_init_template" >/dev/null 2>&1; then
  echo "agent-facing docs/templates must not teach adl-csdlc issue run as primary before wrapper migration" >&2
  exit 1
fi

assert_not_contains_regex "$current_registry" 'adl-csdlc'

echo "PASS test_cli_wrapper_migration_contract"

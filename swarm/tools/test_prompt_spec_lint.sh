#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
LINT_SCRIPT="$ROOT_DIR/swarm/tools/lint_prompt_spec.sh"
CARD_PATHS_LIB="$ROOT_DIR/swarm/tools/card_paths.sh"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mkdir -p "$repo/swarm/tools" "$repo/.adl/cards/761"
cp "$LINT_SCRIPT" "$repo/swarm/tools/lint_prompt_spec.sh"
cp "$CARD_PATHS_LIB" "$repo/swarm/tools/card_paths.sh"
chmod +x "$repo/swarm/tools/lint_prompt_spec.sh"

cat > "$repo/.adl/cards/761/input_761.md" <<'EOF'
# ADL Input Card

Task ID: issue-0761
Run ID: issue-0761
Version: v0.8
Title: lint-pass
Branch: codex/761-lint-pass

## Prompt Spec
```yaml
prompt_schema: adl.v1
actor:
  role: execution_agent
  name: codex
model:
  id: gpt-5-codex
  determinism_mode: stable
inputs:
  sections:
    - goal
    - acceptance_criteria
    - inputs
    - constraints_policies
    - system_invariants
    - reviewer_checklist
outputs:
  output_card: .adl/cards/761/output_761.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: true
  disallow_secrets: true
  disallow_absolute_host_paths: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
  - card_reviewer_gpt.v1.1
```
EOF

(
  cd "$repo"
  ./swarm/tools/lint_prompt_spec.sh --input .adl/cards/761/input_761.md >/dev/null
)

cat > "$repo/.adl/cards/761/input_invalid_761.md" <<'EOF'
# ADL Input Card

Task ID: issue-0761
Run ID: issue-0761
Version: v0.8
Title: lint-fail
Branch: codex/761-lint-fail

## Prompt Spec
```yaml
prompt_schema: adl.v1
actor:
  role: execution_agent
  name: codex
model:
  id: gpt-5-codex
  determinism_mode: stable
inputs:
  sections:
    - unknown_section
outputs:
  output_card: .adl/cards/761/output_761.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: maybe
  disallow_secrets: true
  disallow_absolute_host_paths: true
review_surfaces:
  - card_review_checklist.v1
```
EOF

(
  cd "$repo"
  set +e
  ./swarm/tools/lint_prompt_spec.sh --input .adl/cards/761/input_invalid_761.md >/dev/null 2>&1
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]]; then
    echo "assertion failed: invalid prompt spec unexpectedly passed lint" >&2
    exit 1
  fi
)

cat > "$repo/.adl/cards/761/input_missing_surface_761.md" <<'EOF'
# ADL Input Card

Task ID: issue-0761
Run ID: issue-0761
Version: v0.8
Title: lint-fail-missing-review-surface
Branch: codex/761-lint-fail

## Prompt Spec
```yaml
prompt_schema: adl.v1
actor:
  role: execution_agent
  name: codex
model:
  id: gpt-5-codex
  determinism_mode: stable
inputs:
  sections:
    - goal
outputs:
  output_card: .adl/cards/761/output_761.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: true
  disallow_secrets: true
  disallow_absolute_host_paths: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
```
EOF

(
  cd "$repo"
  set +e
  ./swarm/tools/lint_prompt_spec.sh --input .adl/cards/761/input_missing_surface_761.md >/dev/null 2>&1
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]]; then
    echo "assertion failed: missing card_reviewer_gpt.v1.1 unexpectedly passed lint" >&2
    exit 1
  fi
)

echo "ok"

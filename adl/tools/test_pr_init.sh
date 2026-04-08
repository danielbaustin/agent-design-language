#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export ADL_TOOLING_MANIFEST_ROOT="$ROOT_DIR"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
PROMPT_LINT_SRC="$ROOT_DIR/adl/tools/lint_prompt_spec.sh"
PROMPT_VALIDATOR_SRC="$ROOT_DIR/adl/tools/validate_structured_prompt.sh"
INPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/input_card_template.md"
OUTPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/output_card_template.md"
STP_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_task_prompt.contract.yaml"
SIP_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_implementation_prompt.contract.yaml"
SOR_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_output_record.contract.yaml"
BASH_BIN="$(command -v bash)"
REAL_ADL_BIN="$ROOT_DIR/adl/target/debug/adl"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

if [[ ! -x "$REAL_ADL_BIN" ]]; then
  cargo build --manifest-path "$ROOT_DIR/adl/Cargo.toml" --bin adl >/dev/null
fi

repo="$tmpdir/repo"
bindir="$tmpdir/bin"
gh_log="$tmpdir/gh.log"
mkdir -p \
  "$repo/adl/tools" \
  "$repo/adl/templates/cards" \
  "$repo/adl/schemas" \
  "$repo/.adl/issues/v0.85/bodies" \
  "$bindir"

cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$PROMPT_LINT_SRC" "$repo/adl/tools/lint_prompt_spec.sh"
cp "$PROMPT_VALIDATOR_SRC" "$repo/adl/tools/validate_structured_prompt.sh"
cp "$INPUT_TPL_SRC" "$repo/adl/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/adl/templates/cards/output_card_template.md"
cp "$STP_CONTRACT_SRC" "$repo/adl/schemas/structured_task_prompt.contract.yaml"
cp "$SIP_CONTRACT_SRC" "$repo/adl/schemas/structured_implementation_prompt.contract.yaml"
cp "$SOR_CONTRACT_SRC" "$repo/adl/schemas/structured_output_record.contract.yaml"
chmod +x "$repo/adl/tools/pr.sh" "$repo/adl/tools/lint_prompt_spec.sh" "$repo/adl/tools/validate_structured_prompt.sh"

cat >"$bindir/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
LOG_FILE="${GH_LOG_FILE:?}"
printf '%s\n' "$*" >>"$LOG_FILE"
if [[ "${1:-}" == "issue" && "${2:-}" == "view" ]]; then
  issue="${3:-}"
  shift 3
  if [[ "$issue" == "43" && "$*" == *"--json title"* && "$*" == *"-q .title"* ]]; then
    echo "[v0.86][WP-03] Generated loop prompt"
    exit 0
  fi
  if [[ "$issue" == "43" && "$*" == *"--json labels"* && "$*" == *"-q .labels[].name"* ]]; then
    printf '%s\n' "track:roadmap" "version:v0.86" "area:docs" "type:design"
    exit 0
  fi
  if [[ "$issue" == "46" && "$*" == *"--json title"* && "$*" == *"-q .title"* ]]; then
    echo "[v0.87.1][tools] Dot suffixed milestone prompt"
    exit 0
  fi
  if [[ "$issue" == "46" && "$*" == *"--json labels"* && "$*" == *"-q .labels[].name"* ]]; then
    printf '%s\n' "track:roadmap" "version:v0.87.1" "area:tools" "type:task"
    exit 0
  fi
fi
exit 1
EOF
chmod +x "$bindir/gh"

cat >"$repo/.adl/issues/v0.85/bodies/issue-42-test-init.md" <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "authoring"
slug: "test-init"
title: "[v0.85][authoring] Test init"
labels:
  - "track:roadmap"
  - "version:v0.85"
issue_number: 42
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Sprint Test"
required_outcome_type:
  - "docs"
repo_inputs:
  - "adl/tools/pr.sh"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes: []
pr_start:
  enabled: true
  slug: "test-init"
---

# Issue Card

## Summary
x
## Goal
x
## Required Outcome
x
## Deliverables
x
## Acceptance Criteria
x
## Repo Inputs
x
## Dependencies
x
## Demo Expectations
x
## Non-goals
x
## Issue-Graph Notes
x
## Notes
x
## Tooling Notes
x
EOF

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git add -A
  git commit -q -m "init"
)

assert_contains() {
  local pattern="$1" text="$2" label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed ($label): expected to find '$pattern'" >&2
    echo "actual output:" >&2
    echo "$text" >&2
    exit 1
  }
}

(
  cd "$repo"
  export PATH="$bindir:$PATH"
  export GH_LOG_FILE="$gh_log"
  export ADL_PR_RUST_BIN="$REAL_ADL_BIN"

  out1="$("$BASH_BIN" adl/tools/pr.sh init 42 --slug test-init --no-fetch-issue --version v0.85)"
  assert_contains "STP      .adl/v0.85/tasks/issue-0042__test-init/stp.md" "$out1" "stp path"
  assert_contains "READ     .adl/v0.85/tasks/issue-0042__test-init/sip.md" "$out1" "read path"
  assert_contains "WRITE    .adl/v0.85/tasks/issue-0042__test-init/sor.md" "$out1" "write path"
  assert_contains "CONTRACT minimum v0.86 init = validated source prompt + root stp/sip/sor bundle" "$out1" "contract line"
  assert_contains "STATE    ISSUE_AND_BUNDLE_READY" "$out1" "state line"

  stp_path="$repo/.adl/v0.85/tasks/issue-0042__test-init/stp.md"
  [[ -f "$stp_path" ]] || {
    echo "assertion failed: expected stp.md to exist" >&2
    exit 1
  }
  [[ -f "$repo/.adl/v0.85/tasks/issue-0042__test-init/sip.md" ]] || {
    echo "assertion failed: expected sip.md to exist" >&2
    exit 1
  }
  [[ -f "$repo/.adl/v0.85/tasks/issue-0042__test-init/sor.md" ]] || {
    echo "assertion failed: expected sor.md to exist" >&2
    exit 1
  }
  [[ -L "$repo/.adl/cards/42/input_42.md" ]] || {
    echo "assertion failed: expected canonical input compatibility link" >&2
    exit 1
  }
  [[ -L "$repo/.adl/cards/42/stp_42.md" ]] || {
    echo "assertion failed: expected canonical stp compatibility link" >&2
    exit 1
  }
  [[ -L "$repo/.adl/cards/42/output_42.md" ]] || {
    echo "assertion failed: expected canonical output compatibility link" >&2
    exit 1
  }
  out2="$("$BASH_BIN" adl/tools/pr.sh init 42 --slug test-init --no-fetch-issue --version v0.85)"
  assert_contains "STATE    ISSUE_AND_BUNDLE_READY" "$out2" "idempotent reuse"

  out3="$("$BASH_BIN" adl/tools/pr.sh init 43 --version v0.86)"
  assert_contains "STP      .adl/v0.86/tasks/issue-0043__v0-86-wp-03-generated-loop-prompt/stp.md" "$out3" "generated stp path"
  assert_contains "READ     .adl/v0.86/tasks/issue-0043__v0-86-wp-03-generated-loop-prompt/sip.md" "$out3" "generated sip path"
  assert_contains "WRITE    .adl/v0.86/tasks/issue-0043__v0-86-wp-03-generated-loop-prompt/sor.md" "$out3" "generated sor path"
  assert_contains "SOURCE   .adl/v0.86/bodies/issue-43-v0-86-wp-03-generated-loop-prompt.md" "$out3" "generated source path"
  [[ -f "$repo/.adl/v0.86/bodies/issue-43-v0-86-wp-03-generated-loop-prompt.md" ]] || {
    echo "assertion failed: expected generated canonical source issue prompt" >&2
    exit 1
  }
  [[ -f "$repo/.adl/v0.86/tasks/issue-0043__v0-86-wp-03-generated-loop-prompt/stp.md" ]] || {
    echo "assertion failed: expected generated task-bundle stp" >&2
    exit 1
  }
  [[ -f "$repo/.adl/v0.86/tasks/issue-0043__v0-86-wp-03-generated-loop-prompt/sip.md" ]] || {
    echo "assertion failed: expected generated task-bundle sip" >&2
    exit 1
  }
  [[ -f "$repo/.adl/v0.86/tasks/issue-0043__v0-86-wp-03-generated-loop-prompt/sor.md" ]] || {
    echo "assertion failed: expected generated task-bundle sor" >&2
    exit 1
  }
  [[ -L "$repo/.adl/cards/43/stp_43.md" ]] || {
    echo "assertion failed: expected generated stp compatibility link" >&2
    exit 1
  }
  [[ -L "$repo/.adl/cards/43/input_43.md" ]] || {
    echo "assertion failed: expected generated input compatibility link" >&2
    exit 1
  }
  [[ -L "$repo/.adl/cards/43/output_43.md" ]] || {
    echo "assertion failed: expected generated output compatibility link" >&2
    exit 1
  }
  grep -Fq 'title: "[v0.86][WP-03] Generated loop prompt"' "$repo/.adl/v0.86/bodies/issue-43-v0-86-wp-03-generated-loop-prompt.md" || {
    echo "assertion failed: expected generated source prompt title" >&2
    exit 1
  }

  out4="$("$BASH_BIN" adl/tools/pr.sh init 46)"
  assert_contains "STP      .adl/v0.87.1/tasks/issue-0046__v0-87-1-tools-dot-suffixed-milestone-prompt/stp.md" "$out4" "dot-suffixed stp path"
  assert_contains "READ     .adl/v0.87.1/tasks/issue-0046__v0-87-1-tools-dot-suffixed-milestone-prompt/sip.md" "$out4" "dot-suffixed sip path"
  assert_contains "WRITE    .adl/v0.87.1/tasks/issue-0046__v0-87-1-tools-dot-suffixed-milestone-prompt/sor.md" "$out4" "dot-suffixed sor path"
  assert_contains "SOURCE   .adl/v0.87.1/bodies/issue-46-v0-87-1-tools-dot-suffixed-milestone-prompt.md" "$out4" "dot-suffixed source path"
  [[ -f "$repo/.adl/v0.87.1/bodies/issue-46-v0-87-1-tools-dot-suffixed-milestone-prompt.md" ]] || {
    echo "assertion failed: expected generated dot-suffixed source issue prompt" >&2
    exit 1
  }
  [[ -f "$repo/.adl/v0.87.1/tasks/issue-0046__v0-87-1-tools-dot-suffixed-milestone-prompt/sip.md" ]] || {
    echo "assertion failed: expected dot-suffixed task-bundle sip" >&2
    exit 1
  }
  grep -Fq "Version: v0.87.1" "$repo/.adl/v0.87.1/tasks/issue-0046__v0-87-1-tools-dot-suffixed-milestone-prompt/sip.md" || {
    echo "assertion failed: expected dot-suffixed version in generated sip" >&2
    exit 1
  }

  cat >"$repo/.adl/issues/v0.85/bodies/issue-44-batch-a.md" <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "authoring"
slug: "batch-a"
title: "[v0.85][authoring] Batch A"
labels:
  - "track:roadmap"
  - "version:v0.85"
issue_number: 44
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Sprint Test"
required_outcome_type:
  - "docs"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes: []
pr_start:
  enabled: true
  slug: "batch-a"
---

# Batch A

## Summary
x
## Goal
x
## Required Outcome
x
## Deliverables
x
## Acceptance Criteria
x
## Repo Inputs
x
## Dependencies
x
## Demo Expectations
x
## Non-goals
x
## Issue-Graph Notes
x
## Notes
x
## Tooling Notes
x
EOF

  cat >"$repo/.adl/issues/v0.85/bodies/issue-45-batch-b.md" <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "authoring"
slug: "batch-b"
title: "[v0.85][authoring] Batch B"
labels:
  - "track:roadmap"
  - "version:v0.85"
issue_number: 45
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Sprint Test"
required_outcome_type:
  - "docs"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes: []
pr_start:
  enabled: true
  slug: "batch-b"
---

# Batch B

## Summary
x
## Goal
x
## Required Outcome
x
## Deliverables
x
## Acceptance Criteria
x
## Repo Inputs
x
## Dependencies
x
## Demo Expectations
x
## Non-goals
x
## Issue-Graph Notes
x
## Notes
x
## Tooling Notes
x
EOF

  "$BASH_BIN" adl/tools/pr.sh init 44 --slug batch-a --no-fetch-issue --version v0.85 >"$tmpdir/init44.out" &
  pid1=$!
  "$BASH_BIN" adl/tools/pr.sh init 45 --slug batch-b --no-fetch-issue --version v0.85 >"$tmpdir/init45.out" &
  pid2=$!
  wait "$pid1"
  wait "$pid2"

  [[ -f "$repo/.adl/v0.85/tasks/issue-0044__batch-a/stp.md" ]] || {
    echo "assertion failed: expected concurrent init for issue 44 to succeed" >&2
    exit 1
  }
  [[ -f "$repo/.adl/v0.85/tasks/issue-0045__batch-b/stp.md" ]] || {
    echo "assertion failed: expected concurrent init for issue 45 to succeed" >&2
    exit 1
  }
)

echo "pr.sh init minimal task-bundle initialization: ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export ADL_TOOLING_MANIFEST_ROOT="$ROOT_DIR"
CARD_PROMPT="$ROOT_DIR/adl/tools/card_prompt.sh"
CARD_PATHS_LIB="$ROOT_DIR/adl/tools/card_paths.sh"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mkdir -p "$repo/adl/tools" "$repo/.adl/cards/701"
cp "$CARD_PROMPT" "$repo/adl/tools/card_prompt.sh"
cp "$CARD_PATHS_LIB" "$repo/adl/tools/card_paths.sh"
chmod +x "$repo/adl/tools/card_prompt.sh"

cat > "$repo/.adl/cards/701/input_701.md" <<'CARD'
# ADL Input Card

Task ID: issue-0701
Run ID: issue-0701
Version: v0.75
Title: parser-smoke
Branch: codex/701-parser-smoke

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
    - acceptance_criteria
    - goal
    - inputs
    - constraints_policies
    - non_goals_out_of_scope
    - notes_risks
outputs:
  output_card: .adl/cards/701/output_701.md
  summary_style: concise_structured
constraints:
  include_system_invariants: false
  include_reviewer_checklist: false
  disallow_secrets: true
  disallow_absolute_host_paths: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
  - card_reviewer_gpt.v1
```

## Goal
Generate deterministic prompt output.

## Acceptance Criteria
- output is stable

## Inputs
- a

## Constraints / Policies
- deterministic: true

## System Invariants (must remain true)
- no hidden state

## Reviewer Checklist (machine-readable hints)
```yaml
determinism_required: true
```

## Non-goals / Out of scope
- none

## Notes / Risks
- low
CARD

(
  cd "$repo"
  out1="$tmpdir/prompt-1.txt"
  out2="$tmpdir/prompt-2.txt"
  ./adl/tools/card_prompt.sh --input "$repo/.adl/cards/701/input_701.md" --out "$out1"
  ./adl/tools/card_prompt.sh --input .adl/cards/701/input_701.md --out "$out2"
  cmp -s "$out1" "$out2"

  acceptance_line="$(awk '/^Acceptance Criteria$/ {print NR; exit}' "$out1")"
  goal_line="$(awk '/^Goal$/ {print NR; exit}' "$out1")"
  if [[ -z "$acceptance_line" || -z "$goal_line" || "$acceptance_line" -ge "$goal_line" ]]; then
    echo "assertion failed: Prompt Spec section ordering was not preserved" >&2
    exit 1
  fi

  if rg -n "System Invariants \\(must remain true\\)" "$out1" >/dev/null; then
    echo "assertion failed: system invariants should be excluded by Prompt Spec constraints" >&2
    exit 1
  fi

  if rg -n "Reviewer Checklist \\(machine-readable hints\\)" "$out1" >/dev/null; then
    echo "assertion failed: reviewer checklist should be excluded by Prompt Spec constraints" >&2
    exit 1
  fi

  if rg -n "/Users/|/home/|[A-Za-z]:\\\\" "$out1" >/dev/null; then
    echo "assertion failed: prompt output leaked absolute host path" >&2
    exit 1
  fi
)

mkdir -p "$repo/.adl/cards/702"
cat > "$repo/.adl/cards/702/input_702.md" <<'CARD'
# ADL Input Card

Task ID: issue-0702
Run ID: issue-0702
Version: v0.75
Title: parser-missing-fields
Branch: codex/702-parser-missing

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
  output_card: .adl/cards/702/output_702.md
  summary_style: concise_structured
constraints:
  include_system_invariants: false
  include_reviewer_checklist: false
  disallow_secrets: true
  disallow_absolute_host_paths: true
review_surfaces:
  - card_review_checklist.v1
```

## Goal
Only goal is present.
CARD

(
  cd "$repo"
  out3="$tmpdir/prompt-3.txt"
  ./adl/tools/card_prompt.sh --input .adl/cards/702/input_702.md --out "$out3"
  rg -n '^Goal$' "$out3" >/dev/null
  if rg -n "\(not provided\)" "$out3" >/dev/null; then
    echo "assertion failed: Prompt Spec requested only Goal section" >&2
    exit 1
  fi
)

echo "ok"

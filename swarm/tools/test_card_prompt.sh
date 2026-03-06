#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD_PROMPT="$ROOT_DIR/swarm/tools/card_prompt.sh"
CARD_PATHS_LIB="$ROOT_DIR/swarm/tools/card_paths.sh"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mkdir -p "$repo/swarm/tools" "$repo/.adl/cards/701"
cp "$CARD_PROMPT" "$repo/swarm/tools/card_prompt.sh"
cp "$CARD_PATHS_LIB" "$repo/swarm/tools/card_paths.sh"
chmod +x "$repo/swarm/tools/card_prompt.sh"

cat > "$repo/.adl/cards/701/input_701.md" <<'EOF'
# ADL Input Card

Task ID: issue-0701
Run ID: issue-0701
Version: v0.75
Title: parser-smoke
Branch: codex/701-parser-smoke

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
EOF

(
  cd "$repo"
  out1="$tmpdir/prompt-1.txt"
  out2="$tmpdir/prompt-2.txt"
  ./swarm/tools/card_prompt.sh --input "$repo/.adl/cards/701/input_701.md" --out "$out1"
  ./swarm/tools/card_prompt.sh --input .adl/cards/701/input_701.md --out "$out2"
  cmp -s "$out1" "$out2"
  rg -n "System Invariants \\(must remain true\\)" "$out1" >/dev/null
  rg -n "Reviewer Checklist \\(machine-readable hints\\)" "$out1" >/dev/null
  if rg -n "/Users/|/home/|[A-Za-z]:\\\\" "$out1" >/dev/null; then
    echo "assertion failed: prompt output leaked absolute host path" >&2
    exit 1
  fi
)

mkdir -p "$repo/.adl/cards/702"
cat > "$repo/.adl/cards/702/input_702.md" <<'EOF'
# ADL Input Card

Task ID: issue-0702
Run ID: issue-0702
Version: v0.75
Title: parser-missing-fields
Branch: codex/702-parser-missing

## Goal
Only goal is present.
EOF

(
  cd "$repo"
  out3="$tmpdir/prompt-3.txt"
  ./swarm/tools/card_prompt.sh --input .adl/cards/702/input_702.md --out "$out3"
  rg -n "\(not provided\)" "$out3" >/dev/null
)

echo "ok"

#!/usr/bin/env bash
set -euo pipefail

card_issue_normalize() {
  local raw="$1"
  if [[ ! "$raw" =~ ^[0-9]+$ ]]; then
    echo "invalid issue number: $raw" >&2
    return 1
  fi
  echo "$((10#$raw))"
}

card_issue_pad() {
  local issue
  issue="$(card_issue_normalize "$1")" || return 1
  printf '%04d' "$issue"
}

card_dir_path() {
  local issue
  issue="$(card_issue_normalize "$1")" || return 1
  echo ".adl/cards/${issue}"
}

card_input_path() {
  local issue
  issue="$(card_issue_normalize "$1")" || return 1
  echo ".adl/cards/${issue}/input_${issue}.md"
}

card_output_path() {
  local issue
  issue="$(card_issue_normalize "$1")" || return 1
  echo ".adl/cards/${issue}/output_${issue}.md"
}

card_legacy_input_path() {
  local issue="$1" ver="${2:-v0.2}"
  local pad
  pad="$(card_issue_pad "$issue")"
  echo ".adl/cards/issue-${pad}__input__${ver}.md"
}

card_legacy_output_path() {
  local issue="$1" ver="${2:-v0.2}"
  local pad
  pad="$(card_issue_pad "$issue")"
  echo ".adl/cards/issue-${pad}__output__${ver}.md"
}

card_first_legacy_path() {
  local kind="$1" issue="$2" ver="${3:-v0.2}"
  local preferred pattern
  local -a matches=()
  local pad
  pad="$(card_issue_pad "$issue")" || return 1

  case "$kind" in
    input)
      preferred="$(card_legacy_input_path "$issue" "$ver")"
      pattern=".adl/cards/issue-${pad}__input__v*.md"
      ;;
    output)
      preferred="$(card_legacy_output_path "$issue" "$ver")"
      pattern=".adl/cards/issue-${pad}__output__v*.md"
      ;;
    *)
      echo "invalid card kind: $kind" >&2
      return 1
      ;;
  esac

  if [[ -e "$preferred" || -L "$preferred" ]]; then
    echo "$preferred"
    return 0
  fi

  shopt -s nullglob
  matches=($pattern)
  shopt -u nullglob
  if [[ ${#matches[@]} -gt 0 ]]; then
    echo "${matches[0]}"
    return 0
  fi

  return 1
}

next_migration_path() {
  local legacy="$1"
  local base out i
  base="$(basename "$legacy")"
  out=".adl/cards/_legacy_migrated/${base}"
  i=0
  while [[ -e "$out" || -L "$out" ]]; do
    i=$((i + 1))
    out=".adl/cards/_legacy_migrated/${base}.${i}"
  done
  echo "$out"
}

ensure_canonical_card_from_legacy() {
  local canonical="$1" legacy="$2"
  local migrated

  mkdir -p "$(dirname "$canonical")"

  if [[ ! -e "$canonical" && ! -L "$canonical" && -f "$legacy" ]]; then
    cp -f "$legacy" "$canonical"
    echo "warning: seeded canonical card from legacy content: $canonical" >&2
  fi

  if [[ -L "$legacy" ]]; then
    rm -f "$legacy"
    echo "warning: removed legacy symlink card path: $legacy" >&2
    return 0
  fi

  if [[ -f "$legacy" ]]; then
    mkdir -p ".adl/cards/_legacy_migrated"
    migrated="$(next_migration_path "$legacy")"
    mv "$legacy" "$migrated"
    echo "warning: migrated legacy root card: $legacy -> $migrated" >&2
  fi
}

resolve_input_card_path() {
  local issue="$1" ver="${2:-v0.2}"
  local p_new p_legacy
  p_new="$(card_input_path "$issue")" || return 1
  p_legacy="$(card_first_legacy_path input "$issue" "$ver" || true)"
  if [[ -n "$p_legacy" ]]; then
    ensure_canonical_card_from_legacy "$p_new" "$p_legacy"
  fi
  echo "$p_new"
}

resolve_output_card_path() {
  local issue="$1" ver="${2:-v0.2}"
  local p_new p_legacy
  p_new="$(card_output_path "$issue")" || return 1
  p_legacy="$(card_first_legacy_path output "$issue" "$ver" || true)"
  if [[ -n "$p_legacy" ]]; then
    ensure_canonical_card_from_legacy "$p_new" "$p_legacy"
  fi
  echo "$p_new"
}

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

resolve_input_card_path() {
  local issue="$1" ver="${2:-v0.2}"
  local p_new p_old p_any
  p_new="$(card_input_path "$issue")" || return 1
  p_old="$(card_legacy_input_path "$issue" "$ver")"
  if [[ -f "$p_new" ]]; then
    echo "$p_new"
    return 0
  fi
  if [[ -f "$p_old" ]]; then
    echo "$p_old"
    return 0
  fi
  p_any="$(ls -1 .adl/cards/issue-$(card_issue_pad "$issue")__input__v*.md 2>/dev/null | head -n1 || true)"
  if [[ -n "$p_any" ]]; then
    echo "$p_any"
    return 0
  fi
  echo "$p_new"
}

resolve_output_card_path() {
  local issue="$1" ver="${2:-v0.2}"
  local p_new p_old p_any
  p_new="$(card_output_path "$issue")" || return 1
  p_old="$(card_legacy_output_path "$issue" "$ver")"
  if [[ -f "$p_new" ]]; then
    echo "$p_new"
    return 0
  fi
  if [[ -f "$p_old" ]]; then
    echo "$p_old"
    return 0
  fi
  p_any="$(ls -1 .adl/cards/issue-$(card_issue_pad "$issue")__output__v*.md 2>/dev/null | head -n1 || true)"
  if [[ -n "$p_any" ]]; then
    echo "$p_any"
    return 0
  fi
  echo "$p_new"
}

sync_legacy_card_link() {
  local canonical="$1" legacy="$2"
  local legacy_dir
  legacy_dir="$(dirname "$legacy")"
  mkdir -p "$legacy_dir"

  if [[ -L "$legacy" ]]; then
    local current
    current="$(readlink "$legacy" 2>/dev/null || true)"
    if [[ "$current" == "$canonical" ]]; then
      return 0
    fi
    rm -f "$legacy"
  elif [[ -e "$legacy" ]]; then
    echo "warning: legacy path exists and is not a symlink, leaving as-is: $legacy" >&2
    return 0
  fi

  if ln -s "$canonical" "$legacy" 2>/dev/null; then
    return 0
  fi

  # Best effort fallback for environments where symlink creation is unavailable.
  if cp -f "$canonical" "$legacy" 2>/dev/null; then
    echo "warning: symlink unavailable; copied legacy card: $legacy" >&2
    return 0
  fi

  echo "warning: failed to create legacy compatibility at: $legacy" >&2
}

#!/usr/bin/env bash
set -euo pipefail

card_issue_pad() {
  printf '%04d' "$1"
}

card_dir_path() {
  local issue="$1"
  local pad
  pad="$(card_issue_pad "$issue")"
  echo ".adl/cards/${pad}"
}

card_input_path() {
  local issue="$1"
  local pad
  pad="$(card_issue_pad "$issue")"
  echo ".adl/cards/${pad}/input_${pad}.md"
}

card_output_path() {
  local issue="$1"
  local pad
  pad="$(card_issue_pad "$issue")"
  echo ".adl/cards/${pad}/output_${pad}.md"
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
  p_new="$(card_input_path "$issue")"
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
  p_new="$(card_output_path "$issue")"
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

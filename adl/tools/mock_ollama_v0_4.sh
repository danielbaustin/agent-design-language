#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" != "run" ]]; then
  echo "mock_ollama_v0_4: expected 'run <model>'" >&2
  exit 2
fi

model="${2:-unknown-model}"
prompt="$(cat)"

if [[ "$prompt" == *"sleep=1"* ]]; then
  sleep 1
fi

hash_cmd=""
if command -v sha256sum >/dev/null 2>&1; then
  hash_cmd="sha256sum"
elif command -v shasum >/dev/null 2>&1; then
  hash_cmd="shasum -a 256"
fi

if [[ "$hash_cmd" == "sha256sum" ]]; then
  prompt_hash="$(printf '%s' "$prompt" | sha256sum | awk '{print $1}')"
elif [[ "$hash_cmd" == "shasum -a 256" ]]; then
  prompt_hash="$(printf '%s' "$prompt" | shasum -a 256 | awk '{print $1}')"
else
  prompt_hash="$(printf '%s' "$prompt" | cksum | awk '{print $1}')"
fi

printf 'MOCK_MODEL=%s\n' "$model"
printf 'MOCK_HASH=%s\n' "$prompt_hash"
printf '%s\n' "$prompt"

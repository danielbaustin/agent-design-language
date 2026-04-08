#!/usr/bin/env bash
set -euo pipefail

provider_demo_repo_root() {
  cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd
}

provider_demo_write_readme() {
  local out_dir="$1"
  local title="$2"
  local canonical_command="$3"
  local primary="$4"
  local secondaries="${5:-}"
  local success_signal="${6:-}"

  mkdir -p "$out_dir"
  {
    printf '# %s\n\n' "$title"
    printf 'Canonical command:\n\n```bash\n%s\n```\n\n' "$canonical_command"
    printf 'Primary proof surface:\n- `%s`\n' "$primary"
    if [[ -n "$secondaries" ]]; then
      printf '\nSecondary proof surfaces:\n'
      while IFS= read -r line; do
        [[ -n "$line" ]] || continue
        printf -- '- `%s`\n' "$line"
      done <<<"$secondaries"
    fi
    if [[ -n "$success_signal" ]]; then
      printf '\nSuccess signal:\n- %s\n' "$success_signal"
    fi
  } >"$out_dir/README.md"
}

provider_demo_print_proof_surfaces() {
  local primary="$1"
  local secondaries="${2:-}"
  echo "Demo proof surface:"
  echo "  $primary"
  if [[ -n "$secondaries" ]]; then
    while IFS= read -r line; do
      [[ -n "$line" ]] || continue
      echo "  $line"
    done <<<"$secondaries"
  fi
}

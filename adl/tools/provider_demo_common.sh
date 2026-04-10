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

provider_demo_archive_trace() {
  local out_dir="$1"
  local run_id="$2"
  local repo_root
  repo_root="$(provider_demo_repo_root)"
  local archive_root="$repo_root/.adl/trace-archive"
  local archive_run="$archive_root/milestones/v0.87.1/runs/$run_id"
  local log="$out_dir/trace_archive.log"

  "$repo_root/adl/tools/archive_run_artifacts.sh" \
    --repo-root "$repo_root" \
    --archive-root "$archive_root" \
    --apply >"$log"

  echo "Canonical trace archive:"
  echo "  $archive_run"
  echo "  $archive_root/MANIFEST.tsv"
  printf '%s\n' "$archive_run" >"$out_dir/trace_archive_path.txt"
}

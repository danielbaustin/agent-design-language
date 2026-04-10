#!/usr/bin/env bash
set -euo pipefail

v0871_demo_repo_root() {
  cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd
}

v0871_demo_write_readme() {
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

v0871_demo_print_proof_surfaces() {
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

v0871_demo_archive_trace() {
  local out_dir="$1"
  local run_id="$2"
  local repo_root
  repo_root="$(v0871_demo_repo_root)"
  local archive_root="$repo_root/.adl/trace-archive"
  local archive_run="$archive_root/milestones/v0.87.1/runs/$run_id"
  local log="$out_dir/trace_archive.log"

  "$repo_root/adl/tools/archive_run_artifacts.sh" \
    --repo-root "$repo_root" \
    --archive-root "$archive_root" \
    --apply >"$log"

  printf '%s\n' "$archive_run" >"$out_dir/trace_archive_path.txt"
}

v0871_demo_run_mock_workflow() {
  local out_dir="$1"
  local example="$2"
  local step_out="$3"
  local log_file="$4"
  local repo_root
  repo_root="$(v0871_demo_repo_root)"
  local runtime_root="$out_dir/runtime"
  local runs_root="$runtime_root/runs"

  mkdir -p "$out_dir" "$step_out"

  (
    cd "$repo_root"
    ADL_RUNTIME_ROOT="$runtime_root" \
    ADL_RUNS_ROOT="$runs_root" \
    ADL_OLLAMA_BIN="$repo_root/adl/tools/mock_ollama_v0_4.sh" \
      bash adl/tools/pr.sh run "$example" \
        --trace \
        --allow-unsigned \
        --out "$step_out" \
        | tee "$log_file"
  )
}

v0871_demo_run_mock_workflow_expect_failure() {
  local out_dir="$1"
  local example="$2"
  local step_out="$3"
  local log_file="$4"
  local repo_root
  repo_root="$(v0871_demo_repo_root)"
  local runtime_root="$out_dir/runtime"
  local runs_root="$runtime_root/runs"

  mkdir -p "$out_dir" "$step_out"

  set +e
  (
    cd "$repo_root"
    ADL_RUNTIME_ROOT="$runtime_root" \
    ADL_RUNS_ROOT="$runs_root" \
    ADL_OLLAMA_BIN="$repo_root/adl/tools/mock_ollama_v0_4.sh" \
      bash adl/tools/pr.sh run "$example" \
        --trace \
        --allow-unsigned \
        --out "$step_out"
  ) >"$log_file" 2>&1
  local rc=$?
  set -e

  if [[ $rc -eq 0 ]]; then
    echo "expected failure for $example" >&2
    return 1
  fi
}

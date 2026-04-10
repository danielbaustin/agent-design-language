#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
archive_run_artifacts.sh -- inventory or copy local ADL run artifacts into a milestone archive

Usage:
  adl/tools/archive_run_artifacts.sh [--repo-root DIR] [--archive-root DIR] [--apply] [--include-worktrees] [--force] [--prune-active-runs]

Defaults:
  --repo-root     repository root containing .adl/runs
  --archive-root  <repo-root>/.adl/trace-archive

Behavior:
  - dry-runs by default
  - copies, never deletes, when --apply is supplied
  - copies only the first discovered run for each milestone + run-id pair
  - writes MANIFEST.tsv and README.md under the archive root
  - organizes copied runs under milestones/<milestone>/runs/<run-id>
  - with --prune-active-runs, moves archived top-level .adl/runs entries into source-roots/ after archiving
USAGE
}

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
archive_root=""
apply=0
include_worktrees=0
force=0
prune_active_runs=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo-root)
      repo_root="$(cd "$2" && pwd)"
      shift 2
      ;;
    --archive-root)
      archive_root="$2"
      shift 2
      ;;
    --apply)
      apply=1
      shift
      ;;
    --include-worktrees)
      include_worktrees=1
      shift
      ;;
    --force)
      force=1
      shift
      ;;
    --prune-active-runs)
      prune_active_runs=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$archive_root" ]]; then
  archive_root="$repo_root/.adl/trace-archive"
fi

if [[ "$prune_active_runs" -eq 1 && "$apply" -ne 1 ]]; then
  echo "--prune-active-runs requires --apply so every source run is archived before it is moved" >&2
  exit 2
fi

case "$archive_root" in
  "$repo_root/.adl/runs"|"$repo_root/.adl/runs"/*)
    echo "--archive-root must not be inside .adl/runs when active runs may be pruned" >&2
    exit 2
    ;;
esac

sanitize_path_part() {
  printf '%s' "$1" | sed 's#[^A-Za-z0-9._-]#-#g; s#--*#-#g; s#^-##; s#-$##'
}

relpath() {
  local path="$1"
  case "$path" in
    "$repo_root"/*) printf '%s' "${path#"$repo_root"/}" ;;
    "$repo_root") printf '.' ;;
    *) printf '%s' "$path" ;;
  esac
}

stat_mtime() {
  local path="$1"
  stat -f '%m' "$path" 2>/dev/null || stat -c '%Y' "$path" 2>/dev/null || printf '0'
}

infer_milestone() {
  local run_id="$1"
  local root="$2"
  local manifest="$root/$run_id/run_manifest.json"

  if [[ -f "$manifest" ]]; then
    local from_manifest
    from_manifest="$(sed -n 's/^[[:space:]]*"milestone"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' "$manifest" | head -1)"
    if [[ -n "$from_manifest" ]]; then
      printf '%s' "$from_manifest"
      return 0
    fi
  fi

  local text="$run_id $root"

  if [[ "$text" =~ v0-87-1|v0871|v0\.87\.1 ]]; then
    printf 'v0.87.1'
  elif [[ "$text" =~ v0-87|v087|v0\.87 ]]; then
    printf 'v0.87'
  elif [[ "$text" =~ v0-86|v086|v0\.86 ]]; then
    printf 'v0.86'
  elif [[ "$text" =~ v0-85|v085|v0\.85 ]]; then
    printf 'v0.85'
  elif [[ "$text" =~ v075|v0-75|v0\.75 ]]; then
    printf 'v0.75'
  elif [[ "$text" =~ v0-7|v07|v0\.7 ]]; then
    printf 'v0.7'
  elif [[ "$text" =~ v0-6|v06|v0\.6 ]]; then
    printf 'v0.6'
  elif [[ "$text" =~ v0-5|v05|v0\.5 ]]; then
    printf 'v0.5'
  elif [[ "$text" =~ v0-4|v04|v0\.4 ]]; then
    printf 'v0.4'
  elif [[ "$text" =~ v0-3|v03|v0\.3 ]]; then
    printf 'v0.3'
  elif [[ "$text" =~ v0-2|v02|v0\.2 ]]; then
    printf 'v0.2'
  else
    printf 'unclassified'
  fi
}

has_run_artifact() {
  local run_dir="$1"
  [[ -f "$run_dir/run.json" || -f "$run_dir/run_summary.json" || -f "$run_dir/run_status.json" || -f "$run_dir/logs/trace_v1.json" ]]
}

source_kind_for_root() {
  local root="$1"
  case "$root" in
    "$repo_root/.adl/runs") printf 'repo-adl-runs' ;;
    "$repo_root/.adl/reports/"*) printf 'report-runs' ;;
    "$repo_root/.worktrees/"*) printf 'worktree-adl-runs' ;;
    "$repo_root/artifacts/"*) printf 'artifacts-runtime-runs' ;;
    *) printf 'external-runs' ;;
  esac
}

discover_roots() {
  local candidate

  candidate="$repo_root/.adl/runs"
  [[ -d "$candidate" ]] && printf '%s\n' "$candidate"

  if [[ -d "$repo_root/.adl/reports" ]]; then
    find "$repo_root/.adl/reports" -path '*/runs' -type d -print 2>/dev/null
  fi

  if [[ -d "$repo_root/artifacts" ]]; then
    find "$repo_root/artifacts" -path '*/runtime/runs' -type d -print 2>/dev/null
  fi

  if [[ "$include_worktrees" -eq 1 && -d "$repo_root/.worktrees" ]]; then
    for candidate in "$repo_root"/.worktrees/*/.adl/runs; do
      [[ -d "$candidate" ]] && printf '%s\n' "$candidate"
    done
    for candidate in "$repo_root"/.worktrees/*/artifacts/*/runtime/runs; do
      [[ -d "$candidate" ]] && printf '%s\n' "$candidate"
    done
  fi
}

copy_run() {
  local src="$1"
  local dest="$2"

  if [[ -e "$dest" && "$force" -ne 1 ]]; then
    return 10
  fi

  if [[ -e "$dest" && "$force" -eq 1 ]]; then
    rm -rf "$dest"
  fi

  mkdir -p "$(dirname "$dest")"
  cp -R "$src" "$dest"
}

write_active_runs_readme() {
  local runs_root="$1"

  cat >"$runs_root/README.md" <<'README'
# Active ADL Runs

This directory is reserved for active/new local runtime runs.

Historical local run artifacts should be archived under:

- `../trace-archive/milestones/`
- `../trace-archive/source-roots/`

Use `adl/tools/archive_run_artifacts.sh --apply --prune-active-runs` from the
repository root to preserve existing runs and clear this active surface.

Do not commit files from `.adl/`.
README
}

prune_active_runs_root() {
  local runs_root="$repo_root/.adl/runs"
  local stamp preserve_root run_dir run_id dest
  active_pruned=0
  active_preserve_root=""

  [[ -d "$runs_root" ]] || return 0

  stamp="$(date -u '+%Y%m%dT%H%M%SZ')"
  preserve_root="$archive_root/source-roots/$stamp/repo-adl-runs-flat"

  while IFS= read -r run_dir; do
    [[ -d "$run_dir" ]] || continue
    if ! has_run_artifact "$run_dir"; then
      continue
    fi

    run_id="$(basename "$run_dir")"
    dest="$preserve_root/$run_id"
    if [[ -e "$dest" ]]; then
      echo "refusing to overwrite existing preserved source run: $(relpath "$dest")" >&2
      exit 1
    fi

    mkdir -p "$preserve_root"
    mv "$run_dir" "$dest"
    active_pruned=$((active_pruned + 1))
  done < <(find "$runs_root" -mindepth 1 -maxdepth 1 -type d -print 2>/dev/null | sort)

  mkdir -p "$runs_root"
  write_active_runs_readme "$runs_root"

  if [[ "$active_pruned" -gt 0 ]]; then
    active_preserve_root="$(relpath "$preserve_root")"
  fi
}

tmp_manifest="$(mktemp "${TMPDIR:-/tmp}/adl-run-archive.XXXXXX")"
tmp_seen="$(mktemp "${TMPDIR:-/tmp}/adl-run-archive-seen.XXXXXX")"
trap 'rm -f "$tmp_manifest" "$tmp_seen"' EXIT

printf 'source_root\tsource_kind\trun_id\tmilestone\tarchive_path\tstatus\tknown_artifact_count\thas_run_summary\thas_run_manifest\thas_trace\tmtime_epoch\n' >"$tmp_manifest"

seen=0
copied=0
skipped=0
existing=0
duplicates=0
roots=0
active_pruned=0
active_preserve_root=""

while IFS= read -r runs_root; do
  [[ -n "$runs_root" && -d "$runs_root" ]] || continue
  roots=$((roots + 1))
  source_kind="$(source_kind_for_root "$runs_root")"
  while IFS= read -r run_dir; do
    [[ -d "$run_dir" ]] || continue
    if ! has_run_artifact "$run_dir"; then
      continue
    fi

    seen=$((seen + 1))
    run_id="$(basename "$run_dir")"
    milestone="$(infer_milestone "$run_id" "$runs_root")"
    dest="$archive_root/milestones/$milestone/runs/$run_id"
    key="$milestone	$run_id"

    status="would-copy"
    if grep -Fqx "$key" "$tmp_seen"; then
      duplicates=$((duplicates + 1))
      status="duplicate-skipped"
    else
      printf '%s\n' "$key" >>"$tmp_seen"
      if [[ "$apply" -ne 1 ]]; then
        skipped=$((skipped + 1))
      elif [[ -e "$dest" && "$force" -ne 1 ]]; then
        existing=$((existing + 1))
        status="already-archived"
      elif copy_run "$run_dir" "$dest"; then
        {
          printf 'source_root=%s\n' "$(relpath "$runs_root")"
          printf 'source_run=%s\n' "$(relpath "$run_dir")"
          printf 'source_kind=%s\n' "$source_kind"
          printf 'milestone=%s\n' "$milestone"
        } >"$dest/ARCHIVE_SOURCE.txt"
        copied=$((copied + 1))
        status="copied"
      else
        existing=$((existing + 1))
        status="exists"
      fi
    fi

    known_artifact_count=0
    has_summary="no"
    has_manifest="no"
    has_trace="no"
    [[ -f "$run_dir/run.json" ]] && known_artifact_count=$((known_artifact_count + 1))
    [[ -f "$run_dir/run_status.json" ]] && known_artifact_count=$((known_artifact_count + 1))
    [[ -f "$run_dir/steps.json" ]] && known_artifact_count=$((known_artifact_count + 1))
    if [[ -f "$run_dir/run_manifest.json" ]]; then
      has_manifest="yes"
      known_artifact_count=$((known_artifact_count + 1))
    fi
    if [[ -f "$run_dir/run_summary.json" ]]; then
      has_summary="yes"
      known_artifact_count=$((known_artifact_count + 1))
    fi
    if [[ -f "$run_dir/logs/trace_v1.json" ]]; then
      has_trace="yes"
      known_artifact_count=$((known_artifact_count + 1))
    fi
    mtime="$(stat_mtime "$run_dir")"

    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
      "$(relpath "$runs_root")" \
      "$source_kind" \
      "$run_id" \
      "$milestone" \
      "$(relpath "$dest")" \
      "$status" \
      "$known_artifact_count" \
      "$has_summary" \
      "$has_manifest" \
      "$has_trace" \
      "$mtime" >>"$tmp_manifest"
  done < <(find "$runs_root" -mindepth 1 -maxdepth 1 -type d -print 2>/dev/null | sort)
done < <(discover_roots | awk '!seen[$0]++')

mkdir -p "$archive_root"
cp "$tmp_manifest" "$archive_root/MANIFEST.tsv"
unique_candidates="$(wc -l <"$tmp_seen" | tr -d ' ')"

if [[ "$prune_active_runs" -eq 1 ]]; then
  prune_active_runs_root
fi

{
  printf '# ADL Trace Archive\n\n'
  printf 'Generated by: `adl/tools/archive_run_artifacts.sh`\n\n'
  printf '%s\n' "- Mode: \`$([[ "$apply" -eq 1 ]] && printf apply || printf dry-run)\`"
  printf '%s\n' "- Roots inspected: \`$roots\`"
  printf '%s\n' "- Runs discovered: \`$seen\`"
  printf '%s\n' "- Unique archive candidates: \`$unique_candidates\`"
  printf '%s\n' "- Runs copied: \`$copied\`"
  printf '%s\n' "- Already archived destinations: \`$existing\`"
  printf '%s\n' "- Unique dry-run entries: \`$skipped\`"
  printf '%s\n\n' "- Duplicate source entries skipped: \`$duplicates\`"
  printf '%s\n' "- Active .adl/runs entries moved to source-roots: \`$active_pruned\`"
  if [[ -n "$active_preserve_root" ]]; then
    printf '%s\n' "- Active .adl/runs preserved at: \`$active_preserve_root\`"
  fi
  printf '\n'
  printf '## Unique Archive Counts\n\n'
  awk -F '\t' 'NR > 1 && $6 != "duplicate-skipped" {count[$4]++} END {for (m in count) print "- `" m "`: `" count[m] "`"}' "$tmp_manifest" | sort
  printf '\n## Source Entry Counts\n\n'
  awk -F '\t' 'NR > 1 {count[$4]++} END {for (m in count) print "- `" m "`: `" count[m] "`"}' "$tmp_manifest" | sort
  printf '\n## Files\n\n'
  printf '%s\n' '- `MANIFEST.tsv`: run-level source, milestone, archive path, and artifact-presence inventory'
  printf '%s\n' '- `milestones/<milestone>/runs/<run-id>/`: copied run artifacts when `--apply` is used'
  printf '%s\n' '- `run_manifest.json`: preferred source for milestone/provenance classification when present'
  printf '\nThis tool copies run artifacts by default. With `--apply --prune-active-runs`, it moves top-level `.adl/runs/<run-id>` source directories into `source-roots/` after archiving so `.adl/runs/` returns to an active/new-run surface without deleting data.\n'
  printf 'When duplicate `milestone + run-id` entries exist, the first discovered copy is selected and later sources remain recorded as `duplicate-skipped` in `MANIFEST.tsv`.\n'
} >"$archive_root/README.md"

echo "trace archive root: $(relpath "$archive_root")"
echo "manifest: $(relpath "$archive_root/MANIFEST.tsv")"
echo "runs discovered: $seen"
echo "runs copied: $copied"
echo "duplicate sources skipped: $duplicates"
echo "active .adl/runs entries moved: $active_pruned"
if [[ -n "$active_preserve_root" ]]; then
  echo "active .adl/runs preserved at: $active_preserve_root"
fi
echo "mode: $([[ "$apply" -eq 1 ]] && printf apply || printf dry-run)"

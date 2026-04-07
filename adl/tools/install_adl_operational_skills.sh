#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "${repo_root}" ]]; then
  echo "install_adl_operational_skills.sh: must be run from inside the repo" >&2
  exit 1
fi

codex_home="${CODEX_HOME:-$HOME/.codex}"
source_root="${repo_root}/adl/tools/skills"
dest_root="${codex_home}/skills"

if [[ ! -d "${source_root}" ]]; then
  echo "install_adl_operational_skills.sh: source skills root missing: ${source_root}" >&2
  exit 1
fi

mkdir -p "${dest_root}"

installed_any=0
for source_dir in "${source_root}"/*; do
  [[ -d "${source_dir}" ]] || continue
  [[ -f "${source_dir}/SKILL.md" ]] || continue
  skill_name="$(basename "${source_dir}")"
  dest_dir="${dest_root}/${skill_name}"

  rm -rf "${dest_dir}"
  mkdir -p "${dest_dir}"
  cp -R "${source_dir}/." "${dest_dir}/"
  if ! diff -qr "${source_dir}" "${dest_dir}" >/dev/null; then
    echo "install_adl_operational_skills.sh: install verification failed for ${dest_dir}" >&2
    exit 1
  fi

  installed_any=1
  echo "INSTALLED ${dest_dir}"
done

if [[ "${installed_any}" -ne 1 ]]; then
  echo "install_adl_operational_skills.sh: no skill bundles found in ${source_root}" >&2
  exit 1
fi

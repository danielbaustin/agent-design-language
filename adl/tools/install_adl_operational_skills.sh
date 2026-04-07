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
install_mode="${ADL_OPERATIONAL_SKILLS_INSTALL_MODE:-copy}"

if [[ ! -d "${source_root}" ]]; then
  echo "install_adl_operational_skills.sh: source skills root missing: ${source_root}" >&2
  exit 1
fi

case "${install_mode}" in
  copy|symlink)
    ;;
  *)
    echo "install_adl_operational_skills.sh: unsupported install mode: ${install_mode} (expected copy or symlink)" >&2
    exit 1
    ;;
esac

mkdir -p "${dest_root}"

installed_any=0
for source_dir in "${source_root}"/*; do
  [[ -d "${source_dir}" ]] || continue
  [[ -f "${source_dir}/SKILL.md" ]] || continue
  skill_name="$(basename "${source_dir}")"
  dest_dir="${dest_root}/${skill_name}"

  rm -rf "${dest_dir}"
  case "${install_mode}" in
    copy)
      mkdir -p "${dest_dir}"
      cp -R "${source_dir}/." "${dest_dir}/"
      if ! diff -qr "${source_dir}" "${dest_dir}" >/dev/null; then
        echo "install_adl_operational_skills.sh: install verification failed for ${dest_dir}" >&2
        exit 1
      fi
      ;;
    symlink)
      ln -s "${source_dir}" "${dest_dir}"
      if [[ ! -L "${dest_dir}" ]]; then
        echo "install_adl_operational_skills.sh: expected symlink install for ${dest_dir}" >&2
        exit 1
      fi
      if [[ "$(cd "${dest_dir}" && pwd -P)" != "$(cd "${source_dir}" && pwd -P)" ]]; then
        echo "install_adl_operational_skills.sh: symlink verification failed for ${dest_dir}" >&2
        exit 1
      fi
      ;;
  esac

  installed_any=1
  echo "INSTALLED ${dest_dir} (${install_mode})"
done

if [[ "${installed_any}" -ne 1 ]]; then
  echo "install_adl_operational_skills.sh: no skill bundles found in ${source_root}" >&2
  exit 1
fi

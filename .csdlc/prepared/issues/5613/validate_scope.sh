#!/usr/bin/env bash
set -euo pipefail

root="${1:-.}"
base="${ADL_5613_SCOPE_BASE:-origin/main}"

allowed='^(\.csdlc/(issues/(5337|5339|5358|5591|5602|5613)(/.*)?|prepared/issues/5613(/.*)?|evidence/(5591|5613)(/.*)?|locks/5613\.lock)|csdlc-v2/(src/(cards|lib|model|store|schema)\.rs|src/bin/csdlc-closeout\.rs|operator/skills/csdlc-v2-closeout/SKILL\.md|tests/gate7_terminal_sor_validation_repair_5613\.rs))$'

unexpected="$(git -C "$root" diff --name-only "$base"...HEAD | rg -v "$allowed" || true)"
if [[ -n "$unexpected" ]]; then
  printf 'issue 5613 scope contains unexpected paths:\n%s\n' "$unexpected" >&2
  exit 1
fi

for manifest in Cargo.toml Cargo.lock csdlc-v2/Cargo.toml csdlc-v2/Cargo.lock; do
  if ! git -C "$root" diff --quiet "$base"...HEAD -- "$manifest"; then
    echo "issue 5613 may not change dependency manifests: $manifest" >&2
    exit 1
  fi
done

if git -C "$root" diff --name-only "$base"...HEAD | rg -q '^(adl-runtime|adl-runtime-kernel|adl-v2|infra/runtime-v3|\.github)/'; then
  echo "issue 5613 contains forbidden Runtime, ADL-v2, infra, or CI scope" >&2
  exit 1
fi

printf 'issue 5613 exact scope and dependency contract: pass\n'

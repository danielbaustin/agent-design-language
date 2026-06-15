#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

inventory="docs/milestones/v0.91.5/review/SHELL_WRAPPER_INVENTORY_3713.tsv"
[[ -f "$inventory" ]]

python3 - <<'PY'
from pathlib import Path
import sys

inventory = Path('docs/milestones/v0.91.5/review/SHELL_WRAPPER_INVENTORY_3713.tsv')
valid = {
    'delegated_to_adl_csdlc',
    'local_only_utility',
    'explicit_fail_closed_gap',
    'scheduled_for_removal',
}
lines = inventory.read_text().splitlines()
if not lines or lines[0] != 'path\tstatus\trationale':
    raise SystemExit('inventory header mismatch')
rows = []
for lineno, line in enumerate(lines[1:], start=2):
    parts = line.split('\t')
    if len(parts) != 3:
        raise SystemExit(f'line {lineno}: expected 3 tab-separated fields')
    path, status, rationale = parts
    if status not in valid:
        raise SystemExit(f'line {lineno}: invalid status {status!r}')
    if not rationale.strip():
        raise SystemExit(f'line {lineno}: empty rationale')
    rows.append((path, status, rationale))

paths = [path for path, _, _ in rows]
if len(paths) != len(set(paths)):
    seen = set()
    dupes = sorted({p for p in paths if p in seen or seen.add(p)})
    raise SystemExit(f'duplicate inventory paths: {dupes}')

expected = sorted(p.as_posix() for p in Path('adl/tools').glob('*.sh'))
actual = sorted(paths)
missing = sorted(set(expected) - set(actual))
extra = sorted(set(actual) - set(expected))
if missing or extra:
    raise SystemExit(f'inventory mismatch missing={missing} extra={extra}')

for path in actual:
    if not Path(path).is_file():
        raise SystemExit(f'inventory path does not exist: {path}')

print(f'PASS shell wrapper inventory covers {len(actual)} wrappers')
PY

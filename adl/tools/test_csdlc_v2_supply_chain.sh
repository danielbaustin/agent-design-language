#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
mkdir -p "$ROOT_DIR/.adl/tmp"
TMP_DIR="$(mktemp -d "$ROOT_DIR/.adl/tmp/csdlc-supply-chain.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

output="$(bash "$ROOT_DIR/adl/tools/validate_csdlc_v2_supply_chain.sh" "$TMP_DIR/proof.json")"
python3 - "$output" <<'PY'
import json
import sys
from pathlib import Path

proof = json.loads(Path(sys.argv[1]).read_text())
assert proof["schema"] == "adl.csdlc_v2.release_supply_chain_proof.v1"
assert proof["issue"] == 5546
assert proof["locked_dependency_metadata"]["status"] == "passed"
assert proof["lockfile"]["status"] == "passed"
assert proof["msrv"]["declared"] == "1.85"
assert proof["overall"] in {"passed", "partial_with_explicit_dispositions"}
if proof["overall"] != "passed":
    assert proof["release_gate"] == "not_ready_for_supply_chain_certification"
    assert proof["explicit_dispositions"]
    if proof["msrv"]["proof"]["status"] == "unavailable":
        assert "msrv" in proof["explicit_dispositions"]
PY

FAKE_BIN="$TMP_DIR/fake-bin"
mkdir -p "$FAKE_BIN"
printf '#!/usr/bin/env bash\nexit 17\n' > "$FAKE_BIN/cargo"
chmod +x "$FAKE_BIN/cargo"
PATH="$FAKE_BIN:$PATH" bash "$ROOT_DIR/adl/tools/validate_csdlc_v2_supply_chain.sh" "$TMP_DIR/failed-proof.json" >/dev/null
python3 - "$TMP_DIR/failed-proof.json" <<'PY'
import json
import sys
from pathlib import Path

proof = json.loads(Path(sys.argv[1]).read_text())
assert proof["locked_dependency_metadata"]["status"] == "failed"
assert proof["overall"] == "partial_with_explicit_dispositions"
assert proof["release_gate"] == "not_ready_for_supply_chain_certification"
assert "locked_dependency_metadata" in proof["explicit_dispositions"]
PY

echo "PASS test_csdlc_v2_supply_chain"

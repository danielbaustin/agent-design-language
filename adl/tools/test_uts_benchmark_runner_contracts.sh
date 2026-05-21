#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"

python3 - <<'PY'
import importlib.util
import pathlib

runner_path = pathlib.Path("adl/tools/uts_benchmark_runner.py")
spec = importlib.util.spec_from_file_location("uts_benchmark_runner", runner_path)
module = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(module)

panel = {
    "models": [
        {
            "id": "gpt-5.5",
            "provider_kind": "hosted",
            "provider": "openai",
            "model_id": "gpt-5.5",
        }
    ]
}

selected = module.select_models(panel, "hosted", ["gpt-5.5"])
assert [entry["id"] for entry in selected] == ["gpt-5.5"]

try:
    module.select_models(panel, "hosted", ["missing-model"])
except SystemExit as exc:
    assert "models file references ids not present in model panel" in str(exc), str(exc)
else:
    raise AssertionError("missing model ids should fail before a run is claimed")

clean_report = {
    "deterministic_self_check": {"passed": True},
    "include_governed": False,
    "models": [
        {
            "candidate_id": "gpt-5.5",
            "lanes": {
                "regular": {"status": "evaluated", "full_support": False},
                "uts_only": {"status": "evaluated", "full_support": False},
                "uts_acc": {"status": "not_run", "full_support": False},
            },
        }
    ],
}
exit_code, failures = module.benchmark_exit_status(clean_report)
assert exit_code == 0, failures
assert failures == [], failures

provider_failed_report = {
    "deterministic_self_check": {"passed": True},
    "include_governed": True,
    "models": [
        {
            "candidate_id": "gpt-5.5",
            "lanes": {
                "regular": {"status": "evaluated", "full_support": True},
                "uts_only": {"status": "provider_failed", "full_support": False},
                "uts_acc": {"status": "evaluated", "full_support": True},
            },
        }
    ],
}
exit_code, failures = module.benchmark_exit_status(provider_failed_report)
assert exit_code == 1, failures
assert any("uts_only" in failure for failure in failures), failures

skipped_governed_report = {
    "deterministic_self_check": {"passed": True},
    "include_governed": True,
    "models": [
        {
            "candidate_id": "gpt-5.5",
            "lanes": {
                "regular": {"status": "evaluated", "full_support": True},
                "uts_only": {"status": "evaluated", "full_support": True},
                "uts_acc": {"status": "skipped", "full_support": False},
            },
        }
    ],
}
exit_code, failures = module.benchmark_exit_status(skipped_governed_report)
assert exit_code == 1, failures
assert any("uts_acc" in failure for failure in failures), failures
PY

echo "UTS benchmark runner contracts verified."

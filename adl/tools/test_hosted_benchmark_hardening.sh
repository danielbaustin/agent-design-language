#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"

python3 - <<'PY'
import importlib.util
import json
import pathlib
import tempfile
import urllib.error
import urllib.request
from urllib.parse import urlparse

runner_path = pathlib.Path("adl/tools/uts_benchmark_runner.py")
spec = importlib.util.spec_from_file_location("uts_benchmark_runner", runner_path)
module = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(module)

config_path = pathlib.Path("adl/tools/benchmark/hosted_provider_key_files.json")
config_text = config_path.read_text(encoding="utf-8")
config = json.loads(config_text)
assert config["schema_version"] == "adl.hosted_provider_key_files.v2"
for env_name in ("OPENAI_API_KEY", "GEMINI_API_KEY", "ANTHROPIC_API_KEY"):
    entry = config["keys"][env_name]
    assert isinstance(entry, dict), entry
    assert entry["env_var"] == env_name, entry
    assert entry["file_env_var"].startswith("ADL_"), entry
for banned in ("/Users/", "/private/", "openai2.key", "gcp-ace-2023.key", "ADL_demo_ref_04.txt"):
    assert banned not in config_text, banned

with tempfile.TemporaryDirectory() as tmpdir:
    external = pathlib.Path(tmpdir) / "outside.json"
    external.write_text("{}\n", encoding="utf-8")
    shown = module.display_path(external)
    assert shown.startswith("external/"), shown
    assert str(external) not in shown, shown
    assert "/private/" not in shown and "/Users/" not in shown, shown

redacted = module.redact_artifact_excerpt('{"tool_call":{"name":"get_time","arguments":{}}}')
assert "tool_call" not in redacted, redacted
assert "get_time" not in redacted, redacted
assert "sha256=" in redacted, redacted

note = module.sanitize_artifact_note(
    "provider_model_unavailable_or_error: OpenAI status=401: invalid api key at /Users/daniel/keys/openai2.key"
)
assert "OpenAI" not in note, note
assert "401" not in note, note
assert "/Users/" not in note, note
assert ".key" not in note, note

entry = {
    "id": "hosted-test",
    "provider_kind": "hosted",
    "provider": "openai",
    "model_id": "gpt-5.5",
    "route": "openai",
}
task = {
    "id": "get-time",
    "kind": "tool_call",
    "prompt": "Return a regular tool call for get_time with no arguments.",
    "tool_name": "get_time",
    "expected_arguments": {},
    "optional_enum_arguments": {},
    "require_exact_arguments": True,
}

original_invoke_model = module.invoke_model
try:
    module.invoke_model = lambda _entry, _prompt: ('{"tool_call":{"name":"get_time","arguments":{}}}', 11)
    regular = module.run_lane(entry, [task], "regular")
    case = regular["cases"][0]
    assert case["passed"] is True, case
    assert "get_time" not in case["raw_response_excerpt"], case
    assert "sha256=" in case["raw_response_excerpt"], case

    def raise_provider_error(_entry, _prompt):
        raise RuntimeError(
            "provider_model_unavailable_or_error: OpenAI status=401: invalid api key at /Users/daniel/keys/openai2.key"
        )

    module.invoke_model = raise_provider_error
    failed = module.run_lane(entry, [task], "regular")
    assert failed["status"] == "provider_failed", failed
    assert failed["provider_failure_kind"] == "provider_auth_missing", failed
    assert "/Users/" not in failed["note"], failed
    assert "401" not in failed["note"], failed

    governed = module.simplify_governed_result(
        entry,
        {
            "run_status": "evaluated",
            "scorecard": {"passed_count": 1, "total_cases": 1, "supports_governed_tool_use": True},
            "cases": [
                {
                    "task_id": "governed-1",
                    "classification": "ValidUsable",
                    "passed": True,
                    "duration_ms": 20,
                    "raw_response_excerpt": '{"uts_proposal":{"tool_name":"get_time"}}',
                    "notes": ["provider_model_unavailable_or_error: OpenAI status=500: upstream exploded"],
                }
            ],
        },
    )
    governed_case = governed["cases"][0]
    assert "uts_proposal" not in governed_case["raw_response_excerpt"], governed_case
    assert "500" not in governed_case["note"], governed_case

    with module.hosted_ollama_adapter(entry) as host:
        parsed = urlparse(host)
        assert parsed.path and parsed.path != "/", host
        blocked_url = f"http://127.0.0.1:{parsed.port}/api/tags"
        try:
            urllib.request.urlopen(blocked_url)
        except urllib.error.HTTPError as exc:
            assert exc.code == 403, exc.code
        else:
            raise AssertionError("adapter should reject requests without the bearer path token")

        with urllib.request.urlopen(host + "/api/tags") as response:
            doc = json.loads(response.read().decode("utf-8"))
        assert doc["models"][0]["name"] == "gpt-5.5", doc
finally:
    module.invoke_model = original_invoke_model
PY

echo "Hosted benchmark hardening contracts verified."

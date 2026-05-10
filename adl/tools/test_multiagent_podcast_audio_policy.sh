#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

make_source_episode() {
  local root="$1"
  mkdir -p "${root}/out/podcast"
  cat >"${root}/transcript.md" <<'EOF'
# Transcript
EOF
  cat >"${root}/proof_note.md" <<'EOF'
# Proof note
EOF
  cat >"${root}/out/podcast/01-chatgpt-opening.md" <<'EOF'
I'm ChatGPT, and today we're testing the audio wrapper.
EOF
  cat >"${root}/out/podcast/02-gemini-challenge.md" <<'EOF'
The difficult part is making the proof surfaces stay honest.
EOF
  cat >"${root}/out/podcast/03-claude-reframe.md" <<'EOF'
It also matters that the emotional texture stays coherent.
EOF
  cat >"${root}/out/podcast/04-chatgpt-bridge.md" <<'EOF'
Let's bridge the technical and human sides of the review.
EOF
  cat >"${root}/out/podcast/05-gemini-deepening.md" <<'EOF'
We can separate active rendering from available fallback casting.
EOF
  cat >"${root}/out/podcast/06-claude-closure.md" <<'EOF'
That makes the packet more precise without making it colder.
EOF
}

run_wrapper() {
  local out_dir="$1"
  local source_dir="$2"
  local gemini_provider="$3"
  ADL_PODCAST_AUDIO_TEST_TONES=1 \
  ADL_PODCAST_AUDIO_SOURCE_DIR="${source_dir}" \
  ADL_PODCAST_GEMINI_AUDIO_PROVIDER="${gemini_provider}" \
  bash "${repo_root}/adl/tools/demo_v0911_multiagent_podcast_audio.sh" "${out_dir}" >/dev/null
}

source_dir="${tmpdir}/source_episode"
make_source_episode "${source_dir}"

native_out="${tmpdir}/native"
surrogate_out="${tmpdir}/surrogate"
run_wrapper "${native_out}" "${source_dir}" gemini
run_wrapper "${surrogate_out}" "${source_dir}" openai

python3 - "${native_out}/audio_manifest.json" "${native_out}/audio_packet.md" "${surrogate_out}/audio_manifest.json" "${surrogate_out}/audio_packet.md" <<'PY'
import json
import sys
from pathlib import Path

native_manifest = json.loads(Path(sys.argv[1]).read_text())
native_packet = Path(sys.argv[2]).read_text()
surrogate_manifest = json.loads(Path(sys.argv[3]).read_text())
surrogate_packet = Path(sys.argv[4]).read_text()

native_mastering = native_manifest["mastering"]
assert "speech_only_input_metrics" in native_mastering
assert "full_episode_mix_metrics" in native_mastering
assert "final_mix_metrics" not in native_mastering
assert native_mastering["active_rendering"]["Gemini"]["provider"] == "gemini"
assert native_mastering["active_rendering"]["Gemini"]["mode"] == "native"
assert native_mastering["available_fallback_casting"]["Gemini"]["surrogate"]["provider"] == "openai"
assert "surrogate fallback casting lane remained available but inactive" in native_packet
assert "fallback casting is active in this run" not in native_packet

surrogate_mastering = surrogate_manifest["mastering"]
assert surrogate_mastering["active_rendering"]["Gemini"]["provider"] == "openai"
assert surrogate_mastering["active_rendering"]["Gemini"]["mode"] == "surrogate"
assert "fallback casting is active in this run" in surrogate_packet
assert "surrogate fallback casting lane remained available but inactive" not in surrogate_packet
PY

echo "PASS test_multiagent_podcast_audio_policy"

#!/usr/bin/env bash
set -euo pipefail

provider_demo_repo_root() {
  cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd
}

provider_demo_start_single_request_completion_server() {
  local log_path="$1"
  local port_file="$2"
  local token="$3"
  local output_prefix="$4"

  python3 - "$port_file" "$token" "$output_prefix" >"$log_path" 2>&1 <<'PY' &
import http.server
import json
import socketserver
import sys

port_file, token, output_prefix = sys.argv[1:4]

class ReusableTCPServer(socketserver.TCPServer):
    allow_reuse_address = True

class Handler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        if self.path != "/complete":
            self.send_response(404)
            self.end_headers()
            return
        auth = self.headers.get("Authorization", "")
        if auth != f"Bearer {token}":
            self.send_response(401)
            self.end_headers()
            self.wfile.write(b'{"error":"unauthorized"}')
            return
        length = int(self.headers.get("Content-Length", "0"))
        body = self.rfile.read(length)
        payload = json.loads(body.decode("utf-8"))
        prompt = payload.get("prompt", "")
        response = json.dumps({"output": f"{output_prefix}\n{prompt}"}).encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(response)))
        self.end_headers()
        self.wfile.write(response)

    def log_message(self, format, *args):
        return

with ReusableTCPServer(("127.0.0.1", 0), Handler) as httpd:
    with open(port_file, "w", encoding="utf-8") as fh:
        fh.write(str(httpd.server_address[1]))
    httpd.handle_request()
PY
}

provider_demo_wait_for_port() {
  local port_file="$1"
  local attempts="${2:-100}"
  local sleep_secs="${3:-0.1}"
  local port=""
  local i
  for ((i = 0; i < attempts; i++)); do
    if [[ -s "$port_file" ]]; then
      port="$(<"$port_file")"
      if [[ "$port" =~ ^[0-9]+$ ]]; then
        printf '%s\n' "$port"
        return 0
      fi
    fi
    sleep "$sleep_secs"
  done
  echo "timed out waiting for demo server port in $port_file" >&2
  return 1
}

provider_demo_materialize_loopback_example() {
  local template_path="$1"
  local output_path="$2"
  local endpoint="$3"

  python3 - "$template_path" "$output_path" "$endpoint" <<'PY'
import pathlib
import re
import sys

template_path, output_path, endpoint = sys.argv[1:4]
text = pathlib.Path(template_path).read_text(encoding="utf-8")
updated, count = re.subn(
    r'endpoint:\s*"http://127\.0\.0\.1:\d+/complete"',
    f'endpoint: "{endpoint}"',
    text,
    count=1,
)
if count != 1:
    raise SystemExit(f"expected exactly one loopback completion endpoint in {template_path}")
pathlib.Path(output_path).write_text(updated, encoding="utf-8")
PY
}

provider_demo_write_readme() {
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

provider_demo_print_proof_surfaces() {
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

provider_demo_archive_trace() {
  local out_dir="$1"
  local run_id="$2"
  local repo_root
  repo_root="$(provider_demo_repo_root)"
  local archive_root="$repo_root/.adl/trace-archive"
  local archive_run="$archive_root/milestones/v0.87.1/runs/$run_id"
  local log="$out_dir/trace_archive.log"

  "$repo_root/adl/tools/archive_run_artifacts.sh" \
    --repo-root "$repo_root" \
    --archive-root "$archive_root" \
    --apply >"$log"

  echo "Canonical trace archive:"
  echo "  $archive_run"
  echo "  $archive_root/MANIFEST.tsv"
  printf '%s\n' "$archive_run" >"$out_dir/trace_archive_path.txt"
}

#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "$0")/../.." && pwd -P)
selector="$repo_root/adl/tools/runtime_v3_operational_selector.sh"
fast_root=${ADL_RUNTIME_V3_PROOF_ROOT:-/Volumes/FastWork/adl-5590}
export CARGO_HOME=${CARGO_HOME:-$fast_root/cargo-home}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$fast_root/runtime-target}
mkdir -p "$fast_root" "$CARGO_HOME" "$CARGO_TARGET_DIR"

cargo build --locked --manifest-path "$repo_root/adl-runtime-kernel/Cargo.toml" \
  --bin adl-runtime-kernel
cargo build --locked --manifest-path "$repo_root/adl-runtime/Cargo.toml" \
  --bin adl-runtime-guardian

proof_root=$(mktemp -d "$fast_root/operational-proof.XXXXXX")
cleanup() {
  status=$?
  trap - EXIT INT TERM
  if [ "$status" -ne 0 ] && [ -f "$proof_root/state/runtime.log" ]; then
    echo "operational proof runtime failed; bounded runtime log follows" >&2
    tail -n 40 "$proof_root/state/runtime.log" >&2
  fi
  if [ "$status" -ne 0 ] && [ -f "$proof_root/state/runtime.log.previous" ]; then
    echo "operational proof previous runtime log follows" >&2
    tail -n 40 "$proof_root/state/runtime.log.previous" >&2
  fi
  preserve=false
  if [ -f "$proof_root/state/current-instance" ] &&
     ! ADL_RUNTIME_V3_SELECTOR_STATE_DIR="$proof_root/state" \
       ADL_RUNTIME_V3_SELECTOR_SHUTDOWN_GRACE_MS=30000 "$selector" stop; then
    echo "operational proof cleanup could not confirm guardian descendant termination; preserving $proof_root" >&2
    preserve=true
  fi
  if [ "$preserve" = false ]; then
    rm -rf "$proof_root"
  elif [ "$status" -eq 0 ]; then
    status=70
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT TERM

cert="$proof_root/localhost-cert.pem"
key="$proof_root/localhost-key.pem"
openssl req -x509 -newkey rsa:2048 -nodes -days 1 -sha256 \
  -subj '/CN=localhost' -addext 'subjectAltName=DNS:localhost' \
  -keyout "$key" -out "$cert" >/dev/null 2>&1

port() {
  ruby -rsocket -e 's=TCPServer.new("127.0.0.1",0); puts s.addr[1]; s.close'
}
candidate_port=$(port)
prior_port=$(port)
while [ "$candidate_port" = "$prior_port" ]; do prior_port=$(port); done

control_public_key=d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
operation_public_key=e8da63a40ca687c87cfce05cb24a786c7e75cc49c70db5573f026f1c6a86ceaa
continuity_signing_key=1717171717171717171717171717171717171717171717171717171717171717
observatory_token=runtime-v3-operational-proof-token-0001

write_init() {
  local path=$1
  local api_port=$2
  cat >"$path" <<EOF
schema = "adl.runtime_v3.init.v1"
[api]
address = "localhost:$api_port"
public_base_url = "https://localhost:$api_port"
[api.tls]
certificate_chain_path = "$cert"
private_key_path = "$key"
[observatory]
allowed_origins = ["https://localhost:8765"]
[agents]
count = 1
sample_limit = 1
EOF
}

write_init "$proof_root/candidate.toml" "$candidate_port"
write_init "$proof_root/prior.toml" "$prior_port"

write_launch() {
  local selector_path=$1
  local init_path=$2
  local continuity_path=$3
  mkdir -p "$selector_path"
  cat >"$selector_path/launch" <<EOF
#!/usr/bin/env bash
export ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX=$control_public_key
export ADL_RUNTIME_CONTROL_KEY_ID=operational-proof
export ADL_RUNTIME_CONTROL_PRINCIPAL=operational-proof
export ADL_RUNTIME_CONTINUITY_SIGNING_KEY_HEX=$continuity_signing_key
export ADL_RUNTIME_CONTINUITY_KEY_ID=operational-proof-continuity
export ADL_RUNTIME_CONTINUITY_MIN_GENERATION=0
export ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX=$operation_public_key
export ADL_RUNTIME_OPERATION_KEY_ID=operational-proof
export ADL_RUNTIME_OBSERVATORY_TOKEN=$observatory_token
export ADL_RUNTIME_SNTP_SERVER=127.0.0.1:9
exec "$CARGO_TARGET_DIR/debug/adl-runtime-guardian" \
  --kernel "$CARGO_TARGET_DIR/debug/adl-runtime-kernel" \
  --init "$init_path" --continuity-root "$continuity_path" \
  --restart-budget 1 --backoff-base-ms 10 --backoff-cap-ms 50 \
  --shutdown-grace-ms 20000
EOF
  chmod 700 "$selector_path/launch"
}

write_launch "$proof_root/candidate-selector" "$proof_root/candidate.toml" \
  "$proof_root/candidate-continuity"
write_launch "$proof_root/prior-selector" "$proof_root/prior.toml" \
  "$proof_root/prior-continuity"

cat >"$proof_root/health-probe" <<EOF
#!/usr/bin/env bash
set -euo pipefail
url=\$1
attempt=0
while [ \$attempt -lt 400 ]; do
  if payload=\$(curl --silent --show-error --fail --cacert "$cert" \
      -H 'Authorization: Bearer $observatory_token' "\$url" 2>/dev/null) &&
     printf '%s' "\$payload" | jq -e \
       '.schema == "adl.runtime_v3.observatory_feed.v2" and
        .control.public_base_url == (\$url | sub("/v1/observatory\$"; "")) and
        .control.websocket_endpoint == "/v1/observatory/ws"' \
       --arg url "\$url" >/dev/null; then
    exit 0
  fi
  attempt=\$((attempt + 1))
  sleep 0.05
done
curl --silent --show-error --cacert "$cert" -o /dev/null \
  -w 'observatory health failed: http=%{http_code} error=%{errormsg}\n' \
  -H 'Authorization: Bearer $observatory_token' "\$url" >&2 || true
exit 1
EOF
chmod 700 "$proof_root/health-probe"

export ADL_RUNTIME_V3_SELECTOR_STATE_DIR="$proof_root/state"
export ADL_RUNTIME_V3_SELECTOR_SHUTDOWN_GRACE_MS=30000
transition="$repo_root/.csdlc/prepared/issues/5590/run_operational_selector_transition.sh"
"$transition" "$selector" "$proof_root/health-probe" \
  "$proof_root/candidate-selector" "$proof_root/prior-selector" \
  "https://localhost:$candidate_port/v1/observatory" \
  "https://localhost:$prior_port/v1/observatory"

test "$(cat "$proof_root/state/current-selector")" = \
  "$(cd "$proof_root/prior-selector" && pwd -P)" || {
    echo "rollback selector identity did not resolve to the prior selector" >&2
    exit 71
  }
test -f "$proof_root/candidate-continuity/generation-1/manifest.json" || {
  echo "candidate guardian did not retain generation-1 continuity" >&2
  exit 72
}
jq -e '.schema == "adl.runtime.checkpoint.v1" and .generation == 1 and
  .previous_integrity == null and .signing_algorithm == "ed25519" and
  .signing_key_id == "operational-proof-continuity" and
  (.integrity | length) == 64 and (.signature | length) == 128' \
  "$proof_root/candidate-continuity/generation-1/manifest.json" >/dev/null || {
    echo "candidate generation-1 continuity manifest failed the signed schema contract" >&2
    exit 73
  }

"$selector" stop
test -f "$proof_root/prior-continuity/generation-1/manifest.json" || {
  echo "prior guardian did not retain generation-1 continuity" >&2
  exit 74
}

for name in candidate prior; do
  selector_dir="$proof_root/$name-selector"
  continuity_dir="$proof_root/$name-continuity"
  health_port=$candidate_port
  if [ "$name" = prior ]; then health_port=$prior_port; fi
  "$selector" activate --selector "$selector_dir"
  "$proof_root/health-probe" "https://localhost:$health_port/v1/observatory"
  "$selector" stop
  test -f "$continuity_dir/generation-2/manifest.json" || {
    echo "$name guardian did not retain generation-2 continuity after verified restore" >&2
    exit 75
  }
  jq -e --slurpfile prior "$continuity_dir/generation-1/manifest.json" \
    '.schema == "adl.runtime.checkpoint.v1" and .generation == 2 and
     .previous_integrity == $prior[0].integrity and
     .signing_algorithm == "ed25519" and
     .signing_key_id == "operational-proof-continuity" and
     (.integrity | length) == 64 and (.signature | length) == 128' \
    "$continuity_dir/generation-2/manifest.json" >/dev/null || {
      echo "$name generation-2 continuity lineage failed verified restore contract" >&2
      exit 76
    }
done

printf '%s\n' \
  'runtime_v3_operational_proof=pass guardian=external transport=https auth=bearer websocket=wss rollback=restored continuity=cryptographically_restored'

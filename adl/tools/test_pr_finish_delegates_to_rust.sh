#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mockbin="$tmpdir/mockbin"
mkdir -p "$repo/adl/tools" "$repo/adl" "$mockbin"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
chmod +x "$repo/adl/tools/pr.sh"
touch "$repo/adl/Cargo.toml"

cat >"$mockbin/cargo" <<'EOF_CARGO'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$TMP_CARGO_ARGS"
EOF_CARGO
chmod +x "$mockbin/cargo"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  echo "seed" > README.md
  git add README.md
  git commit -q -m "init"
)

TMP_CARGO_ARGS="$tmpdir/cargo_args.txt"
export TMP_CARGO_ARGS
export PATH="$mockbin:$PATH"

run_and_capture() {
  (
    cd "$repo"
    "$BASH_BIN" adl/tools/pr.sh "$@" >/dev/null
  )
}

: >"$TMP_CARGO_ARGS"
run_and_capture create --title "[v0.86][tools] Example" --slug example-create --version v0.86
run_and_capture init 1151 --slug example --no-fetch-issue --version v0.86
run_and_capture start 1152 --slug rust-start --no-fetch-issue --version v0.86 --allow-open-pr-wave
run_and_capture doctor 1152 --slug rust-start --no-fetch-issue --version v0.86 --mode full
run_and_capture ready 1152 --slug rust-start --no-fetch-issue --version v0.86 --json
run_and_capture preflight 1152 --slug rust-start --no-fetch-issue --version v0.86 --json
run_and_capture doctor 1152 --slug rust-start --no-fetch-issue --version v0.86 --mode full --json
run_and_capture finish 1153 --title "Example" --no-checks --no-open

args="$(cat "$TMP_CARGO_ARGS")"
[[ "$args" == *"--bin adl -- pr create --title [v0.86][tools] Example --slug example-create --version v0.86"* ]] || {
  echo "assertion failed: expected rust delegation for create" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"--bin adl -- pr init 1151 --slug example --no-fetch-issue --version v0.86"* ]] || {
  echo "assertion failed: expected rust delegation for init" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"--bin adl -- pr start 1152 --slug rust-start --no-fetch-issue --version v0.86 --allow-open-pr-wave"* ]] || {
  echo "assertion failed: expected rust delegation for start" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"--bin adl -- pr doctor 1152 --slug rust-start --no-fetch-issue --version v0.86 --mode full"* ]] || {
  echo "assertion failed: expected rust delegation for doctor" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"--bin adl -- pr ready 1152 --slug rust-start --no-fetch-issue --version v0.86 --json"* ]] || {
  echo "assertion failed: expected rust delegation for ready" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"--bin adl -- pr preflight 1152 --slug rust-start --no-fetch-issue --version v0.86 --json"* ]] || {
  echo "assertion failed: expected rust delegation for preflight" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"--bin adl -- pr doctor 1152 --slug rust-start --no-fetch-issue --version v0.86 --mode full --json"* ]] || {
  echo "assertion failed: expected rust delegation for doctor" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"--bin adl -- pr finish 1153 --title Example --no-checks --no-open"* ]] || {
  echo "assertion failed: expected rust delegation for finish" >&2
  echo "$args" >&2
  exit 1
}

echo "pr.sh Rust delegation parity: ok"

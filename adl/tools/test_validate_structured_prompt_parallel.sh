#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT_DIR/adl/tools/validate_structured_prompt.sh"
TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

FAKE_ROOT="$TMPDIR/fake-root"
mkdir -p "$FAKE_ROOT/adl"
touch "$FAKE_ROOT/adl/Cargo.toml"

write_fake_validator() {
  local path="$1"
  local marker="$2"
  mkdir -p "$(dirname "$path")"
  cat >"$path" <<SH
#!/usr/bin/env bash
set -euo pipefail
echo "$marker:\$*"
exit 0
SH
  chmod +x "$path"
}

write_fake_validator "$FAKE_ROOT/adl/target/debug/adl-validate-structured-prompt" "ordinary-debug-validator"
ADL_TOOLING_MANIFEST_ROOT="$FAKE_ROOT" \
ADL_PRIMARY_CHECKOUT_ROOT="$FAKE_ROOT" \
ADL_STRUCTURED_PROMPT_VALIDATOR_DISABLE_PATH_LOOKUP=1 \
  "$SCRIPT" --type srp --input "$ROOT_DIR/.adl/v0.91.5/tasks/issue-3778__pre-v092-bridge-feature-doc-production/srp.md" \
  >"$TMPDIR/ordinary-debug-stdout.txt" 2>"$TMPDIR/ordinary-debug-stderr.txt"
grep -Fq "ordinary-debug-validator:--type srp --input" "$TMPDIR/ordinary-debug-stdout.txt"
rm -f "$FAKE_ROOT/adl/target/debug/adl-validate-structured-prompt"

CARGO_TARGET_FIXTURE="$TMPDIR/cargo-target-fixture"
write_fake_validator "$CARGO_TARGET_FIXTURE/debug/adl-validate-structured-prompt" "cargo-target-validator"
ADL_TOOLING_MANIFEST_ROOT="$FAKE_ROOT" \
ADL_PRIMARY_CHECKOUT_ROOT="$FAKE_ROOT" \
ADL_STRUCTURED_PROMPT_VALIDATOR_DISABLE_PATH_LOOKUP=1 \
CARGO_TARGET_DIR="$CARGO_TARGET_FIXTURE" \
  "$SCRIPT" --type srp --input "$ROOT_DIR/.adl/v0.91.5/tasks/issue-3778__pre-v092-bridge-feature-doc-production/srp.md" \
  >"$TMPDIR/cargo-target-stdout.txt" 2>"$TMPDIR/cargo-target-stderr.txt"
grep -Fq "cargo-target-validator:--type srp --input" "$TMPDIR/cargo-target-stdout.txt"

LLVM_COV_TARGET_FIXTURE="$TMPDIR/llvm-cov-target-fixture"
write_fake_validator "$LLVM_COV_TARGET_FIXTURE/debug/adl-validate-structured-prompt" "llvm-cov-target-validator"
ADL_TOOLING_MANIFEST_ROOT="$FAKE_ROOT" \
ADL_PRIMARY_CHECKOUT_ROOT="$FAKE_ROOT" \
ADL_STRUCTURED_PROMPT_VALIDATOR_DISABLE_PATH_LOOKUP=1 \
CARGO_LLVM_COV_TARGET_DIR="$LLVM_COV_TARGET_FIXTURE" \
  "$SCRIPT" --type srp --input "$ROOT_DIR/.adl/v0.91.5/tasks/issue-3778__pre-v092-bridge-feature-doc-production/srp.md" \
  >"$TMPDIR/llvm-cov-target-stdout.txt" 2>"$TMPDIR/llvm-cov-target-stderr.txt"
grep -Fq "llvm-cov-target-validator:--type srp --input" "$TMPDIR/llvm-cov-target-stdout.txt"

set +e
ADL_TOOLING_MANIFEST_ROOT="$FAKE_ROOT" \
ADL_PRIMARY_CHECKOUT_ROOT="$FAKE_ROOT" \
ADL_STRUCTURED_PROMPT_VALIDATOR_DISABLE_PATH_LOOKUP=1 \
  "$SCRIPT" --type srp --input "$ROOT_DIR/.adl/v0.91.5/tasks/issue-3778__pre-v092-bridge-feature-doc-production/srp.md" \
  >"$TMPDIR/stdout.txt" 2>"$TMPDIR/stderr.txt"
status=$?
set -e

if [[ "$status" -ne 75 ]]; then
  echo "expected missing-binary diagnostic exit 75, got $status" >&2
  cat "$TMPDIR/stderr.txt" >&2
  exit 1
fi

grep -Fq "missing dedicated adl-validate-structured-prompt binary" "$TMPDIR/stderr.txt"
grep -Fq "ADL_STRUCTURED_PROMPT_VALIDATOR_ALLOW_CARGO_FALLBACK=1" "$TMPDIR/stderr.txt"

FAKE_LEGACY_BIN="$TMPDIR/fake-legacy-adl"
cat > "$FAKE_LEGACY_BIN" <<'SH'
#!/usr/bin/env bash
echo "legacy tooling bin should not run without explicit fallback" >&2
exit 42
SH
chmod +x "$FAKE_LEGACY_BIN"
set +e
ADL_TOOLING_MANIFEST_ROOT="$FAKE_ROOT" \
ADL_PRIMARY_CHECKOUT_ROOT="$FAKE_ROOT" \
ADL_TOOLING_RUST_BIN="$FAKE_LEGACY_BIN" \
ADL_STRUCTURED_PROMPT_VALIDATOR_DISABLE_PATH_LOOKUP=1 \
  "$SCRIPT" --type srp --input "$ROOT_DIR/.adl/v0.91.5/tasks/issue-3778__pre-v092-bridge-feature-doc-production/srp.md" \
  >"$TMPDIR/legacy-stdout.txt" 2>"$TMPDIR/legacy-stderr.txt"
legacy_status=$?
set -e
if [[ "$legacy_status" -ne 75 ]]; then
  echo "expected legacy tooling bin to be gated by missing dedicated binary diagnostic, got $legacy_status" >&2
  cat "$TMPDIR/legacy-stderr.txt" >&2
  exit 1
fi
if grep -Fq "legacy tooling bin should not run" "$TMPDIR/legacy-stderr.txt"; then
  echo "legacy tooling bin ran without explicit fallback" >&2
  exit 1
fi

LOCK_DIR="$TMPDIR/cargo-fallback.lock"
set +e
ADL_TOOLING_MANIFEST_ROOT="$FAKE_ROOT" \
ADL_PRIMARY_CHECKOUT_ROOT="$FAKE_ROOT" \
ADL_STRUCTURED_PROMPT_VALIDATOR_ALLOW_CARGO_FALLBACK=1 \
ADL_STRUCTURED_PROMPT_VALIDATOR_DISABLE_PATH_LOOKUP=1 \
ADL_STRUCTURED_PROMPT_VALIDATOR_BUILD_LOCK_DIR="$LOCK_DIR" \
ADL_STRUCTURED_PROMPT_VALIDATOR_BUILD_LOCK_TIMEOUT_SECS=0 \
  "$SCRIPT" --type srp --input "$ROOT_DIR/.adl/v0.91.5/tasks/issue-3778__pre-v092-bridge-feature-doc-production/srp.md" \
  >"$TMPDIR/cargo-stdout.txt" 2>"$TMPDIR/cargo-stderr.txt"
cargo_status=$?
set -e
if [[ "$cargo_status" -eq 0 ]]; then
  echo "expected fake cargo fallback to fail" >&2
  exit 1
fi
if [[ -d "$LOCK_DIR" ]]; then
  echo "cargo fallback lock directory was not cleaned up after failure" >&2
  cat "$TMPDIR/cargo-stderr.txt" >&2
  exit 1
fi

FRESH_LOCK_DIR="$TMPDIR/missing-parent/validator.lock"
set +e
ADL_TOOLING_MANIFEST_ROOT="$FAKE_ROOT" \
ADL_PRIMARY_CHECKOUT_ROOT="$FAKE_ROOT" \
ADL_STRUCTURED_PROMPT_VALIDATOR_ALLOW_CARGO_FALLBACK=1 \
ADL_STRUCTURED_PROMPT_VALIDATOR_DISABLE_PATH_LOOKUP=1 \
ADL_STRUCTURED_PROMPT_VALIDATOR_BUILD_LOCK_DIR="$FRESH_LOCK_DIR" \
ADL_STRUCTURED_PROMPT_VALIDATOR_BUILD_LOCK_TIMEOUT_SECS=0 \
  "$SCRIPT" --type srp --input "$ROOT_DIR/.adl/v0.91.5/tasks/issue-3778__pre-v092-bridge-feature-doc-production/srp.md" \
  >"$TMPDIR/fresh-lock-stdout.txt" 2>"$TMPDIR/fresh-lock-stderr.txt"
fresh_status=$?
set -e
if [[ "$fresh_status" -eq 75 ]] && grep -Fq "already running" "$TMPDIR/fresh-lock-stderr.txt"; then
  echo "fresh checkout fallback lock parent was misclassified as contention" >&2
  cat "$TMPDIR/fresh-lock-stderr.txt" >&2
  exit 1
fi
if [[ -d "$FRESH_LOCK_DIR" ]]; then
  echo "fresh fallback lock directory was not cleaned up after failure" >&2
  cat "$TMPDIR/fresh-lock-stderr.txt" >&2
  exit 1
fi

echo "validate_structured_prompt_parallel: ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

fake_root="$tmpdir/repo"
mockbin="$tmpdir/mockbin"
legacy_bin="$tmpdir/legacy/adl"
mkdir -p "$fake_root/adl/tools" "$fake_root/adl/target/debug" "$mockbin" "$(dirname "$legacy_bin")"
cp "$ROOT_DIR/adl/tools/owner_binary_resolution.sh" "$fake_root/adl/tools/owner_binary_resolution.sh"
cp "$ROOT_DIR/adl/tools/prompt_template.sh" "$fake_root/adl/tools/prompt_template.sh"
cp "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" "$fake_root/adl/tools/validate_structured_prompt.sh"
chmod +x "$fake_root/adl/tools/prompt_template.sh" "$fake_root/adl/tools/validate_structured_prompt.sh"
touch "$fake_root/adl/Cargo.toml"

cat >"$mockbin/cargo" <<'EOF_CARGO'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >>"${ADL_TEST_CARGO_ARGS}"
exit 97
EOF_CARGO
chmod +x "$mockbin/cargo"

prompt_cargo_args="$tmpdir/prompt-cargo.args"
validator_cargo_args="$tmpdir/validator-cargo.args"
: >"$prompt_cargo_args"
: >"$validator_cargo_args"

set +e
(
  cd "$fake_root"
  PATH="$mockbin:$PATH" \
    ADL_TEST_CARGO_ARGS="$prompt_cargo_args" \
    ADL_PROMPT_TEMPLATE_DISABLE_PATH_LOOKUP=1 \
    ADL_TOOLING_MANIFEST_ROOT="$fake_root" \
    "$fake_root/adl/tools/prompt_template.sh" validate-schemas >/dev/null 2>"$tmpdir/prompt.stderr"
)
prompt_status="$?"
set -e
[[ "$prompt_status" == "75" ]] || {
  echo "assertion failed: missing prompt-template binary should fail closed with exit 75, got $prompt_status" >&2
  cat "$tmpdir/prompt.stderr" >&2
  exit 1
}
[[ ! -s "$prompt_cargo_args" ]] || {
  echo "assertion failed: prompt-template wrapper must not run cargo without explicit fallback" >&2
  cat "$prompt_cargo_args" >&2
  exit 1
}
grep -Fq "ADL_PROMPT_TEMPLATE_ALLOW_CARGO_FALLBACK=1" "$tmpdir/prompt.stderr" || {
  echo "assertion failed: prompt-template diagnostic should explain explicit fallback opt-in" >&2
  cat "$tmpdir/prompt.stderr" >&2
  exit 1
}

cat >"$legacy_bin" <<'EOF_LEGACY'
#!/usr/bin/env bash
set -euo pipefail
printf 'legacy:%s\n' "$*" >"${ADL_TEST_LEGACY_ARGS}"
EOF_LEGACY
chmod +x "$legacy_bin"

legacy_args="$tmpdir/legacy.args"
: >"$prompt_cargo_args"
(
  cd "$fake_root"
  PATH="$mockbin:$PATH" \
    ADL_TEST_CARGO_ARGS="$prompt_cargo_args" \
    ADL_TEST_LEGACY_ARGS="$legacy_args" \
    ADL_PROMPT_TEMPLATE_DISABLE_PATH_LOOKUP=1 \
    ADL_PROMPT_TEMPLATE_ALLOW_CARGO_FALLBACK=1 \
    ADL_TOOLING_RUST_BIN="$legacy_bin" \
    ADL_TOOLING_MANIFEST_ROOT="$fake_root" \
    "$fake_root/adl/tools/prompt_template.sh" validate-schemas --repo-root "$fake_root" >/dev/null
)
grep -Fqx "legacy:tooling prompt-template validate-schemas --repo-root $fake_root" "$legacy_args" || {
  echo "assertion failed: prompt-template wrapper should use legacy tooling owner binary before cargo fallback" >&2
  cat "$legacy_args" >&2
  exit 1
}
[[ ! -s "$prompt_cargo_args" ]] || {
  echo "assertion failed: prompt-template legacy owner-binary path must not run cargo" >&2
  cat "$prompt_cargo_args" >&2
  exit 1
}

set +e
(
  cd "$fake_root"
  PATH="$mockbin:$PATH" \
    ADL_TEST_CARGO_ARGS="$validator_cargo_args" \
    ADL_STRUCTURED_PROMPT_VALIDATOR_DISABLE_PATH_LOOKUP=1 \
    ADL_TOOLING_MANIFEST_ROOT="$fake_root" \
    "$fake_root/adl/tools/validate_structured_prompt.sh" --type sip --input missing.md >/dev/null 2>"$tmpdir/validator.stderr"
)
validator_status="$?"
set -e
[[ "$validator_status" == "75" ]] || {
  echo "assertion failed: missing structured-prompt validator should fail closed with exit 75, got $validator_status" >&2
  cat "$tmpdir/validator.stderr" >&2
  exit 1
}
[[ ! -s "$validator_cargo_args" ]] || {
  echo "assertion failed: structured-prompt wrapper must not run cargo without explicit fallback" >&2
  cat "$validator_cargo_args" >&2
  exit 1
}

cat >"$fake_root/adl/target/debug/adl-prompt-template" <<'EOF_PROMPT'
#!/usr/bin/env bash
set -euo pipefail
printf 'prompt:%s\n' "$*" >"${ADL_TEST_PROMPT_ARGS}"
EOF_PROMPT
chmod +x "$fake_root/adl/target/debug/adl-prompt-template"

cat >"$fake_root/adl/target/debug/adl-validate-structured-prompt" <<'EOF_VALIDATOR'
#!/usr/bin/env bash
set -euo pipefail
printf 'validator:%s\n' "$*" >"${ADL_TEST_VALIDATOR_ARGS}"
EOF_VALIDATOR
chmod +x "$fake_root/adl/target/debug/adl-validate-structured-prompt"

prompt_args="$tmpdir/prompt.args"
validator_args="$tmpdir/validator.args"
(
  cd "$fake_root"
  PATH="$mockbin:$PATH" \
    ADL_TEST_PROMPT_ARGS="$prompt_args" \
    ADL_TOOLING_MANIFEST_ROOT="$fake_root" \
    "$fake_root/adl/tools/prompt_template.sh" validate-schemas --repo-root "$fake_root" >/dev/null
)
grep -Fqx "prompt:validate-schemas --repo-root $fake_root" "$prompt_args" || {
  echo "assertion failed: prompt-template wrapper should execute the dedicated binary" >&2
  cat "$prompt_args" >&2
  exit 1
}

(
  cd "$fake_root"
  PATH="$mockbin:$PATH" \
    ADL_TEST_VALIDATOR_ARGS="$validator_args" \
    ADL_TOOLING_MANIFEST_ROOT="$fake_root" \
    "$fake_root/adl/tools/validate_structured_prompt.sh" --type sip --input example.md >/dev/null
)
grep -Fqx "validator:--type sip --input example.md" "$validator_args" || {
  echo "assertion failed: structured-prompt wrapper should execute the dedicated binary" >&2
  cat "$validator_args" >&2
  exit 1
}

echo "prompt-template wrappers avoid implicit cargo: ok"

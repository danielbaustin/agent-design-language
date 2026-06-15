#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT_SRC="$ROOT_DIR/adl/tools/release_ceremony.sh"
BASH_BIN="$(command -v bash)"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

VERSION="v0.90.2"
TAG_NAME="v0.90.2"
STATE_FILE=""
FAKE_BIN=""
FIXTURE=""

assert_contains() {
  local text="$1"
  local pattern="$2"
  local label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed: $label; expected '$pattern'" >&2
    echo "output:" >&2
    echo "$text" >&2
    exit 1
  }
}

assert_not_contains() {
  local text="$1"
  local pattern="$2"
  local label="$3"
  if grep -Fq "$pattern" <<<"$text"; then
    echo "assertion failed: $label; unexpected '$pattern'" >&2
    echo "$text" >&2
    exit 1
  fi
}

make_fixture() {
  FIXTURE="$TMP_DIR/release-fixture"
  mkdir -p "$FIXTURE/adl/tools" "$FIXTURE/adl" "$FIXTURE/docs/milestones/$VERSION"

  cp "$SCRIPT_SRC" "$FIXTURE/adl/tools/release_ceremony.sh"
  chmod +x "$FIXTURE/adl/tools/release_ceremony.sh"

  mkdir -p "$FIXTURE/adl/tools"
  cat >"$FIXTURE/adl/Cargo.toml" <<EOF_INNER
[package]
name = "adl"
version = "${VERSION#v}"
edition = "2021"
default-run = "adl"
license = "MIT OR Apache-2.0"
description = "fixture adl"
EOF_INNER

  cat >"$FIXTURE/docs/milestones/$VERSION/RELEASE_PLAN_${VERSION}.md" <<EOF_INNER
# $VERSION release plan (fixture)
EOF_INNER

  cat >"$FIXTURE/docs/milestones/$VERSION/RELEASE_NOTES_${VERSION}.md" <<EOF_INNER
# $VERSION release notes (fixture)
EOF_INNER

  cat >"$FIXTURE/docs/milestones/$VERSION/MILESTONE_CHECKLIST_${VERSION}.md" <<EOF_INNER
# $VERSION checklist (fixture)
EOF_INNER

  echo "fixture" >"$FIXTURE/README.md"

  git -C "$FIXTURE" init -q --initial-branch=main
  git -C "$FIXTURE" config user.name "Test User"
  git -C "$FIXTURE" config user.email "test@example.com"
  git -C "$FIXTURE" add README.md adl/Cargo.toml adl/tools/release_ceremony.sh \
    "docs/milestones/$VERSION/RELEASE_PLAN_${VERSION}.md" \
    "docs/milestones/$VERSION/RELEASE_NOTES_${VERSION}.md" \
    "docs/milestones/$VERSION/MILESTONE_CHECKLIST_${VERSION}.md"
  git -C "$FIXTURE" commit -q -m "fixture init"

  mkdir -p "$FIXTURE/remote"
  git init -q --bare "$FIXTURE/remote"
  git -C "$FIXTURE" remote add origin "$FIXTURE/remote"
  git -C "$FIXTURE" push -q origin main
}

setup_fake_gh() {
  FAKE_BIN="$FIXTURE/fakebin"
  STATE_FILE="$FIXTURE/releases-state.txt"
  mkdir -p "$FAKE_BIN"
  : >"$STATE_FILE"

  cat >"$FAKE_BIN/adl" <<'EOF_FAKE'
#!/usr/bin/env bash
set -euo pipefail

STATE_FILE="${RELEASE_STATE_FILE:?missing RELEASE_STATE_FILE}"

if [[ "$1" == "tooling" && "$2" == "github-release" ]]; then
  ACTION="$3"
  shift 3
  TAG=""
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --tag)
        TAG="$2"
        shift 2
        ;;
      --repo|--name|--notes-file|--target)
        shift 2
        ;;
      *)
        echo "unexpected github-release arg: $1" >&2
        exit 1
        ;;
    esac
  done
  [[ -n "$TAG" ]] || {
    echo "missing --tag" >&2
    exit 1
  }

  if [[ "$ACTION" == "ensure-absent" ]]; then
    if [[ -f "$STATE_FILE" ]] && grep -Fqx "$TAG" "$STATE_FILE"; then
      echo "GitHub release already exists for tag $TAG" >&2
      exit 1
    fi
    exit 0
  fi

  if [[ "$ACTION" == "ensure-present" ]]; then
    if [[ -f "$STATE_FILE" ]] && grep -Fqx "$TAG" "$STATE_FILE"; then
      exit 0
    fi
    echo "GitHub release does not exist for tag $TAG" >&2
    exit 1
  fi

  if [[ "$ACTION" == "draft" ]]; then
    if ! grep -Fqx "$TAG" "$STATE_FILE" 2>/dev/null; then
      printf '%s\n' "$TAG" >>"$STATE_FILE"
    fi
    exit 0
  fi

  if [[ "$ACTION" == "publish" ]]; then
  if [[ -f "$STATE_FILE" ]] && grep -Fqx "$TAG" "$STATE_FILE"; then
    exit 0
  fi
  exit 1
  fi

  echo "unexpected github-release action: $ACTION" >&2
  exit 1
fi

echo "unexpected adl command: $*" >&2
exit 1
EOF_FAKE
  chmod +x "$FAKE_BIN/adl"
}

reset_git_state() {
  git -C "$FIXTURE" tag -d "$TAG_NAME" >/dev/null 2>&1 || true
  git -C "$FIXTURE" push -q origin ":refs/tags/$TAG_NAME" >/dev/null 2>&1 || true
  : >"$STATE_FILE"
}

run_release_case() {
  local label="$1"
  local expected_status="$2"
  local expected_message="$3"
  shift 3

  local output
  set +e
  output="$(cd "$FIXTURE" && PATH="$FAKE_BIN:$PATH" RELEASE_STATE_FILE="$STATE_FILE" \
    ADL_RELEASE_GITHUB_CMD="$FAKE_BIN/adl" ADL_RELEASE_GITHUB_REPO="owner/repo" \
    "$BASH_BIN" adl/tools/release_ceremony.sh --version "$VERSION" \
    --skip-sor-gate --target-branch main --allow-dirty "$@" 2>&1)"
  local status=$?
  set -e

  if [[ "$status" -ne "$expected_status" ]]; then
    echo "assertion failed: $label (exit $status != $expected_status)" >&2
    echo "$output" >&2
    exit 1
  fi

  if [[ -n "$expected_message" ]]; then
    assert_contains "$output" "$expected_message" "$label"
  fi
}

assert_remote_tag_absent() {
  if git -C "$FIXTURE" ls-remote --exit-code --tags origin "refs/tags/$TAG_NAME" >/dev/null 2>&1; then
    echo "assertion failed: remote tag $TAG_NAME should be absent" >&2
    exit 1
  fi
}

assert_remote_tag_present() {
  git -C "$FIXTURE" ls-remote --exit-code --tags origin "refs/tags/$TAG_NAME" >/dev/null 2>&1
}

assert_release_absent() {
  if [[ -f "$STATE_FILE" ]] && grep -Fqx "$TAG_NAME" "$STATE_FILE"; then
    echo "assertion failed: release $TAG_NAME should be absent" >&2
    exit 1
  fi
}

assert_release_present() {
  if [[ ! -f "$STATE_FILE" ]] || ! grep -Fqx "$TAG_NAME" "$STATE_FILE"; then
    echo "assertion failed: release $TAG_NAME should be present" >&2
    exit 1
  fi
}

assert_local_tag_present() {
  git -C "$FIXTURE" rev-parse -q --verify "refs/tags/$TAG_NAME" >/dev/null 2>&1
}

make_fixture
setup_fake_gh

# Create/tag preconditions: missing local and remote tags should pass for create-tag and tag mutation.
reset_git_state
run_release_case "create-tag succeeds when local and remote are absent" 0 "" --create-tag --tag "$TAG_NAME"
git -C "$FIXTURE" rev-parse -q --verify "refs/tags/$TAG_NAME" >/dev/null || {
  echo "assertion failed: create-tag should create local tag" >&2
  exit 1
}

git -C "$FIXTURE" tag -d "$TAG_NAME" >/dev/null 2>&1

# Push-tag preconditions: local present and remote absent should pass.
reset_git_state
git -C "$FIXTURE" tag -a "$TAG_NAME" -m "release fixture"
run_release_case "push-tag succeeds when local tag exists and remote is absent" 0 "" --push-tag --tag "$TAG_NAME"
assert_remote_tag_present

# Draft/publish release operations delegate to the Rust/octocrab release path.
reset_git_state
git -C "$FIXTURE" tag -a "$TAG_NAME" -m "release fixture"
git -C "$FIXTURE" push -q origin "$TAG_NAME"
run_release_case \
  "draft-release delegates to Rust release support" \
  0 \
  "creating draft GitHub release" \
  --draft-release --tag "$TAG_NAME"
assert_release_present

git -C "$FIXTURE" push -q origin ":refs/tags/$TAG_NAME" >/dev/null 2>&1 || true
git -C "$FIXTURE" tag -d "$TAG_NAME" >/dev/null 2>&1 || true
printf '%s\n' "$TAG_NAME" >"$STATE_FILE"
run_release_case \
  "publish-release delegates to Rust release support" \
  0 \
  "publishing GitHub release" \
  --publish-release --tag "$TAG_NAME"
assert_release_present

# Guard failures for violated preconditions.
reset_git_state
git -C "$FIXTURE" tag -a "$TAG_NAME" -m "release fixture"
run_release_case "create-tag fails when local tag already exists" 1 "tag already exists locally" --create-tag --tag "$TAG_NAME"

reset_git_state
git -C "$FIXTURE" tag -a "$TAG_NAME" -m "release fixture"
git -C "$FIXTURE" push -q origin "$TAG_NAME"
run_release_case "push-tag fails when remote tag already exists" 1 "tag already exists on origin" --push-tag --tag "$TAG_NAME"

reset_git_state
git -C "$FIXTURE" tag -a "$TAG_NAME" -m "release fixture"
git -C "$FIXTURE" push -q origin "$TAG_NAME"
printf '%s
' "$TAG_NAME" >"$STATE_FILE"
run_release_case \
  "draft-release fails when release exists" \
  1 \
  "GitHub release already exists" \
  --draft-release --tag "$TAG_NAME"

reset_git_state
run_release_case \
  "publish-release fails when draft release is missing" \
  1 \
  "GitHub release does not exist" \
  --publish-release --tag "$TAG_NAME"

# Split-step invocation across mutation phases.
reset_git_state
run_release_case "split-step phase 1: create-tag" 0 "" --create-tag --tag "$TAG_NAME"
assert_local_tag_present
run_release_case "split-step phase 2: push-tag" 0 "" --push-tag --tag "$TAG_NAME"
assert_remote_tag_present
assert_release_absent
run_release_case \
  "split-step phase 3: draft-release" \
  0 \
  "creating draft GitHub release" \
  --draft-release --tag "$TAG_NAME"
assert_release_present
run_release_case \
  "split-step phase 4: publish-release" \
  0 \
  "publishing GitHub release" \
  --publish-release --tag "$TAG_NAME"
assert_release_present

echo "test_release_ceremony: ok"

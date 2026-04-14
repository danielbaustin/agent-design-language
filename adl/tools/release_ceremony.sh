#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

VERSION=""
TAG=""
TARGET_BRANCH="main"
CHECK_ONLY=1
CREATE_TAG=0
PUSH_TAG=0
CREATE_DRAFT_RELEASE=0
PUBLISH_RELEASE=0
ALLOW_DIRTY=0
SKIP_SOR_GATE=0

PLAN_FILE=""
NOTES_FILE=""
CHECKLIST_FILE=""

usage() {
  cat <<'EOF'
Usage:
  adl/tools/release_ceremony.sh --version <v0.87.1> [options]

Safe by default:
  Runs release-ceremony preflight checks and prints the ceremony plan.

Mutation flags:
  --create-tag            Create the annotated git tag locally
  --push-tag              Push the tag to origin
  --draft-release         Create a draft GitHub release from the release notes
  --publish-release       Publish an existing GitHub release draft

Other options:
  --tag <tag>             Override tag name (default: same as --version)
  --target-branch <name>  Branch required for ceremony mutations (default: main)
  --allow-dirty           Skip the clean-worktree check
  --skip-sor-gate         Skip adl/tools/check_milestone_closed_issue_sor_truth.sh (closed-issue bundle truth gate)
  -h, --help              Show this help

Examples:
  adl/tools/release_ceremony.sh --version v0.87.1
  adl/tools/release_ceremony.sh --version v0.87.1 --create-tag --push-tag
  adl/tools/release_ceremony.sh --version v0.87.1 --draft-release
  adl/tools/release_ceremony.sh --version v0.87.1 --publish-release
EOF
}

fail() {
  echo "ERROR: $*" >&2
  exit 1
}

info() {
  echo "[release-ceremony] $*"
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || fail "required command not found: $1"
}

find_milestone_doc() {
  local version="$1"
  local stem="$2"
  local exact="$ROOT/docs/milestones/$version/${stem}_${version}.md"
  if [[ -f "$exact" ]]; then
    printf '%s\n' "$exact"
    return 0
  fi

  local fallback
  fallback="$(find "$ROOT/docs/milestones/$version" -maxdepth 1 -type f -name "${stem}_*.md" | sort | head -n 1 || true)"
  [[ -n "$fallback" ]] || fail "could not find ${stem} document under docs/milestones/$version"
  printf '%s\n' "$fallback"
}

assert_clean_worktree() {
  local status
  status="$(git -C "$ROOT" status --short)"
  [[ -z "$status" ]] || fail "working tree is not clean; commit/stash changes or rerun with --allow-dirty"
}

assert_branch() {
  local current
  current="$(git -C "$ROOT" branch --show-current)"
  [[ "$current" == "$TARGET_BRANCH" ]] || fail "current branch is '$current'; expected '$TARGET_BRANCH'"
}

assert_tag_absent_local() {
  git -C "$ROOT" rev-parse -q --verify "refs/tags/$TAG" >/dev/null 2>&1 && fail "tag already exists locally: $TAG"
}

assert_tag_present_local() {
  git -C "$ROOT" rev-parse -q --verify "refs/tags/$TAG" >/dev/null 2>&1 || fail "local tag does not exist: $TAG"
}

assert_tag_absent_remote() {
  git -C "$ROOT" ls-remote --exit-code --tags origin "refs/tags/$TAG" >/dev/null 2>&1 && fail "tag already exists on origin: $TAG"
}

assert_release_absent() {
  gh release view "$TAG" --repo "$(gh repo view --json nameWithOwner -q .nameWithOwner)" >/dev/null 2>&1 && fail "GitHub release already exists for tag: $TAG"
}

assert_release_present() {
  gh release view "$TAG" --repo "$(gh repo view --json nameWithOwner -q .nameWithOwner)" >/dev/null 2>&1 || fail "GitHub release does not exist for tag: $TAG"
}

check_cargo_version() {
  local expected="${VERSION#v}"
  local expected_alt=""
  local actual
  actual="$(sed -n 's/^version = "\(.*\)"/\1/p' "$ROOT/adl/Cargo.toml" | head -n 1)"
  [[ -n "$actual" ]] || fail "could not read version from adl/Cargo.toml"
  if [[ "$expected" != *.*.* ]]; then
    expected_alt="${expected}.0"
  fi

  if [[ "$actual" == "$expected" || ( -n "$expected_alt" && "$actual" == "$expected_alt" ) ]]; then
    return 0
  fi

  if [[ -n "$expected_alt" ]]; then
    fail "adl/Cargo.toml version mismatch: expected $expected or $expected_alt, found $actual"
  fi

  fail "adl/Cargo.toml version mismatch: expected $expected, found $actual"
}

check_sor_gate() {
  if [[ "$SKIP_SOR_GATE" == "1" ]]; then
    info "skipping closed-issue bundle truth gate by request"
    return 0
  fi

  local checker="$ROOT/adl/tools/check_milestone_closed_issue_sor_truth.sh"
  [[ -x "$checker" || -f "$checker" ]] || {
    info "closed-issue bundle truth gate not present; skipping"
    return 0
  }

  info "running closed-issue bundle truth gate for $VERSION"
  bash "$checker" --version "$VERSION"
}

print_plan() {
  cat <<EOF

Release ceremony plan
  version:        $VERSION
  tag:            $TAG
  target branch:  $TARGET_BRANCH
  release plan:   ${PLAN_FILE#$ROOT/}
  release notes:  ${NOTES_FILE#$ROOT/}
  checklist:      ${CHECKLIST_FILE#$ROOT/}

Requested actions
  check only:         $([[ "$CHECK_ONLY" == "1" ]] && echo yes || echo no)
  create tag:         $([[ "$CREATE_TAG" == "1" ]] && echo yes || echo no)
  push tag:           $([[ "$PUSH_TAG" == "1" ]] && echo yes || echo no)
  create draft rel.:  $([[ "$CREATE_DRAFT_RELEASE" == "1" ]] && echo yes || echo no)
  publish release:    $([[ "$PUBLISH_RELEASE" == "1" ]] && echo yes || echo no)

Manual ceremony reminders
  - Review ${PLAN_FILE#$ROOT/} and ${CHECKLIST_FILE#$ROOT/} before mutating release state.
  - Confirm review/remediation/next-milestone gates are actually closed or explicitly dispositioned.
  - Verify ${NOTES_FILE#$ROOT/} is final before creating or publishing the GitHub release.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      VERSION="${2:-}"
      shift 2
      ;;
    --tag)
      TAG="${2:-}"
      shift 2
      ;;
    --target-branch)
      TARGET_BRANCH="${2:-}"
      shift 2
      ;;
    --create-tag)
      CHECK_ONLY=0
      CREATE_TAG=1
      shift
      ;;
    --push-tag)
      CHECK_ONLY=0
      PUSH_TAG=1
      shift
      ;;
    --draft-release)
      CHECK_ONLY=0
      CREATE_DRAFT_RELEASE=1
      shift
      ;;
    --publish-release)
      CHECK_ONLY=0
      PUBLISH_RELEASE=1
      shift
      ;;
    --allow-dirty)
      ALLOW_DIRTY=1
      shift
      ;;
    --skip-sor-gate)
      SKIP_SOR_GATE=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      fail "unknown argument: $1"
      ;;
  esac
done

[[ -n "$VERSION" ]] || fail "--version is required"
[[ -n "$TAG" ]] || TAG="$VERSION"

require_cmd git
require_cmd gh
require_cmd sed
require_cmd find

PLAN_FILE="$(find_milestone_doc "$VERSION" "RELEASE_PLAN")"
NOTES_FILE="$(find_milestone_doc "$VERSION" "RELEASE_NOTES")"
CHECKLIST_FILE="$(find_milestone_doc "$VERSION" "MILESTONE_CHECKLIST")"

info "repo root: $ROOT"
info "loading milestone docs for $VERSION"

if [[ "$ALLOW_DIRTY" != "1" ]]; then
  assert_clean_worktree
else
  info "allowing dirty worktree by request"
fi

assert_branch
check_cargo_version
check_sor_gate

if [[ "$CREATE_TAG" == "1" ]]; then
  assert_tag_absent_local
  assert_tag_absent_remote
fi

if [[ "$PUSH_TAG" == "1" ]]; then
  assert_tag_present_local
  assert_tag_absent_remote
fi

if [[ "$CREATE_DRAFT_RELEASE" == "1" ]]; then
  assert_tag_present_local
  assert_release_absent
fi

if [[ "$PUBLISH_RELEASE" == "1" ]]; then
  assert_release_present
fi

print_plan

if [[ "$CHECK_ONLY" == "1" ]]; then
  info "preflight checks passed; no mutating actions requested"
  exit 0
fi

if [[ "$CREATE_TAG" == "1" ]]; then
  info "creating annotated tag $TAG"
  git -C "$ROOT" tag -a "$TAG" -m "ADL $TAG"
fi

if [[ "$PUSH_TAG" == "1" ]]; then
  info "pushing tag $TAG to origin"
  git -C "$ROOT" push origin "$TAG"
fi

if [[ "$CREATE_DRAFT_RELEASE" == "1" ]]; then
  info "creating GitHub draft release for $TAG from release notes"
  gh release create "$TAG" \
    --draft \
    --verify-tag \
    --title "ADL $TAG" \
    --notes-file "$NOTES_FILE"
fi

if [[ "$PUBLISH_RELEASE" == "1" ]]; then
  info "publishing GitHub release for $TAG"
  gh release edit "$TAG" --draft=false
fi

info "release ceremony actions completed"

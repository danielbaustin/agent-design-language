#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export ADL_TOOLING_MANIFEST_ROOT="$ROOT_DIR"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

python3 - "$ROOT_DIR" "$tmpdir" <<'PY'
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
out = Path(sys.argv[2])
template_root = root / "docs" / "templates" / "prompts" / "1.0.0"

values = {
    "<issue>": "1374",
    "<issue_padded>": "1374",
    "<task_id>": "issue-1374",
    "<run_id>": "issue-1374",
    "<version>": "v0.91.3",
    "<slug>": "template-test",
    "<title>": "Template test",
    "<branch>": "not bound yet",
    "<card_status>": "ready",
    "<issue_url>": "https://github.com/danielbaustin/agent-design-language/issues/1374",
    "<source_issue_prompt>": ".adl/v0.91.3/bodies/issue-1374-template-test.md",
    "<docs_context>": "docs/templates/prompts/current.json",
    "<output_card>": ".adl/v0.91.3/tasks/issue-1374__template-test/sor.md",
    "<stp_card>": ".adl/v0.91.3/tasks/issue-1374__template-test/stp.md",
    "<sip_card>": ".adl/v0.91.3/tasks/issue-1374__template-test/sip.md",
    "<spp_card>": ".adl/v0.91.3/tasks/issue-1374__template-test/spp.md",
    "<srp_card>": ".adl/v0.91.3/tasks/issue-1374__template-test/srp.md",
    "<sor_card>": ".adl/v0.91.3/tasks/issue-1374__template-test/sor.md",
    "<wp>": "process",
    "<required_outcome_type>": "combination",
    "<demo_required>": "false",
    "<issue_graph_note>": "test issue graph note",
    "<summary>": "Template sample.",
    "<goal>": "Create a filled template sample.",
    "<required_outcome>": "A validator-clean filled template sample.",
    "<deliverables>": "Filled sample card.",
    "<acceptance_criteria>": "The sample validates.",
    "<inputs>": "Template source and sample values.",
    "<repo_inputs>": "docs/templates/prompts/1.0.0/",
    "<dependencies>": "none",
    "<target_files_surfaces>": "docs/templates/prompts/1.0.0/",
    "<validation_plan>": "Run structured prompt validation.",
    "<demo_proof_requirements>": "No demo required.",
    "<non_goals>": "Do not widen scope.",
    "<issue_graph_notes>": "none",
    "<notes_risks>": "sample only",
    "<tooling_notes>": "sample only",
    "<target_files_surfaces_inline>": "docs/templates/prompts/1.0.0/",
    "<non_goals_inline>": "Do not widen scope.",
    "<plan_summary>": "Template sample execution plan.",
    "<dependencies_inline>": "none",
    "<repo_inputs_inline>": "docs/templates/prompts/1.0.0/",
    "<deliverables_inline>": "Filled sample card.",
    "<acceptance_criteria_inline>": "The sample validates.",
    "<risks_inline>": "sample only",
    "<validation_plan_inline>": "Run structured prompt validation.",
    "<notes_risks_inline>": "sample only",
    "<status>": "NOT_STARTED",
    "<timestamp>": "2026-05-23T00:00:00Z",
    "<branch_action>": "Preserved pre-run branch truth; no execution branch or worktree is bound yet.",
}

for kind in ["sip", "stp", "spp", "srp", "sor"]:
    text = (template_root / f"{kind}.md").read_text()
    for token, value in values.items():
        text = text.replace(token, value)
    leftovers = sorted(set(re.findall(r"<[a-z][a-z0-9_]*>", text)))
    if leftovers:
        raise SystemExit(f"{kind} template has unfilled placeholders: {leftovers}")
    (out / f"{kind}.md").write_text(text)
PY

bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type sip --phase bootstrap --input "$tmpdir/sip.md"
bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type stp --phase bootstrap --input "$tmpdir/stp.md"
bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type spp --phase bootstrap --input "$tmpdir/spp.md"
bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type srp --phase bootstrap --input "$tmpdir/srp.md"
bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type sor --phase bootstrap --input "$tmpdir/sor.md"

for kind in sip stp spp srp sor; do
  cp "$tmpdir/$kind.md" "$tmpdir/$kind-unresolved-placeholder.md"
  printf '\n<unfilled_extra>\n' >> "$tmpdir/$kind-unresolved-placeholder.md"
  if bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type "$kind" --phase bootstrap --input "$tmpdir/$kind-unresolved-placeholder.md" >/dev/null 2>&1; then
    echo "expected $kind validation to reject unresolved prompt-template placeholders" >&2
    exit 1
  fi
done

if grep -R "Structured Review Policy" "$ROOT_DIR/docs/templates/prompts/1.0.0"; then
  echo "1.0.0 prompt templates must not contain legacy Structured Review Policy wording" >&2
  exit 1
fi

registry_repo="$tmpdir/registry-repo"
mkdir -p "$registry_repo/adl/tools" "$registry_repo/docs/templates/prompts/1.0.0" "$registry_repo/docs/templates/prompts/1.0.1"
cp "$ROOT_DIR/adl/tools/pr.sh" "$registry_repo/adl/tools/pr.sh"
cp "$ROOT_DIR/adl/tools/pr_delegate.sh" "$registry_repo/adl/tools/pr_delegate.sh"
cp "$ROOT_DIR/adl/tools/pr_usage.sh" "$registry_repo/adl/tools/pr_usage.sh"
cp "$ROOT_DIR/adl/tools/card_paths.sh" "$registry_repo/adl/tools/card_paths.sh"
cp "$ROOT_DIR/docs/templates/prompts/1.0.0/stp.md" "$registry_repo/docs/templates/prompts/1.0.0/stp.md"
sed 's#docs/templates/prompts/1.0.0/stp.md#docs/templates/prompts/1.0.1/stp.md#' \
  "$ROOT_DIR/docs/templates/prompts/1.0.0/stp.md" >"$registry_repo/docs/templates/prompts/1.0.1/stp.md"
cat >"$registry_repo/docs/templates/prompts/current.json" <<'JSON'
{
  "schema": "adl.csdlc.prompt_template_registry.v1",
  "csdlc_prompt_template_set": "1.0.1",
  "semver": "1.0.1",
  "status": "active",
  "object_kind": "csdlc_prompt_template_set",
  "templates": {
    "stp": {
      "semantic_role": "Structured Task Prompt",
      "path": "docs/templates/prompts/1.0.1/stp.md"
    }
  }
}
JSON
git -C "$registry_repo" init -q
git -C "$registry_repo" remote add origin https://github.com/danielbaustin/agent-design-language.git
resolved="$(
  cd "$registry_repo"
  ADL_PR_SH_TEMPLATE_RESOLVER_SELF_TEST=1 bash adl/tools/pr.sh stp
)"
case "$resolved" in
  */docs/templates/prompts/1.0.1/stp.md) ;;
  *)
    echo "expected pr.sh template resolver to follow current.json, got: $resolved" >&2
    exit 1
    ;;
esac

echo "PASS: 1.0.0 prompt templates fill and validate"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v089/medium_article_writing}"
FIXTURE_DIR="$ROOT_DIR/demos/fixtures/medium_article_writing"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-89-medium-article-writing"
EXAMPLE="adl/examples/v0-89-medium-article-writing.adl.yaml"
GENERATED_EXAMPLE="$OUT_DIR/v0-89-medium-article-writing.runtime.adl.yaml"
INPUT_PACKET_DIR="$OUT_DIR/input_packet"
ARTICLE_PACKET_DIR="$OUT_DIR/article_packet"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"

sanitize_generated_artifacts() {
  export ADL_SANITIZE_OUT_DIR="$OUT_DIR"
  export ADL_SANITIZE_OUT_REAL
  ADL_SANITIZE_OUT_REAL="$(cd "$OUT_DIR" && pwd -P)"
  export ADL_SANITIZE_ROOT_DIR="$ROOT_DIR"
  export ADL_SANITIZE_ROOT_REAL
  ADL_SANITIZE_ROOT_REAL="$(cd "$ROOT_DIR" && pwd -P)"
  find "$OUT_DIR" -type f \( -name '*.json' -o -name '*.md' -o -name '*.txt' -o -name '*.yaml' \) -print0 |
    xargs -0 perl -0pi -e '
      for my $name (qw(ADL_SANITIZE_OUT_REAL ADL_SANITIZE_OUT_DIR ADL_SANITIZE_ROOT_REAL ADL_SANITIZE_ROOT_DIR)) {
        my $value = $ENV{$name} // "";
        next if $value eq "";
        my $replacement = $name =~ /ROOT/ ? "<repo_root>" : "<output_dir>";
        s/\Q$value\E/$replacement/g;
      }
    '
}

require_fixture() {
  local path="$1"
  [[ -f "$path" ]] || {
    echo "missing required Medium-writing fixture: $path" >&2
    exit 1
  }
}

require_fixture "$FIXTURE_DIR/v0-89-medium-article-brief.md"

rm -rf "$OUT_DIR"
mkdir -p "$INPUT_PACKET_DIR" "$ARTICLE_PACKET_DIR"
cp "$FIXTURE_DIR/v0-89-medium-article-brief.md" "$INPUT_PACKET_DIR/"

cd "$ROOT_DIR"

python3 - "$EXAMPLE" "$GENERATED_EXAMPLE" "$INPUT_PACKET_DIR/v0-89-medium-article-brief.md" <<'PY'
import sys
from pathlib import Path

source, target, brief = sys.argv[1:4]
text = Path(source).read_text(encoding="utf-8")
text = text.replace("@file:packets/v0-89-medium-article-brief.md", f"@file:{brief}")
Path(target).write_text(text, encoding="utf-8")
PY

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
  cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
    "$GENERATED_EXAMPLE" \
    --run \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    >"$OUT_DIR/run_log.txt" 2>&1

cp "$STEP_OUT/article/01-outline.md" "$ARTICLE_PACKET_DIR/outline.md"
cp "$STEP_OUT/article/02-title-options.md" "$ARTICLE_PACKET_DIR/title_options.md"
cp "$STEP_OUT/article/03-draft.md" "$ARTICLE_PACKET_DIR/draft.md"
cp "$STEP_OUT/article/04-editorial-notes.md" "$ARTICLE_PACKET_DIR/editorial_notes.md"
python3 - "$STEP_OUT/article/05-publish-summary.json" "$ARTICLE_PACKET_DIR/publish_summary.json" <<'PY'
import json
import sys
from pathlib import Path

raw = Path(sys.argv[1]).read_text(encoding="utf-8")
payload = raw.split("USER:", 1)[-1].strip() if raw.startswith("USER:") else raw.strip()
parsed = json.loads(payload)
Path(sys.argv[2]).write_text(json.dumps(parsed, indent=2) + "\n", encoding="utf-8")
PY

cat >"$ARTICLE_PACKET_DIR/reviewer_brief.md" <<'EOF'
# Reviewer Brief

Review this bounded Medium-writing packet in the following order:

1. `input_packet/v0-89-medium-article-brief.md`
2. `article_packet/outline.md`
3. `article_packet/title_options.md`
4. `article_packet/draft.md`
5. `article_packet/editorial_notes.md`
6. `article_packet/publish_summary.json`
7. `runtime/runs/v0-89-medium-article-writing/run_summary.json`

This demo proves a bounded publication workflow for one launch-style essay.
It does not claim a full publication program or autonomous publishing authority.
The packet is intentionally reviewer-friendly and easy to inspect in order.
EOF

python3 - "$MANIFEST" "$RUN_ID" <<'PY'
import json
import sys
from pathlib import Path

manifest = {
    "schema_version": "adl.medium_article_writing_demo.v1",
    "demo_id": "v0.89.medium_article_writing",
    "title": "v0.89 bounded Medium article writing demo",
    "execution_mode": "runtime_mock_provider_demo",
    "claim": "ADL can host a bounded, reviewer-friendly Medium-style writing workflow that turns one brief into a polished article packet with explicit editorial surfaces.",
    "artifacts": {
        "brief": "input_packet/v0-89-medium-article-brief.md",
        "outline": "article_packet/outline.md",
        "titles": "article_packet/title_options.md",
        "draft": "article_packet/draft.md",
        "editorial_notes": "article_packet/editorial_notes.md",
        "publish_summary": "article_packet/publish_summary.json",
        "reviewer_brief": "article_packet/reviewer_brief.md",
        "run_summary": f"runtime/runs/{sys.argv[2]}/run_summary.json",
        "steps": f"runtime/runs/{sys.argv[2]}/steps.json",
        "trace": f"runtime/runs/{sys.argv[2]}/logs/trace_v1.json"
    }
}
Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<EOF
# v0.89 Demo - Medium Article Writing

Canonical command:

\`\`\`bash
bash adl/tools/demo_v089_medium_article_writing.sh
\`\`\`

Primary proof surfaces:
- \`demo_manifest.json\`
- \`article_packet/draft.md\`
- \`article_packet/editorial_notes.md\`
- \`article_packet/publish_summary.json\`
- \`runtime/runs/$RUN_ID/run_summary.json\`
- \`runtime/runs/$RUN_ID/logs/trace_v1.json\`

What this proves:
- ADL can host a bounded publication-oriented writing workflow
- the article packet remains reviewer-friendly and inspectable
- editorial notes and publish posture are part of the output, not hidden afterthoughts
EOF

sanitize_generated_artifacts

echo "Medium article writing proof surface under the output directory:"
echo "  demo_manifest.json"
echo "  article_packet/draft.md"
echo "  article_packet/editorial_notes.md"
echo "  article_packet/publish_summary.json"
echo "  runtime/runs/$RUN_ID/run_summary.json"
echo "  runtime/runs/$RUN_ID/logs/trace_v1.json"

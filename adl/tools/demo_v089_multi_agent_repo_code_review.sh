#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v089/multi_agent_repo_code_review}"
TMP_RAW_INVENTORY="$(mktemp)"
trap 'rm -f "$TMP_RAW_INVENTORY"' EXIT

PACKET_DIR="$OUT_DIR/review_packet"
REVIEWERS_DIR="$OUT_DIR/reviewers"
SYNTHESIS_DIR="$OUT_DIR/synthesis"
PACKET_MANIFEST="$PACKET_DIR/review_packet_manifest.json"
INVENTORY_SUMMARY="$PACKET_DIR/inventory_summary.json"
SELECTED_PATHS="$PACKET_DIR/selected_paths.txt"
CODE_REVIEW="$REVIEWERS_DIR/code_review.md"
SECURITY_REVIEW="$REVIEWERS_DIR/security_review.md"
TEST_REVIEW="$REVIEWERS_DIR/test_review.md"
DOCS_REVIEW="$REVIEWERS_DIR/docs_review.md"
CROSS_REVIEW="$REVIEWERS_DIR/cross_review_notes.md"
FINAL_SYNTHESIS="$SYNTHESIS_DIR/final_synthesis_review.md"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"
VALIDATOR="$ROOT_DIR/adl/tools/validate_multi_agent_repo_review_demo.py"

run_date="$(date -u +%Y-%m-%d)"

rm -rf "$OUT_DIR"
mkdir -p "$PACKET_DIR" "$REVIEWERS_DIR" "$SYNTHESIS_DIR"

cd "$ROOT_DIR"

python3 adl/tools/skills/repo-code-review/scripts/repo_inventory.py "$ROOT_DIR" >"$TMP_RAW_INVENTORY"

cat >"$SELECTED_PATHS" <<'EOF'
README.md
demos/README.md
adl/Cargo.toml
adl/tools/pr.sh
adl/tools/skills/repo-code-review/SKILL.md
adl/tools/skills/workflow-conductor/scripts/route_workflow.py
adl/tools/test_workflow_conductor_skill_contracts.sh
docs/tooling/review-surface-format.md
EOF

python3 - "$TMP_RAW_INVENTORY" "$PACKET_MANIFEST" "$INVENTORY_SUMMARY" "$SELECTED_PATHS" <<'PY'
import json
import sys
from pathlib import Path

raw_inventory_path, packet_manifest_path, inventory_summary_path, selected_paths_path = [
    Path(arg) for arg in sys.argv[1:]
]

raw = json.loads(raw_inventory_path.read_text(encoding="utf-8"))
selected_paths = [
    line.strip()
    for line in selected_paths_path.read_text(encoding="utf-8").splitlines()
    if line.strip()
]

sanitized_inventory = {
    "schema": "adl.repo_inventory.packet_summary.v1",
    "repo_root": ".",
    "source": raw.get("source", "git"),
    "inventory": raw["inventory"],
}
inventory_summary_path.write_text(
    json.dumps(sanitized_inventory, indent=2) + "\n",
    encoding="utf-8",
)

packet_manifest = {
    "schema": "adl.v089.multi_agent_repo_review.packet.v1",
    "milestone": "v0.89",
    "title": "Multi-agent repo code review demo packet",
    "review_target": ".",
    "review_mode": "mixed",
    "gate": "release-tail",
    "selection_policy": {
        "inventory_source": "adl/tools/skills/repo-code-review/scripts/repo_inventory.py",
        "strategy": "bounded high-signal repo packet",
        "selected_paths_count": len(selected_paths),
    },
    "selected_paths": selected_paths,
    "included_artifacts": [
        "review_packet/inventory_summary.json",
        "review_packet/selected_paths.txt",
    ],
    "reviewers": [
        {"id": "code-reviewer", "focus": "correctness, design fit, maintainability"},
        {"id": "security-reviewer", "focus": "trust boundaries, dangerous defaults, abuse paths"},
        {"id": "test-reviewer", "focus": "coverage gaps, failure-path proof, regression protection"},
        {"id": "docs-reviewer", "focus": "README, operator guidance, docs-to-code coherence"},
        {"id": "synthesis-reviewer", "focus": "deduplication, severity ordering, final action plan"},
    ],
    "non_goals": [
        "autonomous merge approval",
        "autofix or code rewriting",
        "unbounded repo browsing outside the packet",
    ],
}
packet_manifest_path.write_text(
    json.dumps(packet_manifest, indent=2) + "\n",
    encoding="utf-8",
)
PY

write_code_review() {
  cat >"$CODE_REVIEW" <<EOF
## Metadata
- Review Type: repo_review
- Subject: bounded ADL repository packet
- Reviewer: code-reviewer
- Date: $run_date
- Input Surfaces:
  - review_packet/review_packet_manifest.json
  - review_packet/inventory_summary.json
  - review_packet/selected_paths.txt
- Output Location: reviewers/code_review.md

## Scope
- Reviewed: README/demo entrypoints, workflow conductor routing surface, PR control-plane shell, review-surface contract docs
- Not Reviewed: runtime execution internals outside the selected packet, live provider invocations, generated artifacts
- Review Mode: mixed
- Gate: release-tail

## Findings
1. [P3] Workflow conductor routing remains concentrated in one large dispatch surface
Location: adl/tools/skills/workflow-conductor/scripts/route_workflow.py
Impact: policy and dispatch changes remain harder to review safely because state collection, blocker classification, dispatch planning, and artifact rendering all move together.
Trigger: extending the conductor for a new blocker family or lifecycle override path.
Evidence: the selected packet shows route_workflow.py owning workflow-state inspection, handoff classification, command planning, dispatch execution, and routing-artifact output in one primary script.
Fix Direction: split state collection, dispatch planning, and artifact rendering into smaller tracked modules while keeping the conductor thin.

## System-Level Assessment
The packet shows a thoughtful review/process substrate, but the workflow conductor still carries enough branching and orchestration detail that maintainability risk is starting to accumulate even when behavior is correct.

## Recommended Action Plan
- Fix now: none
- Fix before milestone closeout: factor the conductor script into smaller reviewable units
- Defer: broader lifecycle redesign

## Follow-ups / Deferred Work
- Link the module split to the existing Rust size / maintainability tracking surfaces if the conductor keeps growing.

## Final Assessment
No blocking code-health findings in the bounded packet. The main code-review concern is maintainability risk in the conductor routing surface, not a demonstrated correctness break.
EOF
}

write_security_review() {
  cat >"$SECURITY_REVIEW" <<EOF
## Metadata
- Review Type: repo_review
- Subject: bounded ADL repository packet
- Reviewer: security-reviewer
- Date: $run_date
- Input Surfaces:
  - review_packet/review_packet_manifest.json
  - review_packet/inventory_summary.json
  - review_packet/selected_paths.txt
- Output Location: reviewers/security_review.md

## Scope
- Reviewed: workflow-control entrypoints, review-surface contract rules, packet contents for trust-boundary signals
- Not Reviewed: live network providers, credentials, external services, non-packet runtime modules
- Review Mode: mixed
- Gate: release-tail

## Findings
No material findings.

## System-Level Assessment
The bounded packet does not show an immediate security blocker. The review-surface contract explicitly bans absolute host paths, secrets, raw prompts, and raw tool arguments, which is the right default posture for this demo class.

## Recommended Action Plan
- Fix now: none
- Fix before milestone closeout: none from this packet
- Defer: a wider security review only if the demo later grows live-provider writeback or PR-thread side effects

## Follow-ups / Deferred Work
- None.

## Final Assessment
Within the packet boundary, the demo shape appears security-conscious and bounded. No blocking security findings were identified in this review slice.
EOF
}

write_test_review() {
  cat >"$TEST_REVIEW" <<EOF
## Metadata
- Review Type: repo_review
- Subject: bounded ADL repository packet
- Reviewer: test-reviewer
- Date: $run_date
- Input Surfaces:
  - review_packet/review_packet_manifest.json
  - review_packet/inventory_summary.json
  - review_packet/selected_paths.txt
- Output Location: reviewers/test_review.md

## Scope
- Reviewed: workflow-conductor contract tests, repo-review skill contract, demo/readme validation surfaces in the packet
- Not Reviewed: full runtime test inventory outside the selected paths, GitHub Actions history, live provider integration tests
- Review Mode: mixed
- Gate: release-tail

## Findings
1. [P3] Conductor lifecycle handoff proof still leans heavily on contract-style shell coverage
Location: adl/tools/test_workflow_conductor_skill_contracts.sh
Impact: routing and override regressions can survive when the contract text still matches but a real handoff path changes subtly.
Trigger: changing worktree-vs-root dispatch behavior, override placeholder names, or finish/janitor handoff semantics.
Evidence: the packet's main conductor proof surface is one large shell contract test; recent lifecycle work already depended on careful command_override and bound-worktree behavior.
Fix Direction: add a small end-to-end proof fixture for each critical lifecycle handoff family, especially run-to-finish and finish-to-janitor paths.

## System-Level Assessment
The packet shows meaningful contract testing, but the strongest remaining risk is semantic drift between conductor routing intent and real lifecycle execution paths.

## Recommended Action Plan
- Fix now: none
- Fix before milestone closeout: add at least one end-to-end conductor handoff proof path
- Defer: broader test-suite reshaping outside the workflow surfaces

## Follow-ups / Deferred Work
- Reuse the new multi-agent demo validator style for future conductor artifact checks when practical.

## Final Assessment
No blocking test gap is demonstrated in the packet, but the conductor lifecycle deserves one stronger end-to-end proof layer before we treat it as fully mature.
EOF
}

write_docs_review() {
  cat >"$DOCS_REVIEW" <<EOF
## Metadata
- Review Type: repo_review
- Subject: bounded ADL repository packet
- Reviewer: docs-reviewer
- Date: $run_date
- Input Surfaces:
  - review_packet/review_packet_manifest.json
  - review_packet/inventory_summary.json
  - review_packet/selected_paths.txt
- Output Location: reviewers/docs_review.md

## Scope
- Reviewed: repository demo index, review-surface contract, repo-code-review skill docs in the selected packet
- Not Reviewed: milestone docs outside the packet, historical release docs, external review PDFs
- Review Mode: mixed
- Gate: release-tail

## Findings
1. [P4] Review guidance remains split across multiple operator entrypoints
Location: demos/README.md
Impact: a new operator has to synthesize the demo index, review-surface contract, and skill contract together before understanding the canonical repo-review story.
Trigger: onboarding a reviewer to a new review demo or provider-facing review surface.
Evidence: the bounded packet spreads review guidance across the demo index, review-surface contract doc, and repo-code-review skill bundle rather than one canonical multi-agent repo-review page.
Fix Direction: maintain one demo-centric guide that links packet contract, reviewer roles, and artifact expectations in one place.

## System-Level Assessment
The docs packet is truthful and useful, but the review story still asks the reader to assemble the narrative from several surfaces.

## Recommended Action Plan
- Fix now: none
- Fix before milestone closeout: add one canonical demo page tying the review packet, reviewer roles, and synthesis artifact together
- Defer: broader doc taxonomy cleanup

## Follow-ups / Deferred Work
- None.

## Final Assessment
The docs surface is serviceable and non-blocking, but still a little too distributed for a reviewer-first demo experience.
EOF
}

write_code_review &
code_pid=$!
write_security_review &
security_pid=$!
write_test_review &
test_pid=$!
write_docs_review &
docs_pid=$!
wait "$code_pid" "$security_pid" "$test_pid" "$docs_pid"

cat >"$CROSS_REVIEW" <<EOF
# Cross Review Notes

- code-reviewer confirms the test-reviewer finding: conductor behavior is easiest to trust when route selection and lifecycle handoff both have end-to-end proof.
- test-reviewer agrees the code-reviewer finding is primarily maintainability risk, not a demonstrated correctness bug in the packet.
- security-reviewer did not elevate any blocking trust-boundary issue from the packet and agrees the current concerns are non-blocking.
- docs-reviewer notes the synthesis artifact should explicitly tell operators there are no blocking findings so the lower-priority observations are not misread as release blockers.
EOF

cat >"$FINAL_SYNTHESIS" <<EOF
## Metadata
- Review Type: repo_review
- Subject: bounded ADL repository packet
- Reviewer: synthesis-reviewer
- Date: $run_date
- Input Surfaces:
  - review_packet/review_packet_manifest.json
  - reviewers/code_review.md
  - reviewers/security_review.md
  - reviewers/test_review.md
  - reviewers/docs_review.md
  - reviewers/cross_review_notes.md
- Output Location: synthesis/final_synthesis_review.md

## Scope
- Reviewed: packet manifest, selected repo surfaces, specialist review artifacts, one bounded cross-review pass
- Not Reviewed: live provider behavior, full repo outside the packet, automatic fix proposals
- Review Mode: mixed
- Gate: release-tail

## Findings
Blocking Findings: none.

Lower-Priority Observations:
1. [P3] Workflow conductor routing remains concentrated in one large dispatch surface
Location: adl/tools/skills/workflow-conductor/scripts/route_workflow.py
Impact: maintainability and safe review cost rise together when routing, dispatch, and artifact behavior change in one place.
Trigger: adding new conductor blocker families or lifecycle overrides.
Evidence: code-reviewer identified the conductor script as the central surface for state collection, dispatch planning, and artifact output.
Fix Direction: split the conductor into smaller tracked modules while preserving the thin-orchestrator model.

2. [P3] Conductor lifecycle handoff proof should include stronger end-to-end coverage
Location: adl/tools/test_workflow_conductor_skill_contracts.sh
Impact: semantic dispatch regressions can slip past contract-only proof.
Trigger: changing command overrides, worktree dispatch, or finish/janitor transitions.
Evidence: test-reviewer found the main proof surface is still heavily contract-oriented.
Fix Direction: add one end-to-end proof fixture per critical lifecycle handoff family.

3. [P4] Review guidance remains spread across several operator surfaces
Location: demos/README.md
Impact: new reviewers need extra synthesis effort before they find the canonical multi-agent repo-review path.
Trigger: onboarding to a new review demo or reviewer package.
Evidence: docs-reviewer found the user-facing story split between demo index, review-surface contract, and skill docs.
Fix Direction: add one canonical multi-agent repo-review demo page.

## System-Level Assessment
The specialist reviewers converged on the same overall result: the bounded review packet looks trustworthy enough for a demo/reviewer surface, with no blocking findings, but there is clear room to tighten conductor maintainability, conductor proof depth, and reviewer-facing documentation cohesion.

## Recommended Action Plan
- Fix now: none
- Fix before milestone closeout: split the conductor surface more cleanly if it keeps growing; add one end-to-end conductor handoff proof path; publish one canonical multi-agent repo-review demo page
- Defer: full-repo autonomous review ambitions or merge authority

## Follow-ups / Deferred Work
- Consider a live-provider extension only after the bounded local packet path remains easy to review.
- If a maintainability reviewer role is added later, keep it separate from the code-reviewer role instead of broadening this MVP.

## Final Assessment
This packet demonstrates a credible multi-agent repo review demo shape: specialist reviewers stay distinct, the synthesis is findings-first, and the final output clearly distinguishes blocking findings from lower-priority observations. It is a demo review surface, not autonomous merge authority.
EOF

python3 - "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

manifest = {
    "schema_version": "adl.v089.multi_agent_repo_review_demo.v1",
    "milestone": "v0.89",
    "title": "Multi-agent repo code review demo",
    "artifact_root": "artifacts/v089/multi_agent_repo_code_review",
    "packet_manifest": "review_packet/review_packet_manifest.json",
    "reviewer_artifacts": [
        "reviewers/code_review.md",
        "reviewers/security_review.md",
        "reviewers/test_review.md",
        "reviewers/docs_review.md",
        "reviewers/cross_review_notes.md",
    ],
    "synthesis_artifact": "synthesis/final_synthesis_review.md",
    "roles": [
        "code-reviewer",
        "security-reviewer",
        "test-reviewer",
        "docs-reviewer",
        "synthesis-reviewer",
    ],
    "execution_shape": {
        "packet_build": "deterministic",
        "specialist_reviewers": "parallel",
        "cross_review_rounds": 1,
        "autonomous_merge_authority": False,
    },
}
Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<'EOF'
# v0.89 Multi-Agent Repo Code Review Demo

Canonical command:

```bash
bash adl/tools/demo_v089_multi_agent_repo_code_review.sh
```

Reviewer flow:
- inspect `demo_manifest.json`
- inspect `review_packet/review_packet_manifest.json`
- read the four specialist reviewer artifacts
- read `reviewers/cross_review_notes.md`
- finish with `synthesis/final_synthesis_review.md`

Important boundary:
- this is a bounded review demo
- it does not claim autonomous merge authority
- it keeps the review packet explicit and stable
EOF

python3 "$VALIDATOR" "$OUT_DIR"

echo "v0.89 multi-agent repo code review demo:"
echo "  artifacts/v089/multi_agent_repo_code_review/demo_manifest.json"
echo "  artifacts/v089/multi_agent_repo_code_review/review_packet/review_packet_manifest.json"
echo "  artifacts/v089/multi_agent_repo_code_review/synthesis/final_synthesis_review.md"

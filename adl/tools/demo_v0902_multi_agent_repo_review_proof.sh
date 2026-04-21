#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0902/multi_agent_repo_review_proof}"

artifact_label="custom-artifact-root"
case "$OUT_DIR" in
  "$ROOT_DIR"/*)
    artifact_label="${OUT_DIR#"$ROOT_DIR"/}"
    ;;
esac

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"/{review_packet,specialist_reviews,synthesis,quality_gate}

python3 - "$OUT_DIR" "$artifact_label" <<'PY'
import json
import sys
from pathlib import Path

out = Path(sys.argv[1])
artifact_label = sys.argv[2]
packet = out / "review_packet"
specialists = out / "specialist_reviews"
synthesis = out / "synthesis"
quality = out / "quality_gate"


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text.strip() + "\n", encoding="utf-8")


def write_json(path: Path, payload: dict) -> None:
    write(path, json.dumps(payload, indent=2, sort_keys=True))


manifest = {
    "schema_version": "adl.v0902.multi_agent_repo_review_proof.v1",
    "demo_id": "v0902-multi-agent-repo-review-proof",
    "classification": "proving_fixture",
    "classification_reason": "Deterministic fixture-backed packet proving review packet shape, role boundaries, synthesis semantics, and publication gates without live provider execution.",
    "artifact_root": artifact_label,
    "publication_allowed": False,
    "merge_approval_claimed": False,
    "live_provider_execution": False,
    "skills_represented": [
        "repo-packet-builder",
        "repo-review-code",
        "repo-review-security",
        "repo-review-tests",
        "repo-review-docs",
        "repo-review-synthesis",
        "review-quality-evaluator",
        "redaction-and-evidence-auditor",
    ],
    "required_artifacts": [
        "review_packet/repo_scope.md",
        "review_packet/evidence_index.json",
        "review_packet/specialist_assignments.json",
        "specialist_reviews/code.md",
        "specialist_reviews/security.md",
        "specialist_reviews/tests.md",
        "specialist_reviews/docs.md",
        "synthesis/final_findings_first_review.md",
        "synthesis/coverage_matrix.json",
        "quality_gate/review_quality_evaluation.md",
        "quality_gate/redaction_and_publication_gate.md",
        "README.md",
    ],
}
write_json(out / "run_manifest.json", manifest)

write(
    packet / "repo_scope.md",
    """
# Repository Review Packet Scope

## Review Target

- Repository: `agent-design-language`
- Review mode: fixture-backed ADL self-review packet
- Demo issue: `#2273`
- Review intent: prove a serious multi-agent repo-review artifact shape

## Included Surfaces

- `README.md`
- `demos/README.md`
- `demos/v0.89/multi_agent_repo_code_review_demo.md`
- `demos/v0.90/codebuddy_multi_agent_review_showcase_demo.md`
- `adl/tools/demo_v089_multi_agent_repo_code_review.sh`
- `adl/tools/demo_v090_codebuddy_review_showcase.sh`
- local repo-review skill contracts installed for this operator

## Excluded Surfaces

- live customer repositories
- live model/provider calls
- GitHub mutation, remediation PR creation, and tracker issue creation
- full repository security audit
- customer-facing publication

## Packet Policy

The packet is designed to be inspectable without hidden chat state. It may prove
review shape, specialist role boundaries, severity preservation, non-finding
handling, residual-risk visibility, and publication gates. It must not claim
merge approval, remediation completion, customer-readiness, or live
multi-provider review execution.
""",
)

write_json(
    packet / "evidence_index.json",
    {
        "schema_version": "adl.v0902.repo_review.evidence_index.v1",
        "evidence": [
            {
                "path": "demos/v0.89/multi_agent_repo_code_review_demo.md",
                "category": "baseline_demo",
                "reason": "Shows the original bounded specialist review demo and artifact family.",
                "lanes": ["docs", "synthesis"],
            },
            {
                "path": "demos/v0.90/codebuddy_multi_agent_review_showcase_demo.md",
                "category": "product_shape",
                "reason": "Shows the larger CodeBuddy packet and staged lane boundary.",
                "lanes": ["docs", "security", "synthesis", "quality"],
            },
            {
                "path": "adl/tools/demo_v089_multi_agent_repo_code_review.sh",
                "category": "demo_generator",
                "reason": "Generates the original role artifacts and synthesis packet.",
                "lanes": ["code", "tests"],
            },
            {
                "path": "adl/tools/demo_v090_codebuddy_review_showcase.sh",
                "category": "demo_generator",
                "reason": "Generates the CodeBuddy showcase packet with redaction and quality lanes.",
                "lanes": ["code", "security", "tests", "docs"],
            },
        ],
    },
)

write_json(
    packet / "specialist_assignments.json",
    {
        "schema_version": "adl.v0902.repo_review.specialist_assignments.v1",
        "assignments": [
            {
                "role": "code",
                "skill": "repo-review-code",
                "focus": "behavioral correctness, maintainability, packet generator risks",
                "artifact": "specialist_reviews/code.md",
            },
            {
                "role": "security",
                "skill": "repo-review-security",
                "focus": "publication boundary, secret/path leakage, mutation boundary",
                "artifact": "specialist_reviews/security.md",
            },
            {
                "role": "tests",
                "skill": "repo-review-tests",
                "focus": "validator strength, fail-closed checks, staged-lane proof",
                "artifact": "specialist_reviews/tests.md",
            },
            {
                "role": "docs",
                "skill": "repo-review-docs",
                "focus": "reviewer navigation, non-goals, product-shape clarity",
                "artifact": "specialist_reviews/docs.md",
            },
            {
                "role": "synthesis",
                "skill": "repo-review-synthesis",
                "focus": "severity preservation, dedupe, residual risk, caveats",
                "artifact": "synthesis/final_findings_first_review.md",
            },
        ],
    },
)

write(
    specialists / "code.md",
    """
# Specialist Review: Code

## Scope

- Reviewed: fixture packet generators, demo artifact shape, validator coupling.
- Not reviewed: full runtime internals, live provider execution, customer repos.

## Findings

### Finding MR-CODE-001: [P2] Demo correctness depends on validating review semantics, not only file existence

- Evidence: The older v0.89 demo already generated role files, while the
  harder v0.90.2 claim requires severity, evidence, residual risk, and
  non-finding semantics.
- Impact: A future packet could look complete while quietly dropping severity,
  caveats, or non-finding handling.
- Recommended action: Keep semantic assertions in the validator for severity
  markers, evidence fields, non-finding text, residual-risk sections, and
  publication gates.
- Validation gap: This fixture does not measure live model review quality.
- Related findings: MR-TEST-001.

## Non-Findings

- No evidence of unintended repository mutation in the bounded demo design.
- No evidence that this fixture claims remediation or merge approval.

## Residual Risk

Live specialist reviewers may produce noisier, weaker, or contradictory output
than this deterministic packet. That is intentionally out of scope here.
""",
)

write(
    specialists / "security.md",
    """
# Specialist Review: Security

## Scope

- Reviewed: publication boundary, private path/secret leakage risks, mutation
  boundaries.
- Not reviewed: full secret scanner coverage, live provider payloads, customer
  repositories.

## Findings

No material security findings in this bounded fixture packet.

## Non-Findings

- The packet sets `publication_allowed=false`.
- The packet does not claim customer readiness.
- The packet does not perform live provider calls or GitHub mutation.
- The validation path scans generated artifacts for private host paths and
  common secret-like tokens.

## Residual Risk

Real customer reports require the `redaction-and-evidence-auditor` lane and
manual review before publication. This fixture only proves that the demo packet
keeps that gate explicit.
""",
)

write(
    specialists / "tests.md",
    """
# Specialist Review: Tests

## Scope

- Reviewed: packet validator expectations, fail-closed demo checks, staged
  proof status.
- Not reviewed: full CI history, live model outputs, customer repo fixtures.

## Findings

### Finding MR-TEST-001: [P2] Packet validators must fail closed on overclaiming

- Evidence: The demo's value depends on refusing publication, merge approval,
  and customer-ready claims until explicit gates pass.
- Impact: If the validator only checks artifact existence, a future regression
  could overstate demo proof quality.
- Recommended action: Validate `publication_allowed=false`, absence of merge
  approval language, presence of explicit non-findings, and required synthesis
  sections.
- Validation gap: The validator does not judge whether live reviewer prose is
  high quality.
- Related findings: MR-CODE-001.

## Non-Findings

- No missing required artifact in the generated packet.
- No unstated live-provider dependency in the demo path.

## Residual Risk

The quality of real specialist output still needs `review-quality-evaluator`
or human review after live execution.
""",
)

write(
    specialists / "docs.md",
    """
# Specialist Review: Docs

## Scope

- Reviewed: demo entrypoint clarity, review packet story, reader-facing
  boundaries.
- Not reviewed: product website copy, external customer reports, full docs
  taxonomy.

## Findings

### Finding MR-DOCS-001: [P3] The demo needs one reviewer-first walkthrough

- Evidence: The v0.89 baseline and v0.90 CodeBuddy showcase are useful, but
  readers need one concise v0.90.2 page explaining what is stricter here.
- Impact: Without a walkthrough, the new packet can look like another artifact
  dump rather than a serious review proof surface.
- Recommended action: Add a demo page that names the packet root, role order,
  generated artifacts, validation command, non-goals, and publication boundary.
- Validation gap: Human editorial review may still improve the walkthrough.
- Related findings: none.

## Non-Findings

- No evidence that the docs should promise customer-ready publication.
- No evidence that the demo should replace the broader CodeBuddy showcase.

## Residual Risk

The walkthrough is still a demo doc, not a product onboarding page.
""",
)

write_json(
    synthesis / "coverage_matrix.json",
    {
        "schema_version": "adl.v0902.repo_review.coverage_matrix.v1",
        "roles": {
            "code": {"artifact": "specialist_reviews/code.md", "status": "represented", "findings": 1, "non_findings": True},
            "security": {"artifact": "specialist_reviews/security.md", "status": "represented", "findings": 0, "non_findings": True},
            "tests": {"artifact": "specialist_reviews/tests.md", "status": "represented", "findings": 1, "non_findings": True},
            "docs": {"artifact": "specialist_reviews/docs.md", "status": "represented", "findings": 1, "non_findings": True},
            "synthesis": {"artifact": "synthesis/final_findings_first_review.md", "status": "represented", "findings": 3},
        },
        "required_roles_present": True,
        "disagreements": [
            "Security reports no material findings while code/tests/docs preserve P2/P3 quality risks. Synthesis keeps both facts visible.",
        ],
    },
)

write(
    synthesis / "final_findings_first_review.md",
    """
# Final Findings-First Multi-Agent Repo Review

## Findings

### Finding MR-CODE-001: [P2] Demo correctness depends on validating review semantics, not only file existence

- Source role: code
- Evidence: the older baseline proves role files exist; this harder proof
  requires semantic checks for severity, evidence, caveats, non-findings, and
  publication gates.
- Impact: a packet can look review-shaped while losing the facts reviewers need
  to trust it.
- Recommended action: keep semantic assertions in the validator.
- Residual risk: live model review quality is not measured by this fixture.

### Finding MR-TEST-001: [P2] Packet validators must fail closed on overclaiming

- Source role: tests
- Evidence: the demo must not claim publication, customer readiness, merge
  approval, or live provider execution.
- Impact: weak validation would let a future demo overstate proof quality.
- Recommended action: validate no-publication and no-merge-approval boundaries.
- Residual risk: human quality review is still needed for live outputs.

### Finding MR-DOCS-001: [P3] The demo needs one reviewer-first walkthrough

- Source role: docs
- Evidence: v0.89 and v0.90 surfaces are useful but distributed.
- Impact: readers may see artifacts without understanding the stricter v0.90.2
  proof contract.
- Recommended action: provide a concise v0.90.2 demo page.
- Residual risk: product onboarding remains out of scope.

## Explicit Non-Findings

- Security found no material issue in this bounded fixture packet.
- The packet does not perform live provider calls.
- The packet does not mutate GitHub or create remediation PRs.
- The packet does not claim merge approval, remediation completion, or customer
  publication readiness.

## Specialist Coverage Matrix

| Role | Status | Finding Count | Non-Finding Handling |
| --- | --- | ---: | --- |
| code | represented | 1 | explicit |
| security | represented | 0 | explicit no material findings |
| tests | represented | 1 | explicit |
| docs | represented | 1 | explicit |
| synthesis | represented | 3 | preserves all roles |

## Dedupe And Disagreement Notes

- MR-CODE-001 and MR-TEST-001 overlap around validator strength, but they are
  preserved separately because one is about proof semantics and the other is
  about fail-closed testing.
- Security's no-finding result is not erased by synthesis; it is recorded as an
  explicit non-finding.

## Residual Risk

- This packet is deterministic and fixture-backed.
- Live multi-agent review quality, false-positive rate, and customer-specific
  usefulness remain unmeasured.
- Redaction must run before any customer-facing output.

## Review Boundary

This review is not merge approval. It is not remediation completion. It is not
customer-ready publication. It is a proof that the multi-agent repo-review demo
can now emit a serious, inspectable, findings-first packet.
""",
)

write(
    quality / "review_quality_evaluation.md",
    """
# Review Quality Evaluation

## Status

PASS for internal demo proof.

## Checks

- Findings first: PASS
- Severity markers present: PASS
- Evidence and impact present for each finding: PASS
- Explicit non-findings present: PASS
- Residual risk visible: PASS
- Specialist coverage visible: PASS
- Publication boundary visible: PASS
- Merge approval absent: PASS

## Publication Boundary

Customer-facing publication remains blocked. The packet is fixture-backed and
does not include a live customer repository, live provider review, or external
redaction approval.
""",
)

write(
    quality / "redaction_and_publication_gate.md",
    """
# Redaction And Publication Gate

## Status

- Redaction check: PASS for controlled fixture packet
- Publication allowed: false
- Customer-ready report: false
- Merge approval claimed: false

## Notes

The generated packet uses repository-relative paths and controlled fixture
text. Any future customer-facing packet must run the full
redaction-and-evidence-auditor lane before publication.
""",
)

write(
    out / "README.md",
    """
# v0.90.2 Multi-Agent Repo Review Proof Packet

This packet hardens the older v0.89 multi-agent repo-review demo into a more
serious findings-first proof surface.

## Review Order

1. `review_packet/repo_scope.md`
2. `review_packet/evidence_index.json`
3. `review_packet/specialist_assignments.json`
4. `specialist_reviews/code.md`
5. `specialist_reviews/security.md`
6. `specialist_reviews/tests.md`
7. `specialist_reviews/docs.md`
8. `synthesis/final_findings_first_review.md`
9. `quality_gate/review_quality_evaluation.md`
10. `quality_gate/redaction_and_publication_gate.md`

## Classification

`proving_fixture`: the packet proves artifact shape, role boundaries,
findings-first synthesis, non-finding handling, residual risk, and publication
gates. It does not prove live model review quality.

## Non-Goals

- no customer-facing publication
- no merge approval
- no live provider calls
- no remediation PRs or issue creation
""",
)
PY

python3 "$ROOT_DIR/adl/tools/validate_v0902_multi_agent_repo_review_proof.py" "$OUT_DIR" >/dev/null

echo "demo_v0902_multi_agent_repo_review_proof: ok"

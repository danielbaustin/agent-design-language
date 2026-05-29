#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0914/multi_agent_repo_review_serious_proof}"
VALIDATOR="$ROOT_DIR/adl/tools/validate_v0914_multi_agent_repo_review_serious_proof.py"

artifact_label="custom-artifact-root"
case "$OUT_DIR" in
  "$ROOT_DIR"/*)
    artifact_label="${OUT_DIR#"$ROOT_DIR"/}"
    ;;
esac

if [[ -e "$OUT_DIR" ]]; then
  if [[ ! -d "$OUT_DIR" ]]; then
    echo "refusing to overwrite non-directory output path: $OUT_DIR" >&2
    exit 1
  fi
  case "$OUT_DIR" in
    "$ROOT_DIR"/artifacts/v0914/multi_agent_repo_review_serious_proof|\
    "$ROOT_DIR"/artifacts/v0914/multi_agent_repo_review_serious_proof/*|\
    /tmp/*|/private/tmp/*)
      ;;
    *)
      echo "refusing to delete existing custom output directory outside approved demo roots: $OUT_DIR" >&2
      exit 1
      ;;
  esac
fi

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


write_json(
    out / "run_manifest.json",
    {
        "schema_version": "adl.v0914.multi_agent_repo_review_serious_proof.v1",
        "demo_id": "v0914-multi-agent-repo-review-serious-proof",
        "classification": "proving_fixture",
        "classification_reason": "Deterministic review packet showing serious specialist-role review shape, visible heuristic contract, explicit non-findings, and findings-first synthesis without live provider calls.",
        "artifact_root": artifact_label,
        "publication_allowed": False,
        "merge_approval_claimed": False,
        "live_provider_execution": False,
        "heuristics_visible": True,
        "role_caveats_required": True,
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
    },
)

write(
    packet / "repo_scope.md",
    """
# Repository Review Packet Scope

## Review Target

- Repository: `agent-design-language`
- Review mode: deterministic serious-proof fixture
- Review intent: prove a findings-first multi-agent repo-review packet that reads like a serious review artifact

## Included Surfaces

- `demos/v0.89/multi_agent_repo_code_review_demo.md`
- `demos/v0.91.4/multi_agent_repo_review_serious_proof_demo.md`
- `adl/tools/demo_v089_multi_agent_repo_code_review.sh`
- `adl/tools/demo_v0914_multi_agent_repo_review_serious_proof.sh`
- `adl/tools/validate_multi_agent_repo_review_demo.py`
- `.adl/docs/TBD/workflow_tooling/ADL_REVIEW_HEURISTICS.md`

## Excluded Surfaces

- live provider execution
- repository mutation outside fixture artifact generation
- GitHub issue or PR mutation
- customer publication or remediation authority

## Packet Policy

This packet must stay findings-first, evidence-bound, and explicit about
non-findings, disagreements, caveats, residual risk, and publication limits.
It proves review shape, not autonomous approval.
""",
)

write_json(
    packet / "heuristic_contract.json",
    {
        "schema_version": "adl.v0914.repo_review.heuristic_contract.v1",
        "source": ".adl/docs/TBD/workflow_tooling/ADL_REVIEW_HEURISTICS.md",
        "domains": [
            {
                "role": "code",
                "domain": "Functional Correctness",
                "rules": ["F1", "F5", "F10"],
                "caveat": "bounded packet review; not a full runtime proof",
            },
            {
                "role": "security",
                "domain": "Security & Safety",
                "rules": ["S3", "S5", "S8"],
                "caveat": "no live-provider or customer-data review in this fixture",
            },
            {
                "role": "tests",
                "domain": "Testing & Verification",
                "rules": ["T5", "T6", "T7"],
                "caveat": "semantic validator strength is assessed, not live test history",
            },
            {
                "role": "docs",
                "domain": "Cognitive / ADL-Specific",
                "rules": ["C1", "C2", "C10"],
                "caveat": "operator-facing clarity only; not product onboarding",
            },
            {
                "role": "synthesis",
                "domain": "Architecture & System Alignment",
                "rules": ["A1", "A10", "C6"],
                "caveat": "preserves disagreement; does not erase specialist uncertainty",
            },
        ],
    },
)

write_json(
    packet / "evidence_index.json",
    {
        "schema_version": "adl.v0914.repo_review.evidence_index.v1",
        "evidence": [
            {
                "path": "demos/v0.89/multi_agent_repo_code_review_demo.md",
                "category": "baseline_demo",
                "used_by": ["docs", "synthesis"],
                "reason": "Shows the original bounded specialist-reviewer demo and its artifact family.",
            },
            {
                "path": "adl/tools/demo_v089_multi_agent_repo_code_review.sh",
                "category": "baseline_generator",
                "used_by": ["code", "tests"],
                "reason": "Shows the simpler original generator that this proof hardens conceptually.",
            },
            {
                "path": ".adl/docs/TBD/workflow_tooling/ADL_REVIEW_HEURISTICS.md",
                "category": "heuristic_contract",
                "used_by": ["code", "security", "tests", "docs", "synthesis"],
                "reason": "Makes the visible review-domain and rule contract explicit in the packet.",
            },
        ],
    },
)

write_json(
    packet / "specialist_assignments.json",
    {
        "schema_version": "adl.v0914.repo_review.specialist_assignments.v1",
        "assignments": [
            {
                "role": "code",
                "artifact": "specialist_reviews/code.md",
                "heuristic_domain": "Functional Correctness",
                "focus": "behavioral correctness, bounded maintainability risk, and generator coherence",
            },
            {
                "role": "security",
                "artifact": "specialist_reviews/security.md",
                "heuristic_domain": "Security & Safety",
                "focus": "secret/path leakage, trust boundaries, publication limits",
            },
            {
                "role": "tests",
                "artifact": "specialist_reviews/tests.md",
                "heuristic_domain": "Testing & Verification",
                "focus": "validator strength, fail-closed proof, regression coverage",
            },
            {
                "role": "docs",
                "artifact": "specialist_reviews/docs.md",
                "heuristic_domain": "Cognitive / ADL-Specific",
                "focus": "review readability, evidence clarity, reviewer navigation",
            },
            {
                "role": "synthesis",
                "artifact": "synthesis/final_findings_first_review.md",
                "heuristic_domain": "Architecture & System Alignment",
                "focus": "severity ordering, dedupe, disagreement, residual risk",
            },
        ],
    },
)

specialist_template = """
## Metadata
- Review Type: serious_repo_review_fixture
- Reviewer: {role}
- Heuristic Domain: {domain}
- Artifact: {artifact}

## Scope
- Reviewed: {reviewed}
- Not Reviewed: {not_reviewed}

## Findings
{findings}

## Explicit Non-Findings
{non_findings}

## Role-Specific Caveats
{caveats}

## Residual Risk
{residual_risk}

## Recommended Action
{recommended_action}

## Final Assessment
{final_assessment}
"""

write(
    specialists / "code.md",
    specialist_template.format(
        role="code",
        domain="Functional Correctness",
        artifact="specialist_reviews/code.md",
        reviewed="demo generator and packet semantics for correctness of review artifact shape",
        not_reviewed="full repository runtime behavior or live provider quality",
        findings="1. [P2] Findings-first review quality depends on semantic packet fields, not just artifact existence.\n   - Evidence: the serious proof packet must preserve severity, evidence, caveats, and residual risk explicitly.\n   - Recommended Action: keep validator checks on semantic review fields and findings-first ordering.",
        non_findings="- No demonstrated correctness break in the bounded fixture generator itself.",
        caveats="- This role judges review-shape correctness, not production feature correctness.",
        residual_risk="A future packet could stay structurally complete while silently weakening the content quality of findings.",
        recommended_action="Preserve semantic validation and reviewer-role boundaries as the packet evolves.",
        final_assessment="Non-blocking but substantive: the proof surface is credible only if semantic review fields remain enforced.",
    ),
)

write(
    specialists / "security.md",
    specialist_template.format(
        role="security",
        domain="Security & Safety",
        artifact="specialist_reviews/security.md",
        reviewed="publication boundary, secret/path leakage controls, and trust-boundary claims",
        not_reviewed="live credentials, live provider payloads, or customer repositories",
        findings="No material findings.",
        non_findings="- No secret-like markers found in the fixture packet contract.\n- No publication or merge-approval claim is present.",
        caveats="- This no-finding result applies only to the controlled fixture packet, not to arbitrary live review runs.",
        residual_risk="Live provider-backed review packets still require separate redaction and evidence auditing before publication.",
        recommended_action="Keep the no-publication and no-merge-approval gates explicit and validated.",
        final_assessment="Within the bounded fixture, the security posture is controlled and appropriately constrained.",
    ),
)

write(
    specialists / "tests.md",
    specialist_template.format(
        role="tests",
        domain="Testing & Verification",
        artifact="specialist_reviews/tests.md",
        reviewed="validator strength, fail-closed checks, and coverage of review-shape invariants",
        not_reviewed="GitHub Actions history, flaky runtime behavior, or live model disagreement rates",
        findings="1. [P2] The serious proof surface must fail closed on overclaiming and missing heuristic visibility.\n   - Evidence: packet quality is undermined if publication limits, explicit non-findings, or heuristic-domain mappings disappear without detection.\n   - Recommended Action: validate those invariants directly in the proof validator and focused test.",
        non_findings="- No missing required artifact in the bounded fixture set.",
        caveats="- This lane validates packet semantics, not the substantive truth of a live external repository review.",
        residual_risk="A deterministic fixture cannot measure reviewer false-positive or false-negative rates.",
        recommended_action="Keep the validator strict about publication gates, heuristics visibility, and non-finding handling.",
        final_assessment="The proof is convincing only if validation stays semantic and fail-closed.",
    ),
)

write(
    specialists / "docs.md",
    specialist_template.format(
        role="docs",
        domain="Cognitive / ADL-Specific",
        artifact="specialist_reviews/docs.md",
        reviewed="reviewer navigation, clarity of proof boundaries, and evidence readability",
        not_reviewed="broader docs taxonomy or product-facing onboarding",
        findings="1. [P3] Reviewer-facing demo guidance should name the serious proof packet and reading order explicitly.\n   - Evidence: the older v0.89 demo is useful, but the stronger packet needs one canonical walkthrough.\n   - Recommended Action: publish a dedicated v0.91.4 demo page and reference it from the v0.89 baseline page.",
        non_findings="- No evidence that this fixture should claim customer-facing readiness.\n- No evidence that the older v0.89 page should be removed.",
        caveats="- This lane evaluates operator-facing demo clarity, not external marketing narrative.",
        residual_risk="Even a strong walkthrough still relies on readers honoring the bounded proof and non-goals.",
        recommended_action="Keep one reviewer-first page that names artifacts, order, command, and non-goals.",
        final_assessment="The packet becomes much more credible when the walkthrough is concise and explicit.",
    ),
)

write_json(
    synthesis / "coverage_matrix.json",
    {
        "schema_version": "adl.v0914.repo_review.coverage_matrix.v1",
        "required_roles_present": True,
        "roles": {
            "code": {"artifact": "specialist_reviews/code.md", "findings": 1, "explicit_non_findings": True},
            "security": {"artifact": "specialist_reviews/security.md", "findings": 0, "explicit_non_findings": True},
            "tests": {"artifact": "specialist_reviews/tests.md", "findings": 1, "explicit_non_findings": True},
            "docs": {"artifact": "specialist_reviews/docs.md", "findings": 1, "explicit_non_findings": True},
            "synthesis": {"artifact": "synthesis/final_findings_first_review.md", "findings": 3, "explicit_non_findings": True},
        },
        "disagreements": [
            "Security reports no material findings while other lanes preserve P2/P3 proof-quality risks; synthesis keeps both facts visible.",
        ],
    },
)

write(
    synthesis / "final_findings_first_review.md",
    """
## Findings
1. [P2] Findings-first review quality depends on semantic packet fields, not just artifact existence.
   - Source Role: code
   - Evidence: the packet now carries explicit severity, evidence, caveats, and residual-risk expectations.
   - Recommended Action: preserve semantic validator checks on those fields.
   - Residual Risk: a future fixture could still weaken prose quality while passing structure-only checks.

2. [P2] The proof validator must fail closed on missing heuristic visibility and overclaiming.
   - Source Role: tests
   - Evidence: the serious packet claims visible heuristic domains, no-publication boundaries, and explicit non-finding handling.
   - Recommended Action: validate those invariants directly.
   - Residual Risk: deterministic fixtures cannot measure live-review accuracy.

3. [P3] The serious proof needs one reviewer-first walkthrough.
   - Source Role: docs
   - Evidence: the stronger packet is most useful when the command, artifact order, and non-goals are named in one place.
   - Recommended Action: keep the dedicated v0.91.4 demo page and baseline pointer.
   - Residual Risk: product onboarding remains out of scope.

## Explicit Non-Findings
- Security found no material findings in the bounded fixture packet.
- The packet does not claim merge approval.
- The packet does not claim customer-ready publication.
- The packet does not perform live provider execution.

## Specialist Coverage Matrix
- code: represented, 1 finding, explicit non-findings present
- security: represented, 0 findings, explicit no-finding statement present
- tests: represented, 1 finding, explicit non-findings present
- docs: represented, 1 finding, explicit non-findings present
- synthesis: represented, findings preserved without erasing disagreement

## Dedupe And Disagreement Notes
- The code and tests lanes both care about semantic proof strength, but code focuses on review-shape correctness while tests focuses on fail-closed validation.
- Security's no-finding result is preserved explicitly rather than being drowned out by quality-hardening findings.

## Residual Risk
- This packet is still deterministic and fixture-backed.
- It does not prove live multi-agent review quality or customer-readiness.
- Any future live packet still requires redaction and evidence auditing before publication.

## Review Boundary
This review is not merge approval, remediation completion, or customer publication approval. It is a serious bounded proof surface for multi-agent repo-review packet quality.
""",
)

write(
    quality / "review_quality_evaluation.md",
    """
# Review Quality Evaluation

## Status
PASS for internal serious-proof demo.

## Checks
- Findings first: PASS
- Severity and evidence visible: PASS
- Explicit non-findings present: PASS
- Role-specific caveats present: PASS
- Residual risk visible: PASS
- Heuristic domains visible: PASS
- Publication boundary visible: PASS
- Merge approval absent: PASS
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
- Live provider execution: false

## Notes
This packet is intentionally controlled and repository-relative. Any future live-review publication still requires the redaction-and-evidence-auditor path.
""",
)

write(
    out / "README.md",
    """
# v0.91.4 Multi-Agent Repo Review Serious Proof

Canonical command:

```bash
bash adl/tools/demo_v0914_multi_agent_repo_review_serious_proof.sh
```

Recommended reading order:
1. `review_packet/repo_scope.md`
2. `review_packet/heuristic_contract.json`
3. `review_packet/evidence_index.json`
4. `review_packet/specialist_assignments.json`
5. `specialist_reviews/code.md`
6. `specialist_reviews/security.md`
7. `specialist_reviews/tests.md`
8. `specialist_reviews/docs.md`
9. `synthesis/final_findings_first_review.md`
10. `quality_gate/review_quality_evaluation.md`
11. `quality_gate/redaction_and_publication_gate.md`

Classification: `proving_fixture`

This packet proves a serious findings-first review shape with visible heuristics,
explicit non-findings, role-specific caveats, and publication boundaries. It
does not prove live model review quality or autonomous merge authority.
""",
)
PY

python3 "$VALIDATOR" "$OUT_DIR" >/dev/null

echo "demo_v0914_multi_agent_repo_review_serious_proof: ok"

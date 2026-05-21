# Specialist Coverage

Date: 2026-05-21
Issue: #3173

| Lane | Skill | Status | Notes |
|---|---|---:|---|
| Packet | `repo-packet-builder` | complete | Built inventory under `packet/`. |
| Code | `repo-review-code` | complete | Found benchmark scoring and provider capability risks. |
| Tests | `repo-review-tests` | partial + local recovery | Initial tests subagent did not produce evidence-backed findings; local focused test review covered benchmark argument checks and publication gates. |
| Docs | `repo-review-docs` | complete | Found external-review handoff and evidence portability risks. |
| Security | `repo-review-security` | complete | Found key-path, localhost adapter, artifact redaction, and path-leak risks. |
| Architecture | `repo-architecture-review` | complete | Found profile/panel mismatch, fail-closed scoring, publication gate, and release evidence architecture risks. |
| Dependencies | `repo-dependency-review` | complete | Found key-path portability, floating action, auto-install, and stale helper risks. |
| Redaction | `redaction-and-evidence-auditor` | complete | Derived from security/docs findings; current packet is local/internal only until redaction findings are fixed. |
| Quality | `review-quality-evaluator` | complete | Packet is useful for internal remediation, not publication-ready. |
| Test planning | `review-to-test-planner` | complete | Maps findings to missing tests. |
| Issue planning | `finding-to-issue-planner` | complete | Groups findings into remediation issue candidates. |
| Diagram planning | `repo-diagram-planner` | complete | Proposes diagrams for benchmark evidence architecture and release control plane. |
| Synthesis | `repo-review-synthesis` | complete | Severity-preserving findings register and final report. |

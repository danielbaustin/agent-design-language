# Review Quality Acceptance Checklist

Use this checklist when evaluating whether a bounded ADL review packet or report
meets the `WP-07` quality bar.

## Evidence And Findings

- Every finding cites concrete source evidence.
- Impact is explained, not implied.
- Severity is justified by the observed failure mode.
- Unsupported approval, compliance, publication, or remediation claims are
  absent.

## Coverage And Structure

- Required specialist lanes are present or explicitly skipped.
- The synthesis artifact preserves coverage gaps and disagreement notes.
- Residual risk and non-reviewed surfaces are visible.
- Review output stays structured enough for later synthesis and issue planning.

## Heuristic Discipline

- The packet does not invent findings without source evidence.
- The packet does not flatten all review domains into one generic summary.
- The packet preserves bounded stop points before publication or mutation.
- Caveats remain visible when evidence is incomplete.

## Product And Publication Boundary

- Customer-facing language does not overclaim automated review authority.
- The quality gate status is recorded truthfully as `pass`, `partial`, `fail`,
  or `not_run`.
- Missing redaction, missing evidence, or missing sections stay blocking when
  publication is in scope.

## Acceptance Result

For `WP-07`, the checklist is proving only when:

- the checklist can be applied to the bounded fixture outputs above
- the result stays consistent with the review-quality-evaluator contract
- the packet remains deterministic in fixture mode

This checklist is not itself a publication approval or release approval.

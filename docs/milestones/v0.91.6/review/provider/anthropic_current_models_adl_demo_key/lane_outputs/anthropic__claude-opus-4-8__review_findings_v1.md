# anthropic:claude-opus-4-8 / review_findings_v1

Candidate: `anthropic:claude-opus-4-8`
Lane: `anthropic`
Model: `claude-opus-4-8`

## Output

# Findings

The validation evidence relies on raw `gh pr view` CLI snapshots (mergeStateStatus, state) presented as current sprint proof. This reflects evidence-provenance drift: manual GitHub CLI outputs are historical/manual provenance, not reproducible ADL-native proof. The snapshots capture a point-in-time view that cannot be re-verified within the current pipeline, weakening their standing as authoritative current-sprint evidence.

# Severity

P3

# Routing

Route to the evidence-provenance maintainers to migrate manual `gh` snapshots toward Octocrab-backed ADL-native proof, ensuring current-sprint claims are reproducible rather than captured by hand.

# Residual Risk

Even after migration, prior `

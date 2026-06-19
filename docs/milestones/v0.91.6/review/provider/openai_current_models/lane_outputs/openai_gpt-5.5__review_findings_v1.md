# openai:gpt-5.5 / review_findings_v1

Candidate: `openai:gpt-5.5`
Lane: `openai`
Model: `gpt-5.5`

## Output

# Findings
The excerpt shows evidence-provenance drift: raw `gh pr view` snapshots are being used as current sprint proof. This should be treated as historical/manual provenance rather than current ADL-native proof.

# Severity
P2

# Routing
Route to evidence/provenance ownership to replace or reclassify the CLI snapshots and ensure sprint validation uses accepted current-source evidence.

# Residual Risk
Residual risk is limited to provenance reliability and audit freshness; no code-correctness claim is made.

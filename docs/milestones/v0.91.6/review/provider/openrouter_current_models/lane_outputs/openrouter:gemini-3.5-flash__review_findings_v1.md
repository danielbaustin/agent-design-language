# openrouter:gemini-3.5-flash / review_findings_v1

Candidate: `openrouter:gemini-3.5-flash`
Lane: `openrouter`
Model: `google/gemini-3.5-flash`

## Output

# Findings
A validation drift exists where OpenRouter route packets are conflated with direct-hosted suitability, improperly substituting for ADL-native proof.

# Severity
P3

# Routing
OpenRouter route evidence must stay distinct from native provider proof. Conflating these routing boundaries weakens the validation integrity of manual GitHub snapshots.

# Residual Risk
Manual snapshot validation is susceptible to subsequent drift; there is no code-correctness claim made here.

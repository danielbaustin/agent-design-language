# openrouter:gpt-5.4 / review_findings_v1

Candidate: `openrouter:gpt-5.4`
Lane: `openrouter`
Model: `openai/gpt-5.4`

## Output

# Findings
The excerpt shows proof-boundary drift: OpenRouter route packets for `anthropic/claude-fable-5` and `openai/gpt-5.4` are being stretched to justify native-provider, direct-hosted suitability. OpenRouter route evidence must stay distinct from native provider proof. The draft note overclaims beyond the supplied evidence. Manual GitHub snapshots as “current publication proof” are also weak unless clearly bounded and paired with ADL-native or Octocrab proof.

# Severity
P2

# Routing
Send to evidence/provenance owners and policy reviewers responsible for model-source validation, route attribution, and publication-proof standards.

# Residual Risk
If uncorrected, downstream reviewers may treat aggregator-route evidence as native-hosting proof and accept stale or informal publication artifacts. Residual risk remains around provenance misclassification and auditability; no code-correctness claim.

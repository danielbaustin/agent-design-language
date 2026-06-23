# openrouter:kimi-k2.7-code / review_findings_v1

Candidate: `openrouter:kimi-k2.7-code`
Lane: `openrouter`
Model: `moonshotai/kimi-k2.7-code`

## Output

# Findings
Proof-boundary drift: the OpenRouter route packet for anthropic/claude-opus-4.8 and openai/gpt-5.4 is treated as also establishing Anthropic and OpenAI direct-hosted suitability. OpenRouter route evidence must stay distinct from native provider proof. Manual GitHub snapshots used as current publication proof lack a clear ADL-native or Octocrab proof trail.

# Severity
P2

# Routing
Route to compliance/validation review for proof-boundary separation.

# Residual Risk
Until native-provider evidence is collected separately, residual risk remains that direct-hosted suitability claims rest on aggregator proof and unverified publication artifacts; no code-correctness claim.

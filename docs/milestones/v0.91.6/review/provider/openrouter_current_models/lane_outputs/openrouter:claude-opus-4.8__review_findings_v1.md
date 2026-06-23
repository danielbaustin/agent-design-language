# openrouter:claude-opus-4.8 / review_findings_v1

Candidate: `openrouter:claude-opus-4.8`
Lane: `openrouter`
Model: `anthropic/claude-opus-4.8`

## Output

# Findings
The draft note conflates OpenRouter route evidence with native provider suitability. An OpenRouter route packet for `anthropic/claude-opus-4.8` and `openai/gpt-5.4` only proves the OpenRouter path; it cannot stand in for Anthropic or OpenAI direct-hosted proof. This is proof-boundary drift: aggregator-route evidence is being stretched to cover native provider claims. Additionally, manual GitHub snapshots are used as current publication proof, where ADL-native or Octocrab proof would provide a durable, verifiable boundary instead of point-in-time captures.

# Severity
P2

# Routing
Return to evidence author to split OpenRouter route evidence from native provider proof, and replace manual snapshots with a programmatic publication source.

# Residual Risk
Even after separation, OpenRouter evidence remains distinct from native provider proof and must not be generalized. Manual snapshots may drift from live state. This review addresses proof-boundary scope only and makes no code-correctness claim.

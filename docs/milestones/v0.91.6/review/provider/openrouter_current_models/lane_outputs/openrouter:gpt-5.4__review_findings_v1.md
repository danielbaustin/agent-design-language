# openrouter:gpt-5.4 / review_findings_v1

Candidate: `openrouter:gpt-5.4`
Lane: `openrouter`
Model: `openai/gpt-5.4`

## Output

# Findings
The excerpt shows proof-boundary drift: OpenRouter route packets for `anthropic/claude-opus-4.8` and `openai/gpt-5.4` are being extended to support claims about direct-hosted Anthropic/OpenAI suitability. That is not the same proof domain. OpenRouter route evidence must stay distinct from native provider proof. Manual GitHub snapshots as “current publication proof” also risk crossing evidence scopes unless tied to a provider-native publication chain such as ADL-native or Octocrab proof.

# Severity
P2

# Routing
Send to evidence/provenance owners and policy reviewers for claim-scope correction. Require separation of aggregator-routing evidence from native-provider suitability evidence, and reclassify GitHub snapshots as secondary/supporting unless independently anchored.

# Residual Risk
If boundaries are not tightened, downstream reviewers may accept unsupported provider-native claims based on aggregator evidence. Residual risk remains around publication freshness and provenance interpretation; no code-correctness claim.

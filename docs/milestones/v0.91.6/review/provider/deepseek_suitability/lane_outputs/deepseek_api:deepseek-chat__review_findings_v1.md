# deepseek_api:deepseek-chat / review_findings_v1

Candidate: `deepseek_api:deepseek-chat`
Lane: `hosted_api`
Model: `deepseek-chat`

## Output

# Findings
Evidence-provenance drift: The validation relies on `gh pr view 4054 --json mergeStateStatus` and `--json state` CLI snapshots, which are manual/historical provenance from raw GitHub CLI. These do not derive from ADL-native automated proof (e.g., Octocrab or CI-bound checks), creating a gap between current enforcement and cited evidence.

# Severity
P3

# Routing
Evidence provenance team; reviewers should verify ADL-native `octocrab` or CI-bound merge-state attestations replace manual CLI output.

# Residual Risk
Evidence may become stale between manual capture and review time; no code-correctness claim.

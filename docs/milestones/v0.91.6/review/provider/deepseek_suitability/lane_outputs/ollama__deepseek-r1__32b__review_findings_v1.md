# ollama:deepseek-r1:32b / review_findings_v1

Candidate: `ollama:deepseek-r1:32b`
Lane: `local_ollama`
Model: `deepseek-r1:32b`

## Output

# Findings  
The validation evidence relies on historical/manual provenance from raw `gh` commands (e.g., `gh pr view 4054 --json mergeStateStatus`) and manual GitHub CLI snapshots. Octocrab or ADL-native proof is not utilized in this context.  

# Severity  
P2  

# Routing  
The findings highlight a reliance on outdated evidence-provenance methods, which may affect the accuracy of current sprint proof.  

# Residual Risk  
no code-correctness claim

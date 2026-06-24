# openrouter:qwen/qwen3-coder-next / review_findings_v1

Candidate: `openrouter:qwen/qwen3-coder-next`
Lane: `openrouter`
Model: `qwen/qwen3-coder-next`

## Output

# Findings  
Scope/provenance drift observed: OpenRouter runs confirm local-model suitability but conflate route equivalence with identical provenance; the claim that the route is "equivalent to any matching local family" lacks explicit scope boundaries.

# Severity  
P3  

# Routing  
OpenRouter route-specific evidence supports functional equivalence but does not establish provenance alignment; route usage assumes behavioral parity without verifying model lineage or version constraints.

# Residual Risk  
no code-correctness claim

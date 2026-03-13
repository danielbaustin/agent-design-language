

# V0.8 Planning: Godel Agent Pattern (ADL)

> Paper inspiration: **Gödel Agent: A Self-Referential Agent Framework for Recursive Self-Improvement** (arXiv:2410.04444; Yin et al.).
>
> ADL framing: implement the *benefits* (recursive self-improvement) while keeping ADL’s durability + auditability + replayability guarantees.

## Why this exists

The Gödel Agent paper argues that many agent frameworks can’t reach globally good designs because humans freeze too many components (fixed pipelines, fixed meta-optimizers). Their proposal is a self-referential agent that can iteratively modify its own logic guided by high-level objectives.

For ADL, the key take-away is **not** “live monkey patching,” but this:

- **Search the agent-design space** (prompts, routing, step graph, memory policy, tool policy).
- **Evaluate candidates** with an explicit harness.
- **Promote only improvements** (and preserve a full audit trail).

## Design principles (ADL-safe Gödel)

1. **No hidden state:** every “self-change” is an artifact (patch + evaluation + promotion record).
2. **Replayable:** given the same base profile + same eval suite, results are reproducible (or explicitly labeled stochastic).
3. **Constrained mutation space:** self-improvement operates over *allowed* mutation operators (bounded search space).
4. **Strict gating:** promotion requires passing invariants + meeting objective criteria.
5. **Fork/join friendly:** candidates can be explored in parallel.
6. **Human-in-the-loop optional:** the same mechanism supports manual approval (HITL) or automatic promotion.

## Terminology

- **Agent Profile**: The configuration/state that defines an agent’s behavior (prompts, policies, routing, knobs).
- **Mutation / Patch**: A structured change request applied to a base profile.
- **Candidate**: A mutated profile + an evaluation run.
- **Promotion**: Accepting a candidate as the new “best” profile.
- **Eval Suite**: A deterministic or controlled set of tasks/benchmarks used to score candidates.

## Scope

### In-scope for v0.7.x (first cut)

- A **pattern spec** for a policy/profile evolution loop.
- File formats (schemas) for:
  - `agent_profile.v1`
  - `mutation.v1`
  - `eval_report.v1`
  - `promotion.v1`
- A small set of **mutation operators**:
  - prompt edits
  - tool routing edits
  - planner knob edits
  - memory policy edits
- A minimal **evaluation harness interface**.
- A minimal **promotion policy** (gates + score).

### Explicitly out-of-scope (later)

- Runtime self-modifying Rust code / “monkey patching.”
- Unbounded mutations (“edit any code anywhere”).
- Complex open-ended evolutionary archives (keep in mind for later, e.g., Darwin Gödel Machine style).

## High-level architecture

### Conceptual loop

1. **Seed**: Start from a baseline `agent_profile.v1`.
2. **Propose**: Generate one or more `mutation.v1` patches.
3. **Apply**: Build `candidate_profile = apply(profile, mutation)` in a sandbox.
4. **Evaluate**: Run the eval suite -> `eval_report.v1`.
5. **Gate**: Enforce invariants (schema strictness, regression tests, cost budget, etc.).
6. **Promote/Reject**: Write `promotion.v1` with justification + links to artifacts.
7. **Repeat**: Use promoted profile as the new seed.

### Key ADL twist

All of the above is **artifacted**, so:

- Pause/resume works naturally (ties into v0.6 HITL pause/resume artifacts).
- “What changed?” is always answerable by reading the patch log.
- Rollback is “select previous promoted profile.”

## Artifacts

> Notes:
> - Keep these under `out/<run_id>/godel/…` or similar.
> - All files are written atomically.
> - JSON schemas use `deny_unknown_fields` and are versioned.

### 1) `agent_profile.v1.json`

Represents the behavior surface that self-improves.

Suggested top-level fields:

- `profile_id` (string)
- `version` (string, e.g. `"agent_profile.v1"`)
- `base` (optional: `{ parent_profile_id, parent_hash }`)
- `prompts` (map of prompt template ids -> text)
- `routing` (tool routing rules; provider profile selection rules)
- `planner` (knobs; decomposition style; limits)
- `memory_policy` (what to store/summarize; consumes ObsMem v1 surfaces from v0.75)
- `executor_policy` (timeouts; retries; strictness)
- `metadata` (description; created_at; tags)

### 2) `mutation.v1.json`

A structured patch that is safe to apply.

Suggested fields:

- `mutation_id`
- `version` (e.g. `"mutation.v1"`)
- `target_profile_id`
- `ops[]` (ordered list of operations)
- `rationale` (short human-readable explanation)
- `proposed_by` (agent id / model id)

Operation examples:

- `replace_prompt { prompt_id, new_text, expected_old_hash }`
- `set_routing_rule { rule_id, value }`
- `set_planner_knob { key, value }`
- `set_memory_policy { key, value }`

### 3) `eval_report.v1.json`

Evaluation results for a candidate.

Suggested fields:

- `eval_id`
- `version` (e.g. `"eval_report.v1"`)
- `candidate_profile_id`
- `mutation_id`
- `suite_id`
- `metrics`:
  - `score_primary` (float)
  - `scores_by_task` (map)
  - `cost` (`tokens_in`, `tokens_out`, `wall_time_ms`)
  - `reliability` (`failures`, `timeouts`)
- `diagnostics` (structured reasons; notable regressions)
- `artifacts` (links/paths to logs)

### 4) `promotion.v1.json`

Promotion decision record.

Suggested fields:

- `promotion_id`
- `version` (e.g. `"promotion.v1"`)
- `from_profile_id`
- `to_profile_id`
- `mutation_id`
- `eval_id`
- `decision` (`"promote" | "reject" | "needs_review"`)
- `policy_snapshot` (gates + thresholds used)
- `justification` (human-readable)

## Promotion policy (minimal)

Promotion is allowed iff:

1. **Gates pass**:
   - schemas validate
   - required artifacts exist
   - eval suite completed without disqualifying errors
   - cost not above budget (optional gate)
2. **Objective improves**:
   - `score_primary` strictly improves vs current best by `delta_min`
   - or Pareto improvement (e.g., score improves and cost not worse than threshold)

Also support:

- `needs_review` if within epsilon or if changes are high-risk (e.g., modifies security envelope rules).

## ADL Pattern Spec (draft)

This is a *conceptual* ADL pattern that we can register in the pattern registry.

```yaml
pattern: godel_policy_evolution
version: v0
summary: "Artifacted recursive self-improvement over an agent profile"
inputs:
  - agent_profile.v1
  - eval_suite.v1
params:
  max_generations: 20
  candidates_per_generation: 4
  promotion:
    delta_min: 0.01
    cost_budget_tokens: 200000
steps:
  - name: seed
    emit:
      - out/<run_id>/godel/profiles/best.agent_profile.v1.json

  - name: generation_loop
    repeat: max_generations
    fork:
      count: candidates_per_generation
      steps:
        - name: propose_mutation
          emit:
            - out/<run_id>/godel/mutations/<mutation_id>.mutation.v1.json

        - name: apply_mutation_sandbox
          use:
            - best.agent_profile.v1.json
            - mutation.v1.json
          emit:
            - out/<run_id>/godel/profiles/candidate/<candidate_id>.agent_profile.v1.json

        - name: evaluate_candidate
          use:
            - candidate.agent_profile.v1.json
            - eval_suite.v1
          emit:
            - out/<run_id>/godel/evals/<eval_id>.eval_report.v1.json

    join:
      - name: select_and_promote
        use:
          - all eval_report.v1
          - current best
        emit:
          - out/<run_id>/godel/promotions/<promotion_id>.promotion.v1.json
          - out/<run_id>/godel/profiles/best.agent_profile.v1.json
```

## Evaluation harness interface (v0.7 first cut)

Define a minimal interface that supports:

- suite discovery: `suite_id`
- deterministic tasks (or tasks with controlled randomness)
- machine-readable metrics

Suggested components:

- `eval_suite.v1.json`:
  - list of tasks
  - scoring function definitions
  - pass/fail gates
  - reproducibility configuration (seed; temperature; etc.)

## Safety / security notes

Even with “only profile mutations,” this is a powerful mechanism.

Hard rules:

- **Do not allow mutations to security envelope policies** (remote execution signing/trust) without HITL approval.
- **Do not allow mutations that change artifact validation strictness** (schemas, deny_unknown_fields) without HITL approval.
- **All candidate eval runs are sandboxed** (separate workspace, separate temp output dir).
- **Promotion writes an immutable log** (append-only audit trail).

## Integration points with existing v0.6 / v0.7 work

- v0.6 HITL pause/resume artifacts make the evolution loop safe to pause/resume.
- v0.8 EPIC-C (Godel-oriented adaptive architecture) is the primary home for this pattern.
- v0.8 EPIC-D (Authoring/composition surfaces) defines how these patterns are authored and governed.
- v0.75 EPIC-B (ObsMem v1 integration) supplies the memory interface context referenced by `memory_policy`.
- v0.85+ cluster and deeper resilience surfaces remain separate deferred infrastructure layers.

## Suggested issues to add (v0.7.x)

1. **[v0.7][P1] Spec: `agent_profile.v1` schema**
2. **[v0.7][P1] Spec: `mutation.v1` schema + allowed ops**
3. **[v0.7][P1] Spec: `eval_suite.v1` + `eval_report.v1`**
4. **[v0.7][P1] Runtime: sandbox apply + evaluate runner**
5. **[v0.7][P2] Docs: pattern registry entry for `godel_policy_evolution`**
6. **[v0.7][P2] Safety: promotion gates + HITL approvals for sensitive surfaces**

## Open questions

- Where do we want `agent_profile` to live long-term?
  - inside workflow YAML?
  - separate file referenced by workflow?
  - embedded into run-state artifacts?
- How do we standardize “score_primary” across suites?
- How do we manage stochasticity (temperature, provider variability)?
- Do we want a minimal “archive” (best-of generation) in v0.7, or keep only a linear chain?

## References

- arXiv:2410.04444 — Gödel Agent: A Self-Referential Agent Framework for Recursive Self-Improvement (Yin et al.).

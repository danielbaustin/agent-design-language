

# Layer 8 Provider (Human Provider) Notes

Status: incubation concept
Scope: post-v0.8 / early v0.85 candidate

## Summary

The **Layer 8 Provider** is the idea that a human participant should be modeled as a first-class ADL provider rather than treated as an out-of-band operator.

In practice, humans already participate in agent workflows by:
- generating hypotheses
- reframing problems
- reviewing outputs
- approving or rejecting proposed changes
- supplying tacit knowledge
- resolving ambiguity
- interrupting bad plans

ADL should be able to represent those actions explicitly as artifact-driven workflow steps.

This is called **Layer 8** in the classic sense: the human and organizational layer above the formal machine stack.

## Why this matters

A human provider makes the ADL architecture more honest.

Many real workflows are not purely model- or tool-driven. They are hybrid systems involving:
- humans
- models
- tools
- memory
- policy constraints

If human cognition remains external to the declared workflow, then important reasoning steps become invisible, non-auditable, and hard to replay.

By contrast, a human provider would allow ADL to model hybrid intelligence explicitly.

## Relationship to the Gödel–Hadamard–Bayes loop

The Layer 8 Provider fits naturally into the emerging **Gödel–Hadamard–Bayes cognition loop**.

Humans are especially strong at:
- **Hadamard-style hypothesis generation**
- high-level reframing
- spotting conceptual mistakes
- identifying when the experiment itself is wrong
- injecting judgment under ambiguity

This suggests that some stages of the scientific loop may be provided by a model, some by a tool, and some by a human.

For example:

```text
failure
  ↓
hypothesis (human or model)
  ↓
mutation
  ↓
experiment
  ↓
evaluation
  ↓
record + index
```

The human provider therefore becomes a legitimate cognitive component of the loop rather than a side channel.

## Core idea

A human provider is a provider whose outputs are **artifactized human judgments**.

That means a human step should still have:
- declared inputs
- declared role
- declared task
- declared output artifact
- audit/replay record of what was shown and what was returned

Example conceptual shape:

```yaml
provider:
  type: human
  id: daniel
  capabilities:
    - hypothesis_generation
    - review
    - approval
    - reframing
    - judgment
```

Example task shape:

```yaml
execution:
  provider: human/daniel
  action: generate_hypotheses
  input_artifacts:
    - canonical_evidence_view.json
    - experiment_record.json
  output_artifacts:
    - human_hypothesis_response.json
```

## Determinism model

A human provider is not deterministic in the same way as a pure compute provider.

However, it can still be **artifact-bounded** and **audit-friendly**.

Suggested framing:

```yaml
determinism:
  mode: human_judgment
  replayable: artifact_only
```

Meaning:
- the same human may not give the same answer twice
- but the workflow can preserve exactly:
  - what inputs were shown
  - what role was requested
  - what output artifact was returned

This keeps the step inspectable even when strict deterministic replay is impossible.

## Primary use cases

Strong first use cases for a human provider:
- hypothesis generation
- approval gates
- ambiguity resolution
- moral/policy override
- design review
- strategic reframing
- adjudication when evidence is weak or conflicting

Less suitable first use cases:
- routine transforms
- bulk deterministic computation
- high-throughput evaluation loops

## Provider contract ideas

A future Human Provider v1 could define:
- provider identity
- role/capability declaration
- structured prompt/request artifact
- structured response artifact
- timeout/escalation behavior
- audit metadata
- confidentiality / sensitivity markers

Example response artifact ideas:

```yaml
human_response:
  provider_id: daniel
  role: hypothesis_generation
  request_ref: artifacts/requests/hypothesis_request_001.json
  response_time_utc: 2026-03-09T08:15:00Z
  output:
    hypotheses:
      - id: hyp-001
        text: "The failure is caused by missing evidence normalization."
        confidence: medium
  notes: "Generated from evidence review only."
```

## Safety and governance

A human provider should not become an excuse for unstructured intervention.

Important rules:
- human steps should still be explicit workflow steps
- inputs and outputs should be artifactized
- policy-sensitive decisions should be marked as such
- hidden side-channel decisions should be avoided where practical
- workflows should declare whether a human step is advisory or authoritative

## Why this is important to ADL specifically

ADL is moving toward a model of intelligence that is distributed across:
- workflows
- artifacts
- memory
- providers
- evaluation loops

The Layer 8 Provider is important because it acknowledges that some of the most valuable cognition in the system may come from **human judgment integrated through contracts**, not only from model inference.

This is especially relevant for the Gödel architecture, where disciplined experimentation benefits from:
- human hypothesis formation
- human skepticism
- human redirection when the system is exploring the wrong space

## Candidate scope for a future issue

A future issue such as **Human Provider v1** or **Layer 8 Provider** should likely include:
- concept/spec only
- provider contract shape
- determinism/replay model for human steps
- initial use cases (review, hypothesis generation, approval)
- artifact schema sketches for human request/response

This should be treated as a bounded architecture/design task first, not as a large runtime implementation effort.

## Working thesis

The human provider is the real Layer 8.

If ADL can represent human cognition as an explicit, artifact-driven provider surface, then the system gains a principled way to orchestrate hybrid intelligence rather than pretending that all meaningful reasoning lives inside models or tools.
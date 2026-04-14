# Gemma 4 Issue Clerk Demo

This is a bounded `v0.89` provider-participation demo for testing whether a
Gemma-family local model can contribute usefully to one low-risk operational
task: drafting a structured issue-init proposal from a prepared packet.

The proof target is intentionally modest:

- prepare one explicit issue packet
- ask an Ollama-hosted Gemma-family model for one strict JSON proposal
- validate the proposal deterministically
- either materialize a reviewer-friendly issue body or reject the model output
  cleanly

## What It Proves

This demo proves:

- ADL can host a Gemma-family model in a bounded operational role
- the model is not trusted with file writes or lifecycle ownership
- ADL can validate model output before treating it as useful
- a clean rejection path is as important as a successful proposal

This demo does **not** claim:

- repo-wide review autonomy
- code-editing ownership
- PR publication ownership
- parity with stronger hosted models

## Fastest Smoke Path

Dry-run only:

```bash
bash adl/tools/demo_v089_gemma4_issue_clerk.sh --dry-run
```

That prepares:

- the issue packet
- the exact model prompt
- the artifact root and manifest

No model call is made in dry-run mode.

## Full Demo Run

```bash
bash adl/tools/demo_v089_gemma4_issue_clerk.sh
```

If your Ollama API is not on the default host, set `OLLAMA_HOST` or
`OLLAMA_HOST_URL` first.

Example:

```bash
OLLAMA_HOST=192.168.68.73 \
GEMMA4_OLLAMA_MODEL=gemma4:latest \
bash adl/tools/demo_v089_gemma4_issue_clerk.sh
```

If you want to replay a known response instead of calling a live model:

```bash
ADL_GEMMA4_RESPONSE_FILE=demos/fixtures/gemma4_issue_clerk_demo/valid_response.json \
bash adl/tools/demo_v089_gemma4_issue_clerk.sh
```

## Demo Mechanics

The script:

1. writes one bounded issue packet under `artifacts/v089/gemma4_issue_clerk/`
2. generates one strict JSON-only prompt for the model
3. optionally checks the configured Ollama host for the requested model
4. obtains one raw model response, either from Ollama or a fixture
5. validates the response against a strict contract
6. if valid, materializes `materialized_issue_body.md`
7. if invalid, records a rejection reason instead of pretending success
8. writes `demo_manifest.json` and `run_summary.md`

## Artifact Layout

Primary artifact root:

- `artifacts/v089/gemma4_issue_clerk/`

Key files:

- `issue_packet.json`
- `model_prompt.md`
- `raw_model_response.json`
- `validated_issue_proposal.json` when accepted
- `materialized_issue_body.md` when accepted
- `rejection_reason.txt` when rejected
- `demo_manifest.json`
- `run_summary.md`

## Truth Boundaries

- The current wrapper assumes an Ollama-compatible generate API.
- The default model id is `gemma4:latest`, but the wrapper remains honest if the
  configured host does not actually have that model.
- Rejection is a legitimate and useful outcome for this demo.
- ADL owns validation and issue-body materialization; the model only proposes
  bounded content.

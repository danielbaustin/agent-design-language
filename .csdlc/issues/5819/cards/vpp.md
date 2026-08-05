# Validation Planning Prompt

Template: 1.0.0

Issue: 5819

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5819/design.md

Diagram: .csdlc/prepared/issues/5819/diagram.mmd

## Selected Lanes

[
  {
    "lane": "migration-evidence-contract",
    "proof_role": "Validate exact repository order, before/after manifest digests, assignee dispositions, transfer times, and zero unexplained drift.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      "-rjson",
      "-e",
      "p='.csdlc/evidence/5819/migration-report.json'; r=JSON.parse(File.read(p)); exp=%w[cognitive-sdlc-paper godel-hadamard-bayes-paper general-intelligence-paper-private universal-tool-schema agent-design-language]; abort('wrong order') unless r['repositories'].map{|x|x['name']}==exp; abort('incomplete') unless r['repositories'].all?{|x|x['before_digest']&&x['after_digest']&&x['exact_head']&&x['assignees_verified']==true&&x['unexplained_drift']==[]}; abort('secret leakage flag') unless r['secret_values_retained']==false"
    ],
    "parallel_group": "migration-contract",
    "defer_reason": null
  },
  {
    "lane": "github-live-destination-platform",
    "proof_role": "Verify the five live agent-logic destinations and preserved GitHub repository surfaces after each transfer.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 5000,
    "argv": [
      "gh",
      "repo",
      "view",
      "agent-logic/agent-design-language",
      "--json",
      "nameWithOwner,visibility,defaultBranchRef"
    ],
    "parallel_group": "github-live",
    "defer_reason": null
  },
  {
    "lane": "migration-negative-controls",
    "proof_role": "Prove asksifu remains under danielbaustin and Horust was not transferred or modified.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "gh",
      "repo",
      "view",
      "danielbaustin/asksifu",
      "--json",
      "nameWithOwner,visibility"
    ],
    "parallel_group": "github-live",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors in tracked integration and evidence changes.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby -rjson -e p='.csdlc/evidence/5819/migration-report.json'; r=JSON.parse(File.read(p)); exp=%w[cognitive-sdlc-paper godel-hadamard-bayes-paper general-intelligence-paper-private universal-tool-schema agent-design-language]; abort('wrong order') unless r['repositories'].map{|x|x['name']}==exp; abort('incomplete') unless r['repositories'].all?{|x|x['before_digest']&&x['after_digest']&&x['exact_head']&&x['assignees_verified']==true&&x['unexplained_drift']==[]}; abort('secret leakage flag') unless r['secret_values_retained']==false`
- `gh repo view agent-logic/agent-design-language --json nameWithOwner,visibility,defaultBranchRef`
- `gh repo view danielbaustin/asksifu --json nameWithOwner,visibility`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.

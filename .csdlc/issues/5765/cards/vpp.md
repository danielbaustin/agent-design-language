# Validation Planning Prompt

Template: 1.0.0

Issue: 5765

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5765/design.md

Diagram: .csdlc/prepared/issues/5765/diagram.mmd

## Selected Lanes

[
  {
    "lane": "docs-planning-focused",
    "proof_role": "Parse the changed v0.92 YAML, assert issue 5765 source/prerequisite/non-claim text and exact one-file scope, then run diff check",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      "-e",
      "require 'yaml'; path='docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml'; doc=YAML.load_file(path); entry=doc.fetch('scheduled_planning').find{|item| item['issue']==5765}; abort 'missing issue 5765 schedule' unless entry; abort 'wrong status' unless entry['status']=='planning_only'; abort 'wrong source' unless entry['source']=='.adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md'; abort 'wrong source visibility' unless entry['source_visibility']=='operator_local_unpublished'; abort 'missing asksifu exclusion' unless entry['prerequisites'].any?{|value| value.include?('asksifu')}; abort 'missing six-candidate gate' unless entry['prerequisites'].any?{|value| value.include?('exactly six')}; paths=`git diff --name-only origin/main...HEAD`.lines.map(&:strip); abort 'missing YAML scope' unless paths.include?(path); abort 'unexpected tracked scope' unless paths.all?{|candidate| candidate==path || candidate.start_with?('.csdlc/issues/5765/') || candidate.start_with?('.csdlc/prepared/issues/5765/')}; abort 'diff check failed' unless system('git diff --check origin/main...HEAD'); puts 'planning YAML validation: PASS'"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby -e require 'yaml'; path='docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml'; doc=YAML.load_file(path); entry=doc.fetch('scheduled_planning').find{|item| item['issue']==5765}; abort 'missing issue 5765 schedule' unless entry; abort 'wrong status' unless entry['status']=='planning_only'; abort 'wrong source' unless entry['source']=='.adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md'; abort 'wrong source visibility' unless entry['source_visibility']=='operator_local_unpublished'; abort 'missing asksifu exclusion' unless entry['prerequisites'].any?{|value| value.include?('asksifu')}; abort 'missing six-candidate gate' unless entry['prerequisites'].any?{|value| value.include?('exactly six')}; paths=`git diff --name-only origin/main...HEAD`.lines.map(&:strip); abort 'missing YAML scope' unless paths.include?(path); abort 'unexpected tracked scope' unless paths.all?{|candidate| candidate==path || candidate.start_with?('.csdlc/issues/5765/') || candidate.start_with?('.csdlc/prepared/issues/5765/')}; abort 'diff check failed' unless system('git diff --check origin/main...HEAD'); puts 'planning YAML validation: PASS'`

## Failure Semantics

Fail closed if the reference is missing, the scope widens, or the edit implies migration authorization.

## Handoff

Retain typed evidence before convergence.

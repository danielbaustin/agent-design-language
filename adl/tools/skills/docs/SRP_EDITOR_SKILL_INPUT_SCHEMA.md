# SRP Editor Skill Input Schema

```yaml
skill_input_schema: srp_editor.v1
mode: normalize_srp | record_review_results | repair_review_truth_drift
repo_root: /absolute/path
target:
  srp_path: /absolute/or/repo-relative/path/to/srp.md
  issue_number: <u32 or null>
  source_prompt_path: <path or null>
  linked_stp_path: <path or null>
  linked_sip_path: <path or null>
  linked_spp_path: <path or null>
  linked_sor_path: <path or null>
evidence:
  review_performed: true | false
  reviewer: <bounded name or role or null>
  review_artifact_paths:
    - <path>
  findings:
    - id: <bounded finding id>
      severity: P0 | P1 | P2 | P3 | info
      summary: <bounded text>
      disposition: fixed | accepted | deferred | unresolved | not_applicable
policy:
  preserve_review_policy: true
  preserve_review_result_truth: true
  stop_after_edit: true
  allow_review_claims_without_evidence: false
```

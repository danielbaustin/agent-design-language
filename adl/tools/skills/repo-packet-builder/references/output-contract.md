# Output Contract

The repo packet builder produces a CodeBuddy review packet root.

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/
```

## Required Artifacts

### run_manifest.json

Required fields:

- `schema`
- `run_id`
- `repo_name`
- `repo_ref`
- `review_mode`
- `started_at`
- `completed_at`
- `skills_used`
- `artifact_root`
- `privacy_mode`
- `publication_allowed`

### repo_scope.md

Required sections:

- `# Repo Scope`
- `## Scope Reviewed`
- `## Included Paths`
- `## Excluded Paths`
- `## Non-Reviewed Surfaces`
- `## Assumptions`
- `## Known Limits`
- `## Next Specialist Lanes`

### repo_inventory.json

Required fields:

- `schema`
- `repo_name`
- `file_count`
- `extension_counts`
- `top_level_dirs`
- `manifests`
- `docs`
- `tests`
- `ci`
- `likely_code_roots`
- `largest_files`
- `largest_code_files`

### evidence_index.json

Required fields:

- `schema`
- `evidence`

Each evidence entry should include:

- `path`
- `category`
- `line_count`
- `reason`
- `specialist_lanes`

### specialist_assignments.json

Required fields:

- `schema`
- `assignments`

Default lanes:

- `code`
- `security`
- `tests`
- `docs`
- `architecture`
- `dependencies`
- `diagrams`
- `redaction`
- `synthesis`

## Rules

- Use repo-relative paths.
- Do not write absolute host paths into packet artifacts.
- Do not include source excerpts by default.
- Do not claim review findings.
- Do not claim publication safety before redaction review.
- Do not mutate the reviewed repository.

## Success Summary

When complete, report:

- artifact root
- review mode
- generated artifacts
- included/excluded scope summary
- specialist lanes prepared
- caveats and next skill


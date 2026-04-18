# ADR Curator Skill Input Schema

Schema id: `adr_curator.v1`

This schema describes structured input accepted by the `adr-curator` skill. It
is bounded so CodeBuddy can draft Architecture Decision Record candidates
without accepting decisions, editing ADR files, creating issues, opening PRs, or
mutating repositories.

## Required Shape

```yaml
skill_input_schema: adr_curator.v1
mode: curate_from_review_packet | curate_from_architecture_review | curate_from_findings_file | curate_from_migration_notes | curate_from_path | refresh_adr_packet
repo_root: /absolute/path
target:
  review_packet_path: <path or null>
  architecture_review_artifact: <path or null>
  findings_file: <path or null>
  migration_notes_path: <path or null>
  target_path: <path or null>
  existing_adr_dir: <path or null>
  existing_adr_packet: <path or null>
  artifact_root: <path or null>
policy:
  adr_status_policy: conservative | preserve_source_status
  approval_required: true
  mutation_allowed: false
  supersession_policy: preserve_explicit | infer_candidates
  write_candidate_packet: true | false
  stop_before_acceptance: true
```

## Required Fields

- `skill_input_schema` must equal `adr_curator.v1`.
- `mode` must be one of the supported curation modes.
- `repo_root` must be absolute.
- `target` must identify one bounded packet, review artifact, findings file,
  migration note, existing ADR packet, or target path.
- `policy.approval_required` must be `true`.
- `policy.mutation_allowed` must be `false`.
- `policy.stop_before_acceptance` must be `true`.
- `mode: curate_from_review_packet` requires `target.review_packet_path`.
- `mode: curate_from_architecture_review` requires
  `target.architecture_review_artifact`.
- `mode: curate_from_findings_file` requires `target.findings_file`.
- `mode: refresh_adr_packet` requires `target.existing_adr_packet`.

## Output Contract

The skill writes or returns an ADR candidate packet with:

- ADR candidate catalog
- proposed ADR drafts
- accepted or superseded existing decisions, if any
- deferred decision candidates
- supersession map
- validation notes
- approval boundary
- residual decision risk

The skill may also generate deterministic scaffolding with
`scripts/curate_adrs.py`.

## Stop Boundary

The skill must not:

- accept, reject, supersede, publish, or commit ADRs
- edit ADR files, docs, tests, CI, code, policy files, issues, or PRs
- mutate customer repositories
- run network or paid services
- claim a proposed decision is accepted without explicit source evidence
- replace architecture review, fitness-function planning, issue planning, or
  implementation workflow


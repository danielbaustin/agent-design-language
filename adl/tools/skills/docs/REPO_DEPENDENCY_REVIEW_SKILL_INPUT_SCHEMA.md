# Repo Dependency Review Skill Input Schema

Schema id: `repo_dependency_review.v1`

This schema describes the structured input accepted by the
`repo-dependency-review` skill. It is intentionally small so CodeBuddy can route
dependency and supply-chain review without giving the specialist permission to
perform upgrades or mutate customer repositories.

## Required Shape

```yaml
skill_input_schema: repo_dependency_review.v1
mode: review_repository | review_path | review_branch | review_diff | review_packet
repo_root: /absolute/path
target:
  target_path: <path or null>
  branch: <string or null>
  diff_base: <string or null>
  review_packet_path: <path or null>
  changed_paths:
    - <path>
  artifact_root: <path or null>
policy:
  review_depth: quick | standard | deep
  validation_mode: targeted | inspect_only | none
  write_review_artifact: true | false
  stop_after_review: true
  dependency_focus:
    - manifests
    - lockfiles
    - ci_dependency_setup
    - containers
    - license_cues
```

## Required Fields

- `skill_input_schema` must equal `repo_dependency_review.v1`.
- `mode` must be one of the supported review modes.
- `repo_root` must be absolute.
- `target` must identify one bounded repository, path, branch, diff, or packet
  target.
- `policy.review_depth` must be explicit.
- `policy.stop_after_review` must be `true`.
- `mode: review_packet` requires `target.review_packet_path`.

## Output Contract

The skill writes or returns a findings-first dependency review artifact with:

- findings
- dependency surface map
- reviewed surfaces
- candidate supply-chain findings
- candidate dependency test gaps
- candidate license review notes
- validation performed
- residual risk

The skill may also generate deterministic scaffolding with
`scripts/prepare_dependency_review.py`.

## Stop Boundary

The skill must not:

- edit the reviewed repository
- install, upgrade, downgrade, pin, unpin, vendor, or remove dependencies
- run external vulnerability feeds without an explicit offline source packet
- make legal determinations
- create issues or PRs
- perform synthesis or claim remediation completion


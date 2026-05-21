# Repo Scope

## Scope Reviewed

- Repository: agent-design-language
- Review mode: build_repository_packet
- Target path: not specified
- Diff base: not specified
- Privacy mode: local_only

## Included Paths

- tracked repository files
- top-level manifests
- docs, tests, CI, and likely code roots

## Excluded Paths

- .git/**
- .next/**
- .venv/**
- __pycache__/**
- build/**
- coverage/**
- dist/**
- node_modules/**
- target/**
- venv/**

## Non-Reviewed Surfaces

- Runtime behavior was not executed.
- Specialist review lanes were not run by this packet builder.
- Generated/vendor/cache surfaces are excluded by default.

## Assumptions

- Git-tracked files represent the intended review surface when available.
- Repo-relative paths are sufficient evidence references for downstream lanes.

## Known Limits

- The packet contains path evidence and metadata, not source excerpts.
- Publication safety requires a separate redaction/evidence audit.
- Path or diff scope must be expanded explicitly if downstream reviewers need more context.

## Next Specialist Lanes

- code
- security
- tests
- docs
- architecture
- dependencies
- diagrams
- redaction
- synthesis

## Inventory Summary

- File count: 2770
- Manifest count: 5
- Docs sampled: 40
- Tests sampled: 40
- CI files: 3

## Review Context

- Worktree name: adl-wp-3173

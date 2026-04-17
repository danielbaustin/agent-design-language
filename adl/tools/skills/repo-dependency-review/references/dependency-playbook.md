# Dependency Review Playbook

Use this playbook when the dependency review needs deeper heuristics than the
core `SKILL.md` workflow.

## High-Signal Surfaces

- Python: `pyproject.toml`, `setup.cfg`, `setup.py`, `requirements*.txt`,
  `uv.lock`, `poetry.lock`, `Pipfile.lock`, `.python-version`.
- JavaScript and TypeScript: `package.json`, `package-lock.json`,
  `pnpm-lock.yaml`, `yarn.lock`, `.npmrc`, `.yarnrc.yml`, `nvmrc`.
- Rust: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`.
- Go: `go.mod`, `go.sum`.
- JVM: `pom.xml`, `build.gradle`, `gradle.lockfile`, `settings.gradle`.
- Containers and runtime images: `Dockerfile`, `docker-compose.yml`,
  `.devcontainer/devcontainer.json`.
- CI and release: `.github/workflows/*.yml`, build scripts, install scripts,
  packaging scripts, release workflows.
- Licensing and attribution: `LICENSE*`, `NOTICE*`, `THIRD_PARTY*`,
  `COPYING*`, vendored directories, generated dependency reports.

## Finding Heuristics

Prefer concrete findings when evidence shows:

- a manifest changed without a corresponding lockfile update
- multiple package managers appear to own the same ecosystem without a documented
  policy
- CI uses floating toolchain or runtime versions that contradict repo metadata
- Docker images or setup actions float at broad tags in release-critical paths
- dependency install paths are documented but not tested
- dependency cache keys ignore lockfiles or toolchain versions
- generated or vendored dependency files are reviewed as first-party code
- license-sensitive dependencies or copied code are present without attribution
  follow-up

Avoid findings when the only issue is personal preference. Dependency reviews
should describe the failing scenario, not merely the desired style.

## Validation Ideas

Run validation only when bounded and safe:

- lockfile consistency checks available in the repo
- package manager dry-run or frozen install checks already used by CI
- shell syntax checks for install scripts
- local test targets that prove import, startup, or packaging behavior

Do not run network installs or ecosystem audit commands unless the operator has
explicitly allowed network access for the review.


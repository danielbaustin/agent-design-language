# Repo Packet Builder Playbook

Use this playbook after the skill triggers and before any specialist review
lane runs.

## Packet Priorities

Packet construction should answer:

- What is the exact review scope?
- What files and surfaces are included?
- What files and surfaces are excluded?
- Which repo evidence is high signal for each specialist lane?
- What is known, assumed, unknown, and not reviewed?
- What privacy mode controls evidence disclosure?

## Scope Rules

- Whole-repo mode may include all tracked files except ignored generated/vendor
  surfaces.
- Path mode must stay inside the requested target path.
- Diff mode must include changed paths plus enough nearby context to make review
  meaningful.
- Refresh mode must preserve the original packet's scope unless explicitly
  widened.

## Evidence Categories

Recommended categories:

- `manifest`
- `lockfile`
- `ci`
- `tooling`
- `code`
- `test`
- `docs`
- `architecture_docs`
- `security_docs`
- `demo`
- `generated_or_vendor`
- `large_file`

## Specialist Lane Hints

- `code`: executable source, entrypoints, stateful modules, large code files.
- `security`: auth, secrets, filesystem/network IO, CI, scripts, config.
- `tests`: test roots, fixtures, coverage config, validation commands.
- `docs`: README, docs, milestone docs, onboarding, API/CLI docs.
- `architecture`: architecture docs, module roots, manifests, runtime surfaces.
- `dependencies`: package manifests, lockfiles, Docker/toolchain/CI config.
- `diagrams`: architecture docs, module maps, workflow docs, generated packet
  summaries.
- `redaction`: all artifacts before publication.
- `synthesis`: all specialist outputs after review lanes complete.

## Quality Checks

Before handing off:

- Packet artifacts exist.
- Paths are repo-relative.
- Exclusions are explicit.
- Non-reviewed surfaces are explicit.
- Specialist assignments are bounded.
- No review findings are invented.
- No absolute host paths or secrets are written into public-shaped artifacts.


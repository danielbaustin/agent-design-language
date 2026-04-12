# v0-88-demo-let-the-codex-cli-ollama-demo-target-a-remote-ollama-host

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1691
Run ID: issue-1691
Version: v0.88
Title: [v0.88][demo] Let the Codex CLI + Ollama demo target a remote Ollama host
Branch: codex/1691-v0-88-demo-let-the-codex-cli-ollama-demo-target-a-remote-ollama-host
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5-codex
- Provider: OpenAI
- Start Time: 2026-04-12T16:45:53Z
- End Time: 2026-04-12T23:49:43Z

## Summary
Updated the existing Codex CLI + Ollama operational-skills demo so it truthfully supports a configured remote Ollama host at the demo-wrapper layer, including raw-host normalization (`OLLAMA_HOST=192.168.68.73`), clearer operator/docs language, and bounded host-configuration tests.

## Artifacts produced
- `adl/tools/demo_codex_ollama_operational_skills.sh`
- `adl/tools/test_demo_codex_ollama_operational_skills.sh`
- `adl/tools/test_demo_codex_ollama_semantic_fallback.sh`
- `demos/v0.87.1/codex_ollama_operational_skills_demo.md`
- `demos/README.md`

## Actions taken
- Added demo-layer Ollama host normalization so a raw `OLLAMA_HOST` such as `192.168.68.73` becomes `http://192.168.68.73:11434` without forcing the operator to provide a full URL.
- Tightened the demo wrapper usage and failure text so it now speaks about the configured Ollama host rather than implying same-machine locality.
- Added dry-run and semantic-fallback tests proving the manifest records normalized raw-host input and explicit remote-host URLs.
- Updated the demo docs to explain the local-default vs remote-configured host story and to state clearly that this does not yet claim first-class runtime/provider remote transport support.
- Verified live reachability of the remote Ollama host at `192.168.68.73:11434` and confirmed it advertises models including `gpt-oss:latest`.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths are updated on the issue branch via PR 1692
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: `pr finish` commit + push on the bound issue branch
- Verification performed:
  - `bash adl/tools/pr.sh finish 1691 --title "[v0.88][demo] Let the Codex CLI + Ollama demo target a remote Ollama host" --paths "adl/tools/demo_codex_ollama_operational_skills.sh,adl/tools/test_demo_codex_ollama_operational_skills.sh,adl/tools/test_demo_codex_ollama_semantic_fallback.sh,demos/v0.87.1/codex_ollama_operational_skills_demo.md,demos/README.md"` validated, committed, pushed, and opened PR `#1692`.
  - `git status --short --branch` verified the branch was clean after publication.
- Result: PASS

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout <branch> -- <path>` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- `Integration state` describes lifecycle state of the integrated artifact set, not where verification happened.
- `Verification scope` describes where the verification commands were run.
- `worktree_only` means at least one required path still exists only outside the main repository path.
- `pr_open` should pair with truthful `Worktree-only paths remaining` content; list those paths when they still exist only in the worktree or say `none` only when the branch contents are fully represented in the main repository path.
- If `Integration state` is `pr_open`, verify the actual proof artifacts rather than only the containing directory or card path.
- If `Integration method used` is `direct write in main repo`, `Verification scope` should normally be `main_repo` unless the deviation is explained.
- If `Verification scope` and `Integration method used` differ in a non-obvious way, explain the difference in one line.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `bash adl/tools/test_demo_codex_ollama_operational_skills.sh` verified dry-run manifest generation, prompt preparation, and normalized raw `OLLAMA_HOST` recording for the demo path.
  - `bash adl/tools/test_demo_codex_ollama_semantic_fallback.sh` verified the semantic-fallback path preserves an explicit remote `OLLAMA_HOST_URL` in the manifest while remaining fixture-backed and bounded.
  - `OLLAMA_HOST=192.168.68.73 bash adl/tools/demo_codex_ollama_operational_skills.sh --dry-run --artifact-root .tmp/adl-remote-ollama-demo-proof` verified the real operator path accepts a raw remote host and writes a manifest with the normalized host URL.
  - `python3 - <<'PY' ... urllib.request.urlopen('http://192.168.68.73:11434/api/tags') ... PY` verified the configured remote Ollama host is reachable and reported live models including `gpt-oss:latest`.
  - `git diff --check` verified there are no whitespace or malformed patch artifacts in the tracked changes.
- Results: all listed validation commands passed.

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

Rules:
- Replace the example values below with one actual final value per field.
- Do not leave pipe-delimited enum menus or placeholder text in a finished record.

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "bash adl/tools/test_demo_codex_ollama_operational_skills.sh"
      - "bash adl/tools/test_demo_codex_ollama_semantic_fallback.sh"
      - "OLLAMA_HOST=192.168.68.73 bash adl/tools/demo_codex_ollama_operational_skills.sh --dry-run --artifact-root .tmp/adl-remote-ollama-demo-proof"
      - "python3 urllib request to http://192.168.68.73:11434/api/tags"
      - "git diff --check"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: PASS
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: reran the dry-run and semantic-fallback demo tests after each host-handling/doc pass and reran the real dry-run path with the same raw `OLLAMA_HOST=192.168.68.73` input.
- Fixtures or scripts used: `adl/tools/test_demo_codex_ollama_operational_skills.sh`, `adl/tools/test_demo_codex_ollama_semantic_fallback.sh`, and `adl/tools/demo_codex_ollama_operational_skills.sh`.
- Replay verification (same inputs -> same artifacts/order): identical host inputs produce the same normalized `ollama_host_url` in the manifest and the same bounded prompt/fixture artifact layout for the dry-run path.
- Ordering guarantees (sorting / tie-break rules used): no ordering-sensitive runtime logic changed; the fix is limited to host normalization, manifest recording, and docs/tests.
- Artifact stability notes: the demo manifest now records the normalized host URL deterministically for identical `OLLAMA_HOST` / `OLLAMA_HOST_URL` inputs.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of the demo script, tests, and docs for embedded credentials or host secrets; none were introduced.
- Prompt / tool argument redaction verified: yes; the demo manifest records only the configured host URL and bounded demo inputs, not credentials or arbitrary tool payloads.
- Absolute path leakage check: `git diff --check` passed, and the changed docs/tests do not introduce new unjustified host-path examples beyond the existing repo-root conventions already used by the demo docs.
- Sandbox / policy invariants preserved: yes; the issue remains bounded to demo-wrapper, docs, and tests only, with no runtime/provider transport changes.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this issue used demo dry-run and fixture-backed shell tests rather than a runtime trace bundle.
- Run artifact root: not retained; regenerated at `.tmp/adl-remote-ollama-demo-proof` by the replay command when needed
- Replay command used for verification: `OLLAMA_HOST=192.168.68.73 bash adl/tools/demo_codex_ollama_operational_skills.sh --dry-run --artifact-root .tmp/adl-remote-ollama-demo-proof`
- Replay result: pass; the generated `demo_manifest.json` recorded `http://192.168.68.73:11434` and `native_tool_calling`.

## Artifact Verification
- Primary proof surface: the two updated demo tests plus a replayable dry-run manifest regenerated at `.tmp/adl-remote-ollama-demo-proof/demo_manifest.json`
- Required artifacts present: yes; the wrapper, tests, and docs are all present on the issue branch.
- Artifact schema/version checks: no schema changes were required; the existing demo manifest shape already had `ollama_host_url` and now records the normalized/explicit remote value truthfully.
- Hash/byte-stability checks: not run as a separate hash step; stable manifest replay was verified through repeated dry-run/test results.
- Missing/optional artifacts and rationale: no runtime/provider transport artifact is expected here because that work remains explicitly out of scope.

## Decisions / Deviations
- Kept remote-host support strictly at the demo-wrapper layer and did not touch `adl/src/provider.rs` or claim first-class runtime remote Ollama support.
- Normalized raw host input (`192.168.68.73`) into a full URL so the common operator shorthand works without requiring a more complex environment contract.

## Follow-ups / Deferred work
- First-class remote Ollama provider transport in the ADL runtime remains a separate future backlog item and is intentionally not addressed by this demo issue.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.

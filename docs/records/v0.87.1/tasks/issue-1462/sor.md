# [v0.87.1][WP-13] Demo matrix + integration demos

Task ID: issue-1462
Run ID: issue-1462
Version: v0.87.1
Title: [v0.87.1][WP-13] Demo matrix + integration demos
Branch: codex/1462-v0-87-1-wp-13-demo-matrix-integration-demos
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: OpenAI
- Start Time: 2026-04-10T00:39:00Z
- End Time: 2026-04-10T00:45:20Z

## Summary

Added a canonical `v0.87.1` WP-13 demo-suite entrypoint that runs the currently implemented provider, operator, runtime-state, runtime-review, and bounded multi-agent proof surfaces from one command. After the remaining demo issues landed, refreshed the demo matrix against the completed issue list so reviewers can find every completed `v0.87.1` demo/proof surface, including trace/archive support and the credential-gated live ChatGPT + Claude companion demo, without overclaiming what the CI-safe suite runs.

## Artifacts produced
- `adl/tools/demo_v0871_suite.sh`
- `adl/tools/test_demo_v0871_suite.sh`
- `adl/tools/demo_v0871_review_surface.sh`
- `adl/tools/demo_v0871_provider_http.sh`
- `adl/tools/demo_v0871_provider_chatgpt.sh`
- `adl/examples/v0-87-1-provider-chatgpt-demo.adl.yaml`
- `adl/tools/README.md`
- `demos/README.md`
- `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- `docs/milestones/v0.87.1/FEATURE_DOCS_v0.87.1.md`

## Actions taken
- added `bash adl/tools/demo_v0871_suite.sh` as the canonical WP-13 milestone demo-suite wrapper
- added `bash adl/tools/test_demo_v0871_suite.sh` to validate the suite manifest, README, index, and primary proof roots
- made the D8 review-surface wrapper accept suite-provided D6/D7 proof roots so the suite can be self-contained
- fixed the bounded HTTP and ChatGPT provider shim scripts to use reusable loopback sockets for repeated suite execution
- moved the ChatGPT demo shim to a distinct loopback port so it does not collide with the bounded HTTP demo
- updated the demo matrix with a READY D0 suite row, D0 detail section, cross-demo validation command, and truthful planned-not-run notes
- added a demo issue inventory to the matrix covering `#1467`, `#1485`-`#1488`, `#1490`, `#1491`, `#1500`-`#1502`, and `#1507`-`#1509`
- rebased the branch over current `origin/main`, preserving the merged live-provider and trace/archive demo work
- refreshed the demo issue inventory to mark `#1500`-`#1502` READY and add `#1468`, `#1518`-`#1521`, and `#1533`
- added D13L as a credential-gated live ChatGPT + Claude proof surface while keeping the default D0 suite CI-safe and bounded
- added provider-infrastructure references for `#1469`, `#1474`, and `#1477` so reviewer navigation aligns with the completed issue list
- updated demo discovery docs so reviewers can find the suite from `adl/tools/README.md`, `demos/README.md`, and the feature-doc index

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: pr_branch
- Integration method used: bounded branch update published via `pr finish` and opened as draft PR `#1517`
- Verification performed:
  - `bash -n adl/tools/demo_v0871_suite.sh adl/tools/test_demo_v0871_suite.sh adl/tools/demo_v0871_review_surface.sh adl/tools/demo_v0871_provider_http.sh adl/tools/demo_v0871_provider_chatgpt.sh`
  - `bash adl/tools/test_demo_v0871_provider_http.sh`
  - `bash adl/tools/test_demo_v0871_provider_chatgpt.sh`
  - `bash adl/tools/test_demo_v0871_suite.sh`
  - `ADL_OPENAI_KEY_FILE=.adl/nonexistent-openai.key ADL_ANTHROPIC_KEY_FILE=.adl/nonexistent-claude.key bash adl/tools/test_demo_v0871_real_multi_agent_discussion.sh`
  - `bash adl/tools/test_demo_v0871_review_surface.sh`
  - `for issue in 1467 1468 1485 1486 1487 1488 1490 1491 1500 1501 1502 1507 1508 1509 1518 1519 1520 1521 1533; do rg -q "#$issue" docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md || echo "missing #$issue"; done`
  - `rg -n 'IN_PROGRESS|<<<<<<<|=======|>>>>>>>' docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md || true`
  - `git diff --check`
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check`
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `bash -n adl/tools/demo_v0871_suite.sh adl/tools/test_demo_v0871_suite.sh adl/tools/demo_v0871_review_surface.sh adl/tools/demo_v0871_provider_http.sh adl/tools/demo_v0871_provider_chatgpt.sh` verified the touched shell entrypoints are syntactically valid
  - `bash adl/tools/test_demo_v0871_provider_http.sh` verified the bounded HTTP provider proof wrapper still runs and emits its primary runtime proof surfaces
  - `bash adl/tools/test_demo_v0871_provider_chatgpt.sh` verified the ChatGPT provider proof wrapper still runs on its distinct loopback shim port and emits its primary runtime proof surfaces
  - `bash adl/tools/test_demo_v0871_suite.sh` verified the WP-13 suite runs the implemented proof surfaces and emits `demo_manifest.json`, `README.md`, `index.txt`, provider proof roots, D8 proof roots, and the D13 transcript
  - `bash adl/tools/test_demo_v0871_review_surface.sh` verified the D8 review-surface wrapper still validates through `adl tooling review-runtime-surface`
  - `git diff --check` verified patch hygiene
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check` verified Rust formatting remained clean
- Results:
  - shell syntax check passed
  - bounded HTTP provider demo regression passed
  - ChatGPT provider demo regression passed
  - WP-13 suite regression passed
  - live-provider no-key skip path passed with a clear skip result
  - matrix completed-issue inventory check passed
  - matrix stale in-progress/conflict-marker scan passed
  - D8 review-surface regression passed
  - diff hygiene passed
  - Rust formatting check passed

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "bash -n adl/tools/demo_v0871_suite.sh adl/tools/test_demo_v0871_suite.sh adl/tools/demo_v0871_review_surface.sh adl/tools/demo_v0871_provider_http.sh adl/tools/demo_v0871_provider_chatgpt.sh"
      - "bash adl/tools/test_demo_v0871_provider_http.sh"
      - "bash adl/tools/test_demo_v0871_provider_chatgpt.sh"
      - "bash adl/tools/test_demo_v0871_suite.sh"
      - "ADL_OPENAI_KEY_FILE=.adl/nonexistent-openai.key ADL_ANTHROPIC_KEY_FILE=.adl/nonexistent-claude.key bash adl/tools/test_demo_v0871_real_multi_agent_discussion.sh"
      - "bash adl/tools/test_demo_v0871_review_surface.sh"
      - "completed issue inventory presence check for #1467, #1468, #1485-#1488, #1490, #1491, #1500-#1502, #1507-#1509, #1518-#1521, and #1533"
      - "matrix stale IN_PROGRESS/conflict-marker scan"
      - "git diff --check"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
  determinism:
    status: PASS_WITH_SCOPE
    replay_verified: true
    ordering_guarantees_verified: true
    notes: bounded suite determinism using controlled repo-local inputs, loopback shims, and mock providers; not a full cross-environment determinism guarantee
  security_privacy:
    status: PASS
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: true
      approved: true
```

## Determinism Evidence
- Determinism tests executed:
  - `bash adl/tools/test_demo_v0871_suite.sh`
  - `bash adl/tools/test_demo_v0871_review_surface.sh`
  - `bash adl/tools/test_demo_v0871_provider_http.sh`
  - `bash adl/tools/test_demo_v0871_provider_chatgpt.sh`
- Fixtures or scripts used:
  - local provider shims in the HTTP and ChatGPT demo wrappers
  - `adl/tools/mock_ollama_v0_4.sh`
  - `adl/tools/mock_multi_agent_discussion_provider.py`
  - existing D6, D7, D8, D13 demo wrappers
- Replay verification (same inputs -> same artifacts/order):
  - the focused suite regression reruns the suite from a temporary output root and verifies the same manifest identity, demo package membership, planned-not-run section, and required proof-surface inventory
- Ordering guarantees (sorting / tie-break rules used):
  - the suite manifest writes packages in a fixed order: provider family proofs, D8 review surface, then D13 multi-agent discussion
  - the suite index writes the same fixed proof-surface order
- Artifact stability notes:
  - the suite validates artifact presence and stable manifest fields, but does not claim byte-for-byte stability of all runtime logs

## Security / Privacy Checks
- Secret leakage scan performed:
  - reviewed the new suite wrapper, test, and docs; no secrets or real provider tokens were added
- Prompt / tool argument redaction verified:
  - the suite records proof-surface paths and demo metadata only; it does not publish raw prompts or sensitive tool arguments
- Absolute path leakage check:
  - passed; the suite manifest and index use repo-relative or relative paths, and docs record repository-relative commands
- Sandbox / policy invariants preserved:
  - yes; the work stayed within the issue worktree and used bounded local provider shims

## Replay Artifacts
- Trace bundle path(s): emitted by the underlying provider, D6/D7, and D13 runtime proof roots during the suite run
- Run artifact root: `artifacts/v0871/suite`
- Replay command used for verification: `bash adl/tools/test_demo_v0871_suite.sh`
- Replay result: bounded suite replay passed with controlled local inputs and expected proof-surface inventory

## Artifact Verification
- Primary proof surface: `artifacts/v0871/suite/demo_manifest.json`
- Required artifacts present: yes
- Artifact schema/version checks: suite manifest uses `adl.v0871.demo_suite.v1`; runtime review package uses `adl.runtime_review_surface.v1`
- Hash/byte-stability checks: not run
- Missing/optional artifacts and rationale: planned demo rows remain in `planned_not_run` until specialized wrappers land
- Demo inventory caveat: D13L is recorded as `READY_WITH_OPERATOR_CREDENTIALS` because it depends on operator-managed live-provider credentials and active provider account access; it is intentionally not claimed by the CI-safe D0 suite

## Decisions / Deviations
- Kept WP-13 focused on one canonical suite over implemented proof surfaces rather than pretending all planned demo rows are already runnable.
- Fixed provider wrapper loopback behavior because the suite exposed a real rapid-run collision between provider-family demos.
- Treated D13L as a live-provider companion rather than adding it to D0 because the default suite must remain runnable without external provider credentials.
- Recorded the local live-provider validation attempt truthfully: the command reached the OpenAI turn, then stopped on an external Anthropic billing/credit response, so the matrix uses credential-gated status rather than CI-safe READY.
- Did not commit generated `artifacts/` output; the suite remains a reproducible proof generator rather than a checked-in artifact dump.

## Follow-ups / Deferred work
- D1-D5 and D9-D12 remain planned rows until their specialized wrappers and validations land.
- Later review-tail issues should use `bash adl/tools/demo_v0871_suite.sh` as the default demo proof entrypoint.
- Reviewers who want D13L should run `bash adl/tools/test_demo_v0871_real_multi_agent_discussion.sh` only with valid OpenAI and Anthropic credentials plus active provider account access.

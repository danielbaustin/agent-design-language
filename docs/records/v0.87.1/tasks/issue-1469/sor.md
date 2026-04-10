# v0-87-1-tools-add-chatgpt-provider-profiles-for-gpt-5-family

Task ID: issue-1469
Run ID: issue-1469
Version: v0.87.1
Title: [v0.87.1][tools] Add ChatGPT provider profiles for GPT-5 family
Branch: codex/1469-v0-87-1-tools-add-chatgpt-provider-profiles-for-gpt-5-family
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5.4
- Provider: Codex desktop
- Start Time: 2026-04-08T19:09:35Z
- End Time: 2026-04-08T19:09:35Z

## Summary

Added a dedicated `chatgpt:` provider-profile family for the requested GPT-5 models, documented how it relates to the existing bounded `http:` profile family, and added focused regression coverage so the new profile names remain explicit and stable.

## Artifacts produced
- updated provider profile registry in `adl/src/provider.rs`
- new chatgpt-focused regression coverage in `adl/tests/provider_tests.rs`
- updated provider substrate doc note in `docs/milestones/v0.87/features/PROVIDER_SUBSTRATE_FEATURE.md`

## Actions taken
- corrected issue `#1469` from a mistaken meta/docs bundle into a real tools implementation issue before execution
- bound the issue to branch `codex/1469-v0-87-1-tools-add-chatgpt-provider-profiles-for-gpt-5-family`
- added four `chatgpt:` profile presets that expand into the existing bounded HTTP substrate
- added regression coverage for profile expansion and profile-name presence
- documented the new `chatgpt:` family as a distinct operator-facing profile family over the same HTTP transport substrate

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths from this issue are present on main via merged PR #1472.
- Worktree-only paths remaining: none for required tracked artifacts; issue branch changes have merged to main via PR #1472.
- Integration state: merged
- Verification scope: worktree
- Integration method used: issue branch/worktree changes were published and merged via PR #1472.
- Verification performed:
  - `git status --short`
    - verified the diff is bounded to the three intended tracked paths
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check`
    - verified formatting is clean
  - `cargo test --manifest-path adl/Cargo.toml expand_provider_profiles_accepts_chatgpt_profile_with_endpoint_override -- --nocapture`
    - verified a `chatgpt:` profile expands correctly with a real endpoint override
  - `cargo test --manifest-path adl/Cargo.toml provider_profile_names_include_chatgpt_family -- --nocapture`
    - verified the four requested profile names are present in the registry surface
  - `cargo test --manifest-path adl/Cargo.toml --test provider_tests -- --nocapture`
    - verified the full provider regression suite still passes with the new presets
- Result: PASS

## Validation
- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
  - verified formatting is clean
- `cargo test --manifest-path adl/Cargo.toml expand_provider_profiles_accepts_chatgpt_profile_with_endpoint_override -- --nocapture`
  - verified a `chatgpt:` profile expands correctly with a real endpoint override
- `cargo test --manifest-path adl/Cargo.toml provider_profile_names_include_chatgpt_family -- --nocapture`
  - verified the four requested profile names are present in the registry surface
- `cargo test --manifest-path adl/Cargo.toml --test provider_tests -- --nocapture`
  - verified the full provider regression suite still passes with the new presets
- Results:
  - formatting passed
  - targeted chatgpt profile tests passed
  - full provider test suite passed

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - cargo fmt --manifest-path adl/Cargo.toml --all --check
      - cargo test --manifest-path adl/Cargo.toml expand_provider_profiles_accepts_chatgpt_profile_with_endpoint_override -- --nocapture
      - cargo test --manifest-path adl/Cargo.toml provider_profile_names_include_chatgpt_family -- --nocapture
      - cargo test --manifest-path adl/Cargo.toml --test provider_tests -- --nocapture
  determinism:
    status: PASS
    replay_verified: false
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
- Determinism tests executed: `provider_profile_names_include_chatgpt_family` plus the full `provider_tests` suite
- Fixtures or scripts used: existing Rust provider test fixtures
- Replay verification (same inputs -> same artifacts/order): not run as a separate replay harness; provider registry behavior is covered through stable test assertions
- Ordering guarantees (sorting / tie-break rules used): profile registry remains a `BTreeMap`, so profile-name ordering stays stable
- Artifact stability notes: the change is additive and preserves the existing bounded profile-expansion model

## Security / Privacy Checks
- Secret leakage scan performed: manual review of changed files plus repository-relative command recording only
- Prompt / tool argument redaction verified: yes; no prompts or tool arguments were added to tracked surfaces
- Absolute path leakage check: repository-relative references only in tracked surfaces and this output card
- Sandbox / policy invariants preserved: yes

## Replay Artifacts
- Trace bundle path(s): not applicable
- Run artifact root: not applicable
- Replay command used for verification: not applicable
- Replay result: not applicable

## Artifact Verification
- Primary proof surface: `adl/tests/provider_tests.rs`
- Required artifacts present: yes
- Artifact schema/version checks: not applicable
- Hash/byte-stability checks: not applicable
- Missing/optional artifacts and rationale: no demo or trace artifact was required because this is a bounded provider-profile registry change

## Decisions / Deviations
- Kept the new ChatGPT profiles on the existing `http` transport substrate rather than inventing a new transport kind
- Scoped docs to the canonical provider substrate feature doc instead of broad ADR/history rewrites
- Continued execution after `pr run` half-failed because branch/worktree binding had already completed; repaired the worktree-local cards manually to preserve process continuity

## Follow-ups / Deferred work
- The separate provider-demo planning issue remains the right place to add demo coverage for each provider family later

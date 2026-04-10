# v0-87-1-runtime-add-first-class-claude-provider-family-parity-with-chatgpt-profiles

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1500
Run ID: issue-1500
Version: v0.87.1
Title: [v0.87.1][runtime] Add first-class Claude provider-family parity with ChatGPT profiles
Branch: codex/1500-v0-87-1-runtime-add-first-class-claude-provider-family-parity-with-chatgpt-profiles
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: openai
- Start Time: 2026-04-10T00:46:47Z
- End Time: 2026-04-10T00:51:28Z

## Summary

Added a first-class `claude:` provider/profile family alongside the existing `chatgpt:` family. The implementation keeps Claude on the bounded HTTP completion substrate, adds `adl provider setup claude`, documents the operator-facing setup path, and preserves the existing generic `anthropic` compatibility family.

## Artifacts produced
- updated provider profile registry with `claude:claude-3-7-sonnet` and `claude:claude-3-5-haiku`
- updated provider substrate vendor inference for first-class `claude:` profiles
- updated provider setup command support for `adl provider setup claude`
- updated provider setup documentation and CLI usage examples
- focused provider/profile tests for Claude setup, profile expansion, profile listing, and substrate vendor inference

## Actions taken
- added `claude:` profile presets to `adl/src/provider.rs`
- mapped `claude:` profiles to the Anthropic vendor in `adl/src/provider_substrate.rs`
- added `provider setup claude` support in `adl/src/cli/provider_cmd.rs`
- updated CLI usage examples in `adl/src/cli/usage.rs`
- updated `docs/tooling/PROVIDER_SETUP.md` to list and explain the Claude family
- added focused Rust coverage in `adl/tests/provider_tests.rs` and existing provider command/substrate test modules

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths from this issue are present on main via merged PR #1522.
- Worktree-only paths remaining: none for required tracked artifacts; issue branch changes have merged to main via PR #1522.
- Integration state: merged
- Verification scope: worktree
- Integration method used: issue branch/worktree changes were published and merged via PR #1522.
- Verification performed:
  - `cargo test --manifest-path adl/Cargo.toml provider_profile_registry_includes_first_class_claude_profiles -- --nocapture`
    - verified the internal provider profile registry exposes the first-class Claude profiles
  - `cargo test --manifest-path adl/Cargo.toml provider_setup_writes_expected_bundle_for_claude -- --nocapture`
    - verified `adl provider setup claude` writes the expected local setup bundle
  - `cargo test --manifest-path adl/Cargo.toml provider_substrate_infers_first_class_claude_profile_vendor -- --nocapture`
    - verified provider substrate maps `claude:` profiles to the Anthropic vendor on the bounded HTTP transport
  - `cargo test --manifest-path adl/Cargo.toml provider_setup_supports_all_declared_families -- --nocapture`
    - verified the declared setup-family table includes the new Claude family
  - `cargo test --manifest-path adl/Cargo.toml provider_ -- --nocapture`
    - verified broader provider/profile/provider-substrate behavior still passes
  - `git diff --check`
    - verified the patch has no whitespace errors
- Result: PASS

## Validation
- `cargo test --manifest-path adl/Cargo.toml provider_profile_registry_includes_first_class_claude_profiles -- --nocapture`
  - passed
- `cargo test --manifest-path adl/Cargo.toml provider_setup_writes_expected_bundle_for_claude -- --nocapture`
  - passed
- `cargo test --manifest-path adl/Cargo.toml provider_substrate_infers_first_class_claude_profile_vendor -- --nocapture`
  - passed
- `cargo test --manifest-path adl/Cargo.toml provider_setup_supports_all_declared_families -- --nocapture`
  - passed
- `cargo test --manifest-path adl/Cargo.toml expand_provider_profiles_accepts_claude_profile_with_endpoint_override -- --nocapture`
  - passed
- `cargo test --manifest-path adl/Cargo.toml provider_profile_names_include_claude_family -- --nocapture`
  - passed
- `cargo test --manifest-path adl/Cargo.toml provider_ -- --nocapture`
  - passed
- `git diff --check`
  - passed
- Results:
  - first-class Claude profile, setup, and provider-substrate paths are covered
  - existing provider test surface still passes

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - cargo test --manifest-path adl/Cargo.toml provider_profile_registry_includes_first_class_claude_profiles -- --nocapture
      - cargo test --manifest-path adl/Cargo.toml provider_setup_writes_expected_bundle_for_claude -- --nocapture
      - cargo test --manifest-path adl/Cargo.toml provider_substrate_infers_first_class_claude_profile_vendor -- --nocapture
      - cargo test --manifest-path adl/Cargo.toml provider_setup_supports_all_declared_families -- --nocapture
      - cargo test --manifest-path adl/Cargo.toml expand_provider_profiles_accepts_claude_profile_with_endpoint_override -- --nocapture
      - cargo test --manifest-path adl/Cargo.toml provider_profile_names_include_claude_family -- --nocapture
      - cargo test --manifest-path adl/Cargo.toml provider_ -- --nocapture
      - git diff --check
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
- Determinism tests executed: provider profile registry, provider setup, profile expansion, and provider substrate focused tests
- Fixtures or scripts used: Rust unit/integration tests only
- Replay verification (same inputs -> same artifacts/order): the setup-family and profile-registry tests verify stable family/profile expansion for fixed inputs
- Ordering guarantees (sorting / tie-break rules used): provider profile names are derived from the existing sorted registry surface; broader `provider_` tests passed
- Artifact stability notes: generated provider setup content is deterministic for the `claude` family template

## Security / Privacy Checks
- Secret leakage scan performed: manual review plus generated setup tests; no real credentials were introduced
- Prompt / tool argument redaction verified: yes; provider setup writes placeholder env values only
- Absolute path leakage check: passed; final recorded artifact paths are repository-relative
- Sandbox / policy invariants preserved: yes

## Replay Artifacts
- Trace bundle path(s): not applicable; this issue did not run an ADL trace demo
- Run artifact root: not applicable; validation used unit/integration tests
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml provider_ -- --nocapture`
- Replay result: passed

## Artifact Verification
- Primary proof surface: provider/profile code and focused tests listed in this card
- Required artifacts present: yes
- Artifact schema/version checks: no artifact schema changes were made
- Hash/byte-stability checks: not applicable; no generated persistent artifact was checked in
- Missing/optional artifacts and rationale: no demo artifact is required because this is provider-family parity work

## Decisions / Deviations
- Kept the existing `anthropic` setup family intact as the generic compatibility path.
- Added `claude:` as the first-class model-family surface while retaining the bounded HTTP completion contract.
- Did not add a new Claude demo because the issue scope is provider-family parity, not a demo-bearing proof.

## Follow-ups / Deferred work
- Future work may update multi-agent demos to use `claude:` directly where that improves clarity.

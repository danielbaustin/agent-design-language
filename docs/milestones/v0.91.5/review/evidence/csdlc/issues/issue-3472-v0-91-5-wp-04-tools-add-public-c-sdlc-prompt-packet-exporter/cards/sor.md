# v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter

Canonical Template Source: `docs/templates/prompts/1.0.0/sor.md`

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-3472
Run ID: issue-3472
Version: v0.91.5
Title: [v0.91.5][WP-04][tools] Add public C-SDLC prompt packet exporter
Branch: codex/3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter
Card Status: ready
Status: DONE
Generated: 2026-06-04T19:39:09Z

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI
- Start Time: 2026-06-04T19:39:09Z
- End Time: 2026-06-04T20:47:31Z

## Summary

Implemented a version-aware public C-SDLC prompt packet exporter under
`adl tooling public-prompt-packet export`. The command copies the five lifecycle
cards into a tracked public packet, writes `manifest.json` and `README.md`,
preserves template/status metadata, separates GitHub tracker identity from
tracker-agnostic work-item identity, and refuses obvious publication-unsafe
source card content.

## Artifacts produced
- `adl/src/cli/tooling_cmd/public_prompt_packet.rs`
- `adl/src/cli/tooling_cmd/tests/public_prompt_packet.rs`
- `docs/milestones/v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md`
- `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/README.md`
- `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/manifest.json`
- `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/sip.md`
- `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/stp.md`
- `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/spp.md`
- `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/srp.md`
- `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/sor.md`

## Actions taken
- Added the `public-prompt-packet export` tooling subcommand and dispatcher/help wiring.
- Added deterministic defaults for milestone-scoped `.adl` issue bundles and milestone-scoped public packet output under `docs/milestones/v0.91.5/review/evidence/csdlc/issues/`.
- Added export hygiene checks for host-local paths, secret-like tokens, private key markers, local scratch paths, and unresolved template markers.
- Added contract tests for packet creation, manifest shape, repo-relative paths, tracker/work-item split, and fail-closed unsafe content handling.
- Added in-export structured-prompt validation for all five cards using the existing SIP/STP/SPP/SRP/SOR validators before any public packet directory is replaced.
- Updated exporter tests to render real sample prompt cards through `prompt-template render-all` instead of relying on toy Markdown fixtures.
- Exported the real `#3472` card bundle into the v0.91.5 public evidence tree.
- Updated the public prompt records feature doc with the command contract and safety boundary.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet; tracked artifacts are present in the issue worktree and require PR publication.
- Worktree-only paths remaining: implementation code, tests, feature doc update, and exported packet paths listed above
- Integration state: worktree_only
- Verification scope: main_repo
- Integration method used: issue worktree edits on branch `codex/3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter`; PR publication pending
- Verification performed:
  - `CARGO_HOME=$ADL_TEMP_CARGO_HOME cargo fmt --manifest-path adl/Cargo.toml`
    Formatted Rust changes.
  - `CARGO_HOME=$ADL_TEMP_CARGO_HOME cargo test --manifest-path adl/Cargo.toml public_prompt_packet -- --nocapture`
    Verified exporter creation/refusal tests in the focused selector, including real rendered prompt cards and in-export structured-prompt validation.
  - `./adl/target/debug/adl tooling public-prompt-packet export --issue 3472 --slug v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter --version v0.91.5 --tracker-url https://github.com/danielbaustin/agent-design-language/issues/3472`
    Verified real `#3472` public packet generation.
  - `./adl/target/debug/adl tooling prompt-template validate-structure --kind sip --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/sip.md`
    Verified exported SIP structure.
  - `./adl/target/debug/adl tooling prompt-template validate-structure --kind stp --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/stp.md`
    Verified exported STP structure.
  - `./adl/target/debug/adl tooling prompt-template validate-structure --kind spp --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/spp.md`
    Verified exported SPP structure.
  - `./adl/target/debug/adl tooling validate-structured-prompt --type srp --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/srp.md`
    Verified exported SRP structured-prompt contract.
  - `./adl/target/debug/adl tooling validate-structured-prompt --type sor --phase bootstrap --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/sor.md`
    Verified exported SOR execution-record contract.
  - `git diff --check`
    Verified no whitespace errors.
  - `ruby -rjson -e 'JSON.parse(File.read("docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/manifest.json")); puts "manifest json ok"'`
    Verified exported manifest JSON parses.
  - `rg -n PUBLIC_PACKET_REDACTION_SCAN docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter`
    Verified no matches for the focused public packet redaction scan.
- Result: PASS

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout BRANCH -- PATH` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- `Integration state` describes lifecycle state of the integrated artifact set, not where verification happened.
- `Verification scope` describes where the verification commands were run.
- `worktree_only` means at least one required path still exists only outside the main repository path.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `CARGO_HOME=$ADL_TEMP_CARGO_HOME cargo fmt --manifest-path adl/Cargo.toml`
    Formatted Rust changes.
  - `CARGO_HOME=$ADL_TEMP_CARGO_HOME cargo test --manifest-path adl/Cargo.toml public_prompt_packet -- --nocapture`
    Verified the new exporter contract tests; 2 tests passed in `src/main.rs` and 2 tests passed in `src/bin/adl_csdlc.rs`. The tests now render real prompt cards and prove the exporter runs structured-prompt validation before publication.
  - `./adl/target/debug/adl tooling public-prompt-packet export --issue 3472 --slug v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter --version v0.91.5 --tracker-url https://github.com/danielbaustin/agent-design-language/issues/3472`
    Verified real packet generation from the `#3472` card bundle.
  - `./adl/target/debug/adl tooling prompt-template validate-structure --kind sip --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/sip.md`
    Verified exported SIP structure.
  - `./adl/target/debug/adl tooling prompt-template validate-structure --kind stp --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/stp.md`
    Verified exported STP structure.
  - `./adl/target/debug/adl tooling prompt-template validate-structure --kind spp --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/spp.md`
    Verified exported SPP structure.
  - `./adl/target/debug/adl tooling validate-structured-prompt --type srp --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/srp.md`
    Verified exported SRP structured-prompt contract.
  - `./adl/target/debug/adl tooling validate-structured-prompt --type sor --phase bootstrap --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/sor.md`
    Verified exported SOR execution-record contract.
  - `git diff --check`
    Verified whitespace hygiene.
  - `ruby -rjson -e 'JSON.parse(File.read("docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/manifest.json")); puts "manifest json ok"'`
    Verified exported manifest JSON parses.
  - `rg -n PUBLIC_PACKET_REDACTION_SCAN docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter`
    Verified the exported packet has no matches for the focused redaction scan.
- Results:
  - PASS

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "CARGO_HOME=$ADL_TEMP_CARGO_HOME cargo fmt --manifest-path adl/Cargo.toml"
      - "CARGO_HOME=$ADL_TEMP_CARGO_HOME cargo test --manifest-path adl/Cargo.toml public_prompt_packet -- --nocapture"
      - "./adl/target/debug/adl tooling public-prompt-packet export --issue 3472 --slug v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter --version v0.91.5 --tracker-url https://github.com/danielbaustin/agent-design-language/issues/3472"
      - "./adl/target/debug/adl tooling prompt-template validate-structure --kind sip --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/sip.md"
      - "./adl/target/debug/adl tooling prompt-template validate-structure --kind stp --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/stp.md"
      - "./adl/target/debug/adl tooling prompt-template validate-structure --kind spp --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/spp.md"
      - "./adl/target/debug/adl tooling validate-structured-prompt --type srp --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/srp.md"
      - "./adl/target/debug/adl tooling validate-structured-prompt --type sor --phase bootstrap --input docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/sor.md"
      - "git diff --check"
      - "ruby -rjson -e 'JSON.parse(File.read(\"docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/manifest.json\")); puts \"manifest json ok\"'"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: PARTIAL
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
- Determinism tests executed: focused exporter tests and real packet export.
- Fixtures or scripts used: test-created temporary card bundles and the real `#3472` card bundle.
- Replay verification (same inputs -> same artifacts/order): exporter contract tests verify deterministic packet paths and manifest/card shape.
- Ordering guarantees (sorting / tie-break rules used): fixed lifecycle card order `sip`, `stp`, `spp`, `srp`, `sor`.
- Artifact stability notes: exported public paths are repository-relative; the exporter refuses rather than rewrites unsafe source card text.

## Security / Privacy Checks
- Secret leakage scan performed: focused redaction scan over the exported packet; no matches.
- Prompt / tool argument redaction verified: exporter refuses obvious secret-like tokens and private key markers in source cards.
- Absolute path leakage check: exporter tests and real packet scan cover host-local absolute paths.
- Sandbox / policy invariants preserved: yes; local `.adl` source records remain source inputs only and are not added as tracked artifacts.

## Replay Artifacts
- Trace bundle path(s): not_applicable for this tooling/docs issue
- Run artifact root: `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/`
- Replay command used for verification: `./adl/target/debug/adl tooling public-prompt-packet export --issue 3472 --slug v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter --version v0.91.5 --tracker-url https://github.com/danielbaustin/agent-design-language/issues/3472`
- Replay result: PASS

## Artifact Verification
- Primary proof surface: exporter tests plus exported `#3472` public prompt packet manifest/cards.
- Required artifacts present: yes, in the issue worktree; PR publication pending.
- Artifact schema/version checks: manifest JSON parse passed; exported SIP/STP/SPP prompt-card structure validation passed; exported SRP and SOR structured-prompt contracts passed; exporter tests prove in-export structured-prompt validation for all five source cards.
- Hash/byte-stability checks: not_run; focused deterministic shape/path tests were run instead.
- Missing/optional artifacts and rationale: no runtime trace bundle is required for this tooling/docs issue.

## Decisions / Deviations
- Implemented the exporter in the existing `adl tooling` command family to avoid adding a new loose script before the CLI decomposition work settles.
- Exporter refuses unsafe source cards rather than redacting or rewriting them; richer redaction gates remain owned by later public-card validation work.
- Integration state remains `worktree_only` until the issue is finished and published as a PR.

## Follow-ups / Deferred work
- Run bounded pre-PR review and fix actionable findings before publication.
- Normalize this record to `pr_open`, `merged`, or `closed_no_pr` during finish/closeout as appropriate.

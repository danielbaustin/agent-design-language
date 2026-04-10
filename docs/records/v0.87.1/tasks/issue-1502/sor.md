# v0-87-1-runtime-formalize-transcript-artifact-contract-for-multi-agent-discussion-demos

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1502
Run ID: issue-1502
Version: v0.87.1
Title: [v0.87.1][runtime] Formalize transcript artifact contract for multi-agent discussion demos
Branch: codex/1502-v0-87-1-runtime-formalize-transcript-artifact-contract-for-multi-agent-discussion-demos
Status: DONE

Execution:
- Actor: codex
- Model: GPT-5
- Provider: OpenAI
- Start Time: 2026-04-10T01:08:23Z
- End Time: 2026-04-10T01:13:15Z

## Summary

Formalized the bounded multi-agent transcript artifact contract for the `v0.87.1` Claude + ChatGPT discussion demo. The demo now emits a machine-readable `transcript_contract.json` alongside `transcript.md`, and the demo test validates both surfaces.

## Artifacts produced
- `docs/tooling/MULTI_AGENT_TRANSCRIPT_ARTIFACT_CONTRACT.md`
- `adl/tools/validate_multi_agent_transcript.py`
- updated demo wrapper to emit `transcript_contract.json`
- updated demo test to validate the transcript and contract together
- updated demo/tooling documentation and demo matrix references

## Actions taken
- documented the canonical transcript name, layout, required turn headings, companion artifacts, validation rules, and reviewer checklist
- added a bounded validator for transcript text and optional machine-readable contract JSON
- added `transcript_contract.json` generation to the D13 demo wrapper
- updated the demo test to require the contract artifact and run the validator
- updated docs so reviewers can discover and run the contract validation path

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths from this issue are present on main via merged PR #1526.
- Worktree-only paths remaining: none for required tracked artifacts; issue branch changes have merged to main via PR #1526.
- Integration state: merged
- Verification scope: worktree
- Integration method used: issue branch/worktree changes were published and merged via PR #1526.
- Verification performed:
  - `python3 adl/tools/validate_multi_agent_transcript.py --help`
    - verified the validator entrypoint and documented arguments are available
  - `bash adl/tools/test_demo_v0871_multi_agent_discussion.sh`
    - verified the demo emits and validates the transcript contract surface
- Result: PASS

## Validation
- `python3 adl/tools/validate_multi_agent_transcript.py --help`
  - verified the validator CLI loads successfully
- `bash adl/tools/test_demo_v0871_multi_agent_discussion.sh`
  - verified the bounded multi-agent demo, transcript artifact, transcript contract artifact, manifest, run summary, and trace proof surfaces
- Results:
  - transcript validator help path passes
  - D13 demo test passes with transcript contract validation enabled

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - python3 adl/tools/validate_multi_agent_transcript.py --help
      - bash adl/tools/test_demo_v0871_multi_agent_discussion.sh
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
      present: true
      approved: true
```

## Determinism Evidence
- Determinism tests executed: `bash adl/tools/test_demo_v0871_multi_agent_discussion.sh`
- Fixtures or scripts used: bounded D13 demo wrapper and local deterministic multi-agent provider shim
- Replay verification (same inputs -> same artifacts/order): the demo script reruns from a clean temporary output directory and validates the same five ordered turn headings and contract JSON shape
- Ordering guarantees (sorting / tie-break rules used): contract validation requires turn ordinals, headings, and source outputs to appear in the declared order
- Artifact stability notes: `transcript_contract.json` uses fixed relative artifact paths and a fixed schema version

## Security / Privacy Checks
- Secret leakage scan performed: manual review of changed files; no secrets or credentials were introduced
- Prompt / tool argument redaction verified: validator and contract use relative artifact paths and do not persist provider prompts
- Absolute path leakage check: passed for final tracked artifacts; generated manifest may contain caller-selected output paths as existing demo behavior, while the new contract uses relative paths
- Sandbox / policy invariants preserved: yes; the validator reads transcript/contract files only and does not call providers or write files

## Replay Artifacts
- Trace bundle path(s): generated by the demo test under a temporary directory and not checked in
- Run artifact root: temporary output from `adl/tools/test_demo_v0871_multi_agent_discussion.sh`
- Replay command used for verification: `bash adl/tools/test_demo_v0871_multi_agent_discussion.sh`
- Replay result: PASS

## Artifact Verification
- Primary proof surface: generated `transcript.md` plus `transcript_contract.json`
- Required artifacts present: yes; the focused demo test checks transcript, contract, manifest, run summary, trace, and first/last turn outputs
- Artifact schema/version checks: validator requires `multi_agent_discussion_transcript.v1`
- Hash/byte-stability checks: not separately hashed; the contract is deterministic JSON emitted from fixed demo metadata
- Missing/optional artifacts and rationale: no generated demo artifacts are committed because they are runtime outputs

## Decisions / Deviations

- Used `--allow-open-pr-wave` because issue `1502` was intentionally executed while milestone PRs remained open.
- Kept the contract bounded to D13 transcript artifacts and did not modify runtime trace schema.
- Avoided depending on issue `1501` runtime turn metadata so this PR remains independently reviewable.

## Follow-ups / Deferred work

- Future multi-agent demos can adopt the same contract or define a new contract version when their speaker order or transcript shape differs.

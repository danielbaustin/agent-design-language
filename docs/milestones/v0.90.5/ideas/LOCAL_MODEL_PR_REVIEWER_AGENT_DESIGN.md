# Local Model PR Reviewer Agent Design

## Status

Design draft for issue `#2603`.

This document defines the intended pre-PR review gate for the local model PR
reviewer demo. The implementation target is an agent-callable ADL code-review
tool, not only a static demo packet.

## Purpose

ADL should be able to call an independent reviewer tool after an execution agent has
finished code changes and local validation, but before a PR is opened. If the
reviewer reports actionable blocking findings, the execution session should fix
them and run the review gate again before publication.

The reviewer may be a local model such as Gemma4, another Codex session, ChatGPT,
Claude, or a deterministic fixture backend. The reviewer must not be the same
session that wrote the code.

The first operational surface is:

```text
adl tooling code-review --out <dir> [--backend fixture|ollama] [--visibility packet-only|read-only-repo]
```

The fixture backend is deterministic and always available. The Ollama backend
calls the Ollama HTTP API directly via `/api/generate`; it does not require a new
Ollama-specific CLI wrapper.

## Desired Lifecycle Placement

The target hook is the end of the `pr run` / beginning of `pr finish` boundary:

1. Execution agent completes the bounded issue work in the issue worktree.
2. Execution agent runs the issue validation set and records results.
3. ADL builds a PR-review packet from the worktree diff, cards, source prompt,
   validation evidence, and explicit scope.
4. ADL invokes the external reviewer backend through a review skill contract.
5. ADL normalizes and validates reviewer output.
6. If there are blocking findings, the PR is not opened.
7. Execution agent fixes accepted findings and repeats validation plus review.
8. Only after the review gate is clean, skipped by explicit operator policy, or
   classified non-proving with operator acceptance may `pr finish` open the PR.

This makes the reviewer a pre-publication quality gate, not a merge authority.

## Reviewer Visibility Tiers

The reviewer should support two visibility tiers.

### Tier 1: Packet-Only Review

Packet-only review is the deterministic baseline. The reviewer receives only the
bounded packet fields described below: issue context, cards, changed files, diff
hunks, selected excerpts, validation evidence, static analyzer evidence, scope,
and known risks.

This tier is best for:

- fixture-backed proof paths
- small local models with limited context
- provider-gated or privacy-sensitive environments
- deterministic artifact-shape testing

Packet-only review is safer and more portable, but it is weaker evidence. The
reviewer cannot inspect surrounding code, call sites, hidden invariants, or
tests that were not included in the packet.

### Tier 2: Read-Only Repo Review

Read-only repo review is the stronger target for real use. The reviewer receives
the packet plus bounded read-only access to the issue worktree or a review packet
root that mirrors the relevant repository slice.

Allowed:

- read files
- inspect surrounding code and tests
- inspect manifests, schemas, fixtures, and docs relevant to the diff
- cite evidence from the repository slice

Forbidden:

- edit files
- stage, commit, push, merge, or open PRs
- run arbitrary commands outside ADL-mediated static-analysis lanes
- claim validation that ADL did not supply

The result artifact must record which tier was used:

```json
{
  "visibility_mode": "packet_only|read_only_repo",
  "repo_access": {
    "read_only": true,
    "write_allowed": false,
    "tool_execution_allowed": false
  }
}
```

Gate policy should treat packet-only blessings as weaker evidence than
read-only-repo blessings. A packet-only `blessed` result may support PR opening
for small or fixture-scoped changes, but higher-risk runtime changes should
prefer read-only repo review or an explicit operator waiver.

## Non-Negotiable Boundaries

- The reviewer backend cannot push, commit, merge, edit files, or run unbounded
  tools.
- The reviewer backend cannot claim tests passed unless ADL supplied validation
  evidence.
- The reviewer backend cannot broaden issue scope without routing that concern
  as follow-up work.
- The reviewer backend must identify itself by backend, model, and session or
  invocation id where available.
- The reviewer backend must be distinct from the writing session.
- Human/operator authority remains responsible for final merge decisions.

## Review Packet

The packet should be compact, reproducible, and safe to hand to different
reviewer backends.

Required fields:

- `schema_version`
- `visibility_mode`
- `issue_number`
- `issue_title`
- `source_prompt_excerpt`
- `stp_excerpt`
- `sip_excerpt`
- `sor_excerpt`
- `branch`
- `base_ref`
- `head_ref`
- `changed_files`
- `diff_summary`
- `focused_diff_hunks`
- `validation_evidence`
- `static_analysis_evidence`
- `repo_slice_manifest`
- `review_scope`
- `non_scope`
- `known_risks`
- `redaction_status`

The packet should avoid full unbounded repository dumps, secrets, raw prompts,
credentials, private state, and unrelated history.

When `visibility_mode` is `read_only_repo`, the packet should include a bounded
repo slice manifest that names allowed roots or files. The reviewer may inspect
only that slice unless the operator explicitly widens scope.

## Static Analyzer Lane

Static analysis should run before semantic model review so the model reviews
both the code and machine-check evidence.

Repository-native analyzers available now:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --doc`
- focused `cargo test` or `cargo nextest` for changed Rust surfaces
- `cargo llvm-cov` or the repository coverage-impact lane when required by path
  policy
- `git diff --check`
- `bash adl/tools/validate_structured_prompt.sh ...` for card and prompt
  surfaces
- `bash adl/tools/test_check_coverage_impact.sh`
- `bash adl/tools/test_run_authoritative_coverage_lane.sh`
- demo or review-packet validators under `adl/tools/validate_*.py` when the
  changed surface is one of those demos or packets

Optional analyzers to evaluate later:

- `cargo audit` for Rust dependency vulnerability checks
- `cargo deny` for license, advisory, and duplicate dependency policy
- `typos` for typo detection in docs and code
- `semgrep` for repository-specific security and correctness patterns
- `shellcheck` for shell scripts
- `cargo machete` or `cargo udeps` for unused dependency cleanup

Optional analyzers should not be treated as required until the repository has
configuration, installation guidance, and acceptance rules for them.

## ADL Review Heuristics Integration

The reviewer should consume the ADL review heuristics as a rule taxonomy:

- functional correctness: `F1-F10`
- security and safety: `S1-S10`
- code quality and maintainability: `Q1-Q10`
- architecture and system alignment: `A1-A10`
- performance and efficiency: `P1-P10`
- testing and verification: `T1-T10`
- AI-specific risks: `AI1-AI10`
- ADL-specific review discipline: `C1-C10`

The first implementation can embed this taxonomy as a static prompt/rubric. A
later implementation can compile the rules into machine-readable YAML and route
each rule as `static`, `pattern`, `semantic`, or `hybrid`.

## Reviewer Skill Contract

All reviewer backends must use the same normalized output shape:

```json
{
  "schema_version": "adl.pr_review_result.v1",
  "review_id": "string",
  "reviewer_backend": "fixture|gemma4_local|codex|chatgpt|claude",
  "reviewer_model": "string",
  "reviewer_session": "string",
  "writer_session": "string",
  "same_session_as_writer": false,
  "visibility_mode": "packet_only|read_only_repo",
  "repo_access": {
    "read_only": true,
    "write_allowed": false,
    "tool_execution_allowed": false
  },
  "packet_id": "string",
  "static_analysis_summary": [],
  "findings": [],
  "disposition": "blessed|blocked|non_proving|skipped",
  "residual_risk": [],
  "validation_claims": [],
  "non_claims": []
}
```

Finding fields:

- `title`
- `priority`
- `file`
- `line`
- `body`
- `evidence`
- `heuristic_ids`
- `confidence`
- `blocking`
- `suggested_fix_scope`

The normalizer must reject reviewer output that lacks evidence for blocking
findings, claims unavailable validation, marks `same_session_as_writer` as true,
or attempts to request write or merge authority.

## Backend Modes

### Fixture

The fixture backend proves packet and artifact shape deterministically. It
should include one no-findings path and one blocking-finding path.

### Gemma4 Local

Gemma4 is the flagship local-model path. It should receive the same packet and
produce the same normalized artifact. The first implementation should call the
Ollama API directly:

```text
POST http://127.0.0.1:11434/api/generate
```

`OLLAMA_HOST` or an explicit tool argument may select another trusted local
endpoint. Missing local model availability should be classified as `skipped`,
not as proof failure, unless the operator explicitly asks for a required live
run.

### Codex, ChatGPT, Claude

Hosted or separate-agent reviewers are comparative backends. They must be
operator-gated when credentials, cost, network, or provider availability are
involved. They must still use the same packet and output schema.

## Two-Agent Adversarial Mode

The demo should support a panel mode:

- Blessing reviewer: evaluates whether the PR has enough evidence to publish.
- Adversarial reviewer: searches for correctness, security, proof, test, docs,
  and lifecycle blockers.
- Reconciler: deduplicates findings, records disagreement, and emits a final
  panel disposition.

Panel dispositions:

- `panel_blessed`
- `panel_blocked`
- `panel_split`
- `panel_non_proving`
- `panel_skipped`

The panel is not allowed to override the evidence rules. A blessing reviewer can
only bless from packet evidence, and an adversarial reviewer can only block from
packet evidence.

## Freedom Gate Policy

Initial gate policy:

- Block PR opening on any `P0`, `P1`, or `P2` finding marked `blocking`.
- Block PR opening when the reviewer output is malformed or same-session.
- Block PR opening when required static analysis failed.
- Prefer read-only repo review for runtime, security, schema, lifecycle, or
  broad refactor changes. Packet-only blessing for those changes requires an
  explicit operator waiver or a follow-up read-only review.
- Permit PR opening with explicit residual risk for non-blocking `P3+` findings
  if the execution SOR records them.
- Permit `skipped` only when provider unavailability, operator policy, or
  fixture-only mode is explicitly recorded.

The gate result is merge-readiness evidence only. It is not merge approval.

## Demo Proof Path

Minimum proving path:

1. Build a fixture PR review packet.
2. Run static analyzer simulation or bounded local analyzer evidence.
3. Run fixture reviewer in no-findings mode.
4. Run fixture reviewer in blocking-finding mode.
5. Validate normalized review artifacts in packet-only mode.
6. Run a read-only repo review fixture over a bounded repo slice manifest.
7. Run two-agent panel reconciliation over fixture outputs.
8. Emit a run summary that classifies which live backends ran, skipped, or were
   unavailable.

Optional live path:

1. Run the same packet through direct Ollama HTTP with `gemma4_local` when
   available.
2. Optionally run through `codex`, `chatgpt`, or `claude` when operator-gated.
3. Compare dispositions without claiming provider ranking.

## Open Design Questions

- Whether the pre-PR hook lives inside `pr finish` initially or as an explicit
  `adl pr review-gate` command that `pr finish` can call.
- Whether the static analyzer lane should fail closed by default for Rust source
  changes or inherit the existing CI runtime path policy.
- How much diff context a local model can review reliably before packet
  compression becomes necessary.
- How strongly `pr finish` should enforce this tool once it has enough
  production mileage.

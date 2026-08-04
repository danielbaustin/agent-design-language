# Using Claude Opus for ADL Reviews

This runbook describes the bounded, evidence-first way to use the approved
Claude Opus provider adapter as a pre-publication reviewer. It is an operator
procedure, not a substitute for the typed C-SDLC v2 review record.

## Preconditions

- Work is in an issue-bound worktree on a non-`main` branch.
- The implementation has focused local validation and a clean exact revision.
- The review scope is an explicit list of changed paths.
- The approved credential is supplied from `$HOME/keys/claude2.key` for one
  command only. Never print, copy, commit, or persist its contents.
- The review request includes the actual diff or a concise source-grounded
  patch summary. A prompt that only says “review this” is insufficient.

## Invoke Opus through the Rust adapter

Use the repository's `adl-provider-adapter` binary, not `curl` and not a
provider-specific shell wrapper. Map the approved key only for the invocation:

```sh
ANTHROPIC_API_KEY="$(< "$HOME/keys/claude2.key")" \
  cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- \
  --provider anthropic --model claude-opus-5 \
  --max-output-tokens 4096 \
  --prompt-file /path/to/review-prompt.txt
```

Use a bounded output budget appropriate to the review. Do not set an
artificially tiny budget; the adapter's request clamp is the safety boundary.
The prompt should identify:

1. issue number and acceptance criteria;
2. exact reviewed revision and changed-path scope;
3. relevant diff or source excerpts;
4. validation commands and truthful results;
5. live-provider limitations, if any; and
6. the required findings format: severity, evidence, disposition, and residual
   risk.

## Record findings

Treat Opus output as review evidence, not lifecycle authority. Classify every
finding as actionable/in-scope or routed/out-of-scope. Fix actionable findings,
rerun focused validation, and obtain an exact-head re-review. Then record the
review with the typed v2 command:

```sh
cargo run --manifest-path csdlc-v2/Cargo.toml --bin csdlc-review -- \
  --root . record --request .csdlc/review-record-<issue>.json
```

`review-record-<issue>.json` must name the exact clean scoped revision and
include dispositions for every actionable finding. Publication must fail closed
if the review is missing or stale; use `csdlc-review guard` to diagnose that
condition before publishing.

## Truth boundaries

- A successful Opus response proves that the reviewer adapter reached the
  provider; it does not prove the implementation is correct by itself.
- A live provider probe must be reported separately from mocked/local adapter
  tests. Account balance, quota, authentication, or rate-limit failures must
  remain typed and truthful; never report a completion success that was not
  observed.
- Metadata-only lifecycle commits can stale a source review. Re-record a
  typed metadata-only proof or perform a fresh exact-head review before
  publication.
- Do not expose credentials, full provider responses containing sensitive
  data, or machine-local absolute paths in committed review artifacts.

## Minimum evidence package

- focused test output and `git diff --check` result;
- Opus prompt and response summary with findings/dispositions;
- exact reviewed revision and changed-path scope;
- live-probe disposition, including blocked-provider reasons; and
- typed `csdlc-review` record and guard result immediately before
  `csdlc-publish`.

Validation of this runbook is limited to checking the referenced Rust adapter,
typed review command, credential path convention, and repository lifecycle
skill. It does not claim a provider completion or release approval.

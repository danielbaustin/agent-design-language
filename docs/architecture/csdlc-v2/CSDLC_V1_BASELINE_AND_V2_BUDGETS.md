# C-SDLC v1 Baseline And v2 Budgets

Issue: #5228  
Captured: 2026-07-12  
Host: Apple arm64, macOS 26.5 build 25F71  
Repository revision: `020bba17deb9f172e91a2ec5c0599cf42e4defe9`  
Rust: `rustc 1.92.0 (ded5c06cf 2025-12-08)`  
Cargo: `cargo 1.92.0 (344c4567c 2025-10-21)`  
Installed artifacts: stable `.adl/bin` owner binaries built immediately before
capture with `bash adl/tools/install_owner_binaries.sh`

## Measurement Rules

- Commands are repository-relative unless an installed binary path is being
  measured.
- Build comparisons use the same host/toolchain and state their cache posture.
- Networked doctor timing is kept separate from local computation.
- The selected v1 source slice is a lower bound, not a claim that every one of
  the 614 tracked shell/Python tools belongs to C-SDLC.
- Gate 2 must add normalized v2 measurements against the fixtures defined here.
- Counts and deletion denominators are pinned to the repository revision above.

## Source And Test Surface

Command:

```sh
wc -l \
  adl/tools/pr.sh \
  adl/src/cli/pr_cmd.rs \
  adl/src/cli/pr_cmd_args.rs \
  adl/src/cli/pr_cmd/finish_support.rs \
  adl/src/csdlc_prompt_editor.rs \
  adl/src/session_ledger.rs \
  adl/src/cli/tooling_cmd/prompt_template.rs \
  adl/src/cli/tooling_cmd/structured_prompt.rs
```

Result: 21,438 lines across eight obvious control-plane files.

Tracked top-level tool counts:

| Surface | Count |
| --- | ---: |
| `adl/tools/*.sh` | 469 |
| `adl/tools/*.py` | 145 |

A heuristic count of `#[test]` and `#[tokio::test]` annotations in Rust files
whose paths contain `pr_`, `csdlc`, `prompt`, `issue`, `session`, or `cli`
returned 1,227. This is a navigation baseline, not an exact ownership claim.

Reproduction:

```sh
git ls-files 'adl/tools/*.sh' | wc -l
git ls-files 'adl/tools/*.py' | wc -l
files=$(rg -l '#\[(tokio::)?test\]' adl/src |
  rg 'pr_|csdlc|prompt|issue|session|cli')
printf '%s\n' "$files" | xargs rg -c '#\[(tokio::)?test\]' |
  awk -F: '{s+=$2} END{print s+0}'
```

## Authoritative Deletion Denominator

For the 90% target, Gate 1 defines the incumbent C-SDLC implementation/test
denominator at the pinned revision as:

- 52 Rust files and 40,148 lines: all `adl/src/cli/pr_cmd/**/*.rs`, the six
  adjacent `pr_cmd*.rs` modules, `csdlc_prompt_editor.rs`, `session_ledger.rs`,
  `pr_dispatch_support.rs`, prompt-template/structured-prompt tooling modules,
  direct `adl_pr_*` binaries, `adl_csdlc.rs`, `csdlc.rs`, `adl_issue.rs`, and
  `adl_session.rs`;
- 43 directly owned shell implementation/test files and 9,831 lines: `pr.sh`,
  card/delegate/usage/prompt-template helpers, structured-prompt validation,
  and `check_pr_*`, `test_pr_*`, and `test_prompt_template*` scripts;
- total denominator: 95 files and 49,979 lines.

The exact deterministic discovery commands are:

```sh
{ find adl/src/cli/pr_cmd -type f -name '*.rs';
  printf '%s\n' adl/src/cli/pr_cmd.rs adl/src/cli/pr_cmd_args.rs \
    adl/src/cli/pr_cmd_cards.rs adl/src/cli/pr_cmd_prompt.rs \
    adl/src/cli/pr_cmd_validate.rs adl/src/csdlc_prompt_editor.rs \
    adl/src/session_ledger.rs adl/src/pr_dispatch_support.rs \
    adl/src/cli/tooling_cmd/prompt_template.rs \
    adl/src/cli/tooling_cmd/structured_prompt.rs;
  find adl/src/bin -maxdepth 1 -type f \
    \( -name 'adl_pr_*.rs' -o -name 'adl_csdlc.rs' -o -name 'csdlc.rs' \
       -o -name 'adl_issue.rs' -o -name 'adl_session.rs' \); } | sort -u

{ printf '%s\n' adl/tools/pr.sh adl/tools/card_paths.sh \
    adl/tools/pr_cards.sh adl/tools/pr_delegate.sh adl/tools/pr_usage.sh;
  find adl/tools -maxdepth 1 -type f \
    \( -name 'check_pr_*.sh' -o -name 'test_pr_*.sh' \
       -o -name 'test_prompt_template*.sh' \
       -o -name 'validate_structured_prompt.sh' \
       -o -name 'prompt_template.sh' \); } | sort -u
```

The captured sorted lists hashed to
`c3118c1f3766b5f4a3e549c9073b33fb83164b3006175785b4c08f84c898558f`
(Rust) and
`7160399a788e467ebd934309a99f9777fb0f14d4270989e5114c73b63fa3d8cc`
(shell). Cutover deletion percentage uses this fixed 49,979-line denominator;
new v1 code added later is reported separately and cannot improve the ratio.

Within the authoritative 52-file Rust denominator there are 277 Rust test
annotations. The shell denominator contains 35 `test_*` programs; these are
counted as test programs rather than pretending each internal assertion is one
comparable test. Reproduction:

```sh
xargs rg -c '#\[(tokio::)?test\]' <rust-file-list |
  awk -F: '{s+=$2} END{print s+0}'
awk '/test_/ {n++} END{print n+0}' <shell-file-list
```

The v2 60–100 target and 150 ceiling apply to executable test cases registered
by the standalone Rust workspace. Script-level compatibility tests are not
ported.

## Installed Binary Footprint

The current stable installed binaries are unstripped debug-owner artifacts.
They still demonstrate the effect of linking each thin wrapper to the large
main crate.

| Binary | Bytes | MiB |
| --- | ---: | ---: |
| `adl-pr-create` | 42,924,816 | 40.94 |
| `adl-pr-doctor` | 42,924,672 | 40.94 |
| `adl-pr-run` | 42,909,560 | 40.92 |
| `adl-pr-finish` | 42,924,768 | 40.94 |
| `adl-pr-shepherd` | 42,934,720 | 40.95 |
| `adl-pr-closeout` | 42,934,720 | 40.95 |
| `adl-csdlc` | 48,590,040 | 46.34 |
| Total | 306,143,296 | 291.96 |

Measurement command:

```sh
stat -f '%N %z' .adl/bin/adl-pr-create .adl/bin/adl-pr-doctor \
  .adl/bin/adl-pr-run .adl/bin/adl-pr-finish .adl/bin/adl-pr-shepherd \
  .adl/bin/adl-pr-closeout .adl/bin/adl-csdlc
```

Sample count is one per freshly installed artifact; cache state does not affect
file size.

V2 target: at most 15 MiB per stripped release binary and 70 MiB total for the
seven installed binaries. Gate 2 must measure equivalent stripped release
artifacts before accepting or revising this target.

## Construction

Warm no-change command:

```sh
/usr/bin/time -p cargo build --manifest-path adl/Cargo.toml --bin adl-pr-doctor
```

Result: 0.31 seconds real after the owner-binary installation populated the
target.

Warm touched-entrypoint command:

```sh
touch adl/src/bin/adl_pr_doctor.rs
/usr/bin/time -p cargo build --manifest-path adl/Cargo.toml --bin adl-pr-doctor
```

Result: 3.25 seconds real. Git content remained unchanged.

Clean isolated command:

```sh
mkdir -p /Volumes/FastWork/adl-csdlc-v1-baseline-target
/usr/bin/time -p -o /tmp/5228-cold-build-time \
  env CARGO_TARGET_DIR=/Volumes/FastWork/adl-csdlc-v1-baseline-target \
  cargo build --locked --manifest-path adl/Cargo.toml --bin adl-pr-doctor
```

Cache posture: empty dedicated target directory, Cargo registry/source cache
warm, no prior build artifacts in the target. Result: 418.55 seconds real,
619.94 seconds user, 43.42 seconds system. The build compiled the main ADL,
`adl-runtime`, Google Workspace, and extensive AWS dependency graph for the
doctor binary.

Gate 2 must run the paired v2 clean build on the same host/toolchain or use a
reviewed equivalent CI builder fixture.

V2 targets:

- clean construction at most 50% of the normalized v1 result;
- warm incremental construction at most 25% of v1;
- no build edge to `adl`, `adl-runtime`, or `adl-runtime-kernel`.

## Doctor Latency

Command shape:

```sh
/usr/bin/time -p bash adl/tools/pr.sh doctor 5228 \
  --slug v0-92-csdlc-v2-clean-room-architecture-baseline \
  --version v0.92 --mode full --json
```

Five completed real-time samples were 71.54, 100.19, 113.85, 128.11, and
119.14 seconds. Median was 113.85 seconds; nearest-rank p95 was 128.11 seconds.
CPU time stayed below 0.6 seconds per sample; normal v1 doctor latency is
dominated by live GitHub queries. One earlier pilot was terminated at 30.66
seconds and is excluded from the five-sample statistics.

Cache/network posture: installed warm owner binary, local repository/card
state warm, live GitHub issue/PR scans enabled, sequential samples. Raw JSON
was captured separately per sample and timing used `/usr/bin/time -p`.

V2 separates local doctor from explicit remote refresh:

- local read-only doctor p95 target: below one second over at least 21 fixtures;
- remote refresh reports its own network duration and never changes the local
  performance claim;
- scheduler/shepherd decide whether remote freshness is required.

## Prompt Structure Validation

Focused installed-owner validation:

| Card | Result | Real time |
| --- | --- | ---: |
| SPP structure | PASS | 0.02 s |
| VPP structure | PASS | 0.01 s |

Commands, one sample each on warm installed binaries:

```sh
/usr/bin/time -p .adl/bin/adl-csdlc tooling prompt-template \
  validate-structure --kind spp --input <spp.md> --repo-root .
/usr/bin/time -p .adl/bin/adl-csdlc tooling prompt-template \
  validate-structure --kind vpp --input <vpp.md> --repo-root .
```

This confirms individual structural checks can be fast. The v2 card engine must
keep that property while combining typed values, Markdown.rs mdast, semantic
anchors, cross-card invariants, and atomic transaction proof.

## Init And Bind

Issue creation and binding are mutating remote/repository operations and were
not repeated solely for benchmarking. The #5228 execution logs retain observed
bootstrap and bind evidence. Gate 2 must provide deterministic temporary-repo
fixtures for local init/bind planning; live GitHub/fetch/worktree materialization
is reported separately.

V2 local targets, excluding network/fetch/build/materialization:

- `init` planning p95 below two seconds;
- `bind` planning/validation p95 below two seconds.

## Validation Budgets

- Warm focused C-SDLC validation: under two minutes.
- Complete deterministic non-live C-SDLC validation: under ten minutes.
- No normal C-SDLC validation builds/tests main ADL or Runtime products.
- Cross-product proof is an explicit optional PVF lane and excluded from the
  C-SDLC fast-lane claim.
- Target 60–100 tests; hard ceiling 150.

## Gate 1 Disposition

The measured v1 surface supports proceeding with the standalone clean-room
design. Local init/bind p95 values and the paired v2 construction result are
explicit Gate 2 measurement obligations, not hidden passing claims.

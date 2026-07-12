# C-SDLC v2 Gate 2 Validation

Issue: #5232  
Scope: standalone state engine, automatic cards, semantic editor, and doctor

## Results

| Proof | Result |
| --- | --- |
| `cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check` | PASS |
| `cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings` | PASS |
| `cargo test --manifest-path csdlc-v2/Cargo.toml` | PASS: 12 focused tests |
| Standalone `cargo metadata --no-deps` dependency inspection | PASS: no ADL or Runtime crate |
| Full working-tree selector plan | PASS: selects `csdlc_v2_standalone`, `docs_diff_check`, `ci_path_policy_contracts`, `csdlc_owner_lane`, and `rust_pr_fast`; the latter two are one-time v1 publisher-bridge proof |
| `ci_path_policy_contracts` authoritative command | PASS: all six constituent contract scripts |
| Focused v1 finish registration contract | PASS: exact registered `csdlc-v2/Cargo.toml` command accepted; arbitrary Cargo commands remain refused |
| Legacy `csdlc_owner_lane` bridge proof | PASS |
| Legacy focused `rust_pr_fast` bridge proof | PASS: 197/197 selected finish tests; 937 skipped |
| Mermaid source render to saved SVG/PNG | PASS for both diagrams |
| CLI bootstrap + doctor fixture | PASS: six cards, ready doctor report |
| Identical-input whole-record replay | PASS: recursive issue-directory diff empty |
| 21 final release-doctor samples | PASS: nearest-rank p95 0.02 s; max 0.03 s at `/usr/bin/time -p` resolution |

## Construction And Size

Environment: same macOS host/toolchain/cache family as the Gate 1 baseline;
isolated empty target directory at `/Volumes/FastWork/csdlc-v2-gate2-final-review-cold`.
The target path is measurement-only and is not a durable artifact.

- Isolated final reviewed-revision clean release build: 95.98 s real, 58.36 s
  user, 3.44 s sys.
- Immediate no-change release build: 0.06 s real.
- A final reviewed-revision touched-crate thin-LTO relink measured 16.71 s real,
  below the 104.64 s 25%-of-v1 incremental budget.
- Stripped release `csdlc-edit`: 1.8 MiB.
- Stripped release `csdlc-doctor`: 1.4 MiB.
- Current Rust implementation plus focused tests: 3,044 lines.
- Test count: 12.

Against Gate 1's v1 isolated clean build of 418.55 s, the final reviewed Gate 2
clean release build is 22.93% of v1 (77.07% lower), inside the 50%
construction budget. The two installed owner binaries total roughly 3.2 MiB versus the
15 MiB per-binary and 70 MiB seven-binary-set ceilings.

## Behavioral Coverage

The focused suite proves:

- automatic construction of six typed, card-specific contracts from one issue
  input, including profile-derived SPP/VPP time and token budgets;
- independent per-card template shapes plus mdast structural/anchor validation;
- field ownership and invalid transition refusal;
- stale generation/digest refusal;
- direct Markdown drift detection;
- index/claim/audit/transition/card identity and digest corruption detection;
- design/diagram digest readiness and lifecycle-transition refusal;
- parent-directory fsync barriers, whole-record interruption evidence, and
  deterministic next-writer recovery;
- stable public schema generation.

No live network, GitHub mutation, Git/worktree binding, or PVF execution
participates in the standalone v2 proof. Those remain later-gate work.
The v1 selector integration maps `csdlc-v2/**` to the single standalone Cargo
test command. Because the selector manifest and the legacy finish allowlist
change in this issue, the full PR additionally selects the existing
`ci_path_policy_contracts`, `csdlc_owner_lane`, and `rust_pr_fast` bridge
proofs. Those legacy lanes validate the temporary publication adapter only;
they are not v2 dependencies and are not part of normal v2 construction or
validation. The focused finish-contract build itself demonstrates why v2 must
remain separate: even one filtered legacy test paid the old monolithic build
cost (14m25s cold), and its one owner-binary rebuild took 9m45s. The focused
legacy PR-fast proof then spent 6m19s compiling before running 197 tests. The
complete standalone v2 suite remains 12 focused tests.

## Renderer Note

The first Mermaid attempt used the CLI's missing cached-browser default and
failed before rendering. Re-running with the installed Google Chrome path
produced all four saved SVG/PNG assets. This is renderer configuration variance,
not a diagram-source failure.

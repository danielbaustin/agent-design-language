# C-SDLC v2 Sprint Dependency Review

Revision reviewed: `7c3e1e0e86a4ca982231ce91c39073530c5408e6`

## Findings

### P1 - The locked dependency graph does not support the package's declared Rust 1.85 MSRV

- **Evidence:** `csdlc-v2/Cargo.toml:5` declares `rust-version = "1.85"`, while `csdlc-v2/Cargo.toml:88` permits any compatible `time` 0.3 release. The committed lockfile selects `time` 0.3.53, `time-core` 0.1.9, and `time-macros` 0.2.31 at `csdlc-v2/Cargo.lock:1847-1875`. Those exact locked crates declare Rust 1.88.0 as their MSRV in the locally cached registry metadata. The lockfile also selects Octocrab 0.53.1, Clap 4.6.1, and Tokio 1.52.3; their declared MSRVs are at or below 1.85, so the `time` family is the concrete blocker.
- **Scenario:** A clean builder honors the package contract and invokes locked construction with Rust 1.85. Cargo rejects the selected `time` family before compiling C-SDLC v2 because those packages require Rust 1.88.
- **Impact:** The standalone workspace is not constructible on its advertised minimum toolchain. A release builder, downstream operator, or future CI lane pinned to 1.85 fails even though current validation on Rust 1.92 is green. This also makes the Gate 2 claim of a standalone Cargo boundary weaker than the manifest contract presented to consumers.
- **Missing proof:** No checked MSRV lane builds `csdlc-v2/Cargo.lock` with Rust 1.85, and no dependency-policy check rejects locked packages whose `rust-version` exceeds the root package's declaration. The current host did not have 1.85 installed, so the review did not install a toolchain; the incompatibility was established from the exact locked package metadata.
- **Follow-up owner:** C-SDLC v2 dependency/toolchain owner. Either restore a lock graph compatible with 1.85 or raise the package MSRV and prove that toolchain in CI and installation documentation.
- **Discovery boundary:** Independently derived from the manifest and lockfile. None of testing discoveries #5364-#5373 concerns Rust or crate-version compatibility.

### P1 - The final stable installation omits the binary required to resolve the sole generation authority

- **Evidence:** `csdlc-v2/Cargo.toml:61-63` declares `csdlc-install`, but `csdlc-v2/operator/skills.json:1-10` includes only lifecycle routes and auxiliaries. `SkillManifest::binaries` derives the install set exclusively from those routes (`csdlc-v2/src/operator.rs:113-125`), and `install_binaries` copies exactly that set (`csdlc-v2/src/operator.rs:289-325`). The final coexistence inventory likewise requires only ten lifecycle binaries (`csdlc-v2/operator/coexistence.json:17`). The installer test confirms an eleven-binary receipt and accepts it as complete (`csdlc-v2/tests/gate10a.rs:31-71`), while the repository contract requires current routing through `csdlc-install resolve` (`csdlc-v2/AGENTS.md:6-8`).
- **Scenario:** An operator builds all binaries, runs `csdlc-install install` into `.adl/bin/csdlc-v2/`, and then uses only that stable directory as the operational authority. The installed set passes `csdlc-install verify`, but contains no installed `csdlc-install`; the required `resolve` entrypoint is available only from an unstaged Cargo target or another ungoverned path.
- **Impact:** The declared final authority is not self-contained. Fresh and clean-room installations cannot perform the mandated selector resolution from the verified stable generation directory, encouraging fallback to disposable build output or stale external binaries whose provenance is outside the installation receipt.
- **Missing proof:** No install test invokes `csdlc-install resolve` from the installed destination, and neither the skill manifest nor final coexistence inventory asserts that the installer/resolver itself is present and executable.
- **Follow-up owner:** C-SDLC v2 installation/operator owner. Define and test a bootstrappable stable owner set that includes the resolver or provide a separately verified bootstrap contract.
- **Discovery boundary:** Independently derived from installation-set construction. None of #5364-#5373 reports the missing installed resolver. The concurrent issue #5375 test specialist's observation that the stable set lacked `csdlc-install` corroborates this finding; it was not used as its source. Testing discovery #5369 is a different selector-file-shape defect and does not subsume this installation failure.

### P1 - The installation receipt authenticates supplied bytes but records no build or source provenance

- **Evidence:** `InstallReceipt` contains only destination, binary name, and BLAKE3 digest (`csdlc-v2/src/operator.rs:53-65`). Installation accepts any regular executable files from a caller-selected directory and hashes their bytes (`csdlc-v2/src/operator.rs:289-325`); verification only recomputes those same hashes (`csdlc-v2/src/operator.rs:388-427`). The proof fixture writes each binary name as executable text, installs those arbitrary files, and obtains a passing coexistence report (`csdlc-v2/tests/gate10a.rs:31-71`). Nevertheless, Gate 10A calls this "BLAKE3 provenance" (`docs/architecture/csdlc-v2/gate10a/DESIGN.md:3`), and Gate 10D2 cites that fixture as proof of "generation selector and installation provenance" (`docs/architecture/csdlc-v2/gate10d2/CAPABILITY_MATRIX.json:30`).
- **Scenario:** A stale, locally modified, incorrectly profiled, or malicious set of executables is placed in the source directory. `csdlc-install install` creates a matching receipt, and `verify` reports the set complete because the receipt says only that the copied bytes have not changed since installation.
- **Impact:** Gate 10 verification cannot distinguish reviewed binaries built from the approved revision and lockfile from arbitrary executable content. The stable path can therefore acquire a valid-looking receipt for code that did not pass the sprint's review or dependency controls, undermining the supply-chain claim used to authorize v1 sunset.
- **Missing proof:** The receipt and verifier bind neither Git revision, dirty-tree state, `Cargo.lock` digest, manifest digest, Rust/Cargo version, target triple, release profile, build command, nor an authenticated builder identity. No negative test proves rejection of bytes built from a different revision or graph.
- **Follow-up owner:** C-SDLC v2 supply-chain and installation owner. Replace the integrity-only claim with truthful terminology or bind installation to reviewable build provenance and add cross-revision/stale-build rejection tests.
- **Discovery boundary:** Independently derived from receipt schema and the Gate 10A fixture. None of testing discoveries #5364-#5373 addresses binary build provenance.

### P2 - The independent workspace has no continuous dependency, MSRV, or supply-chain gate

- **Evidence:** Gate 1 proved JSON syntax and a locked **v1** build, then explicitly deferred paired v2 clean construction (`docs/architecture/csdlc-v2/CSDLC_V2_GATE1_VALIDATION.md:6-35`). Gate 2's dependency proof was `cargo metadata --no-deps`, which proves only the absence of ADL/Runtime package edges (`docs/architecture/csdlc-v2/gate2/GATE2_VALIDATION.md:8-18`). Gate 10B's fixed build, test, and clippy commands omit `--locked` (`csdlc-v2/operator/pre-switch-proof.json:8-11`), and the current CI workflow contains no build, test, clippy, MSRV, audit, deny, SBOM, or license step for `csdlc-v2`; its only C-SDLC-v2-specific condition excludes matching PR branch names from one coverage-impact step (`.github/workflows/ci.yaml:584-598`). Gate 10D2 records one locked test result but no graph/MSRV/provenance analysis (`docs/architecture/csdlc-v2/gate10d2/DELETION_EVIDENCE.json:26-30`).
- **Scenario:** A lockfile refresh selects a crate with a higher MSRV, a materially broader feature graph, or a newly duplicated transport/crypto stack. Local current-stable tests pass, and the normal CI workflow never evaluates the standalone workspace's dependency contract.
- **Impact:** Dependency drift can reach the final authority without automated evidence that it remains reproducible, MSRV-compatible, license-reviewed, or within the intended small clean-room graph. The present Rust 1.85/`time` mismatch demonstrates that this is an active control gap rather than a theoretical hygiene concern.
- **Missing proof:** There is no checked `cargo test/clippy --locked` lane for this manifest, no declared MSRV matrix, no `cargo deny`/vet/audit policy, no license or source allowlist, no SBOM/provenance artifact, and no exact dependency/feature snapshot comparison at Gate 10D2.
- **Follow-up owner:** Repository CI and C-SDLC v2 dependency owners. Add a focused independent-workspace lane and define which supply-chain assertions are release gates versus explicitly deferred review work.
- **Discovery boundary:** Independently derived from the tracked proof manifests and CI workflow. None of #5364-#5373 covers dependency governance.

## Reviewed Dependency Surfaces

- `csdlc-v2/Cargo.toml` and `csdlc-v2/Cargo.lock`, including direct dependency use, selected versions, duplicate package families, default-feature behavior, and Rust-version compatibility.
- Octocrab's reduced feature selection: defaults are disabled; the selected graph uses Rustls with Ring and contains no OpenSSL or `aws-lc-rs` package. The remaining crypto/HTTP graph is substantial but directly attributable to the typed GitHub boundary.
- Direct dependency use across production and tests. No clearly unused direct dependency was found; `libc` is used by PVF process-group termination and Unix tests, while `sha2`, `time`, Octocrab, Tokio, and URL serve Gate 10 eligibility/cutover or GitHub owners.
- Lockfile source posture. The reviewed graph uses crates.io registry sources with checksums and contains no Git or path dependency on ADL, Runtime v2, or Runtime v3.
- Gate 1/2/9/10 dependency and clean-room evidence, Gate 10A installation/receipt verification, Gate 10B fixed proof commands, Gate 10D2 capability/deletion evidence, and current GitHub Actions coverage.
- Stable binary-set derivation, installation transaction, receipt verification, and final coexistence inventory.

## Testing-Discovery Comparison

Pre-existing testing discoveries #5364-#5373 were consulted only after the dependency findings were independently established. They concern planning-card replan, deleted v1 VPP commands, helper-skill authority, planned-milestone staleness, dirty-review publication, selector-file shape, PR-state collection, shepherding, v1-origin PR tails, and shepherd CLI schema. None is promoted or repeated here as a dependency finding. The only corroboration labeled above comes from the concurrent issue #5375 test specialist, not from #5364-#5373.

## Validation And Limits

- `cargo tree --manifest-path csdlc-v2/Cargo.toml --locked --offline -d` completed and was reviewed for duplicate package families. The visible duplicates were normal target/build-context or major-version splits; no separate finding was justified.
- `cargo tree --manifest-path csdlc-v2/Cargo.toml --locked --offline -e features` completed and was reviewed for default-feature and transport/crypto activation.
- `cargo check --manifest-path csdlc-v2/Cargo.toml --locked --offline --target-dir <external-target-dir>` passed on Rust 1.92.0.
- An exact Rust 1.85 build was not run because that toolchain is not installed and the review did not mutate the operator's toolchain state. Locked crate metadata establishes the 1.88 requirement of the selected `time` family.
- A cold offline metadata resolution failed on the first package absent from the local cache. This confirms that `Cargo.lock` plus checksums does not itself vendor dependencies; it is not reported as a defect because the repository makes no vendoring/offline-install claim.
- External vulnerability feeds and license databases were intentionally not queried. This review makes no claim that the locked graph is vulnerability-free or legally approved.

## Residual Dependency Risk

The lockfile gives deterministic crate identities and checksums, the manifest is independent of ADL/Runtime packages, and Octocrab defaults are intentionally narrowed. Those are useful foundations. Until the four findings above are resolved, however, the final installed authority is neither build-provenance-bound nor self-resolving, its advertised MSRV is false for the committed graph, and future dependency drift has no continuous repository gate.

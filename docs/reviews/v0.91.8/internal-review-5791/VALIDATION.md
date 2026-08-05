# Validation

Review head: `70f4e76509de219ccff6ffb534f9199d74eaece2`

## Commands Run

```text
CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/5791/csdlc-v2 cargo run --locked --manifest-path csdlc-v2/Cargo.toml --bin csdlc-install -- verify --repo /Volumes/FastWork/adl-wp-5791 --bin-dir /Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2 --inventory csdlc-v2/operator/coexistence.json
```

Result: passed. The stable installed C-SDLC v2 binary set contains `csdlc-clean`
and no longer contains deleted `csdlc-closeout` or `csdlc-merge` binaries.

```text
CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/5791/csdlc-v2 cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate_terminal_authority_deletion
```

Result: passed, 4 tests.

```text
bash adl/tools/test_closeout_completed_issue_wave.sh
```

Result: passed.

```text
CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/5791/csdlc-v2 cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate4 routine_lifecycle_contract_measures_four_commands_and_two_artifacts
```

Result: passed.

```text
bash adl/tools/test_editor_action.sh
```

Result: passed.

```text
bash adl/tools/test_generate_active_command_reference_scan.sh
```

Result: passed.

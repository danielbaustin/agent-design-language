# Active Command Reference Scan 3735

Issue: #3735
Parent sprint: #3732
Captured: 2026-06-16
Status: generated_scan_report

## Summary

This report implements the `#3628` active-bundle scan gate for the
toolkit-simplification sprint. It scans the current repo surfaces for
legacy command-family references and classifies matching command-family
hits as `active`, `historical`, or `unknown` using the path classes
defined for shim-cut review.

Deletion or fail-closed shim cuts remain blocked while any relevant
`active` or `unknown` references remain unrouted.

## Generation Command

```bash
python3 adl/tools/generate_active_command_reference_scan.py
python3 adl/tools/generate_active_command_reference_scan.py --check
```

## Scan Inputs

- `AGENTS.md`
- `docs/templates/`
- `adl/tools/skills/`
- `docs/planning/`
- `docs/milestones/v0.91.5/`
- `.adl/v0.91.5/tasks/`
- `.adl/v0.91.5/bodies/`
- `.adl/cards/`
- `adl/tools/`

## Unique Evidence Totals

- `active`: 31
- `historical`: 20
- `unknown`: 76

## Command Family Hit Totals

| Command family | Active | Historical | Unknown |
| --- | --- | --- | --- |
| direct `adl pr ...` issue-mode commands | 9 | 0 | 7 |
| `adl pr run <adl.yaml>` runtime-through-PR | 0 | 0 | 5 |
| `adl tooling prompt-template ...` | 1 | 1 | 3 |
| legacy `adl tooling ...` helper/review commands | 6 | 12 | 14 |
| legacy umbrella runtime forms | 5 | 6 | 19 |
| `adl/tools/codex_pr.sh` | 8 | 1 | 4 |
| unapproved helper binaries | 2 | 0 | 33 |
| `adl-csdlc issue run <issue>` | 2 | 0 | 6 |

## Findings

| Command reference | Path | Line | Class | Required action before deletion | Preferred owner | Evidence excerpt |
| --- | --- | --- | --- | --- | --- | --- |
| `adl tooling prompt-template ...` | `adl/tools/test_cli_owner_command_guidance.sh` | 22 | `active` | migrate if active; preserve if historical; route if unknown | `adl-csdlc tooling prompt-template ...` | `adl tooling prompt-template"` |
| `adl-csdlc issue run <issue>` | `adl/tools/test_cli_wrapper_migration_contract.sh` | 100 | `active` | block if active until wrapper migration explicitly changes public truth | `adl/tools/pr.sh run <issue>` | `adl-csdlc issue run[[:space:]]+<issue>\|(^\|[^[:alnum:]_-])adl-csdlc issue run[[:space:]]+[0-9]+' \` |
| `adl-csdlc issue run <issue>` | `adl/tools/test_cli_wrapper_migration_contract.sh` | 104 | `active` | block if active until wrapper migration explicitly changes public truth | `adl/tools/pr.sh run <issue>` | `adl-csdlc issue run as primary before wrapper migration" >&2` |
| `adl/tools/codex_pr.sh` | `adl/tools/batched_checks.sh` | 35 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh"` |
| `adl/tools/codex_pr.sh` | `adl/tools/codex_pr.sh` | 19 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh <input-card> --paths "<p1,p2,...>" [--mode full-auto\|auto-edit\|suggest\|help] [--slug <slug>] [--pr` |
| `adl/tools/codex_pr.sh` | `adl/tools/demo_v0891_quality_gate.sh` | 29 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh --help"),"` |
| `adl/tools/codex_pr.sh` | `adl/tools/demo_v089_quality_gate.sh` | 29 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh --help"),"` |
| `adl/tools/codex_pr.sh` | `adl/tools/demo_v0901_quality_gate.sh` | 49 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh --help"),"` |
| `adl/tools/codex_pr.sh` | `adl/tools/test_batched_checks_no_codexpr_usage_banner.sh` | 6 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh"` |
| `adl/tools/codex_pr.sh` | `adl/tools/test_batched_checks_no_codexpr_usage_banner.sh` | 15 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh"` |
| `adl/tools/codex_pr.sh` | `adl/tools/test_batched_checks_no_codexpr_usage_banner.sh` | 20 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh" "$repo/adl/tools/codexw.sh"` |
| direct `adl pr ...` issue-mode commands | `adl/tools/test_cli_owner_command_guidance.sh` | 15 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr create"` |
| direct `adl pr ...` issue-mode commands | `adl/tools/test_cli_owner_command_guidance.sh` | 16 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr init"` |
| direct `adl pr ...` issue-mode commands | `adl/tools/test_cli_owner_command_guidance.sh` | 17 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr doctor"` |
| direct `adl pr ...` issue-mode commands | `adl/tools/test_cli_owner_command_guidance.sh` | 18 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr ready"` |
| direct `adl pr ...` issue-mode commands | `adl/tools/test_cli_owner_command_guidance.sh` | 19 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr preflight"` |
| direct `adl pr ...` issue-mode commands | `adl/tools/test_cli_owner_command_guidance.sh` | 20 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr run"` |
| direct `adl pr ...` issue-mode commands | `adl/tools/test_cli_owner_command_guidance.sh` | 21 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr finish"` |
| direct `adl pr ...` issue-mode commands | `adl/tools/test_cli_wrapper_migration_contract.sh` | 93 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr run [^\`[:space:]]+\.adl\.ya?ml' \` |
| direct `adl pr ...` issue-mode commands | `adl/tools/test_pr_run_ambiguity_policy.sh` | 94 | `active` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr run [^\`[:space:]]+\.adl\.ya?ml' \` |
| legacy `adl tooling ...` helper/review commands | `adl/tools/README.md` | 133 | `active` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling card-prompt --issue <issue_num> --out /tmp/prompt.txt` |
| legacy `adl tooling ...` helper/review commands | `adl/tools/test_cli_owner_command_guidance.sh` | 23 | `active` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling code-review"` |
| legacy `adl tooling ...` helper/review commands | `adl/tools/test_cli_owner_command_guidance.sh` | 24 | `active` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling review-card-surface"` |
| legacy `adl tooling ...` helper/review commands | `adl/tools/test_cli_owner_command_guidance.sh` | 25 | `active` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling review-runtime-surface"` |
| legacy `adl tooling ...` helper/review commands | `adl/tools/test_cli_owner_command_guidance.sh` | 26 | `active` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling verify-review-output-provenance"` |
| legacy `adl tooling ...` helper/review commands | `adl/tools/test_cli_owner_command_guidance.sh` | 27 | `active` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling verify-repo-review-contract"` |
| legacy umbrella runtime forms | `adl/tools/demo_one_command.sh` | 18 | `active` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl demo\`` |
| legacy umbrella runtime forms | `adl/tools/demo_v0891_wp13_demo_integration.sh` | 33 | `active` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json",` |
| legacy umbrella runtime forms | `adl/tools/demo_v0891_wp13_demo_integration.sh` | 34 | `active` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl identity demo-proof-entry-points --out .adl/state/demo_proof_entry_points_v1.json",` |
| legacy umbrella runtime forms | `adl/tools/run_v0915_openrouter_matrix.py` | 578 | `active` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl provider setup openrouter\` into the tracked packet directory.",` |
| legacy umbrella runtime forms | `adl/tools/run_v0915_openrouter_matrix.py` | 597 | `active` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl provider setup openrouter\` and the generated tracked \`provider_setup/\` bundle.",` |
| unapproved helper binaries | `adl/tools/demo_v0891_wp13_demo_integration.sh` | 33 | `active` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json",` |
| unapproved helper binaries | `adl/tools/demo_v0891_wp13_demo_integration.sh` | 34 | `active` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl identity demo-proof-entry-points --out .adl/state/demo_proof_entry_points_v1.json",` |
| `adl tooling prompt-template ...` | `docs/milestones/v0.91.5/review/tooling_adoption/CSDLC_SMALL_BINARIES_PROOF_3832.md` | 87 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-csdlc tooling prompt-template ...` | `adl tooling prompt-template\`` |
| `adl/tools/codex_pr.sh` | `docs/milestones/v0.91.5/review/SHELL_WRAPPER_INVENTORY_3713.tsv` | 23 | `historical` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh	scheduled_for_removal	Legacy operator convenience wrapper; replacement is repo-native pr/workflow` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/review/CHECKLIST_MINI_SPRINT_FUTURE_SESSION_RUNBOOK_3742.md` | 40 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling markdown-ast-edit replace-section\`.` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/review/INTEGRATED_C_SDLC_TIMING_PROOF_3716.md` | 61 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling markdown-ast-edit replace-section\` and focused Rust tests for the new substrate. \|` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/review/MARKDOWN_AST_EDITING_SUBSTRATE_3715.md` | 10 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling markdown-ast-edit replace-section \` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/review/NATIVE_RELEASE_WATCHER_GITHUB_SUPPORT_3718.md` | 11 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling github-release\` with native octocrab-backed operations for:` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/review/NATIVE_RELEASE_WATCHER_GITHUB_SUPPORT_3718.md` | 18 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling github-release\` command so the shell workflow proves delegation without touching real GitHub release state.` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/review/OCTOCRAB_REFACTOR_TEMPLATE_AST_INTEGRATION_CHECKLIST_2026-06-14.md` | 105 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling markdown-ast-edit replace-section\` and the \`#3715\` proof packet.` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/cards/sor.md` | 30 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling public-prompt-packet export\`. The command copies the five lifecycle` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/manifest.json` | 44 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling public-prompt-packet export",` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3473-v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/manifest.json` | 44 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling public-prompt-packet export",` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3562-v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/manifest.json` | 44 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling public-prompt-packet export",` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/review/tooling_adoption/CSDLC_SMALL_BINARIES_PROOF_3832.md` | 85 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling validate-structured-prompt\`` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/review/tooling_adoption/CSDLC_SMALL_BINARIES_PROOF_3832.md` | 86 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling lint-prompt-spec\`` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/review/logging_observability/HEARTBEAT_TIMEOUT_PROGRESS_PROOF_3708.md` | 34 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl agent run\` / \`tick\` / \`status\`` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/review/logging_observability/HEARTBEAT_TIMEOUT_PROGRESS_PROOF_3708.md` | 92 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl agent run\` emits operator-facing heartbeat events while the process is` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/review/native_deepseek_provider/DEEPSEEK_NATIVE_PROVIDER_PROOF_3549.md` | 22 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl provider setup deepseek\` now emits \`type: "deepseek"\` instead of the` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/review/native_deepseek_provider/DEEPSEEK_NATIVE_PROVIDER_PROOF_3549.md` | 145 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl provider setup deepseek --force` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md` | 15 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl provider setup openrouter\` into the tracked packet directory.` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md` | 39 | `historical` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl provider setup openrouter\` and the generated tracked \`provider_setup/\` bundle.` |
| `adl pr run <adl.yaml>` runtime-through-PR | `docs/milestones/v0.91.5/CLI_ACTIVE_BUNDLE_SCAN_GATE_3628.md` | 75 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime run <adl.yaml> ...` | `adl pr run <adl.yaml>\` runtime-through-PR \| \`adl-runtime run <adl.yaml>\` \| Prove no active runtime/demo/proof packet sti` |
| `adl pr run <adl.yaml>` runtime-through-PR | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 56 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime run <adl.yaml> ...` | `adl pr run <adl.yaml> ...\` runtime YAML mode \| \`adl/src/cli/pr_cmd.rs\` and runtime execution path \| \`adl-runtime run <ad` |
| `adl pr run <adl.yaml>` runtime-through-PR | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 110 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime run <adl.yaml> ...` | `adl pr run <adl.yaml>\` to \`adl-runtime run <adl.yaml>\`.` |
| `adl pr run <adl.yaml>` runtime-through-PR | `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md` | 49 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime run <adl.yaml> ...` | `adl pr run <adl.yaml>\` runtime-through-PR \| Deprecated ambiguity path. \| \`adl-runtime run <adl.yaml> ...\` \| Keep only as` |
| `adl pr run <adl.yaml>` runtime-through-PR | `docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md` | 90 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime run <adl.yaml> ...` | `adl pr run <adl.yaml>\` runtime-through-PR commands;` |
| `adl tooling prompt-template ...` | `docs/milestones/v0.91.5/CLI_ACTIVE_BUNDLE_SCAN_GATE_3628.md` | 76 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-csdlc tooling prompt-template ...` | `adl tooling prompt-template ...\` \| \`adl-csdlc tooling prompt-template ...\` \| Prove live docs, skills, and generated-card` |
| `adl tooling prompt-template ...` | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 58 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-csdlc tooling prompt-template ...` | `adl tooling prompt-template ...\` \| \`adl/src/cli/tooling_cmd/prompt_template.rs\` \| \`adl-csdlc prompt-template ...\` \| Add` |
| `adl tooling prompt-template ...` | `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md` | 50 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-csdlc tooling prompt-template ...` | `adl tooling prompt-template ...\` \| Legacy umbrella tooling form. \| \`adl-csdlc tooling prompt-template ...\` \| Keep shim.` |
| `adl-csdlc issue run <issue>` | `docs/milestones/v0.91.5/CLI_ACTIVE_BUNDLE_SCAN_GATE_3628.md` | 145 | `unknown` | block if active until wrapper migration explicitly changes public truth | `adl/tools/pr.sh run <issue>` | `adl-csdlc issue run\|adl-crypto\|adl-godel\|adl-identity" AGENTS.md docs/templates/prompts adl/tools/skills docs/milestones` |
| `adl-csdlc issue run <issue>` | `docs/milestones/v0.91.5/CLI_ACTIVE_BUNDLE_SCAN_GATE_3628.md` | 154 | `unknown` | block if active until wrapper migration explicitly changes public truth | `adl/tools/pr.sh run <issue>` | `adl-csdlc issue run <issue>\` as the public` |
| `adl-csdlc issue run <issue>` | `docs/milestones/v0.91.5/CLI_OWNER_COMMAND_GUIDANCE_AUDIT_3611.md` | 61 | `unknown` | block if active until wrapper migration explicitly changes public truth | `adl/tools/pr.sh run <issue>` | `adl-csdlc issue run <issue>\` as the primary agent-facing issue` |
| `adl-csdlc issue run <issue>` | `docs/milestones/v0.91.5/CLI_REFACTOR_MINI_SPRINT_REVIEW_3600.md` | 215 | `unknown` | block if active until wrapper migration explicitly changes public truth | `adl/tools/pr.sh run <issue>` | `adl-csdlc issue run\`.` |
| `adl-csdlc issue run <issue>` | `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md` | 156 | `unknown` | block if active until wrapper migration explicitly changes public truth | `adl/tools/pr.sh run <issue>` | `adl-csdlc issue run <issue>\` is the public` |
| `adl-csdlc issue run <issue>` | `docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md` | 91 | `unknown` | block if active until wrapper migration explicitly changes public truth | `adl/tools/pr.sh run <issue>` | `adl-csdlc issue run <issue>\` as the primary agent-facing issue-binding` |
| `adl/tools/codex_pr.sh` | `docs/milestones/v0.91.5/CLI_ACTIVE_BUNDLE_SCAN_GATE_3628.md` | 79 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh\` \| \`adl/tools/pr.sh\` or future wrapper internals \| Prove no active operator workflow or closeout p` |
| `adl/tools/codex_pr.sh` | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 30 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh\`` |
| `adl/tools/codex_pr.sh` | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 75 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh ...\` \| \`adl/tools/codex_pr.sh\` \| \`adl-csdlc\` wrapper family, later \| Keep as compatibility wrapper` |
| `adl/tools/codex_pr.sh` | `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md` | 54 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl/tools/codex_pr.sh ...\` \| Legacy compatibility wrapper. \| \`adl/tools/pr.sh ...\` now; later \`adl-csdlc\` internals if p` |
| direct `adl pr ...` issue-mode commands | `docs/milestones/v0.91.5/CLI_ACTIVE_BUNDLE_SCAN_GATE_3628.md` | 75 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr run <adl.yaml>\` runtime-through-PR \| \`adl-runtime run <adl.yaml>\` \| Prove no active runtime/demo/proof packet sti` |
| direct `adl pr ...` issue-mode commands | `docs/milestones/v0.91.5/CLI_ACTIVE_BUNDLE_SCAN_GATE_3628.md` | 145 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr run .*\\.ya?ml\|adl-csdlc issue run\|adl-crypto\|adl-godel\|adl-identity" AGENTS.md docs/templates/prompts adl/tools/` |
| direct `adl pr ...` issue-mode commands | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 55 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr create/init/run/doctor/ready/preflight/finish/closeout\` issue mode \| \`adl/src/cli/pr_cmd.rs\`, \`adl/src/cli/pr_cmd` |
| direct `adl pr ...` issue-mode commands | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 56 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr run <adl.yaml> ...\` runtime YAML mode \| \`adl/src/cli/pr_cmd.rs\` and runtime execution path \| \`adl-runtime run <ad` |
| direct `adl pr ...` issue-mode commands | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 110 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr run <adl.yaml>\` to \`adl-runtime run <adl.yaml>\`.` |
| direct `adl pr ...` issue-mode commands | `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md` | 49 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr run <adl.yaml>\` runtime-through-PR \| Deprecated ambiguity path. \| \`adl-runtime run <adl.yaml> ...\` \| Keep only as` |
| direct `adl pr ...` issue-mode commands | `docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md` | 90 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl/tools/pr.sh ...` | `adl pr run <adl.yaml>\` runtime-through-PR commands;` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 59 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling csdlc-prompt-editor\` \| \`adl/src/cli/tooling_cmd.rs\` \| \`adl-csdlc prompt-editor ...\` \| Keep shim. \| Editor mo` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 60 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling generate-wp-issue-wave\` \| \`adl/src/cli/tooling_cmd/wp_issue_wave.rs\` \| \`adl-csdlc issue-wave generate ...\` \|` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 61 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling lint-prompt-spec\`, \`validate-structured-prompt\`, \`card-prompt\` \| \`adl/src/cli/tooling_cmd/structured_prompt.` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 62 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling code-review\` \| \`adl/src/cli/tooling_cmd/code_review.rs\` \| \`adl-review code-review ...\` \| Keep shim until #35` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 63 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling review-card-surface\`, \`review-runtime-surface\`, \`verify-review-output-provenance\`, \`verify-repo-review-contr` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/CLI_REVIEW_COMPATIBILITY_3599.md` | 24 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling code-review\`` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/CLI_REVIEW_COMPATIBILITY_3599.md` | 25 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling review-card-surface\`` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/CLI_REVIEW_COMPATIBILITY_3599.md` | 26 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling review-runtime-surface\`` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/CLI_REVIEW_COMPATIBILITY_3599.md` | 27 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling verify-review-output-provenance\`` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/CLI_REVIEW_COMPATIBILITY_3599.md` | 28 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling verify-repo-review-contract\`` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md` | 51 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling csdlc-prompt-editor\` and C-SDLC card tooling \| Legacy umbrella tooling form. \| \`adl-csdlc ...\` owner family,` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/LOCAL_ADL_STATE_DISPOSITION_3473.md` | 63 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling public-prompt-packet export\`. \| Durable C-SDLC truth should be tracked under milestone evidence, not hidden` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md` | 55 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling public-prompt-packet export \` |
| legacy `adl tooling ...` helper/review commands | `docs/milestones/v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md` | 85 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-review ...` | `adl tooling public-prompt-packet validate \` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/CLI_ACTIVE_BUNDLE_SCAN_GATE_3628.md` | 78 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl demo\`, \`adl provider\`, \`adl agent\`, \`adl godel\`, \`adl identity\`, \`adl runtime-v2\` \| \`adl-runtime ...\` \| Prove v0.92` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 64 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl demo <name> ...\` \| \`adl/src/cli/demo_cmd.rs\` \| \`adl-runtime demo <name> ...\` \| Keep shim. \| Demo default run behavio` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 65 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl runtime-v2 ...\` \| \`adl/src/cli/runtime_v2_cmd.rs\` \| \`adl-runtime runtime-v2 ...\` or \`adl-runtime proof ...\` \| Keep s` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 66 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl provider setup <family>\` \| \`adl/src/cli/provider_cmd.rs\` \| \`adl-runtime provider setup <family>\` \| Keep shim. \| Supp` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 67 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl agent tick/run/status/inspect/stop\` \| \`adl/src/cli/agent_cmd.rs\` \| \`adl-runtime agent ...\` \| Keep shim. \| Lease beha` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 68 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl instrument ...\` \| \`adl/src/cli/commands.rs\` \| \`adl-runtime instrument ...\` \| Keep shim. \| Graph JSON/DOT, replay, re` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 69 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl learn export ...\` \| \`adl/src/cli/commands.rs\` \| \`adl-runtime learn export ...\` \| Keep shim. \| JSONL, bundle-v1, trac` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 70 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl keygen\`, \`adl sign\`, \`adl verify\` \| \`adl/src/cli/commands.rs\`, \`adl/src/signing.rs\` \| \`adl-runtime keygen/sign/verif` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 71 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl identity ...\` \| \`adl/src/cli/identity_cmd.rs\` \| \`adl-runtime identity ...\` for v0.91.5; possible future identity bin` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 72 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl godel run/inspect/evaluate/affect-slice\` \| \`adl/src/cli/godel_cmd.rs\` \| \`adl-runtime godel ...\` for v0.91.5; possibl` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 74 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl artifact validate-control-path\` \| \`adl/src/cli/artifact_cmd.rs\` \| \`adl-runtime artifact validate-control-path\` \| Kee` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md` | 53 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl demo\`, \`adl runtime-v2\`, \`adl provider\`, \`adl agent\`, \`adl instrument\`, \`adl learn\`, \`adl keygen/sign/verify\`, \`adl` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/HEARTBEAT_TIMEOUT_PROGRESS_POLICY_3708.md` | 51 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl agent run\` should emit` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/HEARTBEAT_TIMEOUT_PROGRESS_POLICY_3708.md` | 98 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl agent run\`, \`adl agent tick\`, and \`adl agent status\` now emit` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 60 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl identity ...\` wording. A new binary would fossilize another command family before public prompt/card migration catch` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 73 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl keygen\`, \`adl sign\`, and \`adl verify\` should stay under` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 122 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl identity ...\`` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 153 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl keygen\`, \`adl sign\`, \`adl verify\`,` |
| legacy umbrella runtime forms | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 154 | `unknown` | migrate if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl godel\`, or \`adl identity\`, they should be handled through the compatibility` |
| unapproved helper binaries | `docs/milestones/v0.91.5/CLI_ACTIVE_BUNDLE_SCAN_GATE_3628.md` | 78 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl godel\`, \`adl identity\`, \`adl runtime-v2\` \| \`adl-runtime ...\` \| Prove v0.92 activation docs and active runtime proof` |
| unapproved helper binaries | `docs/milestones/v0.91.5/CLI_ACTIVE_BUNDLE_SCAN_GATE_3628.md` | 145 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\|adl-godel\|adl-identity" AGENTS.md docs/templates/prompts adl/tools/skills docs/milestones/v0.91.5\`` |
| unapproved helper binaries | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 70 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\` deferred \| Keep shim; do not introduce \`adl-crypto\` in this mini-sprint. \| Key path outputs, signature verif` |
| unapproved helper binaries | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 71 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl identity ...\` \| \`adl/src/cli/identity_cmd.rs\` \| \`adl-runtime identity ...\` for v0.91.5; possible future identity bin` |
| unapproved helper binaries | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 72 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl godel run/inspect/evaluate/affect-slice\` \| \`adl/src/cli/godel_cmd.rs\` \| \`adl-runtime godel ...\` for v0.91.5; possibl` |
| unapproved helper binaries | `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | 116 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\`, \`adl-godel\`, or \`adl-identity\`` |
| unapproved helper binaries | `docs/milestones/v0.91.5/CLI_REFACTOR_MINI_SPRINT_REVIEW_3600.md` | 154 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\`, \`adl-godel\`, and identity splits as deferred work.` |
| unapproved helper binaries | `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md` | 53 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl godel\`, \`adl identity\`, \`adl artifact\` \| Legacy umbrella runtime forms. \| \`adl-runtime ...\` \| Keep shims. Warnings o` |
| unapproved helper binaries | `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md` | 55 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\`, \`adl-godel\`, \`adl-identity\` \| Not implemented and not approved by #3614. \| Keep under \`adl-runtime\` for v0.` |
| unapproved helper binaries | `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md` | 158 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\`, \`adl-godel\`, or \`adl-identity\` as` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 11 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\`, \`adl-godel\`, and \`adl-identity\`.` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 19 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\`, \`adl-godel\`, or \`adl-identity\` in v0.91.5 Sprint` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 58 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\` \| \`adl-runtime keygen/sign/verify\` \| Defer. Keep under \`adl-runtime\`. \| The command surface is small, but se` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 59 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-godel\` \| \`adl-runtime godel ...\` \| Defer. Keep under \`adl-runtime\`. \| Gödel mechanics are v0.92 activation-facing an` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 60 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-identity\` \| \`adl-runtime identity ...\` \| Defer. Keep under \`adl-runtime\`. \| Identity has many proof hooks and existi` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 64 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\`` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 75 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\` yet.` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 79 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\` is attractive as a future security boundary, but premature as a` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 90 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-godel\`` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 114 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-identity\`` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 122 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl identity ...\`` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 143 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\`` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 144 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-godel\`` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 145 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-identity\`` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 154 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl godel\`, or \`adl identity\`, they should be handled through the compatibility` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 186 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\|adl-godel\|adl-identity" AGENTS.md docs/templates/prompts docs/milestones/v0.91.5 adl/src\` \| Confirm helper bi` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 196 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\`, \`adl-godel\`, and \`adl-identity\` appear in v0.91.5 planning and` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 198 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-godel-*\` also appears as internal temp-directory prefixes in Gödel tests` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 200 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\`,` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 201 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-godel\`, or \`adl-identity\` as implemented commands.` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 205 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-crypto\`.` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 206 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-godel\`.` |
| unapproved helper binaries | `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md` | 207 | `unknown` | forbid if active; preserve if historical; route if unknown | `adl-runtime ...` | `adl-identity\`.` |

## Deletion Recommendation

- Do not delete or fail-close any scanned command family while `active` findings remain.
- Route every `unknown` finding through a bounded follow-on before compatibility removal.
- Treat `historical` findings as readability evidence, not as executable dependencies.

## Known Classification Rules

- `AGENTS.md`, prompt templates, skills, planning docs, open issue bodies, and task bundles classify as `active`.
- `adl/tools/` scripts and active validation/demo helpers classify as `active` unless a later issue carves out an explicit historical-fixture rule.
- `docs/milestones/v0.91.5/review/` and older closed-milestone evidence classify as `historical`.
- `docs/milestones/v0.91.5/` outside review packets and `.adl/cards/` classify as `unknown` until a later issue narrows them further.
- Unique evidence totals count one row per path/line/class; command-family hit totals may overlap when one source line names multiple legacy families.

## Non-Claims

- This issue does not delete any compatibility shim.
- This issue does not rewrite historical records solely to remove old command strings.
- This issue does not claim every `unknown` reference is unsafe; it only routes them for future review.

# ADL Demos

`demos/` is the canonical user-facing entrypoint for finding and running ADL demos from repository root.

## Start Here

If you want the fastest successful demo run:

```bash
cargo run -q --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-3-fork-join-seq-run.adl.yaml --print-plan
```

If you want the `v0.86` bounded cognitive review surface:

```bash
./adl/tools/demo_v086_review_surface.sh
```

If you want the historical `v0.8` flagship demo surface:

```bash
cargo run --manifest-path demos/transpiler_demo/Cargo.toml --quiet
```

If you want the current `v0.89.1` quality-gate proof package:

```bash
bash adl/tools/demo_v0891_quality_gate.sh
```

If you want the current `v0.89.1` integration demo package:

```bash
bash adl/tools/demo_v0891_wp13_demo_integration.sh
```

If you want the local Codex CLI + Ollama operational-skills demo:

```bash
bash adl/tools/demo_codex_ollama_operational_skills.sh --dry-run
```

If you want the bounded Claude + ChatGPT tea discussion demo:

```bash
bash adl/tools/demo_v0871_multi_agent_discussion.sh
```

If you want the bounded five-agent Hey Jude MIDI demo:

```bash
bash adl/tools/demo_v0891_five_agent_hey_jude.sh
```

If you want the bounded arXiv manuscript workflow demo:

```bash
bash adl/tools/demo_v0891_arxiv_manuscript_workflow.sh
```

If you want the v0.90.2 arXiv-writer field-test packet:

```bash
bash adl/tools/test_demo_v0902_arxiv_writer_field_test.sh
```

If you want the bounded multi-agent repo code review demo:

```bash
bash adl/tools/demo_v089_multi_agent_repo_code_review.sh
```

If you want the stricter v0.90.2 multi-agent repo-review proof packet:

```bash
bash adl/tools/demo_v0902_multi_agent_repo_review_proof.sh
```

If you want the v0.90.2 Paper Sonata manuscript review/revision proof packet:

```bash
bash adl/tools/demo_v0902_paper_sonata_expansion.sh
```

If you want the CodeBuddy multi-agent review showcase packet:

```bash
bash adl/tools/demo_v090_codebuddy_review_showcase.sh
```

If you want the ADL architecture document generation proof packet:

```bash
bash adl/tools/demo_v090_architecture_document_generation.sh
```

If you want the fixture-backed CSM Observatory static console:

```text
demos/v0.90.1/csm_observatory_static_console.html
```

If you want the fixture-backed CSM Observatory operator report:

```bash
bash adl/tools/test_demo_v0901_csm_observatory_operator_report.sh
```

If you want the command-driven CSM Observatory demo bundle:

```bash
bash adl/tools/demo_v0901_csm_observatory.sh
```

If you want the integrated Runtime v2 foundation proof demo:

```bash
cargo run --manifest-path adl/Cargo.toml -- demo demo-l-v0901-runtime-v2-foundation --run --trace --out artifacts/v0901 --no-open
```

## Demo Categories

- Runtime workflow demos live in `adl/examples/`.
  These are the actual ADL files you can pass to the CLI today.
- Milestone runbooks and reviewer docs live here in `demos/`.
- Spec-only example artifacts live in `adl-spec/examples/` and are not the main starting point for runnable demos.

## Demo Taxonomy

Use these categories consistently:

- Ordinary demos:
  bounded runnable surfaces intended for normal demo sweeps, milestone proof rows, or public-facing walkthroughs.
- Reviewer packages:
  heavier integrated review surfaces that bundle multiple proof rows for milestone or reviewer use.
- Release and quality proof packages:
  heavyweight validation or release-tail surfaces that are useful for review and ceremony work, but should not be treated as ordinary quick demos.

Current heavyweight proof-package examples:
- `bash adl/tools/demo_v0871_suite.sh`
- `bash adl/tools/demo_v0871_quality_gate.sh`
- `bash adl/tools/demo_v0871_release_review_package.sh`
- `bash adl/tools/demo_v088_review_surface.sh`
- `bash adl/tools/demo_v0891_quality_gate.sh`
- `bash adl/tools/demo_v0891_wp13_demo_integration.sh`

When planning a demo sweep, do not assume every proof-package command above belongs in the ordinary demo lane.

## Recommended Paths

### v0.87 demo program

- `v0.87/v087_demo_program.md`
- `../docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`

### v0.87.1 Codex CLI + Ollama operational-skills demo

- `v0.87.1/codex_ollama_operational_skills_demo.md`
- `v0.87.1/claude_chatgpt_multi_agent_discussion_demo.md`

Use `bash adl/tools/demo_v0871_suite.sh` for the canonical WP-13 demo-suite
entrypoint. It runs the currently implemented `v0.87.1` proof surfaces and
writes a suite manifest at `artifacts/v0871/suite/demo_manifest.json`.

Treat that suite as the current reviewer starting point for the milestone.
It is a heavyweight reviewer proof package, not an ordinary quick demo.
The live-provider D13L companion proof remains explicit and credential-gated
rather than part of the default suite.

Use `v0.87.1/codex_ollama_operational_skills_demo.md` for a bounded demo that
installs the tracked skills into a demo-local `CODEX_HOME`, points Codex CLI at
an Ollama-backed model on the default local host or a configured remote host,
and runs the editor skills against a prepared local bundle fixture.

Use `bash adl/tools/demo_codex_ollama_operational_skills.sh --dry-run` to
prepare the prompt, workspace, and manifest without invoking a local model.

Use `v0.87.1/claude_chatgpt_multi_agent_discussion_demo.md` for a bounded
multi-agent runtime demo that keeps Claude and ChatGPT as two explicit named
agents in a longer act-structured tea discussion and emits a transcript plus runtime proof
artifacts.

Use `v0.89/gemini_in_the_loop_demo.md` for a bounded provider-harmony demo that
packages one review packet for Gemini, validates Gemini's structured response,
and records a reviewer-facing findings artifact plus runtime proof.

### v0.89 bounded provider-participation demos

- `v0.89/v089_demo_program.md`
- `v0.89/deep_agents_comparative_wave_demo.md`
- `v0.89/medium_article_writing_demo.md`
- `v0.89/gemini_provider_harmony_and_economics_demo.md`
- `v0.89/gemma4_issue_clerk_demo.md`
- `v0.89/paper_sonata_demo_refresh.md`
- `v0.89/multi_agent_repo_code_review_demo.md`

Use `v0.89/v089_demo_program.md` for the canonical `v0.89` proof-row entrypoint
guide. It ties D1 through D7 to one reviewer flow and points at the bounded
proof-entrypoint manifest produced by:

```bash
bash adl/tools/demo_v089_proof_entrypoints.sh
```

That command writes a lightweight review packet at:
- `artifacts/v089/proof_entrypoints/demo_manifest.json`
- `artifacts/v089/proof_entrypoints/README.md`
- `artifacts/v089/proof_entrypoints/index.txt`

Use `bash adl/tools/demo_v089_review_surface.sh` when you want the integrated
`v0.89` reviewer package instead of the lighter row-entry map. It emits:
- `artifacts/v089/review_surface/demo_manifest.json`
- `artifacts/v089/review_surface/claim_matrix.md`
- `artifacts/v089/review_surface/proof_entrypoints/demo_manifest.json`

Use `v0.89/deep_agents_comparative_wave_demo.md` for a richer comparative demo
where ChatGPT, Claude, and Gemini contribute bounded positions and ADL emits a
findings-first synthesis package instead of a flat proof row.

Use `v0.89/medium_article_writing_demo.md` for a bounded publication-oriented
demo where ADL turns one launch-essay brief into a reviewer-friendly article
packet with outline, draft, editorial notes, and runtime proof.

Use `v0.89/gemini_provider_harmony_and_economics_demo.md` for the stronger
Gemini follow-on where ADL not only hosts Gemini in a bounded workflow, but
also records why Gemini was selected through provider-fit and cost-class
reasoning artifacts.

Use `v0.89/gemma4_issue_clerk_demo.md` for a bounded operational-participation
demo where an Ollama-hosted Gemma-family model proposes one issue-init payload,
ADL validates it, and the final result is either accepted deterministically or
rejected truthfully.

Use `v0.89/paper_sonata_demo_refresh.md` for the refreshed reviewer-facing
Paper Sonata package. It keeps the bounded flagship workflow, but adds a packet
manifest, claim matrix, revision requests, and a clearer reviewer brief.

### v0.89.1 adversarial/runtime and publication demos

- `v0.89.1/five_agent_hey_jude_midi_demo.md`
- `v0.89.1/arxiv_manuscript_workflow_demo.md`
- `v0.89.1/gemini_provider_harmony_roundtable_demo.md`
- `v0.89.1/deep_agents_comparative_wave_follow_on_demo.md`
- `v0.89.1/long_lived_stock_league_demo.md`

Use `bash adl/tools/demo_v0891_quality_gate.sh` for the current release-tail
quality-gate proof package.

Use `bash adl/tools/demo_v0891_wp13_demo_integration.sh` for the integrated
`v0.89.1` demo package.

Use `bash adl/tools/demo_v0891_five_agent_hey_jude.sh` for the flagship bounded
multi-agent music demo.

Use `bash adl/tools/demo_v0891_arxiv_manuscript_workflow.sh` for the bounded
manuscript workflow packet.

Use `v0.89/multi_agent_repo_code_review_demo.md` for a bounded specialist-reviewer
demo where ADL prepares one repo review packet, emits code/security/test/docs
review artifacts, and then writes one synthesized findings-first review.

### v0.90 long-lived agents and CodeBuddy review demos

- `v0.90/long_lived_stock_league_demo.md`
- `v0.90/stock_league_demo_extensions.md`
- `v0.90/codebuddy_multi_agent_review_showcase_demo.md`
- `v0.90/aptitude_atlas_repo_review_aptitude_demo.md`

Use `bash adl/tools/demo_v090_codebuddy_review_showcase.sh` for the
fixture-backed CodeBuddy showcase packet. It emits a product-style review
package with repo scope, specialist reviews, diagram and redaction gates,
test/issue/ADR/fitness follow-through, final report, and demo-operator
classification. It is intentionally classified as `non_proving` until the
staged `review-quality-evaluator` lane from `#2070` lands.

### v0.90.2 review proof demos

- `v0.90.2/multi_agent_repo_review_proof_demo.md`
- `v0.90.2/paper_sonata_expansion_demo.md`

Use `bash adl/tools/demo_v0902_multi_agent_repo_review_proof.sh` for the
stricter multi-agent repo-review proof packet. It is fixture-backed and
deterministic, but it validates a stronger review contract than the original
v0.89 demo: specialist boundaries, findings-first synthesis, explicit
non-findings, residual risk, publication gates, and no merge-approval claims.

Use `bash adl/tools/demo_v0902_paper_sonata_expansion.sh` for the bounded Paper
Sonata manuscript review/revision proof packet. It preserves the delivered
v0.88/v0.89 baseline while adding source/draft/review/revision separation,
addressed revision requests, and an explicit no-submission publication gate.

### v0.90.1 Runtime v2 and CSM Observatory demos

- `v0.90.1/csm_observatory_static_console.md`
- `v0.90.1/csm_observatory_operator_report.md`
- `v0.90.1/csm_observatory_cli_demo.md`

Use `v0.90.1/csm_observatory_static_console.html` for the first read-only CSM
Observatory prototype. It renders the fixture-backed proto-csm-01 visibility
packet into a control-room surface with manifold header, citizen constellation,
kernel pulse, Freedom Gate docket, trace ribbon, and read-only operator action
rail.

Use `bash adl/tools/test_demo_v0901_csm_observatory_operator_report.sh` to
validate the packet-to-report proof surface. The generated Markdown report is a
compact reviewer artifact for manifold status, citizen state, invariant risk,
Freedom Gate decisions, trace tail, operator-action boundaries, and caveats.

Use `bash adl/tools/demo_v0901_csm_observatory.sh` for the first command-driven
Observatory demo bundle. It emits the packet copy, operator report, static
console reference, and demo manifest from one ADL CLI command.

### v0.90.2 publication field-test demos

- `v0.90.2/arxiv_writer_field_test_demo.md`

Use `bash adl/tools/test_demo_v0902_arxiv_writer_field_test.sh` for the
bounded arXiv-writer field test. It validates the `What Is ADL?` source packet
and manuscript packet, including claim-boundary labels, citation-gap handling,
and the explicit no-submission publication boundary.

Use `v0.87/v087_demo_program.md` for the canonical `v0.87` demo order and bounded
repo-local commands.

Use `bash adl/tools/demo_v087_suite.sh` for the one-command substrate review
path. It populates:
- `artifacts/v087/trace_v1/`
- `artifacts/v087/provider_portability/`
- `artifacts/v087/shared_obsmem/`
- `artifacts/v087/skills/`
- `artifacts/v087/control_plane/`
- `artifacts/v087/reviewer_package/`

### v0.86 demo program

- `../docs/milestones/v0.86/DEMO_MATRIX_v0.86.md`
- `../docs/milestones/v0.86/features/LOCAL_AGENT_DEMOS.md`

Use `../docs/milestones/v0.86/DEMO_MATRIX_v0.86.md` for the canonical milestone review order and proof-surface contract.

Use `./adl/tools/demo_v086_review_surface.sh` for the one-command reviewer entry point. It generates:
- `artifacts/v086/review_surface/demo_manifest.json`
- `artifacts/v086/review_surface/README.txt`
- `artifacts/v086/review_surface/index.txt`

Use `./adl/tools/demo_v086_control_path.sh` for the primary bounded cognitive-path proof surface. It generates:
- `artifacts/v086/control_path/demo-g-v086-control-path/summary.txt`
- `artifacts/v086/control_path/demo-g-v086-control-path/arbitration.json`
- `artifacts/v086/control_path/demo-g-v086-control-path/freedom_gate.json`
- `artifacts/v086/control_path/demo-g-v086-control-path/trace.jsonl`

### v0.85 demo program

- `v0.85/v085_demo_program.md`
- `v0.85/steering_queue_checkpoint_demo.md`
- `v0.85/hitl_editor_review_demo.md`
- `v0.85/godel_hypothesis_engine_demo.md`
- `aee-recovery/README.md`
- `v0.85/affect_godel_vertical_slice_demo.md`

Use `v0.85/v085_demo_program.md` for the canonical milestone review order.

Use `adl/tools/demo_steering_queue_checkpoint.sh` for the bounded
steering/checkpoint/resume proof surface. It emits:
- `.adl/runs/v0-85-hitl-steering-demo/pause_state.json`
- `.adl/runs/v0-85-hitl-steering-demo/run.json`
- `.adl/reports/demo-steering-queue-checkpoint/steer.json`

Use `adl/tools/demo_hitl_editor_review.sh` for the bounded editor/review
workflow proof surface. It emits:
- `.adl/reports/demo-hitl-editor-review/editor_review_demo_manifest.v1.json`

Use `v0.85/godel_hypothesis_engine_demo.md` for the milestone-facing WP-10 review
path. For a one-command review flow, run
`adl/tools/demo_godel_hypothesis_engine.sh`.

### Quickstart runtime demos

- `adl/examples/v0-3-fork-join-seq-run.adl.yaml`
- `adl/examples/v0-3-on-error-retry.adl.yaml`
- `adl/examples/v0-5-primitives-minimal.adl.yaml`

### Historical v0.8 demos

- `rust-transpiler/README.md`
- `v0.8/v0.8-bounded-critical-demos.md`
- `v0.8/godel_failure_hypothesis_experiment.md`
- `v0.85/adaptive_godel_loop_demo.md`
- `v0.85/experiment_prioritization_demo.md`
- `v0.85/cross_workflow_learning_demo.md`
- `v0.85/promotion_eval_loop_demo.md`
- `aee-recovery/README.md`

Use `aee-recovery/README.md` for the bounded Adaptive Execution Engine recovery
demo: failure, explicit retry-budget recommendation, bounded overlay, and
successful recovery with replayable artifacts.

Use `adl/tools/demo_aee_bounded_adaptation.sh` for the one-command v0.85
bounded adaptation demo that emits `learning/aee_decision.json` as the primary
adaptive decision artifact.

Use `v0.8/godel_failure_hypothesis_experiment.md` for the concrete
`adl godel run` / `adl godel inspect` / `adl godel evaluate` review path and the
persisted Gödel schema/runtime artifacts it produces, including the
first-class `godel_hypothesis.v1.json` hypothesis record. For a one-command
review flow, run `adl/tools/demo_godel_hypothesis_engine.sh`.

Use `v0.85/promotion_eval_loop_demo.md` for the bounded WP-14 review path that proves
the prior Gödel artifacts now feed a structured evaluation report and an
explicit promotion decision through the persisted
`godel_eval_report.v1.json` and `godel_promotion_decision.v1.json` artifacts.
For a one-command review flow, run `adl/tools/demo_promotion_eval_loop.sh`.

Use `v0.85/adaptive_godel_loop_demo.md` for the bounded WP-11 policy-learning slice.
It shows a deterministic policy decision and before/after policy comparison
derived from the persisted hypothesis artifact. For a one-command review flow,
run `adl/tools/demo_adaptive_godel_loop.sh`.

Use `v0.85/experiment_prioritization_demo.md` for the bounded WP-12 prioritization
slice. It shows a deterministic ranked experiment output with explicit
confidence values and stable tie-break behavior derived from the hypothesis and
policy artifacts. For a one-command review flow, run
`adl/tools/demo_experiment_prioritization.sh`.

Use `v0.85/cross_workflow_learning_demo.md` for the bounded WP-13 review path that
proves workflow-A prioritization output changes a downstream workflow-B
decision through the persisted `godel_cross_workflow_learning.v1.json`
artifact. For a one-command review flow, run
`adl/tools/demo_cross_workflow_learning.sh`.

Use `v0.85/affect_godel_vertical_slice_demo.md` for the bounded WP-17 slice that
proves an affect transition changes the top Gödel-adjacent strategy ranking
through the persisted `godel_affect_vertical_slice.v1.json` artifact. For a
one-command review flow, run
`adl/tools/demo_affect_godel_vertical_slice.sh`.

### Historical/runtime fixture inventory

- `../adl/examples/README.md`

Use that README when you specifically want the full crate-local example inventory. For user-facing demo discovery, start here instead.

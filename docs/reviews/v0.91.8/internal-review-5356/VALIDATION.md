# Validation

Packet origin revision before repair edits: `9cfc5f3f0d5d8027264e60e82eeec1b664daf9b6`

| Command | Result | Notes |
| --- | --- | --- |
| `ruby .csdlc/prepared/issues/5356/check-dependencies.rb` | pass | Records `dependency_sha` `3d4321e832a8931b5611cf59dbb566462e564836` and landed squash SHA `dc7fd24c5b145bcb9cb28c7d3b9ca7079d7fb653`. |
| `ruby .csdlc/prepared/issues/5356/validate-preparation.rb` | pass | Six cards, six specialist lanes, typed doctor pass, 692 authored nonblank lines. |
| `ruby -ryaml -e 'YAML.safe_load(File.read("docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml"), aliases: true); puts "yaml pass"'` | pass | YAML parses. |
| `python3 docs/milestones/v0.91.8/review/wp15_demo_matrix_5733/validate_v0918_demo_matrix.py` | pass | Demo matrix validator reports `v0918_demo_matrix: PASS`. |
| `rg -n "Active release-tail issue: WP-17|WP-17 \`#5360\` owns the present|WP-17 #5360 now|WP-17 is aligning|Active documentation and release-truth|Pending internal milestone review" docs/milestones/v0.91.8 -S` | pass | No stale current-truth matches after repair. |
| `git diff --check` | pass | No whitespace errors. |
| `csdlc-doctor --repo . --issue 5356` | pass | Phase `implemented`, no findings after typed finalize. |
| `ruby .csdlc/prepared/issues/5356/run-validation-lane.rb code` | pass | Runs `cargo test --locked -p adl-runtime runtime_api_contract_advertises_only_served_routes`; 1 focused Runtime API test passed. |
| `ruby .csdlc/prepared/issues/5356/run-validation-lane.rb docs` | pass | No stale current-truth matches after repair. |
| `ruby .csdlc/prepared/issues/5356/run-validation-lane.rb evidence` | pass | Preparation/review packet validator passed with six cards and six specialist lanes. |

Final exact-head validation was rerun after retained findings were fixed; typed
review/publication state will record the accepted revision for merge readiness.

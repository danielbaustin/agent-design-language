#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ONLY_CHECKS=",${ADL_V0913_PROOF_ONLY_CHECKS:-},"

run_check() {
  local key="$1"
  shift
  if [[ "$ONLY_CHECKS" != ",," && "$ONLY_CHECKS" != *",$key,"* ]]; then
    echo "SKIP $key (filtered)"
    return 0
  fi
  echo "RUN  $key"
  "$@"
}

validate_card_lifecycle_bundle() {
  bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type sip --phase bootstrap \
    --input "$ROOT_DIR/docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sip.md"
  bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type stp \
    --input "$ROOT_DIR/docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/stp.md"
  bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type spp --phase final \
    --input "$ROOT_DIR/docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/spp.md"
  bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type srp --phase final \
    --input "$ROOT_DIR/docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/srp.md"
  bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type sor --phase final \
    --input "$ROOT_DIR/docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sor.md"
}

validate_card_lifecycle_contract() {
  cargo test --manifest-path "$ROOT_DIR/adl/Cargo.toml" tracked_csdlc_card_bundle -- --nocapture
  cargo test --manifest-path "$ROOT_DIR/adl/Cargo.toml" card_lifecycle_accepts_tracked_csdlc_bundle -- --nocapture
}

run_check transition_dag_packet \
  python3 "$ROOT_DIR/adl/tools/validate_transition_dag_packet.py" \
  "$ROOT_DIR/docs/milestones/v0.91.3/review/transition_dag"
run_check transition_dag_contract \
  bash "$ROOT_DIR/adl/tools/test_transition_dag_packet.sh"

run_check transition_manifest_schema \
  cargo test --manifest-path "$ROOT_DIR/adl/Cargo.toml" cognitive_transition_schema -- --nocapture

run_check card_lifecycle_bundle validate_card_lifecycle_bundle

run_check card_lifecycle_contract validate_card_lifecycle_contract

run_check evidence_bundle_packet \
  python3 "$ROOT_DIR/adl/tools/validate_evidence_bundle_packet.py" \
  "$ROOT_DIR/docs/milestones/v0.91.3/review/evidence_bundle"
run_check evidence_bundle_contract \
  bash "$ROOT_DIR/adl/tools/test_evidence_bundle_packet.sh"

run_check merge_readiness_packet \
  python3 "$ROOT_DIR/adl/tools/validate_merge_readiness_packet.py" \
  "$ROOT_DIR/docs/milestones/v0.91.3/review/merge_readiness"
run_check merge_readiness_contract \
  bash "$ROOT_DIR/adl/tools/test_merge_readiness_packet.sh"

run_check obsmem_handoff_packet \
  python3 "$ROOT_DIR/adl/tools/validate_obsmem_handoff_packet.py" \
  "$ROOT_DIR/docs/milestones/v0.91.3/review/obsmem_handoff"
run_check obsmem_handoff_contract \
  bash "$ROOT_DIR/adl/tools/test_obsmem_handoff_packet.sh"

run_check first_proof_readiness_packet \
  python3 "$ROOT_DIR/adl/tools/validate_first_proof_readiness_packet.py" \
  "$ROOT_DIR/docs/milestones/v0.91.3/review/first_proof_readiness"
run_check first_proof_readiness_contract \
  bash "$ROOT_DIR/adl/tools/test_first_proof_readiness_packet.sh"

run_check first_proof_demo_packet \
  python3 "$ROOT_DIR/adl/tools/validate_first_proof_demo_packet.py" \
  "$ROOT_DIR/docs/milestones/v0.91.3/review/first_proof_demo"
run_check first_proof_demo_contract \
  bash "$ROOT_DIR/adl/tools/test_first_proof_demo_packet.sh"

run_check five_minute_html_game_packet \
  python3 "$ROOT_DIR/adl/tools/validate_five_minute_html_game_packet.py" \
  "$ROOT_DIR/docs/milestones/v0.91.3/review/five_minute_html_game"
run_check five_minute_html_game_contract \
  bash "$ROOT_DIR/adl/tools/test_five_minute_html_game_packet.sh"

run_check five_minute_sprint_console_packet \
  python3 "$ROOT_DIR/adl/tools/validate_five_minute_sprint_console_packet.py" \
  "$ROOT_DIR/docs/milestones/v0.91.3/review/five_minute_sprint_console"
run_check five_minute_sprint_console_contract \
  bash "$ROOT_DIR/adl/tools/test_five_minute_sprint_console_packet.sh"

run_check podcast_studio_v2_packet \
  python3 "$ROOT_DIR/adl/tools/validate_podcast_studio_v2_packet.py" \
  "$ROOT_DIR/docs/milestones/v0.91.3/review/podcast_studio_v2" \
  "$ROOT_DIR/demos/v0.91.3/adl_podcast_studio_v2_episode_card.html" \
  "$ROOT_DIR/docs/milestones/v0.91.3/features/PODCAST_STUDIO_V2_DEMO.md"
run_check podcast_studio_v2_contract \
  bash "$ROOT_DIR/adl/tools/test_podcast_studio_v2_packet.sh"

run_check csdlc_demo_proof_contract_packet \
  python3 "$ROOT_DIR/adl/tools/validate_csdlc_demo_proof_contract_packet.py" \
  "$ROOT_DIR/docs/milestones/v0.91.3/review/csdlc_demo_proof_contract"
run_check csdlc_demo_proof_contract_contract \
  bash "$ROOT_DIR/adl/tools/test_csdlc_demo_proof_contract_packet.sh"

run_check quality_gate_doc_surface \
  python3 "$ROOT_DIR/adl/tools/validate_v0913_quality_gate_review_surfaces.py" \
  "$ROOT_DIR" quality_gate_doc
run_check quality_gate_packet_surface \
  python3 "$ROOT_DIR/adl/tools/validate_v0913_quality_gate_review_surfaces.py" \
  "$ROOT_DIR" quality_gate_packet
run_check demo_coverage_surface \
  python3 "$ROOT_DIR/adl/tools/validate_v0913_quality_gate_review_surfaces.py" \
  "$ROOT_DIR" demo_coverage

echo "PASS run_v0913_proof_validation_lane"

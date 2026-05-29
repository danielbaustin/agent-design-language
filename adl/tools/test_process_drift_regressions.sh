#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${ROOT_DIR}"

echo "[process-drift] sprint-conductor helper regressions"
bash adl/tools/test_sprint_conductor_helpers.sh

echo "[process-drift] structured prompt legacy SRP policy wording"
cargo test --manifest-path adl/Cargo.toml structured_prompt_srp_rejects_legacy_review_policy_artifact_type

echo "[process-drift] structured prompt stale SRP review truth"
cargo test --manifest-path adl/Cargo.toml structured_prompt_srp_completed_card_status_requires_final_review_truth

echo "[process-drift] structured prompt stale SOR closeout truth"
cargo test --manifest-path adl/Cargo.toml structured_prompt_sor_completed_card_status_requires_full_closeout_truth

echo "[process-drift] structured prompt absolute host path leakage"
cargo test --manifest-path adl/Cargo.toml structured_prompt_srp_validator_rejects_absolute_host_path_leakage

echo "PASS test_process_drift_regressions"

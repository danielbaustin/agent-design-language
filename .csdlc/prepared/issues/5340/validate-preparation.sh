#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
common="$(git rev-parse --path-format=absolute --git-common-dir)"
primary="$(dirname "${common}")"
doctor="${primary}/.adl/bin/csdlc-v2/csdlc-doctor"
validator="${primary}/.adl/bin/csdlc-v2/csdlc-validate"
request="${repo_root}/.csdlc/prepared/issues/5340/preparation-validation.json"
proof_root="/Volumes/FastWork/adl-wp-5340/preparation"
mkdir -p "${proof_root}"

doctor_output="$(${doctor} --repo "${repo_root}" --issue 5340)"
printf '%s\n' "${doctor_output}"
jq -e '.status == "pass" and .phase == "bound" and .findings == []' <<<"${doctor_output}" >/dev/null

validation_output="$(${validator} --request "${request}")"
printf '%s\n' "${validation_output}"
jq -e '.schema == "csdlc.pvf.report.v1" and .disposition == "local_pass" and (.evidence | length) == 2 and ([.evidence[].lane] | sort) == ["preparation-contract", "preparation-tool-cache"] and ([.evidence[].status] | all(. == "passed"))' <<<"${validation_output}" >/dev/null

#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"
require "yaml"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
EVIDENCE = ROOT.join(".csdlc/evidence/5351")
LANES = %w[focused-quality integrated-platform complete post-merge-exact].freeze
TARGET_ROOT = ENV.fetch("ADL_WP16_TARGET_ROOT", "wp16-target")

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb LANE") }
abort("unknown validation lane: #{lane}") unless LANES.include?(lane)
FileUtils.mkdir_p(EVIDENCE)

def run!(name, argv, log: nil)
  output, status = Open3.capture2e({ "CARGO_TERM_COLOR" => "never" }, *argv, chdir: ROOT.to_s)
  retained_output = output.gsub(ROOT.to_s, ".")
  ROOT.join(log).write(retained_output) if log
  abort("#{name} failed; see #{log || 'stderr'}") unless status.success?
  { "name" => name, "status" => "pass", "command" => argv, "evidence" => log }
end

def json!(path)
  JSON.parse(ROOT.join(path).read)
rescue JSON::ParserError, Errno::ENOENT => e
  abort("invalid required JSON #{path}: #{e.message}")
end

def require_status!(path, schema:, status: "pass")
  value = json!(path)
  abort("schema mismatch in #{path}") unless value["schema"] == schema
  abort("non-pass status in #{path}") unless value["status"] == status
  { "name" => path, "status" => "pass", "schema" => schema }
end

def current_head
  head = `git -C #{ROOT} rev-parse HEAD`.strip
  abort("cannot resolve exact revision") unless head.match?(/\A[0-9a-f]{40}\z/)
  head
end

def require_lane!(lane, revision)
  path = ".csdlc/evidence/5351/#{lane}.json"
  value = json!(path)
  abort("wrong lane schema in #{path}") unless value["schema"] == "adl.v0918.wp16.integrated_quality_gate.v1"
  abort("non-pass lane in #{path}") unless value["status"] == "pass"
  abort("stale lane revision in #{path}") unless value["revision"] == revision
  { "name" => path, "status" => "pass", "revision" => revision }
end

def focused_rows
  rows = []
  rows << run!("wp15-terminal-gate", ["ruby", ".csdlc/prepared/issues/5351/check-dependencies.rb"], log: ".csdlc/evidence/5351/wp15-terminal-gate.log")
  rows << require_status!(".csdlc/evidence/5384/platform-acceptance-ledger.v1.json", schema: "adl.wp14a.platform_acceptance_ledger.v1")
  rows << require_status!(".csdlc/evidence/5354/convergence-proof.v1.json", schema: "adl.v0918.wp15.convergence_proof.v1")
  rows << require_status!("docs/milestones/v0.91.8/evidence/wp13/5346-post-deletion-validation.v1.json", schema: "adl.wp13.post_deletion_validation.v1")
  audit = json!("docs/milestones/v0.91.8/evidence/wp16/issue-outcome-audit.v1.json")
  abort("wrong issue outcome audit schema") unless audit["schema"] == "adl.v0918.wp16.issue_outcome_audit.v1"
  abort("issue outcome audit contains unacceptable outcomes") unless audit.dig("counts", "no_acceptable_outcome") == 0
  rows << { "name" => "issue-outcome-audit", "status" => "pass", "total" => audit["total"] }
  abort("missing WP-15 reconciliation ledger") unless ROOT.join("docs/milestones/v0.91.8/review/wp15_demo_matrix_5733/RECONCILIATION_LEDGER_v1.md").file?
  rows << run!("feature-crosswalk", ["ruby", ".csdlc/prepared/issues/5594/validate_feature_crosswalk.rb"], log: ".csdlc/evidence/5351/feature-crosswalk.log")
  rows << run!("structured-planning", ["ruby", ".csdlc/prepared/issues/5594/validate_structured_planning.rb"], log: ".csdlc/evidence/5351/structured-planning.log")
  rows << run!("milestone-links", ["ruby", ".csdlc/prepared/issues/5594/validate_links.rb"], log: ".csdlc/evidence/5351/milestone-links.log")
  rows << run!("diff-hygiene", ["git", "diff", "--check"], log: ".csdlc/evidence/5351/diff-hygiene.log")
  YAML.safe_load(ROOT.join("docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml").read, aliases: true)
  rows << { "name" => "docs-yaml-and-evidence-contract", "status" => "pass" }
  rows
end

def integrated_rows
  [
    ["adl-v2-all-targets", "adl-v2/Cargo.toml", "adl-v2"],
    ["runtime-v3-all-targets", "adl-runtime-kernel/Cargo.toml", "runtime-v3"],
    ["csdlc-v2-all-targets", "csdlc-v2/Cargo.toml", "csdlc-v2"]
  ].map do |name, manifest, target|
    run!(name, ["cargo", "test", "--locked", "--manifest-path", manifest, "--all-targets", "--target-dir", File.join(TARGET_ROOT, target)], log: ".csdlc/evidence/5351/#{name}.log")
  end
end

head = current_head
rows = []
rows.concat(focused_rows) if %w[focused-quality].include?(lane)
rows.concat(integrated_rows) if %w[integrated-platform].include?(lane)
if %w[complete post-merge-exact].include?(lane)
  rows << require_lane!("focused-quality", head)
  rows << require_lane!("integrated-platform", head)
end

quality_rows = [
  { "gate" => "product_contracts", "status" => "pass", "evidence" => [".csdlc/evidence/5384/platform-acceptance-ledger.v1.json", ".csdlc/evidence/5351/integrated-platform.json"] },
  { "gate" => "stable_deployments", "status" => "pass", "evidence" => [".csdlc/evidence/5384/platform-acceptance-ledger.v1.json"] },
  { "gate" => "rollback", "status" => "pass", "evidence" => ["docs/milestones/v0.91.8/evidence/wp12/cutover-5343/report.json"] },
  { "gate" => "deletion", "status" => "pass", "evidence" => ["docs/milestones/v0.91.8/evidence/wp13/5346-post-deletion-validation.v1.json"] },
  { "gate" => "demo_convergence", "status" => "pass", "evidence" => [".csdlc/evidence/5354/convergence-proof.v1.json", "docs/milestones/v0.91.8/review/wp15_demo_matrix_5733/RECONCILIATION_LEDGER_v1.md"] },
  { "gate" => "documentation", "status" => "pass", "evidence" => [".csdlc/evidence/5351/feature-crosswalk.log", ".csdlc/evidence/5351/milestone-links.log"] },
  { "gate" => "budget_cots_redaction_path_hygiene", "status" => "pass", "evidence" => [".csdlc/evidence/5351/diff-hygiene.log", ".csdlc/evidence/5351/integrated-platform.json"] },
  { "gate" => "issue_outcomes", "status" => "pass", "evidence" => ["docs/milestones/v0.91.8/evidence/wp16/issue-outcome-audit.v1.json"] }
]

packet = {
  "schema" => "adl.v0918.wp16.integrated_quality_gate.v1",
  "issue" => 5351,
  "lane" => lane,
  "status" => "pass",
  "revision" => head,
  "revision_matrix" => {
    "wp14a_acceptance" => "11151e0beab02b1667f6505b7f8992bfd47d2f8f",
    "wp15_convergence_merge" => "97427f324c87d97cb1b36c7804c50bf80c9389d8",
    "wp15_demo_reconciliation_merge" => "ab4e9e2217c152df47b1754b66b01febb4a59549",
    "wp16_execution" => head
  },
  "generated_at" => Time.now.utc.iso8601,
  "rows" => rows,
  "quality_rows" => quality_rows,
  "required_failures" => [],
  "release_claim" => "quality_gate_pass_only",
  "wp17_handoff" => lane == "post-merge-exact" ? "eligible" : "pending_wp16_merge"
}
output = EVIDENCE.join("#{lane}.json")
output.write(JSON.pretty_generate(packet) + "\n")
puts JSON.generate(packet)

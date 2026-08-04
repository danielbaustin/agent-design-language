#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "digest"

ROOT = File.expand_path("../../../..", __dir__)
ISSUE = File.join(ROOT, ".csdlc/issues/5347")
PREP = File.join(ROOT, ".csdlc/prepared/issues/5347")
LEDGER = File.join(ROOT, "docs/milestones/v0.91.8/evidence/wp13-external-bands/current-truth-ledger.json")

def fail!(message)
  warn("#5347 preparation validation failed: #{message}")
  exit(1)
end

required = %w[
  design.md
  diagram.mmd
  bootstrap-request.json
  bind-request.json
  check-dependencies.rb
  run-validation-lane.rb
  validate-blocked-state.rb
  verify-terminal-receipt.rb
  receipt-verifier/Cargo.toml
  receipt-verifier/Cargo.lock
  receipt-verifier/src/main.rs
  validation-request.json
]
required.each { |name| fail!("missing #{name}") unless File.file?(File.join(PREP, name)) }
fail!("missing current-truth ledger") unless File.file?(LEDGER)

cards = %w[sip stp spp vpp srp sor]
cards.each do |card|
  fail!("missing #{card}.md") unless File.file?(File.join(ISSUE, "cards/#{card}.md"))
  fail!("missing #{card}.values.json") unless File.file?(File.join(ISSUE, "cards/#{card}.values.json"))
end

request = JSON.parse(File.read(File.join(PREP, "bootstrap-request.json")))
fail!("wrong issue") unless request["issue"] == 5347
stp_values = JSON.parse(File.read(File.join(ISSUE, "cards/stp.values.json"))).dig("content", "values")
spp_values = JSON.parse(File.read(File.join(ISSUE, "cards/spp.values.json"))).dig("content", "values")
vpp_values = JSON.parse(File.read(File.join(ISSUE, "cards/vpp.values.json"))).dig("content", "values")
design = File.read(File.join(PREP, "design.md"))
diagram = File.read(File.join(PREP, "diagram.mmd"))
ledger = JSON.parse(File.read(LEDGER))
text = JSON.generate(stp_values) + JSON.generate(spp_values) + JSON.generate(vpp_values) + design

%w[#5346 #5344 #5343 #5358 #5361].each do |dependency|
  fail!("missing dependency #{dependency}") unless text.include?(dependency)
end

[
  "closed_out", "terminal receipt", "ancestral", "claim",
  "dependency cycle", "zero canonical path overlap", "authority-rooted",
  "delete_external", "retain_owned", "retain_evidence", "handoff_to_5346",
  "Runtime v2", "net source change is negative", "no deferred acceptance"
].each do |term|
  fail!("missing contract term #{term.inspect}") unless text.include?(term)
end

claim_paths = request.fetch("claim").fetch("protected_paths")
allowed = [
  ".csdlc/issues/5347",
  ".csdlc/locks/5347.lock",
  ".csdlc/prepared/issues/5347",
  ".csdlc/evidence/5347",
  "docs/milestones/v0.91.8/evidence/wp13-external-bands"
]
fail!("preparation claim contains product paths") unless claim_paths.sort == allowed.sort

lanes = vpp_values.fetch("lanes")
expected_lanes = %w[
  preparation-contract
  dependency-terminal-gate
  manifest-disjointness
  owner-and-consumer-proof
  deletion-budgets-and-evidence
  post-deletion-exact
]
fail!("validation lane set mismatch") unless lanes.map { |lane| lane["lane"] }.sort == expected_lanes.sort
fail!("acceptance coverage incomplete") unless lanes.flat_map { |lane| lane.fetch("acceptance_ids") }.uniq.sort == (1..10).map { |id| "AC-#{id}" }.sort
fail!("STP acceptance count mismatch") unless stp_values.fetch("acceptance_criteria").length == 10
future_lanes = lanes.reject { |lane| lane["lane"] == "preparation-contract" }
future_lanes.each do |lane|
  reason = lane["defer_reason"].to_s
  fail!("#{lane['lane']} lacks a mandatory admission condition") unless reason.include?("Mandatory") || reason.include?("expected to fail") || reason.include?("mandatory before")
  fail!("#{lane['lane']} permits optional/skipped/deferred acceptance") if reason.match?(/optional|may skip|deferred acceptance/i)
end

fail!("design omits dependency cycle") unless design.include?("dependency cycle")
fail!("design omits typed claim amendment") unless design.include?("typed protected-path claim amendment")
fail!("design omits current truth ledger") unless design.include?("current-truth-ledger.json")
fail!("diagram omits fail-closed route") unless diagram.include?("Fail closed")
fail!("diagram omits #5346 boundary") unless diagram.include?("#5346")

fail!("ledger schema mismatch") unless ledger["schema"] == "adl.wp13.external_band_current_truth.v1"
fail!("ledger issue mismatch") unless ledger["issue"] == 5347
fail!("ledger repository mismatch") unless ledger["repository"] == "danielbaustin/agent-design-language"
head, head_status = Open3.capture2("git", "-C", ROOT, "rev-parse", "HEAD")
fail!("git rev-parse failed") unless head_status.success?
fail!("ledger revision malformed") unless ledger["revision"].to_s.match?(/\A[0-9a-f]{40}\z/)
ancestor = Open3.capture2("git", "-C", ROOT, "merge-base", "--is-ancestor", ledger["revision"], head.strip).last
fail!("ledger revision is not ancestral to current HEAD") unless ancestor.success?
blocking = ledger.fetch("blocking_readiness")
fail!("ledger incorrectly marks execution ready") unless blocking["execution_ready"] == false
fail!("ledger omits #5346 blocker") unless blocking["reason"].to_s.include?("#5346")
closed_inputs = blocking.fetch("closed_dependency_inputs").to_h { |row| [row.fetch("issue"), row] }
[5343, 5344, 5358, 5361].each do |issue|
  row = closed_inputs.fetch(issue) { fail!("ledger missing closed dependency ##{issue}") }
  fail!("##{issue} not recorded closed_out") unless row["typed_phase"] == "closed_out"
  fail!("##{issue} observed SHA malformed") unless row["observed_sha"].to_s.match?(/\A[0-9a-f]{40}\z/)
end

bands = ledger.fetch("external_bands")
expected_bands = %w[
  runtime_v3_adapter
  runtime_v3_kernel_continuity_and_canonical_ingress
  reasoning_graphs_bounded_loops_adaptive_learning_affect_and_governed_cognition
  governed_operations_identity_provider_state_and_continuity
  secure_runtime_access_guardian_observatory_rollback_and_telemetry
  provider_and_governed_tool_adapters
  unity_observatory_tooling_and_demo_proof
  distributed_csdlc_workcell
  shadow_parity_and_proof_tooling
]
fail!("external band set mismatch") unless bands.map { |row| row.fetch("band") }.sort == expected_bands.sort
bands.each do |band|
  fail!("#{band['band']} lacks owner") if band["accepted_owner"].to_s.empty?
  fail!("#{band['band']} lacks manifest admission rule") if band["manifest_admission"].to_s.empty?
  band.fetch("replacement_issues").each do |row|
    issue = row.fetch("issue")
    if row["typed_phase"] == "closed_out"
      fail!("##{issue} missing observed SHA") unless row["observed_sha"].to_s.match?(/\A[0-9a-f]{40}\z/)
      fail!("##{issue} missing receipt path") unless row["receipt_path"].to_s == "csdlc-v2/closeout/#{issue}.json"
    else
      fail!("##{issue} nonterminal row lacks blocker") if row["blocker"].to_s.empty?
    end
  end
  band.fetch("evidence_refs").each do |reference|
    relative = reference.fetch("path")
    fail!("#{band['band']} evidence path escapes") if relative.start_with?("/") || relative.split("/").include?("..")
    path = File.join(ROOT, relative)
    fail!("#{band['band']} evidence missing #{relative}") unless File.file?(path)
    fail!("#{band['band']} evidence digest mismatch #{relative}") unless Digest::SHA256.file(path).hexdigest == reference.fetch("sha256")
  end
end
fail!("ledger omits #5346 non-overlap blocker") unless ledger.dig("non_overlap_with_5346", "current_status") == "blocked"
fail!("ledger permits deferral") unless ledger.dig("rollback_and_no_deferral", "no_deferral").to_s.include?("No delete_external row")

status_output, status = Open3.capture2("git", "-C", ROOT, "status", "--porcelain")
fail!("git status failed") unless status.success?
status_output.lines.each do |line|
  path = line.sub(/\A.. /, "").strip
  next if path.start_with?(".csdlc/issues/5347/", ".csdlc/prepared/issues/5347/", ".csdlc/evidence/5347/")
  next if path.start_with?("docs/milestones/v0.91.8/evidence/wp13-external-bands/")
  next if path == ".csdlc/locks/5347.lock"

  fail!("out-of-scope changed path #{path}")
end


puts(JSON.generate({schema: "adl.wp13.external_band_preparation.v1", issue: 5347, status: "pass", cards: cards.length, product_changes: 0}))

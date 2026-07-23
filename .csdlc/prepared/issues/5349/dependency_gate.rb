#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ISSUE = 5349
PREPARATION_PATHS = [
  ".csdlc/issues/5349",
  ".csdlc/locks/5349.lock",
  ".csdlc/prepared/issues/5349",
  ".csdlc/evidence/5349"
].freeze
FUTURE_PRODUCT_PATH = "adl-v2/crates/adl-adapters"

def git(*argv)
  stdout, stderr, status = Open3.capture3("git", *argv)
  [stdout.strip, stderr.strip, status]
end

def paths_overlap?(left, right)
  left == right || left.start_with?("#{right}/") || right.start_with?("#{left}/")
end

common_dir_text, common_dir_error, common_dir_status = git(
  "rev-parse", "--path-format=absolute", "--git-common-dir"
)
unless common_dir_status.success?
  warn common_dir_error
  exit 3
end

common_dir = Pathname(common_dir_text)
required_interfaces = {
  "engine_ports" => "adl-v2/crates/adl-engine/src/model.rs",
  "record_contracts" => "adl-v2/crates/adl-records/src/lib.rs"
}.map do |name, path|
  _stdout, _stderr, status = git("cat-file", "-e", "origin/main:#{path}")
  { "name" => name, "path" => path, "available_on_origin_main" => status.success? }
end

runtime_path = "adl-v2/crates/adl-runtime-v3-adapter/Cargo.toml"
_stdout, _stderr, runtime_status = git("cat-file", "-e", "origin/main:#{runtime_path}")
observations = [{
  "name" => "runtime_v3_integration_seam",
  "path" => runtime_path,
  "available_on_origin_main" => runtime_status.success?,
  "blocking" => false
}]

# Closeout receipts are audit evidence. They are deliberately not consulted by
# this execution gate and their absence can never block product work.
ready = required_interfaces.all? { |entry| entry["available_on_origin_main"] }

claim_collisions = Dir.glob(".csdlc/issues/*/index.json").sort.each_with_object([]) do |path, collisions|
  record = JSON.parse(File.read(path))
  claim = record["claim"]
  next unless claim.is_a?(Hash) && record["issue"] != ISSUE

  overlaps = Array(claim["protected_paths"]).product(
    PREPARATION_PATHS + [FUTURE_PRODUCT_PATH]
  ).select { |claimed, target| paths_overlap?(claimed, target) }
  next if overlaps.empty?

  collisions << {
    "issue" => record["issue"],
    "claim_id" => claim["id"],
    "overlaps" => overlaps
  }
rescue JSON::ParserError => error
  collisions << {
    "issue" => File.basename(File.dirname(path)),
    "claim_id" => nil,
    "overlaps" => [],
    "error" => "malformed_issue_record:#{error.message}"
  }
end

ready &&= claim_collisions.empty?

puts JSON.pretty_generate(
  "schema" => "adl.csdlc.issue_5349_dependency_gate.v1",
  "status" => ready ? "ready" : "waiting",
  "origin_main" => git("rev-parse", "origin/main").first,
  "snapshot_boundary" => "local fetched origin/main and tracked typed issue records; refresh read-only GitHub truth before product claim amendment",
  "receipt_policy" => "non_blocking_audit_evidence",
  "claim_collisions" => claim_collisions,
  "required_interfaces" => required_interfaces,
  "observations" => observations
)
exit(ready ? 0 : 2)

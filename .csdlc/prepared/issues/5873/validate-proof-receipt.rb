#!/usr/bin/env ruby
# frozen_string_literal: true
require "json"
require "digest"
require "open3"

expected_issue = 5873
expected_wp = "WP-04.11"
expected_paths = ["adl-runtime/src/distributed/placement.rs","adl-runtime/tests/distributed_placement.rs"]
expected_test = "distributed_placement"
required_platforms = []
evidence_path = ARGV.fetch(0, ".csdlc/evidence/5873/execution-proof.json")
abort "missing execution proof: #{evidence_path}" unless File.file?(evidence_path)
proof = JSON.parse(File.read(evidence_path))
abort "wrong schema" unless proof["schema"] == "adl.wp04.execution_proof.v1"
abort "wrong issue" unless proof["issue"] == expected_issue
abort "wrong WP" unless proof["wp"] == expected_wp
head, status = Open3.capture2("git", "rev-parse", "HEAD")
abort "cannot resolve HEAD" unless status.success?
abort "stale source revision" unless proof["source_revision"] == head.strip
abort "proof did not pass" unless proof["status"] == "passed"
abort "protected path drift" unless proof["protected_paths"] == expected_paths
commands = Array(proof["commands"])
matching = commands.select do |command|
  argv = Array(command["argv"])
  argv.include?(expected_test) && argv.include?("--no-tests=fail") && command["exit_code"] == 0 && command["selected_tests"].to_i.positive?
end
abort "missing nonzero exact test command #{expected_test}" unless matching.length == 1
abort "negative cases missing" if Array(proof["negative_cases"]).empty?
artifacts = Array(proof["artifacts"])
abort "artifacts missing" if artifacts.empty?
artifacts.each do |artifact|
  path = artifact.fetch("path")
  abort "artifact missing: #{path}" unless File.file?(path)
  digest = Digest::SHA256.file(path).hexdigest
  abort "artifact digest mismatch: #{path}" unless digest == artifact.fetch("sha256")
end
receipts = Array(proof["native_receipts"])
required_platforms.each do |platform|
  receipt = receipts.find { |entry| entry["platform"] == platform }
  abort "missing native receipt for #{platform}" unless receipt
  abort "stale native receipt for #{platform}" unless receipt["source_revision"] == head.strip
  abort "missing native argv for #{platform}" if Array(receipt["argv"]).empty?
  abort "missing runner identity for #{platform}" if receipt["runner_identity"].to_s.empty?
  abort "invalid output digest for #{platform}" unless receipt["output_sha256"].to_s.match?(/\A[0-9a-f]{64}\z/)
end
puts "PASS: #{expected_wp} exact-revision execution proof"

#!/usr/bin/env ruby

require "json"
require "open3"

root = File.expand_path("../../../..", __dir__)
manifest = JSON.parse(File.read(File.join(__dir__, "dependency-gate.json")))
base_sha = manifest.fetch("expected_base_sha")
allowed = [
  ".csdlc/issues/5384/",
  ".csdlc/prepared/issues/5384/",
  ".csdlc/locks/5384.lock",
  ".csdlc/issues/5354/audit.jsonl",
  ".csdlc/issues/5354/index.json",
  ".csdlc/issues/5594/audit.jsonl",
  ".csdlc/issues/5594/index.json",
  "docs/milestones/v0.91.8/WBS_v0.91.8.md",
  "docs/milestones/v0.91.8/WP_EXECUTION_READINESS_v0.91.8.md",
  "docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml",
  "docs/milestones/v0.91.8/features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md"
]
planning_paths = allowed.grep(%r{\Adocs/})
expected_claim_paths = [
  ".csdlc/issues/5384",
  ".csdlc/locks/5384.lock",
  ".csdlc/prepared/issues/5384",
  *planning_paths
].sort

commands = [
  ["git", "-C", root, "diff", "--name-only", "#{base_sha}...HEAD"],
  ["git", "-C", root, "diff", "--name-only", "--cached"],
  ["git", "-C", root, "diff", "--name-only"],
  ["git", "-C", root, "ls-files", "--others", "--exclude-standard"]
]
paths = commands.flat_map do |command|
  output, status = Open3.capture2(*command)
  abort "scope inventory command failed: #{command.join(" ")}" unless status.success?
  output.lines.map(&:strip).reject(&:empty?)
end.uniq.sort

outside = paths.reject do |path|
  allowed.any? { |entry| entry.end_with?("/") ? path.start_with?(entry) : path == entry }
end

record = JSON.parse(File.read(File.join(root, ".csdlc/issues/5384/index.json")))
claim_paths = record.fetch("claim").fetch("protected_paths").sort
claim_scope_valid = claim_paths == expected_claim_paths

overlap = lambda do |left, right|
  left == right || left.start_with?("#{right}/") || right.start_with?("#{left}/")
end
claim_collisions = Dir.glob(File.join(root, ".csdlc/issues/*/index.json")).each_with_object([]) do |path, findings|
  other = JSON.parse(File.read(path))
  next if other.fetch("issue") == 5384 || other["claim"].nil?

  collisions = other.fetch("claim").fetch("protected_paths").product(planning_paths)
    .select { |claimed, planned| overlap.call(claimed, planned) }
  next if collisions.empty?

  findings << {
    issue: other.fetch("issue"),
    phase: other.fetch("phase"),
    claim_id: other.fetch("claim").fetch("id"),
    overlaps: collisions
  }
end

ready = outside.empty? && claim_scope_valid && claim_collisions.empty?
result = {
  schema: "adl.csdlc.preparation_scope.result.v1",
  issue: 5384,
  base_sha: base_sha,
  ready: ready,
  paths: paths,
  outside_protected_paths: outside,
  expected_claim_paths: expected_claim_paths,
  claim_paths: claim_paths,
  claim_scope_valid: claim_scope_valid,
  claim_collisions: claim_collisions
}
puts JSON.pretty_generate(result)
exit(ready ? 0 : 3)

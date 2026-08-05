#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "digest"
require "open3"

SECTION = ARGV.delete("--section") ? ARGV.shift : "all"

path = ARGV.fetch(0, "docs/milestones/v0.91.8/review/runtime_v3_acceptance_5361.v1.json")
abort("acceptance register missing: #{path}") unless File.file?(path)

data = JSON.parse(File.read(path))
abort("wrong schema") unless data["schema"] == "adl.runtime_v3.acceptance.v1"
abort("wrong issue") unless data["issue"] == 5361
abort("acceptance must name an exact revision") unless data["revision"].to_s.match?(/\A[0-9a-f]{40}\z/)
accepted_revision = data.fetch("revision")

def git_success?(*argv)
  _out, _err, status = Open3.capture3("git", *argv)
  status.success?
end

def exact_revision!(revision, label)
  abort("#{label} lacks an exact revision") unless revision.to_s.match?(/\A[0-9a-f]{40}\z/)
  abort("#{label} revision is absent from Git: #{revision}") unless git_success?("cat-file", "-e", "#{revision}^{commit}")
end

def retained_proof!(proof, label, accepted_revision)
  revision = proof["revision"]
  exact_revision!(revision, label)
  abort("#{label} revision is not integrated into #{accepted_revision}") unless git_success?("merge-base", "--is-ancestor", revision, accepted_revision)

  path = proof["proof_ref"].to_s
  abort("#{label} proof_ref must be repo-relative") if path.empty? || path.start_with?("/", "../") || path.split("/").include?("..")
  digest = proof["proof_sha256"].to_s
  abort("#{label} proof digest is invalid") unless digest.match?(/\A[0-9a-f]{64}\z/)
  contents, error, status = Open3.capture3("git", "show", "#{revision}:#{path}")
  abort("#{label} proof artifact is absent at #{revision}:#{path}: #{error.strip}") unless status.success?
  abort("#{label} proof digest mismatch at claimed revision") unless Digest::SHA256.hexdigest(contents) == digest
end

exact_revision!(accepted_revision, "acceptance")

required = [5336, 5591, 5592, 5589, 5590, 5341, 5349, 5350, 5501]
proofs = Array(data["dependency_proofs"])
by_issue = proofs.to_h { |proof| [proof["issue"], proof] }
if %w[all dependencies].include?(SECTION)
  missing = required.reject do |issue|
    proof = by_issue[issue]
    next false unless proof && proof["status"] == "integrated"

    retained_proof!(proof, "dependency ##{issue}", accepted_revision)
    true
  end
  abort("missing integrated dependency proof: #{missing.join(', ')}") unless missing.empty?

  consumers = Array(data["consumer_proofs"])
  %w[adl_v2 provider_tools multi_agent_workcell].each do |consumer|
    proof = consumers.find { |entry| entry["consumer"] == consumer && entry["status"] == "passed" }
    abort("missing consumer proof: #{consumer}") unless proof
    retained_proof!(proof, "consumer #{consumer}", accepted_revision)
  end
end

surface_groups = {
  "access" => %w[secure_local_https secure_remote_https observatory telemetry address_configuration],
  "operations" => %w[canonical_ingress checkpoint_replay_resume pressure_shutdown guardian rollback recovery],
  "quality" => %w[reasoning_graphs loops affect_control adaptive_learning governed_operations runtime_v2_independence strict_lint line_count module_growth dependency_audit test_count local_pre_pr_validation exact_revision_review]
}
operational_proofs = Array(data["proofs"])
selected_surfaces = SECTION == "all" ? surface_groups.values.flatten : Array(surface_groups[SECTION])
selected_surfaces.each do |surface|
  proof = operational_proofs.find { |entry| entry["surface"] == surface && entry["status"] == "passed" }
  abort("missing operational proof: #{surface}") unless proof
  retained_proof!(proof, "surface #{surface}", accepted_revision)
end

if %w[all quality].include?(SECTION)
  non_claims = Array(data["non_claims"])
  %w[aws gpu remote_provider].each do |surface|
    next if non_claims.any? { |entry| entry["surface"] == surface && entry["status"] == "not_claimed" }
    next if operational_proofs.any? { |proof| proof["surface"] == surface && proof["status"] == "passed" }

    abort("surface must be proven or retained as a non-claim: #{surface}")
  end
end

case SECTION
when "all", "dependencies", "access", "operations", "quality"
  puts "Runtime v3 acceptance register section #{SECTION} is complete and exact-revision bound"
else
  abort("unknown acceptance-register section: #{SECTION}")
end

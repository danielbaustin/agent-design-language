#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ISSUE_CONFIG = {
  5825 => ["birthday", "docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md"],
  5826 => ["birthday_identity", "docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md"],
  5827 => ["birthday_continuity", "docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md"],
  5828 => ["memory_palace", "docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md"],
  5829 => ["capability_envelope", "docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md"],
  5830 => ["cognitive_profile", "docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md"],
  5831 => ["adaptive_learning", "docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md"],
  5833 => ["birth_witness", "docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md"]
}.freeze

def fail!(message)
  warn(message)
  exit 1
end

def repo_file(root, value, label)
  fail!("#{label} must be a nonempty repository-relative path") unless value.is_a?(String) && !value.empty?
  path = Pathname.new(value)
  fail!("#{label} must be repository-relative") if path.absolute? || path.each_filename.include?("..")
  absolute = root.join(path).cleanpath
  fail!("#{label} escapes repository root") unless absolute.to_s.start_with?("#{root}/")
  fail!("#{label} does not exist: #{value}") unless absolute.file?
  absolute
end

def tree_digest(root, relative)
  directory = root.join(relative)
  fail!("fixture directory does not exist: #{relative}") unless directory.directory?
  files = Dir.glob(directory.join("**", "*").to_s).select { |path| File.file?(path) }.sort
  fail!("fixture directory is empty: #{relative}") if files.empty?
  digest = Digest::SHA256.new
  files.each do |file|
    rel = Pathname.new(file).relative_path_from(root).to_s
    digest << rel << "\0" << Digest::SHA256.file(file).hexdigest << "\n"
  end
  digest.hexdigest
end

def source_revision_valid?(root, source_sha, protected_paths)
  return false unless /\A[0-9a-f]{40}\z/.match?(source_sha.to_s)
  _, commit_status = Open3.capture2e("git", "cat-file", "-e", "#{source_sha}^{commit}", chdir: root.to_s)
  return false unless commit_status.success?
  _, ancestor_status = Open3.capture2e("git", "merge-base", "--is-ancestor", source_sha, "HEAD", chdir: root.to_s)
  return false unless ancestor_status.success?
  _, diff_status = Open3.capture2e("git", "diff", "--quiet", source_sha, "HEAD", "--", *protected_paths, chdir: root.to_s)
  diff_status.success?
end

issue = File.basename(File.dirname(__FILE__)).to_i
test_target, feature_path = ISSUE_CONFIG.fetch(issue) { fail!("unsupported issue-local validator path") }
fixture_path = "adl-runtime-kernel/tests/fixtures/#{test_target}"
fail!("expected exactly two receipt paths") unless ARGV.length == 2

root_output, status = Open3.capture2("git", "rev-parse", "--show-toplevel")
fail!("cannot resolve repository root") unless status.success?
root = Pathname.new(root_output.strip).realpath
head_output, head_status = Open3.capture2("git", "rev-parse", "HEAD", chdir: root.to_s)
fail!("cannot resolve exact HEAD") unless head_status.success?
head = head_output.strip

expected_argv = [
  "cargo", "nextest", "run", "--manifest-path", "adl-runtime-kernel/Cargo.toml",
  "--test", test_target, "--no-tests=fail", "--status-level", "all"
]
expected_fixture_digest = tree_digest(root, fixture_path)
required_hex = /\A[0-9a-f]{64}\z/

receipts = ARGV.map do |receipt_path|
  path = repo_file(root, receipt_path, "receipt path")
  JSON.parse(path.read)
rescue JSON::ParserError => error
  fail!("invalid receipt JSON: #{error.message}")
end

fail!("native receipts must cover exactly linux and macos") unless receipts.map { |r| r["platform"] }.sort == %w[linux macos]
fail!("native receipts must bind one exact source_sha") unless receipts.map { |r| r["source_sha"] }.uniq.one?

receipts.each do |receipt|
  platform = receipt.fetch("platform")
  fail!("#{platform}: status must be passed") unless receipt["status"] == "passed"
  tests_run = Integer(receipt["tests_run"], exception: false)
  fail!("#{platform}: tests_run must be positive") unless tests_run&.positive?
  source_paths = [
    "adl-runtime-kernel/Cargo.toml", "adl-runtime-kernel/src/lib.rs",
    "adl-runtime-kernel/src/#{test_target}.rs", "adl-runtime-kernel/tests/#{test_target}.rs",
    fixture_path, feature_path
  ]
  fail!("#{platform}: exact source_sha is absent, non-ancestral, or stale for owned source paths") unless source_revision_valid?(root, receipt["source_sha"], source_paths)
  fail!("#{platform}: argv does not match the declared exact test") unless receipt["argv"] == expected_argv
  fail!("#{platform}: fixture_path mismatch") unless receipt["fixture_path"] == fixture_path
  fail!("#{platform}: fixture digest mismatch") unless receipt["fixture_digest"] == expected_fixture_digest
  fail!("#{platform}: runner_identity is required") unless receipt["runner_identity"].is_a?(String) && !receipt["runner_identity"].strip.empty?

  output = repo_file(root, receipt["output_path"], "#{platform} output_path")
  output_digest = Digest::SHA256.file(output).hexdigest
  fail!("#{platform}: invalid output_digest") unless required_hex.match?(receipt["output_digest"].to_s)
  fail!("#{platform}: output digest mismatch") unless receipt["output_digest"] == output_digest

  artifact = repo_file(root, receipt["native_artifact_path"], "#{platform} native_artifact_path")
  artifact_digest = Digest::SHA256.file(artifact).hexdigest
  fail!("#{platform}: invalid native_artifact_digest") unless required_hex.match?(receipt["native_artifact_digest"].to_s)
  fail!("#{platform}: native artifact digest mismatch") unless receipt["native_artifact_digest"] == artifact_digest
end

fail!("platform outputs differ") unless receipts.map { |r| r["output_digest"] }.uniq.one?
puts JSON.generate(issue: issue, status: "passed", reviewed_head: head, source_sha: receipts.first["source_sha"], fixture_digest: expected_fixture_digest, platforms: %w[linux macos])

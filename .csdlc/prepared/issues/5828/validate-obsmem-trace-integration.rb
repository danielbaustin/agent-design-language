#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

AUTHORITY_PATHS = [
  "adl/src/obsmem_contract/models.rs",
  "adl-runtime-kernel/src/observability.rs",
  "adl-runtime-kernel/src/proof.rs"
].freeze
EXPECTED_ARGV = [
  "cargo", "nextest", "run", "--manifest-path", "adl-runtime-kernel/Cargo.toml",
  "--test", "memory_palace", "--no-tests=fail", "--status-level", "all"
].freeze
FIXTURE_PATH = "adl-runtime-kernel/tests/fixtures/memory_palace"

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

fail!("usage: validate-obsmem-trace-integration.rb RECEIPT.json") unless ARGV.length == 1
root_output, status = Open3.capture2("git", "rev-parse", "--show-toplevel")
fail!("cannot resolve repository root") unless status.success?
root = Pathname.new(root_output.strip).realpath
head_output, head_status = Open3.capture2("git", "rev-parse", "HEAD", chdir: root.to_s)
fail!("cannot resolve exact HEAD") unless head_status.success?
head = head_output.strip

receipt_path = repo_file(root, ARGV.fetch(0), "receipt path")
receipt = JSON.parse(receipt_path.read)
fail!("status must be passed") unless receipt["status"] == "passed"
tests_run = Integer(receipt["tests_run"], exception: false)
fail!("tests_run must be positive") unless tests_run&.positive?
source_paths = AUTHORITY_PATHS + [
  "adl-runtime-kernel/Cargo.toml", "adl-runtime-kernel/src/lib.rs",
  "adl-runtime-kernel/src/memory_palace.rs", "adl-runtime-kernel/tests/memory_palace.rs",
  FIXTURE_PATH, "docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md"
]
fail!("exact source_sha is absent, non-ancestral, or stale for owned and authority paths") unless source_revision_valid?(root, receipt["source_sha"], source_paths)
fail!("argv does not match the declared exact test") unless receipt["argv"] == EXPECTED_ARGV
fail!("runner_identity is required") unless receipt["runner_identity"].is_a?(String) && !receipt["runner_identity"].strip.empty?
fail!("trace_id is required") unless receipt["trace_id"].is_a?(String) && !receipt["trace_id"].strip.empty?
citations = receipt["citation_ids"]
fail!("citation_ids must be a nonempty unique string array") unless citations.is_a?(Array) && !citations.empty? && citations.all? { |v| v.is_a?(String) && !v.empty? } && citations.uniq.length == citations.length

authority_digests = receipt["authority_digests"]
fail!("authority_digests must name the exact authority set") unless authority_digests.is_a?(Hash) && authority_digests.keys.sort == AUTHORITY_PATHS.sort
AUTHORITY_PATHS.each do |path|
  actual = Digest::SHA256.file(repo_file(root, path, "authority path")).hexdigest
  fail!("authority digest mismatch: #{path}") unless authority_digests[path] == actual
end

fail!("fixture_path mismatch") unless receipt["fixture_path"] == FIXTURE_PATH
actual_fixture_digest = tree_digest(root, FIXTURE_PATH)
fail!("fixture digest mismatch") unless receipt["fixture_digest"] == actual_fixture_digest

output = repo_file(root, receipt["output_path"], "output_path")
actual_output_digest = Digest::SHA256.file(output).hexdigest
fail!("output digest mismatch") unless receipt["output_digest"] == actual_output_digest

puts JSON.generate(status: "passed", reviewed_head: head, source_sha: receipt["source_sha"], fixture_digest: actual_fixture_digest, output_digest: actual_output_digest, trace_id: receipt["trace_id"], citation_count: citations.length)

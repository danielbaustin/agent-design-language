#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "optparse"
require "pathname"
require "rbconfig"

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

def canonical_json(value)
  case value
  when Hash
    "{" + value.keys.sort.map { |key| "#{JSON.generate(key)}:#{canonical_json(value.fetch(key))}" }.join(",") + "}"
  when Array
    "[" + value.map { |entry| canonical_json(entry) }.join(",") + "]"
  else
    JSON.generate(value)
  end
end

def repo_path(root, relative, label)
  path = Pathname.new(relative.to_s)
  fail!("#{label} must be repository-relative") if relative.to_s.empty? || path.absolute? || path.each_filename.include?("..")
  absolute = root.join(path).cleanpath
  fail!("#{label} escapes repository root") unless absolute.to_s.start_with?("#{root}/")
  absolute
end

def source_paths(test_target, feature_path)
  [
    "adl-runtime-kernel/Cargo.toml",
    "adl-runtime-kernel/src/lib.rs",
    "adl-runtime-kernel/src/#{test_target}.rs",
    "adl-runtime-kernel/tests/#{test_target}.rs",
    "adl-runtime-kernel/tests/fixtures/#{test_target}",
    feature_path
  ]
end

def source_manifest(root, paths)
  rows = paths.flat_map do |relative|
    absolute = root.join(relative)
    files = absolute.directory? ? Dir.glob(absolute.join("**", "*").to_s).select { |path| File.file?(path) }.sort : [absolute.to_s]
    fail!("source contract path is absent: #{relative}") if files.empty? || files.any? { |path| !File.file?(path) }
    files.map do |file|
      rel = Pathname.new(file).relative_path_from(root).to_s
      { "path" => rel, "sha256" => Digest::SHA256.file(file).hexdigest }
    end
  end
  rows.sort_by { |row| row.fetch("path") }
end

options = {}
OptionParser.new do |parser|
  parser.on("--platform PLATFORM") { |value| options[:platform] = value }
  parser.on("--receipt PATH") { |value| options[:receipt] = value }
  parser.on("--semantic-output PATH") { |value| options[:semantic_output] = value }
end.parse!
fail!("unexpected positional arguments") unless ARGV.empty?
fail!("platform must be macos or linux") unless %w[macos linux].include?(options[:platform])
fail!("native receipts must be produced by GitHub Actions") unless ENV["GITHUB_ACTIONS"] == "true"

root_text, root_status = Open3.capture2("git", "rev-parse", "--show-toplevel")
fail!("cannot resolve repository root") unless root_status.success?
root = Pathname.new(root_text.strip).realpath
head_text, head_status = Open3.capture2("git", "rev-parse", "HEAD", chdir: root.to_s)
fail!("cannot resolve exact HEAD") unless head_status.success?
head = head_text.strip

issue = File.basename(File.dirname(__FILE__)).to_i
test_target, feature_path = ISSUE_CONFIG.fetch(issue) { fail!("unsupported issue-local producer path") }
expected_os = options[:platform] == "macos" ? "Darwin" : "Linux"
host_os, host_status = Open3.capture2("uname", "-s")
fail!("producer platform does not match native runner") unless host_status.success? && host_os.strip == expected_os

receipt_path = repo_path(root, options[:receipt], "receipt")
semantic_path = repo_path(root, options[:semantic_output], "semantic output")
evidence_root = root.join(".csdlc/evidence/#{issue}/native-platform").cleanpath
[receipt_path, semantic_path].each do |path|
  fail!("evidence output must remain below #{evidence_root.relative_path_from(root)}") unless path.to_s.start_with?("#{evidence_root}/")
end
FileUtils.mkdir_p(receipt_path.dirname)
FileUtils.mkdir_p(semantic_path.dirname)
command_output_path = receipt_path.dirname.join("#{options[:platform]}-nextest.log")
manifest_path = receipt_path.dirname.join("#{options[:platform]}-source-manifest.json")

test_argv = [
  "cargo", "nextest", "run", "--manifest-path", "adl-runtime-kernel/Cargo.toml",
  "--test", test_target, "--no-tests=fail", "--status-level", "all"
]
stdout, stderr, status = Open3.capture3(
  { "ADL_NATIVE_SEMANTIC_OUTPUT" => semantic_path.relative_path_from(root).to_s },
  *test_argv,
  chdir: root.to_s
)
command_output = stdout + stderr
command_output_path.write(command_output)
fail!("native nextest command failed") unless status.success?
summary = command_output.match(/(?<count>\d+)\s+tests?\s+run:/)
fail!("native nextest output lacks a positive test summary") unless summary && summary[:count].to_i.positive?
fail!("test did not produce the declared semantic output") unless semantic_path.file? && semantic_path.size.positive?

manifest = source_manifest(root, source_paths(test_target, feature_path))
manifest_path.write(JSON.pretty_generate(manifest) + "\n")
producer_rel = Pathname.new(__FILE__).realpath.relative_path_from(root).to_s
payload = {
  "issue" => issue,
  "platform" => options[:platform],
  "source_sha" => head,
  "producer_path" => producer_rel,
  "producer_sha256" => Digest::SHA256.file(root.join(producer_rel)).hexdigest,
  "producer_argv" => ["ruby", producer_rel, "--platform", options[:platform], "--receipt", options[:receipt], "--semantic-output", options[:semantic_output]],
  "test_argv" => test_argv,
  "test_environment" => { "ADL_NATIVE_SEMANTIC_OUTPUT" => semantic_path.relative_path_from(root).to_s },
  "tests_run" => summary[:count].to_i,
  "command_output_path" => command_output_path.relative_path_from(root).to_s,
  "command_output_sha256" => Digest::SHA256.file(command_output_path).hexdigest,
  "semantic_output_path" => semantic_path.relative_path_from(root).to_s,
  "semantic_output_sha256" => Digest::SHA256.file(semantic_path).hexdigest,
  "source_manifest_path" => manifest_path.relative_path_from(root).to_s,
  "source_manifest_sha256" => Digest::SHA256.file(manifest_path).hexdigest,
  "runner" => {
    "provider" => "github_actions",
    "repository" => ENV.fetch("GITHUB_REPOSITORY"),
    "workflow_ref" => ENV.fetch("GITHUB_WORKFLOW_REF"),
    "run_id" => ENV.fetch("GITHUB_RUN_ID"),
    "run_attempt" => ENV.fetch("GITHUB_RUN_ATTEMPT"),
    "job" => ENV.fetch("GITHUB_JOB"),
    "os" => host_os.strip,
    "architecture" => RbConfig::CONFIG.fetch("host_cpu")
  },
  "status" => "passed"
}
receipt = {
  "schema" => "adl.native_ci_receipt.v1",
  "payload" => payload,
  "payload_sha256" => Digest::SHA256.hexdigest(canonical_json(payload))
}
receipt_path.write(JSON.pretty_generate(receipt) + "\n")
puts JSON.generate(issue: issue, platform: options[:platform], source_sha: head, receipt: options[:receipt])

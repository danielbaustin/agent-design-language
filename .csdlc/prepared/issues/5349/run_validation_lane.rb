#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"
require "rbconfig"

ISSUE = "5349"
BASE = ".csdlc/prepared/issues/#{ISSUE}"
MANIFEST = "adl-v2/crates/adl-adapters/Cargo.toml"
TARGET_ROOT = "/Volumes/FastWork/adl-5349"

def run!(environment, *argv)
  stdout, stderr, status = Open3.capture3(environment, *argv)
  $stdout.write(stdout)
  $stderr.write(stderr)
  exit(status.exitstatus || 1) unless status.success?
end

lane = ARGV.fetch(0) do
  warn "usage: run_validation_lane.rb <lane>"
  exit 64
end

FileUtils.mkdir_p(TARGET_ROOT)
environment = { "CARGO_TARGET_DIR" => File.join(TARGET_ROOT, lane) }

case lane
when "all"
  %w[dependency mock https governed-tool compatibility negative-authority complete strict-quality inventory].each do |required_lane|
    run!({}, RbConfig.ruby, __FILE__, required_lane)
  end
when "dependency"
  exec("ruby", "#{BASE}/dependency_gate.rb")
when "mock"
  exec(environment, "cargo", "test", "--manifest-path", MANIFEST, "--test", "mock_adapter")
when "https"
  exec(environment, "cargo", "test", "--manifest-path", MANIFEST, "--features", "test-transport", "--test", "https_adapter")
when "governed-tool"
  exec(environment, "cargo", "test", "--manifest-path", MANIFEST, "--test", "governed_tool_adapter")
when "compatibility"
  exec(environment, "cargo", "test", "--manifest-path", MANIFEST, "--test", "compatibility_adapter")
when "negative-authority"
  run!(environment, "cargo", "test", "--manifest-path", MANIFEST, "--test", "secret_hygiene")
  exec(environment, "cargo", "test", "--manifest-path", MANIFEST, "--test", "negative_authority")
when "complete"
  exec(environment, "cargo", "test", "--locked", "--manifest-path", MANIFEST, "--all-targets", "--all-features")
when "strict-quality"
  run!(environment, "cargo", "fmt", "--manifest-path", MANIFEST, "--all", "--", "--check")
  exec(environment, "cargo", "clippy", "--locked", "--manifest-path", MANIFEST, "--all-targets", "--all-features", "--", "-D", "warnings")
when "inventory"
  run!(environment, "cargo", "tree", "--locked", "--manifest-path", MANIFEST)
  exec("ruby", "#{BASE}/validate_budget.rb")
when "exact-revision"
  run!({}, "git", "diff", "--check", "origin/main...HEAD")
  status, _status_error, status_result = Open3.capture3("git", "status", "--porcelain")
  abort "exact-revision lane requires a clean worktree" unless status_result.success? && status.empty?
  head, _head_error, head_result = Open3.capture3("git", "rev-parse", "HEAD")
  abort "unable to resolve HEAD" unless head_result.success?
  record = JSON.parse(File.read(".csdlc/issues/#{ISSUE}/index.json"))
  review = record["review"]
  reviewed_revision = review&.fetch("reviewed_revision", "").to_s
  exact_review = reviewed_revision == head.strip ||
    reviewed_revision.match?(/\Agit-blake3:#{Regexp.escape(head.strip)}:[0-9a-f]{64}\z/)
  unless review.is_a?(Hash) && review["status"] == "completed" && exact_review
    abort "typed review does not identify exact HEAD"
  end
  rollback = File.read("#{BASE}/design.md")[/^## Rollback\n.*?(?=^## )/m].to_s
  abort "rollback contract is missing" if rollback.empty?
  vpp = JSON.parse(File.read(".csdlc/issues/#{ISSUE}/cards/vpp.values.json"))
  lanes = vpp.dig("content", "values", "lanes") || []
  abort "required validation lane is deferred" if lanes.any? { |entry| !entry["defer_reason"].nil? }
  run!({}, "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-install", "resolve", "--root", ".")
  run!({}, "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor", "--repo", ".", "--issue", ISSUE)
  exit 0
else
  warn "unknown lane: #{lane}"
  exit 64
end

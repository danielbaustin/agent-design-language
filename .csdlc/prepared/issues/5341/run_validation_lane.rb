#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "open3"
require "pathname"

ISSUE = 5341
BASE = Pathname(".csdlc/prepared/issues/#{ISSUE}")
MANIFEST = "adl-v2/crates/adl-runtime-v3-adapter/Cargo.toml"
TARGET_ROOT = "/Volumes/FastWork/adl-5341/target"
CARGO_HOME = "/Volumes/FastWork/adl-cargo-home"

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
environment = { "CARGO_HOME" => CARGO_HOME, "CARGO_TARGET_DIR" => TARGET_ROOT }

case lane
when "dependency-gate"
  exec("ruby", BASE.join("dependency_gate.rb").to_s)
when "mapping-unit"
  exec(environment, "cargo", "test", "--manifest-path", MANIFEST, "--test", "adapter", "mapping_")
when "canonical-ingress-integration"
  exec(environment, "cargo", "test", "--manifest-path", MANIFEST, "--test", "adapter", "canonical_ingress_")
when "negative-authority"
  exec(environment, "cargo", "test", "--manifest-path", MANIFEST, "--test", "adapter", "rejects_")
when "complete-adapter-suite"
  exec(environment, "cargo", "test", "--manifest-path", MANIFEST, "--all-targets", "--all-features")
when "strict-quality"
  run!(environment, "cargo", "fmt", "--manifest-path", MANIFEST, "--all", "--", "--check")
  exec(environment, "cargo", "clippy", "--manifest-path", MANIFEST, "--all-targets", "--all-features", "--", "-D", "warnings")
when "inventory-and-boundary"
  run!(environment, "cargo", "tree", "--locked", "--manifest-path", MANIFEST)
  run!({}, "ruby", BASE.join("validate_budget.rb").to_s)
  forbidden = Regexp.union(
    /runtime[_-]?v2/i,
    /aws/i,
    /csdlc/i,
    /std::net/,
    /tokio::net/,
    /TcpListener/,
    /axum/,
    /reqwest/,
    /rustls/,
    /127[.]0[.]0[.]1/,
    /0[.]0[.]0[.]0/,
    /http:/
  )
  files = Dir.glob("adl-v2/crates/adl-runtime-v3-adapter/{src,tests}/**/*").select { |path| File.file?(path) }
  matches = files.flat_map do |path|
    File.readlines(path, chomp: true).each_with_index.map do |line, index|
      "#{path}:#{index + 1}:#{line}" if line.match?(forbidden)
    end.compact
  end
  unless matches.empty?
    warn matches.join("\n")
    exit 1
  end
when "exact-revision-truth"
  run!({}, "git", "diff", "--check", "origin/main...HEAD")
  run!({}, "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor", "--repo", ".", "--issue", ISSUE.to_s)
  exit 0
else
  warn "unknown lane: #{lane}"
  exit 64
end

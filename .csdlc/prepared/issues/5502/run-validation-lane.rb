#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb <lane>") }
allowed = %w[convergence-contract property-matrix budgets post-merge-exact]
abort("unsupported lane: #{lane}") unless allowed.include?(lane)

root = File.expand_path("../../../..", __dir__)

def run(root, *argv)
  system(*argv, chdir: root)
end

case lane
when "convergence-contract", "property-matrix"
  ok = run(root, "cargo", "test", "--manifest-path", "adl-v2/Cargo.toml", "-p", "adl-workcell-convergence")
when "budgets"
  lib_lines = Dir.glob(File.join(root, "adl-v2/crates/adl-workcell-convergence/src/**/*.rs")).sum { |path| File.readlines(path).size }
  test_lines = Dir.glob(File.join(root, "adl-v2/crates/adl-workcell-convergence/tests/**/*.rs")).sum { |path| File.readlines(path).size }
  ok = lib_lines <= 2_500 && test_lines <= 2_500
  puts JSON.pretty_generate(status: ok ? "passed" : "failed", lane: lane, product_lines: lib_lines, test_lines: test_lines)
when "post-merge-exact"
  ok = run(root, "ruby", ".csdlc/prepared/issues/5502/check-dependencies.rb") &&
       run(root, "cargo", "test", "--manifest-path", "adl-v2/Cargo.toml", "-p", "adl-workcell-convergence")
end

abort("#{lane} failed") unless ok
puts JSON.pretty_generate(status: "passed", lane: lane)

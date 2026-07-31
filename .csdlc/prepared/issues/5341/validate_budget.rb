#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname("adl-v2/crates/adl-runtime-v3-adapter")
SOURCE_BUDGET = 500
TEST_BUDGET = 1_000
MODULE_BUDGET = 250
MIN_TESTS = 12
EXPECTED_PRODUCTION_COTS = %w[serde_json sha2].freeze

def rust_files(path)
  Dir.glob(path.join("**", "*.rs").to_s).sort.map { |entry| Pathname(entry) }
end

def physical_lines(path)
  path.each_line.count
end

unless ROOT.directory?
  puts JSON.pretty_generate(
    "schema" => "adl.csdlc.issue_5341_budget.v1",
    "status" => "waiting",
    "reason" => "adapter_crate_not_implemented"
  )
  exit 2
end

source_files = rust_files(ROOT.join("src"))
test_files = rust_files(ROOT.join("tests"))
source_lines = source_files.sum { |path| physical_lines(path) }
test_lines = test_files.sum { |path| physical_lines(path) }
largest_module = source_files.map { |path| [path.to_s, physical_lines(path)] }.max_by(&:last)
inline_test_files = source_files.select { |path| path.read.include?("#[cfg(test)]") }
test_count = test_files.sum do |path|
  path.read.scan(/^\s*#\[(?:tokio::)?test(?:\([^\n]*\))?\]\s*$/).length
end

cargo_toml = ROOT.join("Cargo.toml")
cargo_text = cargo_toml.file? ? cargo_toml.read : ""
dependency_section = cargo_text[/^\[dependencies\]\s*$.*?(?=^\[|\z)/m].to_s
external_production = dependency_section.lines.map do |line|
  next if line.strip.empty? || line.lstrip.start_with?("#", "[")
  next if line.include?("path =")
  line.split("=", 2).first&.strip
end.compact
external_production.reject!(&:empty?)

checks = {
  "source_lines" => source_lines <= SOURCE_BUDGET,
  "test_lines" => test_lines <= TEST_BUDGET,
  "largest_module" => largest_module.nil? || largest_module.last <= MODULE_BUDGET,
  "minimum_tests" => test_count >= MIN_TESTS,
  "no_inline_tests" => inline_test_files.empty?,
  "declared_production_cots_only" => external_production.sort == EXPECTED_PRODUCTION_COTS
}

puts JSON.pretty_generate(
  "schema" => "adl.csdlc.issue_5341_budget.v1",
  "status" => checks.values.all? ? "passed" : "failed",
  "budgets" => {
    "source_lines_max" => SOURCE_BUDGET,
    "test_lines_max" => TEST_BUDGET,
    "module_lines_max" => MODULE_BUDGET,
    "minimum_tests" => MIN_TESTS,
    "direct_production_cots" => EXPECTED_PRODUCTION_COTS
  },
  "observed" => {
    "source_lines" => source_lines,
    "test_lines" => test_lines,
    "largest_module" => largest_module,
    "test_count" => test_count,
    "inline_test_files" => inline_test_files.map(&:to_s),
    "direct_production_cots" => external_production
  },
  "checks" => checks
)
exit(checks.values.all? ? 0 : 1)

#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname("adl-v2/crates/adl-adapters")
SOURCE_BUDGET = 1_500
TEST_BUDGET = 2_500
MODULE_BUDGET = 350
MIN_TESTS = 30
EXPECTED_PRODUCTION = {
  "reqwest" => "0.13.4",
  "secrecy" => "0.10.3",
  "url" => "2.5.8",
  "serde" => "1.0.229",
  "serde_json" => "1.0.151",
  "tokio" => "1.53.1"
}.freeze
EXPECTED_DEV = { "wiremock" => "0.6.5" }.freeze
EXPECTED_FEATURES = {
  "reqwest" => %w[json rustls stream],
  "secrecy" => [],
  "url" => [],
  "serde" => ["derive"],
  "serde_json" => [],
  "tokio" => %w[io-util rt sync time]
}.freeze
EXPECTED_PATH_PACKAGES = {
  "adl-engine" => "adl-v2/crates/adl-engine",
  "adl-records" => "adl-v2/crates/adl-records"
}.freeze
FORBIDDEN_PACKAGES = %w[
  aws-config aws-sdk-bedrockruntime libloading native-tls openssl policy-engine
].freeze

def rust_files(path)
  Dir.glob(path.join("**", "*.rs").to_s).sort.map { |entry| Pathname(entry) }
end

def physical_lines(path)
  path.each_line.count
end

def dependency_versions(text, section)
  body = text[/^\[#{Regexp.escape(section)}\]\s*$.*?(?=^\[|\z)/m].to_s
  body.lines.each_with_object({}) do |line, dependencies|
    next if line.strip.empty? || line.lstrip.start_with?("#")

    name, value = line.split("=", 2).map(&:strip)
    next if name.nil? || value.nil? || value.include?("path =")

    version = value[/version\s*=\s*"([^"]+)"/, 1] || value[/\A"([^"]+)"\z/, 1]
    dependencies[name] = version&.delete_prefix("=")
  end
end

def cargo_metadata
  stdout, stderr, status = Open3.capture3(
    { "CARGO_TARGET_DIR" => "/Volumes/FastWork/adl-5349/inventory" },
    "cargo", "metadata", "--locked", "--format-version", "1",
    "--manifest-path", ROOT.join("Cargo.toml").to_s
  )
  raise "cargo metadata failed: #{stderr}" unless status.success?

  JSON.parse(stdout)
end

unless ROOT.directory?
  puts JSON.pretty_generate(
    "schema" => "adl.csdlc.issue_5349_budget.v1",
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

cargo_text = ROOT.join("Cargo.toml").read
production = dependency_versions(cargo_text, "dependencies")
development = dependency_versions(cargo_text, "dev-dependencies")
metadata = cargo_metadata
package = metadata.fetch("packages").find do |candidate|
  Pathname(candidate.fetch("manifest_path")).cleanpath ==
    ROOT.join("Cargo.toml").expand_path.cleanpath
end
raise "adapter package missing from cargo metadata" unless package

direct = package.fetch("dependencies")
registry_dependencies = direct.reject { |dependency| dependency["path"] }
path_dependencies = direct.select { |dependency| dependency["path"] }
metadata_versions = registry_dependencies.to_h do |dependency|
  [dependency.fetch("name"), dependency.fetch("req").delete_prefix("=")]
end
production_metadata = registry_dependencies.reject { |dependency| dependency["kind"] == "dev" }
metadata_features = production_metadata.to_h do |dependency|
  [dependency.fetch("name"), dependency.fetch("features").sort]
end
default_features = production_metadata.to_h do |dependency|
  [dependency.fetch("name"), dependency.fetch("uses_default_features")]
end
path_package_paths = path_dependencies.to_h do |dependency|
  path = Pathname(dependency.fetch("path")).expand_path.cleanpath
  relative = path.relative_path_from(Pathname.pwd.expand_path.cleanpath).to_s
  [dependency.fetch("name"), relative]
end
resolved_names = metadata.fetch("packages").map { |entry| entry.fetch("name") }.uniq.sort
checks = {
  "source_lines" => source_lines <= SOURCE_BUDGET,
  "test_lines" => test_lines <= TEST_BUDGET,
  "largest_module" => largest_module.nil? || largest_module.last <= MODULE_BUDGET,
  "minimum_tests" => test_count >= MIN_TESTS,
  "no_inline_tests" => inline_test_files.empty?,
  "production_cots" => production == EXPECTED_PRODUCTION,
  "development_cots" => development == EXPECTED_DEV,
  "exact_registry_requirements" => metadata_versions == EXPECTED_PRODUCTION.merge(EXPECTED_DEV),
  "production_features" => metadata_features == EXPECTED_FEATURES,
  "restricted_default_features_disabled" =>
    %w[reqwest tokio].all? { |name| default_features[name] == false },
  "approved_path_dependencies" => path_package_paths == EXPECTED_PATH_PACKAGES,
  "forbidden_packages_absent" => (resolved_names & FORBIDDEN_PACKAGES).empty?,
  "crate_scope" => ROOT.cleanpath.to_s == "adl-v2/crates/adl-adapters"
}

puts JSON.pretty_generate(
  "schema" => "adl.csdlc.issue_5349_budget.v1",
  "status" => checks.values.all? ? "passed" : "failed",
  "budgets" => {
    "source_lines_max" => SOURCE_BUDGET,
    "test_lines_max" => TEST_BUDGET,
    "module_lines_max" => MODULE_BUDGET,
    "minimum_tests" => MIN_TESTS,
    "production_cots" => EXPECTED_PRODUCTION,
    "development_cots" => EXPECTED_DEV
  },
  "observed" => {
    "source_lines" => source_lines,
    "test_lines" => test_lines,
    "largest_module" => largest_module,
    "test_count" => test_count,
    "inline_test_files" => inline_test_files.map(&:to_s),
    "production_cots" => production,
    "development_cots" => development,
    "metadata_versions" => metadata_versions,
    "metadata_features" => metadata_features,
    "default_features" => default_features,
    "path_package_paths" => path_package_paths,
    "forbidden_packages_present" => resolved_names & FORBIDDEN_PACKAGES
  },
  "checks" => checks
)
exit(checks.values.all? ? 0 : 1)

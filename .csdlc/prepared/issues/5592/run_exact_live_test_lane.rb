#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "optparse"

options = {
  inventory: ".csdlc/prepared/issues/5592/future-live-test-inventory.json"
}
OptionParser.new do |parser|
  parser.on("--inventory PATH") { |path| options[:inventory] = path }
  parser.on("--lane ID") { |id| options[:lane] = id }
  parser.on("--list-file PATH") { |path| options[:list_file] = path }
  parser.on("--check-only") { options[:check_only] = true }
  parser.on("--self-test") { options[:self_test] = true }
end.parse!

def discovered_tests(listing)
  listing.lines.map do |line|
    match = line.match(/^([^\s].*): test$/)
    match && match[1]
  end.compact
end

def require_exact_tests!(expected, listing, context)
  discovered = discovered_tests(listing)
  matches = expected.select { |name| discovered.count(name) == 1 }
  abort "#{context}: zero exact live-kernel test matches" if matches.empty?
  missing = expected - matches
  abort "#{context}: missing exact tests: #{missing.join(', ')}" unless missing.empty?
  duplicates = expected.select { |name| discovered.count(name) > 1 }
  abort "#{context}: duplicate exact tests: #{duplicates.join(', ')}" unless duplicates.empty?
end

if options[:self_test]
  exact = "adaptive_learning_consumes_exact_one_shot_mutation_authority"
  legacy = "adaptive_learning_dag_proof_resolves_only_learning_blocker: test\n"
  begin
    require_exact_tests!([exact], legacy, "legacy metadata isolation")
    abort "self-test: legacy metadata unexpectedly earned live-kernel credit"
  rescue SystemExit => error
    raise unless error.status != 0
  end
  require_exact_tests!([exact], "#{exact}: test\n", "exact inventory")
  puts "self-test=pass zero_match=fail legacy_metadata_credit=denied exact_match=pass"
  exit 0
end

abort "--lane is required" unless options[:lane]
inventory = JSON.parse(File.read(options[:inventory]))
abort "inventory issue mismatch" unless inventory.fetch("issue") == 5592
abort "inventory proof class is not future_live_kernel" unless inventory.fetch("proof_class") == "future_live_kernel"
abort "inventory does not fail on zero matches" unless inventory.fetch("zero_matches") == "fail"
abort "inventory runtime path is not canonical ingress" unless inventory.fetch("required_runtime_path") == "guardian_canonical_ingress"
abort "inventory can credit metadata" unless inventory.fetch("forbidden_credit").include?("metadata")

lane = inventory.fetch("lanes").find { |entry| entry.fetch("id") == options[:lane] }
abort "unknown exact-test lane: #{options[:lane]}" unless lane
expected = lane.fetch("exact_tests")
abort "lane has zero exact test identities" if expected.empty?
abort "lane has duplicate exact test identities" unless expected.uniq == expected

listing = if options[:list_file]
            File.read(options[:list_file])
          else
            command = [
              "cargo", "test", "--manifest-path", inventory.fetch("manifest_path"),
              "--test", inventory.fetch("test_target"), "--", "--list"
            ]
            stdout, stderr, status = Open3.capture3(*command)
            abort "cargo test inventory failed: #{stderr.strip}" unless status.success?
            stdout
          end

require_exact_tests!(expected, listing, options[:lane])
if options[:check_only]
  puts "lane=#{options[:lane]} exact_matches=#{expected.length} execution=not_requested"
  exit 0
end
abort "--list-file is check-only evidence and cannot execute tests" if options[:list_file]

expected.each do |test_name|
  command = [
    "cargo", "test", "--manifest-path", inventory.fetch("manifest_path"),
    "--test", inventory.fetch("test_target"), test_name, "--", "--exact", "--nocapture"
  ]
  abort "exact live-kernel test failed: #{test_name}" unless system(*command)
end
puts "lane=#{options[:lane]} exact_matches=#{expected.length} execution=pass"

#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "optparse"

options = { self_test_zero_match: false }
OptionParser.new do |parser|
  parser.on("--inventory PATH") { |value| options[:inventory] = value }
  parser.on("--lane ID") { |value| options[:lane] = value }
  parser.on("--manifest PATH") { |value| options[:manifest] = value }
  parser.on("--self-test-zero-match") { options[:self_test_zero_match] = true }
end.parse!

abort "inventory, lane, and manifest are required" unless options.values_at(:inventory, :lane, :manifest).all?

inventory = JSON.parse(File.read(options.fetch(:inventory)))
lane = inventory.fetch("lanes").fetch(options.fetch(:lane))
expected = lane.fetch("tests")
minimum = lane.fetch("minimum_count")
filter = lane.fetch("filter")

abort "focused inventory must require at least one test" unless minimum.positive?
abort "focused inventory count is below its minimum" unless expected.length >= minimum
abort "focused inventory contains duplicate tests" unless expected.uniq.length == expected.length
abort "focused inventory contains an out-of-filter test" unless expected.all? { |name| name.include?(filter) }

def verify_inventory!(expected, observed, minimum, filter)
  matched = observed.select { |name| name.include?(filter) }.sort
  abort "focused lane matched zero tests: #{filter}" if matched.empty?
  abort "focused lane matched #{matched.length}, below required #{minimum}: #{filter}" if matched.length < minimum
  abort "focused lane inventory drift for #{filter}: expected=#{expected.sort.inspect} observed=#{matched.inspect}" unless matched == expected.sort
end

if options.fetch(:self_test_zero_match)
  verify_inventory!(expected, [], minimum, filter)
  abort "zero-match self-test unexpectedly passed"
end

list_stdout, list_stderr, list_status = Open3.capture3(
  "cargo", "test", "--manifest-path", options.fetch(:manifest), "--", "--list"
)
abort "cargo test inventory failed: #{list_stderr}" unless list_status.success?

observed = list_stdout.lines.map { |line| line[/\A(.+): test\s*\z/, 1] }.compact
verify_inventory!(expected, observed, minimum, filter)

expected.sort.each do |test_name|
  stdout, stderr, status = Open3.capture3(
    "cargo", "test", "--manifest-path", options.fetch(:manifest), test_name, "--", "--exact"
  )
  unless status.success?
    warn stdout
    abort "focused test failed: #{test_name}: #{stderr}"
  end
end

puts "lane=#{options.fetch(:lane)} discovered=#{expected.length} executed=#{expected.length} zero_match_guard=pass"

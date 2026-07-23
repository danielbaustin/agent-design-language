#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

prepared = File.expand_path(__dir__)
guard = File.join(prepared, "validate-source-authority.rb")
fixtures = Dir.glob(File.join(prepared, "fixtures/source-authority/*.rs")).sort
abort("source-authority negative fixtures are absent") unless fixtures.length >= 6

results = fixtures.map do |fixture|
  stdout, stderr, status = Open3.capture3("ruby", guard, fixture)
  combined = stderr + stdout
  abort("guard accepted forbidden fixture #{File.basename(fixture)}") if status.success?
  abort("guard failed without authority evidence for #{File.basename(fixture)}") unless combined.include?("forbidden product-source authority")
  { fixture: File.basename(fixture), rejected: true }
end

puts JSON.generate(
  schema: "adl.wp06.source-authority-negative-proof.v1",
  fixtures: results,
  outcome: "passed"
)

#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

def values(issue, card)
  JSON.parse(File.read(".csdlc/issues/#{issue}/cards/#{card}.values.json")).dig("content", "values")
end

[5844, 5845].each do |issue|
  index = JSON.parse(File.read(".csdlc/issues/#{issue}/index.json"))
  raise "#{issue} claim not released" unless index["claim"].nil?
  revision = index.dig("design_review", "approved", "revision")
  raise "#{issue} design is not approved" unless revision
  raise "#{issue} SPP design drift" unless values(issue, "spp")["design_digest"] == revision
  raise "#{issue} VPP design drift" unless values(issue, "vpp")["design_digest"] == revision

  design = File.read(".csdlc/prepared/issues/#{issue}/design.md")
  section = design[/^## Owned Paths\n(.*?)(?=^## |\z)/m, 1]
  raise "#{issue} missing Owned Paths" unless section
  paths = section.scan(/`([^`]+)`/).flatten
  raise "#{issue} has no owned paths" if paths.empty?
  raise "#{issue} source baseline leaked into ownership" if paths.any? { |path| path.include?("v0.91.") || path.include?("TBD/") }
end

spp44 = values(5844, "spp")
vpp44 = values(5844, "vpp")
expected44 = {"elapsed_seconds" => 144_000, "total_tokens" => 740_000, "validation_seconds" => 18_000}
raise "5844 execution estimate mismatch" unless spp44["execution_estimates"] == expected44
raise "5844 validation budget mismatch" unless [vpp44["planned_validation_seconds"], vpp44["planned_validation_tokens"]] == [18_000, 60_000]

spp45 = values(5845, "spp")
vpp45 = values(5845, "vpp")
expected45 = {"elapsed_seconds" => 288_000, "total_tokens" => 700_000, "validation_seconds" => 21_600}
raise "5845 execution estimate mismatch" unless spp45["execution_estimates"] == expected45
raise "5845 validation budget mismatch" unless [vpp45["planned_validation_seconds"], vpp45["planned_validation_tokens"]] == [21_600, 80_000]

lanes = vpp45["lanes"].to_h { |lane| [lane["lane"], lane] }
expected_lanes = %w[
  wp24a-package-positive
  wp24a-package-negative
  wp24a-macos-playback
  wp24a-linux-playback
  wp24a-desktop-chromium-playback
  wp24a-ios-safari-playback
  wp24a-platform-receipt-binding
]
raise "5845 validation lane set mismatch" unless lanes.keys == expected_lanes

required_commands = %w[
  record_podcast_native_playback.sh
  record_podcast_browser_playback.mjs
  record_podcast_ios_safari_playback.sh
  validate-platform-playback-receipts.rb
]
required_commands.each do |token|
  raise "5845 missing #{token}" unless vpp45["lanes"].any? { |lane| lane["argv"].any? { |arg| arg.include?(token) } }
end

puts "PASS: #5844/#5845 second-pass semantic readiness"

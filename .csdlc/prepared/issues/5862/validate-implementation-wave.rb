#!/usr/bin/env ruby
# frozen_string_literal: true
require "json"
require "yaml"
expected = {"WP-04.01":5863,"WP-04.02":5864,"WP-04.03":5865,"WP-04.04":5866,"WP-04.05":5867,"WP-04.06":5868,"WP-04.07":5869,"WP-04.08":5870,"WP-04.09":5871,"WP-04.10":5872,"WP-04.11":5873,"WP-04.12":5874,"WP-04.13":5875,"WP-04.14":5876,"WP-04.15":5877,"WP-04.16":5878}
abort "expected sixteen children" unless expected.length == 16
all_paths = {}
expected.each do |wp, issue|
  index_path = ".csdlc/issues/#{issue}/index.json"
  abort "missing index for #{wp} ###{issue}" unless File.file?(index_path)
  index = JSON.parse(File.read(index_path))
  abort "issue mismatch for #{wp}" unless index["issue"] == issue
  abort "#{wp} design not approved" unless index.dig("design_review", "approved", "revision").to_s.match?(/\A[0-9a-f]{64}\z/)
  abort "#{wp} preparation claim remains active" unless index["claim"].nil?
  %w[sip stp spp vpp].each do |card|
    values = JSON.parse(File.read(".csdlc/issues/#{issue}/cards/#{card}.values.json"))
    abort "#{wp} #{card} not ready" unless values["status"] == "ready"
  end
  %w[srp sor].each do |card|
    values = JSON.parse(File.read(".csdlc/issues/#{issue}/cards/#{card}.values.json"))
    abort "#{wp} #{card} not truthful pre-phase" unless %w[pre_phase draft].include?(values["status"])
  end
  design = File.read(".csdlc/prepared/issues/#{issue}/design.md")
  section = design[/## Exclusive Owned Paths\n\n(.*?)\n\n## /m, 1]
  abort "#{wp} missing exact owned paths" unless section
  paths = section.scan(/`([^`]+)`/).flatten
  abort "#{wp} has no owned paths" if paths.empty?
  paths.each do |path|
    abort "path collision #{path}: #{all_paths[path]} and #{wp}" if all_paths.key?(path)
    all_paths[path] = wp
  end
end
umbrella = JSON.parse(File.read(".csdlc/issues/5862/index.json"))
abort "umbrella claim remains active" unless umbrella["claim"].nil?
gate = File.read(".csdlc/prepared/issues/5821/design.md")
expected.each do |wp, issue|
  abort "gate mapping missing #{wp} ##{issue}" unless gate.include?("| #{wp} | ##{issue} |")
end
puts "PASS: WP-04-IMP #5862, sixteen approved claim-null children, #{all_paths.length} exclusive paths"

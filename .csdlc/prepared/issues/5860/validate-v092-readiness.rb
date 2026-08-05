#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

SPRINTS = {
  5858 => [5818, 5819, 5812, 5801, 5853, 5822, 5823, 5824],
  5855 => [5800, 5820, 5795, 5821, 5832, 5837],
  5857 => [5825, 5826, 5827, 5828, 5829, 5830, 5831, 5833, 5834],
  5854 => [5835, 5836, 5838, 5839, 5840, 5844, 5845],
  5856 => [5786, 5841, 5842, 5843, 5846, 5847, 5848, 5849, 5850, 5851, 5852]
}.freeze

FORBIDDEN = {
  "placeholder design" => /Status: design required before Ready\./,
  "generic scope" => /implementation paths to be narrowed during preparation/i,
  "generic plan" => /Prepare the exact issue scope, implement the required outcome/i,
  "generic first step" => /Prepare exact scope, design, paths, and validation plan/i
}.freeze

errors = []
rows = []

SPRINTS.each do |sprint, issues|
  issues.each do |issue|
    root = ".csdlc/issues/#{issue}"
    prepared = ".csdlc/prepared/issues/#{issue}"
    index_path = "#{root}/index.json"
    design_path = "#{prepared}/design.md"
    diagram_path = "#{prepared}/diagram.mmd"

    [index_path, design_path, diagram_path].each do |path|
      errors << "##{issue}: missing #{path}" unless File.file?(path) && !File.zero?(path)
    end
    next unless File.file?(index_path) && File.file?(design_path) && File.file?(diagram_path)

    index = JSON.parse(File.read(index_path))
    corpus = [File.read(design_path)]
    statuses = {}

    %w[sip stp spp vpp srp sor].each do |card|
      rendered = "#{root}/cards/#{card}.md"
      values = "#{root}/cards/#{card}.values.json"
      unless File.file?(rendered) && File.file?(values)
        errors << "##{issue}: missing #{card} rendered or values card"
        next
      end
      JSON.parse(File.read(values))
      text = File.read(rendered)
      corpus << text if %w[sip stp spp vpp].include?(card)
      statuses[card] = text[/^Status:\s*(.+)$/, 1]
    rescue JSON::ParserError => e
      errors << "##{issue}: invalid #{card} values JSON: #{e.message}"
    end

    FORBIDDEN.each do |label, pattern|
      errors << "##{issue}: #{label}" if corpus.join("\n").match?(pattern)
    end

    approval = index["design_review"]
    approved = approval.is_a?(Hash) && approval.key?("approved")
    errors << "##{issue}: design review is not approved" unless approved
    errors << "##{issue}: active preparation claim remains" if index["claim"]

    %w[sip stp spp vpp].each do |card|
      errors << "##{issue}: #{card} status is #{statuses[card].inspect}, expected ready" unless statuses[card] == "ready"
    end
    %w[srp sor].each do |card|
      allowed = %w[pre_phase draft]
      errors << "##{issue}: #{card} status is #{statuses[card].inspect}, expected truthful pre-execution state" unless allowed.include?(statuses[card])
    end

    rows << [issue, sprint, index["phase"], approved, statuses]
  end
end

unless errors.empty?
  warn "v0.92 readiness: FAIL (#{errors.length} findings)"
  errors.each { |error| warn "- #{error}" }
  exit 1
end

puts "v0.92 readiness: PASS (#{rows.length} children across #{SPRINTS.length} sprints)"

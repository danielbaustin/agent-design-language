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
matrix_path = ARGV.each_cons(2).find { |left, _right| left == "--write-matrix" }&.last

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
    card_values = {}

    %w[sip stp spp vpp srp sor].each do |card|
      rendered = "#{root}/cards/#{card}.md"
      values = "#{root}/cards/#{card}.values.json"
      unless File.file?(rendered) && File.file?(values)
        errors << "##{issue}: missing #{card} rendered or values card"
        next
      end
      parsed = JSON.parse(File.read(values))
      card_values[card] = parsed
      text = File.read(rendered)
      corpus << text if %w[sip stp spp vpp].include?(card)
      statuses[card] = text[/^Status:\s*(.+)$/, 1]
      errors << "##{issue}: #{card} values issue mismatch" unless parsed.dig("identity", "issue") == issue
      errors << "##{issue}: #{card} values generation mismatch" unless parsed.dig("identity", "generation") == index["generation"]
      errors << "##{issue}: #{card} rendered/value status mismatch" unless parsed["status"] == statuses[card]
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

    if approved && card_values.key?("spp") && card_values.key?("vpp")
      revision = approval.dig("approved", "revision")
      spp = card_values.dig("spp", "content", "values") || {}
      vpp = card_values.dig("vpp", "content", "values") || {}
      errors << "##{issue}: SPP design digest does not match approval" unless spp["design_digest"] == revision
      errors << "##{issue}: VPP design digest does not match approval" unless vpp["design_digest"] == revision
      errors << "##{issue}: SPP/VPP diagram digest mismatch" unless !spp["diagram_digest"].to_s.empty? && spp["diagram_digest"] == vpp["diagram_digest"]
      errors << "##{issue}: SPP design reference mismatch" unless spp["design_ref"] == design_path
      errors << "##{issue}: VPP design reference mismatch" unless vpp["design_ref"] == design_path
      errors << "##{issue}: SPP diagram reference mismatch" unless spp["diagram_ref"] == diagram_path
      errors << "##{issue}: VPP diagram reference mismatch" unless vpp["diagram_ref"] == diagram_path
    end

    stp = card_values.dig("stp", "content", "values") || {}
    %w[deliverables acceptance_criteria dependencies repo_inputs non_goals].each do |field|
      errors << "##{issue}: STP #{field} is empty" if !stp[field].is_a?(Array) || stp[field].empty?
    end
    spp = card_values.dig("spp", "content", "values") || {}
    %w[steps invariants risks stop_conditions].each do |field|
      errors << "##{issue}: SPP #{field} is empty" if !spp[field].is_a?(Array) || spp[field].empty?
    end
    vpp = card_values.dig("vpp", "content", "values") || {}
    lanes = vpp["lanes"]
    if !lanes.is_a?(Array) || lanes.empty?
      errors << "##{issue}: VPP lanes are empty"
    else
      lanes.each_with_index do |lane, lane_index|
        errors << "##{issue}: VPP lane #{lane_index + 1} lacks proof role" if lane["proof_role"].to_s.strip.empty?
        errors << "##{issue}: VPP lane #{lane_index + 1} lacks acceptance mapping" if !lane["acceptance_ids"].is_a?(Array) || lane["acceptance_ids"].empty?
        errors << "##{issue}: VPP lane #{lane_index + 1} lacks argv" if !lane["argv"].is_a?(Array) || lane["argv"].empty?
      end
    end

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

if matrix_path
  lines = [
    "# v0.92 Child Execution-Readiness Matrix",
    "",
    "Generated by `.csdlc/prepared/issues/5860/validate-v092-readiness.rb` after a full passing validation.",
    "",
    "| Issue | Sprint | Phase | Design | Claim | SIP | STP | SPP | VPP | SRP | SOR |",
    "|---:|---:|---|---|---|---|---|---|---|---|---|"
  ]
  rows.sort_by(&:first).each do |issue, sprint, phase, approved, statuses|
    lines << "| ##{issue} | ##{sprint} | `#{phase}` | #{approved ? 'approved' : 'pending'} | released | `#{statuses['sip']}` | `#{statuses['stp']}` | `#{statuses['spp']}` | `#{statuses['vpp']}` | `#{statuses['srp']}` | `#{statuses['sor']}` |"
  end
  lines.concat([
    "",
    "Result: **PASS** - #{rows.length} children across #{SPRINTS.length} sprint umbrellas.",
    "",
    "`bound` with a released preparation claim is the current v2 handoff posture. The next execution session must reacquire its issue claim just in time; issue #5861 owns simplifying this state model."
  ])
  File.write(matrix_path, lines.join("\n") + "\n")
end

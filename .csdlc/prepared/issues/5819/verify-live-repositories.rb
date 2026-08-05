#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

destinations = %w[
  cognitive-sdlc-paper
  godel-hadamard-bayes-paper
  general-intelligence-paper-private
  universal-tool-schema
  agent-design-language
].map { |name| "agent-logic/#{name}" }
negative_controls = %w[danielbaustin/asksifu danielbaustin/Horust]

(destinations + negative_controls).each do |repository|
  stdout, stderr, status = Open3.capture3(
    "gh", "repo", "view", repository,
    "--json", "nameWithOwner,visibility,defaultBranchRef"
  )
  abort "live repository query failed for #{repository}: #{stderr.strip}" unless status.success?
  row = JSON.parse(stdout)
  abort "live repository identity mismatch for #{repository}" unless row["nameWithOwner"] == repository
  abort "live repository lacks default branch for #{repository}" if row.dig("defaultBranchRef", "name").to_s.empty?
end

puts "WP-02 live repository identities valid: five destinations and two negative controls"

#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

root = File.expand_path("../../../..", __dir__)
JSON.parse(File.read(File.join(root, "docs/milestones/v0.91.8/feature_preservation_crosswalk_5594.v1.json")))
puts "json ok"

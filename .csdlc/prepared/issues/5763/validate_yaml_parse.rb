#!/usr/bin/env ruby
# frozen_string_literal: true

require "yaml"

root = File.expand_path("../../../..", __dir__)
YAML.load_file(File.join(root, "docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml"))
puts "yaml ok"

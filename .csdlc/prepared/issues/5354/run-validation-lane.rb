#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb LANE") }
allowed = %w[integrated-live-demo claim-boundary-matrix complete post-merge-exact].freeze
abort("unknown validation lane: #{lane}") unless allowed.include?(lane)

warn("#5354 #{lane}: unavailable during preparation; run after the typed #5384 terminal gate and exact product-path claim amendment")
exit 1

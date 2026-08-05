#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

root = Pathname.new(__dir__).join("../../../..").cleanpath
path = root.join(".csdlc/evidence/5823/platform-matrix.json")
abort "missing platform matrix" unless path.file? && !path.zero?
matrix = JSON.parse(path.read)

%w[linux macos].each do |platform|
  row = matrix[platform] || abort("missing #{platform}")
  abort "#{platform} must be native live proof" unless row["qualification"] == "live" && row["native"] == true
  %w[runner revision command_profile_digest result_digest receipt].each do |field|
    abort "#{platform} missing #{field}" if row[field].to_s.empty?
  end
  abort "#{platform} failed" unless row["outcome"] == "passed"
end

windows = matrix["windows"] || abort("missing windows")
abort "Windows qualification invalid" unless %w[live fixture].include?(windows["qualification"])
abort "live Windows row is not native" if windows["qualification"] == "live" && windows["native"] != true
abort "fixture Windows row overclaims native proof" if windows["qualification"] == "fixture" && windows["native"] != false
%w[revision command_profile_digest result_digest receipt].each do |field|
  abort "windows missing #{field}" if windows[field].to_s.empty?
end
abort "windows failed" unless windows["outcome"] == "passed"

puts "WP-06 platform matrix valid: native Linux + macOS, Windows #{windows['qualification']}"

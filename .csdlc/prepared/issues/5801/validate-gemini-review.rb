#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

root = Pathname.new(__dir__).join("../../..").cleanpath
path = root.join(".csdlc/evidence/5801/gemini-3.1-pro-review.json")
abort "missing Gemini 3.1 Pro review packet" unless path.file? && !path.zero?
packet = JSON.parse(path.read)
abort "wrong reviewer model" unless packet["provider"] == "google" && packet["model"] == "gemini-3.1-pro"
%w[reviewed_revision prompt_digest response_digest reviewed_at].each do |field|
  abort "Gemini review missing #{field}" if packet[field].to_s.empty?
end
abort "Gemini review findings missing" unless packet["findings"].is_a?(Array)
abort "Gemini review dispositions missing" unless packet["dispositions"].is_a?(Array)
open_ids = packet["findings"].filter_map { |finding| finding["id"] if finding["actionable"] == true }
disposed = packet["dispositions"].map { |disposition| disposition["finding_id"] }
abort "undisposed Gemini findings: #{(open_ids - disposed).join(', ')}" unless (open_ids - disposed).empty?

puts "WP-02A retained Gemini 3.1 Pro review valid at #{packet['reviewed_revision']}"

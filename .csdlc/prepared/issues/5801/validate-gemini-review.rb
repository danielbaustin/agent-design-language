#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"
require "digest"
require "shellwords"

root = Pathname.new(__dir__).join("../../..").cleanpath
path = root.join(".csdlc/evidence/5801/gemini-3.1-pro-review.json")
abort "missing Gemini 3.1 Pro review packet" unless path.file? && !path.zero?
packet = JSON.parse(path.read)
abort "wrong reviewer model" unless packet["provider"] == "google" && packet["model"] == "gemini-3.1-pro"
%w[reviewed_revision prompt_path prompt_sha256 response_path response_sha256 topology_path topology_sha256 topology_blob reviewed_at].each do |field|
  abort "Gemini review missing #{field}" if packet[field].to_s.empty?
end
abort "invalid reviewed revision" unless packet["reviewed_revision"].match?(/\A[0-9a-f]{40}\z/)

%w[prompt response topology].each do |kind|
  relative = packet.fetch("#{kind}_path")
  digest = packet.fetch("#{kind}_sha256")
  abort "invalid #{kind} digest" unless digest.match?(/\A[0-9a-f]{64}\z/)
  artifact = root.join(relative).cleanpath
  abort "#{kind} artifact escapes repository" unless artifact.to_s.start_with?(root.to_s + File::SEPARATOR)
  abort "missing #{kind} artifact: #{relative}" unless artifact.file? && !artifact.zero?
  abort "#{kind} digest mismatch" unless Digest::SHA256.file(artifact).hexdigest == digest
end

topology_ref = "#{packet.fetch('reviewed_revision')}:#{packet.fetch('topology_path')}"
topology_blob = `git rev-parse #{topology_ref.shellescape} 2>/dev/null`.strip
abort "topology is absent from reviewed revision" unless $CHILD_STATUS.success?
abort "topology blob mismatch" unless topology_blob == packet["topology_blob"]
abort "Gemini review findings missing" unless packet["findings"].is_a?(Array)
abort "Gemini review dispositions missing" unless packet["dispositions"].is_a?(Array)
open_ids = packet["findings"].filter_map { |finding| finding["id"] if finding["actionable"] == true }
disposed = packet["dispositions"].map { |disposition| disposition["finding_id"] }
abort "undisposed Gemini findings: #{(open_ids - disposed).join(', ')}" unless (open_ids - disposed).empty?

puts "WP-02A retained Gemini 3.1 Pro review valid at #{packet['reviewed_revision']}"

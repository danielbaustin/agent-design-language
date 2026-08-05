#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

path = ARGV.fetch(0, ".csdlc/evidence/5832/acip-native-receipts.json")
abort "missing #{path}" unless File.file?(path)
packet = JSON.parse(File.read(path))
abort "wrong schema" unless packet["schema"] == "adl.acip_native_receipts.v1"
head, status = Open3.capture2("git", "rev-parse", "HEAD")
abort "cannot resolve HEAD" unless status.success?
head = head.strip
abort "stale packet" unless packet["source_revision"] == head
receipts = Array(packet["receipts"])
%w[macos linux windows].each do |platform|
  receipt = receipts.find { |entry| entry["platform"] == platform }
  abort "missing #{platform} receipt" unless receipt
  abort "stale #{platform} receipt" unless receipt["source_revision"] == head
  abort "missing runner identity" if receipt["runner_identity"].to_s.empty?
  abort "missing exact argv" if Array(receipt["argv"]).empty?
  %w[runtime_binary_sha256 proto_sha256 catalog_sha256 transcript_sha256 output_sha256].each do |field|
    abort "invalid #{platform} #{field}" unless receipt[field].to_s.match?(/\A[0-9a-f]{64}\z/)
  end
  abort "#{platform} ran no exchanges" unless receipt["successful_exchanges"].to_i.positive?
  abort "#{platform} ran no negative cases" unless receipt["negative_cases"].to_i.positive?
  %w[production_guardian rustls_wss authenticated_bidirectional protobuf_json_parity reconnect_backpressure replay_denied denied_access].each do |field|
    abort "#{platform} did not prove #{field}" unless receipt[field] == true
  end
end
puts "PASS: exact-revision production ACIP/WSS receipts on macOS, Linux, and native Windows"

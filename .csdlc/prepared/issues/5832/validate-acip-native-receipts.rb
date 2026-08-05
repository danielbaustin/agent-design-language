#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

SHA256 = /\A[0-9a-f]{64}\z/
ISSUE = 5832
REQUIRED_ASSERTIONS = %w[production_guardian rustls_wss authenticated_bidirectional protobuf_json_parity reconnect_backpressure replay_denied denied_access].freeze

def checked_file(path, digest, label, allow_empty: false)
  abort "#{label} path must be issue-local" unless path.to_s.start_with?(".csdlc/evidence/#{ISSUE}/")
  abort "missing #{label}: #{path}" unless File.file?(path)
  abort "empty #{label}: #{path}" if !allow_empty && File.zero?(path)
  abort "invalid #{label} digest" unless digest.to_s.match?(SHA256)
  abort "#{label} digest mismatch" unless Digest::SHA256.file(path).hexdigest == digest
end

path = ARGV.fetch(0, ".csdlc/evidence/#{ISSUE}/acip-native-receipts.json")
abort "missing #{path}" unless File.file?(path)
packet = JSON.parse(File.read(path))
abort "wrong schema" unless packet["schema"] == "adl.acip_native_receipts.v2"
head, status = Open3.capture2("git", "rev-parse", "HEAD")
abort "cannot resolve HEAD" unless status.success?
head = head.strip
abort "stale packet" unless packet["source_revision"] == head
receipts = Array(packet["receipts"])
abort "platform denominator drift" unless receipts.map { |entry| entry["platform"] }.sort == %w[linux macos windows]
receipts.each do |receipt|
  platform = receipt.fetch("platform")
  abort "stale #{platform} receipt" unless receipt["source_revision"] == head
  runner = receipt.fetch("runner")
  %w[provider run_id os arch].each { |field| abort "missing #{platform} runner #{field}" if runner[field].to_s.empty? }
  abort "invalid runner identity" unless runner["identity_sha256"].to_s.match?(SHA256)
  command = receipt.fetch("command")
  abort "wrong #{platform} producer command" unless Array(command["argv"]) == ["bash", "adl/tools/validate_v092_acip_wss.sh"]
  abort "#{platform} producer failed" unless command["exit_code"] == 0
  checked_file(command["stdout_path"], command["stdout_sha256"], "#{platform} stdout")
  checked_file(command["stderr_path"], command["stderr_sha256"], "#{platform} stderr", allow_empty: true)
  abort "#{platform} ran no exchanges" unless receipt["successful_exchanges"].to_i.positive?
  abort "#{platform} ran no negative cases" unless receipt["negative_cases"].to_i.positive?
  artifacts = Array(receipt["artifacts"])
  abort "#{platform} artifacts missing" if artifacts.empty?
  artifacts.each { |artifact| checked_file(artifact.fetch("path"), artifact.fetch("sha256"), "#{platform} artifact") }
  assertions = Array(receipt["assertions"])
  abort "#{platform} assertion denominator drift" unless assertions.map { |entry| entry["name"] }.sort == REQUIRED_ASSERTIONS.sort
  assertions.each do |assertion|
    abort "#{platform} did not prove #{assertion['name']}" unless assertion["result"] == "passed"
    checked_file(assertion["evidence_path"], assertion["evidence_sha256"], "#{platform} #{assertion['name']}")
  end
end
abort "native runner runs are not distinct" unless receipts.map { |r| r.dig("runner", "run_id") }.uniq.length == 3
puts "PASS: exact-head production ACIP/WSS logs, artifacts, and assertions on macOS, Linux, and native Windows"

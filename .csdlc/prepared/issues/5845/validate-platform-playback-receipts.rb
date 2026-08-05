#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.realpath
PLATFORM_ROOT = ROOT.join(".csdlc/evidence/5845/platform").cleanpath
EXPECTED = {
  "macos-native" => ["adl/tools/record_podcast_native_playback.sh", "--platform", "macos"],
  "linux-native" => ["adl/tools/record_podcast_native_playback.sh", "--platform", "linux"],
  "desktop-chromium" => ["adl/tools/record_podcast_browser_playback.mjs", "--browser", "chromium"],
  "ios-safari-device" => [
    "adl/tools/record_podcast_ios_safari_playback.sh",
    "--device-id-hash-env", "ADL_IOS_DEVICE_ID_SHA256",
    "--episode-url-env", "ADL_IOS_EPISODE_URL"
  ]
}.freeze

def fail!(message)
  warn("FAIL: #{message}")
  exit(1)
end

def repo_file!(relative, label, under: nil)
  fail!("#{label} must be a nonempty repo-relative path") unless relative.is_a?(String) && !relative.empty?
  path = ROOT.join(relative).cleanpath
  fail!("#{label} escapes repository") unless path.to_s.start_with?(ROOT.to_s + File::SEPARATOR)
  fail!("#{label} is outside #{under.relative_path_from(ROOT)}") if under && !path.to_s.start_with?(under.to_s + File::SEPARATOR)
  fail!("#{label} does not exist: #{relative}") unless path.file?
  path
end

source_arg = ARGV.shift
receipt_dir = ARGV.shift || ".csdlc/evidence/5845/platform"
if source_arg == "--source-sha-from-git-head"
  source_sha, stderr, status = Open3.capture3("git", "rev-parse", "HEAD", chdir: ROOT.to_s)
  fail!("cannot resolve git HEAD: #{stderr.strip}") unless status.success?
  source_sha = source_sha.strip
else
  source_sha = source_arg
end
fail!("usage: #{$PROGRAM_NAME} <40-hex-source-sha|--source-sha-from-git-head> [receipt-dir]") unless source_sha&.match?(/\A[0-9a-f]{40}\z/) && ARGV.empty?
head_sha, head_stderr, head_status = Open3.capture3("git", "rev-parse", "HEAD", chdir: ROOT.to_s)
fail!("cannot resolve exact candidate HEAD: #{head_stderr.strip}") unless head_status.success?
fail!("source SHA must equal exact candidate HEAD") unless source_sha == head_sha.strip

dir = ROOT.join(receipt_dir).cleanpath
fail!("receipt directory must be within platform evidence root") unless dir == PLATFORM_ROOT || dir.to_s.start_with?(PLATFORM_ROOT.to_s + File::SEPARATOR)
receipts = Dir.glob(dir.join("*/receipt.json").to_s).sort
fail!("expected exactly four platform receipts") unless receipts.length == EXPECTED.length

seen = {}
receipts.each do |receipt_path|
  envelope = JSON.parse(File.read(receipt_path))
  fail!("#{receipt_path}: wrong schema") unless envelope["schema"] == "adl.podcast_playback_receipt.v1"
  payload = envelope["payload"]
  fail!("#{receipt_path}: payload must be an object") unless payload.is_a?(Hash)
  digest = Digest::SHA256.hexdigest(JSON.generate(payload))
  fail!("#{receipt_path}: payload digest mismatch") unless envelope["payload_sha256"] == digest

  platform = payload["platform_id"]
  expected_argv = EXPECTED[platform]
  fail!("#{receipt_path}: unexpected or duplicate platform #{platform.inspect}") unless expected_argv && !seen[platform]
  seen[platform] = true
  fail!("#{receipt_path}: source SHA mismatch") unless payload["source_sha"] == source_sha

  argv = payload["argv"]
  fail!("#{receipt_path}: argv must be a nonempty string array") unless argv.is_a?(Array) && argv.all? { |value| value.is_a?(String) && !value.empty? }
  fail!("#{receipt_path}: argv contains an unresolved placeholder") if argv.any? { |value| value.include?("<") || value.include?(">") }
  expected_argv.each { |token| fail!("#{receipt_path}: argv missing #{token}") unless argv.include?(token) }

  runner = payload["runner"]
  required_runner = %w[kind os os_version architecture identity]
  fail!("#{receipt_path}: incomplete runner identity") unless runner.is_a?(Hash) && required_runner.all? { |key| runner[key].is_a?(String) && !runner[key].empty? }

  device = payload["device"]
  if platform == "desktop-chromium"
    fail!("#{receipt_path}: desktop browser identity missing") unless device.is_a?(Hash) && %w[browser version].all? { |key| device[key].is_a?(String) && !device[key].empty? }
  elsif platform == "ios-safari-device"
    required_device = %w[device_id_hash model os_version browser version]
    fail!("#{receipt_path}: iOS device identity missing") unless device.is_a?(Hash) && required_device.all? { |key| device[key].is_a?(String) && !device[key].empty? }
    fail!("#{receipt_path}: iOS device identity must be SHA-256") unless device["device_id_hash"].match?(/\A[0-9a-f]{64}\z/)
  else
    fail!("#{receipt_path}: native device must be null") unless device.nil?
  end

  media = repo_file!(payload["media_path"], "media_path")
  capture = repo_file!(payload["capture_path"], "capture_path", under: PLATFORM_ROOT)
  fail!("#{receipt_path}: media digest mismatch") unless payload["media_sha256"] == Digest::SHA256.file(media).hexdigest
  fail!("#{receipt_path}: capture digest mismatch") unless payload["capture_sha256"] == Digest::SHA256.file(capture).hexdigest

  fail!("#{receipt_path}: invalid timestamps") unless payload["started_at"].is_a?(String) && !payload["started_at"].empty? && payload["ended_at"].is_a?(String) && !payload["ended_at"].empty?
  fail!("#{receipt_path}: duration must be positive") unless payload["duration_seconds"].is_a?(Numeric) && payload["duration_seconds"].positive?
  result = payload["result"]
  required_results = %w[passed playback_started playback_completed audible controls_operable]
  fail!("#{receipt_path}: playback result is not fully proving") unless result.is_a?(Hash) && required_results.all? { |key| result[key] == true }
end

fail!("platform set mismatch") unless seen.keys.sort == EXPECTED.keys.sort
puts("PASS: four source-bound native/browser playback receipts verified for #{source_sha}")

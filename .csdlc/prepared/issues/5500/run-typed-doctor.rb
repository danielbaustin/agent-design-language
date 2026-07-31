#!/usr/bin/env ruby
# frozen_string_literal: true

require "open3"
require "pathname"
require "json"

root = Pathname.new(__dir__).join("../../../..").cleanpath
common_dir, common_error, common_status = Open3.capture3("git", "rev-parse", "--path-format=absolute", "--git-common-dir", chdir: root.to_s)
abort("cannot resolve shared Git directory: #{common_error}") unless common_status.success?

repository = Pathname.new(common_dir.strip).parent
binary_dir = repository.join(".adl", "bin", "csdlc-v2")
installer = binary_dir.join("csdlc-install")
doctor = binary_dir.join("csdlc-doctor")
receipt = binary_dir.join("install-receipt.json")
abort("stable typed v2 installer is unavailable") unless installer.executable?
abort("stable typed v2 doctor is unavailable") unless doctor.executable?
abort("stable typed v2 install receipt is unavailable") unless receipt.file? && receipt.readable?
receipt_record = JSON.parse(receipt.read)
abort("unexpected typed v2 install receipt schema") unless receipt_record["schema"] == "csdlc.install_receipt.v1"
installed_names = receipt_record.fetch("binaries").map { |entry| entry.fetch("name") }
abort("typed v2 receipt omits required router or doctor") unless %w[csdlc-install csdlc-doctor].all? { |name| installed_names.include?(name) }

resolve_out, resolve_error, resolve_status = Open3.capture3(
  installer.to_s,
  "resolve",
  "--repo", root.to_s,
  "--issue", "5500",
  chdir: root.to_s
)
abort("typed generation resolution failed: #{resolve_out}#{resolve_error}") unless resolve_status.success? && resolve_out.include?("v2")

exec doctor.to_s, "--repo", ".", "--issue", "5500", chdir: root.to_s

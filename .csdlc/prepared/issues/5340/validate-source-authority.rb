#!/usr/bin/env ruby
# frozen_string_literal: true

require "pathname"
require "fileutils"

root = File.realpath(File.expand_path("../../../..", __dir__))
prepared = File.join(root, ".csdlc/prepared/issues/5340")
crate = File.join(root, "adl-v2/crates/adl-engine")
manifest = File.join(prepared, "source-authority-validator/Cargo.toml")
files = if ARGV.empty?
          product = %w[src examples].flat_map { |name| Dir.glob(File.join(crate, name, "**/*.rs")) }
          build_script = File.join(crate, "build.rs")
          product << build_script if File.file?(build_script)
          product
        else
          ARGV.map { |path| File.realpath(path) }
        end
abort("engine product source is absent") if files.empty?

fast_root = ENV.fetch("ADL_WP5340_FAST_ROOT", "/Volumes/FastWork/adl-wp-5340")
FileUtils.mkdir_p(fast_root)
fast_root = File.realpath(fast_root)
abort("FastWork root escaped /Volumes/FastWork") unless fast_root.start_with?("/Volumes/FastWork/")

paths = {
  "CARGO_HOME" => ENV.fetch("ADL_WP5340_CARGO_HOME", File.join(fast_root, "cargo-home")),
  "CARGO_TARGET_DIR" => File.join(fast_root, "source-authority-target"),
  "SCCACHE_DIR" => File.join(fast_root, "sccache"),
  "TMPDIR" => File.join(fast_root, "tmp")
}
environment = paths.merge(
  "CARGO_INCREMENTAL" => "0",
  "CARGO_NET_OFFLINE" => "true",
  "CARGO_BUILD_RUSTC_WRAPPER" => nil,
  "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER" => nil,
  "RUSTC_WRAPPER" => nil,
  "RUSTC_WORKSPACE_WRAPPER" => nil
)
paths.each_value { |path| FileUtils.mkdir_p(path) }
paths.each do |name, path|
  canonical = File.realpath(path)
  abort("#{name} escaped /Volumes/FastWork") unless canonical.start_with?("/Volumes/FastWork/")
end

exec(
  environment,
  "cargo", "run", "--quiet", "--offline", "--locked",
  "--manifest-path", manifest, "--", *files.sort
)

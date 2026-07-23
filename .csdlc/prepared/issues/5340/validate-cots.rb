#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

metadata_path = ARGV.fetch(0)
metadata = JSON.parse(File.read(metadata_path))
root = File.realpath(File.expand_path("../../../..", __dir__))
engine = File.realpath(File.join(root, "adl-v2/crates/adl-engine"))
compiler = File.realpath(File.join(root, "adl-v2/crates/adl-compiler"))
language = File.realpath(File.join(root, "adl-v2/crates/adl-language"))
packages = metadata.fetch("packages")
engine_package = packages.find { |package| File.realpath(File.dirname(package.fetch("manifest_path"))) == engine }
abort("engine package missing from cargo metadata") unless engine_package

expected = {
  "adl-compiler" => nil,
  "adl-language" => nil,
  "serde" => "=1.0.229",
  "serde_json" => "=1.0.151",
  "sha2" => "=0.10.9",
  "hex" => "=0.4.3"
}.freeze
crates_io = "registry+https://github.com/rust-lang/crates.io-index"
lock_path = File.join(engine, "Cargo.lock")
lock_reader = <<~'PYTHON'
  import json
  import sys
  import tomllib

  with open(sys.argv[1], "rb") as source:
      lock = tomllib.load(source)
  print(json.dumps(lock.get("package", []), sort_keys=True, separators=(",", ":")))
PYTHON
lock_output, lock_error, lock_status = Open3.capture3("python3", "-c", lock_reader, lock_path)
abort("Cargo.lock TOML validation failed: #{lock_error}") unless lock_status.success?
lock_packages = JSON.parse(lock_output)
locked_registry = lambda do |name, version|
  matches = lock_packages.select do |package|
    package.fetch("name") == name && package.fetch("version") == version && package["source"] == crates_io
  end
  abort("#{name} #{version} has alternate or missing Cargo.lock entries") unless matches.length == 1
  locked = matches.first
  checksum = locked["checksum"]
  abort("#{name} #{version} Cargo.lock checksum is absent or malformed") unless checksum.is_a?(String) && checksum.match?(/\A[0-9a-f]{64}\z/)
  locked
end
dependencies = engine_package.fetch("dependencies")
observed = dependencies.map { |dependency| dependency.fetch("name") }.sort
abort("direct dependencies differ from exact reviewed COTS set: #{observed.join(', ')}") unless observed == expected.keys.sort

dependencies.each do |dependency|
  name = dependency.fetch("name")
  if name == "adl-compiler"
    path = dependency.fetch("path")
    abort("adl-compiler is not the canonical path dependency") unless path && File.realpath(path) == compiler
    abort("adl-compiler unexpectedly has a registry or Git source") unless dependency["source"].nil?
    abort("adl-compiler must be a normal product dependency") unless dependency["kind"].nil?
  elsif name == "adl-language"
    path = dependency.fetch("path")
    abort("adl-language is not the canonical test-only path dependency") unless path && File.realpath(path) == language
    abort("adl-language unexpectedly has a registry or Git source") unless dependency["source"].nil?
    abort("adl-language must remain test-only") unless dependency.fetch("kind") == "dev"
  else
    abort("#{name} requirement is not exact #{expected.fetch(name)}") unless dependency.fetch("req") == expected.fetch(name)
    source = dependency.fetch("source")
    abort("#{name} is not sourced from the approved crates.io registry") unless source == crates_io
    version = expected.fetch(name).delete_prefix("=")
    named = packages.select { |package| package.fetch("name") == name }
    abort("#{name} has alternate or missing resolved versions") unless named.length == 1 && named.first.fetch("version") == version
    package = named.first
    abort("#{name} resolved from a non-crates.io source") unless package.fetch("source") == crates_io
    locked_registry.call(name, version)
  end
end

allowed_paths = [engine, compiler, language]
packages.each do |package|
  source = package["source"]
  abort("Git dependency is forbidden: #{package.fetch('name')}") if source&.start_with?("git+")
  if source.nil?
    path = File.realpath(File.dirname(package.fetch("manifest_path")))
    abort("unreviewed local package path: #{path}") unless allowed_paths.include?(path)
  elsif source == crates_io
    locked_registry.call(package.fetch("name"), package.fetch("version"))
  else
    abort("unreviewed package source: #{source}")
  end
end

forbidden = /(?:\A|[-_])(runtime|csdlc|tokio|async|smol|reqwest|hyper|rustls|openssl|aws|azure|gcp|sqlx|diesel|petgraph|rand|getrandom|cron|schedule|scheduler|retry|workflow)(?:\z|[-_])/i
forbidden_packages = packages.map { |package| package.fetch("name") }.select { |name| name.match?(forbidden) }
abort("forbidden dependency families: #{forbidden_packages.sort.join(', ')}") unless forbidden_packages.empty?

resolved_direct = dependencies.reject { |dependency| %w[adl-compiler adl-language].include?(dependency.fetch("name")) }.to_h do |dependency|
  name = dependency.fetch("name")
  version = expected.fetch(name).delete_prefix("=")
  package = packages.find { |item| item.fetch("name") == name && item.fetch("version") == version }
  locked = locked_registry.call(name, version)
  [name, { version: version, source: package.fetch("source"), checksum: locked.fetch("checksum") }]
end
puts JSON.generate(
  schema: "adl.wp06.cots-proof.v1",
  compiler_path: "adl-v2/crates/adl-compiler",
  test_language_path: "adl-v2/crates/adl-language",
  registry_dependencies: resolved_direct,
  forbidden_dependencies: forbidden_packages,
  outcome: "passed"
)

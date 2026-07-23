#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

root = File.realpath(File.expand_path("../../../..", __dir__))
crate = File.join(root, "adl-v2/crates/adl-engine")
abort("engine crate is absent") unless File.directory?(crate)

implementation_roots = %w[src examples].map { |name| File.join(crate, name) }
test_roots = %w[tests fixtures benches scripts].map { |name| File.join(crate, name) }
implementation = implementation_roots.flat_map { |dir| Dir.glob(File.join(dir, "**/*.rs")) }
build_script = File.join(crate, "build.rs")
implementation << build_script if File.file?(build_script)
tests = test_roots.flat_map { |dir| Dir.glob(File.join(dir, "**/*")) }.select { |path| File.file?(path) }

count = lambda do |paths|
  paths.uniq.sum { |path| File.foreach(path).count }
end
implementation_lines = count.call(implementation)
test_fixture_lines = count.call(tests)
abort("implementation LoC #{implementation_lines} exceeds 4000") if implementation_lines > 4000
abort("test/fixture LoC #{test_fixture_lines} exceeds 3500") if test_fixture_lines > 3500

classified = (implementation + tests).map { |path| File.realpath(path) }.to_h { |path| [path, true] }
code_extensions = %w[.rs .sh .rb .py]
unbudgeted = Dir.glob(File.join(crate, "**/*")).select do |path|
  File.file?(path) && code_extensions.include?(File.extname(path)) && !classified[File.realpath(path)]
end
abort("unbudgeted code surface: #{unbudgeted.map { |path| Pathname.new(path).relative_path_from(Pathname.new(root)) }.join(', ')}") unless unbudgeted.empty?

puts JSON.generate(
  schema: "adl.wp06.loc-proof.v1",
  implementation_lines: implementation_lines,
  implementation_ceiling: 4000,
  test_fixture_lines: test_fixture_lines,
  test_fixture_ceiling: 3500,
  unbudgeted_code: [],
  outcome: "passed"
)

#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "timeout"

ROOT = File.expand_path("../../../..", __dir__)
CRATE = File.join(ROOT, "adl-v2", "crates", "adl-cli")
MANIFEST = File.join(CRATE, "Cargo.toml")
PINS = {"clap"=>"4.6.4", "serde"=>"1.0.229", "serde_json"=>"1.0.151", "tempfile"=>"3.27.0", "fs2"=>"0.4.3", "sha2"=>"0.10.9"}.freeze
FORBIDDEN = /\A(?:aws-|reqwest|hyper|tokio|sqlx|rusqlite|ratatui|crossterm|libloading|adl-runtime|adl-runtime-kernel|csdlc-v2)\z/

def capture!(seconds, *argv)
  output = +""
  status = nil
  Timeout.timeout(seconds) do
    Open3.popen2e(*argv, pgroup: true) do |_stdin, stream, wait|
      output = stream.read
      status = wait.value
    end
  end
  abort("command failed: #{argv.join(' ')}\n#{output}") unless status.success?
  output
rescue Timeout::Error
  Process.kill("TERM", -$CHILD_STATUS.pid) if $CHILD_STATUS
  abort("command exceeded #{seconds}s: #{argv.join(' ')}")
end

abort("missing #{MANIFEST}") unless File.file?(MANIFEST)
expected_head = ENV.fetch("ADL_WP10_EXPECTED_HEAD")
head = capture!(10, "git", "rev-parse", "HEAD").strip
abort("expected head #{expected_head}, observed #{head}") unless head == expected_head
abort("exact-revision proof requires a clean worktree") unless system("git", "diff", "--quiet") && system("git", "diff", "--cached", "--quiet")

metadata = JSON.parse(capture!(120, "cargo", "metadata", "--locked", "--format-version", "1", "--manifest-path", MANIFEST))
packages = metadata.fetch("packages")
PINS.each do |name, version|
  observed = packages.select { |pkg| pkg["name"] == name }.map { |pkg| pkg["version"] }.uniq
  abort("#{name} closure #{observed.inspect}, expected #{version}") unless observed == [version]
end
forbidden = packages.map { |pkg| pkg["name"] }.grep(FORBIDDEN)
abort("forbidden dependencies: #{forbidden.sort.uniq.join(', ')}") unless forbidden.empty?

rust_files = Dir.glob(File.join(CRATE, "**", "*.rs")).sort
implementation = rust_files.select { |path| path.include?("/src/") }
tests = rust_files - implementation
line_count = ->(paths) { paths.sum { |path| File.foreach(path).count } }
impl_lines = line_count.call(implementation)
test_lines = line_count.call(tests)
modules = implementation.to_h { |path| [path.delete_prefix("#{ROOT}/"), File.foreach(path).count] }
selector_lines = modules.select { |path, _| path.include?("selector") }.values.sum
abort("implementation LoC #{impl_lines} exceeds 2500") if impl_lines > 2500
abort("test LoC #{test_lines} exceeds 2500") if test_lines > 2500
abort("selector LoC #{selector_lines} exceeds 800") if selector_lines > 800
oversized = modules.select { |_path, lines| lines > 1000 }
abort("modules exceed 1000 lines: #{oversized.inspect}") unless oversized.empty?

listing = capture!(120, "cargo", "test", "--locked", "--manifest-path", MANIFEST, "--all-targets", "--", "--list")
test_count = listing.lines.count { |line| line.match?(/: test\s*$/) }
abort("no tests discovered") if test_count.zero?
capture!(600, "cargo", "test", "--locked", "--manifest-path", MANIFEST, "--all-targets")

puts JSON.generate(schema: "adl.v0918.wp10_implementation_gate.v1", status: "pass", head: head,
  implementation_lines: impl_lines, test_lines: test_lines, selector_lines: selector_lines,
  largest_module_lines: modules.values.max || 0, test_count: test_count)

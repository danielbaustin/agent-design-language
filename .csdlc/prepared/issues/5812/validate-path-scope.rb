#!/usr/bin/env ruby
# frozen_string_literal: true

allowed = [
  "adl/src/csm_freedom_gate.rs",
  ".csdlc/issues/5812/",
  ".csdlc/prepared/issues/5812/",
  ".csdlc/evidence/5812/",
  ".csdlc/locks/5812.lock"
]
changed = `git diff --name-only origin/main...HEAD`.lines.map(&:strip).reject(&:empty?)
unauthorized = changed.reject { |path| allowed.any? { |prefix| path == prefix || path.start_with?(prefix) } }
abort "out-of-scope paths: #{unauthorized.join(', ')}" unless unauthorized.empty?
abort "Cargo metadata changed" if changed.any? { |path| File.basename(path).match?(/\ACargo\.(toml|lock)\z/) }
abort "Google Drive surface changed" if changed.any? { |path| path.downcase.include?("google") || path.include?("gws") }

puts "Issue 5812 path scope valid: #{changed.length} changed paths"

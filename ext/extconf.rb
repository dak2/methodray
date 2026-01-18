# frozen_string_literal: true

require 'mkmf'

# Check if precompiled binaries exist (platform gem)
lib_dir = File.expand_path('../lib/methodray', __dir__)
cli_binary = File.join(lib_dir, 'methodray-cli')
extension = Dir.glob(File.join(lib_dir, 'methodray.{bundle,so,dll}')).first

if File.exist?(cli_binary) && extension
  # Precompiled gem - create dummy Makefile
  File.write('Makefile', "install:\n\t@echo 'Using precompiled binaries'\n")
  exit 0
end

# Source gem - build from Rust
require 'rb_sys/mkmf'

# Build the Ruby FFI extension
create_rust_makefile('methodray/methodray')

# Project root directory
project_root = File.expand_path('..', __dir__)
target_dir = File.join(project_root, 'target', 'release')

# Append CLI binary build to the generated Makefile
File.open('Makefile', 'a') do |f|
  f.puts <<~MAKEFILE

    # Build CLI binary after the extension
    install: install-cli

    install-cli:
    \t@echo "Building CLI binary..."
    \tcd #{__dir__} && cargo build --release --features cli --bin methodray-cli
    \t@mkdir -p $(DESTDIR)$(sitearchdir)
    \tcp #{target_dir}/methodray-cli $(DESTDIR)$(sitearchdir)/methodray-cli
    \tchmod 755 $(DESTDIR)$(sitearchdir)/methodray-cli
  MAKEFILE
end

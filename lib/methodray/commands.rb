# frozen_string_literal: true

module MethodRay
  module Commands
    COMMANDS_DIR = __dir__

    class << self
      def help
        puts <<~HELP
          MethodRay v#{MethodRay::VERSION} - A fast static analysis tool for Ruby methods.

          Usage:
            methodray help                    # Show this help
            methodray version                 # Show version
            methodray check [FILE] [OPTIONS]  # Type check a Ruby file
            methodray watch FILE              # Watch file for changes and auto-check
            methodray clear-cache             # Clear RBS method cache

          Examples:
            methodray check app/models/user.rb
            methodray watch app/models/user.rb
        HELP
      end

      def version
        puts "MethodRay v#{MethodRay::VERSION}"
      end

      def check(args)
        exec_rust_cli('check', args)
      end

      def watch(args)
        exec_rust_cli('watch', args)
      end

      def clear_cache(args)
        exec_rust_cli('clear-cache', args)
      end

      private

      def exec_rust_cli(command, args)
        binary_path = find_rust_binary

        unless binary_path
          warn 'Error: CLI binary not found.'
          warn ''
          warn 'For development, build with:'
          warn '  cd rust && cargo build --release --bin methodray --features cli'
          warn ''
          warn 'If installed via gem, this might be a platform compatibility issue.'
          warn 'Please report at: https://github.com/dak2/method-ray/issues'
          exit 1
        end

        exec(binary_path, command, *args)
      end

      def find_rust_binary
        # Platform-specific binary name
        cli_binary = Gem.win_platform? ? 'methodray-cli.exe' : 'methodray-cli'
        legacy_binary = Gem.win_platform? ? 'methodray.exe' : 'methodray'

        candidates = [
          # CLI binary built during gem install (lib/methodray directory)
          File.expand_path(cli_binary, COMMANDS_DIR),
          # Development: target/release (project root)
          File.expand_path("../../target/release/#{cli_binary}", COMMANDS_DIR),
          # Development: rust/target/release (legacy standalone binary)
          File.expand_path("../../rust/target/release/#{legacy_binary}", COMMANDS_DIR)
        ]

        candidates.find { |path| File.executable?(path) }
      end
    end
  end
end

# frozen_string_literal: true

module MethodRay
  module Commands
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
        binary_name = Gem.win_platform? ? 'methodray.exe' : 'methodray'

        # Determine Ruby platform identifier
        ruby_platform = detect_ruby_platform

        candidates = [
          # Precompiled binary in gem (platform-specific directory)
          File.expand_path("../#{ruby_platform}/#{binary_name}", __dir__),
          # Precompiled binary in gem (lib/methodray directory)
          File.expand_path("../#{binary_name}", __dir__),
          # Development: rust/target/release
          File.expand_path("../../../rust/target/release/#{binary_name}", __dir__),
          # Development: target/release (from project root)
          File.expand_path("../../../target/release/#{binary_name}", __dir__)
        ]

        candidates.find { |path| File.executable?(path) }
      end

      def detect_ruby_platform
        cpu = case RbConfig::CONFIG['host_cpu']
              when /x86_64|amd64/ then 'x86_64'
              when /arm64|aarch64/ then 'arm64'
              else RbConfig::CONFIG['host_cpu']
              end

        os = case RbConfig::CONFIG['host_os']
             when /darwin/ then 'darwin'
             when /linux/ then 'linux'
             when /mingw|mswin/ then 'mingw'
             else RbConfig::CONFIG['host_os']
             end

        "#{cpu}-#{os}"
      end
    end
  end
end

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
        exec_rust_cli("check", args)
      end

      def watch(args)
        exec_rust_cli("watch", args)
      end

      def clear_cache(args)
        exec_rust_cli("clear-cache", args)
      end

      private

      def exec_rust_cli(command, args)
        binary_path = find_rust_binary

        unless binary_path
          $stderr.puts "Error: Rust binary not found. Please build it first:"
          $stderr.puts "  cargo build --release --bin methodray --features cli"
          exit 1
        end

        exec(binary_path, command, *args)
      end

      def find_rust_binary
        candidates = [
          File.expand_path("../../target/release/methodray", __dir__),
          File.expand_path("../../../target/release/methodray", __dir__),
          File.expand_path("../../ext/target/release/methodray", __dir__),
        ]

        candidates.find { |path| File.executable?(path) }
      end
    end
  end
end

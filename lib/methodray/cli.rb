module MethodRay
  class CLI
    def self.start(args)
      command = args.shift

      case command
      when "help", "--help", "-h", nil
        show_help
      when "version", "--version", "-v"
        puts "MethodRay v#{MethodRay::VERSION}"
      else
        puts "Unknown command: #{command}"
        show_help
        exit 1
      end
    end

    def self.show_help
      puts <<~HELP
        MethodRay v#{MethodRay::VERSION} - A fast static analysis tool for Ruby methods.

        Usage:
          methodray help          # Show this help
          methodray version       # Show version
      HELP
    end
  end
end

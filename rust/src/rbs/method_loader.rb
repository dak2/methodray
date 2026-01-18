# frozen_string_literal: true

require 'rbs'

# TODO: use ruby-rbs crate when available
# https://github.com/ruby/rbs/pull/2808
module Rbs
  class MethodLoader
    TARGET_CLASSES = %w[
      String Integer Float Array Hash Symbol
      TrueClass FalseClass NilClass
      Range Regexp Struct Enumerable
    ].freeze

    def initialize
      loader = ::RBS::EnvironmentLoader.new
      env = ::RBS::Environment.from_loader(loader).resolve_type_names
      @builder = ::RBS::DefinitionBuilder.new(env: env)
    end

    def load_methods
      results = []

      TARGET_CLASSES.each do |class_name|
        type_name = ::RBS::TypeName.new(
          name: class_name.to_sym,
          namespace: ::RBS::Namespace.root
        )

        definition = @builder.build_instance(type_name)

        definition.methods.each do |method_name, method_def|
          method_type = method_def.method_types.first
          next unless method_type

          return_type = method_type.type.return_type.to_s

          results << {
            receiver_class: class_name,
            method_name: method_name.to_s,
            return_type: return_type
          }
        end
      rescue StandardError => e
        warn "Skipped #{class_name}: #{e.message}" if ENV['DEBUG']
      end

      results
    end
  end
end

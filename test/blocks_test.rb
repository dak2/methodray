# frozen_string_literal: true

require 'test_helper'

class BlocksTest < Minitest::Test
  def setup
    @analyzer = MethodRay::Analyzer.new('test.rb')
  end

  # Basic block with single parameter - code parses correctly
  def test_block_with_single_parameter
    code = <<~RUBY
      items = [1, 2, 3]
      items.each { |x| x }
    RUBY
    result = @analyzer.infer_types(code)
    # Outer variable should be tracked
    assert_includes result, 'items: Array'
  end

  # Block with multiple parameters
  def test_block_with_multiple_parameters
    code = <<~RUBY
      data = {a: 1, b: 2}
      data.each { |k, v| k }
    RUBY
    result = @analyzer.infer_types(code)
    assert_includes result, 'data: Hash'
  end

  # do...end syntax
  def test_block_do_end_syntax
    code = <<~RUBY
      items = [1, 2, 3]
      items.each do |item|
        item
      end
    RUBY
    result = @analyzer.infer_types(code)
    assert_includes result, 'items: Array'
  end

  # Block accessing outer scope variable
  def test_block_accesses_outer_scope
    code = <<~RUBY
      prefix = "Item: "
      items = [1, 2, 3]
      items.each { |x| prefix.upcase }
    RUBY
    result = @analyzer.infer_types(code)
    assert_includes result, 'prefix: String'
    assert_includes result, 'items: Array'
  end

  # map with block
  def test_map_with_block
    code = <<~RUBY
      items = [1, 2, 3]
      items.map { |x| x }
    RUBY
    result = @analyzer.infer_types(code)
    assert_includes result, 'items: Array'
  end

  # select with block
  def test_select_with_block
    code = <<~RUBY
      items = [1, 2, 3]
      items.select { |x| x }
    RUBY
    result = @analyzer.infer_types(code)
    assert_includes result, 'items: Array'
  end

  # Nested blocks
  def test_nested_blocks
    code = <<~RUBY
      outer = [[1, 2], [3, 4]]
      outer.each do |arr|
        arr.each { |x| x }
      end
    RUBY
    result = @analyzer.infer_types(code)
    assert_includes result, 'outer: Array'
  end

  # Block inside method definition
  def test_block_inside_method
    code = <<~RUBY
      class Processor
        def process
          items = [1, 2, 3]
          items.each { |item| item }
        end
      end
    RUBY
    result = @analyzer.infer_types(code)
    # Should parse without error
    refute_nil result
  end

  # Block with optional parameter and default value
  def test_block_with_optional_parameter
    code = <<~RUBY
      items = [1, 2, 3]
      items.each { |x = "default"| x.upcase }
    RUBY
    result = @analyzer.infer_types(code)
    assert_includes result, 'items: Array'
  end

  # Block with rest parameter
  def test_block_with_rest_parameter
    code = <<~RUBY
      items = [1, 2, 3]
      items.each { |*args| args.first }
    RUBY
    result = @analyzer.infer_types(code)
    assert_includes result, 'items: Array'
  end

  # Outer variable modified before block
  def test_outer_variable_before_block
    code = <<~RUBY
      message = "Hello"
      items = [1, 2, 3]
      items.each { |x| message }
    RUBY
    result = @analyzer.infer_types(code)
    assert_includes result, 'message: String'
    assert_includes result, 'items: Array'
  end
end

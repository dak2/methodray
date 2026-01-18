# frozen_string_literal: true

require 'test_helper'

class MethodRayTest < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::MethodRay::VERSION
  end

  def test_analyzer_can_be_created
    analyzer = MethodRay::Analyzer.new('.')
    assert_instance_of MethodRay::Analyzer, analyzer
  end

  def test_analyzer_version_method
    analyzer = MethodRay::Analyzer.new('.')
    assert_equal '0.1.0', analyzer.version
  end

  def test_infer_types_string_literal
    analyzer = MethodRay::Analyzer.new('test.rb')
    result = analyzer.infer_types('x = "hello"')
    assert_includes result, 'x: String'
  end

  def test_infer_types_integer_literal
    analyzer = MethodRay::Analyzer.new('test.rb')
    result = analyzer.infer_types('x = 42')
    assert_includes result, 'x: Integer'
  end

  def test_infer_types_method_chain
    analyzer = MethodRay::Analyzer.new('test.rb')
    result = analyzer.infer_types('x = "hello".upcase')
    assert_includes result, 'x: String'
  end

  def test_infer_types_multiple_vars
    analyzer = MethodRay::Analyzer.new('test.rb')
    result = analyzer.infer_types(<<~RUBY)
      x = "hello"
      y = 123
    RUBY
    assert_includes result, 'x: String'
    assert_includes result, 'y: Integer'
  end
end

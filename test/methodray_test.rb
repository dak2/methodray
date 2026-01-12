require "test_helper"

class MethodRayTest < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::MethodRay::VERSION
  end

  def test_analyzer_can_be_created
    analyzer = MethodRay::Analyzer.new(".")
    assert_instance_of MethodRay::Analyzer, analyzer
  end

  def test_analyzer_version_method
    analyzer = MethodRay::Analyzer.new(".")
    assert_equal "0.1.0", analyzer.version
  end
end

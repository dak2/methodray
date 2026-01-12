require_relative "methodray/version"
require_relative "methodray/methodray"  # ネイティブ拡張

module MethodRay
  class Error < StandardError; end

  def self.version
    VERSION
  end
end

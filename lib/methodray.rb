# frozen_string_literal: true

require 'rbs'
require_relative 'methodray/version'
require_relative 'methodray/methodray' # ネイティブ拡張

module MethodRay
  class Error < StandardError; end
end

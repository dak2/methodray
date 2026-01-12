require_relative "lib/methodray/version"

Gem::Specification.new do |spec|
  spec.name          = "methodray"
  spec.version       = MethodRay::VERSION
  spec.authors       = ["dak2"]
  spec.email         = [""]

  spec.summary       = "MethodRay is a fast static analysis tool for Ruby methods."
  spec.description   = <<~EOD
    Basically, MethodRay performs a type analysis of unannotated Ruby code.

    This gem analyzes method definitions statically to prevent `NoMethodError` at runtime.
  EOD
  spec.homepage      = "https://github.com/dak2/methodray"
  spec.license       = "MIT"
  spec.required_ruby_version = ">= 3.4"

  spec.files = Dir[
    "lib/**/*.rb",
    "exe/*",
    "ext/**/*",
    "src/**/*.rs",
    "Cargo.toml",
    "Cargo.lock",
    "README.md"
  ]

  spec.bindir        = "exe"
  spec.executables   = ["methodray"]
  spec.require_paths = ["lib"]
  spec.extensions    = ["ext/extconf.rb"]

  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency "rb_sys", "~> 0.9"
  spec.add_development_dependency "rake-compiler", "~> 1.2"
  spec.add_development_dependency "minitest", "~> 5.0"
end

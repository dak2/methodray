require "bundler/gem_tasks"
require "rake/extensiontask"
require "rake/testtask"

Rake::ExtensionTask.new("methodray") do |ext|
  ext.ext_dir = "ext"
  ext.lib_dir = "lib/methodray"
end

# Minitestテストタスク
Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/*_test.rb"]
end

# デフォルトタスク: コンパイル → テスト
task default: [:compile, :test]

# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("batch_api.gemspec")

RbSys::ExtensionTask.new("batch_api", GEMSPEC) do |ext|
  ext.lib_dir = "lib/batch_api"
end

task default: %i[compile spec]

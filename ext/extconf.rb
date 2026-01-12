require "mkmf"
require "rb_sys/mkmf"

# ルートのCargo.tomlを使用するように環境変数で指定
ENV["CARGO_MANIFEST_PATH"] = File.expand_path("../../Cargo.toml", __dir__)

create_rust_makefile("methodray/methodray")

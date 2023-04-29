export RUSTFLAGS="-Zcodegen-backend=$(readlink -f $(dirname $0))/target/debug/librustc_codegen_xlang.so"
cargo $@
RUSTFLAGS='-C target-cpu=native' cargo build --release
cbindgen --config cbindgen.toml --output tiny_skia.h
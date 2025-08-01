This crate provides very primitive C bindings to the tiny-skia library. Note that this library is not meant for production use cases, it mainly exists to be able to include tiny-skia in the Blend2D
benchmark harness.

In order to build the bindings, you simply need to run the `run.sh` script (note that by default, the script passes the `target-cpu=native` flag to the compiler.
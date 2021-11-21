# rust-bindgen

> https://doc.rust-lang.org/nomicon/ffi.html

## Usage

Automatically generates Rust FFI bindings to C:
```sh
$ bindgen -o src/utils/bindings.rs src/utils/c_utils.c --no-layout-tests
```

Build & Run
```sh
$ gcc -fPIC -shared -o src/utils/libutils.so src/utils/c_utils.c
$ LD_LIBRARY_PATH="./src/utils" RUSTFLAGS='-L ./src/utils' cargo run
```

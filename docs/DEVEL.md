# Developer Guide

## Table of Contents

- [Deploy in WASM](#deploy-in-wasm)
- [Profile](#profile)
- [See also](#see-also)

## Deploy in WASM

### Setup

```sh
rustup target add wasm32-unknown-unknown
```

### Build for local execution

```sh
cargo clean

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-dir dist --out-name flappy \
    --target web target/wasm32-unknown-unknown/release/flappy.wasm

GITHUB_HOSTING_PATH=. ./scripts/distmore && cd dist && basic-http-server
```

### Build for deployment

```sh
cargo clean

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-dir dist --out-name flappy \
    --target web target/wasm32-unknown-unknown/release/flappy.wasm

./scripts/distmore
```

## Profile

**NOTE**: Varying by operating system, "dynamic_linking" may not work to run
the binary alone

### Profile Runtime

#### By Tracy

```sh
# ... tracy running on the back ...
cargo clean
cargo run --release --features bevy/trace_tracy
```

#### By direct execution(ex. third party profiler)

```sh
cargo build --release

cp -r ./assets ./target/release/
./target/release/flappy
```

### Profile Build-time

#### By Timings

```sh
cargo clean
cargo build --release --timings
```

## See also

[Profiling in Bevy](https://github.com/bevyengine/bevy/blob/main/docs/profiling.md)

[Timings](https://doc.rust-lang.org/nightly/cargo/reference/timings.html)

[Tracy Web Viewer](https://ui.perfetto.dev/)

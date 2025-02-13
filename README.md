# Flappy Bird

Flappy Bird clone powered by Bevy 0.15

## Start

```sh
$ cargo install
$ cargo run --features "bevy/dynamic_linking"
```

## Deploy in wasm

```sh
$ rustup target add wasm32-unknown-unknown
```

```sh
$ cargo build --release --target wasm32-unknown-unknown

$ wasm-bindgen --out-dir dist --out-name flappy \
    --target web target/wasm32-unknown-unknown/release/flappy.wasm

$ ./scripts/distmore.sh
$ cd dist && basic-http-server
```

## References

[Flappy Rust](https://www.rustfinity.com/tutorials/flappy-rust/)

[Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/platforms/wasm.html)

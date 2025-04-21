# Profile

**NOTE**: Varying by operating system, "dynamic_linking" may not work to run
the binary alone

## Profile Runtime

### By Tracy

```sh
# ... tracy running on the back ...
cargo clean
cargo run --release --features bevy/trace_tracy
```

### By direct execution(ex. third party profiler)

```sh
cargo build --release

cp -r ./assets ./target/release/
./target/release/flappy
```

## Profile Build-time

### By Timings

```sh
cargo clean
cargo build --release --timings
```

## See also

[Profiling in Bevy](https://github.com/bevyengine/bevy/blob/main/docs/profiling.md)

[Timings](https://doc.rust-lang.org/nightly/cargo/reference/timings.html)

[Tracy Web Viewer](https://ui.perfetto.dev/)

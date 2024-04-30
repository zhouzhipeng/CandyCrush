```bash
# run editor with hot reloading
RUSTFLAGS="-C prefer-dynamic=yes" cargo run --package editor --no-default-features --features dylib --profile dev-hot-reload
```


```bash
# rebuild game 
RUSTFLAGS="-C prefer-dynamic=yes" cargo build --package game_dylib  --no-default-features --features dylib-engine --profile dev-hot-reload
```
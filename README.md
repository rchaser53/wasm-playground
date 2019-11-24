
1. cargo install cargo-wasi (※1)
2. install wasmtime (※2)

```
cargo wasi build --release
wasmtime ../wasm-playground/target/wasm32-wasi/release/wasm_playground.wasm --invoke greet "nya-n"
```

※1 https://github.com/bytecodealliance/cargo-wasi
※2 curl https://wasmtime.dev/install.sh -sSf | bash


1. cargo install cargo-wasi (※1)
2. install wasmtime (※2)

```
./target/release/wasmtime ../wasm-playground/target/wasm32-wasi/release/wasm_playground.wasm --invoke hello "nya-n"
```

※1 https://github.com/bytecodealliance/cargo-wasi

※2 11/23/19 now, failed to install install.sh, so build directly. (https://github.com/bytecodealliance/wasmtime)
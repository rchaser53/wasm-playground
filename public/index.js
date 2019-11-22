const rust = import('./wasm/wasm_playground_bg.wasm');

rust
  .then(m => m.greet('World!'))
  .catch(console.error);

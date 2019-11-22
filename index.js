const rust = import('./pkg');

rust
  .then(m => m.greet("aaaa"))
  .catch(console.error);

use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn greet(name: &str) {
    println!("Hello, {}! in wasm", name);
}

#[cfg(all(target_os="macos"))]
pub fn greet(name: &str) {
    println!("Hello, {}! in standard environment", name);
}
  
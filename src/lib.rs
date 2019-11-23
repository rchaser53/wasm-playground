use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

/* cannot use JavaScript function like alert in wasi */
// #[wasm_bindgen]
// pub fn hello(name: &str) -> String {
//     format!("Hello, {}!", name)
// }

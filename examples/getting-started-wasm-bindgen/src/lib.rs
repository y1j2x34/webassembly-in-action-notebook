extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    // 调用 JavaScript 函数（window.alert）
    pub fn alert(s: &str);
}
// 定义一个可以被 JavaScript 调用的函数
#[wasm_bindgen]
pub fn say(what: &str) {
    alert(&format!("Hello, {}!", what));
}
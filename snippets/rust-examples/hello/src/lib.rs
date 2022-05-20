use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/def.js")]
extern "C" {
    fn reload() -> ();

    type User;

    #[wasm_bindgen(constructor)]
    fn new() -> User;

    #[wasm_bindgen(method, getter)]
    fn id(this: &User) -> u32;

    #[wasm_bindgen(method, setter)]
    fn set_id(this: &User, id: u32) -> User;

    #[wasm_bindgen(method)]
    fn say(this: &User);
}
#[wasm_bindgen(start)]
pub fn run() {
    let u = User::new();
    u.set_id("123asd");
    u.say();
    reload();
}

#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}
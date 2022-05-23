use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/def.js")]
extern "C" {
    fn reload() -> Option<i32>;

    type User;

    #[wasm_bindgen(constructor)]
    fn new(id: String) -> User;

    #[wasm_bindgen(method, getter)]
    fn id(this: &User) -> String;

    #[wasm_bindgen(method, setter)]
    fn set_id(this: &User, id: String) -> User;

    #[wasm_bindgen(method)]
    fn say(this: &User);
}
#[wasm_bindgen(start)]
pub fn run() {
    let u = User::new(String::from("456789"));
    u.set_id(String::from("123asd"));
    u.say();
    reload();
}

#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}
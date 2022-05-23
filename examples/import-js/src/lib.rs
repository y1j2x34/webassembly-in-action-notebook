use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js-module.js")]
extern "C" {
    fn reload() -> JsValue;

    type User; // class User

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
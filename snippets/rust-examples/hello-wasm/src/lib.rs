
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern  {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    (&h: expr $(,$t: expr)*) => {
        log(
            &format_args!(
                $h $(,$t)*
            ).to_string()
        )
    }
}

pub fn say_hello_using_web_sys(name: &str) {
    use web_sys::console;
    console::log_1(&"Hello using web-sys: ".into(), name);
}

pub fn say_hello_using_macro(name: &str) {
    console_log!("Hello using macro: ", name);
}

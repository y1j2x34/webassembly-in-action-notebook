
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern  {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    () => ();
    ($($t: tt)*) => {
        log(
            &format_args!(
                $($t)*
            ).to_string()
        )
    }
}

#[wasm_bindgen]
pub fn say_hello_using_web_sys(name: &str) {
    use web_sys::console;
    console::log_2(&"Hello using web-sys: ".into(), &name.into());
}

#[wasm_bindgen]
pub fn say_hello_using_macro(name: &str) {
    console_log!("Hello using macro {}", name);
}

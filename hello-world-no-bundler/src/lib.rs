extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn alert(message: String);
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: String);
}

#[wasm_bindgen(js_name=alertHelloWorld)]
pub fn alert_hello_world () {
    alert(format!("Hello World"));
}

#[wasm_bindgen(js_name=logHelloWorld)]
pub fn log_hello_world () {
    log(format!("Hello World"));
}
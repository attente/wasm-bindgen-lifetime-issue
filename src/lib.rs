extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct S {
}

#[wasm_bindgen]
impl S {
    pub fn f() -> Result<(), &'static str> {
        Err("error")
    }
}

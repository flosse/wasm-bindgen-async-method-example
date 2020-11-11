use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Wrapper {
    inner: RefCell<InnerCore>,
}

pub struct InnerCore {
    val: u32,
}

// TODO:
// Using `#[wasm_bindgen]` does not compile:
//
//     the trait `IntoWasmAbi` is not implemented for `std::rc::Rc<Wrapper>`
//
// #[wasm_bindgen]
impl Wrapper {
    //#[wasm_bindgen(constructor)]
    pub fn new() -> Rc<Wrapper> {
        Rc::new(Self {
            inner: RefCell::new(InnerCore { val: 33 }),
        })
    }

    pub async fn method_one(self: Rc<Wrapper>, _: u32) -> Result<JsValue, JsValue> {
        todo!()
    }

    pub async fn method_two(self: Rc<Wrapper>, _: String) -> Result<JsValue, JsValue> {
        todo!()
    }
}

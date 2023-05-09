mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn diff(original: JsValue, current: JsValue) -> JsValue {
    let original = serde_wasm_bindgen::from_value(original).expect("original is not valid JSON");
    let current = serde_wasm_bindgen::from_value(current).expect("current is not valid JSON");
    let patch = json_patch::diff(&original, &current);

    serde_wasm_bindgen::to_value(&patch).expect("patch is not valid JSON")
}

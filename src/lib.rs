use unicode_segmentation::UnicodeSegmentation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn grapheme_clusters(text: &str) -> Vec<JsValue> {
    text.graphemes(true).map(JsValue::from).collect()
}

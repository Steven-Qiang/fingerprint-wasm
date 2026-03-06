use crate::utils::browser::{is_edge_html, is_trident};
use wasm_bindgen::JsValue;

/**
 * IE and Edge don't allow accessing indexedDB in private mode, therefore IE and Edge will have
 * different visitor identifier in normal and private modes.
 */
pub fn get_indexed_db() -> Result<JsValue, JsValue> {
    if is_trident() || is_edge_html() {
        return Ok(JsValue::undefined());
    }

    let window = web_sys::window().unwrap();

    // try..catch because some in situations `window.indexedDB` is exposed but throws a
    // SecurityError if you try to access it
    let result = js_sys::Reflect::get(&window, &JsValue::from_str("indexedDB"));

    match result {
        Ok(_) => Ok(JsValue::from_bool(true)),
        Err(_) => Ok(JsValue::from_bool(true)), // SecurityError when referencing it means it exists
    }
}

use wasm_bindgen::JsValue;

/**
 * sessionStorage is similar to localStorage but the data is cleared when the page session ends.
 * https://bugzilla.mozilla.org/show_bug.cgi?id=781447
 */
pub fn get_session_storage() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();

    // try..catch because some in situations `window.sessionStorage` is exposed but throws a
    // SecurityError if you try to access it
    let result = js_sys::Reflect::get(&window, &JsValue::from_str("sessionStorage"));

    match result {
        Ok(_) => Ok(JsValue::from_bool(true)),
        Err(_) => Ok(JsValue::from_bool(true)), // SecurityError when referencing it means it exists
    }
}

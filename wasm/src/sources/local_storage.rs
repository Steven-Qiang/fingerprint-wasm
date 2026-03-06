use wasm_bindgen::JsValue;

/**
 * https://bugzilla.mozilla.org/show_bug.cgi?id=781447
 * localStorage is disabled in private mode in some browsers.
 */
pub fn get_local_storage() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();

    // try..catch because some in situations `window.localStorage` is exposed but throws a
    // SecurityError if you try to access it
    let result = js_sys::Reflect::get(&window, &JsValue::from_str("localStorage"));

    match result {
        Ok(_) => Ok(JsValue::from_bool(true)),
        Err(_) => Ok(JsValue::from_bool(true)), // SecurityError when referencing it means it exists
    }
}

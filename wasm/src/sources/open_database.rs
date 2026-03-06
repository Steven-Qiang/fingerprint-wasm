use wasm_bindgen::JsValue;

/**
 * openDatabase is a method that allows web pages to access Web SQL databases.
 * It's a non-standard feature that is supported only in some browsers.
 */
pub fn get_open_database() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();

    match js_sys::Reflect::get(&window, &JsValue::from_str("openDatabase")) {
        Ok(value) => Ok(JsValue::from_bool(
            !value.is_undefined() && !value.is_null(),
        )),
        _ => Ok(JsValue::from_bool(false)),
    }
}

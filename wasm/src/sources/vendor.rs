use wasm_bindgen::JsValue;

/**
 * navigator.vendor returns the name of the browser vendor.
 * It's a non-standard property that is not supported in all browsers.
 */
pub fn get_vendor() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();

    let vendor = js_sys::Reflect::get(&navigator, &JsValue::from_str("vendor"))
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_default();

    Ok(JsValue::from_str(&vendor))
}

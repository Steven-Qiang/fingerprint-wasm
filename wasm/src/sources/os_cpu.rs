use wasm_bindgen::JsValue;

/**
 * navigator.oscpu is a property that returns a string that represents the current operating
 * system. It's available only in Firefox and related browsers.
 */
pub fn get_os_cpu() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();

    match js_sys::Reflect::get(&navigator, &JsValue::from_str("oscpu")) {
        Ok(value) if !value.is_undefined() => Ok(value),
        _ => Ok(JsValue::null()),
    }
}

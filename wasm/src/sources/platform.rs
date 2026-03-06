use wasm_bindgen::JsValue;

use crate::utils::browser::{is_desktop_webkit, is_ipad, is_webkit};

/**
 * navigator.platform returns a string representing the platform of the browser.
 * In iOS 13+, Safari on iPad in desktop mode returns "MacIntel" instead of "iPad".
 * This function detects such cases and returns the correct platform.
 */
pub fn get_platform() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let platform = navigator.platform().unwrap_or_else(|_| "".to_string());

    if platform == "MacIntel" {
        if is_webkit() && !is_desktop_webkit() {
            return Ok(JsValue::from_str(if is_ipad() { "iPad" } else { "iPhone" }));
        }
    }

    Ok(JsValue::from_str(&platform))
}

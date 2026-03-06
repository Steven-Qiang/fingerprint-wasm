use wasm_bindgen::JsValue;

use crate::utils::browser::{is_gecko, is_gecko_143_or_newer};

/**
 * A version of the entropy source with stabilization to make it suitable for static
 * fingerprinting. hardwareConcurrency is tiered in Firefox 143+ private mode, so we apply the
 * same tiering logic.
 *
 * @see https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency
 */
pub fn get_hardware_concurrency() -> Result<JsValue, JsValue> {
    let value = get_unstable_hardware_concurrency();

    if let Some(v) = value {
        if is_gecko() && is_gecko_143_or_newer() {
            let tiered = if v >= 8 { 8 } else { 4 };
            return Ok(JsValue::from_f64(tiered as f64));
        }
        return Ok(JsValue::from_f64(v as f64));
    }

    Ok(JsValue::undefined())
}

fn get_unstable_hardware_concurrency() -> Option<i32> {
    let window = web_sys::window()?;
    let navigator = window.navigator();
    let value = navigator.hardware_concurrency();

    if value.is_nan() {
        return None;
    }

    Some(value as i32)
}

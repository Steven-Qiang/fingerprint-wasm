use crate::utils::browser::{is_safari_webkit, is_webkit, is_webkit_616_or_newer};
use wasm_bindgen::JsValue;

/**
 * A version of the entropy source with stabilization to make it suitable for static
 * fingerprinting. The window resolution is always the document size in private mode of Safari
 * 17, so the window resolution is not used in Safari 17.
 */
pub fn get_screen_resolution() -> Result<JsValue, JsValue> {
    if is_webkit() && is_webkit_616_or_newer() && is_safari_webkit() {
        return Ok(JsValue::undefined());
    }

    get_unstable_screen_resolution()
}

/**
 * A version of the entropy source without stabilization.
 *
 * Warning for package users:
 * This function is out of Semantic Versioning, i.e. can change unexpectedly. Usage is at your
 * own risk.
 */
fn get_unstable_screen_resolution() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let screen = window.screen().unwrap();

    let width = parse_dimension(js_sys::Reflect::get(&screen, &JsValue::from_str("width")).ok());
    let height = parse_dimension(js_sys::Reflect::get(&screen, &JsValue::from_str("height")).ok());

    let mut dimensions = vec![width, height];
    dimensions.sort_by(|a, b| {
        let a_val = a.as_f64().unwrap_or(0.0);
        let b_val = b.as_f64().unwrap_or(0.0);
        b_val
            .partial_cmp(&a_val)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let result = js_sys::Array::new();
    result.push(&dimensions[0]);
    result.push(&dimensions[1]);

    result.sort().reverse();

    Ok(JsValue::from(result))
}

/**
 * Some browsers return screen resolution as strings, e.g. "1200", instead of a number, e.g.
 * 1200. I suspect it's done by certain plugins that randomize browser properties to prevent
 * fingerprinting. Some browsers even return screen resolution as not numbers.
 */
fn parse_dimension(value: Option<JsValue>) -> JsValue {
    match value {
        Some(v) => {
            if v.is_null() || v.is_undefined() {
                return JsValue::null();
            }

            let int_value = if let Some(num) = v.as_f64() {
                if num.is_nan() {
                    return JsValue::null();
                }
                num as i32
            } else if let Some(s) = v.as_string() {
                match s.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => return JsValue::null(),
                }
            } else {
                return JsValue::null();
            };

            JsValue::from(int_value)
        }
        None => JsValue::null(),
    }
}

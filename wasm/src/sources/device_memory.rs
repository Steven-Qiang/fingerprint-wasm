use wasm_bindgen::JsValue;

/**
 * deviceMemory doesn't follow the same fingerprinting protection rules in Samsung Internet as
 * in other browsers. Samsung Internet returns a fake value (4) in normal mode and the real
 * value in secret mode. So we return undefined in Samsung Internet to avoid fingerprinting.
 * @see https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deviceMemory
 */
pub fn get_device_memory() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();

    let device_memory = js_sys::Reflect::get(
        &JsValue::from(navigator),
        &JsValue::from_str("deviceMemory"),
    );

    match device_memory {
        Ok(value) => {
            if value.is_null() || value.is_undefined() {
                return Ok(JsValue::undefined());
            }

            let float_value = if let Some(num) = value.as_f64() {
                num
            } else if let Some(s) = value.as_string() {
                match s.parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => return Ok(JsValue::undefined()),
                }
            } else {
                return Ok(JsValue::undefined());
            };

            if float_value.is_nan() {
                return Ok(JsValue::undefined());
            }

            Ok(JsValue::from_f64(float_value))
        }
        Err(_) => Ok(JsValue::undefined()),
    }
}

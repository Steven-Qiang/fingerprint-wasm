use wasm_bindgen::JsValue;

/**
 * @see https://www.w3.org/TR/mediaqueries-5/#dynamic-range
 */
pub fn is_hdr() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();

    if does_match(&window, "high")? {
        return Ok(JsValue::from_bool(true));
    }

    if does_match(&window, "standard")? {
        return Ok(JsValue::from_bool(false));
    }

    Ok(JsValue::undefined())
}

fn does_match(window: &web_sys::Window, value: &str) -> Result<bool, JsValue> {
    match window.match_media(&format!("(dynamic-range: {})", value)) {
        Ok(Some(match_media)) => Ok(match_media.matches()),
        _ => Ok(false),
    }
}

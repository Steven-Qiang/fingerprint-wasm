use wasm_bindgen::JsValue;

/**
 * @see https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion
 */
pub fn is_motion_reduced() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();

    if does_match(&window, "reduce")? {
        return Ok(JsValue::from_bool(true));
    }

    if does_match(&window, "no-preference")? {
        return Ok(JsValue::from_bool(false));
    }

    Ok(JsValue::undefined())
}

fn does_match(window: &web_sys::Window, value: &str) -> Result<bool, JsValue> {
    match window.match_media(&format!("(prefers-reduced-motion: {})", value)) {
        Ok(Some(match_media)) => Ok(match_media.matches()),
        _ => Ok(false),
    }
}

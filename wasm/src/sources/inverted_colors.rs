use wasm_bindgen::JsValue;

/**
 * @see https://developer.mozilla.org/en-US/docs/Web/CSS/@media/inverted-colors
 */
pub fn are_colors_inverted() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();

    if does_match(&window, "inverted")? {
        return Ok(JsValue::from_bool(true));
    }

    if does_match(&window, "none")? {
        return Ok(JsValue::from_bool(false));
    }

    Ok(JsValue::null())
}

fn does_match(window: &web_sys::Window, value: &str) -> Result<bool, JsValue> {
    match window.match_media(&format!("(inverted-colors: {})", value)) {
        Ok(Some(match_media)) => Ok(match_media.matches()),
        _ => Ok(false),
    }
}

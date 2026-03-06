use wasm_bindgen::JsValue;

/**
 * @see https://developer.mozilla.org/en-US/docs/Web/CSS/@media/color-gamut
 */
pub fn get_color_gamut() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();

    let gamuts = ["rec2020", "p3", "srgb"];

    for gamut in gamuts {
        if let Ok(Some(match_media)) = window.match_media(&format!("(color-gamut: {})", gamut)) {
            if match_media.matches() {
                return Ok(JsValue::from_str(gamut));
            }
        }
    }

    Ok(JsValue::undefined())
}

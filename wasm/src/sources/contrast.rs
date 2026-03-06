use wasm_bindgen::JsValue;

const CONTRAST_PREFERENCE_LESS: i32 = -1;
const CONTRAST_PREFERENCE_NONE: i32 = 0;
const CONTRAST_PREFERENCE_MORE: i32 = 1;
const CONTRAST_PREFERENCE_FORCED_COLORS: i32 = 10;

/**
 * @see https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-contrast
 */
pub fn get_contrast() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();

    if does_match(&window, "no-preference")? {
        return Ok(JsValue::from(CONTRAST_PREFERENCE_NONE));
    }

    if does_match(&window, "high")? || does_match(&window, "more")? {
        return Ok(JsValue::from(CONTRAST_PREFERENCE_MORE));
    }

    if does_match(&window, "low")? || does_match(&window, "less")? {
        return Ok(JsValue::from(CONTRAST_PREFERENCE_LESS));
    }

    if does_match(&window, "forced")? {
        return Ok(JsValue::from(CONTRAST_PREFERENCE_FORCED_COLORS));
    }

    Ok(JsValue::undefined())
}

fn does_match(window: &web_sys::Window, value: &str) -> Result<bool, JsValue> {
    match window.match_media(&format!("(prefers-contrast: {})", value)) {
        Ok(Some(match_media)) => Ok(match_media.matches()),
        _ => Ok(false),
    }
}

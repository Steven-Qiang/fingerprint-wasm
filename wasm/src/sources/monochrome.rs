use wasm_bindgen::JsValue;

/**
 * @see https://developer.mozilla.org/en-US/docs/Web/CSS/@media/monochrome
 */
pub fn get_monochrome_depth() -> Result<JsValue, JsValue> {
    if !is_monochrome_supported() {
        return Ok(JsValue::undefined());
    }

    let value;
    let mut low = 0;
    let mut high = 100;

    while (high - low) > 1 {
        let mid = (low + high) / 2;
        if is_monochrome_at_least(mid) {
            low = mid;
        } else {
            high = mid;
        }
    }

    value = low;

    Ok(JsValue::from(value))
}

fn is_monochrome_supported() -> bool {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return false,
    };

    let media_query = "(min-monochrome: 0)";

    if let Ok(mql) = window.match_media(media_query) {
        if let Some(mql) = mql {
            return mql.matches();
        }
    }

    false
}

fn is_monochrome_at_least(value: i32) -> bool {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return false,
    };

    let media_query = format!("(min-monochrome: {})", value);

    if let Ok(mql) = window.match_media(&media_query) {
        if let Some(mql) = mql {
            return mql.matches();
        }
    }

    false
}

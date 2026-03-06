use wasm_bindgen::JsValue;

/**
 * navigator.cookieEnabled cannot detect custom or nuanced cookie blocking configurations. For
 * example, when blocking cookies via the Advanced Privacy Settings in IE9, it always returns
 * true. And there have been issues in the past with site-specific exceptions. Don't rely on it.
 *
 * @see https://github.com/Modernizr/Modernizr/blob/master/feature-detects/cookies.js Taken from here
 */
pub fn are_cookies_enabled() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // try..catch because some in situations `document.cookie` is exposed but throws a
    // SecurityError if you try to access it; e.g. documents created from data URIs
    // or in sandboxed iframes (depending on flags/context)
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // Create cookie
        let _ = js_sys::Reflect::set(
            &document,
            &JsValue::from_str("cookie"),
            &JsValue::from_str("cookietest=1; SameSite=Strict;"),
        );

        let cookies = js_sys::Reflect::get(&document, &JsValue::from_str("cookie"))
            .unwrap_or(JsValue::from_str(""));
        let result = cookies
            .as_string()
            .unwrap_or_default()
            .contains("cookietest=");

        // Delete cookie
        let _ = js_sys::Reflect::set(
            &document,
            &JsValue::from_str("cookie"),
            &JsValue::from_str(
                "cookietest=1; SameSite=Strict; expires=Thu, 01-Jan-1970 00:00:01 GMT",
            ),
        );

        result
    }));

    match result {
        Ok(enabled) => Ok(JsValue::from_bool(enabled)),
        Err(_) => Ok(JsValue::from_bool(false)),
    }
}

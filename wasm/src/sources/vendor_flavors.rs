use wasm_bindgen::JsValue;

/**
 * Vendor flavors detection.
 * Returns an array of browser vendor-specific global objects that are present.
 * These objects are used to identify specific browser flavors and versions.
 */
pub fn get_vendor_flavors() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let mut flavors = Vec::new();

    // Check for browser-specific global variables
    let keys = [
        // Blink and some browsers on iOS
        "chrome",
        // Safari on macOS
        "safari",
        // Chrome on iOS (checked in 85 on 13 and 87 on 14)
        "__crWeb",
        "__gCrWeb",
        // Yandex Browser on iOS, macOS and Android (checked in 21.2 on iOS 14, macOS and Android)
        "yandex",
        // Yandex Browser on iOS (checked in 21.2 on 14)
        "__yb",
        "__ybro",
        // Firefox on iOS (checked in 32 on 14)
        "__firefox__",
        // Edge on iOS (checked in 46 on 14)
        "__edgeTrackingPreventionStatistics",
        "webkit",
        // Opera Touch on iOS (checked in 2.6 on 14)
        "oprt",
        // Samsung Internet on Android (checked in 11.1)
        "samsungAr",
        // UC Browser on Android (checked in 12.10 and 13.0)
        "ucweb",
        "UCShellJava",
        // Puffin on Android (checked in 9.0)
        "puffinDevice",
    ];

    for key in &keys {
        if let Ok(value) = js_sys::Reflect::get(&window, &JsValue::from_str(key)) {
            if !value.is_null() && !value.is_undefined() {
                if value.is_object() {
                    flavors.push(key.to_string());
                }
            }
        }
    }

    // Sort the flavors
    flavors.sort();

    // Convert to JavaScript array
    let result = js_sys::Array::new();
    for flavor in flavors {
        result.push(&JsValue::from_str(&flavor));
    }

    Ok(JsValue::from(result))
}

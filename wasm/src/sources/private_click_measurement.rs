use wasm_bindgen::{JsCast, JsValue};

// Checks whether the Safari's Privacy Preserving Ad Measurement setting is on.
// The setting is on when the value is not undefined.
// A.k.a. private click measurement, privacy-preserving ad attribution.
pub fn get_private_click_measurement() -> Result<JsValue, JsValue> {
    match inner_get_private_click_measurement() {
        Some(value) => Ok(JsValue::from_str(&value)),
        None => Ok(JsValue::undefined()),
    }
}

// Internal implementation
fn inner_get_private_click_measurement() -> Option<String> {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            // Create a link element
            if let Some(link) = document.create_element("a").ok() {
                // Check for attributionSourceId or attributionsourceid
                let source_id =
                    match js_sys::Reflect::get(&link, &JsValue::from_str("attributionSourceId")) {
                        Ok(value) if !value.is_undefined() => value,
                        _ => match js_sys::Reflect::get(
                            &link,
                            &JsValue::from_str("attributionsourceid"),
                        ) {
                            Ok(value) if !value.is_undefined() => value,
                            _ => return None,
                        },
                    };

                // Convert to string
                if let Some(source_id_str) = source_id.as_string() {
                    return Some(source_id_str);
                } else {
                    // Try to convert using toString()
                    if let Ok(to_string) =
                        js_sys::Reflect::get(&source_id, &JsValue::from_str("toString"))
                    {
                        if to_string.is_function() {
                            match to_string.dyn_into::<js_sys::Function>() {
                                Ok(func) => match func.call0(&source_id) {
                                    Ok(result) => {
                                        if let Some(result_str) = result.as_string() {
                                            return Some(result_str);
                                        }
                                    }
                                    Err(_) => {}
                                },
                                Err(_) => {}
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

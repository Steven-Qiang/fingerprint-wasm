use wasm_bindgen::JsValue;

// Returns whether the PDF viewer is enabled
pub fn is_pdf_viewer_enabled() -> Result<JsValue, JsValue> {
    match inner_is_pdf_viewer_enabled() {
        Some(value) => Ok(JsValue::from(value)),
        None => Ok(JsValue::undefined()),
    }
}

// Internal implementation
fn inner_is_pdf_viewer_enabled() -> Option<bool> {
    if let Some(window) = web_sys::window() {
        let navigator = window.navigator();
        match js_sys::Reflect::get(&navigator, &JsValue::from_str("pdfViewerEnabled")) {
            Ok(value) => {
                if let Some(bool_value) = value.as_bool() {
                    return Some(bool_value);
                }
            }
            Err(_) => {
                // Property doesn't exist
            }
        }
    }
    None
}

use wasm_bindgen::JsValue;

pub fn get_color_depth(_ctx: &crate::sources::SourceContext) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let screen = window.screen().unwrap();
    let color_depth = screen.color_depth().unwrap_or(0);
    Ok(JsValue::from(color_depth))
}

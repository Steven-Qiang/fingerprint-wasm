use wasm_bindgen::JsValue;

pub fn get_cpu_class(_ctx: &crate::sources::SourceContext) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();

    match js_sys::Reflect::get(&navigator, &JsValue::from_str("cpuClass")) {
        Ok(value) if !value.is_undefined() => Ok(value),
        _ => Ok(JsValue::null()),
    }
}

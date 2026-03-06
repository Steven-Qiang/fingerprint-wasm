use wasm_bindgen::{JsCast, JsValue};

/**
 * Touch support detection.
 * Returns an object with maxTouchPoints, touchEvent, and touchStart properties.
 */
pub fn get_touch_support() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let document = window.document().unwrap();

    let result = js_sys::Object::new();

    let max_touch_points = if let Ok(max_tp) =
        js_sys::Reflect::get(&navigator, &JsValue::from_str("maxTouchPoints"))
    {
        if let Some(num) = max_tp.as_f64() {
            num as i32
        } else if let Ok(ms_max_tp) =
            js_sys::Reflect::get(&navigator, &JsValue::from_str("msMaxTouchPoints"))
        {
            if let Some(num) = ms_max_tp.as_f64() {
                num as i32
            } else {
                0
            }
        } else {
            0
        }
    } else if let Ok(ms_max_tp) =
        js_sys::Reflect::get(&navigator, &JsValue::from_str("msMaxTouchPoints"))
    {
        if let Some(num) = ms_max_tp.as_f64() {
            num as i32
        } else {
            0
        }
    } else {
        0
    };

    js_sys::Reflect::set(
        &result,
        &JsValue::from_str("maxTouchPoints"),
        &JsValue::from(max_touch_points),
    )
    .unwrap();

    let touch_event = js_sys::Reflect::get(&document, &JsValue::from_str("createEvent"))
        .map(|create_event| {
            if let Ok(func) = create_event.dyn_into::<js_sys::Function>() {
                func.call1(&document, &JsValue::from_str("TouchEvent"))
                    .is_ok()
            } else {
                false
            }
        })
        .unwrap_or(false);

    js_sys::Reflect::set(
        &result,
        &JsValue::from_str("touchEvent"),
        &JsValue::from_bool(touch_event),
    )
    .unwrap();

    let touch_start =
        js_sys::Reflect::has(&window, &JsValue::from_str("ontouchstart")).unwrap_or(false);
    js_sys::Reflect::set(
        &result,
        &JsValue::from_str("touchStart"),
        &JsValue::from_bool(touch_start),
    )
    .unwrap();

    Ok(JsValue::from(result))
}

use crate::utils::{
    browser::{is_chromium, is_webkit},
    dom::with_iframe,
};
use wasm_bindgen::{JsCast, JsValue};

const DEFAULT_TEXT: &str = "mmMwWLliI0fiflO&1";

/**
 * The values in font preferences are the widths of the default text in the given fonts.
 * The default text is a string that contains characters that are different in different fonts.
 * The widths are measured in an iframe to avoid affecting the page layout.
 * The result is an object where keys are font preset names and values are widths.
 */
pub async fn get_font_preferences() -> Result<JsValue, JsValue> {
    let sizes = with_iframe(|_iframe, i_window| {
        let document = match i_window.document() {
            Some(doc) => doc,
            None => return js_sys::Object::new(),
        };

        let body = match document.body() {
            Some(b) => b,
            None => return js_sys::Object::new(),
        };

        let body_style = body.style();
        let _ = body_style.set_property("width", "4000px");
        let _ = body_style.set_property("webkitTextSizeAdjust", "none");
        let _ = body_style.set_property("textSizeAdjust", "none");

        if is_chromium() {
            let dpr = i_window.device_pixel_ratio();
            let _ = body_style.set_property("zoom", &format!("{}", 1.0 / dpr));
        } else if is_webkit() {
            let _ = body_style.set_property("zoom", "reset");
        }

        if let Ok(lines_of_text) = document.create_element("div") {
            let words: Vec<&str> = (0..200).map(|_| "word").collect();
            lines_of_text.set_text_content(Some(&words.join(" ")));
            let _ = body.append_child(&lines_of_text);
        }

        let presets: Vec<(&str, Vec<(&str, &str)>)> = vec![
            ("default", vec![]),
            ("apple", vec![("font", "-apple-system-body")]),
            ("serif", vec![("fontFamily", "serif")]),
            ("sans", vec![("fontFamily", "sans-serif")]),
            ("mono", vec![("fontFamily", "monospace")]),
            ("min", vec![("fontSize", "1px")]),
            ("system", vec![("fontFamily", "system-ui")]),
        ];

        let mut elements: std::collections::HashMap<String, web_sys::Element> =
            std::collections::HashMap::new();

        for (key, styles) in &presets {
            if let Ok(element) = document.create_element("span") {
                element.set_text_content(Some(DEFAULT_TEXT));
                let style = match js_sys::Reflect::get(&element, &JsValue::from_str("style")) {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                let _ = js_sys::Reflect::set(
                    &style,
                    &JsValue::from_str("whiteSpace"),
                    &JsValue::from_str("nowrap"),
                );

                for (name, value) in styles {
                    let _ = js_sys::Reflect::set(
                        &style,
                        &JsValue::from_str(name),
                        &JsValue::from_str(value),
                    );
                }

                if let Ok(br) = document.create_element("br") {
                    let _ = body.append_child(&br);
                }
                let _ = body.append_child(&element);
                elements.insert(key.to_string(), element);
            }
        }

        let sizes = js_sys::Object::new();

        let keys_order = ["default", "apple", "serif", "sans", "mono", "min", "system"];
        for key in &keys_order {
            if let Some(element) = elements.get(*key) {
                let width = if let Ok(get_bounding_client_rect) =
                    js_sys::Reflect::get(element, &JsValue::from_str("getBoundingClientRect"))
                {
                    if let Ok(func) = get_bounding_client_rect.dyn_into::<js_sys::Function>() {
                        if let Ok(rect) = func.call0(element) {
                            if let Ok(w) = js_sys::Reflect::get(&rect, &JsValue::from_str("width"))
                            {
                                w.as_f64().unwrap_or(0.0)
                            } else {
                                0.0
                            }
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };
                let _ = js_sys::Reflect::set(
                    &sizes,
                    &JsValue::from_str(key),
                    &JsValue::from_f64(width),
                );
            }
        }

        sizes
    })
    .await?;

    Ok(JsValue::from(sizes))
}

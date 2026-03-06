use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlIFrameElement, window};

fn parse_simple_css_selector(
    selector: &str,
) -> (Option<&str>, std::collections::HashMap<&str, Vec<&str>>) {
    let selector = selector.trim();

    let mut tag: Option<&str> = None;
    let mut rest = selector;

    if let Some(pos) = selector.find(|c| c == '.' || c == '#' || c == '[') {
        tag = Some(&selector[..pos]);
        rest = &selector[pos..];
    } else if !selector.is_empty() {
        tag = Some(selector);
        rest = "";
    }

    let mut attributes: std::collections::HashMap<&str, Vec<&str>> =
        std::collections::HashMap::new();

    let mut chars = rest.chars().peekable();
    while let Some(&first_char) = chars.peek() {
        match first_char {
            '.' => {
                chars.next();
                let mut class_name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '-' || c == '_' {
                        class_name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !class_name.is_empty() {
                    let class_str = Box::leak(class_name.into_boxed_str());
                    attributes
                        .entry("class")
                        .or_insert_with(Vec::new)
                        .push(class_str);
                }
            }
            '#' => {
                chars.next();
                let mut id = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '-' || c == '_' {
                        id.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !id.is_empty() {
                    let id_str = Box::leak(id.into_boxed_str());
                    attributes.entry("id").or_insert_with(Vec::new).push(id_str);
                }
            }
            '[' => {
                chars.next();
                let mut attr_content = String::new();
                while let Some(&c) = chars.peek() {
                    if c == ']' {
                        chars.next();
                        break;
                    }
                    attr_content.push(c);
                    chars.next();
                }

                let attr_content = attr_content.trim();
                if let Some(eq_pos) = attr_content.find('=') {
                    let name = attr_content[..eq_pos].trim();
                    let mut value = attr_content[eq_pos + 1..].trim();

                    if value.starts_with('"') && value.ends_with('"') {
                        value = &value[1..value.len() - 1];
                    } else if value.starts_with('\'') && value.ends_with('\'') {
                        value = &value[1..value.len() - 1];
                    }

                    let name_str = Box::leak(name.to_string().into_boxed_str());
                    let value_str = Box::leak(value.to_string().into_boxed_str());
                    attributes
                        .entry(name_str)
                        .or_insert_with(Vec::new)
                        .push(value_str);
                } else if !attr_content.is_empty() {
                    let name_str = Box::leak(attr_content.to_string().into_boxed_str());
                    attributes.entry(name_str).or_insert_with(Vec::new).push("");
                }
            }
            _ => {
                chars.next();
            }
        }
    }

    (tag, attributes)
}

pub fn selector_to_element(selector: &str) -> web_sys::Element {
    let document = window().unwrap().document().unwrap();
    let (tag, attributes) = parse_simple_css_selector(selector);

    let tag_name = tag.unwrap_or("div");
    let element = document.create_element(tag_name).unwrap();

    for (name, values) in attributes {
        let value = values.join(" ");
        if name == "style" {
            add_style_string(&element, &value);
        } else {
            let _ = element.set_attribute(name, &value);
        }
    }

    element
}

fn add_style_string(element: &web_sys::Element, source: &str) {
    let style = match js_sys::Reflect::get(element, &JsValue::from_str("style")) {
        Ok(s) => s,
        Err(_) => return,
    };

    for property in source.split(';') {
        let trimmed = property.trim();
        if !trimmed.is_empty() {
            if let Some((name, value)) = trimmed.split_once(':') {
                let name = name.trim();
                let value = value.trim();
                let _ = js_sys::Reflect::set(
                    &style,
                    &JsValue::from_str(name),
                    &JsValue::from_str(value),
                );
            }
        }
    }
}

fn check_iframe_ready_state(iframe: HtmlIFrameElement, resolve: js_sys::Function) {
    if let Some(content_window) = iframe.content_window() {
        if let Some(content_document) = content_window.document() {
            if content_document.ready_state() == "complete" {
                let _ = resolve.call0(&JsValue::undefined());
                return;
            }
        }
    }

    let iframe_next = iframe.clone();
    let resolve_next = resolve.clone();
    let window = window().unwrap();
    let timeout_closure = Closure::once(move |_: JsValue| {
        check_iframe_ready_state(iframe_next, resolve_next);
    });
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            timeout_closure.as_ref().unchecked_ref(),
            10,
        )
        .unwrap();
    timeout_closure.forget();
}

pub async fn with_iframe<F, R>(callback: F) -> Result<R, JsValue>
where
    F: FnOnce(web_sys::HtmlIFrameElement, web_sys::Window) -> R,
{
    let document = window().unwrap().document().unwrap();

    while document.body().is_none() {
        let delay_promise = js_sys::Promise::new(&mut |resolve, _| {
            let window = window().unwrap();
            let closure = Closure::once(move |_: JsValue| {
                let _ = resolve.call0(&JsValue::undefined());
            });
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    50,
                )
                .unwrap();
            closure.forget();
        });
        let _ = JsFuture::from(delay_promise).await;
    }

    let iframe = document
        .create_element("iframe")
        .unwrap()
        .dyn_into::<HtmlIFrameElement>()
        .unwrap();

    let style = iframe.style();
    style.set_property("display", "block").unwrap();
    style.set_property("position", "absolute").unwrap();
    style.set_property("top", "0").unwrap();
    style.set_property("left", "0").unwrap();
    style.set_property("visibility", "hidden").unwrap();
    iframe.set_width("0");
    iframe.set_height("0");

    document.body().unwrap().append_child(&iframe).unwrap();

    let load_promise = js_sys::Promise::new(&mut |resolve, reject| {
        let resolve_clone = resolve.clone();
        let reject_clone = reject.clone();
        let iframe_clone = iframe.clone();

        let onload_closure = Closure::once(move |_: JsValue| {
            let _ = resolve_clone.call0(&JsValue::undefined());
        });
        iframe.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
        onload_closure.forget();

        let onerror_closure = Closure::once(move |_: JsValue| {
            let _ = reject_clone.call1(
                &JsValue::undefined(),
                &JsValue::from_str("iframe load error"),
            );
        });
        iframe.set_onerror(Some(onerror_closure.as_ref().unchecked_ref()));
        onerror_closure.forget();

        iframe.set_src("about:blank");

        check_iframe_ready_state(iframe_clone, resolve.clone());
    });

    let _ = JsFuture::from(load_promise).await;

    while iframe.content_document().and_then(|d| d.body()).is_none() {
        let delay_promise = js_sys::Promise::new(&mut |resolve, _| {
            let window = window().unwrap();
            let closure = Closure::once(move |_: JsValue| {
                let _ = resolve.call0(&JsValue::undefined());
            });
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    50,
                )
                .unwrap();
            closure.forget();
        });
        let _ = JsFuture::from(delay_promise).await;
    }

    let content_window = match iframe.content_window() {
        Some(window) => window,
        None => return Err(JsValue::from_str("No content window")),
    };

    let result = callback(iframe.clone(), content_window);

    document.body().unwrap().remove_child(&iframe).unwrap();

    Ok(result)
}

pub fn is_any_parent_cross_origin() -> bool {
    if let Some(current_window) = window() {
        let mut current = current_window.clone();

        loop {
            match current.parent() {
                Ok(Some(parent_window)) => {
                    if parent_window == current {
                        return false;
                    }

                    let current_location = current.location();
                    let parent_location = parent_window.location();

                    match (current_location.origin(), parent_location.origin()) {
                        (Ok(current_origin), Ok(parent_origin)) => {
                            if current_origin != parent_origin {
                                return true;
                            }
                        }
                        _ => {
                            return true;
                        }
                    }

                    current = parent_window;
                }
                Ok(None) => {
                    return false;
                }
                Err(_) => {
                    return true;
                }
            }
        }
    }
    false
}

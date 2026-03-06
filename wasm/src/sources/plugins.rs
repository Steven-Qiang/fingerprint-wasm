use wasm_bindgen::JsValue;

/**
 * navigator.plugins is a PluginArray object which lists the plugins installed in the browser.
 * It's a non-standard feature that is not supported in all browsers.
 * The order of plugins is not guaranteed and can vary between browsers.
 */
pub fn get_plugins() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();

    let raw_plugins = js_sys::Reflect::get(&navigator, &JsValue::from_str("plugins"));

    if raw_plugins.is_err() {
        return Ok(JsValue::null());
    }

    let raw_plugins = raw_plugins.unwrap();

    if raw_plugins.is_undefined() || raw_plugins.is_null() {
        return Ok(JsValue::null());
    }

    let plugins_array = js_sys::Array::new();

    if let Ok(length) = js_sys::Reflect::get(&raw_plugins, &JsValue::from_str("length")) {
        if let Some(length_num) = length.as_f64() {
            for i in 0..(length_num as u32) {
                if let Ok(plugin) = js_sys::Reflect::get(&raw_plugins, &JsValue::from(i)) {
                    if plugin.is_undefined() || plugin.is_null() {
                        continue;
                    }

                    let plugin_obj = js_sys::Object::new();

                    if let Ok(name) = js_sys::Reflect::get(&plugin, &JsValue::from_str("name")) {
                        js_sys::Reflect::set(&plugin_obj, &JsValue::from_str("name"), &name)
                            .unwrap();
                    }

                    if let Ok(description) =
                        js_sys::Reflect::get(&plugin, &JsValue::from_str("description"))
                    {
                        js_sys::Reflect::set(
                            &plugin_obj,
                            &JsValue::from_str("description"),
                            &description,
                        )
                        .unwrap();
                    }

                    let mime_types_array = js_sys::Array::new();

                    if let Ok(plugin_length) =
                        js_sys::Reflect::get(&plugin, &JsValue::from_str("length"))
                    {
                        if let Some(plugin_length_num) = plugin_length.as_f64() {
                            for j in 0..(plugin_length_num as u32) {
                                if let Ok(mime_type) =
                                    js_sys::Reflect::get(&plugin, &JsValue::from(j))
                                {
                                    let mime_type_obj = js_sys::Object::new();

                                    if let Ok(type_val) =
                                        js_sys::Reflect::get(&mime_type, &JsValue::from_str("type"))
                                    {
                                        js_sys::Reflect::set(
                                            &mime_type_obj,
                                            &JsValue::from_str("type"),
                                            &type_val,
                                        )
                                        .unwrap();
                                    }

                                    if let Ok(suffixes) = js_sys::Reflect::get(
                                        &mime_type,
                                        &JsValue::from_str("suffixes"),
                                    ) {
                                        js_sys::Reflect::set(
                                            &mime_type_obj,
                                            &JsValue::from_str("suffixes"),
                                            &suffixes,
                                        )
                                        .unwrap();
                                    }

                                    mime_types_array.push(&mime_type_obj);
                                }
                            }
                        }
                    }

                    js_sys::Reflect::set(
                        &plugin_obj,
                        &JsValue::from_str("mimeTypes"),
                        &mime_types_array,
                    )
                    .unwrap();

                    plugins_array.push(&plugin_obj);
                }
            }
        }
    }

    Ok(JsValue::from(plugins_array))
}

use wasm_bindgen::{JsCast, JsValue};

// Status enum for date time locale
#[derive(Copy, Clone)]
enum Status {
    IntlAPINotSupported = -1,        // The browser doesn't support Intl API
    DateTimeFormatNotSupported = -2, // The browser doesn't support DateTimeFormat constructor
    LocaleNotAvailable = -3,         // DateTimeFormat locale is undefined or null
}

// Returns the date time locale
pub fn get_date_time_locale() -> Result<JsValue, JsValue> {
    let result = inner_get_date_time_locale();
    Ok(JsValue::from_str(&result))
}

// Internal implementation
fn inner_get_date_time_locale() -> String {
    if let Some(window) = web_sys::window() {
        // Check if Intl API is supported
        if let Ok(intl) = js_sys::Reflect::get(&window, &JsValue::from_str("Intl")) {
            if !intl.is_undefined() && !intl.is_null() {
                // Check if DateTimeFormat is supported
                if let Ok(date_time_format) =
                    js_sys::Reflect::get(&intl, &JsValue::from_str("DateTimeFormat"))
                {
                    if date_time_format.is_function() {
                        // Create a new DateTimeFormat instance
                        if let Ok(date_time_format_func) =
                            date_time_format.dyn_into::<js_sys::Function>()
                        {
                            match js_sys::Reflect::construct(
                                &date_time_format_func,
                                &js_sys::Array::new(),
                            ) {
                                Ok(format) => {
                                    // Get resolved options
                                    if let Ok(resolved_options) = js_sys::Reflect::get(
                                        &format,
                                        &JsValue::from_str("resolvedOptions"),
                                    ) {
                                        if resolved_options.is_function() {
                                            match resolved_options.dyn_into::<js_sys::Function>() {
                                                Ok(func) => match func.call0(&format) {
                                                    Ok(options) => {
                                                        // Get locale from options
                                                        match js_sys::Reflect::get(
                                                            &options,
                                                            &JsValue::from_str("locale"),
                                                        ) {
                                                            Ok(locale) => {
                                                                if locale.is_string() {
                                                                    return locale
                                                                        .as_string()
                                                                        .unwrap_or_else(|| {
                                                                            String::from("")
                                                                        });
                                                                } else if locale.is_null()
                                                                    || locale.is_undefined()
                                                                {
                                                                    return Status::LocaleNotAvailable.to_string();
                                                                }
                                                            }
                                                            Err(_) => {
                                                                return Status::LocaleNotAvailable
                                                                    .to_string();
                                                            }
                                                        }
                                                    }
                                                    Err(_) => {
                                                        return Status::LocaleNotAvailable
                                                            .to_string();
                                                    }
                                                },
                                                Err(_) => {
                                                    return Status::LocaleNotAvailable.to_string();
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    return Status::DateTimeFormatNotSupported.to_string();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Status::IntlAPINotSupported.to_string()
}

// Implement ToString for Status to return the integer value as string
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

use crate::utils::browser::{is_chromium, is_chromium_86_or_newer};
use wasm_bindgen::JsValue;

/**
 * navigator.languages is a read-only property that returns an array of strings representing
 * the user's preferred languages. The language is represented using a BCP 47 language tag.
 * In Chrome 86+, the languages array is not available in incognito mode.
 */
pub fn get_languages() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();

    let result = js_sys::Array::new();

    let language = get_first_language(&navigator);

    if let Some(lang) = language {
        if !lang.is_empty() {
            let lang_array = js_sys::Array::new();
            lang_array.push(&JsValue::from_str(&lang));
            result.push(&lang_array);
        }
    }

    let languages = navigator.languages();
    if languages.is_array() {
        if !(is_chromium() && is_chromium_86_or_newer()) {
            result.push(&languages);
        }
    } else if let Some(languages_str) = languages.as_string() {
        if !languages_str.is_empty() {
            let lang_array = js_sys::Array::new();
            for lang in languages_str.split(',') {
                lang_array.push(&JsValue::from_str(lang));
            }
            result.push(&lang_array);
        }
    }

    Ok(JsValue::from(result))
}

fn get_first_language(navigator: &web_sys::Navigator) -> Option<String> {
    if let Some(lang) = navigator.language() {
        if !lang.is_empty() {
            return Some(lang);
        }
    }

    for prop in &["userLanguage", "browserLanguage", "systemLanguage"] {
        if let Ok(value) = js_sys::Reflect::get(&JsValue::from(navigator), &JsValue::from_str(prop))
        {
            if let Some(s) = value.as_string() {
                if !s.is_empty() {
                    return Some(s);
                }
            }
        }
    }

    None
}

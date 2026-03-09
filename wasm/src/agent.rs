use js_sys::{Object, JSON};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

use crate::{confidence::get_confidence, sources, utils::hashing};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export interface FingerprintOptions {
    debug?: boolean;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "FingerprintOptions")]
    pub type FingerprintOptions;
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Serialize, Deserialize, Clone)]
pub struct ConfidenceResult {
    pub score: f64,
    pub comment: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Serialize, Deserialize, Clone)]
pub struct FingerprintResult {
    #[wasm_bindgen(js_name = "visitorId")]
    pub visitor_id: String,
    pub confidence: ConfidenceResult,
    #[wasm_bindgen(js_name = "componentsJson")]
    pub components_json: String,
    pub version: String,
}

#[wasm_bindgen(js_name = "getFingerprint")]
pub async fn get_fingerprint(
    options: Option<FingerprintOptions>,
) -> Result<FingerprintResult, JsValue> {
    let opts = parse_options(options)?;
    let get_components = sources::load_builtin_sources(opts);
    let components = wasm_bindgen_futures::JsFuture::from(get_components).await?;

    make_agent_result(components)
}

fn parse_options(options: Option<FingerprintOptions>) -> Result<sources::Options, JsValue> {
    let mut opts = sources::Options::default();

    if let Some(js_opts) = options {
        let opts_value: JsValue = js_opts.into();
        if let Ok(debug) = js_sys::Reflect::get(&opts_value, &JsValue::from_str("debug")) {
            if let Some(debug_val) = debug.as_bool() {
                opts.debug = debug_val;
            }
        }
    }

    Ok(opts)
}

fn make_agent_result(components: JsValue) -> Result<FingerprintResult, JsValue> {
    let confidence_js = get_confidence(components.clone())?;
    let confidence: ConfidenceResult = serde_wasm_bindgen::from_value(confidence_js)?;

    let version = env!("CARGO_PKG_VERSION").to_string();
    let visitor_id = hash_components(components.clone())?;

    let components_json = JSON::stringify(&components)
        .map_err(|e| JsValue::from_str(&format!("Failed to stringify: {:?}", e)))?
        .as_string()
        .unwrap_or_else(|| "{}".to_string());

    Ok(FingerprintResult {
        visitor_id,
        confidence,
        components_json,
        version,
    })
}

pub fn hash_components(components: JsValue) -> Result<String, JsValue> {
    let canonical_string = components_to_canonical_string(components)?;
    Ok(hashing::x64hash128(&canonical_string))
}

fn components_to_canonical_string(components: JsValue) -> Result<String, JsValue> {
    let components_obj: Object = components.dyn_into()?;
    let keys_array = Object::keys(&components_obj);
    let mut sorted_keys: BTreeMap<String, JsValue> = BTreeMap::new();

    for i in 0..keys_array.length() {
        let key = keys_array.get(i).as_string().unwrap_or_default();
        let value = js_sys::Reflect::get(&components_obj, &JsValue::from_str(&key))?;
        sorted_keys.insert(key, value);
    }

    let mut result = String::new();
    for (key, component) in sorted_keys {
        let processed_key = key
            .replace("\\", "\\\\")
            .replace(":", "\\:")
            .replace("|", "\\|");
        let processed_value = if js_sys::Reflect::has(&component, &JsValue::from_str("error"))? {
            "error".to_string()
        } else {
            let value = js_sys::Reflect::get(&component, &JsValue::from_str("value"))?;
            if value.is_undefined() {
                "undefined".to_string()
            } else {
                match JSON::stringify(&value) {
                    Ok(json_str) => json_str.as_string().unwrap_or_else(|| "null".to_string()),
                    Err(_) => "null".to_string(),
                }
            }
        };
        if !result.is_empty() {
            result.push('|');
        }
        result.push_str(&format!("{}:{}", processed_key, processed_value));
    }

    Ok(result)
}

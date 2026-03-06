use crate::utils::browser::{is_android, is_webkit};
use wasm_bindgen::{JsCast, JsValue};

/**
 * Special fingerprint values for audio context base latency
 */
enum SpecialFingerprint {
    NotSupported = -1, // The browser doesn't support AudioContext or baseLatency
    Disabled = -2,     // Entropy source is disabled because of console warnings
    NotFinite = -3,    /* Weird case where `baseLatency` is not a float number but `Infinity`
                        * instead */
}

/**
 * Returns the audio context base latency
 * The signal emits warning in Chrome and Firefox, therefore it is enabled on Safari where it
 * doesn't produce warning and on Android where it's less visible
 */
pub async fn get_audio_context_base_latency() -> Result<JsValue, JsValue> {
    let result = inner_get_audio_context_base_latency();
    Ok(JsValue::from(result))
}

// Internal implementation
fn inner_get_audio_context_base_latency() -> f64 {
    // The signal emits warning in Chrome and Firefox, therefore it is enabled on Safari where it
    // doesn't produce warning and on Android where it's less visible
    let is_allowed_platform = is_android() || is_webkit();
    if !is_allowed_platform {
        return (SpecialFingerprint::Disabled as i32) as f64;
    }

    if let Some(window) = web_sys::window() {
        if let Ok(audio_context) = js_sys::Reflect::get(&window, &JsValue::from_str("AudioContext"))
        {
            if audio_context.is_function() {
                // Create a new AudioContext
                match audio_context.dyn_into::<js_sys::Function>() {
                    Ok(audio_context_func) => {
                        match js_sys::Reflect::construct(&audio_context_func, &js_sys::Array::new())
                        {
                            Ok(context) => {
                                // Get baseLatency
                                match js_sys::Reflect::get(
                                    &context,
                                    &JsValue::from_str("baseLatency"),
                                ) {
                                    Ok(latency) => {
                                        if latency.is_null() || latency.is_undefined() {
                                            return (SpecialFingerprint::NotSupported as i32)
                                                as f64;
                                        }

                                        if let Some(latency_f64) = latency.as_f64() {
                                            if !latency_f64.is_finite() {
                                                return (SpecialFingerprint::NotFinite as i32)
                                                    as f64;
                                            }
                                            return latency_f64;
                                        }
                                    }
                                    Err(_) => {
                                        return (SpecialFingerprint::NotSupported as i32) as f64;
                                    }
                                }
                            }
                            Err(_) => {
                                return (SpecialFingerprint::NotSupported as i32) as f64;
                            }
                        }
                    }
                    Err(_) => {
                        return (SpecialFingerprint::NotSupported as i32) as f64;
                    }
                }
            }
        }
    }

    (SpecialFingerprint::NotSupported as i32) as f64
}

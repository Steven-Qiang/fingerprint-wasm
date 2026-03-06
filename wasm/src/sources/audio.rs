use crate::utils::browser::{
    is_chromium, is_chromium_122_or_newer, is_desktop_webkit, is_safari_webkit,
    is_samsung_internet, is_webkit, is_webkit_606_or_newer, is_webkit_616_or_newer,
};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

const SPECIAL_FINGERPRINT_KNOWN_FOR_SUSPENDING: f64 = -1.0;
const SPECIAL_FINGERPRINT_NOT_SUPPORTED: f64 = -2.0;
const SPECIAL_FINGERPRINT_TIMEOUT: f64 = -3.0;
const SPECIAL_FINGERPRINT_KNOWN_FOR_ANTIFINGERPRINTING: f64 = -4.0;

/**
 * A deep description: https://fingerprint.com/blog/audio-fingerprinting/
 * Inspired by and based on https://github.com/cozylife/audio-fingerprint
 *
 * A version of the entropy source with stabilization to make it suitable for static
 * fingerprinting. Audio signal is noised in private mode of Safari 17, so audio fingerprinting
 * is skipped in Safari 17.
 */
pub async fn get_audio_fingerprint() -> Result<JsValue, JsValue> {
    if does_browser_perform_antifingerprinting() {
        return Ok(JsValue::from_f64(
            SPECIAL_FINGERPRINT_KNOWN_FOR_ANTIFINGERPRINTING,
        ));
    }

    get_unstable_audio_fingerprint().await
}

async fn get_unstable_audio_fingerprint() -> Result<JsValue, JsValue> {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_NOT_SUPPORTED)),
    };

    let audio_context =
        match js_sys::Reflect::get(&window, &JsValue::from_str("OfflineAudioContext")) {
            Ok(ctx) if !ctx.is_undefined() => ctx,
            _ => {
                match js_sys::Reflect::get(&window, &JsValue::from_str("webkitOfflineAudioContext"))
                {
                    Ok(ctx) if !ctx.is_undefined() => ctx,
                    _ => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_NOT_SUPPORTED)),
                }
            }
        };

    if does_browser_suspend_audio_context() {
        return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_KNOWN_FOR_SUSPENDING));
    }

    let hash_from_index = 4500;
    let hash_to_index = 5000;
    let sample_rate = 44100;

    let context_func = match audio_context.dyn_into::<js_sys::Function>() {
        Ok(f) => f,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_NOT_SUPPORTED)),
    };

    let args = js_sys::Array::new();
    args.push(&JsValue::from(1));
    args.push(&JsValue::from(hash_to_index));
    args.push(&JsValue::from(sample_rate));

    let context = match js_sys::Reflect::construct(&context_func, &args) {
        Ok(ctx) => ctx,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_NOT_SUPPORTED)),
    };

    let create_oscillator =
        match js_sys::Reflect::get(&context, &JsValue::from_str("createOscillator")) {
            Ok(f) => f,
            Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_NOT_SUPPORTED)),
        };

    let create_oscillator_func = match create_oscillator.dyn_into::<js_sys::Function>() {
        Ok(f) => f,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_NOT_SUPPORTED)),
    };

    let oscillator = match create_oscillator_func.call0(&context) {
        Ok(osc) => osc,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_NOT_SUPPORTED)),
    };

    let _ = js_sys::Reflect::set(
        &oscillator,
        &JsValue::from_str("type"),
        &JsValue::from_str("triangle"),
    );

    if let Ok(frequency) = js_sys::Reflect::get(&oscillator, &JsValue::from_str("frequency")) {
        let _ = js_sys::Reflect::set(
            &frequency,
            &JsValue::from_str("value"),
            &JsValue::from_f64(10000.0),
        );
    }

    let create_compressor =
        match js_sys::Reflect::get(&context, &JsValue::from_str("createDynamicsCompressor")) {
            Ok(f) => f,
            Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_NOT_SUPPORTED)),
        };

    let create_compressor_func = match create_compressor.dyn_into::<js_sys::Function>() {
        Ok(f) => f,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_NOT_SUPPORTED)),
    };

    let compressor = match create_compressor_func.call0(&context) {
        Ok(comp) => comp,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_NOT_SUPPORTED)),
    };

    set_compressor_parameter(&compressor, "threshold", -50.0);
    set_compressor_parameter(&compressor, "knee", 40.0);
    set_compressor_parameter(&compressor, "ratio", 12.0);
    set_compressor_parameter(&compressor, "attack", 0.0);
    set_compressor_parameter(&compressor, "release", 0.25);

    connect_audio_node(&oscillator, &compressor);
    connect_audio_node_to_destination(&compressor, &context);

    if let Ok(start) = js_sys::Reflect::get(&oscillator, &JsValue::from_str("start")) {
        if let Ok(start_func) = start.dyn_into::<js_sys::Function>() {
            let _ = start_func.call1(&oscillator, &JsValue::from_f64(0.0));
        }
    }

    let buffer = match render_audio(&context).await {
        Ok(b) => b,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_TIMEOUT)),
    };

    let get_channel_data = match js_sys::Reflect::get(&buffer, &JsValue::from_str("getChannelData"))
    {
        Ok(f) => f,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_TIMEOUT)),
    };

    let get_channel_data_func = match get_channel_data.dyn_into::<js_sys::Function>() {
        Ok(f) => f,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_TIMEOUT)),
    };

    let channel_data = match get_channel_data_func.call1(&buffer, &JsValue::from(0)) {
        Ok(d) => d,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_TIMEOUT)),
    };

    let subarray = match js_sys::Reflect::get(&channel_data, &JsValue::from_str("subarray")) {
        Ok(f) => f,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_TIMEOUT)),
    };

    let subarray_func = match subarray.dyn_into::<js_sys::Function>() {
        Ok(f) => f,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_TIMEOUT)),
    };

    let signal = match subarray_func.call1(&channel_data, &JsValue::from(hash_from_index)) {
        Ok(s) => s,
        Err(_) => return Ok(JsValue::from_f64(SPECIAL_FINGERPRINT_TIMEOUT)),
    };

    let hash = get_hash(&signal);
    Ok(JsValue::from_f64(hash))
}

fn set_compressor_parameter(compressor: &JsValue, parameter_name: &str, value: f64) {
    if let Ok(parameter) = js_sys::Reflect::get(compressor, &JsValue::from_str(parameter_name)) {
        let _ = js_sys::Reflect::set(
            &parameter,
            &JsValue::from_str("value"),
            &JsValue::from_f64(value),
        );
    }
}

fn connect_audio_node(source: &JsValue, destination: &JsValue) {
    if let Ok(connect) = js_sys::Reflect::get(source, &JsValue::from_str("connect")) {
        if let Ok(connect_func) = connect.dyn_into::<js_sys::Function>() {
            let _ = connect_func.call1(source, destination);
        }
    }
}

fn connect_audio_node_to_destination(source: &JsValue, context: &JsValue) {
    if let Ok(destination) = js_sys::Reflect::get(context, &JsValue::from_str("destination")) {
        connect_audio_node(source, &destination);
    }
}

fn get_hash(signal: &JsValue) -> f64 {
    let mut hash = 0.0;

    if let Ok(length) = js_sys::Reflect::get(signal, &JsValue::from_str("length")) {
        if let Some(len) = length.as_f64() {
            for i in 0..(len as usize) {
                if let Ok(value) = js_sys::Reflect::get(signal, &JsValue::from(i)) {
                    if let Some(v) = value.as_f64() {
                        hash += v.abs();
                    }
                }
            }
        }
    }

    hash
}

fn does_browser_suspend_audio_context() -> bool {
    is_webkit() && !is_desktop_webkit() && !is_webkit_606_or_newer()
}

fn does_browser_perform_antifingerprinting() -> bool {
    (is_webkit() && is_webkit_616_or_newer() && is_safari_webkit())
        || (is_chromium() && is_samsung_internet() && is_chromium_122_or_newer())
}

async fn render_audio(context: &JsValue) -> Result<JsValue, JsValue> {
    let context_clone = context.clone();

    let (sender, receiver) = futures::channel::oneshot::channel();
    let sender = std::cell::RefCell::new(Some(sender));

    let oncomplete_callback = Closure::once(move |event: JsValue| {
        if let Ok(rendered_buffer) =
            js_sys::Reflect::get(&event, &JsValue::from_str("renderedBuffer"))
        {
            if let Some(s) = sender.borrow_mut().take() {
                let _ = s.send(Ok(rendered_buffer));
            }
        }
    });

    let _ = js_sys::Reflect::set(
        &context_clone,
        &JsValue::from_str("oncomplete"),
        &oncomplete_callback.as_ref().clone(),
    );

    oncomplete_callback.forget();

    let start_rendering =
        match js_sys::Reflect::get(&context_clone, &JsValue::from_str("startRendering")) {
            Ok(f) => f,
            Err(_) => return Err(JsValue::from_str("startRendering not found")),
        };

    let start_rendering_func = match start_rendering.dyn_into::<js_sys::Function>() {
        Ok(f) => f,
        Err(_) => return Err(JsValue::from_str("startRendering is not a function")),
    };

    let _ = start_rendering_func.call0(&context_clone);

    receiver
        .await
        .map_err(|_| JsValue::from_str("channel closed"))?
}

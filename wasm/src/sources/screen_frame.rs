use js_sys::{Array, Reflect};
use wasm_bindgen::JsValue;
use web_sys::window;

use crate::utils::browser::{
    is_gecko, is_gecko_143_or_newer, is_safari_webkit, is_webkit, is_webkit_616_or_newer,
};

const ROUNDING_PRECISION: f64 = 10.0;

/**
 * A version of the entropy source with stabilization to make it suitable for static
 * fingerprinting. Screen frame is always zero in private mode of Safari 17 and Firefox 143+,
 * so screen frame is not used in Safari 17 and Firefox 143+.
 */
pub async fn get_screen_frame() -> Result<JsValue, JsValue> {
    let is_safari_17_or_above = is_webkit() && is_webkit_616_or_newer() && is_safari_webkit();
    let is_firefox_143_or_above = is_gecko() && is_gecko_143_or_newer();

    if is_safari_17_or_above || is_firefox_143_or_above {
        return Ok(JsValue::undefined());
    }

    let window = window().ok_or_else(|| JsValue::from_str("无法获取 window"))?;
    let screen = window
        .screen()
        .or_else(|_| Err(JsValue::from_str("无法获取 screen")))?;

    let frame_size = get_current_screen_frame(&screen);

    let process_size = |value: Option<f64>| -> JsValue {
        match value {
            Some(v) => JsValue::from_f64(round(v, ROUNDING_PRECISION)),
            None => JsValue::null(),
        }
    };

    let array = Array::new();
    array.push(&process_size(frame_size[0]));
    array.push(&process_size(frame_size[1]));
    array.push(&process_size(frame_size[2]));
    array.push(&process_size(frame_size[3]));

    Ok(JsValue::from(array))
}

fn get_current_screen_frame(screen: &web_sys::Screen) -> [Option<f64>; 4] {
    let width = to_float(&Reflect::get(&JsValue::from(screen), &JsValue::from_str("width")).ok());
    let height = to_float(&Reflect::get(&JsValue::from(screen), &JsValue::from_str("height")).ok());
    let avail_width =
        to_float(&Reflect::get(&JsValue::from(screen), &JsValue::from_str("availWidth")).ok());
    let avail_height =
        to_float(&Reflect::get(&JsValue::from(screen), &JsValue::from_str("availHeight")).ok());
    let avail_left =
        to_float(&Reflect::get(&JsValue::from(screen), &JsValue::from_str("availLeft")).ok());
    let avail_top =
        to_float(&Reflect::get(&JsValue::from(screen), &JsValue::from_str("availTop")).ok());

    let avail_left_0 = avail_left.unwrap_or(0.0);
    let avail_top_0 = avail_top.unwrap_or(0.0);

    let top = replace_nan(avail_top, None);
    let right = match (width, avail_width) {
        (Some(w), Some(aw)) => replace_nan(Some(w - aw - avail_left_0), None),
        _ => None,
    };
    let bottom = match (height, avail_height) {
        (Some(h), Some(ah)) => replace_nan(Some(h - ah - avail_top_0), None),
        _ => None,
    };
    let left = replace_nan(avail_left, None);

    [top, right, bottom, left]
}

fn to_float(value: &Option<JsValue>) -> Option<f64> {
    match value {
        Some(v) => {
            if v.is_null() || v.is_undefined() {
                return None;
            }
            if let Some(num) = v.as_f64() {
                return Some(num);
            }
            if let Some(s) = v.as_string() {
                return s.parse::<f64>().ok();
            }
            None
        }
        None => None,
    }
}

fn replace_nan<T>(value: Option<f64>, replacement: T) -> Option<f64>
where
    T: Into<Option<f64>>,
{
    match value {
        Some(v) if !v.is_nan() => Some(v),
        _ => replacement.into(),
    }
}

fn round(value: f64, base: f64) -> f64 {
    if base.abs() >= 1.0 {
        (value / base).round() * base
    } else {
        let counter_base = 1.0 / base;
        (value * counter_base).round() / counter_base
    }
}

use js_sys::Object;
use wasm_bindgen::JsValue;

use crate::utils::browser::{
    is_android, is_desktop_webkit, is_safari_webkit, is_webkit, is_webkit_616_or_newer,
};

pub const COMMENT_TEMPLATE: &str = "$ if upgrade to Pro: https://fingerprint.com/github/?utm_source=oss&utm_medium=referral&utm_campaign=confidence_score";

pub fn get_confidence(components: JsValue) -> Result<JsValue, JsValue> {
    let open_confidence_score = get_open_confidence_score(&components);
    let pro_confidence_score = derive_pro_confidence_score(open_confidence_score);

    let confidence = Object::new();
    js_sys::Reflect::set(
        &confidence,
        &JsValue::from_str("score"),
        &JsValue::from_f64(open_confidence_score),
    )?;
    js_sys::Reflect::set(
        &confidence,
        &JsValue::from_str("comment"),
        &JsValue::from_str(&COMMENT_TEMPLATE.replace('$', &format!("{}", pro_confidence_score))),
    )?;

    Ok(JsValue::from(confidence))
}

fn derive_pro_confidence_score(open_confidence_score: f64) -> f64 {
    round(0.99 + 0.01 * open_confidence_score, 0.0001)
}

fn round(value: f64, precision: f64) -> f64 {
    (value / precision).round() * precision
}

/**
 * In order to calculate the true probability of the visitor identifier being correct, we need
 * to know the number of website visitors (the higher the number, the less the probability
 * because the fingerprint entropy is limited). JS agent doesn't know the number of visitors, so
 * we can only do an approximate assessment.
 */
fn get_open_confidence_score(components: &JsValue) -> f64 {
    if is_android() {
        return 0.4;
    }

    // Safari (mobile and desktop)
    if is_webkit() {
        return if is_desktop_webkit() && !(is_webkit_616_or_newer() && is_safari_webkit()) {
            0.5
        } else {
            0.3
        };
    }

    let platform = get_platform_value(components);

    // Windows
    // The score is greater than on macOS because of the higher variety of devices running Windows.
    // Chrome provides more entropy than Firefox according too
    // https://netmarketshare.com/browser-market-share.aspx?options=%7B%22filter%22%3A%7B%22%24and%22%3A%5B%7B%22platform%22%3A%7B%22%24in%22%3A%5B%22Windows%22%5D%7D%7D%5D%7D%2C%22dateLabel%22%3A%22Trend%22%2C%22attributes%22%3A%22share%22%2C%22group%22%3A%22browser%22%2C%22sort%22%3A%7B%22share%22%3A-1%7D%2C%22id%22%3A%22browsersDesktop%22%2C%22dateInterval%22%3A%22Monthly%22%2C%22dateStart%22%3A%222019-11%22%2C%22dateEnd%22%3A%222020-10%22%2C%22segments%22%3A%22-1000%22%7D
    // So we assign the same score to them.
    if platform.starts_with("Win") {
        return 0.6;
    }

    // macOS
    // Chrome provides more entropy than Safari and Safari provides more entropy than Firefox.
    // Chrome is more popular than Safari and Safari is more popular than Firefox according to
    // https://netmarketshare.com/browser-market-share.aspx?options=%7B%22filter%22%3A%7B%22%24and%22%3A%5B%7B%22platform%22%3A%7B%22%24in%22%3A%5B%22Mac%20OS%22%5D%7D%7D%5D%7D%2C%22dateLabel%22%3A%22Trend%22%2C%22attributes%22%3A%22share%22%2C%22group%22%3A%22browser%22%2C%22sort%22%3A%7B%22share%22%3A-1%7D%2C%22id%22%3A%22browsersDesktop%22%2C%22dateInterval%22%3A%22Monthly%22%2C%22dateStart%22%3A%222019-11%22%2C%22dateEnd%22%3A%222020-10%22%2C%22segments%22%3A%22-1000%22%7D
    // So we assign the same score to them.
    if platform.starts_with("Mac") {
        return 0.5;
    }

    // Another platform, e.g. a desktop Linux. It's rare, so it should be pretty unique.
    0.7
}

fn get_platform_value(components: &JsValue) -> String {
    if let Ok(platform) = js_sys::Reflect::get(components, &JsValue::from_str("platform")) {
        if !platform.is_null() && !platform.is_undefined() {
            if let Ok(value) = js_sys::Reflect::get(&platform, &JsValue::from_str("value")) {
                if let Some(s) = value.as_string() {
                    return s;
                }
            }
        }
    }
    String::new()
}

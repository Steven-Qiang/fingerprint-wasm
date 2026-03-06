use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;

fn count_truthy(values: &[bool]) -> usize {
    values.iter().filter(|&&v| v).count()
}

fn has_property(obj: &JsValue, prop: &str) -> bool {
    js_sys::Reflect::has(obj, &JsValue::from_str(prop)).unwrap_or(false)
}

fn get_window_prop(prop: &str) -> JsValue {
    window()
        .and_then(|w| js_sys::Reflect::get(&w, &JsValue::from_str(prop)).ok())
        .unwrap_or(JsValue::undefined())
}

fn object_to_string(obj: &JsValue) -> String {
    let to_string = js_sys::Reflect::get(obj, &JsValue::from_str("toString")).ok();
    if let Some(to_string) = to_string {
        if let Ok(func) = to_string.dyn_into::<js_sys::Function>() {
            if let Ok(result) = func.call0(obj) {
                if let Some(s) = result.as_string() {
                    return s;
                }
            }
        }
    }
    String::new()
}

pub fn is_trident() -> bool {
    if let Some(window) = window() {
        let navigator = window.navigator();
        return count_truthy(&[
            has_property(&window, "MSCSSMatrix"),
            has_property(&window, "msSetImmediate"),
            has_property(&window, "msIndexedDB"),
            has_property(&navigator, "msMaxTouchPoints"),
            has_property(&navigator, "msPointerEnabled"),
        ]) >= 4;
    }
    false
}

pub fn is_edge_html() -> bool {
    if let Some(window) = window() {
        let navigator = window.navigator();
        return count_truthy(&[
            has_property(&window, "msWriteProfilerMark"),
            has_property(&window, "MSStream"),
            has_property(&navigator, "msLaunchUri"),
            has_property(&navigator, "msSaveBlob"),
        ]) >= 3
            && !is_trident();
    }
    false
}

pub fn is_chromium() -> bool {
    if let Some(window) = window() {
        let navigator = window.navigator();
        let vendor = js_sys::Reflect::get(&navigator, &JsValue::from_str("vendor"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
        return count_truthy(&[
            has_property(&navigator, "webkitPersistentStorage"),
            has_property(&navigator, "webkitTemporaryStorage"),
            vendor.starts_with("Google"),
            has_property(&window, "webkitResolveLocalFileSystemURL"),
            has_property(&window, "BatteryManager"),
            has_property(&window, "webkitMediaStream"),
            has_property(&window, "webkitSpeechGrammar"),
        ]) >= 5;
    }
    false
}

pub fn is_webkit() -> bool {
    if let Some(window) = window() {
        let navigator = window.navigator();
        let vendor = js_sys::Reflect::get(&navigator, &JsValue::from_str("vendor"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
        return count_truthy(&[
            has_property(&window, "ApplePayError"),
            has_property(&window, "CSSPrimitiveValue"),
            has_property(&window, "Counter"),
            vendor.starts_with("Apple"),
            has_property(&window, "RGBColor"),
            has_property(&window, "WebKitMediaKeys"),
        ]) >= 4;
    }
    false
}

pub fn is_desktop_webkit() -> bool {
    if let Some(window) = window() {
        let html_element = get_window_prop("HTMLElement");
        let document = get_window_prop("Document");

        let has_autocapitalize = if !html_element.is_undefined() {
            let prototype =
                js_sys::Reflect::get(&html_element, &JsValue::from_str("prototype")).ok();
            prototype
                .map(|p| has_property(&p, "autocapitalize"))
                .unwrap_or(false)
        } else {
            false
        };

        let has_pointer_lock_element = if !document.is_undefined() {
            let prototype = js_sys::Reflect::get(&document, &JsValue::from_str("prototype")).ok();
            prototype
                .map(|p| has_property(&p, "pointerLockElement"))
                .unwrap_or(false)
        } else {
            false
        };

        return count_truthy(&[
            has_property(&window, "safari"),
            !has_property(&window, "ongestureend"),
            !has_property(&window, "TouchEvent"),
            !has_property(&window, "orientation"),
            !has_autocapitalize,
            has_pointer_lock_element,
        ]) >= 4;
    }
    false
}

pub fn is_safari_webkit() -> bool {
    if let Some(window) = window() {
        let print_func = js_sys::Reflect::get(&window, &JsValue::from_str("print")).ok();
        let is_native = print_func
            .as_ref()
            .map(|f| {
                let to_string_result = js_sys::Reflect::get(f, &JsValue::from_str("toString")).ok();
                if let Some(to_string) = to_string_result {
                    if let Ok(to_string_func) = to_string.dyn_into::<js_sys::Function>() {
                        if let Ok(result) = to_string_func.call0(f) {
                            if let Some(result_str) = result.as_string() {
                                return result_str.contains("[native code]");
                            }
                        }
                    }
                }
                false
            })
            .unwrap_or(false);

        let browser = js_sys::Reflect::get(&window, &JsValue::from_str("browser")).ok();
        let browser_str = browser
            .as_ref()
            .map(|b| object_to_string(b))
            .unwrap_or_default();

        return is_native && browser_str == "[object WebPageNamespace]";
    }
    false
}

pub fn is_gecko() -> bool {
    if let Some(window) = window() {
        let navigator = window.navigator();
        let document = window.document();

        let has_moz_appearance = document
            .and_then(|d| d.document_element())
            .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
            .map(|el| {
                let style = el.style();
                let moz_appearance =
                    js_sys::Reflect::get(&style, &JsValue::from_str("MozAppearance")).ok();
                moz_appearance.map(|v| !v.is_undefined()).unwrap_or(false)
            })
            .unwrap_or(false);

        return count_truthy(&[
            has_property(&navigator, "buildID"),
            has_moz_appearance,
            has_property(&window, "onmozfullscreenchange"),
            has_property(&window, "mozInnerScreenX"),
            has_property(&window, "CSSMozDocumentRule"),
            has_property(&window, "CanvasCaptureMediaStream"),
        ]) >= 4;
    }
    false
}

fn css_supports(css: &JsValue, property: &str, value: &str) -> bool {
    if let Ok(supports_func) = css.clone().dyn_into::<js_sys::Function>() {
        let args = js_sys::Array::new();
        args.push(&JsValue::from_str(property));
        args.push(&JsValue::from_str(value));
        supports_func
            .call1(css, &args)
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    } else {
        false
    }
}

pub fn is_gecko_120_or_newer() -> bool {
    if let Some(window) = window() {
        let navigator = window.navigator();
        let css = js_sys::Reflect::get(&window, &JsValue::from_str("CSS")).ok();

        let supports_light_dark = css
            .as_ref()
            .map(|c| css_supports(c, "color", "light-dark(#000, #fff)"))
            .unwrap_or(false);
        let supports_lh = css
            .as_ref()
            .map(|c| css_supports(c, "height", "1lh"))
            .unwrap_or(false);

        return count_truthy(&[
            has_property(&navigator, "userActivation"),
            supports_light_dark,
            supports_lh,
            has_property(&navigator, "globalPrivacyControl"),
        ]) >= 3;
    }
    false
}

pub fn is_gecko_143_or_newer() -> bool {
    if let Some(window) = window() {
        let css = js_sys::Reflect::get(&window, &JsValue::from_str("CSS")).ok();

        let supports_details_content = css
            .as_ref()
            .map(|c| {
                if let Ok(supports_func) = c.clone().dyn_into::<js_sys::Function>() {
                    let args = js_sys::Array::new();
                    args.push(&JsValue::from_str("selector(::details-content)"));
                    supports_func
                        .call1(c, &args)
                        .ok()
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false)
                } else {
                    false
                }
            })
            .unwrap_or(false);

        let supports_before_marker = css
            .as_ref()
            .map(|c| {
                if let Ok(supports_func) = c.clone().dyn_into::<js_sys::Function>() {
                    let args = js_sys::Array::new();
                    args.push(&JsValue::from_str("selector(::before::marker)"));
                    supports_func
                        .call1(c, &args)
                        .ok()
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false)
                } else {
                    false
                }
            })
            .unwrap_or(false);

        let supports_after_marker = css
            .as_ref()
            .map(|c| {
                if let Ok(supports_func) = c.clone().dyn_into::<js_sys::Function>() {
                    let args = js_sys::Array::new();
                    args.push(&JsValue::from_str("selector(::after::marker)"));
                    supports_func
                        .call1(c, &args)
                        .ok()
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false)
                } else {
                    false
                }
            })
            .unwrap_or(false);

        let composition_event = get_window_prop("CompositionEvent");
        let has_locale = if !composition_event.is_undefined() {
            let prototype =
                js_sys::Reflect::get(&composition_event, &JsValue::from_str("prototype")).ok();
            prototype
                .map(|p| has_property(&p, "locale"))
                .unwrap_or(false)
        } else {
            false
        };

        return count_truthy(&[
            supports_details_content,
            supports_before_marker,
            supports_after_marker,
            !has_locale,
        ]) >= 3;
    }
    false
}

pub fn is_chromium_86_or_newer() -> bool {
    if let Some(window) = window() {
        let intl = js_sys::Reflect::get(&window, &JsValue::from_str("Intl")).ok();
        let intl_str = intl
            .as_ref()
            .map(|i| object_to_string(i))
            .unwrap_or_default();

        let reflect = js_sys::Reflect::get(&window, &JsValue::from_str("Reflect")).ok();
        let reflect_str = reflect
            .as_ref()
            .map(|r| object_to_string(r))
            .unwrap_or_default();

        return count_truthy(&[
            !has_property(&window, "MediaSettingsRange"),
            has_property(&window, "RTCEncodedAudioFrame"),
            intl_str == "[object Intl]",
            reflect_str == "[object Reflect]",
        ]) >= 3;
    }
    false
}

pub fn is_chromium_122_or_newer() -> bool {
    if let Some(window) = window() {
        let set_prototype = js_sys::Reflect::get(
            &js_sys::Set::new(&js_sys::Array::new()),
            &JsValue::from_str("prototype"),
        )
        .ok();
        let has_union = set_prototype
            .as_ref()
            .map(|s| has_property(s, "union"))
            .unwrap_or(false);

        let url_pattern = js_sys::Reflect::get(&window, &JsValue::from_str("URLPattern")).ok();
        let has_regexp_groups = url_pattern
            .as_ref()
            .map(|up| {
                let prototype = js_sys::Reflect::get(up, &JsValue::from_str("prototype")).ok();
                prototype
                    .map(|p| has_property(&p, "hasRegExpGroups"))
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        let webgl = js_sys::Reflect::get(&window, &JsValue::from_str("WebGLRenderingContext")).ok();
        let has_rgb8 = webgl
            .as_ref()
            .map(|w| has_property(w, "RGB8"))
            .unwrap_or(false);

        return count_truthy(&[
            has_union,
            has_property(&window, "Iterator"),
            has_regexp_groups,
            has_rgb8,
        ]) >= 3;
    }
    false
}

pub fn is_webkit_606_or_newer() -> bool {
    if let Some(window) = window() {
        return count_truthy(&[
            has_property(&window, "DOMRectList"),
            has_property(&window, "RTCPeerConnectionIceEvent"),
            has_property(&window, "SVGGeometryElement"),
            has_property(&window, "ontransitioncancel"),
        ]) >= 3;
    }
    false
}

pub fn is_webkit_616_or_newer() -> bool {
    if let Some(window) = window() {
        let navigator = window.navigator();
        let css = js_sys::Reflect::get(&window, &JsValue::from_str("CSS")).ok();
        let html_button =
            js_sys::Reflect::get(&window, &JsValue::from_str("HTMLButtonElement")).ok();

        let has_popover = html_button
            .as_ref()
            .map(|hb| {
                let prototype = js_sys::Reflect::get(hb, &JsValue::from_str("prototype")).ok();
                prototype
                    .map(|p| has_property(&p, "popover"))
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        let supports_font_size_adjust = css
            .as_ref()
            .map(|c| css_supports(c, "font-size-adjust", "ex-height 0.5"))
            .unwrap_or(false);
        let supports_text_transform = css
            .as_ref()
            .map(|c| css_supports(c, "text-transform", "full-width"))
            .unwrap_or(false);

        return count_truthy(&[
            !has_property(&navigator, "getStorageUpdates"),
            has_popover,
            has_property(&window, "CSSCounterStyleRule"),
            supports_font_size_adjust,
            supports_text_transform,
        ]) >= 4;
    }
    false
}

pub fn is_android() -> bool {
    let is_it_chromium = is_chromium();
    let is_it_gecko = is_gecko();

    if let Some(window) = window() {
        let navigator = window.navigator();

        if is_it_chromium {
            let connection =
                js_sys::Reflect::get(&navigator, &JsValue::from_str("connection")).ok();
            let has_typechange = connection
                .as_ref()
                .map(|c| has_property(c, "ontypechange"))
                .unwrap_or(false);

            let audio_constructor = js_sys::Reflect::get(&window, &JsValue::from_str("Audio")).ok();
            let has_sink_id = audio_constructor
                .as_ref()
                .and_then(|ctor| {
                    if let Ok(func) = ctor.clone().dyn_into::<js_sys::Function>() {
                        if let Ok(audio) = js_sys::Reflect::construct(&func, &js_sys::Array::new())
                        {
                            Some(has_property(&audio, "sinkId"))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .unwrap_or(false);

            return count_truthy(&[
                !has_property(&window, "SharedWorker"),
                has_typechange,
                !has_sink_id,
            ]) >= 2;
        } else if is_it_gecko {
            let app_version = navigator.app_version().unwrap_or_default();
            return count_truthy(&[
                has_property(&window, "onorientationchange"),
                has_property(&window, "orientation"),
                app_version.to_lowercase().contains("android"),
            ]) >= 2;
        }
    }
    false
}

pub fn is_samsung_internet() -> bool {
    if let Some(window) = window() {
        let navigator = window.navigator();

        let audio_constructor = js_sys::Reflect::get(&window, &JsValue::from_str("Audio")).ok();
        let audio_prototype = audio_constructor
            .as_ref()
            .and_then(|c| js_sys::Reflect::get(c, &JsValue::from_str("prototype")).ok());

        let has_sr_latency = audio_prototype
            .as_ref()
            .map(|p| has_property(p, "srLatency"))
            .unwrap_or(false);
        let has_sr_channel_count = audio_prototype
            .as_ref()
            .map(|p| has_property(p, "srChannelCount"))
            .unwrap_or(false);

        let visual_viewport =
            js_sys::Reflect::get(&window, &JsValue::from_str("visualViewport")).ok();
        let has_segments = visual_viewport
            .as_ref()
            .map(|v| has_property(v, "segments"))
            .unwrap_or(false);

        let image_constructor = js_sys::Reflect::get(&window, &JsValue::from_str("Image")).ok();
        let image_prototype = image_constructor
            .as_ref()
            .and_then(|c| js_sys::Reflect::get(c, &JsValue::from_str("prototype")).ok());
        let has_get_text_information = image_prototype
            .as_ref()
            .map(|p| has_property(p, "getTextInformation"))
            .unwrap_or(false);

        return count_truthy(&[
            has_sr_latency,
            has_sr_channel_count,
            has_property(&navigator, "devicePosture"),
            has_segments,
            has_get_text_information,
        ]) >= 3;
    }
    false
}

/**
 * Checked on:
 * Safari on iPadOS (both mobile and desktop modes): 8, 11-18
 * Chrome on iPadOS (both mobile and desktop modes): 11-18
 * Safari on iOS (both mobile and desktop modes): 9-18
 * Chrome on iOS (both mobile and desktop modes): 9-18
 */
pub fn is_ipad() -> bool {
    if let Some(window) = window() {
        let navigator = window.navigator();

        // Before iOS 13. Safari tampers the value in "request desktop site" mode since iOS 13.
        if let Ok(platform) = navigator.platform() {
            if platform == "iPad" {
                return true;
            }
        }

        if let Some(screen) = window.screen().ok() {
            let width = js_sys::Reflect::get(&screen, &JsValue::from_str("width"))
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let height = js_sys::Reflect::get(&screen, &JsValue::from_str("height"))
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);

            if height == 0.0 {
                return false;
            }

            let screen_ratio = width / height;

            return count_truthy(&[
                // Since iOS 13. Doesn't work in Chrome on iPadOS <15, but works in desktop mode.
                has_property(&window, "MediaSource"),
                // Since iOS 12. Doesn't work in Chrome on iPadOS.
                has_property(
                    &js_sys::Reflect::get(&window, &JsValue::from_str("Element"))
                        .ok()
                        .and_then(|e| {
                            js_sys::Reflect::get(&e, &JsValue::from_str("prototype")).ok()
                        })
                        .unwrap_or(JsValue::undefined()),
                    "webkitRequestFullscreen",
                ),
                // iPhone 4S that runs iOS 9 matches this, but it is not supported
                // Doesn't work in incognito mode of Safari ≥17 with split screen because of
                // tracking prevention
                screen_ratio > 0.65 && screen_ratio < 1.53,
            ]) >= 2;
        }
    }
    false
}

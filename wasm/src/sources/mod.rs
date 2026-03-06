use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

mod apple_pay;
mod architecture;
mod audio;
mod audio_base_latency;
mod canvas;
mod color_depth;
mod color_gamut;
mod contrast;
mod cookies_enabled;
mod cpu_class;
mod date_time_locale;
mod device_memory;
mod dom_blockers;
mod font_preferences;
mod fonts;
mod forced_colors;
mod hardware_concurrency;
mod hdr;
mod indexed_db;
mod inverted_colors;
mod languages;
mod local_storage;
mod math;
mod monochrome;
mod open_database;
mod os_cpu;
mod pdf_viewer_enabled;
mod platform;
mod plugins;
mod private_click_measurement;
mod reduced_motion;
mod reduced_transparency;
mod screen_frame;
mod screen_resolution;
mod session_storage;
mod timezone;
mod touch_support;
mod vendor;
mod vendor_flavors;
mod webgl;

#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_namespace = Date)]
    unsafe fn now() -> f64;
}

enum SourceResult {
    Sync(Result<JsValue, JsValue>),
    Async(js_sys::Promise),
}

struct SourceDefinition {
    name: &'static str,
    source: fn() -> SourceResult,
}

static SOURCES: &[SourceDefinition] = &[
    SourceDefinition {
        name: "fonts",
        source: || SourceResult::Async(wasm_bindgen_futures::future_to_promise(fonts::get_fonts())),
    },
    SourceDefinition {
        name: "domBlockers",
        source: || {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(
                dom_blockers::get_dom_blockers(),
            ))
        },
    },
    SourceDefinition {
        name: "fontPreferences",
        source: || {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(
                font_preferences::get_font_preferences(),
            ))
        },
    },
    SourceDefinition {
        name: "audio",
        source: || {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(
                audio::get_audio_fingerprint(),
            ))
        },
    },
    SourceDefinition {
        name: "screenFrame",
        source: || {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(
                screen_frame::get_screen_frame(),
            ))
        },
    },
    SourceDefinition {
        name: "canvas",
        source: || SourceResult::Sync(canvas::get_canvas_fingerprint()),
    },
    SourceDefinition {
        name: "osCpu",
        source: || SourceResult::Sync(os_cpu::get_os_cpu()),
    },
    SourceDefinition {
        name: "languages",
        source: || SourceResult::Sync(languages::get_languages()),
    },
    SourceDefinition {
        name: "colorDepth",
        source: || SourceResult::Sync(color_depth::get_color_depth()),
    },
    SourceDefinition {
        name: "deviceMemory",
        source: || SourceResult::Sync(device_memory::get_device_memory()),
    },
    SourceDefinition {
        name: "screenResolution",
        source: || SourceResult::Sync(screen_resolution::get_screen_resolution()),
    },
    SourceDefinition {
        name: "hardwareConcurrency",
        source: || SourceResult::Sync(hardware_concurrency::get_hardware_concurrency()),
    },
    SourceDefinition {
        name: "timezone",
        source: || SourceResult::Sync(timezone::get_timezone()),
    },
    SourceDefinition {
        name: "sessionStorage",
        source: || SourceResult::Sync(session_storage::get_session_storage()),
    },
    SourceDefinition {
        name: "localStorage",
        source: || SourceResult::Sync(local_storage::get_local_storage()),
    },
    SourceDefinition {
        name: "indexedDB",
        source: || SourceResult::Sync(indexed_db::get_indexed_db()),
    },
    SourceDefinition {
        name: "openDatabase",
        source: || SourceResult::Sync(open_database::get_open_database()),
    },
    SourceDefinition {
        name: "cpuClass",
        source: || SourceResult::Sync(cpu_class::get_cpu_class()),
    },
    SourceDefinition {
        name: "platform",
        source: || SourceResult::Sync(platform::get_platform()),
    },
    SourceDefinition {
        name: "plugins",
        source: || SourceResult::Sync(plugins::get_plugins()),
    },
    SourceDefinition {
        name: "touchSupport",
        source: || SourceResult::Sync(touch_support::get_touch_support()),
    },
    SourceDefinition {
        name: "vendor",
        source: || SourceResult::Sync(vendor::get_vendor()),
    },
    SourceDefinition {
        name: "vendorFlavors",
        source: || SourceResult::Sync(vendor_flavors::get_vendor_flavors()),
    },
    SourceDefinition {
        name: "cookiesEnabled",
        source: || SourceResult::Sync(cookies_enabled::are_cookies_enabled()),
    },
    SourceDefinition {
        name: "colorGamut",
        source: || SourceResult::Sync(color_gamut::get_color_gamut()),
    },
    SourceDefinition {
        name: "invertedColors",
        source: || SourceResult::Sync(inverted_colors::are_colors_inverted()),
    },
    SourceDefinition {
        name: "forcedColors",
        source: || SourceResult::Sync(forced_colors::are_colors_forced()),
    },
    SourceDefinition {
        name: "monochrome",
        source: || SourceResult::Sync(monochrome::get_monochrome_depth()),
    },
    SourceDefinition {
        name: "contrast",
        source: || SourceResult::Sync(contrast::get_contrast()),
    },
    SourceDefinition {
        name: "reducedMotion",
        source: || SourceResult::Sync(reduced_motion::is_motion_reduced()),
    },
    SourceDefinition {
        name: "reducedTransparency",
        source: || SourceResult::Sync(reduced_transparency::is_transparency_reduced()),
    },
    SourceDefinition {
        name: "hdr",
        source: || SourceResult::Sync(hdr::is_hdr()),
    },
    SourceDefinition {
        name: "math",
        source: || SourceResult::Sync(math::get_math_fingerprint()),
    },
    SourceDefinition {
        name: "pdfViewerEnabled",
        source: || SourceResult::Sync(pdf_viewer_enabled::is_pdf_viewer_enabled()),
    },
    SourceDefinition {
        name: "architecture",
        source: || SourceResult::Sync(architecture::get_architecture()),
    },
    SourceDefinition {
        name: "applePay",
        source: || {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(
                apple_pay::get_apple_pay_state(),
            ))
        },
    },
    SourceDefinition {
        name: "privateClickMeasurement",
        source: || SourceResult::Sync(private_click_measurement::get_private_click_measurement()),
    },
    SourceDefinition {
        name: "audioBaseLatency",
        source: || {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(
                audio_base_latency::get_audio_context_base_latency(),
            ))
        },
    },
    SourceDefinition {
        name: "dateTimeLocale",
        source: || SourceResult::Sync(date_time_locale::get_date_time_locale()),
    },
    SourceDefinition {
        name: "webGlBasics",
        source: || SourceResult::Sync(webgl::get_web_gl_basics()),
    },
    SourceDefinition {
        name: "webGlExtensions",
        source: || SourceResult::Sync(webgl::get_web_gl_extensions()),
    },
];

pub fn load_builtin_sources() -> js_sys::Promise {
    wasm_bindgen_futures::future_to_promise(async move {
        let components = load_sources(SOURCES).await?;
        Ok(JsValue::from(components))
    })
}

async fn load_sources(sources: &[SourceDefinition]) -> Result<js_sys::Object, JsValue> {
    let components = js_sys::Object::new();

    for source_def in sources {
        let load_start_time = unsafe { now() };

        let result = match (source_def.source)() {
            SourceResult::Sync(r) => r,
            SourceResult::Async(promise) => wasm_bindgen_futures::JsFuture::from(promise).await,
        };

        let duration = unsafe { now() } - load_start_time;

        let component = js_sys::Object::new();
        match result {
            Ok(value) => {
                if !value.is_null() {
                    js_sys::Reflect::set(&component, &JsValue::from_str("value"), &value).unwrap();
                }
            }
            Err(error) => {
                js_sys::Reflect::set(&component, &JsValue::from_str("error"), &error).unwrap();
            }
        }
        js_sys::Reflect::set(
            &component,
            &JsValue::from_str("duration"),
            &JsValue::from_f64(duration),
        )
        .unwrap();

        js_sys::Reflect::set(&components, &JsValue::from_str(source_def.name), &component).unwrap();
    }

    Ok(components)
}

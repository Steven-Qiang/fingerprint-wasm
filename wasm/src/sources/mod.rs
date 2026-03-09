use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console;

#[derive(Default, Clone)]
pub struct Options {
    pub debug: bool,
}

#[derive(Clone)]
pub struct SourceContext {
    pub options: Options,
}

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
    source: fn(&SourceContext) -> SourceResult,
}

static SOURCES: &[SourceDefinition] = &[
    SourceDefinition {
        name: "fonts",
        source: |ctx| {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(fonts::get_fonts(
                ctx.clone(),
            )))
        },
    },
    SourceDefinition {
        name: "domBlockers",
        source: |ctx| {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(
                dom_blockers::get_dom_blockers(ctx.clone()),
            ))
        },
    },
    SourceDefinition {
        name: "fontPreferences",
        source: |ctx| {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(
                font_preferences::get_font_preferences(ctx.clone()),
            ))
        },
    },
    SourceDefinition {
        name: "audio",
        source: |ctx| {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(
                audio::get_audio_fingerprint(ctx.clone()),
            ))
        },
    },
    SourceDefinition {
        name: "screenFrame",
        source: |ctx| {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(
                screen_frame::get_screen_frame(ctx.clone()),
            ))
        },
    },
    SourceDefinition {
        name: "canvas",
        source: |ctx| SourceResult::Sync(canvas::get_canvas_fingerprint(ctx)),
    },
    SourceDefinition {
        name: "osCpu",
        source: |ctx| SourceResult::Sync(os_cpu::get_os_cpu(ctx)),
    },
    SourceDefinition {
        name: "languages",
        source: |ctx| SourceResult::Sync(languages::get_languages(ctx)),
    },
    SourceDefinition {
        name: "colorDepth",
        source: |ctx| SourceResult::Sync(color_depth::get_color_depth(ctx)),
    },
    SourceDefinition {
        name: "deviceMemory",
        source: |ctx| SourceResult::Sync(device_memory::get_device_memory(ctx)),
    },
    SourceDefinition {
        name: "screenResolution",
        source: |ctx| SourceResult::Sync(screen_resolution::get_screen_resolution(ctx)),
    },
    SourceDefinition {
        name: "hardwareConcurrency",
        source: |ctx| SourceResult::Sync(hardware_concurrency::get_hardware_concurrency(ctx)),
    },
    SourceDefinition {
        name: "timezone",
        source: |ctx| SourceResult::Sync(timezone::get_timezone(ctx)),
    },
    SourceDefinition {
        name: "sessionStorage",
        source: |ctx| SourceResult::Sync(session_storage::get_session_storage(ctx)),
    },
    SourceDefinition {
        name: "localStorage",
        source: |ctx| SourceResult::Sync(local_storage::get_local_storage(ctx)),
    },
    SourceDefinition {
        name: "indexedDB",
        source: |ctx| SourceResult::Sync(indexed_db::get_indexed_db(ctx)),
    },
    SourceDefinition {
        name: "openDatabase",
        source: |ctx| SourceResult::Sync(open_database::get_open_database(ctx)),
    },
    SourceDefinition {
        name: "cpuClass",
        source: |ctx| SourceResult::Sync(cpu_class::get_cpu_class(ctx)),
    },
    SourceDefinition {
        name: "platform",
        source: |ctx| SourceResult::Sync(platform::get_platform(ctx)),
    },
    SourceDefinition {
        name: "plugins",
        source: |ctx| SourceResult::Sync(plugins::get_plugins(ctx)),
    },
    SourceDefinition {
        name: "touchSupport",
        source: |ctx| SourceResult::Sync(touch_support::get_touch_support(ctx)),
    },
    SourceDefinition {
        name: "vendor",
        source: |ctx| SourceResult::Sync(vendor::get_vendor(ctx)),
    },
    SourceDefinition {
        name: "vendorFlavors",
        source: |ctx| SourceResult::Sync(vendor_flavors::get_vendor_flavors(ctx)),
    },
    SourceDefinition {
        name: "cookiesEnabled",
        source: |ctx| SourceResult::Sync(cookies_enabled::are_cookies_enabled(ctx)),
    },
    SourceDefinition {
        name: "colorGamut",
        source: |ctx| SourceResult::Sync(color_gamut::get_color_gamut(ctx)),
    },
    SourceDefinition {
        name: "invertedColors",
        source: |ctx| SourceResult::Sync(inverted_colors::are_colors_inverted(ctx)),
    },
    SourceDefinition {
        name: "forcedColors",
        source: |ctx| SourceResult::Sync(forced_colors::are_colors_forced(ctx)),
    },
    SourceDefinition {
        name: "monochrome",
        source: |ctx| SourceResult::Sync(monochrome::get_monochrome_depth(ctx)),
    },
    SourceDefinition {
        name: "contrast",
        source: |ctx| SourceResult::Sync(contrast::get_contrast(ctx)),
    },
    SourceDefinition {
        name: "reducedMotion",
        source: |ctx| SourceResult::Sync(reduced_motion::is_motion_reduced(ctx)),
    },
    SourceDefinition {
        name: "reducedTransparency",
        source: |ctx| SourceResult::Sync(reduced_transparency::is_transparency_reduced(ctx)),
    },
    SourceDefinition {
        name: "hdr",
        source: |ctx| SourceResult::Sync(hdr::is_hdr(ctx)),
    },
    SourceDefinition {
        name: "math",
        source: |ctx| SourceResult::Sync(math::get_math_fingerprint(ctx)),
    },
    SourceDefinition {
        name: "pdfViewerEnabled",
        source: |ctx| SourceResult::Sync(pdf_viewer_enabled::is_pdf_viewer_enabled(ctx)),
    },
    SourceDefinition {
        name: "architecture",
        source: |ctx| SourceResult::Sync(architecture::get_architecture(ctx)),
    },
    SourceDefinition {
        name: "applePay",
        source: |ctx| {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(
                apple_pay::get_apple_pay_state(ctx.clone()),
            ))
        },
    },
    SourceDefinition {
        name: "privateClickMeasurement",
        source: |ctx| {
            SourceResult::Sync(private_click_measurement::get_private_click_measurement(
                ctx,
            ))
        },
    },
    SourceDefinition {
        name: "audioBaseLatency",
        source: |ctx| {
            SourceResult::Async(wasm_bindgen_futures::future_to_promise(
                audio_base_latency::get_audio_context_base_latency(ctx.clone()),
            ))
        },
    },
    SourceDefinition {
        name: "dateTimeLocale",
        source: |ctx| SourceResult::Sync(date_time_locale::get_date_time_locale(ctx)),
    },
    SourceDefinition {
        name: "webGlBasics",
        source: |ctx| SourceResult::Sync(webgl::get_web_gl_basics(ctx)),
    },
    SourceDefinition {
        name: "webGlExtensions",
        source: |ctx| SourceResult::Sync(webgl::get_web_gl_extensions(ctx)),
    },
];

pub fn load_builtin_sources(options: Options) -> js_sys::Promise {
    wasm_bindgen_futures::future_to_promise(async move {
        let components = load_sources(SOURCES, options).await?;
        Ok(JsValue::from(components))
    })
}

fn format_js_error(error: &JsValue) -> String {
    match js_sys::JSON::stringify(error) {
        Ok(s) => s.as_string().unwrap_or_else(|| format!("{:?}", error)),
        Err(_) => format!("{:?}", error),
    }
}

async fn load_sources(
    sources: &[SourceDefinition],
    options: Options,
) -> Result<js_sys::Object, JsValue> {
    let components = js_sys::Object::new();
    let total_start = unsafe { now() };
    let context = SourceContext {
        options: options.clone(),
    };

    for (index, source_def) in sources.iter().enumerate() {
        let load_start_time = unsafe { now() };

        if options.debug {
            console::log_1(&JsValue::from_str(&format!(
                "[Source {}/{}] Starting: {}",
                index + 1,
                sources.len(),
                source_def.name
            )));
        }

        let result = match (source_def.source)(&context) {
            SourceResult::Sync(r) => {
                if options.debug {
                    console::log_1(&JsValue::from_str(&format!(
                        "[Source {}] {} is SYNC, executing...",
                        index + 1,
                        source_def.name
                    )));
                }
                r
            }
            SourceResult::Async(promise) => {
                if options.debug {
                    console::log_1(&JsValue::from_str(&format!(
                        "[Source {}] {} is ASYNC, waiting...",
                        index + 1,
                        source_def.name
                    )));
                }
                let result = wasm_bindgen_futures::JsFuture::from(promise).await;
                if options.debug {
                    console::log_1(&JsValue::from_str(&format!(
                        "[Source {}] {} ASYNC completed",
                        index + 1,
                        source_def.name
                    )));
                }
                result
            }
        };

        let duration = unsafe { now() } - load_start_time;

        let component = js_sys::Object::new();
        match result {
            Ok(value) => {
                if !value.is_null() {
                    js_sys::Reflect::set(&component, &JsValue::from_str("value"), &value).unwrap();
                }
                if options.debug {
                    console::log_1(&JsValue::from_str(&format!(
                        "[Source {}] {} SUCCESS ({:.2}ms)",
                        index + 1,
                        source_def.name,
                        duration
                    )));
                }
            }
            Err(error) => {
                js_sys::Reflect::set(&component, &JsValue::from_str("error"), &error).unwrap();
                if options.debug {
                    console::error_1(&JsValue::from_str(&format!(
                        "[Source {}] {} ERROR ({:.2}ms): {}",
                        index + 1,
                        source_def.name,
                        duration,
                        format_js_error(&error)
                    )));
                }
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

    let total_duration = unsafe { now() } - total_start;
    if options.debug {
        console::log_1(&JsValue::from_str(&format!(
            "[Complete] All {} sources finished in {:.2}ms",
            sources.len(),
            total_duration
        )));
    }
    Ok(components)
}

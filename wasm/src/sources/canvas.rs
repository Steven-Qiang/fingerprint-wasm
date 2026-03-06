use js_sys::{Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

use crate::utils::browser::{
    is_gecko, is_gecko_120_or_newer, is_safari_webkit, is_webkit, is_webkit_616_or_newer,
};

pub fn get_canvas_fingerprint() -> Result<JsValue, JsValue> {
    let skip_images = does_browser_perform_anti_fingerprinting();
    let result = get_unstable_canvas_fingerprint(skip_images);
    Ok(JsValue::from(result))
}

/**
 * A version of the entropy source with stabilization to make it suitable for static
 * fingerprinting.
 *
 * Canvas image is noised in private mode of Safari 17, so image rendering is skipped in Safari
 * 17. Firefox 120+ randomizes canvas data in private browsing and strict ETP mode,
 * so image rendering is skipped in Firefox 120+.
 *
 * @see https://www.browserleaks.com/canvas#how-does-it-work
 * @see https://bugzilla.mozilla.org/show_bug.cgi?id=1816189 Firefox canvas randomization
 */
pub fn get_unstable_canvas_fingerprint(skip_images: bool) -> JsValue {
    let (canvas, context) = make_canvas_context();

    let (winding, geometry, text) = match context {
        Some(ctx) => {
            let winding = does_support_winding(&ctx);

            if skip_images {
                (
                    winding,
                    JsValue::from_str("skipped"),
                    JsValue::from_str("skipped"),
                )
            } else {
                let (geo, txt) = render_images(&canvas, &ctx);
                (winding, geo, txt)
            }
        }
        None => (
            false,
            JsValue::from_str("unsupported"),
            JsValue::from_str("unsupported"),
        ),
    };

    let result = Object::new();
    Reflect::set(
        &result,
        &JsValue::from_str("winding"),
        &JsValue::from_bool(winding),
    )
    .unwrap();
    Reflect::set(&result, &JsValue::from_str("geometry"), &geometry).unwrap();
    Reflect::set(&result, &JsValue::from_str("text"), &text).unwrap();

    result.into()
}

fn make_canvas_context() -> (HtmlCanvasElement, Option<CanvasRenderingContext2d>) {
    let canvas = window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canvas.set_width(1);
    canvas.set_height(1);

    let context = canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok());

    (canvas, context)
}

fn does_support_winding(context: &CanvasRenderingContext2d) -> bool {
    // https://web.archive.org/web/20170825024655/http://blogs.adobe.com/webplatform/2013/01/30/winding-rules-in-canvas/
    // https://github.com/Modernizr/Modernizr/blob/master/feature-detects/canvas/winding.js
    context.begin_path();
    context.rect(0.0, 0.0, 10.0, 10.0);
    context.rect(2.0, 2.0, 6.0, 6.0);
    context.is_point_in_path_with_f64(5.0, 5.0)
}

fn render_images(
    canvas: &HtmlCanvasElement,
    context: &CanvasRenderingContext2d,
) -> (JsValue, JsValue) {
    render_text_image(canvas, context);
    let text_image_1 = canvas_to_string(canvas);
    let text_image_2 = canvas_to_string(canvas);

    if text_image_1 != text_image_2 {
        return (JsValue::from_str("unstable"), JsValue::from_str("unstable"));
    }

    render_geometry_image(canvas, context);
    let geometry_image = canvas_to_string(canvas);

    (geometry_image, text_image_1)
}

fn render_text_image(canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d) {
    canvas.set_width(240);
    canvas.set_height(60);

    context.set_text_baseline("alphabetic");

    context.set_fill_style_str("#f60");
    context.fill_rect(100.0, 1.0, 62.0, 20.0);

    context.set_fill_style_str("#069");
    context.set_font("11pt \"Times New Roman\"");

    let printed_text = format!(
        "Cwm fjordbank gly {}",
        String::from_utf8(vec![0xF0, 0x9F, 0x98, 0x83]).unwrap()
    );
    context.fill_text(&printed_text, 2.0, 15.0).ok();

    context.set_fill_style_str("rgba(102, 204, 0, 0.2)");
    context.set_font("18pt Arial");
    context.fill_text(&printed_text, 4.0, 45.0).ok();
}

fn render_geometry_image(canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d) {
    canvas.set_width(122);
    canvas.set_height(110);

    let _ = context.set_global_composite_operation("multiply");

    let colors = vec![
        ("#f2f", 40.0, 40.0),
        ("#2ff", 80.0, 40.0),
        ("#ff2", 60.0, 80.0),
    ];

    for (color, x, y) in colors {
        context.set_fill_style_str(color);
        context.begin_path();
        let _ = context.arc_with_anticlockwise(x, y, 40.0, 0.0, 2.0 * std::f64::consts::PI, true);
        context.close_path();
        context.fill();
    }

    context.set_fill_style_str("#f9c");
    let _ = context.arc_with_anticlockwise(60.0, 60.0, 60.0, 0.0, 2.0 * std::f64::consts::PI, true);
    let _ = context.arc_with_anticlockwise(60.0, 60.0, 20.0, 0.0, 2.0 * std::f64::consts::PI, true);

    let fill_func = Reflect::get(&JsValue::from(context), &JsValue::from_str("fill")).unwrap();
    if let Ok(func) = fill_func.dyn_into::<js_sys::Function>() {
        let _ = func.call1(&JsValue::from(context), &JsValue::from_str("evenodd"));
    }
}

fn canvas_to_string(canvas: &HtmlCanvasElement) -> JsValue {
    JsValue::from_str(&canvas.to_data_url().unwrap_or_default())
}

fn does_browser_perform_anti_fingerprinting() -> bool {
    let is_safari_17_or_above = is_webkit() && is_webkit_616_or_newer() && is_safari_webkit();
    let is_firefox_120_or_above = is_gecko() && is_gecko_120_or_newer();

    is_safari_17_or_above || is_firefox_120_or_above
}
